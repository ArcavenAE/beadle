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
//! The payload key set is CLOSED: an unrecognized key is a hard error naming
//! the offending field (beadle#66). Serde cannot carry that rule for us —
//! `#[serde(deny_unknown_fields)]` is not dependable on a variant of an
//! internally-tagged enum, and the store struct must stay permissive so
//! `Store::read_all` can still parse historical rows. So ingest validates the
//! key set itself, against [`KNOWN_FIELDS`], before building the record.
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
//!     "cited_evidence": ".factory/logs/D-042.md line 118",
//!     "short_title": "PASS certifies uncommitted artifacts",
//!     "triage_state": "needs-triage",
//!     "attn": {"subtype": "governance", "order": "reply-needed", "why": "..."},
//!     "possibly_fixed": {"by": "PR #776", "confidence": "low", "verify": "..."}
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
use beadle_store::{Attn, ClassificationRecord, PossiblyFixed, Record, Store};
use serde_json::Value;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use crate::{intent, vocab::vocab};

/// Every key `classify ingest` recognizes on a classification payload item.
/// Anything else is rejected by name — the schema-drift alarm beadle#66 asked
/// for. `kind` is accepted (and must read `classification`) because the rich
/// fixtures carry it; the remaining 24 are the `ClassificationRecord` fields.
pub const KNOWN_FIELDS: &[&str] = &[
    "kind",
    "ts",
    "target",
    "number",
    "run",
    "report_type",
    "defect_nature",
    "reproducibility",
    "leverage",
    "alignment",
    "provenance",
    "integrity",
    "integrity_anchor",
    "operational_impact",
    "silent_data_loss",
    "priority",
    "cluster",
    "quick_win_eligible",
    "rationale",
    "cited_evidence",
    "quick_win_disqualification",
    "short_title",
    "triage_state",
    "attn",
    "possibly_fixed",
];

/// Keys of the nested `attn` object.
const ATTN_FIELDS: &[&str] = &["subtype", "order", "why"];

/// Keys of the nested `possibly_fixed` object.
const POSSIBLY_FIXED_FIELDS: &[&str] = &["by", "confidence", "verify"];

/// Closed domain for `triage_state`.
const TRIAGE_STATES: &[&str] = &["needs-triage", "accepted", "needs-information"];

/// Closed domain for `possibly_fixed.confidence`.
const FIX_CONFIDENCE: &[&str] = &["low", "medium", "high"];

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

    reject_unknown(obj, KNOWN_FIELDS, "")?;
    if let Some(kind) = obj.get("kind") {
        match kind.as_str() {
            Some("classification") => {}
            other => bail!(
                "`kind` must be \"classification\" on a classify payload, got {}",
                other
                    .map(|s| format!("`{s}`"))
                    .unwrap_or_else(|| kind.to_string())
            ),
        }
    }

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

    let short_title = opt_str(obj, "short_title", "")?;
    let triage_state = match opt_str(obj, "triage_state", "")? {
        Some(s) => {
            if !TRIAGE_STATES.iter().any(|a| *a == s) {
                bail!("`triage_state` value `{}` not in {:?}", s, TRIAGE_STATES);
            }
            Some(s)
        }
        None => None,
    };
    let attn = opt_attn(obj)?;
    let possibly_fixed = opt_possibly_fixed(obj)?;

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
        short_title,
        triage_state,
        attn,
        possibly_fixed,
    })
}

/// Reject any key not in `known`, naming the offender and where it sat.
/// `path` is `""` for the record itself, or e.g. `"attn."` for a nested object.
fn reject_unknown(obj: &serde_json::Map<String, Value>, known: &[&str], path: &str) -> Result<()> {
    let unknown: Vec<String> = obj
        .keys()
        .filter(|key| !known.iter().any(|k| k == *key))
        .map(|key| format!("{path}{key}"))
        .collect();
    if !unknown.is_empty() {
        bail!(
            "unknown field(s) {:?} in classification payload; known fields are {:?}. \
             If this is a deliberate schema change, add each one to \
             `classify::KNOWN_FIELDS` and to `ClassificationRecord` in the same commit \
             (beadle#66)",
            unknown,
            known
        );
    }
    Ok(())
}

/// An optional string: absent or JSON `null` is `None`; a non-string is an
/// error rather than a silent `None` (the beadle#66 failure mode).
fn opt_str(obj: &serde_json::Map<String, Value>, key: &str, path: &str) -> Result<Option<String>> {
    match obj.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(s)) => Ok(Some(s.clone())),
        Some(other) => bail!("`{path}{key}` must be a string or null, got {}", other),
    }
}

/// An optional nested object: absent or `null` is `None`; anything that is not
/// an object is an error.
fn opt_obj<'a>(
    obj: &'a serde_json::Map<String, Value>,
    key: &str,
) -> Result<Option<&'a serde_json::Map<String, Value>>> {
    match obj.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Object(m)) => Ok(Some(m)),
        Some(other) => bail!(
            "`{key}` must be an object or null, got {}. Historical fixtures carry \
             older encodings (a bare string subtype, or `{{type, reason, reading_order}}`); \
             normalize with scripts/backfill-classification-fields.py",
            other
        ),
    }
}

fn opt_attn(obj: &serde_json::Map<String, Value>) -> Result<Option<Attn>> {
    let Some(m) = opt_obj(obj, "attn")? else {
        return Ok(None);
    };
    reject_unknown(m, ATTN_FIELDS, "attn.")?;
    let subtype = opt_str(m, "subtype", "attn.")?
        .ok_or_else(|| anyhow!("`attn.subtype` is required when `attn` is present"))?;
    if subtype.starts_with("attn.") {
        bail!("`attn.subtype` must not carry the `attn.` prefix, got `{subtype}`");
    }
    Ok(Some(Attn {
        subtype,
        order: opt_str(m, "order", "attn.")?,
        why: opt_str(m, "why", "attn.")?,
    }))
}

