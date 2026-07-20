//! `beadle classify ingest` — accept a validated JSON ClassificationRecord
//! payload from an upstream classifier (the Phase-0 Claude Code skill or,
//! eventually, a Phase-1 Go/Rust classifier) and append it to the store.
//!
//! This crate does not itself classify. Heuristic keyword-matching would
//! launder guesses as data and defeat the whole point of the classification
//! superset (`elem-defect-classification-superset`). The ingest path is the
//! contract; the skill produces the payload.
//!
//! Payload shape (JSON, one record or a JSON array of records):
//!
//!   {
//!     "target": "vsdd-factory",
//!     "number": 313,
//!     "run": 10,
//!     "report_type": "bug",
//!     "defect_nature": "spec-requirements",
//!     "reproducibility": "bohrbug",
//!     "leverage": "systemic",
//!     "alignment": "advances",
//!     "provenance": "pilot-derived",
//!     "integrity": true,
//!     "integrity_anchor": "spec_process",
//!     "silent_data_loss": true,
//!     "operational_impact": "halt",
//!     "priority": "P0",
//!     "cluster": ["ratchet-integrity"],
//!     "quick_win_eligible": false,
//!     "rationale": "PASS certified against uncommitted artifacts (finding-004)",
//!     "cited_evidence": ".factory/logs/D-042.md line 118"
//!   }
//!
//! Timestamps default to now (RFC3339); an explicit `ts` in the payload wins
//! so the skill can replay historical classifications deterministically.

use std::{
    fs,
    io::{self, Read},
    path::Path,
};

use anyhow::{anyhow, bail, Context, Result};
use beadle_store::{ClassificationRecord, Record, Store};
use serde_json::Value;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use crate::{intent, vocab::vocab};

pub fn ingest(root: &Path, target: &str, payload_path: Option<&Path>) -> Result<()> {
    let intent = intent::load(root, target)?;
    if intent.repo.is_empty() {
        bail!("intent for target `{target}` has empty repo");
    }
    let store = Store::open(root.join("store"), target)?;

    let raw = match payload_path {
        Some(p) => fs::read_to_string(p).with_context(|| format!("read {}", p.display()))?,
        None => {
            let mut buf = String::new();
            io::stdin()
                .read_to_string(&mut buf)
                .context("read classify payload from stdin")?;
            buf
        }
    };

    let parsed: Value = serde_json::from_str(&raw).context("parse JSON payload")?;
    let items: Vec<Value> = match parsed {
        Value::Array(v) => v,
        Value::Object(_) => vec![parsed],
        _ => bail!("payload must be a JSON object or array of objects"),
    };

    let latest_run = store.latest_run()?.map(|r| r.run).unwrap_or(0);
    let default_ts = OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_string());

    let mut recs: Vec<Record> = Vec::with_capacity(items.len());
    for (i, item) in items.into_iter().enumerate() {
        let rec = validate(&item, target, latest_run, &default_ts)
            .with_context(|| format!("payload item {}", i))?;
        recs.push(Record::Classification(Box::new(rec)));
    }

    if recs.is_empty() {
        eprintln!("beadle classify ingest: no records in payload");
        return Ok(());
    }

    store.append(&recs)?;
    eprintln!(
        "beadle classify ingest: appended {} classification record(s) to target={target}",
        recs.len()
    );
    println!("{{\"appended\": {}}}", recs.len());
    Ok(())
}

