//! `beadle classify migrate-impact` — re-map legacy (pre-finding-009)
//! `operational_impact` values in an existing store to the canonical liveness
//! vocabulary, and backfill the `silent_data_loss` safety flag.
//!
//! Runs 10–13 were ingested through a lossy map (beadle#32): `panic` and
//! `halt` both became `user-visible-error`, `degraded` became
//! `degraded-performance`, `data_loss` became `silent-data-loss`. The reverse
//! map is ambiguous for `user-visible-error`, so ground truth comes from the
//! rich classification fixtures (`docs/fixtures/*-classifications-rich.json`),
//! keyed by issue number. Records that neither a fixture nor the unambiguous
//! mechanical map can resolve FAIL the migration loudly — nothing is written
//! unless every legacy value resolves.
//!
//! The store is rewritten atomically with a timestamped backup beside it, and
//! a `note` record (topic `migration`) is appended as the audit trail.

use std::{collections::BTreeMap, fs, path::Path};

use anyhow::{bail, Context, Result};
use beadle_store::{NoteRecord, Record, Store};
use serde::Deserialize;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use crate::vocab::vocab;

/// The slice of a rich-fixture record migration cares about. Fixtures carry
/// many more analysis fields; serde ignores what we don't name.
#[derive(Debug, Deserialize)]
struct FixtureRecord {
    number: u32,
    #[serde(default)]
    operational_impact: Option<String>,
    #[serde(default)]
    silent_data_loss: bool,
}

