//! Instrument v2 end-to-end (`aae-orc-it9r3` / ArcavenAE/beadle#46).
//!
//! `beadle direction` published 🔴 drifting on the same store, at the same
//! watermark, where the run-19 board published 🟡 WATCH. The crate computed
//! **instrument v1** — run-denominated streaks and an ungated headline —
//! while the board ran **instrument v2** (SKILL §6b), which the skill applied
//! by hand every run. Two commands, two verdicts, neither naming its
//! instrument.
//!
//! This test drives the real binary over a run-19-shaped fixture and asserts
//! the four things §6b makes true: the headline is gated by rule 4, the bands
//! are scored against the manifest's capacity model, A4 streaks are
//! denominated in attention windows, and the evaluated run-16 prediction is
//! carried verbatim rather than re-scored under the new instrument.

use std::{fs, process::Command};

use serde_json::{Value, json};
use tempfile::TempDir;

/// The run-19 capacity model, verbatim in shape from
/// `targets/vsdd-factory.intent.yaml` (manifest schema v0.6).
const INTENT: &str = r#"schema_version: 0.6
target:
  repo: BOHICA-LABS/vsdd-factory
maintainers:
  - drbothen
  - Zious11
measured_contributors:
  - arcavenai
maintainer_capacity:
  class: episodic-side-project
  team_size: 2
  attention_window: action-day
  observed_windows: ["2026-07-08", "2026-07-15", "2026-07-19", "2026-07-21",
    "2026-07-22", "2026-07-23", "2026-07-24", "2026-07-25", "2026-08-07",
    "2026-08-10", "2026-08-13", "2026-08-15", "2026-08-16", "2026-08-17",
    "2026-08-25", "2026-08-26", "2026-08-27", "2026-08-28", "2026-08-29",
    "2026-08-30", "2026-08-31", "2026-09-01", "2026-09-03", "2026-09-04",
    "2026-09-05", "2026-09-06", "2026-09-07", "2026-09-08", "2026-09-09",
    "2026-09-11", "2026-09-12"]
  success_bands:
    merge_latency:
      green: "merged within <= 2 attention windows of PR readiness"
  observed:
    as_of_run: 19
    pr_acceptance:
      merged: 30
      decided: 31
    measured_prs:
      - number: 729
        ready: "2026-08-01"
        fixes: [515]
      - number: 768
        ready: "2026-08-04"
        fixes: []
"#;

/// A manifest with no capacity model — the v1 path.
const INTENT_V1: &str = "schema_version: 0.3\nrepo: BOHICA-LABS/vsdd-factory\nmaintainers:\n  - drbothen\nmeasured_contributors:\n  - arcavenai\n";

fn run_row(run: u32, day: &str) -> Value {
    json!({
        "kind": "run",
        "ts": format!("{day}T00:00:00Z"),
        "target": "t",
        "run": run,
        "watermark_before": 0,
        "watermark_after": 836,
        "counts": {},
        "digest": "",
        "new_this_run": [],
    })
}

/// An SDL classification. `impact` is the finding-009 liveness axis; the
/// safety class travels in `silent_data_loss`, which is why #588 (degraded)
/// and #523 (data_loss) are both in the alarm set.
fn sdl_row(number: u32, run: u32, day: &str, impact: &str) -> Value {
    json!({
        "kind": "classification",
        "ts": format!("{day}T00:00:00Z"),
        "target": "t",
        "number": number,
        "run": run,
        "report_type": "bug",
        "defect_nature": "logic",
        "reproducibility": "bohrbug",
        "leverage": "systemic",
        "alignment": "advances",
        "provenance": "pilot-derived",
        "integrity": false,
        "operational_impact": impact,
        "silent_data_loss": true,
        "priority": "P0",
        "rationale": "fixture",
    })
}

fn maintainer_comment(number: u32, day: &str, run: u32) -> Value {
    json!({
        "kind": "comment_event",
        "ts": format!("{day}T00:00:00Z"),
        "target": "t",
        "number": number,
        "event": "comment",
        "actor": "drbothen",
        "actor_role": "maintainer",
        "observed_in_run": run,
    })
}

fn note_row(run: u32, topic: &str, text: &str) -> Value {
    json!({
        "kind": "note",
        "ts": format!("2026-09-18T00:00:00Z"),
        "target": "t",
        "run": run,
        "topic": topic,
        "text": text,
    })
}

