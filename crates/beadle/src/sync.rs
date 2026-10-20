//! `beadle sync <target>`: comment delta-sweep across open issues.
//!
//! Purpose: capture maintainer-engagement events (the compass) without
//! re-reading whole issue bodies. We fetch open issues once with
//! `gh issue list ... --json comments`, then emit a `CommentEvent` for every
//! comment we haven't seen before (per (issue, ts, actor) tuple).
//!
//! Cost: one `gh` call regardless of open-issue count, thanks to bulk fields.

use anyhow::Result;
use beadle_store::{CommentEventRecord, Record, Store};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::path::Path;

use crate::{enumerate::fetch_was_truncated, gh, intent};

#[derive(Deserialize)]
struct GhCommentAuthor {
    #[serde(default)]
    login: Option<String>,
}

#[derive(Deserialize)]
struct GhComment {
    #[serde(default)]
    author: Option<GhCommentAuthor>,
    #[serde(rename = "createdAt")]
    created_at: String,
    #[serde(default)]
    body: String,
}

#[derive(Deserialize)]
struct GhIssueForSync {
    number: u32,
    #[serde(default)]
    comments: Vec<GhComment>,
}

/// Upper bound on the open-issue fetch. A tripwire, not a tuning knob: `gh`
/// truncates silently at `--limit`, so a result of exactly this many may have
/// lost rows (ArcavenAE/beadle#96).
const SYNC_FETCH_LIMIT: usize = 500;

/// Fetch the open issues with their comments through `fetch`, which stands in
/// for `gh::json` so a test can hand back a fixture instead of calling `gh`.
fn fetch_open_issues(
    repo: &str,
    limit: usize,
    fetch: impl FnOnce(&[&str]) -> Result<Vec<GhIssueForSync>>,
) -> Result<Vec<GhIssueForSync>> {
    let limit_s = limit.to_string();
    let args = [
        "issue",
        "list",
        "--repo",
        repo,
        "--state",
        "open",
        "--limit",
        limit_s.as_str(),
        "--json",
        "number,comments",
    ];
    let issues = fetch(&args)?;
    if fetch_was_truncated(issues.len(), limit) {
        anyhow::bail!(
            "gh returned {} open issues against --limit {limit}: the fetch was \
             probably truncated and this sync would silently skip the comments on \
             whatever fell off the end. Raise SYNC_FETCH_LIMIT or paginate; do not \
             proceed on a partial corpus.",
            issues.len()
        );
    }
    Ok(issues)
}

pub fn run(root: &Path, target: &str) -> Result<()> {
    let intent = intent::load(root, target)?;
    let store = Store::open(root.join("store"), target)?;

    // Load existing comment fingerprints so we can skip them.
    let seen: HashSet<(u32, String, String)> = store
        .read_all()?
        .into_iter()
        .filter_map(|r| match r {
            Record::CommentEvent(c) => Some((c.number, c.ts.clone(), c.actor.clone())),
            _ => None,
        })
        .collect();

    let latest_run = store.latest_run()?;
    let next_run = latest_run.as_ref().map(|r| r.run + 1).unwrap_or(1);

    eprintln!(
        "beadle sync: target={target} repo={} run={next_run} known-comments={}",
        intent.repo,
        seen.len()
    );

    let issues = fetch_open_issues(&intent.repo, SYNC_FETCH_LIMIT, gh::json)?;

    let mut recs: Vec<Record> = Vec::new();
    let mut maintainer_events = 0u32;
    let mut measured_events = 0u32;
    let mut other_events = 0u32;

    for iss in &issues {
        for c in &iss.comments {
            let actor = c
                .author
                .as_ref()
                .and_then(|a| a.login.clone())
                .unwrap_or_else(|| "unknown".to_string());
            let key = (iss.number, c.created_at.clone(), actor.clone());
            if seen.contains(&key) {
                continue;
            }
            let role = intent.actor_role(&actor);
            match role {
                "maintainer" => maintainer_events += 1,
                "measured" => measured_events += 1,
                _ => other_events += 1,
            }

            let body_bytes = c.body.as_bytes();
            let mut hasher = Sha256::new();
            hasher.update(body_bytes);
            let body_sha = format!("{:x}", hasher.finalize());

            recs.push(Record::CommentEvent(CommentEventRecord {
                ts: c.created_at.clone(),
                target: target.to_string(),
                number: iss.number,
                event: "comment".to_string(),
                actor,
                actor_role: role.to_string(),
                body_len: Some(body_bytes.len()),
                body_sha256: Some(body_sha),
                observed_in_run: next_run,
            }));
        }
    }

    if !recs.is_empty() {
        store.append(&recs)?;
    }
    eprintln!(
        "beadle sync: appended {} events (maintainer={} measured={} other={})",
        recs.len(),
        maintainer_events,
        measured_events,
        other_events
    );

    let summary = serde_json::json!({
        "target": target,
        "run": next_run,
        "new_events": recs.len(),
        "maintainer_events": maintainer_events,
        "measured_events": measured_events,
        "other_events": other_events,
        "open_issues_scanned": issues.len(),
    });
    println!("{}", summary);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn issues(n: usize) -> Vec<GhIssueForSync> {
        (1..=n as u32)
            .map(|number| GhIssueForSync {
                number,
                comments: Vec::new(),
            })
            .collect()
    }

    #[test]
    fn a_fetch_returning_exactly_its_limit_is_refused() {
        // `gh` gives no truncation signal, so 500 of 500 may be 500 of 519.
        let err = fetch_open_issues("acme/widget", 500, |_| Ok(issues(500)))
            .map(|v| v.len())
            .unwrap_err()
            .to_string();
        assert!(err.contains("--limit 500"), "{err}");
        assert!(err.contains("truncated"), "{err}");
    }

    #[test]
    fn a_fetch_under_its_limit_passes_through() {
        let got = fetch_open_issues("acme/widget", 500, |_| Ok(issues(499))).unwrap();
        assert_eq!(got.len(), 499);
    }

    #[test]
    fn the_fetch_asks_gh_for_the_limit_it_checks_against() {
        fetch_open_issues("acme/widget", 500, |args| {
            let at = args.iter().position(|a| *a == "--limit").unwrap();
            assert_eq!(args[at + 1], "500");
            Ok(Vec::new())
        })
        .unwrap();
    }
}
