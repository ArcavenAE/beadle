//! `beadle note` — append a NoteRecord to a target's store.
//!
//! The Phase-0 path for durable run annotations that aren't classifications:
//! per-dispatch perf ledger entries (`--topic perf`), migration audit trails,
//! operator remarks. Notes are diagnostic — nothing gates on them (ADR-007
//! diagnostic-not-gate); a typed record kind (e.g. PerfRecord) arrives only
//! when a consumer needs structure the JSON-in-text can't carry.

use std::path::Path;

use anyhow::{bail, Context, Result};
use beadle_store::{NoteRecord, Record, Store};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use crate::intent;

pub fn run(root: &Path, target: &str, topic: &str, text: &str, run: Option<u32>) -> Result<()> {
    let intent = intent::load(root, target)?;
    if intent.repo.is_empty() {
        bail!("intent for target `{target}` has empty repo");
    }
    if topic.trim().is_empty() {
        bail!("--topic must be non-empty");
    }
    if text.trim().is_empty() {
        bail!("--text must be non-empty");
    }

    let store = Store::open(root.join("store"), target)?;
    let run = match run {
        Some(r) => r,
        None => store.latest_run()?.map(|r| r.run).unwrap_or(0),
    };
    let ts = OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_string());

    store
        .append(&[Record::Note(NoteRecord {
            ts,
            target: target.to_string(),
            run,
            topic: topic.to_string(),
            text: text.to_string(),
        })])
        .context("append note record")?;

    eprintln!("beadle note: appended topic={topic} run={run} to target={target}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use beadle_store::{Counts, RunRecord};
    use tempfile::TempDir;

    use super::*;

    fn setup(td: &TempDir) {
        let tdir = td.path().join("targets");
        std::fs::create_dir_all(&tdir).unwrap();
        std::fs::write(
            tdir.join("t.intent.yaml"),
            "schema_version: 0.1\nrepo: acme/widget\nmaintainers:\n  - alice\nmeasured_contributors:\n  - bot\n",
        )
        .unwrap();
    }

    fn read_notes(td: &TempDir) -> Vec<NoteRecord> {
        let store = Store::open(td.path().join("store"), "t").unwrap();
        store
            .read_all()
            .unwrap()
            .into_iter()
            .filter_map(|r| match r {
                Record::Note(n) => Some(n),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn appends_note_with_explicit_run() {
        let td = TempDir::new().unwrap();
        setup(&td);
        run(td.path(), "t", "perf", r#"{"grader":1,"outcome":"ok"}"#, Some(14)).unwrap();
        let notes = read_notes(&td);
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].topic, "perf");
        assert_eq!(notes[0].run, 14);
        assert!(notes[0].text.contains("grader"));
    }

    #[test]
    fn run_defaults_to_latest_run_record() {
        let td = TempDir::new().unwrap();
        setup(&td);
        let store = Store::open(td.path().join("store"), "t").unwrap();
        store
            .append(&[Record::Run(RunRecord {
                ts: "2026-07-20T00:00:00Z".into(),
                target: "t".into(),
                run: 7,
                watermark_before: 0,
                watermark_after: 10,
                counts: Counts::default(),
                digest: "d".into(),
                warmup: None,
                intent_version: None,
                new_this_run: vec![],
                notes: None,
            })])
            .unwrap();
        run(td.path(), "t", "perf", "phase summary", None).unwrap();
        let notes = read_notes(&td);
        assert_eq!(notes[0].run, 7);
    }

    #[test]
    fn rejects_empty_topic_or_text() {
        let td = TempDir::new().unwrap();
        setup(&td);
        assert!(run(td.path(), "t", "", "x", None).is_err());
        assert!(run(td.path(), "t", "perf", "  ", None).is_err());
    }
}
