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
//!
//! # Two instruments
//!
//! The four signals above are **instrument v1**: run-denominated streaks and
//! an ungated `max`-across-signals headline. SKILL §6b defines **instrument
//! v2 (capacity-adjusted)** on top of them, conditioned on the target
//! manifest's `maintainer_capacity:` block — attention-window denominators,
//! baseline-anchored success bands, a descriptive selection depth, and a
//! two-condition gate on the red headline. A target with no capacity block
//! is scored under v1 and the output says so.
//!
//! §6b forbids re-scoring an evaluated prediction under a new instrument, so
//! the instrument travels with the verdict: every emitted report carries
//! `instrument: v1|v2`, and recorded prediction evaluations are carried
//! verbatim, never recomputed.

use std::{
    collections::{BTreeSet, HashMap},
    path::Path,
};

use anyhow::Result;
use beadle_store::{
    ClassificationRecord, CommentEventRecord, NoteRecord, Record, RunRecord, Store,
};
use serde::Serialize;
use time::{
    Date, OffsetDateTime, format_description::well_known::Rfc3339, macros::format_description,
};

use crate::intent::{self, MaintainerCapacity};

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
    let intent = intent::load(root, target)?;
    let store = Store::open(root.join("store"), target)?;
    let records = store.read_all()?;

    let outcome = compute_outcome(target, &records, intent.maintainer_capacity.as_ref());
    let instrument = outcome.instrument;
    let overall_label = outcome.report.verdict;
    let top = outcome.report.top_signal.clone();
    let latest_run = outcome.report.run;

    let json = serde_json::to_string_pretty(&outcome)?;
    println!("{}", json);

    if let Some(disclosure) = outcome.disclosure {
        eprintln!("beadle direction: {}", disclosure);
    }

    if write_note {
        let ts = OffsetDateTime::now_utc()
            .format(&Rfc3339)
            .unwrap_or_default();
        let note = NoteRecord {
            ts,
            target: target.to_string(),
            run: latest_run,
            topic: "direction-verdict".to_string(),
            // The instrument travels with the verdict (§6b) — a stored
            // verdict with no instrument cannot be read back safely.
            text: format!(
                "instrument={} verdict={} top_signal={}",
                instrument, overall_label, top
            ),
        };
        store.append(&[Record::Note(note)])?;
        eprintln!(
            "beadle direction: appended note record (topic=direction-verdict, instrument={}, verdict={})",
            instrument, overall_label
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

/// One SDL issue's zero-engagement streak, in the run-number space v1 uses.
/// Shared by the v1 signal and the v2 attention-window re-denomination so the
/// two instruments can never disagree about *which* issues are in the alarm
/// set — only about how the silence is denominated.
#[derive(Debug, Clone)]
struct SdlStreak {
    number: u32,
    /// Contiguous run streak ending at `latest_run`.
    runs: u32,
    /// Timestamp of the classification effective at the streak's first run —
    /// when this issue's current run of silence began.
    onset_ts: String,
    /// How the record earns its place in the SDL class. Members do not all
    /// earn it the same way, and the difference is worth seeing.
    basis: String,
}

/// Why a classification counts as silent-data-loss. `flag-only` records carry
/// the safety flag while their liveness axis says something else — legitimate
/// under the finding-009 split, and also where a mis-flag hides.
fn sdl_basis(c: &ClassificationRecord) -> String {
    match c.operational_impact.as_deref() {
        Some("data_loss") => "impact=data_loss".to_string(),
        Some("silent-data-loss") => "legacy impact=silent-data-loss".to_string(),
        Some(other) => format!("flag-only (operational_impact={})", other),
        None => "flag-only (operational_impact=absent)".to_string(),
    }
}

/// Build the SDL zero-engagement alarm set. Returns the total number of
/// SDL-classified issues in the store and one entry per issue still in the
/// alarm set (maintainer-engaged issues are dropped by the escape hatch).
fn sdl_streaks(
    all_classifications: &[ClassificationRecord],
    all_comments: &[CommentEventRecord],
    latest_run: u32,
) -> (u32, Vec<SdlStreak>) {
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

    let mut sdl_issues = 0u32;
    let mut streaks: Vec<SdlStreak> = Vec::new();

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
        if streak == 0 {
            continue;
        }

        // The run the current streak opened at, and the classification that
        // was effective there — its ts is when this silence began.
        let onset_run = latest_run.saturating_sub(streak - 1);
        let onset_class = class_history
            .iter()
            .rev()
            .find(|c| c.run <= onset_run)
            .or_else(|| class_history.first());

        streaks.push(SdlStreak {
            number,
            runs: streak,
            onset_ts: onset_class.map(|c| c.ts.clone()).unwrap_or_default(),
            basis: onset_class.map(sdl_basis).unwrap_or_default(),
        });
    }

    (sdl_issues, streaks)
}

/// A4 — silent-data-loss zero-engagement alarm.
///
/// Joins `ClassificationRecord` × `CommentEventRecord` across all runs in the
/// store:
///
/// - Consider every issue that has EVER been classified with
///   `operational_impact = "silent-data-loss"`. (Once flagged SDL, the risk
///   persists even if a later reclassification softens it — the alarm is
///   about *the run of silence*, not about whether SDL is the current top
///   label.)
/// - For each such issue, walk the runs it was classified SDL in, in order,
///   and find the LONGEST contiguous streak ending at `latest_run`. Streak
///   entries: runs where the latest classification observed on-or-before
///   that run tagged the issue SDL.
/// - **Escape hatch:** any `CommentEventRecord` with `actor_role="maintainer"`
///   on that issue drops it from the alarm set entirely. Distinguishes
///   "silent and neglected" from "pending human response" per finding-004.
///
/// Verdict:
/// - `pending` if no SDL classifications exist in the store, OR if
///   `latest_run < SDL_ZERO_ENGAGEMENT_WATCH_STREAK` (insufficient run
///   history to form even a 2-run streak).
/// - `drifting` if any issue's streak ≥ 3 (frontier A4 harshest clause).
/// - `watch`    if any issue's streak ≥ 2.
/// - `on-course` otherwise.
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

    let (sdl_issues, streaks) = sdl_streaks(all_classifications, all_comments, latest_run);

    let mut drifting_issues: Vec<u32> = Vec::new();
    let mut watch_only_issues: Vec<u32> = Vec::new();
    let mut longest_streak = 0u32;

    for s in &streaks {
        if s.runs > longest_streak {
            longest_streak = s.runs;
        }
        if s.runs >= SDL_ZERO_ENGAGEMENT_DRIFTING_STREAK {
            drifting_issues.push(s.number);
        } else if s.runs >= SDL_ZERO_ENGAGEMENT_WATCH_STREAK {
            watch_only_issues.push(s.number);
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

// ========================================================================
// Instrument v2 — capacity-adjusted calibration (SKILL §6b)
// ========================================================================

/// What `beadle direction` says when the target declares no capacity model.
/// Two commands giving two verdicts with neither naming its instrument is the
/// trap this removes (ArcavenAE/beadle#46).
const V1_DISCLOSURE: &str = "instrument v1 — run-denominated streaks, ungated headline. \
This is NOT the capacity-adjusted verdict the board publishes: the target manifest declares \
no `maintainer_capacity:` block, so SKILL §6b cannot be applied.";

/// Merge-latency anchors, in attention windows. GREEN is the manifest's
/// (`merged within <= 2 attention windows of PR readiness`); §6b names no
/// yellow/red split, so YELLOW runs to twice the green anchor and RED beyond.
const MERGE_LATENCY_GREEN_WINDOWS: u32 = 2;
const MERGE_LATENCY_YELLOW_MULTIPLE: u32 = 2;

/// PR-acceptance bands (§6b rule 2): GREEN ≥ 60% (human band ~68–73%),
/// YELLOW 0.40–0.60, RED < 0.40 (bot/agent band ~37–45%).
const PR_ACCEPTANCE_GREEN: f64 = 0.60;
const PR_ACCEPTANCE_RED: f64 = 0.40;

/// Engagement cadence for `episodic-side-project` (§6b rule 2): GREEN = ≥ 1
/// attention window per month, RED = 0 windows per quarter.
const CADENCE_GREEN_DAYS: i32 = 30;
const CADENCE_RED_DAYS: i32 = 90;

/// §6b rule 4(b): a ready measured-side fix PR against the silent lane that
/// has sat at least this many attention windows falsifies the
/// fix-delivery-cost explanation for that lane's silence.
const COST_FALSIFYING_WINDOWS: u32 = 2;

/// A direction verdict with the instrument that produced it. `report` is the
/// v1-shaped payload the renderer already consumes; under v2 its `verdict`
/// is the **gated** headline and its A4 rationale is window-denominated.
#[derive(Debug, Serialize)]
pub struct DirectionOutcome {
    /// `v1` | `v2`. §6b: the instrument travels with the verdict.
    pub instrument: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disclosure: Option<&'static str>,
    #[serde(flatten)]
    pub report: DirectionReport,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<CapacityReport>,
}

/// The v2 layer: everything §6b adds on top of the four v1 signals.
#[derive(Debug, Serialize)]
pub struct CapacityReport {
    pub class: String,
    pub team_size: Option<u32>,
    pub windows: WindowSet,
    pub bands: Bands,
    /// A4 streaks re-denominated in attention windows (rule 1). Runs and
    /// calendar days ride along as shadow fields.
    pub a4_windows: Vec<A4Windows>,
    pub selection_depth: SelectionDepth,
    pub red_gate: RedGate,
    /// Recorded prediction evaluations, carried verbatim — never re-scored.
    pub predictions: Vec<PredictionRecord>,
}

/// The attention-window set (rule 1): calendar days with ≥ 1 maintainer
/// action on the target.
#[derive(Debug, Serialize)]
pub struct WindowSet {
    pub unit: String,
    pub total: u32,
    /// Windows the store can see on its own — maintainer comment/merge events
    /// on corpus issues. A lower bound: actions on the maintainers' own PRs
    /// never reach the store.
    pub from_store: u32,
    /// Windows the manifest declares (`maintainer_capacity.observed_windows`).
    pub from_manifest: u32,
    pub first: Option<String>,
    pub latest: Option<String>,
    /// Observation horizon — the latest run's date. Windows after it are not
    /// yet observed and are excluded.
    pub observed_through: String,
    /// Tail of the window list, for eyeballing without printing all of it.
    pub recent: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct Bands {
    pub pr_acceptance: Band,
    pub engagement_cadence: Band,
    pub merge_latency: Band,
}

/// A baseline-anchored success band (rule 2). Always reports the band AND
/// the raw number; `pending` when the input does not exist rather than a
/// green that nothing measured.
#[derive(Debug, Serialize)]
pub struct Band {
    pub band: &'static str,
    pub raw: String,
    pub rationale: String,
}

impl Band {
    fn pending(reason: &str) -> Self {
        Band {
            band: "pending",
            raw: "n/a".to_string(),
            rationale: reason.to_string(),
        }
    }
    fn is_red(&self) -> bool {
        self.band == "red"
    }
}

#[derive(Debug, Serialize)]
pub struct A4Windows {
    pub number: u32,
    /// Primary denominator (rule 1).
    pub attention_windows: u32,
    /// Shadow — the v1 denominator.
    pub runs: u32,
    /// Shadow.
    pub calendar_days: i32,
    pub onset: String,
    pub basis: String,
}

/// Rule 3 — the acted-on severity ceiling, reported as selection depth with
/// its cost covariate. Descriptive: it carries no verdict and cannot move the
/// headline. Cost-ranked triage under scarcity is rational, not misalignment.
#[derive(Debug, Serialize)]
pub struct SelectionDepth {
    pub depth: &'static str,
    pub hard_ceiling: Option<String>,
    pub acted_this_window: u32,
    pub cost_covariate: String,
    pub rationale: String,
}

/// Rule 4 — the two-condition gate on a red headline.
#[derive(Debug, Serialize)]
pub struct RedGate {
    pub rule: &'static str,
    /// What the signals say on their own — what v1 would have published.
    pub signal_verdict: &'static str,
    pub condition_a_red_band: bool,
    pub red_bands: Vec<String>,
    pub condition_b_cost_explanation_falsified: bool,
    pub condition_b_rationale: String,
    /// Where the evidence for (b) came from. The store holds no PR records,
    /// so (b) is judged against manifest-declared data — a hand-maintained
    /// input with a drift surface. Saying so in the output is the only thing
    /// standing between a stale `fixes:` line and an unearned red headline.
    pub condition_b_evidence: &'static str,
    pub headline_capped: bool,
    pub headline: &'static str,
}

/// A prediction evaluation as the store recorded it. `rescored` is always
/// false: §6b forbids re-scoring an evaluated prediction under a new
/// instrument, so this is a read of the record, not a computation.
#[derive(Debug, Serialize)]
pub struct PredictionRecord {
    pub due_run: u32,
    pub result: String,
    pub instrument: String,
    pub rescored: bool,
    pub recorded_in_run: u32,
    pub reaffirmed_in_runs: Vec<u32>,
}

/// Compute a stamped direction outcome. With no capacity block this is v1
/// plus the disclosure; with one it is v2 — window-denominated streaks,
/// scored bands, and a rule-4-gated headline.
pub fn compute_outcome(
    target: &str,
    records: &[Record],
    capacity: Option<&MaintainerCapacity>,
) -> DirectionOutcome {
    let mut report = compute(target, records);
    let Some(cap) = capacity else {
        return DirectionOutcome {
            instrument: "v1",
            disclosure: Some(V1_DISCLOSURE),
            report,
            capacity: None,
        };
    };
    let capacity_report = apply_v2(records, cap, &mut report);
    DirectionOutcome {
        instrument: "v2",
        disclosure: None,
        report,
        capacity: Some(capacity_report),
    }
}

fn parse_date(s: &str) -> Option<Date> {
    let fmt = format_description!("[year]-[month]-[day]");
    Date::parse(s.get(..10).unwrap_or(s), fmt).ok()
}

fn day_of(ts: &str) -> String {
    ts.get(..10).unwrap_or(ts).to_string()
}

/// Count attention windows in `(after, through]`. ISO dates sort
/// chronologically, so this is a string comparison.
fn windows_between(windows: &[String], after: &str, through: &str) -> u32 {
    windows
        .iter()
        .filter(|w| w.as_str() > after && w.as_str() <= through)
        .count() as u32
}

fn days_between(from: &str, to: &str) -> i32 {
    match (parse_date(from), parse_date(to)) {
        (Some(a), Some(b)) => b.to_julian_day() - a.to_julian_day(),
        _ => 0,
    }
}

/// Severity rung, deepest first — the pre-registered ordering
/// (P2 < P1 < P0b < P0a) extended over the whole priority vocabulary.
fn rung(priority: &str) -> u8 {
    match priority {
        "P0a" => 6,
        "P0b" => 5,
        "P0" => 4,
        "P1" => 3,
        "P2" => 2,
        "P3" => 1,
        _ => 0,
    }
}

/// Build the v2 layer and fold its consequences back into the v1-shaped
/// report: the headline is gated per rule 4, and the A4 row is re-denominated
/// in attention windows per rule 1.
fn apply_v2(
    records: &[Record],
    cap: &MaintainerCapacity,
    report: &mut DirectionReport,
) -> CapacityReport {
    let runs: Vec<&RunRecord> = records
        .iter()
        .filter_map(|r| match r {
            Record::Run(rr) => Some(rr),
            _ => None,
        })
        .collect();
    let latest_run = report.run;
    let observed_through = runs
        .last()
        .map(|r| day_of(&r.ts))
        .unwrap_or_else(|| "9999-12-31".to_string());

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

    let windows = window_set(&all_comments, cap, &observed_through);

    let (_, streaks) = sdl_streaks(&all_classifications, &all_comments, latest_run);
    let a4_windows: Vec<A4Windows> = streaks
        .iter()
        .map(|s| {
            let onset = day_of(&s.onset_ts);
            A4Windows {
                number: s.number,
                attention_windows: windows_between(&windows.dates, &onset, &observed_through),
                runs: s.runs,
                calendar_days: days_between(&onset, &observed_through),
                onset,
                basis: s.basis.clone(),
            }
        })
        .collect();

    let bands = Bands {
        pr_acceptance: pr_acceptance_band(cap),
        engagement_cadence: engagement_cadence_band(&windows.dates, &observed_through),
        merge_latency: merge_latency_band(cap, &windows.dates, &observed_through),
    };

    let selection_depth = selection_depth(&all_classifications, &all_comments, latest_run, cap);

    let a4_alarm: BTreeSet<u32> = streaks
        .iter()
        .filter(|s| s.runs >= SDL_ZERO_ENGAGEMENT_WATCH_STREAK)
        .map(|s| s.number)
        .collect();
    let red_gate = red_gate(
        report.verdict,
        &bands,
        &a4_alarm,
        cap,
        &windows.dates,
        &observed_through,
    );

    // Rule 1 — the A4 row reports windows first, runs and calendar as shadow.
    redenominate_a4(report, &a4_windows);
    // Rule 4 — the headline is gated; the signal row keeps its own verdict.
    report.verdict = red_gate.headline;

    CapacityReport {
        class: cap.class.clone(),
        team_size: cap.team_size,
        windows: windows.report,
        bands,
        a4_windows,
        selection_depth,
        red_gate,
        predictions: predictions(records),
    }
}

struct Windows {
    dates: Vec<String>,
    report: WindowSet,
}

fn window_set(
    all_comments: &[CommentEventRecord],
    cap: &MaintainerCapacity,
    observed_through: &str,
) -> Windows {
    let from_store: BTreeSet<String> = all_comments
        .iter()
        .filter(|e| e.actor_role == "maintainer")
        .map(|e| day_of(&e.ts))
        .filter(|d| d.as_str() <= observed_through)
        .collect();
    let from_manifest: BTreeSet<String> = cap
        .observed_windows
        .iter()
        .map(|d| day_of(d))
        .filter(|d| d.as_str() <= observed_through)
        .collect();

    // Union. The store sees only corpus actions; the manifest carries the
    // run's full sweep, including maintainer work on their own PRs. Neither
    // is a superset of the other, so neither alone is the window set.
    let mut union: BTreeSet<String> = from_store.clone();
    union.extend(from_manifest.iter().cloned());
    let dates: Vec<String> = union.into_iter().collect();

    let recent: Vec<String> = dates.iter().rev().take(8).rev().cloned().collect();
    let report = WindowSet {
        unit: cap.attention_window.clone(),
        total: dates.len() as u32,
        from_store: from_store.len() as u32,
        from_manifest: from_manifest.len() as u32,
        first: dates.first().cloned(),
        latest: dates.last().cloned(),
        observed_through: observed_through.to_string(),
        recent,
    };
    Windows { dates, report }
}

fn pr_acceptance_band(cap: &MaintainerCapacity) -> Band {
    let Some(pr) = cap.pr_acceptance else {
        return Band::pending(
            "no PR records in the store and no `maintainer_capacity.observed.pr_acceptance` \
             in the manifest — the acceptance band has no input",
        );
    };
    if pr.decided == 0 {
        return Band::pending("no measured-side PR decisions yet");
    }
    let ratio = f64::from(pr.merged) / f64::from(pr.decided);
    let band = if ratio >= PR_ACCEPTANCE_GREEN {
        "green"
    } else if ratio < PR_ACCEPTANCE_RED {
        "red"
    } else {
        "yellow"
    };
    Band {
        band,
        raw: format!("{}/{} = {:.1}%", pr.merged, pr.decided, ratio * 100.0),
        rationale: format!(
            "green ≥ {:.0}% (human band ~68–73%), red < {:.0}% (bot/agent band ~37–45%)",
            PR_ACCEPTANCE_GREEN * 100.0,
            PR_ACCEPTANCE_RED * 100.0
        ),
    }
}

fn engagement_cadence_band(windows: &[String], observed_through: &str) -> Band {
    if windows.is_empty() {
        return Band {
            band: "red",
            raw: "0 windows".to_string(),
            rationale: "no maintainer action on record".to_string(),
        };
    }
    let green_cut = shift_days(observed_through, -CADENCE_GREEN_DAYS);
    let red_cut = shift_days(observed_through, -CADENCE_RED_DAYS);
    let in_month = windows_between(windows, &green_cut, observed_through);
    let in_quarter = windows_between(windows, &red_cut, observed_through);

    let band = if in_month >= 1 {
        "green"
    } else if in_quarter >= 1 {
        "yellow"
    } else {
        "red"
    };
    Band {
        band,
        raw: format!(
            "{} window(s) in the trailing {} days ({} in {})",
            in_month, CADENCE_GREEN_DAYS, in_quarter, CADENCE_RED_DAYS
        ),
        rationale: format!(
            "episodic-side-project band: green ≥ 1 window/month, red 0 windows/quarter; \
             last window {}",
            windows.last().map(String::as_str).unwrap_or("none")
        ),
    }
}

fn shift_days(date: &str, delta: i32) -> String {
    match parse_date(date) {
        Some(d) => Date::from_julian_day(d.to_julian_day() + delta)
            .map(|d| d.to_string())
            .unwrap_or_else(|_| date.to_string()),
        None => date.to_string(),
    }
}

fn merge_latency_band(
    cap: &MaintainerCapacity,
    windows: &[String],
    observed_through: &str,
) -> Band {
    if cap.measured_prs.is_empty() {
        return Band::pending(
            "no PR records in the store and no `maintainer_capacity.observed.measured_prs` \
             in the manifest — the merge-latency band has no input",
        );
    }
    let yellow_ceiling = MERGE_LATENCY_GREEN_WINDOWS * MERGE_LATENCY_YELLOW_MULTIPLE;
    let mut worst: Option<(u32, String)> = None;

    for pr in &cap.measured_prs {
        let (latency, note) = match pr.merged.as_deref() {
            Some(merged) => (
                windows_between(windows, &pr.ready, &day_of(merged)),
                format!("#{} merged {}", pr.number, day_of(merged)),
            ),
            None => (
                windows_between(windows, &pr.ready, observed_through),
                format!("#{} ready {}, unmerged", pr.number, pr.ready),
            ),
        };
        if worst.as_ref().is_none_or(|(w, _)| latency > *w) {
            worst = Some((latency, note));
        }
    }

    let (latency, note) = worst.unwrap_or((0, "no measured PRs".to_string()));
    let band = if latency <= MERGE_LATENCY_GREEN_WINDOWS {
        "green"
    } else if latency <= yellow_ceiling {
        "yellow"
    } else {
        "red"
    };
    Band {
        band,
        raw: format!("{} attention window(s) — worst: {}", latency, note),
        rationale: format!(
            "green ≤ {} windows of readiness (manifest); yellow to {}× that, red beyond",
            MERGE_LATENCY_GREEN_WINDOWS, MERGE_LATENCY_YELLOW_MULTIPLE
        ),
    }
}

fn selection_depth(
    all_classifications: &[ClassificationRecord],
    all_comments: &[CommentEventRecord],
    latest_run: u32,
    cap: &MaintainerCapacity,
) -> SelectionDepth {
    let acted: BTreeSet<u32> = all_comments
        .iter()
        .filter(|e| e.actor_role == "maintainer" && e.observed_in_run == latest_run)
        .map(|e| e.number)
        .collect();

    let cost_covariate = match cap.acted_with_diff {
        Some(c) if c.acted > 0 => format!(
            "{}/{} acted items arrived with a mergeable diff ({:.0}%)",
            c.with_diff,
            c.acted,
            f64::from(c.with_diff) / f64::from(c.acted) * 100.0
        ),
        _ => "pending — not derivable from the store (no PR↔issue linkage); declare \
              `maintainer_capacity.observed.acted_with_diff`"
            .to_string(),
    };

    if acted.is_empty() {
        return SelectionDepth {
            depth: "undefined",
            hard_ceiling: None,
            acted_this_window: 0,
            cost_covariate,
            rationale: "no maintainer action on a corpus item in this run's window — depth \
                        is undefined, not shallow; nothing was selected either way"
                .to_string(),
        };
    }

    let mut ceiling: Option<String> = None;
    for number in &acted {
        let latest = all_classifications
            .iter()
            .filter(|c| c.number == *number)
            .max_by(|a, b| a.run.cmp(&b.run).then_with(|| a.ts.cmp(&b.ts)));
        if let Some(c) = latest {
            if ceiling
                .as_deref()
                .is_none_or(|cur| rung(&c.priority) > rung(cur))
            {
                ceiling = Some(c.priority.clone());
            }
        }
    }

    let depth = match ceiling.as_deref().map(rung) {
        Some(r) if r >= 5 => "deep",
        Some(r) if r >= 3 => "mixed",
        Some(_) => "shallow",
        None => "unclassified",
    };

    SelectionDepth {
        depth,
        rationale: format!(
            "{} corpus item(s) acted on this window; deepest rung {} — descriptive only \
             (§6b rule 3), cost-ranked triage under scarcity is rational",
            acted.len(),
            ceiling.as_deref().unwrap_or("unclassified")
        ),
        hard_ceiling: ceiling,
        acted_this_window: acted.len() as u32,
        cost_covariate,
    }
}

/// Rule 4 — a red headline on a capacity-limited target requires BOTH a red
/// capacity-adjusted band AND a falsified fix-delivery-cost explanation.
/// Signal-level `drifting` stays in its own row and in `top_signal`.
fn red_gate(
    signal_verdict: &'static str,
    bands: &Bands,
    a4_alarm: &BTreeSet<u32>,
    cap: &MaintainerCapacity,
    windows: &[String],
    observed_through: &str,
) -> RedGate {
    let mut red_bands = Vec::new();
    if bands.pr_acceptance.is_red() {
        red_bands.push("pr_acceptance".to_string());
    }
    if bands.engagement_cadence.is_red() {
        red_bands.push("engagement_cadence".to_string());
    }
    if bands.merge_latency.is_red() {
        red_bands.push("merge_latency".to_string());
    }
    let condition_a = !red_bands.is_empty();

    // (b) is falsified only by a measured-side fix PR *against the silent
    // lane* that sat ≥ 2 attention windows. A PR that fixes something else,
    // however long it waited, leaves the cost explanation standing.
    let mut falsifying: Vec<String> = Vec::new();
    for pr in &cap.measured_prs {
        if pr.merged.is_some() {
            continue;
        }
        let hits: Vec<u32> = pr
            .fixes
            .iter()
            .copied()
            .filter(|n| a4_alarm.contains(n))
            .collect();
        if hits.is_empty() {
            continue;
        }
        let waited = windows_between(windows, &pr.ready, observed_through);
        if waited >= COST_FALSIFYING_WINDOWS {
            falsifying.push(format!(
                "#{} (fixes {}) waited {} windows",
                pr.number,
                hits.iter()
                    .map(|n| format!("#{}", n))
                    .collect::<Vec<_>>()
                    .join(", "),
                waited
            ));
        }
    }
    let condition_b = !falsifying.is_empty();

    let condition_b_rationale = if condition_b {
        format!(
            "falsified — {}: a ready SDL-lane fix sat ≥ {} attention windows, so cost of \
             delivery no longer explains the silence",
            falsifying.join("; "),
            COST_FALSIFYING_WINDOWS
        )
    } else {
        let lane: Vec<String> = a4_alarm.iter().map(|n| format!("#{}", n)).collect();
        let shipped: Vec<String> = cap
            .measured_prs
            .iter()
            .map(|p| {
                let f = if p.fixes.is_empty() {
                    "nothing in the lane".to_string()
                } else {
                    p.fixes
                        .iter()
                        .map(|n| format!("#{}", n))
                        .collect::<Vec<_>>()
                        .join(", ")
                };
                format!("#{} → {}", p.number, f)
            })
            .collect();
        format!(
            "intact — no measured-side fix PR against the silent lane ({}); measured PRs on \
             record: {}. §6b rule 7 puts the A4 discharge path on the measured side, and it \
             has not been walked",
            if lane.is_empty() {
                "empty".to_string()
            } else {
                lane.join(" ")
            },
            if shipped.is_empty() {
                "none".to_string()
            } else {
                shipped.join("; ")
            }
        )
    };

    let would_be_red = signal_verdict == "drifting";
    let headline = if would_be_red && !(condition_a && condition_b) {
        "watch"
    } else {
        signal_verdict
    };

    RedGate {
        rule: "SKILL §6b rule 4 — red requires a red capacity-adjusted band AND a falsified \
               fix-delivery-cost explanation",
        signal_verdict,
        condition_a_red_band: condition_a,
        red_bands,
        condition_b_cost_explanation_falsified: condition_b,
        condition_b_rationale,
        condition_b_evidence: if cap.measured_prs.is_empty() {
            "none — no PR records in the store and none declared in the manifest; (b) cannot \
             be satisfied, so the headline can only be capped, never released"
        } else {
            "manifest-declared (`maintainer_capacity.observed.measured_prs`) — the store holds \
             no PR records, so this is an operator-maintained input, not an observation"
        },
        headline_capped: headline != signal_verdict,
        headline,
    }
}

/// Rule 1 — restate the A4 row with attention windows first and the run /
/// calendar denominators as shadow. Same alarm set, same signal verdict; only
/// the units change, and the units were the overstatement.
fn redenominate_a4(report: &mut DirectionReport, a4: &[A4Windows]) {
    let ZeroEngagementOrPending::Live(sig) = &mut report.signals.silent_data_loss_zero_engagement
    else {
        return;
    };
    if a4.is_empty() {
        return;
    }
    let longest = a4.iter().map(|w| w.attention_windows).max().unwrap_or(0);
    let longest_days = a4.iter().map(|w| w.calendar_days).max().unwrap_or(0);
    let alarm: Vec<&A4Windows> = a4
        .iter()
        .filter(|w| w.runs >= SDL_ZERO_ENGAGEMENT_WATCH_STREAK)
        .collect();
    let per_issue: Vec<String> = alarm
        .iter()
        .map(|w| format!("#{} {}", w.number, w.attention_windows))
        .collect();

    sig.rationale = format!(
        "{} SDL issue(s), zero maintainer engagement — longest silence {} attention windows \
         ({} runs · {} calendar days shadow); per issue: {}",
        alarm.len(),
        longest,
        sig.longest_streak,
        longest_days,
        per_issue.join(" · ")
    );
    report.top_signal = format!(
        "silent-data-loss-zero-engagement {}: {}",
        sig.verdict, sig.rationale
    );
}

/// Read recorded prediction evaluations out of the store's `prediction` notes.
/// Rule 5 / instrument versioning: an evaluated prediction keeps the result
/// the instrument that scored it produced. This function reads; it never
/// re-scores, which is why `rescored` is a constant.
fn predictions(records: &[Record]) -> Vec<PredictionRecord> {
    let notes: Vec<&NoteRecord> = records
        .iter()
        .filter_map(|r| match r {
            Record::Note(n) if n.topic == "prediction" => Some(n),
            _ => None,
        })
        .collect();

    let mut out: Vec<PredictionRecord> = Vec::new();
    for n in &notes {
        let Some(rest) = after(&n.text, "EVALUATION (due_run ") else {
            continue;
        };
        let Some(due_run) = leading_u32(rest) else {
            continue;
        };
        let Some(result) = after(rest, "): ").map(first_word) else {
            continue;
        };
        out.push(PredictionRecord {
            due_run,
            result,
            instrument: "unrecorded".to_string(),
            rescored: false,
            recorded_in_run: n.run,
            reaffirmed_in_runs: Vec::new(),
        });
    }

    // A later note may name the instrument the evaluation was scored under
    // ("the run-16 ramp prediction remains FAILED under instrument v1").
    // That is the record speaking, not a recomputation.
    for p in &mut out {
        let marker = format!("run-{} ", p.due_run);
        for n in &notes {
            if n.run <= p.recorded_in_run || !n.text.contains(&marker) {
                continue;
            }
            if let Some(rest) = after(&n.text, "under instrument ") {
                p.instrument = first_word(rest).trim_end_matches([',', '.']).to_string();
                p.reaffirmed_in_runs.push(n.run);
            }
        }
    }
    out.sort_by_key(|p| p.due_run);
    out
}

fn after<'a>(haystack: &'a str, marker: &str) -> Option<&'a str> {
    haystack.find(marker).map(|i| &haystack[i + marker.len()..])
}

fn leading_u32(s: &str) -> Option<u32> {
    let digits: String = s.chars().take_while(char::is_ascii_digit).collect();
    digits.parse().ok()
}

fn first_word(s: &str) -> String {
    s.split_whitespace()
        .next()
        .unwrap_or("")
        .trim_end_matches(['.', ','])
        .to_string()
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
            ..Default::default()
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

    // ================================================================
    // Instrument v2 — SKILL §6b
    // ================================================================

    use crate::intent::{ActedWithDiff, MeasuredPr, PrAcceptance};

    /// The run-19 window set: 31 declared by the manifest plus 2026-07-13,
    /// which only the store saw. Neither source is a superset of the other.
    fn run19_windows() -> Vec<String> {
        [
            "2026-07-08",
            "2026-07-13",
            "2026-07-15",
            "2026-07-19",
            "2026-07-21",
            "2026-07-22",
            "2026-07-23",
            "2026-07-24",
            "2026-07-25",
            "2026-08-07",
            "2026-08-10",
            "2026-08-13",
            "2026-08-15",
            "2026-08-16",
            "2026-08-17",
            "2026-08-25",
            "2026-08-26",
            "2026-08-27",
            "2026-08-28",
            "2026-08-29",
            "2026-08-30",
            "2026-08-31",
            "2026-09-01",
            "2026-09-03",
            "2026-09-04",
            "2026-09-05",
            "2026-09-06",
            "2026-09-07",
            "2026-09-08",
            "2026-09-09",
            "2026-09-11",
            "2026-09-12",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    fn capacity() -> MaintainerCapacity {
        MaintainerCapacity {
            class: "episodic-side-project".into(),
            team_size: Some(2),
            attention_window: "action-day".into(),
            observed_windows: run19_windows(),
            pr_acceptance: Some(PrAcceptance {
                merged: 30,
                decided: 31,
            }),
            measured_prs: vec![
                MeasuredPr {
                    number: 729,
                    ready: "2026-08-01".into(),
                    merged: None,
                    fixes: vec![515],
                },
                MeasuredPr {
                    number: 768,
                    ready: "2026-08-04".into(),
                    merged: None,
                    fixes: vec![],
                },
            ],
            acted_with_diff: None,
        }
    }

    #[test]
    fn windows_are_counted_strictly_after_onset() {
        let w = run19_windows();
        // #523/#588 were first classified SDL on 2026-07-13 — the window
        // that day does not count against them; everything after does.
        assert_eq!(windows_between(&w, "2026-07-13", "2026-09-18"), 30);
        // #635 was first classified SDL on 2026-07-20.
        assert_eq!(windows_between(&w, "2026-07-20", "2026-09-18"), 28);
        // A PR ready 2026-08-01 has waited 23 windows at run 19.
        assert_eq!(windows_between(&w, "2026-08-01", "2026-09-18"), 23);
    }

    #[test]
    fn windows_beyond_the_observation_horizon_are_excluded() {
        let w = run19_windows();
        // At run 16 (2026-07-22) only the windows up to that date exist,
        // which is how the run-16 re-assessment got 4 windows for #523.
        assert_eq!(windows_between(&w, "2026-07-13", "2026-07-22"), 4);
        assert_eq!(windows_between(&w, "2026-07-20", "2026-07-22"), 2);
    }

    #[test]
    fn pr_acceptance_bands_follow_the_literature_anchors() {
        let mut cap = capacity();
        assert_eq!(pr_acceptance_band(&cap).band, "green"); // 30/31
        cap.pr_acceptance = Some(PrAcceptance {
            merged: 5,
            decided: 10,
        });
        assert_eq!(pr_acceptance_band(&cap).band, "yellow"); // 0.50
        cap.pr_acceptance = Some(PrAcceptance {
            merged: 3,
            decided: 10,
        });
        assert_eq!(pr_acceptance_band(&cap).band, "red"); // 0.30, bot band
    }

    #[test]
    fn a_band_with_no_input_is_pending_not_green() {
        let mut cap = capacity();
        cap.pr_acceptance = None;
        cap.measured_prs.clear();
        let acceptance = pr_acceptance_band(&cap);
        let latency = merge_latency_band(&cap, &run19_windows(), "2026-09-18");
        assert_eq!(acceptance.band, "pending");
        assert_eq!(latency.band, "pending");
        assert!(
            acceptance.rationale.contains("no PR records in the store"),
            "pending names the gap: {}",
            acceptance.rationale
        );
    }

    #[test]
    fn merge_latency_is_red_at_run_19() {
        let band = merge_latency_band(&capacity(), &run19_windows(), "2026-09-18");
        assert_eq!(band.band, "red", "{}", band.raw);
        assert!(band.raw.contains("23 attention window"), "{}", band.raw);
        assert!(band.raw.contains("#729"), "{}", band.raw);
    }

    #[test]
    fn merge_latency_is_green_inside_two_windows() {
        let mut cap = capacity();
        cap.measured_prs = vec![MeasuredPr {
            number: 1,
            ready: "2026-09-04".into(),
            merged: Some("2026-09-06".into()),
            fixes: vec![],
        }];
        let band = merge_latency_band(&cap, &run19_windows(), "2026-09-18");
        assert_eq!(band.band, "green", "{}", band.raw);
    }

    #[test]
    fn engagement_cadence_is_green_for_an_episodic_side_project() {
        let band = engagement_cadence_band(&run19_windows(), "2026-09-18");
        assert_eq!(band.band, "green", "{}", band.raw);
    }

    #[test]
    fn engagement_cadence_goes_red_after_a_silent_quarter() {
        let band = engagement_cadence_band(&run19_windows(), "2027-01-31");
        assert_eq!(band.band, "red", "{}", band.raw);
    }

    fn bands_with(pr: &'static str, cadence: &'static str, latency: &'static str) -> Bands {
        let mk = |b: &'static str| Band {
            band: b,
            raw: "test".into(),
            rationale: "test".into(),
        };
        Bands {
            pr_acceptance: mk(pr),
            engagement_cadence: mk(cadence),
            merge_latency: mk(latency),
        }
    }

    #[test]
    fn rule_4_caps_the_headline_when_the_cost_explanation_stands() {
        // Run 19 exactly: merge_latency IS red, but no measured-side PR
        // targets the silent lane, so (b) is unmet and red is not earned.
        let alarm: BTreeSet<u32> = [479, 523, 588, 635].into_iter().collect();
        let gate = red_gate(
            "drifting",
            &bands_with("green", "green", "red"),
            &alarm,
            &capacity(),
            &run19_windows(),
            "2026-09-18",
        );
        assert!(gate.condition_a_red_band);
        assert!(!gate.condition_b_cost_explanation_falsified);
        assert!(gate.headline_capped);
        assert_eq!(gate.headline, "watch");
        assert_eq!(
            gate.signal_verdict, "drifting",
            "the signal keeps its own verdict — only the headline is gated"
        );
    }

    #[test]
    fn rule_4_lets_red_through_when_both_conditions_hold() {
        let alarm: BTreeSet<u32> = [523].into_iter().collect();
        let mut cap = capacity();
        // A ready fix PR against the silent lane, sitting 23 windows.
        cap.measured_prs.push(MeasuredPr {
            number: 900,
            ready: "2026-08-01".into(),
            merged: None,
            fixes: vec![523],
        });
        let gate = red_gate(
            "drifting",
            &bands_with("green", "green", "red"),
            &alarm,
            &cap,
            &run19_windows(),
            "2026-09-18",
        );
        assert!(gate.condition_b_cost_explanation_falsified);
        assert!(!gate.headline_capped);
        assert_eq!(gate.headline, "drifting");
        assert!(
            gate.condition_b_rationale.contains("#900"),
            "names the PR that falsified it: {}",
            gate.condition_b_rationale
        );
    }

    #[test]
    fn rule_4_still_caps_when_a_lane_fix_is_younger_than_two_windows() {
        let alarm: BTreeSet<u32> = [523].into_iter().collect();
        let mut cap = capacity();
        cap.measured_prs.push(MeasuredPr {
            number: 900,
            ready: "2026-09-11".into(), // one window has passed
            merged: None,
            fixes: vec![523],
        });
        let gate = red_gate(
            "drifting",
            &bands_with("green", "green", "red"),
            &alarm,
            &cap,
            &run19_windows(),
            "2026-09-18",
        );
        assert!(!gate.condition_b_cost_explanation_falsified);
        assert_eq!(gate.headline, "watch");
    }

    #[test]
    fn rule_4_never_promotes_a_verdict() {
        let alarm: BTreeSet<u32> = [523].into_iter().collect();
        let gate = red_gate(
            "watch",
            &bands_with("red", "red", "red"),
            &alarm,
            &capacity(),
            &run19_windows(),
            "2026-09-18",
        );
        assert_eq!(gate.headline, "watch");
        assert!(!gate.headline_capped);
    }

    #[test]
    fn sdl_streaks_capture_the_onset_of_the_current_silence() {
        // #7 classified SDL at run 2 and still SDL at run 4 — the streak
        // opens at run 2, so the onset is the run-2 record's timestamp.
        let mut early = mk_class_at(7, 2, true);
        early.ts = "2026-07-13T00:00:00Z".into();
        let later = mk_class_at(7, 4, true);
        let (_, streaks) = sdl_streaks(&[early, later], &[], 4);
        assert_eq!(streaks.len(), 1);
        assert_eq!(streaks[0].runs, 3);
        assert_eq!(streaks[0].onset_ts, "2026-07-13T00:00:00Z");
    }

    #[test]
    fn sdl_basis_separates_a_data_loss_impact_from_a_bare_flag() {
        let mut impact = mk_class_at(1, 1, true);
        impact.silent_data_loss = true;
        impact.operational_impact = Some("data_loss".into());
        assert_eq!(sdl_basis(&impact), "impact=data_loss");

        // #479's shape: the safety flag is set while the liveness axis says
        // `degraded`. Legitimate under the finding-009 split — and also
        // exactly where a mis-flag hides, so it is reported, not silently
        // folded in with the rest.
        let mut flag_only = mk_class_at(2, 1, false);
        flag_only.silent_data_loss = true;
        flag_only.operational_impact = Some("degraded".into());
        assert!(flag_only.is_silent_data_loss());
        assert_eq!(
            sdl_basis(&flag_only),
            "flag-only (operational_impact=degraded)"
        );
    }

    #[test]
    fn selection_depth_is_undefined_when_nothing_was_selected() {
        let classes = vec![mk_class_at(1, 3, true)];
        // A maintainer comment, but from an earlier run's window.
        let comments = vec![mk_comment(1, 2, "maintainer")];
        let d = selection_depth(&classes, &comments, 3, &capacity());
        assert_eq!(d.depth, "undefined");
        assert_eq!(d.acted_this_window, 0);
        assert!(
            d.rationale.contains("not shallow"),
            "absence is reported as absence: {}",
            d.rationale
        );
    }

    #[test]
    fn selection_depth_reports_the_acted_on_ceiling_with_its_cost_covariate() {
        let mut deep = mk_class_at(10, 3, false);
        deep.priority = "P0a".into();
        let mut shallow = mk_class_at(11, 3, false);
        shallow.priority = "P2".into();
        let comments = vec![
            mk_comment(10, 3, "maintainer"),
            mk_comment(11, 3, "maintainer"),
        ];
        let mut cap = capacity();
        cap.acted_with_diff = Some(ActedWithDiff {
            with_diff: 2,
            acted: 2,
        });
        let d = selection_depth(&[deep, shallow], &comments, 3, &cap);
        assert_eq!(d.depth, "deep");
        assert_eq!(d.hard_ceiling.as_deref(), Some("P0a"));
        assert_eq!(d.acted_this_window, 2);
        assert!(d.cost_covariate.contains("2/2"), "{}", d.cost_covariate);
    }

    #[test]
    fn selection_depth_cost_covariate_is_pending_without_a_declaration() {
        let mut acted = mk_class_at(10, 3, false);
        acted.priority = "P2".into();
        let d = selection_depth(&[acted], &[mk_comment(10, 3, "maintainer")], 3, &capacity());
        assert_eq!(d.depth, "shallow");
        assert!(
            d.cost_covariate.starts_with("pending"),
            "{}",
            d.cost_covariate
        );
    }

    fn mk_note(run: u32, topic: &str, text: &str) -> Record {
        Record::Note(NoteRecord {
            ts: format!("2026-07-{:02}T00:00:00Z", run),
            target: "t".into(),
            run,
            topic: topic.into(),
            text: text.into(),
        })
    }

    #[test]
    fn a_recorded_prediction_is_read_never_rescored() {
        let records = vec![
            mk_note(
                16,
                "prediction",
                "EVALUATION (due_run 16, registered 2026-07-20 blind to this data): FAILED. \
                 Criterion: hard acted-on ceiling past P0b by run-16.",
            ),
            mk_note(
                19,
                "prediction",
                "PREDICTION PROTOCOL run 19: no prediction due. The run-16 ramp prediction \
                 remains FAILED under instrument v1 and is not re-scored.",
            ),
        ];
        let preds = predictions(&records);
        assert_eq!(preds.len(), 1);
        assert_eq!(preds[0].due_run, 16);
        assert_eq!(preds[0].result, "FAILED");
        assert_eq!(
            preds[0].instrument, "v1",
            "the evaluation keeps the instrument that scored it"
        );
        assert!(!preds[0].rescored);
        assert_eq!(preds[0].reaffirmed_in_runs, vec![19]);
    }

    /// Build a run-19-shaped record set: four SDL issues with the real
    /// onsets, eight runs, and no maintainer engagement on any of them.
    fn run19_records() -> Vec<Record> {
        let mut out: Vec<Record> = Vec::new();
        for (run, ts) in [
            (9u32, "2026-07-01"),
            (13, "2026-07-13"),
            (14, "2026-07-20"),
            (15, "2026-07-22"),
            (16, "2026-07-22"),
            (17, "2026-07-24"),
            (18, "2026-09-09"),
            (19, "2026-09-18"),
        ] {
            let mut r = mk_run(run, 4);
            r.ts = format!("{}T00:00:00Z", ts);
            out.push(Record::Run(r));
        }
        // #523 and #588 opened their silence at the run-9 classification
        // (2026-07-13); #635 at run 13 (2026-07-20); #479 at run 10.
        for (number, run, ts, impact) in [
            (479u32, 10u32, "2026-07-05", "degraded"),
            (523, 9, "2026-07-13", "data_loss"),
            (588, 9, "2026-07-13", "degraded"),
            (635, 13, "2026-07-20", "data_loss"),
        ] {
            let mut c = mk_class_at(number, run, false);
            c.ts = format!("{}T00:00:00Z", ts);
            c.silent_data_loss = true;
            c.operational_impact = Some(impact.into());
            out.push(Record::Classification(Box::new(c)));
        }
        // Maintainer activity on OTHER issues — the store-side window source.
        for (n, day) in [(100u32, "2026-07-13"), (101, "2026-08-16")] {
            out.push(Record::CommentEvent(CommentEventRecord {
                ts: format!("{}T00:00:00Z", day),
                target: "t".into(),
                number: n,
                event: "comment".into(),
                actor: "drbothen".into(),
                actor_role: "maintainer".into(),
                body_len: None,
                body_sha256: None,
                observed_in_run: 13,
            }));
        }
        out.push(mk_note(
            16,
            "prediction",
            "EVALUATION (due_run 16, registered 2026-07-20 blind to this data): FAILED.",
        ));
        out.push(mk_note(
            19,
            "prediction",
            "The run-16 ramp prediction remains FAILED under instrument v1 and is not re-scored.",
        ));
        out
    }

    #[test]
    fn v1_is_ungated_and_says_which_instrument_it_is() {
        let outcome = compute_outcome("t", &run19_records(), None);
        assert_eq!(outcome.instrument, "v1");
        assert_eq!(
            outcome.report.verdict, "drifting",
            "v1 publishes the ungated max across signals"
        );
        assert!(outcome.capacity.is_none());
        let disclosure = outcome.disclosure.expect("v1 discloses itself");
        assert!(disclosure.contains("instrument v1"), "{}", disclosure);
        assert!(
            disclosure.contains("NOT the capacity-adjusted verdict"),
            "{}",
            disclosure
        );
    }

    /// The acceptance criterion from `aae-orc-it9r3` / beadle#46, in one test.
    #[test]
    fn v2_on_run_19_is_watch_with_merge_latency_red_and_rule_4_unmet() {
        let cap = capacity();
        let outcome = compute_outcome("t", &run19_records(), Some(&cap));
        let capacity_report = outcome.capacity.expect("v2 carries the capacity layer");

        assert_eq!(outcome.instrument, "v2");
        assert_eq!(outcome.report.verdict, "watch", "the gated headline");
        assert_eq!(capacity_report.bands.merge_latency.band, "red");
        assert_eq!(capacity_report.red_gate.signal_verdict, "drifting");
        assert!(capacity_report.red_gate.condition_a_red_band);
        assert!(
            !capacity_report
                .red_gate
                .condition_b_cost_explanation_falsified
        );
        assert!(capacity_report.red_gate.headline_capped);

        // A4 in attention windows, runs as shadow.
        let windows: Vec<(u32, u32, u32)> = capacity_report
            .a4_windows
            .iter()
            .map(|w| (w.number, w.attention_windows, w.runs))
            .collect();
        assert_eq!(
            windows,
            vec![(479, 32, 10), (523, 30, 11), (588, 30, 11), (635, 28, 7)],
            "windows first, runs as shadow"
        );

        // The A4 signal keeps its own drifting verdict (§6b rule 4) and its
        // row now leads with windows.
        let ZeroEngagementOrPending::Live(a4) =
            &outcome.report.signals.silent_data_loss_zero_engagement
        else {
            panic!("expected a live A4 signal");
        };
        assert_eq!(a4.verdict, "drifting");
        assert!(
            a4.rationale.contains("attention windows"),
            "{}",
            a4.rationale
        );
        assert!(a4.rationale.contains("#523 30"), "{}", a4.rationale);

        // The run-16 prediction is untouched.
        assert_eq!(capacity_report.predictions.len(), 1);
        assert_eq!(capacity_report.predictions[0].result, "FAILED");
        assert_eq!(capacity_report.predictions[0].instrument, "v1");
        assert!(!capacity_report.predictions[0].rescored);
    }

    #[test]
    fn the_window_set_unions_store_and_manifest_sources() {
        let cap = capacity();
        let outcome = compute_outcome("t", &run19_records(), Some(&cap));
        let w = outcome.capacity.expect("v2").windows;
        assert_eq!(w.total, 32);
        assert_eq!(w.from_store, 2, "the store sees only corpus actions");
        assert_eq!(w.from_manifest, 32);
        assert_eq!(w.observed_through, "2026-09-18");
    }

    /// Pins the shape `render.rs` needs for the one-line wiring: destructure
    /// the outcome, hand `report` to the existing renderer untouched, keep
    /// `capacity` and `instrument` for the block above it. If this stops
    /// compiling, the wiring instruction in the handoff is stale.
    #[test]
    fn the_outcome_destructures_for_the_renderer() {
        let cap = capacity();
        let DirectionOutcome {
            report,
            capacity: capacity_layer,
            instrument,
            ..
        } = compute_outcome("t", &run19_records(), Some(&cap));

        fn takes_a_report(_: &DirectionReport) {}
        takes_a_report(&report);
        assert_eq!(instrument, "v2");
        assert!(capacity_layer.is_some());
    }
}
