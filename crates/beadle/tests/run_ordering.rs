//! Run-record-first ordering (`aae-orc-w5yx`, ported from ArcavenAE/aae-orc#36).
//!
//! `beadle enum` appends observations tagged with the NEW run immediately, but
//! the store's `Run` record is only written later, at render/push. Anything
//! keying on `Record::Run` therefore computes against the PREVIOUS run for the
//! entire pass it is supposed to inform — observed in three consecutive
//! sessions (finding-019 root cause 4): during run-11 work, `direction`
//! reported `run: 9` and "no classification records for run 9" while 66 fresh
//! run-10 records sat in the same store.
//!
//! The bug is invisible at rest, which is why it survived. Between passes the
//! newest `Run` record and the newest observation agree, so every check passes;
//! it fires the moment the next pass opens.

use std::{fs, process::Command};

use serde_json::{Value, json};
use tempfile::TempDir;

const INTENT: &str = "schema_version: 0.3\nrepo: BOHICA-LABS/vsdd-factory\n\
                      maintainers:\n  - drbothen\nmeasured_contributors:\n  - arcavenai\n";

fn run_row(run: u32) -> Value {
    json!({
        "kind": "run", "ts": "2026-09-09T00:00:00Z", "target": "t", "run": run,
        "watermark_before": 0, "watermark_after": 830,
        "counts": {}, "digest": "", "new_this_run": [],
    })
}

fn class_row(number: u32, run: u32) -> Value {
    json!({
        "kind": "classification", "ts": "2026-09-23T00:00:00Z", "target": "t",
        "number": number, "run": run,
        "report_type": "bug", "defect_nature": "logic", "reproducibility": "bohrbug",
        "leverage": "systemic", "alignment": "advances", "provenance": "pilot-derived",
        "integrity": false, "priority": "P2", "rationale": "fixture",
    })
}

fn workspace(rows: &[Value]) -> TempDir {
    let td = TempDir::new().unwrap();
    let targets = td.path().join("targets");
    fs::create_dir_all(&targets).unwrap();
    fs::write(targets.join("t.intent.yaml"), INTENT).unwrap();
    let store = td.path().join("store/t");
    fs::create_dir_all(&store).unwrap();
    let jsonl: String = rows
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

/// The acceptance criterion. Mid-pass: run 19 is finalized, the classifier has
/// ingested run-20 records, and no run-20 `Run` record exists yet.
#[test]
fn direction_computes_against_the_open_run_mid_pass() {
    let td = workspace(&[run_row(19), class_row(901, 20), class_row(902, 20)]);
    let d = direction(&td);
    assert_eq!(
        d["run"], 20,
        "direction must work in the open run, not the last finalized one"
    );
}

/// The signals the defect silenced. Keying on `Record::Run` put the run-20
/// classifications outside the window, so the derived signals emitted `pending`
/// for exactly the pass they existed to inform.
#[test]
fn the_open_runs_classifications_are_in_scope() {
    let td = workspace(&[run_row(19), class_row(901, 20), class_row(902, 20)]);
    let text = serde_json::to_string(&direction(&td)).unwrap();
    assert!(
        !text.contains("no classification records for run 19"),
        "run-19 emptiness must not be reported while the pass is in run 20: {text}"
    );
}

/// Control: finalizing the run must not change the answer. If it did, the fix
/// would be shifting the bug rather than removing it.
#[test]
fn finalizing_the_run_record_changes_nothing() {
    let open = workspace(&[run_row(19), class_row(901, 20), class_row(902, 20)]);
    let closed = workspace(&[
        run_row(19),
        class_row(901, 20),
        class_row(902, 20),
        run_row(20),
    ]);
    assert_eq!(direction(&open)["run"], direction(&closed)["run"]);
    assert_eq!(direction(&open)["run"], 20);
}

/// Negative control: between passes nothing claims a newer run, so the working
/// run is the finalized one. A fix that always advanced would pass the tests
/// above and be wrong here.
#[test]
fn at_rest_the_working_run_is_the_finalized_run() {
    let td = workspace(&[run_row(19), class_row(901, 19)]);
    assert_eq!(direction(&td)["run"], 19);
}