/// A run-19-shaped store: eight runs, four SDL issues with the real onset
/// dates and no maintainer engagement, maintainer activity on other issues,
/// and the recorded run-16 prediction evaluation.
fn store_rows() -> Vec<Value> {
    let mut rows = vec![
        run_row(9, "2026-07-01"),
        run_row(13, "2026-07-13"),
        run_row(14, "2026-07-20"),
        run_row(15, "2026-07-22"),
        run_row(16, "2026-07-22"),
        run_row(17, "2026-07-24"),
        run_row(18, "2026-09-09"),
        run_row(19, "2026-09-18"),
        sdl_row(479, 10, "2026-07-05", "degraded"),
        sdl_row(523, 9, "2026-07-13", "data_loss"),
        sdl_row(588, 9, "2026-07-13", "degraded"),
        sdl_row(635, 13, "2026-07-20", "data_loss"),
    ];
    // Maintainer action on issues outside the silent lane.
    rows.push(maintainer_comment(100, "2026-07-13", 13));
    rows.push(maintainer_comment(101, "2026-08-16", 18));
    rows.push(note_row(
        16,
        "prediction",
        "EVALUATION (due_run 16, registered 2026-07-20 blind to this data): FAILED. \
         Criterion: hard acted-on ceiling past P0b by run-16.",
    ));
    rows.push(note_row(
        19,
        "prediction",
        "PREDICTION PROTOCOL run 19: no prediction due. The run-16 ramp prediction remains \
         FAILED under instrument v1 and is not re-scored.",
    ));
    rows
}

fn workspace(intent: &str) -> TempDir {
    let td = TempDir::new().unwrap();
    let targets = td.path().join("targets");
    fs::create_dir_all(&targets).unwrap();
    fs::write(targets.join("t.intent.yaml"), intent).unwrap();

    let store = td.path().join("store/t");
    fs::create_dir_all(&store).unwrap();
    let jsonl: String = store_rows()
        .iter()
        .map(|r| format!("{}\n", serde_json::to_string(r).unwrap()))
        .collect();
    fs::write(store.join("state.jsonl"), jsonl).unwrap();
    td
}