pub fn migrate_impact(
    root: &Path,
    target: &str,
    fixtures: &[std::path::PathBuf],
    dry_run: bool,
) -> Result<()> {
    let v = vocab();
    let store = Store::open(root.join("store"), target)?;
    let recs = store.read_all()?;

    // Ground truth by issue number; a later fixture wins on collision (the
    // newest run's re-read of an issue is the freshest classification).
    let mut truth: BTreeMap<u32, (Option<String>, bool)> = BTreeMap::new();
    for f in fixtures {
        let raw = fs::read_to_string(f).with_context(|| format!("read {}", f.display()))?;
        let frecs: Vec<FixtureRecord> =
            serde_json::from_str(&raw).with_context(|| format!("parse {}", f.display()))?;
        for fr in frecs {
            truth.insert(fr.number, (fr.operational_impact, fr.silent_data_loss));
        }
    }

    let mut remapped = 0u32;
    let mut sdl_backfilled = 0u32;
    let mut unresolved: Vec<(u32, String)> = Vec::new();
    let mut out: Vec<Record> = Vec::with_capacity(recs.len() + 1);

    for rec in recs {
        match rec {
            Record::Classification(c) => {
                let mut c2 = *c;
                // Safety-flag backfill: fixture ground truth, plus the legacy
                // encoding (SDL used to live inside operational_impact).
                let fixture_sdl = truth.get(&c2.number).map(|(_, s)| *s).unwrap_or(false);
                let legacy_sdl = c2.operational_impact.as_deref() == Some("silent-data-loss");
                if (fixture_sdl || legacy_sdl) && !c2.silent_data_loss {
                    c2.silent_data_loss = true;
                    sdl_backfilled += 1;
                }
                // Liveness re-map for legacy values only; canonical (or absent)
                // values pass through untouched.
                if let Some(cur) = c2.operational_impact.clone() {
                    if !v.operational_impact.contains(&cur) {
                        let from_fixture = truth
                            .get(&c2.number)
                            .and_then(|(t, _)| t.clone())
                            .filter(|t| v.operational_impact.contains(t));
                        let mapped = from_fixture.or_else(|| {
                            v.legacy_operational_impact
                                .get(cur.as_str())
                                .cloned()
                                .flatten()
                        });
                        match mapped {
                            Some(t) => {
                                c2.operational_impact = Some(t);
                                remapped += 1;
                            }
                            None => unresolved.push((c2.number, cur)),
                        }
                    }
                }
                out.push(Record::Classification(Box::new(c2)));
            }
            other => out.push(other),
        }
    }

    if !unresolved.is_empty() {
        let listing = unresolved
            .iter()
            .map(|(n, val)| format!("#{n} `{val}`"))
            .collect::<Vec<_>>()
            .join(", ");
        bail!(
            "{} record(s) carry an ambiguous legacy operational_impact with no \
             fixture ground truth: {listing}. Pass the rich classification \
             fixture covering these issues via --fixture; nothing was written",
            unresolved.len()
        );
    }

    eprintln!(
        "beadle classify migrate-impact: {} record(s) re-mapped, {} silent_data_loss backfilled ({} fixture record(s) loaded)",
        remapped,
        sdl_backfilled,
        truth.len()
    );
    if dry_run {
        eprintln!("dry-run: store untouched");
        return Ok(());
    }
    if remapped == 0 && sdl_backfilled == 0 {
        eprintln!("store already canonical; nothing to write");
        return Ok(());
    }

    let now = OffsetDateTime::now_utc();
    let ts = now
        .format(&Rfc3339)
        .unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_string());
    let run = store.latest_run()?.map(|r| r.run).unwrap_or(0);
    out.push(Record::Note(NoteRecord {
        ts: ts.clone(),
        target: target.to_string(),
        run,
        topic: "migration".to_string(),
        text: format!(
            "operational_impact aligned to the finding-009 liveness vocabulary \
             (beadle#32): {remapped} re-mapped, {sdl_backfilled} silent_data_loss \
             backfilled from {} fixture record(s)",
            truth.len()
        ),
    }));

    let label = ts.replace([':', '+'], "-");
    let backup = store.rewrite(&out, &label)?;
    match backup {
        Some(b) => eprintln!(
            "store rewritten; previous state backed up to {}",
            b.display()
        ),
        None => eprintln!("store rewritten (no previous state file)"),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use beadle_store::ClassificationRecord;
    use serde_json::json;
    use tempfile::TempDir;

    use super::*;

    fn legacy_rec(number: u32, impact: &str) -> Record {
        Record::Classification(Box::new(ClassificationRecord {
            ts: "2026-07-13T00:00:00Z".into(),
            target: "t".into(),
            number,
            run: 13,
            report_type: "bug".into(),
            defect_nature: "logic".into(),
            reproducibility: "bohrbug".into(),
            leverage: "systemic".into(),
            alignment: "advances".into(),
            provenance: "pilot-derived".into(),
            integrity: false,
            integrity_anchor: None,
            operational_impact: Some(impact.into()),
            silent_data_loss: false,
            priority: "P2".into(),
            cluster: vec![],
            quick_win_eligible: false,
            rationale: "r".into(),
            cited_evidence: None,
            quick_win_disqualification: None,
        }))
    }

    fn impact_of(rec: &Record) -> (Option<String>, bool) {
        match rec {
            Record::Classification(c) => (c.operational_impact.clone(), c.silent_data_loss),
            _ => panic!("not a classification"),
        }
    }

    #[test]
    fn remaps_via_fixture_ground_truth() {
        let td = TempDir::new().unwrap();
        let store = Store::open(td.path().join("store"), "t").unwrap();
        store
            .append(&[
                legacy_rec(541, "user-visible-error"),
                legacy_rec(516, "user-visible-error"),
            ])
            .unwrap();
        let fixture = json!([
            {"number": 541, "operational_impact": "panic", "silent_data_loss": false},
            {"number": 516, "operational_impact": "halt", "silent_data_loss": false}
        ]);
        let fpath = td.path().join("rich.json");
        std::fs::write(&fpath, fixture.to_string()).unwrap();

        migrate_impact(td.path(), "t", &[fpath], false).unwrap();

        let recs = store.read_all().unwrap();
        assert_eq!(impact_of(&recs[0]).0.as_deref(), Some("panic"));
        assert_eq!(impact_of(&recs[1]).0.as_deref(), Some("halt"));
        // audit note appended
        assert!(matches!(recs.last().unwrap(), Record::Note(n) if n.topic == "migration"));
        // backup exists
        let baks: Vec<_> = std::fs::read_dir(td.path().join("store").join("t"))
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.file_name()
                    .to_string_lossy()
                    .starts_with("state.jsonl.bak-")
            })
            .collect();
        assert_eq!(baks.len(), 1);
    }

    #[test]
    fn mechanical_map_handles_unambiguous_legacy_and_sets_sdl() {
        let td = TempDir::new().unwrap();
        let store = Store::open(td.path().join("store"), "t").unwrap();
        store
            .append(&[
                legacy_rec(1, "degraded-performance"),
                legacy_rec(2, "silent-data-loss"),
            ])
            .unwrap();

        migrate_impact(td.path(), "t", &[], false).unwrap();

        let recs = store.read_all().unwrap();
        assert_eq!(impact_of(&recs[0]), (Some("degraded".into()), false));
        // legacy SDL encoding → data_loss on the liveness axis + safety flag set
        assert_eq!(impact_of(&recs[1]), (Some("data_loss".into()), true));
    }

    #[test]
    fn ambiguous_legacy_without_fixture_fails_loud_and_writes_nothing() {
        let td = TempDir::new().unwrap();
        let store = Store::open(td.path().join("store"), "t").unwrap();
        store
            .append(&[legacy_rec(7, "user-visible-error")])
            .unwrap();

        let err = migrate_impact(td.path(), "t", &[], false).unwrap_err();
        assert!(err.to_string().contains("#7"));
        // untouched
        let recs = store.read_all().unwrap();
        assert_eq!(impact_of(&recs[0]).0.as_deref(), Some("user-visible-error"));
        assert_eq!(recs.len(), 1);
    }

    #[test]
    fn dry_run_reports_but_writes_nothing() {
        let td = TempDir::new().unwrap();
        let store = Store::open(td.path().join("store"), "t").unwrap();
        store
            .append(&[legacy_rec(1, "degraded-performance")])
            .unwrap();

        migrate_impact(td.path(), "t", &[], true).unwrap();

        let recs = store.read_all().unwrap();
        assert_eq!(
            impact_of(&recs[0]).0.as_deref(),
            Some("degraded-performance")
        );
        assert_eq!(recs.len(), 1);
    }

    #[test]
    fn canonical_store_is_left_untouched() {
        let td = TempDir::new().unwrap();
        let store = Store::open(td.path().join("store"), "t").unwrap();
        store.append(&[legacy_rec(1, "halt")]).unwrap();

        migrate_impact(td.path(), "t", &[], false).unwrap();

        let recs = store.read_all().unwrap();
        assert_eq!(recs.len(), 1, "no migration note when nothing changed");
    }
}
