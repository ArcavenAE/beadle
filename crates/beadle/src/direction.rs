//! `beadle direction <target>` — compute the run's direction verdict.
//!
//! Phase-4 signals per `question-direction-verdict-escalation`:
//!   A.  filing-density derivative        — computable from run records
//!   B.  integrity-density derivative     — needs classification records
//!   C.  silent-data-loss share           — needs classification records
//!   A4. silent-data-loss zero-engagement — joins ClassificationRecord ×
//!                                          CommentEventRecord across runs
//!
//! B, C, and A4 consume `ClassificationRecord` rows; A4 also joins against
//! `CommentEventRecord`s. Signals emit `pending` with a specific reason
//! when the data they need is not present. The verdict is `max` across
//! whichever signals are live (drift on any axis is drift; pending signals
//! never dominate). Emitted to stdout as JSON and optionally appended to
//! the store as a `note` row (topic=`direction-verdict`) so the audit
//! trail exists — the editor slot still owns the free-text paragraph per
//! `question-renderer-editorial-boundary`.

use std::{
    collections::{BTreeSet, HashMap},
    path::Path,
};

use anyhow::Result;
use beadle_store::{
    ClassificationRecord, CommentEventRecord, NoteRecord, Record, RunRecord, Store,
};
use serde::Serialize;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use crate::intent;

/// Filing-density-derivative thresholds (frontier candidates — recalibrate
/// once ≥ 2 targets have ≥ 3 runs of real data).
const WATCH_RATE_RISE_PCT: f64 = 25.0;
const DRIFTING_RATE_RISE_PCT: f64 = 50.0;
const WATCH_ABSOLUTE_MULTIPLIER: f64 = 2.0;

/// Integrity-density-derivative thresholds — share of new-in-run issues that
/// carry `integrity=true`. Sourced from `question-direction-verdict-escalation`.
const INTEGRITY_WATCH_PCT: f64 = 15.0;
const INTEGRITY_DRIFTING_PCT: f64 = 30.0;

/// Silent-data-loss share thresholds — count of `operational_impact=silent-data-loss`
/// as a fraction of the current run's classified issues.
const SDL_WATCH_PCT: f64 = 5.0;
const SDL_DRIFTING_PCT: f64 = 10.0;

/// Silent-data-loss zero-engagement alarm thresholds — consecutive-run streaks
/// (including the current run) with an SDL classification and zero maintainer
/// engagement across all observed runs of the issue. The frontier question
/// names "3+ runs" as the harshest escalation.
const SDL_ZERO_ENGAGEMENT_WATCH_STREAK: u32 = 2;
const SDL_ZERO_ENGAGEMENT_DRIFTING_STREAK: u32 = 3;

#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Verdict {
    OnCourse,
    Watch,
    Drifting,
}

impl Verdict {
    fn label(self) -> &'static str {
        match self {
            Verdict::OnCourse => "on-course",
            Verdict::Watch => "watch",
            Verdict::Drifting => "drifting",
        }
    }
}

#[derive(Debug, Serialize)]
pub struct DirectionReport {
    pub target: String,
    pub run: u32,
    pub verdict: &'static str,
    pub top_signal: String,
    pub signals: Signals,
}

