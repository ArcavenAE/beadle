//! `beadle enum <target>` — watermark-bounded issue enumeration.
//!
//! Fetches open issues from the target repo via `gh`, filters to numbers
//! strictly greater than the current watermark (or --full = ignore watermark),
//! and appends `IssueRecord` rows to the store. Writes a summary to stderr;
//! JSON summary of new numbers to stdout so it composes with a shell pipe.

use std::{collections::HashSet, path::Path};

use anyhow::{Context, Result};
use beadle_store::{IssueRecord, Record, Store};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

use crate::{gh, intent};

#[derive(Deserialize)]
struct GhAuthor {
    #[serde(default)]
    login: Option<String>,
}

#[derive(Deserialize)]
struct GhLabel {
    name: String,
}

#[derive(Deserialize)]
struct GhAssignee {
    login: String,
}

#[derive(Deserialize)]
struct GhIssue {
    number: u32,
    title: String,
    #[serde(default)]
    author: Option<GhAuthor>,
    state: String,
    #[serde(rename = "createdAt")]
    created_at: String,
    #[serde(rename = "updatedAt")]
    updated_at: String,
    #[serde(rename = "closedAt", default)]
    closed_at: Option<String>,
    #[serde(default)]
    labels: Vec<GhLabel>,
    #[serde(default)]
    assignees: Vec<GhAssignee>,
    #[serde(default)]
    body: String,
}

/// Upper bound on a single `gh issue list` fetch.
///
/// Not a tuning knob — a tripwire. `gh` truncates silently at `--limit`, so a
/// fetch returning exactly this many is indistinguishable from a truncated one
/// and [`fetch_issues`] refuses it rather than reporting success on partial
/// data (ArcavenAE/beadle#82).
const ISSUE_FETCH_LIMIT: usize = 1000;

/// Whether a fetch of `count` against `limit` may have lost rows.
///
/// `>=`, not `>`: a result of exactly `limit` is indistinguishable from a
/// truncated one, so it counts as truncated. Treating it as complete is the
/// bug — `gh` gives no signal either way.
fn fetch_was_truncated(count: usize, limit: usize) -> bool {
    count >= limit
}

/// Tracked issues that GitHub reports closed while the store still believes
/// they are open.
///
/// The asymmetry is the point: a closed issue found here is NOT new, so it must
/// not enter `new_numbers` and must not move the watermark. It is a fresh
/// observation of something already tracked.
fn needs_reconciliation(believed_open: &HashSet<u32>, closed_numbers: &[u32]) -> Vec<u32> {
    let mut out: Vec<u32> = closed_numbers
        .iter()
        .copied()
        .filter(|n| believed_open.contains(n))
        .collect();
    out.sort_unstable();
    out.dedup();
    out
}

/// Fetch issues in one state, refusing a result that may have been truncated.
///
/// The old code asked for 500 against a repo with 519 open issues and reported
/// `fetched 500 open issues` as success. Newest-first ordering made it harmless
/// incrementally — the dropped 19 were the oldest and sat below the watermark —
/// but `--full` exists to re-observe everything, and silently returning 500 of
/// 519 from the recovery path is the silent-success class finding-019 is about.
fn fetch_issues(repo: &str, state: &str, limit: usize) -> Result<Vec<GhIssue>> {
    let limit_s = limit.to_string();
    let args = vec![
        "issue",
        "list",
        "--repo",
        repo,
        "--state",
        state,
        "--limit",
        limit_s.as_str(),
        "--json",
        "number,title,author,state,createdAt,updatedAt,closedAt,labels,assignees,body",
    ];
    let issues: Vec<GhIssue> = gh::json(&args)?;
    if fetch_was_truncated(issues.len(), limit) {
        anyhow::bail!(
            "gh returned {} {state} issues against --limit {limit}: the fetch was \
             probably truncated and this run would silently omit whatever fell off \
             the end. Raise ISSUE_FETCH_LIMIT or paginate; do not proceed on a \
             partial corpus.",
            issues.len()
        );
    }
    Ok(issues)
}

fn to_record(iss: &GhIssue, target: &str, run: u32) -> Record {
    let body_bytes = iss.body.as_bytes();
    let mut hasher = Sha256::new();
    hasher.update(body_bytes);
    Record::Issue(IssueRecord {
        ts: iss.updated_at.clone(),
        target: target.to_string(),
        number: iss.number,
        observed_in_run: run,
        title: iss.title.clone(),
        author: iss
            .author
            .as_ref()
            .and_then(|a| a.login.clone())
            .unwrap_or_else(|| "unknown".to_string()),
        state: iss.state.to_lowercase(),
        created_at: iss.created_at.clone(),
        updated_at: iss.updated_at.clone(),
        closed_at: iss.closed_at.clone(),
        labels: iss.labels.iter().map(|l| l.name.clone()).collect(),
        assignees: iss.assignees.iter().map(|a| a.login.clone()).collect(),
        body_len: body_bytes.len(),
        body_sha256: format!("{:x}", hasher.finalize()),
    })
}

