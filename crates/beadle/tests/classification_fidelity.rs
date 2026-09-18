//! Store-fidelity regression for beadle#66 / `aae-orc-l5b5i`.
//!
//! `classify ingest` used to drop `short_title`, `triage_state`, `attn` and
//! `possibly_fixed` on the floor: `ClassificationRecord` did not declare them,
//! serde ignored the unknowns, and ingest re-serialised the struct. The
//! git-tracked rich fixture was the only durable carrier for the 👤 lane, the
//! finding-005 title-led rows, and the fixed-but-open sweep — which inverts
//! charter B1 and is issue-class #313 applied to beadle itself.
//!
//! This test drives the real binary over the real run-18 fixture and asserts
//! the round-trip is lossless key-for-key, plus that an unknown field is now a
//! loud, named failure rather than a silent drop.

use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use serde_json::Value;
use tempfile::TempDir;

const INTENT: &str = "schema_version: 0.1\nrepo: BOHICA-LABS/vsdd-factory\nmaintainers:\n  - drbothen\nmeasured_contributors:\n  - arcavenai\n";

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/fixtures")
        .join(name)
}

fn run18() -> Vec<Value> {
    let raw = fs::read_to_string(fixture("vsdd-factory-312-run18-classifications-rich.json"))
        .expect("read run-18 rich fixture");
    serde_json::from_str(&raw).expect("parse run-18 rich fixture")
}

fn workspace(td: &TempDir) {
    let t = td.path().join("targets");
    fs::create_dir_all(&t).unwrap();
    fs::write(t.join("vsdd-factory.intent.yaml"), INTENT).unwrap();
}

/// Run `beadle classify ingest` over `payload`, returning stderr on failure.
fn ingest(td: &TempDir, payload: &Value) -> Result<(), String> {
    let p = td.path().join("payload.json");
    fs::write(&p, serde_json::to_string(payload).unwrap()).unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_beadle"))
        .args(["--root".as_ref(), td.path().as_os_str()])
        .args(["classify", "ingest", "vsdd-factory"])
        .arg("--file")
        .arg(&p)
        .output()
        .expect("spawn beadle");
    if out.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).into_owned())
    }
}

fn stored(td: &TempDir) -> Vec<Value> {
    let path = td.path().join("store/vsdd-factory/state.jsonl");
    fs::read_to_string(path)
        .expect("read store")
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).expect("parse store line"))
        .collect()
}

fn keys(v: &Value) -> BTreeSet<String> {
    v.as_object().unwrap().keys().cloned().collect()
}

/// The acceptance criterion: a run-18 fixture record round-trips with all 25
/// fields readable back from the store.
#[test]
fn run18_fixture_roundtrips_with_every_field() {
    let td = TempDir::new().unwrap();
    workspace(&td);
    let fx = run18();
    ingest(&td, &Value::Array(fx.clone())).expect("ingest whole run-18 fixture");

    let got = stored(&td);
    assert_eq!(
        got.len(),
        fx.len(),
        "every fixture record reached the store"
    );

    // Key-set parity, record by record. This is the assertion that would have
    // caught the original defect: the store used to carry 21 of the 25 keys.
    for want in &fx {
        let n = want["number"].as_u64().unwrap();
        let have = got
            .iter()
            .find(|r| r["number"].as_u64() == Some(n))
            .unwrap_or_else(|| panic!("#{n} missing from store"));
        assert_eq!(keys(want), keys(have), "#{n} key set drifted on ingest");
        assert_eq!(keys(have).len(), 25, "#{n} should carry 25 keys");
    }

    // The four previously-dropped fields, by value, on the records that
    // exercise every shape: a possibly_fixed verdict, an attn-lane entry, and
    // a plain row whose attn is null.
    let by_num = |n: u64| {
        got.iter()
            .find(|r| r["number"].as_u64() == Some(n))
            .unwrap()
            .clone()
    };

    let a = by_num(762);
    assert_eq!(
        a["short_title"],
        Value::from("Records-tier treadmill: line-cite ban, mechanical records-lint, micro-burst")
    );
    assert_eq!(a["triage_state"], Value::from("accepted"));
    assert_eq!(a["attn"], Value::Null);
    assert_eq!(a["possibly_fixed"]["confidence"], Value::from("low"));
    assert!(a["possibly_fixed"]["by"].as_str().unwrap().contains("#776"));
    assert!(a["possibly_fixed"]["verify"].as_str().unwrap().len() > 50);

    let b = by_num(766);
    assert_eq!(b["attn"]["subtype"], Value::from("governance"));
    assert_eq!(b["attn"]["order"], Value::from("reply-needed"));
    assert!(b["attn"]["why"].as_str().unwrap().contains("rename"));

    // Sanity: the fixture really does exercise all three triage states and
    // both attn subtypes, so the parity assertion above is not vacuous.
    let states: BTreeSet<&str> = got
        .iter()
        .filter_map(|r| r["triage_state"].as_str())
        .collect();
    assert_eq!(
        states,
        BTreeSet::from(["accepted", "needs-information", "needs-triage"])
    );
    let lane = got.iter().filter(|r| !r["attn"].is_null()).count();
    assert_eq!(lane, 2, "run-18 puts two issues in the attn lane");
}

