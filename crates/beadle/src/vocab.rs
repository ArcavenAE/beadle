//! Shared classification vocabulary — single source of truth for the bounded
//! axes, embedded from `skills/beadle-triage/vocabulary.json` at build time.
//!
//! finding-020 F2 / beadle#32: the binary's enums and SKILL §3 drifted because
//! each carried its own copy of the vocabulary, and validation green-lit the
//! divergence. The manifest is canonical for BOTH the skill (its §3 prose
//! defers to the file) and this binary; `tests/vocabulary_contract.rs` holds
//! the two together so they cannot drift silently again.

use std::{collections::BTreeMap, sync::OnceLock};

use anyhow::{Context, Result};
use serde::Deserialize;

const MANIFEST: &str = include_str!("../../../skills/beadle-triage/vocabulary.json");

#[derive(Debug, Deserialize)]
pub struct Vocabulary {
    #[allow(dead_code)] // read by the contract tests; versions future migrations
    pub schema_version: u32,
    pub report_type: Vec<String>,
    pub defect_nature: Vec<String>,
    pub reproducibility: Vec<String>,
    /// finding-009 liveness tokens. The safety cluster lives in the record's
    /// `integrity` / `integrity_anchor` / `silent_data_loss` fields — never here.
    /// (The manifest also carries `operational_impact_labels`, the store-token →
    /// GitHub-label map; the binary doesn't consume it yet, so it is not
    /// deserialized here — the contract tests read it from the JSON directly.)
    pub operational_impact: Vec<String>,
    pub priority: Vec<String>,
    /// Pre-finding-009 severity-flavored values → canonical liveness token,
    /// or None where the legacy value is ambiguous and migration needs
    /// per-issue ground truth (the rich classification fixtures).
    pub legacy_operational_impact: BTreeMap<String, Option<String>>,
}

impl Vocabulary {
    pub fn is_legacy_impact(&self, v: &str) -> bool {
        self.legacy_operational_impact.contains_key(v)
    }
}

/// The embedded manifest. A parse failure here is a build defect (the manifest
/// ships inside the binary), caught by `vocabulary_contract` tests before any
/// release — so an `expect` with an actionable message is the honest shape.
pub fn vocab() -> &'static Vocabulary {
    static V: OnceLock<Vocabulary> = OnceLock::new();
    V.get_or_init(|| {
        parse().expect(
            "embedded skills/beadle-triage/vocabulary.json failed to parse — \
             fix the manifest and re-run tests/vocabulary_contract.rs",
        )
    })
}

fn parse() -> Result<Vocabulary> {
    serde_json::from_str(MANIFEST).context("parse skills/beadle-triage/vocabulary.json")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_manifest_parses() {
        let v = parse().unwrap();
        assert_eq!(v.schema_version, 1);
        assert!(!v.report_type.is_empty());
    }

    #[test]
    fn legacy_map_covers_no_canonical_token() {
        let v = vocab();
        for legacy in v.legacy_operational_impact.keys() {
            assert!(
                !v.operational_impact.contains(legacy),
                "`{legacy}` is both legacy and canonical"
            );
        }
    }
}
