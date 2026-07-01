//! `beadle render <target>` — materialize the dashboard body from the store.
//!
//! Design: renderer owns derived zones (sentinel, counts, compass, clusters,
//! freshness). Editor owns marked slots preserved verbatim across regens.
//! Sentinel carries two digests — `derived_digest` (this file's output) and
//! `body_digest` (final merged body) — so a hand-edit to a derived zone is a
//! first-class detectable event on the next run.
//!
//! See `_kos/nodes/frontier/question-renderer-editorial-boundary.yaml`.

use anyhow::Result;
use beadle_store::{ClusterRecord, IssueRecord, Record, RunRecord, Store};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap};
use std::path::Path;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

use crate::intent;

/// Body-budget budget line. Exceeding it emits a warning and (future work)
/// triggers rollup of oldest editorial detail to a linked "Triage backlog" issue.
const BODY_BUDGET_BYTES: usize = 55 * 1024;

/// Cluster decay thresholds — runs since last add.
const DECAY_WARMING: u32 = 3;
const DECAY_ROLLUP: u32 = 5;
const DECAY_ARCHIVED: u32 = 8;

pub fn run(root: &Path, target: &str) -> Result<String> {
    let intent = intent::load(root, target)?;
    let store = Store::open(root.join("store"), target)?;
    let records = store.read_all()?;

    let latest_run = records
        .iter()
        .rev()
        .find_map(|r| {
            if let Record::Run(rr) = r {
                Some(rr.clone())
            } else {
                None
            }
        })
        .unwrap_or_else(|| synthetic_run(target));

    let latest_issues = latest_issue_observations(&records);
    let latest_clusters = latest_cluster_observations(&records);
    let comment_stats = comment_stats(&records);

    let derived = render_derived(
        target,
        &intent.repo,
        &latest_run,
        &latest_issues,
        &latest_clusters,
        &comment_stats,
    );
    let derived_digest = sha256_hex(&derived);
    let body = compose(target, &intent.repo, &latest_run, &derived, &derived_digest);

    let bytes = body.len();
    if bytes > BODY_BUDGET_BYTES {
        eprintln!(
            "beadle render: WARN body {} bytes exceeds budget {} — item C rollup pending classifications in store",
            bytes, BODY_BUDGET_BYTES
        );
    } else {
        eprintln!(
            "beadle render: body {} bytes ({}% of budget)",
            bytes,
            (bytes * 100) / BODY_BUDGET_BYTES
        );
    }

    Ok(body)
}

fn synthetic_run(target: &str) -> RunRecord {
    RunRecord {
        ts: OffsetDateTime::now_utc()
            .format(&Rfc3339)
            .unwrap_or_default(),
        target: target.to_string(),
        run: 0,
        watermark_before: 0,
        watermark_after: 0,
        counts: Default::default(),
        digest: String::new(),
        warmup: Some("cold-start".to_string()),
        intent_version: None,
        new_this_run: vec![],
        notes: None,
    }
}

/// For each issue number, take the most-recent observation seen in the store.
fn latest_issue_observations(records: &[Record]) -> Vec<IssueRecord> {
    let mut by_number: HashMap<u32, IssueRecord> = HashMap::new();
    for rec in records {
        if let Record::Issue(i) = rec {
            let keep = by_number
                .get(&i.number)
                .map(|prev| prev.observed_in_run <= i.observed_in_run)
                .unwrap_or(true);
            if keep {
                by_number.insert(i.number, i.clone());
            }
        }
    }
    let mut out: Vec<IssueRecord> = by_number.into_values().collect();
    out.sort_by_key(|i| std::cmp::Reverse(i.number));
    out
}

/// For each cluster name, take the most-recent observation.
fn latest_cluster_observations(records: &[Record]) -> Vec<ClusterRecord> {
    let mut by_name: HashMap<String, ClusterRecord> = HashMap::new();
    for rec in records {
        if let Record::Cluster(c) = rec {
            let keep = by_name
                .get(&c.name)
                .map(|prev| prev.run <= c.run)
                .unwrap_or(true);
            if keep {
                by_name.insert(c.name.clone(), c.clone());
            }
        }
    }
    let mut out: Vec<ClusterRecord> = by_name.into_values().collect();
    out.sort_by_key(|c| c.name.clone());
    out
}

#[derive(Debug, Default, Clone)]
struct CommentStats {
    total: u32,
    maintainer: u32,
    measured: u32,
    other: u32,
    /// per-actor totals — sorted descending in the render.
    per_actor: BTreeMap<String, u32>,
    /// issues touched by at least one comment.
    issues_with_comments: u32,
}