/// The second acceptance criterion: an unknown field is rejected, by name.
#[test]
fn unknown_field_is_rejected_by_name() {
    let td = TempDir::new().unwrap();
    workspace(&td);
    let mut rec = run18().remove(0);
    // `alignment_rationale` is a real key from the run-13/14 fixtures — exactly
    // the kind of drift that used to vanish silently.
    rec["alignment_rationale"] = Value::from("advances because ...");
    let err = ingest(&td, &rec).expect_err("unknown field must be rejected");
    assert!(
        err.contains("alignment_rationale"),
        "error must name the field, got: {err}"
    );
    assert!(
        !td.path().join("store/vsdd-factory/state.jsonl").exists(),
        "a rejected payload must not write a partial store"
    );
}

#[test]
fn unknown_nested_field_is_rejected_with_its_path() {
    let td = TempDir::new().unwrap();
    workspace(&td);
    let mut rec = run18().remove(0);
    rec["attn"] = serde_json::json!({"subtype": "governance", "escalate": true});
    let err = ingest(&td, &rec).expect_err("unknown nested field must be rejected");
    assert!(err.contains("attn.escalate"), "got: {err}");
}

/// The run-12 and run-14 fixtures encode the same facet as a bare string and
/// as `{type, reason, reading_order}`. Neither is silently coerced.
#[test]
fn legacy_attn_encodings_are_rejected_with_a_pointer_to_the_backfill() {
    let td = TempDir::new().unwrap();
    workspace(&td);

    let mut bare = run18().remove(0);
    bare["attn"] = Value::from("governance");
    let err = ingest(&td, &bare).expect_err("bare-string attn must be rejected");
    assert!(
        err.contains("backfill-classification-fields.py"),
        "got: {err}"
    );

    let mut old_obj = run18().remove(0);
    old_obj["attn"] = serde_json::json!({"type": "direction", "reason": "x"});
    let err = ingest(&td, &old_obj).expect_err("{type,reason} attn must be rejected");
    assert!(err.contains("attn.type"), "got: {err}");
}

#[test]
fn closed_domains_are_enforced() {
    let td = TempDir::new().unwrap();
    workspace(&td);

    let mut bad_state = run18().remove(0);
    bad_state["triage_state"] = Value::from("triaged");
    let err = ingest(&td, &bad_state).expect_err("unknown triage_state must be rejected");
    assert!(err.contains("triaged"), "got: {err}");

    let mut bad_conf = run18().remove(0);
    bad_conf["possibly_fixed"] =
        serde_json::json!({"by": "PR #1", "confidence": "probably", "verify": "check it"});
    let err = ingest(&td, &bad_conf).expect_err("unknown confidence must be rejected");
    assert!(err.contains("probably"), "got: {err}");

    let mut no_by = run18().remove(0);
    no_by["possibly_fixed"] = serde_json::json!({"confidence": "low"});
    let err = ingest(&td, &no_by).expect_err("possibly_fixed.by is required");
    assert!(err.contains("possibly_fixed.by"), "got: {err}");
}