fn validate(
    v: &Value,
    target: &str,
    latest_run: u32,
    default_ts: &str,
) -> Result<ClassificationRecord> {
    let obj = v
        .as_object()
        .ok_or_else(|| anyhow!("record must be a JSON object"))?;

    let ts = obj
        .get("ts")
        .and_then(Value::as_str)
        .unwrap_or(default_ts)
        .to_string();
    let rec_target = obj.get("target").and_then(Value::as_str).unwrap_or(target);
    if rec_target != target {
        bail!(
            "record target `{}` does not match ingest target `{}`",
            rec_target,
            target
        );
    }
    let number = obj
        .get("number")
        .and_then(Value::as_u64)
        .ok_or_else(|| anyhow!("missing/invalid `number`"))? as u32;
    let run = obj
        .get("run")
        .and_then(Value::as_u64)
        .map(|n| n as u32)
        .unwrap_or(latest_run);

    let v = vocab();
    let report_type = req_enum(obj, "report_type", &v.report_type)?;
    let defect_nature = req_enum(obj, "defect_nature", &v.defect_nature)?;
    let reproducibility = req_enum(obj, "reproducibility", &v.reproducibility)?;
    let leverage = req_str(obj, "leverage")?;
    let alignment = req_str(obj, "alignment")?;
    let provenance = req_str(obj, "provenance")?;
    let integrity = obj
        .get("integrity")
        .and_then(Value::as_bool)
        .ok_or_else(|| anyhow!("missing/invalid `integrity` bool"))?;

    let integrity_anchor = obj
        .get("integrity_anchor")
        .and_then(Value::as_str)
        .map(|s| s.to_string());
    let silent_data_loss = obj
        .get("silent_data_loss")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let operational_impact = match obj.get("operational_impact").and_then(Value::as_str) {
        Some(s) => {
            if v.is_legacy_impact(s) {
                bail!(
                    "operational_impact `{}` is a legacy pre-finding-009 value; \
                     the axis takes the liveness tokens {:?} (beadle#32). \
                     Existing stores re-map via `beadle classify migrate-impact`",
                    s,
                    v.operational_impact
                );
            }
            if !v.operational_impact.iter().any(|a| a == s) {
                bail!(
                    "operational_impact `{}` not in {:?}",
                    s,
                    v.operational_impact
                );
            }
            Some(s.to_string())
        }
        None => None,
    };

    if integrity && integrity_anchor.is_none() {
        bail!("integrity=true requires `integrity_anchor` naming the systems-of-record tier");
    }

    let priority = req_enum(obj, "priority", &v.priority)?;

    let cluster = obj
        .get("cluster")
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let quick_win_eligible = obj
        .get("quick_win_eligible")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let rationale = req_str(obj, "rationale")?;
    let cited_evidence = obj
        .get("cited_evidence")
        .and_then(Value::as_str)
        .map(|s| s.to_string());
    let quick_win_disqualification = obj
        .get("quick_win_disqualification")
        .and_then(Value::as_str)
        .map(|s| s.to_string());

    if quick_win_eligible && integrity {
        bail!(
            "quick_win_eligible=true is invalid when integrity=true (HARD EXCLUSION per elem-defect-classification-superset)"
        );
    }
    if quick_win_eligible && silent_data_loss {
        bail!(
            "quick_win_eligible=true is invalid on silent-data-loss records (LANE EXCLUSION BY RULE, finding-020 F3)"
        );
    }
    if quick_win_eligible && operational_impact.as_deref() == Some("panic") {
        bail!(
            "quick_win_eligible=true is invalid on impact=panic records (LANE EXCLUSION BY RULE, finding-020 F3)"
        );
    }

    Ok(ClassificationRecord {
        ts,
        target: target.to_string(),
        number,
        run,
        report_type,
        defect_nature,
        reproducibility,
        leverage,
        alignment,
        provenance,
        integrity,
        integrity_anchor,
        operational_impact,
        silent_data_loss,
        priority,
        cluster,
        quick_win_eligible,
        rationale,
        cited_evidence,
        quick_win_disqualification,
    })
}

fn req_str(obj: &serde_json::Map<String, Value>, key: &str) -> Result<String> {
    obj.get(key)
        .and_then(Value::as_str)
        .map(String::from)
        .ok_or_else(|| anyhow!("missing/invalid string `{}`", key))
}

