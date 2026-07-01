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
//!     "operational_impact": "silent-data-loss",
//!     "priority": "P0",
//!     "cluster": ["ratchet-integrity"],
//!     "quick_win_eligible": false,
//!     "rationale": "PASS certified against uncommitted artifacts (finding-004)",
//!     "cited_evidence": ".factory/logs/D-042.md line 118"
//!   }
//!
//! Timestamps default to now (RFC3339); an explicit `ts` in the payload wins
//! so the skill can replay historical classifications deterministically.

use anyhow::{anyhow, bail, Context, Result};
use beadle_store::{ClassificationRecord, Record, Store};
use serde_json::Value;
use std::fs;
use std::io::{self, Read};
use std::path::Path;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

use crate::intent;

/// Recognized values on the four bounded axes. These match
/// `elem-defect-classification-superset`; keep the vocabulary tight so
/// downstream signals aren't decoding free-text.
const REPORT_TYPES: &[&str] = &[
    "task",
    "bug",
    "feature",
    "regression",
    "security",
    "dependency",
    "ci-build",
    "flaky-test",
    "tech-debt",
    "perf",
    "docs",
    "question",
    "rfc",
];
const DEFECT_NATURES: &[&str] = &[
    "syntax",
    "off-by-one",
    "null-resource-lifecycle",
    "concurrency-race",
    "logic",
    "algorithmic",
    "spec-requirements",
    "design-architectural",
    "directional-intent-misalignment",
];
const REPRODUCIBILITY: &[&str] = &["bohrbug", "mandelbug", "heisenbug", "unknown"];
const OPERATIONAL_IMPACT: &[&str] = &[
    "silent-data-loss",
    "silent-corruption",
    "false-verdict",
    "user-visible-error",
    "degraded-performance",
    "none",
];

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
    let rec_target = obj
        .get("target")
        .and_then(Value::as_str)
        .unwrap_or(target);
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

    let report_type = req_enum(obj, "report_type", REPORT_TYPES)?;
    let defect_nature = req_enum(obj, "defect_nature", DEFECT_NATURES)?;
    let reproducibility = req_enum(obj, "reproducibility", REPRODUCIBILITY)?;
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
    let operational_impact = match obj.get("operational_impact").and_then(Value::as_str) {
        Some(s) => {
            if !OPERATIONAL_IMPACT.contains(&s) {
                bail!(
                    "operational_impact `{}` not in {:?}",
                    s,
                    OPERATIONAL_IMPACT
                );
            }
            Some(s.to_string())
        }
        None => None,
    };

    if integrity && integrity_anchor.is_none() {
        bail!("integrity=true requires `integrity_anchor` naming the systems-of-record tier");
    }

    let priority = req_str(obj, "priority")?;

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

fn req_enum(
    obj: &serde_json::Map<String, Value>,
    key: &str,
    allowed: &[&str],
) -> Result<String> {
    let v = req_str(obj, key)?;
    if !allowed.contains(&v.as_str()) {
        bail!("`{}` value `{}` not in {:?}", key, v, allowed);
    }
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tempfile::TempDir;

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
        assert!(err.to_string().contains("payload item 0") || err.chain().any(|c| c.to_string().contains("vibes")));
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
}