fn direction(td: &TempDir) -> Value {
    let out = Command::new(env!("CARGO_BIN_EXE_beadle"))
        .args(["--root".as_ref(), td.path().as_os_str()])
        .args(["direction", "t"])
        .output()
        .expect("spawn beadle");
    assert!(
        out.status.success(),
        "beadle direction failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    serde_json::from_slice(&out.stdout).expect("direction emits JSON")
}

/// The acceptance criterion, end to end: the same store that made v1 publish
/// 🔴 yields 🟡 WATCH under v2, because rule 4's second condition is unmet.
#[test]
fn run19_yields_watch_under_v2_with_merge_latency_red() {
    let td = workspace(INTENT);
    let d = direction(&td);

    assert_eq!(d["instrument"], "v2");
    assert_eq!(d["verdict"], "watch", "the gated headline");

    let gate = &d["capacity"]["red_gate"];
    assert_eq!(
        gate["signal_verdict"], "drifting",
        "the signals still say drifting — only the headline is gated"
    );
    assert_eq!(gate["condition_a_red_band"], true);
    assert_eq!(gate["condition_b_cost_explanation_falsified"], false);
    assert_eq!(gate["headline_capped"], true);
    assert_eq!(gate["red_bands"], json!(["merge_latency"]));

    let bands = &d["capacity"]["bands"];
    assert_eq!(bands["merge_latency"]["band"], "red");
    assert_eq!(bands["pr_acceptance"]["band"], "green");
    assert_eq!(bands["engagement_cadence"]["band"], "green");
    // Rule 2: report the band AND the raw number.
    assert_eq!(bands["pr_acceptance"]["raw"], "30/31 = 96.8%");
    assert!(
        bands["merge_latency"]["raw"]
            .as_str()
            .unwrap()
            .contains("23 attention window"),
        "{}",
        bands["merge_latency"]["raw"]
    );
}

/// Rule 1: A4 streaks are denominated in attention windows, with runs and
/// calendar days as shadow. A run-denominated streak overstates the silence
/// whenever beadle's cadence outpaces the maintainers' — it always does.
#[test]
fn a4_streaks_report_attention_windows_first() {
    let td = workspace(INTENT);
    let d = direction(&td);

    let rows: Vec<(u64, u64, u64)> = d["capacity"]["a4_windows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|w| {
            (
                w["number"].as_u64().unwrap(),
                w["attention_windows"].as_u64().unwrap(),
                w["runs"].as_u64().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        rows,
        vec![(479, 32, 10), (523, 30, 11), (588, 30, 11), (635, 28, 7)]
    );

    // The signal row leads with windows too, not just the capacity block.
    let rationale = d["signals"]["silent_data_loss_zero_engagement"]["rationale"]
        .as_str()
        .unwrap();
    assert!(rationale.contains("attention windows"), "{rationale}");
    assert!(rationale.contains("#523 30"), "{rationale}");
}

/// #479 carries `silent_data_loss: true` with `operational_impact: degraded`.
/// So does #588, which the board *does* count — so the flag, not the impact
/// axis, is what puts an issue in the class. The membership basis is reported
/// per issue so a disagreement about a row is visible instead of silent.
#[test]
fn sdl_membership_basis_is_reported_per_issue() {
    let td = workspace(INTENT);
    let d = direction(&td);
    let by_number: Vec<(u64, String)> = d["capacity"]["a4_windows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|w| {
            (
                w["number"].as_u64().unwrap(),
                w["basis"].as_str().unwrap().to_string(),
            )
        })
        .collect();
    assert_eq!(by_number[0].1, "flag-only (operational_impact=degraded)"); // 479
    assert_eq!(by_number[1].1, "impact=data_loss"); // 523
    assert_eq!(by_number[2].1, "flag-only (operational_impact=degraded)"); // 588
}

/// §6b forbids re-scoring an evaluated prediction under a new instrument.
/// Computing v2 must leave the run-16 evaluation FAILED, under v1.
#[test]
fn the_run16_prediction_is_not_rescored_under_v2() {
    let td = workspace(INTENT);
    let d = direction(&td);
    let preds = d["capacity"]["predictions"].as_array().unwrap();
    assert_eq!(preds.len(), 1);
    assert_eq!(preds[0]["due_run"], 16);
    assert_eq!(preds[0]["result"], "FAILED");
    assert_eq!(preds[0]["instrument"], "v1");
    assert_eq!(preds[0]["rescored"], false);
}

/// Rule 3: the severity ceiling is descriptive. Run 19 had zero maintainer
/// actions on the corpus, so depth is undefined — reported as absence, never
/// as a fall to shallow, and it carries no verdict either way.
#[test]
fn selection_depth_is_descriptive_and_undefined_without_actions() {
    let td = workspace(INTENT);
    let d = direction(&td);
    let depth = &d["capacity"]["selection_depth"];
    assert_eq!(depth["depth"], "undefined");
    assert_eq!(depth["acted_this_window"], 0);
    assert!(depth.get("verdict").is_none(), "depth carries no verdict");
}

/// A target with no capacity model is still computable under v1 — and says
/// so, so that two commands can never again give two answers with neither
/// naming its instrument.
#[test]
fn v1_remains_computable_and_declares_itself() {
    let td = workspace(INTENT_V1);
    let out = Command::new(env!("CARGO_BIN_EXE_beadle"))
        .args(["--root".as_ref(), td.path().as_os_str()])
        .args(["direction", "t"])
        .output()
        .expect("spawn beadle");
    assert!(out.status.success());

    let d: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(d["instrument"], "v1");
    assert_eq!(d["verdict"], "drifting", "v1 is ungated");
    assert!(d["capacity"].is_null(), "no capacity layer under v1");

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("instrument v1"),
        "v1 discloses itself on stderr: {stderr}"
    );
}

/// Rule 4(b) is judged against a hand-maintained manifest field, because the
/// store holds no PR records. A stale `fixes:` line is therefore the one
/// input that can manufacture a red headline — so the gate names its own
/// evidence source in the output rather than letting it pass as observation.
#[test]
fn the_red_gate_names_the_provenance_of_its_evidence() {
    let td = workspace(INTENT);
    let d = direction(&td);
    let evidence = d["capacity"]["red_gate"]["condition_b_evidence"]
        .as_str()
        .unwrap();
    assert!(evidence.contains("manifest-declared"), "{evidence}");
    assert!(evidence.contains("not an observation"), "{evidence}");
}

/// With no `observed:` block the bands go `pending` and the headline is
/// unchanged — the acceptance-critical numbers (the gated headline and the
/// A4 window streaks) do not depend on the declared block at all.
#[test]
fn dropping_the_observed_block_changes_bands_but_not_the_headline() {
    let bare = INTENT
        .split("  observed:")
        .next()
        .expect("split on the observed block");
    let td = workspace(bare);
    let d = direction(&td);

    assert_eq!(d["verdict"], "watch", "headline is unchanged");
    assert_eq!(d["capacity"]["bands"]["pr_acceptance"]["band"], "pending");
    assert_eq!(d["capacity"]["bands"]["merge_latency"]["band"], "pending");
    assert_eq!(
        d["capacity"]["red_gate"]["condition_a_red_band"], false,
        "a pending band is not a red band"
    );

    let windows: Vec<u64> = d["capacity"]["a4_windows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|w| w["attention_windows"].as_u64().unwrap())
        .collect();
    assert_eq!(
        windows,
        vec![32, 30, 30, 28],
        "A4 denominators are unchanged"
    );
}