fn opt_possibly_fixed(obj: &serde_json::Map<String, Value>) -> Result<Option<PossiblyFixed>> {
    let Some(m) = opt_obj(obj, "possibly_fixed")? else {
        return Ok(None);
    };
    reject_unknown(m, POSSIBLY_FIXED_FIELDS, "possibly_fixed.")?;
    let by = opt_str(m, "by", "possibly_fixed.")?.ok_or_else(|| {
        anyhow!("`possibly_fixed.by` is required when `possibly_fixed` is present")
    })?;
    let confidence = opt_str(m, "confidence", "possibly_fixed.")?.ok_or_else(|| {
        anyhow!("`possibly_fixed.confidence` is required when `possibly_fixed` is present")
    })?;
    if !FIX_CONFIDENCE.iter().any(|a| *a == confidence) {
        bail!(
            "`possibly_fixed.confidence` value `{}` not in {:?}",
            confidence,
            FIX_CONFIDENCE
        );
    }
    Ok(Some(PossiblyFixed {
        by,
        confidence,
        verify: opt_str(m, "verify", "possibly_fixed.")?,
    }))
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

    /// beadle#66 in miniature: the four late-added fields must reach the
    /// store. Before the fix these were accepted by the parser and dropped by
    /// the serialiser, with no diagnostic at any layer.
    #[test]
    fn late_added_fields_persist_to_store() {
        let td = TempDir::new().unwrap();
        setup(&td);
        let mut p = base_payload();
        p["short_title"] = json!("a short title");
        p["triage_state"] = json!("needs-information");
        p["attn"] = json!({"subtype": "direction", "order": "standing", "why": "w"});
        p["possibly_fixed"] = json!({"by": "PR #1", "confidence": "high", "verify": "v"});
        run_payload(&td, &p).unwrap();

        let store = Store::open(td.path().join("store"), "t").unwrap();
        match &store.read_all().unwrap()[0] {
            Record::Classification(c) => {
                assert_eq!(c.short_title.as_deref(), Some("a short title"));
                assert_eq!(c.triage_state.as_deref(), Some("needs-information"));
                let attn = c.attn.as_ref().unwrap();
                assert_eq!(attn.subtype, "direction");
                assert_eq!(attn.order.as_deref(), Some("standing"));
                let pf = c.possibly_fixed.as_ref().unwrap();
                assert_eq!(pf.by, "PR #1");
                assert_eq!(pf.confidence, "high");
                assert!(c.is_in_attn_lane());
            }
            other => panic!("unexpected record {other:?}"),
        }
    }

    /// An explicit `null` is the absent case, not a parse error — every
    /// run-18/19 fixture record spells the empty lane that way.
    #[test]
    fn explicit_nulls_are_the_absent_case() {
        let td = TempDir::new().unwrap();
        setup(&td);
        let mut p = base_payload();
        for key in ["short_title", "triage_state", "attn", "possibly_fixed"] {
            p[key] = json!(null);
        }
        run_payload(&td, &p).unwrap();
        let store = Store::open(td.path().join("store"), "t").unwrap();
        match &store.read_all().unwrap()[0] {
            Record::Classification(c) => {
                assert!(c.short_title.is_none() && c.triage_state.is_none());
                assert!(c.attn.is_none() && c.possibly_fixed.is_none());
                assert!(!c.is_in_attn_lane());
            }
            other => panic!("unexpected record {other:?}"),
        }
    }

    /// The store's `attn.subtype` is bare (`governance`), never `attn.governance`.
    #[test]
    fn rejects_prefixed_attn_subtype() {
        let td = TempDir::new().unwrap();
        setup(&td);
        let mut p = base_payload();
        p["attn"] = json!({"subtype": "attn.governance"});
        let err = run_payload(&td, &p).unwrap_err();
        assert!(format!("{:#}", err).contains("must not carry the `attn.` prefix"));
    }

    /// A payload key the record does not model is a hard, named failure — the
    /// schema-drift alarm that was missing when the four fields went overboard.
    #[test]
    fn rejects_unknown_payload_field_by_name() {
        let td = TempDir::new().unwrap();
        setup(&td);
        let mut p = base_payload();
        p["alignment_rationale"] = json!("advances because ...");
        let err = run_payload(&td, &p).unwrap_err();
        let msg = format!("{:#}", err);
        assert!(msg.contains("alignment_rationale"), "got: {msg}");
        assert!(msg.contains("KNOWN_FIELDS"), "got: {msg}");
    }

    /// `KNOWN_FIELDS` and the struct must not drift apart; this is the pair
    /// that silently diverged in the first place.
    #[test]
    fn known_fields_matches_the_serialized_record() {
        let td = TempDir::new().unwrap();
        setup(&td);
        run_payload(&td, &base_payload()).unwrap();
        let store = Store::open(td.path().join("store"), "t").unwrap();
        let line = std::fs::read_to_string(store.state_path()).unwrap();
        let v: Value = serde_json::from_str(line.trim()).unwrap();
        let serialized: std::collections::BTreeSet<&str> =
            v.as_object().unwrap().keys().map(String::as_str).collect();
        let known: std::collections::BTreeSet<&str> = KNOWN_FIELDS.iter().copied().collect();
        assert_eq!(
            serialized, known,
            "KNOWN_FIELDS drifted from ClassificationRecord's serialized shape"
        );
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
