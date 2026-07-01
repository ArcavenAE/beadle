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
use beadle_store::{ClassificationRecord, ClusterRecord, IssueRecord, Record, RunRecord, Store};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap};
use std::path::Path;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

use crate::direction::{self, DirectionReport};
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
    let latest_class = latest_classifications(&records);
    let class_summary = classification_summary(&latest_class);
    let direction_report = direction::compute(target, &records);

    let derived = render_derived(
        target,
        &intent.repo,
        &latest_run,
        &latest_issues,
        &latest_clusters,
        &comment_stats,
        &latest_class,
        &class_summary,
        &direction_report,
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

/// For each issue number, the most-recent `ClassificationRecord`. Later
/// classifications supersede earlier ones for the same issue — the skill is
/// allowed to reclassify as new evidence arrives.
fn latest_classifications(records: &[Record]) -> HashMap<u32, ClassificationRecord> {
    let mut by_number: HashMap<u32, ClassificationRecord> = HashMap::new();
    for rec in records {
        if let Record::Classification(c) = rec {
            let keep = by_number
                .get(&c.number)
                .map(|prev| prev.run <= c.run || prev.ts <= c.ts)
                .unwrap_or(true);
            if keep {
                by_number.insert(c.number, (**c).clone());
            }
        }
    }
    by_number
}

#[derive(Debug, Default, Clone)]
struct ClassificationSummary {
    total: u32,
    by_report_type: BTreeMap<String, u32>,
    integrity: u32,
    silent_data_loss: u32,
    quick_win_eligible: u32,
    p0: u32,
    p1: u32,
}

fn classification_summary(latest: &HashMap<u32, ClassificationRecord>) -> ClassificationSummary {
    let mut s = ClassificationSummary::default();
    for c in latest.values() {
        s.total += 1;
        *s.by_report_type.entry(c.report_type.clone()).or_insert(0) += 1;
        if c.integrity {
            s.integrity += 1;
        }
        if c.operational_impact.as_deref() == Some("silent-data-loss") {
            s.silent_data_loss += 1;
        }
        if c.quick_win_eligible {
            s.quick_win_eligible += 1;
        }
        match c.priority.as_str() {
            "P0" => s.p0 += 1,
            "P1" => s.p1 += 1,
            _ => {}
        }
    }
    s
}

/// Compact chip: `bug · logic · P1 ⚠★` — report_type · defect_nature ·
/// priority + flag glyphs. Glyphs: `⚠` integrity, `▲` silent-data-loss,
/// `★` quick-win-eligible.
fn classification_chip(c: &ClassificationRecord) -> String {
    let mut flags = String::new();
    if c.integrity {
        flags.push('⚠');
    }
    if c.operational_impact.as_deref() == Some("silent-data-loss") {
        flags.push('▲');
    }
    if c.quick_win_eligible {
        flags.push('★');
    }
    let flag_seg = if flags.is_empty() {
        String::new()
    } else {
        format!(" {}", flags)
    };
    format!(
        "{} · {} · {}{}",
        c.report_type, c.defect_nature, c.priority, flag_seg
    )
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
fn verdict_glyph(verdict: &str) -> &'static str {
    match verdict {
        "drifting" => "🔴",
        "watch" => "🟡",
        "on-course" => "🟢",
        _ => "⚪",
    }
}

/// Render the derived-zone Direction verdict block. Deterministic projection
/// of `DirectionReport` — the free-text paragraph is still an editor slot
/// below. See `question-renderer-editorial-boundary` sub-question A: this is
/// a *whole section* the renderer owns (the numbers), sitting above the
/// editor's *whole section* (the prose rationale).
fn render_direction_block(d: &DirectionReport) -> String {
    use crate::direction::{SignalOrPending, ZeroEngagementOrPending};
    let mut out = String::new();

    out.push_str(&format!(
        "**{} {}** — run {} — top signal: {}\n\n",
        verdict_glyph(d.verdict),
        d.verdict,
        d.run,
        d.top_signal,
    ));

    out.push_str("| Signal | Verdict | Detail |\n|---|---|---|\n");

    let f = &d.signals.filing_density;
    out.push_str(&format!(
        "| filing-density | {} {} | {} |\n",
        verdict_glyph(f.verdict),
        f.verdict,
        md_escape(&f.rationale),
    ));

    let (iv, idetail) = match &d.signals.integrity_density {
        SignalOrPending::Live(s) => (s.verdict, s.rationale.clone()),
        SignalOrPending::Pending(p) => (p.verdict, format!("pending — {}", p.reason)),
    };
    out.push_str(&format!(
        "| integrity-density (B) | {} {} | {} |\n",
        verdict_glyph(iv),
        iv,
        md_escape(&idetail),
    ));

    let (sv, sdetail) = match &d.signals.silent_data_loss_share {
        SignalOrPending::Live(s) => (s.verdict, s.rationale.clone()),
        SignalOrPending::Pending(p) => (p.verdict, format!("pending — {}", p.reason)),
    };
    out.push_str(&format!(
        "| silent-data-loss-share (C) | {} {} | {} |\n",
        verdict_glyph(sv),
        sv,
        md_escape(&sdetail),
    ));

    let (zv, zdetail) = match &d.signals.silent_data_loss_zero_engagement {
        ZeroEngagementOrPending::Live(s) => {
            let mut detail = s.rationale.clone();
            if !s.drifting_issues.is_empty() {
                let names: Vec<String> =
                    s.drifting_issues.iter().map(|n| format!("#{}", n)).collect();
                detail.push_str(&format!(" · drifting: {}", names.join(", ")));
            } else if !s.watch_only_issues.is_empty() {
                let names: Vec<String> = s
                    .watch_only_issues
                    .iter()
                    .map(|n| format!("#{}", n))
                    .collect();
                detail.push_str(&format!(" · watch: {}", names.join(", ")));
            }
            (s.verdict, detail)
        }
        ZeroEngagementOrPending::Pending(p) => (p.verdict, format!("pending — {}", p.reason)),
    };
    out.push_str(&format!(
        "| silent-data-loss-zero-engagement (A4) | {} {} | {} |\n\n",
        verdict_glyph(zv),
        zv,
        md_escape(&zdetail),
    ));

    out
}

#[allow(clippy::too_many_arguments)]
fn render_derived(
    target: &str,
    repo: &str,
    run: &RunRecord,
    issues: &[IssueRecord],
    clusters: &[ClusterRecord],
    comments: &CommentStats,
    classifications: &HashMap<u32, ClassificationRecord>,
    class_summary: &ClassificationSummary,
    direction: &DirectionReport,
) -> String {
    let mut out = String::new();

    out.push_str("## Direction verdict\n\n");
    out.push_str(&render_direction_block(direction));
    out.push('\n');

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

    out.push_str("## Classification summary\n\n");
    if class_summary.total == 0 {
        out.push_str("_no classifications in store — signals B/C emit `pending` until the classifier skill produces records for this run_\n\n");
    } else {
        out.push_str(&format!(
            "| Metric | Value | Notes |\n|---|---|---|\n\
             | Classified issues | {} | of {} observed |\n\
             | Integrity (⚠) | {} | HARD: requires `integrity_anchor` |\n\
             | Silent-data-loss (▲) | {} | operational_impact axis |\n\
             | Quick-win eligible (★) | {} | HARD: never on integrity=true |\n\
             | P0 / P1 | {} / {} | priority axis |\n\n",
            class_summary.total,
            issues.len(),
            class_summary.integrity,
            class_summary.silent_data_loss,
            class_summary.quick_win_eligible,
            class_summary.p0,
            class_summary.p1,
        ));
        if !class_summary.by_report_type.is_empty() {
            out.push_str("| Report type | Count |\n|---|---|\n");
            let mut pairs: Vec<(&String, &u32)> =
                class_summary.by_report_type.iter().collect();
            pairs.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
            for (rt, n) in pairs {
                out.push_str(&format!("| `{}` | {} |\n", rt, n));
            }
            out.push('\n');
        }
    }

    out.push_str(&format!(
        "## Open issues — {} observations (most-recent first)\n\n",
        issues.len()
    ));
    if issues.is_empty() {
        out.push_str("_no issue observations in store_\n\n");
    } else {
        out.push_str("<details><summary>Expand full list</summary>\n\n");
        out.push_str("| # | Title | Classification | Updated | Author | Labels |\n");
        out.push_str("|---|---|---|---|---|---|\n");
        for i in issues {
            let labels = if i.labels.is_empty() {
                "—".to_string()
            } else {
                i.labels.join(", ")
            };
            let chip = classifications
                .get(&i.number)
                .map(classification_chip)
                .unwrap_or_else(|| "_unclassified_".to_string());
            out.push_str(&format!(
                "| [#{}](https://github.com/{}/issues/{}) | {} | {} | {} | @{} | {} |\n",
                i.number,
                repo,
                i.number,
                md_escape(&i.title),
                md_escape(&chip),
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

    fn mk_class(number: u32, run: u32, ts: &str) -> ClassificationRecord {
        ClassificationRecord {
            ts: ts.into(),
            target: "t".into(),
            number,
            run,
            report_type: "bug".into(),
            defect_nature: "logic".into(),
            reproducibility: "bohrbug".into(),
            leverage: "minutiae".into(),
            alignment: "advances".into(),
            provenance: "pilot-derived".into(),
            integrity: false,
            integrity_anchor: None,
            operational_impact: None,
            priority: "P2".into(),
            cluster: vec![],
            quick_win_eligible: false,
            rationale: "r".into(),
            cited_evidence: None,
            quick_win_disqualification: None,
        }
    }

    #[test]
    fn latest_classification_supersedes_earlier() {
        let a = mk_class(42, 1, "2026-07-01T00:00:00Z");
        let mut b = mk_class(42, 2, "2026-07-02T00:00:00Z");
        b.priority = "P0".into();
        b.integrity = true;
        b.integrity_anchor = Some("spec_process".into());
        let latest = latest_classifications(&[
            Record::Classification(Box::new(a.clone())),
            Record::Classification(Box::new(b.clone())),
        ]);
        assert_eq!(latest.len(), 1);
        let got = latest.get(&42).unwrap();
        assert_eq!(got.priority, "P0");
        assert!(got.integrity);
    }

    #[test]
    fn classification_summary_counts() {
        let mut c1 = mk_class(1, 1, "2026-07-01T00:00:00Z");
        c1.integrity = true;
        c1.integrity_anchor = Some("spec_process".into());
        c1.operational_impact = Some("silent-data-loss".into());
        c1.priority = "P0".into();

        let mut c2 = mk_class(2, 1, "2026-07-01T00:00:00Z");
        c2.quick_win_eligible = true;
        c2.priority = "P3".into();
        c2.report_type = "docs".into();

        let mut c3 = mk_class(3, 1, "2026-07-01T00:00:00Z");
        c3.priority = "P1".into();

        let latest = latest_classifications(&[
            Record::Classification(Box::new(c1)),
            Record::Classification(Box::new(c2)),
            Record::Classification(Box::new(c3)),
        ]);
        let s = classification_summary(&latest);
        assert_eq!(s.total, 3);
        assert_eq!(s.integrity, 1);
        assert_eq!(s.silent_data_loss, 1);
        assert_eq!(s.quick_win_eligible, 1);
        assert_eq!(s.p0, 1);
        assert_eq!(s.p1, 1);
        assert_eq!(s.by_report_type.get("bug").copied().unwrap_or(0), 2);
        assert_eq!(s.by_report_type.get("docs").copied().unwrap_or(0), 1);
    }

    #[test]
    fn chip_shows_flags_and_priority() {
        let plain = mk_class(1, 1, "2026-07-01T00:00:00Z");
        assert_eq!(classification_chip(&plain), "bug · logic · P2");

        let mut integrity = plain.clone();
        integrity.integrity = true;
        integrity.integrity_anchor = Some("spec_process".into());
        integrity.priority = "P0".into();
        assert_eq!(classification_chip(&integrity), "bug · logic · P0 ⚠");

        let mut sdl_qw = plain.clone();
        sdl_qw.operational_impact = Some("silent-data-loss".into());
        sdl_qw.quick_win_eligible = true;
        assert_eq!(classification_chip(&sdl_qw), "bug · logic · P2 ▲★");
    }

    #[test]
    fn render_includes_classification_zones() {
        let issue = IssueRecord {
            ts: "2026-07-01T00:00:00Z".into(),
            target: "t".into(),
            number: 42,
            observed_in_run: 1,
            title: "wobble".into(),
            author: "alice".into(),
            state: "open".into(),
            created_at: "2026-07-01T00:00:00Z".into(),
            updated_at: "2026-07-01T00:00:00Z".into(),
            closed_at: None,
            labels: vec![],
            assignees: vec![],
            body_len: 0,
            body_sha256: "s".into(),
        };
        let mut c = mk_class(42, 1, "2026-07-01T00:00:00Z");
        c.integrity = true;
        c.integrity_anchor = Some("spec_process".into());
        c.priority = "P0".into();

        let run = RunRecord {
            ts: "2026-07-01T00:00:00Z".into(),
            target: "t".into(),
            run: 1,
            watermark_before: 0,
            watermark_after: 42,
            counts: Default::default(),
            digest: "d".into(),
            warmup: None,
            intent_version: None,
            new_this_run: vec![],
            notes: None,
        };
        let mut latest_class = HashMap::new();
        latest_class.insert(42, c);
        let summary = classification_summary(&latest_class);
        let direction = mk_direction_pending();
        let body = render_derived(
            "t",
            "acme/widget",
            &run,
            &[issue],
            &[],
            &CommentStats::default(),
            &latest_class,
            &summary,
            &direction,
        );
        assert!(body.contains("## Classification summary"), "summary header");
        assert!(body.contains("| Integrity"), "integrity row");
        assert!(body.contains("bug · logic · P0 ⚠"), "chip in row");
        assert!(body.contains("Classification"), "chip column header");
    }

    #[test]
    fn render_pending_when_no_classifications() {
        let run = RunRecord {
            ts: "2026-07-01T00:00:00Z".into(),
            target: "t".into(),
            run: 1,
            watermark_before: 0,
            watermark_after: 0,
            counts: Default::default(),
            digest: "d".into(),
            warmup: None,
            intent_version: None,
            new_this_run: vec![],
            notes: None,
        };
        let empty: HashMap<u32, ClassificationRecord> = HashMap::new();
        let summary = classification_summary(&empty);
        let direction = mk_direction_pending();
        let body = render_derived(
            "t",
            "acme/widget",
            &run,
            &[],
            &[],
            &CommentStats::default(),
            &empty,
            &summary,
            &direction,
        );
        assert!(
            body.contains("no classifications in store"),
            "pending disclosure must be surfaced"
        );
    }

    fn mk_direction_pending() -> DirectionReport {
        use crate::direction::{
            FilingDensity, PendingSignal, ShareSignal, Signals, SignalOrPending,
            ZeroEngagementOrPending,
        };
        let _ = ShareSignal {
            classified_this_run: 0,
            numerator: 0,
            share_pct: 0.0,
            verdict: "on-course",
            rationale: String::new(),
        };
        DirectionReport {
            target: "t".into(),
            run: 1,
            verdict: "on-course",
            top_signal: "filing-density on-course: no history".into(),
            signals: Signals {
                filing_density: FilingDensity {
                    current_run_new: 0,
                    trailing_3_mean: 0.0,
                    rise_pct_vs_trailing: None,
                    verdict: "on-course",
                    rationale: "no runs recorded".into(),
                },
                integrity_density: SignalOrPending::Pending(PendingSignal {
                    verdict: "pending",
                    reason: "no classification records".into(),
                }),
                silent_data_loss_share: SignalOrPending::Pending(PendingSignal {
                    verdict: "pending",
                    reason: "no classification records".into(),
                }),
                silent_data_loss_zero_engagement: ZeroEngagementOrPending::Pending(
                    PendingSignal {
                        verdict: "pending",
                        reason: "no silent-data-loss classifications".into(),
                    },
                ),
            },
        }
    }

    #[test]
    fn render_includes_direction_verdict_block() {
        let run = RunRecord {
            ts: "2026-07-01T00:00:00Z".into(),
            target: "t".into(),
            run: 1,
            watermark_before: 0,
            watermark_after: 0,
            counts: Default::default(),
            digest: "d".into(),
            warmup: None,
            intent_version: None,
            new_this_run: vec![],
            notes: None,
        };
        let empty: HashMap<u32, ClassificationRecord> = HashMap::new();
        let summary = classification_summary(&empty);
        let direction = mk_direction_pending();
        let body = render_derived(
            "t",
            "acme/widget",
            &run,
            &[],
            &[],
            &CommentStats::default(),
            &empty,
            &summary,
            &direction,
        );
        assert!(body.contains("## Direction verdict"), "verdict header");
        assert!(body.contains("🟢 on-course"), "verdict glyph + label");
        assert!(body.contains("| filing-density |"), "filing-density row");
        assert!(
            body.contains("| integrity-density (B) |"),
            "integrity-density row"
        );
        assert!(
            body.contains("| silent-data-loss-share (C) |"),
            "SDL share row"
        );
        assert!(
            body.contains("| silent-data-loss-zero-engagement (A4) |"),
            "A4 row"
        );
        assert!(body.contains("pending — "), "pending signals render their reason");
    }

    #[test]
    fn render_direction_drifting_names_issues() {
        use crate::direction::{
            FilingDensity, PendingSignal, Signals, SignalOrPending,
            ZeroEngagementOrPending, ZeroEngagementSignal,
        };
        let direction = DirectionReport {
            target: "t".into(),
            run: 3,
            verdict: "drifting",
            top_signal:
                "silent-data-loss-zero-engagement drifting: 1 SDL issue(s) with ≥ 3-run silence"
                    .into(),
            signals: Signals {
                filing_density: FilingDensity {
                    current_run_new: 0,
                    trailing_3_mean: 0.0,
                    rise_pct_vs_trailing: None,
                    verdict: "on-course",
                    rationale: "n/a".into(),
                },
                integrity_density: SignalOrPending::Pending(PendingSignal {
                    verdict: "pending",
                    reason: "none".into(),
                }),
                silent_data_loss_share: SignalOrPending::Pending(PendingSignal {
                    verdict: "pending",
                    reason: "none".into(),
                }),
                silent_data_loss_zero_engagement: ZeroEngagementOrPending::Live(
                    ZeroEngagementSignal {
                        sdl_issues: 1,
                        watch_count: 1,
                        drifting_count: 1,
                        drifting_issues: vec![42],
                        watch_only_issues: vec![],
                        longest_streak: 3,
                        verdict: "drifting",
                        rationale:
                            "1 SDL issue(s) with ≥ 3-run silence + zero maintainer engagement"
                                .into(),
                    },
                ),
            },
        };
        let block = render_direction_block(&direction);
        assert!(block.contains("🔴 drifting"), "drifting glyph");
        assert!(block.contains("drifting: #42"), "issue named in detail");
        assert!(
            block.contains("top signal: silent-data-loss-zero-engagement"),
            "top signal surfaced"
        );
    }
}
