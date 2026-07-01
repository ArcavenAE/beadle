//! `beadle direction <target>` — compute the run's direction verdict.
//!
//! Phase-4 first cut per `question-direction-verdict-escalation`. Three
//! signals are named in the frontier question:
//!   A. filing-density derivative      — computable today from run records
//!   B. integrity-density derivative   — needs classification records
//!   C. silent-data-loss share         — needs classification records
//!
//! This cut computes A and marks B/C as `pending classification data`. The
//! verdict is emitted to stdout as JSON and appended to the store as a
//! `note` row (topic=`direction-verdict`) so the audit trail exists — the
//! editor slot still owns the free-text paragraph per
//! `question-renderer-editorial-boundary`.

use anyhow::Result;
use beadle_store::{NoteRecord, Record, RunRecord, Store};
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
    pub integrity_density: PendingSignal,
    pub silent_data_loss_share: PendingSignal,
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
pub struct PendingSignal {
    pub verdict: &'static str,
    pub reason: &'static str,
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

    let filing = filing_density_signal(&runs);
    let integrity = PendingSignal {
        verdict: "pending",
        reason: "classification records not yet in store",
    };
    let sdl = PendingSignal {
        verdict: "pending",
        reason: "classification records not yet in store",
    };

    let overall = max_verdict(&[verdict_from(&filing.verdict)]);
    let top = format!(
        "filing-density {}: {}",
        filing.verdict,
        filing.rationale.clone()
    );

    let latest_run = runs.last().map(|r| r.run).unwrap_or(0);
    let report = DirectionReport {
        target: target.to_string(),
        run: latest_run,
        verdict: overall.label(),
        top_signal: top,
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
            text: format!(
                "verdict={} top_signal={}",
                overall.label(),
                report.top_signal
            ),
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
            "current {} new vs trailing-3 mean {:.1} → {:+.0}% ({}× trailing-mean)",
            current_new, mean, rise_pct, format!("{:.1}", absolute_mult)
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

fn verdict_from(label: &str) -> Verdict {
    match label {
        "drifting" => Verdict::Drifting,
        "watch" => Verdict::Watch,
        _ => Verdict::OnCourse,
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

    #[test]
    fn on_course_when_flat() {
        let runs = vec![mk_run(1, 5), mk_run(2, 5), mk_run(3, 5), mk_run(4, 5)];
        let s = filing_density_signal(&runs);
        assert_eq!(s.verdict, "on-course", "{}", s.rationale);
    }

    #[test]
    fn watch_on_25_percent_rise() {
        // trailing mean = 4; current = 5 = +25%.
        let runs = vec![mk_run(1, 4), mk_run(2, 4), mk_run(3, 4), mk_run(4, 5)];
        let s = filing_density_signal(&runs);
        assert_eq!(s.verdict, "watch", "{}", s.rationale);
    }

    #[test]
    fn drifting_on_50_percent_rise() {
        // trailing mean = 4; current = 6 = +50%.
        let runs = vec![mk_run(1, 4), mk_run(2, 4), mk_run(3, 4), mk_run(4, 6)];
        let s = filing_density_signal(&runs);
        assert_eq!(s.verdict, "drifting", "{}", s.rationale);
    }

    #[test]
    fn watch_on_absolute_multiplier() {
        // trailing mean = 2; current = 4 = 2× (+100%, ≥ drifting).
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
}