#[derive(Debug, Serialize)]
pub struct Signals {
    pub filing_density: FilingDensity,
    pub integrity_density: SignalOrPending,
    pub silent_data_loss_share: SignalOrPending,
    pub silent_data_loss_zero_engagement: ZeroEngagementOrPending,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ZeroEngagementOrPending {
    Live(ZeroEngagementSignal),
    Pending(PendingSignal),
}

#[derive(Debug, Serialize)]
pub struct ZeroEngagementSignal {
    /// Total SDL-classified issues currently in the store.
    pub sdl_issues: u32,
    /// Count of SDL issues with a streak ≥ watch threshold.
    pub watch_count: u32,
    /// Count of SDL issues with a streak ≥ drifting threshold.
    pub drifting_count: u32,
    /// Issue numbers in the drifting set (up to 20 for report compactness).
    pub drifting_issues: Vec<u32>,
    /// Issue numbers in the watch-only set (up to 20).
    pub watch_only_issues: Vec<u32>,
    /// Longest streak observed on any SDL issue this run.
    pub longest_streak: u32,
    pub verdict: &'static str,
    pub rationale: String,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum SignalOrPending {
    Live(ShareSignal),
    Pending(PendingSignal),
}

#[derive(Debug, Serialize)]
pub struct FilingDensity {
    pub current_run_new: u32,
    pub trailing_3_mean: f64,
    pub rise_pct_vs_trailing: Option<f64>,
    pub verdict: &'static str,
    pub rationale: String,
}

#[derive(Debug, Serialize)]
pub struct ShareSignal {
    pub classified_this_run: u32,
    pub numerator: u32,
    pub share_pct: f64,
    pub verdict: &'static str,
    pub rationale: String,
}

#[derive(Debug, Serialize)]
pub struct PendingSignal {
    pub verdict: &'static str,
    pub reason: String,
}

/// Compute the direction report from a set of store records. Pure —
/// called by both `run()` (which prints + optionally writes a note) and
/// `render::render_dashboard` (which surfaces the verdict in the derived
/// zone). Extracting the computation is the render→direction bridge.
pub fn compute(target: &str, records: &[Record]) -> DirectionReport {
    let runs: Vec<RunRecord> = records
        .iter()
        .filter_map(|r| {
            if let Record::Run(rr) = r {
                Some(rr.clone())
            } else {
                None
            }
        })
        .collect();
    let latest_run = runs.last().map(|r| r.run).unwrap_or(0);

    let classifications_this_run: Vec<ClassificationRecord> = records
        .iter()
        .filter_map(|r| match r {
            Record::Classification(c) if c.run == latest_run => Some((**c).clone()),
            _ => None,
        })
        .collect();

    let all_classifications: Vec<ClassificationRecord> = records
        .iter()
        .filter_map(|r| match r {
            Record::Classification(c) => Some((**c).clone()),
            _ => None,
        })
        .collect();
    let all_comments: Vec<CommentEventRecord> = records
        .iter()
        .filter_map(|r| match r {
            Record::CommentEvent(c) => Some(c.clone()),
            _ => None,
        })
        .collect();

    let filing = filing_density_signal(&runs);
    let integrity = integrity_density_signal(&classifications_this_run, latest_run);
    let sdl = silent_data_loss_signal(&classifications_this_run, latest_run);
    let sdl_zero = sdl_zero_engagement_signal(&all_classifications, &all_comments, latest_run);

    let live_verdicts = vec![
        verdict_from(filing.verdict),
        live_verdict(&integrity),
        live_verdict(&sdl),
        live_verdict_zero(&sdl_zero),
    ];
    let overall = max_verdict(&live_verdicts);

    let top = top_signal(&filing, &integrity, &sdl, &sdl_zero, overall);

    DirectionReport {
        target: target.to_string(),
        run: latest_run,
        verdict: overall.label(),
        top_signal: top,
        signals: Signals {
            filing_density: filing,
            integrity_density: integrity,
            silent_data_loss_share: sdl,
            silent_data_loss_zero_engagement: sdl_zero,
        },
    }
}

pub fn run(root: &Path, target: &str, write_note: bool) -> Result<()> {
    let _intent = intent::load(root, target)?;
    let store = Store::open(root.join("store"), target)?;
    let records = store.read_all()?;

    let report = compute(target, &records);
    let overall_label = report.verdict;
    let top = report.top_signal.clone();
    let latest_run = report.run;

    let json = serde_json::to_string_pretty(&report)?;
    println!("{}", json);

    if write_note {
        let ts = OffsetDateTime::now_utc()
            .format(&Rfc3339)
            .unwrap_or_default();
        let note = NoteRecord {
            ts,
            target: target.to_string(),
            run: latest_run,
            topic: "direction-verdict".to_string(),
            text: format!("verdict={} top_signal={}", overall_label, top),
        };
        store.append(&[Record::Note(note)])?;
        eprintln!(
            "beadle direction: appended note record (topic=direction-verdict, verdict={})",
            overall_label
        );
    }
    Ok(())
}

fn filing_density_signal(runs: &[RunRecord]) -> FilingDensity {
    let Some(current) = runs.last() else {
        return FilingDensity {
            current_run_new: 0,
            trailing_3_mean: 0.0,
            rise_pct_vs_trailing: None,
            verdict: "on-course",
            rationale: "no runs recorded".to_string(),
        };
    };
    let current_new = current.new_this_run.len() as u32;

    let history: Vec<&RunRecord> = runs.iter().rev().skip(1).take(3).collect();
    if history.is_empty() {
        return FilingDensity {
            current_run_new: current_new,
            trailing_3_mean: 0.0,
            rise_pct_vs_trailing: None,
            verdict: "on-course",
            rationale: format!(
                "single run ({} new); no history to derive against",
                current_new
            ),
        };
    }
    let hist_sum: u32 = history.iter().map(|r| r.new_this_run.len() as u32).sum();
    let mean = hist_sum as f64 / history.len() as f64;

    let (rise, verdict, rationale) = if mean <= 0.0 {
        if current_new == 0 {
            (
                Some(0.0),
                "on-course",
                "no new issues in current or trailing runs".to_string(),
            )
        } else {
            (
                None,
                "watch",
                format!(
                    "trailing-3 mean is 0 but current run has {} new — insufficient signal, watch",
                    current_new
                ),
            )
        }
    } else {
        let rise_pct = ((current_new as f64 - mean) / mean) * 100.0;
        let absolute_mult = current_new as f64 / mean;
        let v = if rise_pct >= DRIFTING_RATE_RISE_PCT {
            "drifting"
        } else if rise_pct >= WATCH_RATE_RISE_PCT || absolute_mult >= WATCH_ABSOLUTE_MULTIPLIER {
            "watch"
        } else {
            "on-course"
        };
        let r = format!(
            "current {} new vs trailing-3 mean {:.1} → {:+.0}% ({:.1}× trailing-mean)",
            current_new, mean, rise_pct, absolute_mult
        );
        (Some(rise_pct), v, r)
    };

    FilingDensity {
        current_run_new: current_new,
        trailing_3_mean: mean,
        rise_pct_vs_trailing: rise,
        verdict,
        rationale,
    }
}

fn integrity_density_signal(classifications: &[ClassificationRecord], run: u32) -> SignalOrPending {
    if classifications.is_empty() {
        return SignalOrPending::Pending(PendingSignal {
            verdict: "pending",
            reason: format!("no classification records for run {}", run),
        });
    }
    let denom = classifications.len() as u32;
    let num = classifications.iter().filter(|c| c.integrity).count() as u32;
    let share = if denom == 0 {
        0.0
    } else {
        (num as f64 / denom as f64) * 100.0
    };
    let verdict = if share > INTEGRITY_DRIFTING_PCT {
        "drifting"
    } else if share >= INTEGRITY_WATCH_PCT {
        "watch"
    } else {
        "on-course"
    };
    SignalOrPending::Live(ShareSignal {
        classified_this_run: denom,
        numerator: num,
        share_pct: share,
        verdict,
        rationale: format!(
            "{}/{} classified issues carry integrity=true → {:.1}%",
            num, denom, share
        ),
    })
}

fn silent_data_loss_signal(classifications: &[ClassificationRecord], run: u32) -> SignalOrPending {
    if classifications.is_empty() {
        return SignalOrPending::Pending(PendingSignal {
            verdict: "pending",
            reason: format!("no classification records for run {}", run),
        });
    }
    let denom = classifications.len() as u32;
    let num = classifications
        .iter()
        .filter(|c| c.is_silent_data_loss())
        .count() as u32;
    let share = if denom == 0 {
        0.0
    } else {
        (num as f64 / denom as f64) * 100.0
    };
    let verdict = if share > SDL_DRIFTING_PCT {
        "drifting"
    } else if share >= SDL_WATCH_PCT {
        "watch"
    } else {
        "on-course"
    };
    SignalOrPending::Live(ShareSignal {
        classified_this_run: denom,
        numerator: num,
        share_pct: share,
        verdict,
        rationale: format!(
            "{}/{} classified issues flagged silent-data-loss → {:.1}%",
            num, denom, share
        ),
    })
}

/// A4 — silent-data-loss zero-engagement alarm.
///
/// Joins `ClassificationRecord` × `CommentEventRecord` across all runs in the
/// store:
///
///   - Consider every issue that has EVER been classified with
///     `operational_impact = "silent-data-loss"`. (Once flagged SDL, the risk
///     persists even if a later reclassification softens it — the alarm is
///     about *the run of silence*, not about whether SDL is the current top
///     label.)
///   - For each such issue, walk the runs it was classified SDL in, in order,
///     and find the LONGEST contiguous streak ending at `latest_run`. Streak
///     entries: runs where the latest classification observed on-or-before
///     that run tagged the issue SDL.
///   - **Escape hatch:** any `CommentEventRecord` with `actor_role="maintainer"`
///     on that issue drops it from the alarm set entirely. Distinguishes
///     "silent and neglected" from "pending human response" per finding-004.
///
/// Verdict:
///   - `pending` if no SDL classifications exist in the store, OR if
///     `latest_run < SDL_ZERO_ENGAGEMENT_WATCH_STREAK` (insufficient run
///     history to form even a 2-run streak).
///   - `drifting` if any issue's streak ≥ 3 (frontier A4 harshest clause).
///   - `watch`    if any issue's streak ≥ 2.
///   - `on-course` otherwise.
fn sdl_zero_engagement_signal(
    all_classifications: &[ClassificationRecord],
    all_comments: &[CommentEventRecord],
    latest_run: u32,
) -> ZeroEngagementOrPending {
    if !all_classifications.iter().any(|c| c.is_silent_data_loss()) {
        return ZeroEngagementOrPending::Pending(PendingSignal {
            verdict: "pending",
            reason: "no silent-data-loss classifications in store".to_string(),
        });
    }
    if latest_run < SDL_ZERO_ENGAGEMENT_WATCH_STREAK {
        return ZeroEngagementOrPending::Pending(PendingSignal {
            verdict: "pending",
            reason: format!(
                "insufficient run history (run {}, need ≥ {} for watch streak)",
                latest_run, SDL_ZERO_ENGAGEMENT_WATCH_STREAK
            ),
        });
    }

    // For each issue, collect the runs at which SDL was the observed label
    // (i.e., the latest classification observed at that run tags SDL).
    //
    // Approach: sort classifications per-issue by (run, ts). For each run r
    // an issue was ever classified in, the "effective label at run r" is the
    // last classification whose run ≤ r. If that effective label is SDL,
    // count r as an SDL-run for the issue.
    let mut per_issue_class: HashMap<u32, Vec<ClassificationRecord>> = HashMap::new();
    for c in all_classifications {
        per_issue_class.entry(c.number).or_default().push(c.clone());
    }
    for v in per_issue_class.values_mut() {
        v.sort_by(|a, b| a.run.cmp(&b.run).then_with(|| a.ts.cmp(&b.ts)));
    }

    // Set of issue numbers with any maintainer engagement (escape hatch).
    let maintainer_touched: BTreeSet<u32> = all_comments
        .iter()
        .filter(|e| e.actor_role == "maintainer")
        .map(|e| e.number)
        .collect();

    let mut drifting_issues: Vec<u32> = Vec::new();
    let mut watch_only_issues: Vec<u32> = Vec::new();
    let mut sdl_issues = 0u32;
    let mut longest_streak = 0u32;

    let mut all_numbers: Vec<u32> = per_issue_class.keys().copied().collect();
    all_numbers.sort_unstable();

    for number in all_numbers {
        let class_history = &per_issue_class[&number];
        let ever_sdl = class_history.iter().any(|c| c.is_silent_data_loss());
        if !ever_sdl {
            continue;
        }
        sdl_issues += 1;

        if maintainer_touched.contains(&number) {
            continue; // escape hatch
        }

        // Effective-label-at-run: for run r, find the last class in history
        // with run ≤ r; that's the label at r.
        let earliest_class_run = class_history.first().map(|c| c.run).unwrap_or(latest_run);
        let mut streak = 0u32;
        for r in (earliest_class_run..=latest_run).rev() {
            let effective = class_history.iter().rev().find(|c| c.run <= r);
            let is_sdl_at_r = effective.map(|c| c.is_silent_data_loss()).unwrap_or(false);
            if is_sdl_at_r {
                streak += 1;
            } else {
                break; // streak is contiguous ending at latest_run
            }
        }

        if streak > longest_streak {
            longest_streak = streak;
        }
        if streak >= SDL_ZERO_ENGAGEMENT_DRIFTING_STREAK {
            drifting_issues.push(number);
        } else if streak >= SDL_ZERO_ENGAGEMENT_WATCH_STREAK {
            watch_only_issues.push(number);
        }
    }

    let drifting_count = drifting_issues.len() as u32;
    let watch_count = drifting_count + watch_only_issues.len() as u32;

    let verdict = if drifting_count > 0 {
        "drifting"
    } else if watch_count > 0 {
        "watch"
    } else {
        "on-course"
    };

    let rationale = if drifting_count > 0 {
        format!(
            "{} SDL issue(s) with ≥ {}-run silence + zero maintainer engagement (longest streak: {} runs)",
            drifting_count, SDL_ZERO_ENGAGEMENT_DRIFTING_STREAK, longest_streak
        )
    } else if watch_count > 0 {
        format!(
            "{} SDL issue(s) with ≥ {}-run silence + zero maintainer engagement (longest streak: {} runs)",
            watch_count, SDL_ZERO_ENGAGEMENT_WATCH_STREAK, longest_streak
        )
    } else if sdl_issues == 0 {
        "no SDL-classified issues".to_string()
    } else {
        format!(
            "{} SDL issue(s) tracked; none in a zero-engagement streak of ≥ {} runs",
            sdl_issues, SDL_ZERO_ENGAGEMENT_WATCH_STREAK
        )
    };

    drifting_issues.sort_unstable();
    drifting_issues.truncate(20);
    watch_only_issues.sort_unstable();
    watch_only_issues.truncate(20);

    ZeroEngagementOrPending::Live(ZeroEngagementSignal {
        sdl_issues,
        watch_count,
        drifting_count,
        drifting_issues,
        watch_only_issues,
        longest_streak,
        verdict,
        rationale,
    })
}

fn verdict_from(label: &str) -> Verdict {
    match label {
        "drifting" => Verdict::Drifting,
        "watch" => Verdict::Watch,
        _ => Verdict::OnCourse,
    }
}

fn live_verdict(s: &SignalOrPending) -> Verdict {
    match s {
        SignalOrPending::Live(sig) => verdict_from(sig.verdict),
        SignalOrPending::Pending(_) => Verdict::OnCourse,
    }
}

fn live_verdict_zero(s: &ZeroEngagementOrPending) -> Verdict {
    match s {
        ZeroEngagementOrPending::Live(sig) => verdict_from(sig.verdict),
        ZeroEngagementOrPending::Pending(_) => Verdict::OnCourse,
    }
}

fn max_verdict(v: &[Verdict]) -> Verdict {
    let mut worst = Verdict::OnCourse;
    for x in v {
        if matches!(x, Verdict::Drifting) {
            return Verdict::Drifting;
        }
        if matches!(x, Verdict::Watch) {
            worst = Verdict::Watch;
        }
    }
    worst
}

fn top_signal(
    filing: &FilingDensity,
    integrity: &SignalOrPending,
    sdl: &SignalOrPending,
    sdl_zero: &ZeroEngagementOrPending,
    overall: Verdict,
) -> String {
    let filing_v = verdict_from(filing.verdict);
    let integrity_v = live_verdict(integrity);
    let sdl_v = live_verdict(sdl);
    let sdl_zero_v = live_verdict_zero(sdl_zero);
    let target = overall;

    // Prefer the SDL zero-engagement alarm when it matches the target — it's the
    // signal that measures silence itself, exactly what beadle exists to catch.
    if sdl_zero_v == target {
        if let ZeroEngagementOrPending::Live(s) = sdl_zero {
            return format!(
                "silent-data-loss-zero-engagement {}: {}",
                s.verdict, s.rationale
            );
        }
    }
    if integrity_v == target {
        if let SignalOrPending::Live(s) = integrity {
            return format!("integrity-density {}: {}", s.verdict, s.rationale);
        }
    }
    if sdl_v == target {
        if let SignalOrPending::Live(s) = sdl {
            return format!("silent-data-loss {}: {}", s.verdict, s.rationale);
        }
    }
    if filing_v == target {
        return format!("filing-density {}: {}", filing.verdict, filing.rationale);
    }
    format!("filing-density {}: {}", filing.verdict, filing.rationale)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_run(run: u32, new_count: usize) -> RunRecord {
        RunRecord {
            ts: format!("2026-07-{:02}T00:00:00Z", run),
            target: "t".into(),
            run,
            watermark_before: 0,
            watermark_after: 0,
            counts: Default::default(),
            digest: String::new(),
            warmup: None,
            intent_version: None,
            new_this_run: (0..new_count as u32).collect(),
            notes: None,
        }
    }

    fn mk_class(number: u32, run: u32, integrity: bool, sdl: bool) -> ClassificationRecord {
        ClassificationRecord {
            ts: "2026-07-01T00:00:00Z".into(),
            target: "t".into(),
            number,
            run,
            report_type: "bug".into(),
            defect_nature: "logic".into(),
            reproducibility: "bohrbug".into(),
            leverage: "minutiae".into(),
            alignment: "advances".into(),
            provenance: "pilot-derived".into(),
            integrity,
            integrity_anchor: if integrity {
                Some("spec_process".into())
            } else {
                None
            },
            operational_impact: if sdl {
                // Legacy pre-finding-009 encoding — deliberately kept so the
                // is_silent_data_loss() compat arm stays covered.
                Some("silent-data-loss".into())
            } else {
                None
            },
            silent_data_loss: false,
            priority: "P2".into(),
            cluster: vec![],
            quick_win_eligible: false,
            rationale: "test".into(),
            cited_evidence: None,
            quick_win_disqualification: None,
        }
    }

    #[test]
    fn on_course_when_flat() {
        let runs = vec![mk_run(1, 5), mk_run(2, 5), mk_run(3, 5), mk_run(4, 5)];
        let s = filing_density_signal(&runs);
        assert_eq!(s.verdict, "on-course", "{}", s.rationale);
    }

    #[test]
    fn watch_on_25_percent_rise() {
        let runs = vec![mk_run(1, 4), mk_run(2, 4), mk_run(3, 4), mk_run(4, 5)];
        let s = filing_density_signal(&runs);
        assert_eq!(s.verdict, "watch", "{}", s.rationale);
    }

    #[test]
    fn drifting_on_50_percent_rise() {
        let runs = vec![mk_run(1, 4), mk_run(2, 4), mk_run(3, 4), mk_run(4, 6)];
        let s = filing_density_signal(&runs);
        assert_eq!(s.verdict, "drifting", "{}", s.rationale);
    }

    #[test]
    fn watch_on_absolute_multiplier() {
        let runs = vec![mk_run(1, 2), mk_run(2, 2), mk_run(3, 2), mk_run(4, 4)];
        let s = filing_density_signal(&runs);
        assert_eq!(s.verdict, "drifting", "{}", s.rationale);
    }

    #[test]
    fn single_run_is_on_course() {
        let runs = vec![mk_run(1, 5)];
        let s = filing_density_signal(&runs);
        assert_eq!(s.verdict, "on-course", "{}", s.rationale);
    }

    #[test]
    fn empty_is_on_course() {
        let runs: Vec<RunRecord> = vec![];
        let s = filing_density_signal(&runs);
        assert_eq!(s.verdict, "on-course", "{}", s.rationale);
    }

    #[test]
    fn integrity_pending_when_no_classifications() {
        let sig = integrity_density_signal(&[], 5);
        matches!(sig, SignalOrPending::Pending(_));
    }

    #[test]
    fn integrity_on_course_below_watch() {
        let classes = vec![
            mk_class(1, 1, false, false),
            mk_class(2, 1, false, false),
            mk_class(3, 1, false, false),
            mk_class(4, 1, false, false),
            mk_class(5, 1, false, false),
            mk_class(6, 1, false, false),
            mk_class(7, 1, false, false),
            mk_class(8, 1, true, false),
        ];
        let sig = integrity_density_signal(&classes, 1);
        if let SignalOrPending::Live(s) = sig {
            assert_eq!(s.verdict, "on-course", "{}", s.rationale);
        } else {
            panic!("expected Live");
        }
    }

    #[test]
    fn integrity_watch_at_20_percent() {
        let classes = vec![
            mk_class(1, 1, true, false),
            mk_class(2, 1, false, false),
            mk_class(3, 1, false, false),
            mk_class(4, 1, false, false),
            mk_class(5, 1, false, false),
        ];
        let sig = integrity_density_signal(&classes, 1);
        if let SignalOrPending::Live(s) = sig {
            assert_eq!(s.verdict, "watch", "{}", s.rationale);
        } else {
            panic!("expected Live");
        }
    }

    #[test]
    fn integrity_drifting_at_40_percent() {
        let classes = vec![
            mk_class(1, 1, true, false),
            mk_class(2, 1, true, false),
            mk_class(3, 1, false, false),
            mk_class(4, 1, false, false),
            mk_class(5, 1, false, false),
        ];
        let sig = integrity_density_signal(&classes, 1);
        if let SignalOrPending::Live(s) = sig {
            assert_eq!(s.verdict, "drifting", "{}", s.rationale);
        } else {
            panic!("expected Live");
        }
    }

    #[test]
    fn sdl_watch_at_5_percent() {
        let mut classes = vec![mk_class(1, 1, false, true)];
        for i in 2..=20 {
            classes.push(mk_class(i, 1, false, false));
        }
        let sig = silent_data_loss_signal(&classes, 1);
        if let SignalOrPending::Live(s) = sig {
            assert_eq!(s.verdict, "watch", "{}", s.rationale);
        } else {
            panic!("expected Live");
        }
    }

    #[test]
    fn sdl_drifting_above_10_percent() {
        let mut classes = vec![mk_class(1, 1, false, true), mk_class(2, 1, false, true)];
        for i in 3..=10 {
            classes.push(mk_class(i, 1, false, false));
        }
        let sig = silent_data_loss_signal(&classes, 1);
        if let SignalOrPending::Live(s) = sig {
            assert_eq!(s.verdict, "drifting", "{}", s.rationale);
        } else {
            panic!("expected Live");
        }
    }

    fn mk_class_at(number: u32, run: u32, sdl: bool) -> ClassificationRecord {
        mk_class(number, run, false, sdl)
    }

    fn mk_comment(number: u32, run: u32, actor_role: &str) -> CommentEventRecord {
        CommentEventRecord {
            ts: format!("2026-07-{:02}T00:00:00Z", run),
            target: "t".into(),
            number,
            event: "comment".into(),
            actor: "someone".into(),
            actor_role: actor_role.into(),
            body_len: None,
            body_sha256: None,
            observed_in_run: run,
        }
    }

    #[test]
    fn sdl_zero_engagement_pending_when_no_sdl_classifications() {
        // Only non-SDL classifications exist.
        let classes = vec![mk_class(1, 2, false, false), mk_class(2, 2, true, false)];
        let sig = sdl_zero_engagement_signal(&classes, &[], 2);
        matches!(sig, ZeroEngagementOrPending::Pending(_));
    }

    #[test]
    fn sdl_zero_engagement_pending_when_insufficient_run_history() {
        // SDL exists but latest_run < watch threshold (2).
        let classes = vec![mk_class_at(1, 1, true)];
        let sig = sdl_zero_engagement_signal(&classes, &[], 1);
        matches!(sig, ZeroEngagementOrPending::Pending(_));
    }

    #[test]
    fn sdl_zero_engagement_on_course_when_maintainer_engaged() {
        // Issue #1 SDL across runs 1..=3, BUT a maintainer commented.
        let classes = vec![
            mk_class_at(1, 1, true),
            mk_class_at(1, 2, true),
            mk_class_at(1, 3, true),
        ];
        let comments = vec![mk_comment(1, 2, "maintainer")];
        let sig = sdl_zero_engagement_signal(&classes, &comments, 3);
        if let ZeroEngagementOrPending::Live(s) = sig {
            assert_eq!(s.verdict, "on-course", "{}", s.rationale);
            assert_eq!(s.sdl_issues, 1);
            assert_eq!(s.watch_count, 0);
            assert_eq!(s.drifting_count, 0);
        } else {
            panic!("expected Live");
        }
    }

    #[test]
    fn sdl_zero_engagement_watch_on_2_run_streak() {
        // Issue #1 SDL runs 1 and 2. Latest run 2. No maintainer engagement.
        let classes = vec![mk_class_at(1, 1, true), mk_class_at(1, 2, true)];
        let sig = sdl_zero_engagement_signal(&classes, &[], 2);
        if let ZeroEngagementOrPending::Live(s) = sig {
            assert_eq!(s.verdict, "watch", "{}", s.rationale);
            assert_eq!(s.watch_count, 1);
            assert_eq!(s.drifting_count, 0);
            assert_eq!(s.longest_streak, 2);
            assert_eq!(s.watch_only_issues, vec![1]);
        } else {
            panic!("expected Live");
        }
    }

    #[test]
    fn sdl_zero_engagement_drifting_on_3_run_streak() {
        // Issue #7 SDL runs 1..=3. Latest run 3. No engagement.
        let classes = vec![
            mk_class_at(7, 1, true),
            mk_class_at(7, 2, true),
            mk_class_at(7, 3, true),
        ];
        let sig = sdl_zero_engagement_signal(&classes, &[], 3);
        if let ZeroEngagementOrPending::Live(s) = sig {
            assert_eq!(s.verdict, "drifting", "{}", s.rationale);
            assert_eq!(s.drifting_count, 1);
            assert_eq!(s.longest_streak, 3);
            assert_eq!(s.drifting_issues, vec![7]);
        } else {
            panic!("expected Live");
        }
    }

    #[test]
    fn sdl_zero_engagement_user_comment_does_not_bypass_alarm() {
        // A non-maintainer comment (user, arcavenai, bot) must NOT drop the
        // issue from the alarm set — only actor_role="maintainer" is the
        // escape hatch (invariant B3: maintainer engagement is the compass).
        let classes = vec![
            mk_class_at(1, 1, true),
            mk_class_at(1, 2, true),
            mk_class_at(1, 3, true),
        ];
        let comments = vec![mk_comment(1, 2, "user"), mk_comment(1, 3, "arcavenai")];
        let sig = sdl_zero_engagement_signal(&classes, &comments, 3);
        if let ZeroEngagementOrPending::Live(s) = sig {
            assert_eq!(s.verdict, "drifting", "{}", s.rationale);
            assert_eq!(s.drifting_count, 1);
        } else {
            panic!("expected Live");
        }
    }

    #[test]
    fn sdl_zero_engagement_streak_is_contiguous_ending_at_latest_run() {
        // Issue SDL at run 1 and 3, but softened at run 2. The latest streak
        // ending at run 3 is 1 (not 2 — the run-2 non-SDL breaks contiguity).
        let classes = vec![
            mk_class_at(1, 1, true),
            mk_class_at(1, 2, false), // softened
            mk_class_at(1, 3, true),  // SDL again
        ];
        let sig = sdl_zero_engagement_signal(&classes, &[], 3);
        if let ZeroEngagementOrPending::Live(s) = sig {
            // Streak is 1 (only run 3), below watch threshold.
            assert_eq!(s.verdict, "on-course", "{}", s.rationale);
            assert_eq!(s.longest_streak, 1);
        } else {
            panic!("expected Live");
        }
    }

    #[test]
    fn max_verdict_returns_worst() {
        assert_eq!(
            max_verdict(&[Verdict::OnCourse, Verdict::Watch, Verdict::Drifting]),
            Verdict::Drifting
        );
        assert_eq!(
            max_verdict(&[Verdict::OnCourse, Verdict::Watch]),
            Verdict::Watch
        );
        assert_eq!(max_verdict(&[Verdict::OnCourse]), Verdict::OnCourse);
    }
}
