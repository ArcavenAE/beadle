//! `beadle direction <target>` — compute the run's direction verdict.
//!
//! Phase-4 first cut per `question-direction-verdict-escalation`. Three
//! signals are named in the frontier question:
//!   A. filing-density derivative      — computable today from run records
//!   B. integrity-density derivative   — needs classification records
//!   C. silent-data-loss share         — needs classification records
//!
//! B and C consume `ClassificationRecord` rows for the current run when
//! they exist; otherwise they emit `pending` with a specific reason. The
//! verdict is `max` across whichever signals are live (drift on any axis
//! is drift; pending signals never dominate). Emitted to stdout as JSON
//! and optionally appended to the store as a `note` row
//! (topic=`direction-verdict`) so the audit trail exists — the editor
//! slot still owns the free-text paragraph per
//! `question-renderer-editorial-boundary`.

use anyhow::Result;
use beadle_store::{ClassificationRecord, NoteRecord, Record, RunRecord, Store};
use serde::Serialize;
use std::path::Path;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

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

pub fn run(root: &Path, target: &str, write_note: bool) -> Result<()> {
    let _intent = intent::load(root, target)?;
    let store = Store::open(root.join("store"), target)?;
    let records = store.read_all()?;

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

    let filing = filing_density_signal(&runs);
    let integrity = integrity_density_signal(&classifications_this_run, latest_run);
    let sdl = silent_data_loss_signal(&classifications_this_run, latest_run);

    let live_verdicts = vec![
        verdict_from(filing.verdict),
        live_verdict(&integrity),
        live_verdict(&sdl),
    ];
    let overall = max_verdict(&live_verdicts);

    let top = top_signal(&filing, &integrity, &sdl, overall);

    let report = DirectionReport {
        target: target.to_string(),
        run: latest_run,
        verdict: overall.label(),
        top_signal: top.clone(),
        signals: Signals {
            filing_density: filing,
            integrity_density: integrity,
            silent_data_loss_share: sdl,
        },
    };

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
            text: format!("verdict={} top_signal={}", overall.label(), top),
        };
        store.append(&[Record::Note(note)])?;
        eprintln!(
            "beadle direction: appended note record (topic=direction-verdict, verdict={})",
            overall.label()
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

fn integrity_density_signal(
    classifications: &[ClassificationRecord],
    run: u32,
) -> SignalOrPending {
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

fn silent_data_loss_signal(
    classifications: &[ClassificationRecord],
    run: u32,
) -> SignalOrPending {
    if classifications.is_empty() {
        return SignalOrPending::Pending(PendingSignal {
            verdict: "pending",
            reason: format!("no classification records for run {}", run),
        });
    }
    let denom = classifications.len() as u32;
    let num = classifications
        .iter()
        .filter(|c| {
            c.operational_impact
                .as_deref()
                .map(|s| s == "silent-data-loss")
                .unwrap_or(false)
        })
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
    overall: Verdict,
) -> String {
    let filing_v = verdict_from(filing.verdict);
    let integrity_v = live_verdict(integrity);
    let sdl_v = live_verdict(sdl);
    let target = overall;

    if filing_v == target {
        return format!("filing-density {}: {}", filing.verdict, filing.rationale);
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
                Some("silent-data-loss".into())
            } else {
                None
            },
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
        let mut classes = vec![
            mk_class(1, 1, false, true),
            mk_class(2, 1, false, true),
        ];
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