fn req_enum(obj: &serde_json::Map<String, Value>, key: &str, allowed: &[String]) -> Result<String> {
    let v = req_str(obj, key)?;
    if !allowed.iter().any(|a| a == &v) {
        bail!("`{}` value `{}` not in {:?}", key, v, allowed);
    }
    Ok(v)
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use tempfile::TempDir;

    use super::*;

    fn intent_yaml() -> &'static str {
        "schema_version: 0.1\nrepo: acme/widget\nmaintainers:\n  - alice\nmeasured_contributors:\n  - bot\n"
    }

    fn setup(td: &TempDir) {
        let tdir = td.path().join("targets");
        std::fs::create_dir_all(&tdir).unwrap();
        std::fs::write(tdir.join("t.intent.yaml"), intent_yaml()).unwrap();
    }

    #[test]
    fn accepts_valid_record() {
        let td = TempDir::new().unwrap();
        setup(&td);
        let payload = json!({
            "target": "t",
            "number": 42,
            "run": 1,
            "report_type": "bug",
            "defect_nature": "logic",
            "reproducibility": "bohrbug",
            "leverage": "minutiae",
            "alignment": "advances",
            "provenance": "pilot-derived",
            "integrity": false,
            "priority": "P2",
            "rationale": "logic error in module X"
        });
        let path = td.path().join("p.json");
        std::fs::write(&path, payload.to_string()).unwrap();
        ingest(td.path(), "t", Some(&path)).unwrap();

        let store = Store::open(td.path().join("store"), "t").unwrap();
        let recs = store.read_all().unwrap();
        assert_eq!(recs.len(), 1);
        matches!(&recs[0], Record::Classification(_));
    }

    #[test]
    fn rejects_unknown_defect_nature() {
        let td = TempDir::new().unwrap();
        setup(&td);
        let payload = json!({
            "target": "t",
            "number": 1,
            "run": 1,
            "report_type": "bug",
            "defect_nature": "vibes",
            "reproducibility": "bohrbug",
            "leverage": "minutiae",
            "alignment": "advances",
            "provenance": "pilot-derived",
            "integrity": false,
            "priority": "P3",
            "rationale": "x"
        });
        let path = td.path().join("p.json");
        std::fs::write(&path, payload.to_string()).unwrap();
        let err = ingest(td.path(), "t", Some(&path)).unwrap_err();
        assert!(
            err.to_string().contains("payload item 0")
                || err.chain().any(|c| c.to_string().contains("vibes"))
        );
    }

    #[test]
    fn rejects_integrity_without_anchor() {
        let td = TempDir::new().unwrap();
        setup(&td);
        let payload = json!({
            "target": "t", "number": 1, "run": 1,
            "report_type": "bug", "defect_nature": "logic",
            "reproducibility": "bohrbug", "leverage": "systemic",
            "alignment": "advances", "provenance": "pilot-derived",
            "integrity": true, "priority": "P0", "rationale": "x"
        });
        let path = td.path().join("p.json");
        std::fs::write(&path, payload.to_string()).unwrap();
        assert!(ingest(td.path(), "t", Some(&path)).is_err());
    }

    #[test]
    fn rejects_quick_win_on_integrity() {
        let td = TempDir::new().unwrap();
        setup(&td);
        let payload = json!({
            "target": "t", "number": 1, "run": 1,
            "report_type": "bug", "defect_nature": "logic",
            "reproducibility": "bohrbug", "leverage": "systemic",
            "alignment": "advances", "provenance": "pilot-derived",
            "integrity": true, "integrity_anchor": "spec_process",
            "priority": "P0", "quick_win_eligible": true, "rationale": "x"
        });
        let path = td.path().join("p.json");
        std::fs::write(&path, payload.to_string()).unwrap();
        assert!(ingest(td.path(), "t", Some(&path)).is_err());
    }

    #[test]
    fn accepts_array_payload() {
        let td = TempDir::new().unwrap();
        setup(&td);
        let payload = json!([
            {"target":"t","number":1,"run":1,"report_type":"bug","defect_nature":"logic","reproducibility":"bohrbug","leverage":"minutiae","alignment":"advances","provenance":"pilot-derived","integrity":false,"priority":"P2","rationale":"a"},
            {"target":"t","number":2,"run":1,"report_type":"docs","defect_nature":"spec-requirements","reproducibility":"unknown","leverage":"minutiae","alignment":"advances","provenance":"speculative","integrity":false,"priority":"P3","rationale":"b"}
        ]);
        let path = td.path().join("p.json");
        std::fs::write(&path, payload.to_string()).unwrap();
        ingest(td.path(), "t", Some(&path)).unwrap();

        let store = Store::open(td.path().join("store"), "t").unwrap();
        assert_eq!(store.read_all().unwrap().len(), 2);
    }

    fn base_payload() -> serde_json::Value {
        json!({
            "target": "t", "number": 1, "run": 1,
            "report_type": "bug", "defect_nature": "logic",
            "reproducibility": "bohrbug", "leverage": "systemic",
            "alignment": "advances", "provenance": "pilot-derived",
            "integrity": false, "priority": "P2", "rationale": "x"
        })
    }

    fn run_payload(td: &TempDir, payload: &serde_json::Value) -> Result<()> {
        let path = td.path().join("p.json");
        std::fs::write(&path, payload.to_string()).unwrap();
        ingest(td.path(), "t", Some(&path))
    }

    #[test]
    fn accepts_finding_009_liveness_tokens() {
        let td = TempDir::new().unwrap();
        setup(&td);
        for tok in ["panic", "halt", "data_loss", "degraded", "none"] {
            let mut p = base_payload();
            p["operational_impact"] = json!(tok);
            run_payload(&td, &p).unwrap();
        }
    }

    #[test]
    fn rejects_legacy_impact_with_migration_pointer() {
        let td = TempDir::new().unwrap();
        setup(&td);
        let mut p = base_payload();
        p["operational_impact"] = json!("silent-data-loss");
        let err = run_payload(&td, &p).unwrap_err();
        let msg = format!("{:#}", err);
        assert!(msg.contains("legacy"), "got: {msg}");
        assert!(msg.contains("migrate-impact"), "got: {msg}");
    }

    #[test]
    fn accepts_superset_report_types() {
        let td = TempDir::new().unwrap();
        setup(&td);
        for rt in ["process-gap", "enhancement", "policy", "proposal"] {
            let mut p = base_payload();
            p["report_type"] = json!(rt);
            run_payload(&td, &p).unwrap();
        }
    }

    #[test]
    fn accepts_p0a_p0b_priority_and_rejects_unknown() {
        let td = TempDir::new().unwrap();
        setup(&td);
        for pr in ["P0a", "P0b", "P4"] {
            let mut p = base_payload();
            p["priority"] = json!(pr);
            run_payload(&td, &p).unwrap();
        }
        let mut p = base_payload();
        p["priority"] = json!("high");
        assert!(run_payload(&td, &p).is_err());
    }

    #[test]
    fn rejects_quick_win_on_panic() {
        let td = TempDir::new().unwrap();
        setup(&td);
        let mut p = base_payload();
        p["operational_impact"] = json!("panic");
        p["quick_win_eligible"] = json!(true);
        let err = run_payload(&td, &p).unwrap_err();
        assert!(format!("{:#}", err).contains("finding-020 F3"));
    }

    #[test]
    fn rejects_quick_win_on_silent_data_loss_flag() {
        let td = TempDir::new().unwrap();
        setup(&td);
        let mut p = base_payload();
        p["silent_data_loss"] = json!(true);
        p["quick_win_eligible"] = json!(true);
        let err = run_payload(&td, &p).unwrap_err();
        assert!(format!("{:#}", err).contains("finding-020 F3"));
    }

    #[test]
    fn silent_data_loss_flag_persists_to_store() {
        let td = TempDir::new().unwrap();
        setup(&td);
        let mut p = base_payload();
        p["silent_data_loss"] = json!(true);
        p["operational_impact"] = json!("none");
        run_payload(&td, &p).unwrap();
        let store = Store::open(td.path().join("store"), "t").unwrap();
        let recs = store.read_all().unwrap();
        match &recs[0] {
            Record::Classification(c) => {
                assert!(c.silent_data_loss);
                assert!(c.is_silent_data_loss());
            }
            other => panic!("unexpected record {other:?}"),
        }
    }
}