fn comment_stats(records: &[Record]) -> CommentStats {
    let mut stats = CommentStats::default();
    let mut issue_hits: std::collections::HashSet<u32> = std::collections::HashSet::new();
    for rec in records {
        if let Record::CommentEvent(c) = rec {
            stats.total += 1;
            match c.actor_role.as_str() {
                "maintainer" => stats.maintainer += 1,
                "measured" => stats.measured += 1,
                _ => stats.other += 1,
            }
            *stats.per_actor.entry(c.actor.clone()).or_insert(0) += 1;
            issue_hits.insert(c.number);
        }
    }
    stats.issues_with_comments = issue_hits.len() as u32;
    stats
}

/// Compute display decay for a cluster given the run it was last updated in
/// and the current run number.
fn decay_display(cluster: &ClusterRecord, current_run: u32) -> String {
    let gap = current_run.saturating_sub(cluster.last_added_run);
    if gap >= DECAY_ARCHIVED {
        format!("archived ({} runs quiet)", gap)
    } else if gap >= DECAY_ROLLUP {
        format!("rollup-candidate ({} runs quiet)", gap)
    } else if gap >= DECAY_WARMING {
        format!("warming ({} runs quiet)", gap)
    } else {
        format!("active (last add run {})", cluster.last_added_run)
    }
}