pub fn run(root: &Path, target: &str, full: bool) -> Result<()> {
    let intent = intent::load(root, target)?;
    let store = Store::open(root.join("store"), target)?;

    let watermark = if full { 0 } else { store.watermark()? };
    let latest_run = store.latest_run()?;
    let next_run = latest_run.as_ref().map(|r| r.run + 1).unwrap_or(1);

    eprintln!(
        "beadle enum: target={target} repo={} watermark={watermark} run={next_run}",
        intent.repo
    );

    let now = OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .context("format RFC3339 timestamp")?;

    // Pass 1 — DISCOVERY. New issues above the watermark.
    let issues = fetch_issues(&intent.repo, "open", ISSUE_FETCH_LIMIT)?;
    eprintln!("beadle enum: fetched {} open issues", issues.len());

    let mut new_numbers: Vec<u32> = Vec::new();
    let mut recs: Vec<Record> = Vec::new();
    for iss in &issues {
        if iss.number <= watermark {
            continue;
        }
        recs.push(to_record(iss, target, next_run));
        new_numbers.push(iss.number);
    }
    new_numbers.sort_unstable();

    // Pass 2 — RECONCILIATION. Issues the store still believes are open that
    // GitHub has since closed.
    //
    // Discovery alone can never learn this. It queries `--state open` and skips
    // anything at or below the watermark, so an issue's state was frozen at its
    // first observation forever — #418, #465 and #472 read `open` in the store
    // for nine runs after GitHub closed them (ArcavenAE/beadle#81). `--full` did
    // not help: it zeroes the watermark but the query stays `--state open`.
    //
    // A closed issue is not a *new* issue, so it never enters `new_numbers` and
    // never moves the watermark. It is a fresh observation of something already
    // tracked, which `latest_issue_observations` resolves by `observed_in_run`.
    let believed_open: HashSet<u32> = beadle_store::latest_issue_observations(&store.read_all()?)
        .into_iter()
        .filter(|i| i.state == "open")
        .map(|i| i.number)
        .collect();

    let closed = fetch_issues(&intent.repo, "closed", ISSUE_FETCH_LIMIT)?;
    let closed_numbers: Vec<u32> = closed.iter().map(|i| i.number).collect();
    let reconciled = needs_reconciliation(&believed_open, &closed_numbers);
    for iss in closed.iter().filter(|i| reconciled.contains(&i.number)) {
        recs.push(to_record(iss, target, next_run));
    }

    if reconciled.is_empty() {
        eprintln!(
            "beadle enum: reconciled 0 — no tracked issue closed since it was last seen \
             ({} closed on GitHub, none of them believed open here)",
            closed.len()
        );
    } else {
        eprintln!(
            "beadle enum: reconciled {} issue(s) closed since last observed: {:?}",
            reconciled.len(),
            reconciled
        );
    }

    if recs.is_empty() {
        eprintln!("beadle enum: no new issues above watermark {watermark}");
    } else {
        store.append(&recs)?;
        eprintln!(
            "beadle enum: appended {} issue observation(s) — {} new, {} reconciled",
            recs.len(),
            new_numbers.len(),
            reconciled.len()
        );
    }

    // stdout: machine-readable summary; drives the next step (sync/render).
    let summary = serde_json::json!({
        "target": target,
        "run": next_run,
        "watermark_before": watermark,
        "watermark_after": new_numbers.last().copied().unwrap_or(watermark),
        "new_issue_numbers": new_numbers,
        "reconciled_closed": reconciled,
        "open_issue_count": issues.len(),
        "timestamp": now,
    });
    println!("{}", summary);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_fetch_returning_exactly_its_limit_counts_as_truncated() {
        // The defect: `gh` gives no truncation signal, so 500-of-519 reported
        // success. Exactly-limit is indistinguishable from truncated and must
        // never be treated as complete.
        assert!(fetch_was_truncated(500, 500));
        assert!(fetch_was_truncated(501, 500));
        assert!(!fetch_was_truncated(499, 500));
    }

    #[test]
    fn an_empty_repo_is_not_a_truncated_fetch() {
        assert!(!fetch_was_truncated(0, 1000));
    }

    #[test]
    fn reconciliation_selects_only_tracked_issues_that_closed() {
        let believed_open = HashSet::from([418, 465, 472, 500]);
        // 999 is closed on GitHub but was never tracked; 500 is tracked and
        // still open, so neither belongs in the result.
        let closed = [472, 418, 999, 465];
        assert_eq!(
            needs_reconciliation(&believed_open, &closed),
            vec![418, 465, 472]
        );
    }

    #[test]
    fn reconciliation_is_empty_when_nothing_tracked_has_closed() {
        let believed_open = HashSet::from([1, 2, 3]);
        assert!(needs_reconciliation(&believed_open, &[99, 100]).is_empty());
    }

    #[test]
    fn reconciliation_never_returns_an_untracked_number() {
        // Guards the watermark: anything returned here gets a fresh observation
        // of an ALREADY TRACKED issue. An untracked number leaking through would
        // be a discovery masquerading as a reconciliation.
        let believed_open = HashSet::new();
        assert!(needs_reconciliation(&believed_open, &[1, 2, 3]).is_empty());
    }
}