/// Render just the derived zone — the piece the sentinel digest is computed over.
/// Editor slots are NOT part of this string.
fn render_derived(
    target: &str,
    repo: &str,
    run: &RunRecord,
    issues: &[IssueRecord],
    clusters: &[ClusterRecord],
    comments: &CommentStats,
) -> String {
    let mut out = String::new();

    out.push_str("## Baseline (derived from store)\n\n");
    out.push_str("| Metric | Value | Provenance |\n");
    out.push_str("|---|---|---|\n");
    out.push_str(&format!(
        "| Open issues | {} | count(issue records, latest observation) |\n",
        issues.len()
    ));
    out.push_str(&format!(
        "| Comment events | {} | count(comment_event) — {} issues touched |\n",
        comments.total, comments.issues_with_comments
    ));
    out.push_str(&format!(
        "| Maintainer comments | {} | actor_role=maintainer |\n",
        comments.maintainer
    ));
    out.push_str(&format!(
        "| Measured comments | {} | actor_role=measured |\n",
        comments.measured
    ));
    out.push_str(&format!(
        "| Other comments | {} | actor_role=other |\n",
        comments.other
    ));
    out.push_str(&format!(
        "| filed_vs_acted_gap | {} : {} | measured open : maintainer actions |\n",
        issues.len(),
        comments.maintainer
    ));
    out.push_str(&format!(
        "| Watermark | #{} | max(issue.number) |\n\n",
        run.watermark_after
    ));

    out.push_str("### Top commenters (per-actor totals, all runs)\n\n");
    let mut actor_pairs: Vec<(&String, &u32)> = comments.per_actor.iter().collect();
    actor_pairs.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
    if actor_pairs.is_empty() {
        out.push_str("_no comments observed_\n\n");
    } else {
        for (actor, count) in actor_pairs.iter().take(10) {
            out.push_str(&format!("- `{}` — {}\n", actor, count));
        }
        out.push('\n');
    }

    out.push_str("## Clusters (with decay)\n\n");
    if clusters.is_empty() {
        out.push_str("_no clusters recorded_\n\n");
    } else {
        out.push_str("| Cluster | Members | Decay |\n");
        out.push_str("|---|---|---|\n");
        for c in clusters {
            let members = if c.members.len() > 8 {
                let head: Vec<String> = c.members.iter().take(6).map(|n| format!("#{}", n)).collect();
                format!(
                    "{} … (+{} more)",
                    head.join(", "),
                    c.members.len() - 6
                )
            } else {
                c.members
                    .iter()
                    .map(|n| format!("#{}", n))
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            out.push_str(&format!(
                "| `{}` | {} | {} |\n",
                c.name,
                members,
                decay_display(c, run.run)
            ));
        }
        out.push('\n');
    }

    out.push_str(&format!(
        "## Open issues — {} observations (most-recent first)\n\n",
        issues.len()
    ));
    if issues.is_empty() {
        out.push_str("_no issue observations in store_\n\n");
    } else {
        out.push_str("<details><summary>Expand full list</summary>\n\n");
        out.push_str("| # | Title | Updated | Author | Labels |\n");
        out.push_str("|---|---|---|---|---|\n");
        for i in issues {
            let labels = if i.labels.is_empty() {
                "—".to_string()
            } else {
                i.labels.join(", ")
            };
            out.push_str(&format!(
                "| [#{}](https://github.com/{}/issues/{}) | {} | {} | @{} | {} |\n",
                i.number,
                repo,
                i.number,
                md_escape(&i.title),
                short_date(&i.updated_at),
                i.author,
                md_escape(&labels),
            ));
        }
        out.push_str("\n</details>\n\n");
    }

    out.push_str(&format!(
        "_derived at {}Z · run {} · target `{}` · repo `{}`_\n",
        OffsetDateTime::now_utc()
            .format(&Rfc3339)
            .unwrap_or_default()
            .trim_end_matches('Z')
            .split('.')
            .next()
            .unwrap_or(""),
        run.run,
        target,
        repo
    ));

    out
}

/// Compose the final body: header + sentinel + editor slots + derived zone.
fn compose(
    target: &str,
    repo: &str,
    run: &RunRecord,
    derived: &str,
    derived_digest: &str,
) -> String {
    let sentinel = build_sentinel(target, run, derived_digest);
    let mut out = String::new();
    out.push_str(&format!(
        "# 📋 beadle — Triage Dashboard · {}\n\n",
        repo
    ));
    out.push_str(&sentinel);
    out.push_str("\n\n");
    out.push_str(EDITOR_SLOT_DIRECTION);
    out.push_str("\n\n");
    out.push_str(derived);
    out.push_str("\n");
    out.push_str(EDITOR_SLOT_NOTES);
    out.push_str("\n");
    out
}

fn build_sentinel(target: &str, run: &RunRecord, derived_digest: &str) -> String {
    // Body digest is computed over the whole final body during push;
    // here we emit `derived_digest` and a placeholder that push replaces.
    let payload = serde_json::json!({
        "schema": 1,
        "target": target,
        "run": run.run,
        "watermark": run.watermark_after,
        "counts": run.counts,
        "derived_digest": derived_digest,
        "body_digest": "pending",
        "intent_version": run.intent_version,
        "warmup": run.warmup,
        "generated_at": OffsetDateTime::now_utc()
            .format(&Rfc3339)
            .unwrap_or_default(),
    });
    format!(
        "<!-- beadle-state:v1\n{}\nbeadle-state -->",
        serde_json::to_string_pretty(&payload).unwrap_or_default()
    )
}

/// Editor slots. The `push` command extracts existing slot contents from the
/// live issue body and re-inserts them here before writing.
pub const EDITOR_SLOT_DIRECTION: &str = "<!-- editor:direction-verdict -->\n\
_Editor: fill in the direction verdict paragraph. Renderer only surfaces the numbers; \
the interpretation is yours. The sentinel's derived_digest tracks whether any of these \
numbers changed since the last run._\n\
<!-- /editor:direction-verdict -->";

pub const EDITOR_SLOT_NOTES: &str = "## Editor notes\n\n\
<!-- editor:notes -->\n\
_Editor: free-form notes, per-issue verdict chips, quick-win analysis, escalations, \
whatever the compass tells you to say this run._\n\
<!-- /editor:notes -->";

fn md_escape(s: &str) -> String {
    s.replace('|', "\\|").replace('\n', " ")
}

fn short_date(iso: &str) -> String {
    // "2026-07-01T15:56:36Z" -> "2026-07-01"
    iso.split('T').next().unwrap_or(iso).to_string()
}

fn sha256_hex(s: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(s.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decay_progresses() {
        // Cluster last added in run 1. gap = current_run - last_added_run.
        // Thresholds: warming≥3, rollup≥5, archived≥8.
        let c = ClusterRecord {
            ts: "2026-07-01T00:00:00Z".into(),
            target: "t".into(),
            name: "n".into(),
            run: 1,
            description: None,
            members: vec![1],
            last_added_run: 1,
            decay: "active".into(),
        };
        assert!(decay_display(&c, 1).starts_with("active"), "gap=0");
        assert!(decay_display(&c, 3).starts_with("active"), "gap=2 still active");
        assert!(decay_display(&c, 4).starts_with("warming"), "gap=3 warming");
        assert!(decay_display(&c, 6).starts_with("rollup-candidate"), "gap=5");
        assert!(decay_display(&c, 9).starts_with("archived"), "gap=8");
    }

    #[test]
    fn latest_observation_wins() {
        let a = IssueRecord {
            ts: "2026-07-01T00:00:00Z".into(),
            target: "t".into(),
            number: 100,
            observed_in_run: 1,
            title: "old".into(),
            author: "x".into(),
            state: "open".into(),
            created_at: "2026-07-01T00:00:00Z".into(),
            updated_at: "2026-07-01T00:00:00Z".into(),
            closed_at: None,
            labels: vec![],
            assignees: vec![],
            body_len: 0,
            body_sha256: "s".into(),
        };
        let mut b = a.clone();
        b.observed_in_run = 2;
        b.title = "new".into();
        let latest =
            latest_issue_observations(&[Record::Issue(a.clone()), Record::Issue(b.clone())]);
        assert_eq!(latest.len(), 1);
        assert_eq!(latest[0].title, "new");
    }
}
