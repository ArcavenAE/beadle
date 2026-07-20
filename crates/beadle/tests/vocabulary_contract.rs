//! SKILL↔binary vocabulary contract (finding-020 F2 / beadle#32).
//!
//! The classification vocabulary lives once, in
//! `skills/beadle-triage/vocabulary.json`; the binary embeds it and SKILL §3
//! prose defers to it. These tests are the tripwire that keeps the three
//! surfaces — manifest, SKILL prose, committed rich fixtures — from drifting
//! apart silently, which is exactly how the pre-alignment enum went wrong
//! (the validator's vocabulary diverged from the spec's, and validation
//! green-lit the divergence).

use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
};

use serde_json::Value;

fn repo_path(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

fn manifest() -> Value {
    let raw = std::fs::read_to_string(repo_path("skills/beadle-triage/vocabulary.json")).unwrap();
    serde_json::from_str(&raw).unwrap()
}

fn axis(m: &Value, key: &str) -> Vec<String> {
    m[key]
        .as_array()
        .unwrap_or_else(|| panic!("manifest axis `{key}` missing"))
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect()
}

fn skill_text() -> String {
    std::fs::read_to_string(repo_path("skills/beadle-triage/SKILL.md")).unwrap()
}

#[test]
fn manifest_axes_are_nonempty_and_unique() {
    let m = manifest();
    for key in [
        "report_type",
        "defect_nature",
        "reproducibility",
        "operational_impact",
        "priority",
    ] {
        let vals = axis(&m, key);
        assert!(!vals.is_empty(), "axis `{key}` empty");
        let set: BTreeSet<_> = vals.iter().collect();
        assert_eq!(set.len(), vals.len(), "axis `{key}` has duplicates");
    }
}

#[test]
fn operational_impact_is_exactly_the_finding_009_axis() {
    let m = manifest();
    assert_eq!(
        axis(&m, "operational_impact"),
        vec!["panic", "halt", "data_loss", "degraded", "none"],
        "operational_impact must be the finding-009 liveness vocabulary"
    );
}

#[test]
fn skill_names_the_manifest_as_canonical() {
    assert!(
        skill_text().contains("vocabulary.json"),
        "SKILL.md must point executors at vocabulary.json — that pointer is \
         what makes the manifest canonical for the skill side"
    );
}

/// Every `impact.<token>` label mentioned in SKILL prose must be a manifest
/// label, and every manifest label must appear in SKILL prose. This is the
/// live SKILL↔binary drift tripwire.
#[test]
fn skill_impact_labels_match_manifest() {
    let m = manifest();
    let labels: BTreeSet<String> = m["operational_impact_labels"]
        .as_object()
        .unwrap()
        .values()
        .map(|v| v.as_str().unwrap().to_string())
        .collect();

    let text = skill_text();
    let mut in_skill: BTreeSet<String> = BTreeSet::new();
    for (idx, _) in text.match_indices("impact.") {
        let tail = &text[idx + "impact.".len()..];
        let token: String = tail
            .chars()
            .take_while(|c| c.is_ascii_lowercase() || *c == '-')
            .collect();
        // Trim a trailing hyphen left by prose like `impact.*`-style mentions.
        let token = token.trim_end_matches('-').to_string();
        if !token.is_empty() {
            in_skill.insert(format!("impact.{token}"));
        }
    }

    assert_eq!(
        in_skill, labels,
        "SKILL.md impact.* labels and manifest operational_impact_labels drifted"
    );
}

/// The committed rich fixtures are the empirical record of what the skill's
/// graders actually emit on the aligned axes — every value they use must be
/// ingestable. (report_type and operational_impact only: defect_nature and
/// reproducibility shorthand in fixtures is normalized by the skill before
/// ingest and is out of scope here.)
#[test]
fn fixture_vocabulary_is_ingestable() {
    let m = manifest();
    let report_types: BTreeSet<String> = axis(&m, "report_type").into_iter().collect();
    let impacts: BTreeSet<String> = axis(&m, "operational_impact").into_iter().collect();

    let fixture_dir = repo_path("docs/fixtures");
    let mut checked = 0;
    for entry in std::fs::read_dir(&fixture_dir).unwrap() {
        let path = entry.unwrap().path();
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        if !name.ends_with("-classifications-rich.json") {
            continue;
        }
        let raw = std::fs::read_to_string(&path).unwrap();
        let recs: Vec<Value> = serde_json::from_str(&raw).unwrap();
        for r in &recs {
            if let Some(rt) = r["report_type"].as_str() {
                assert!(
                    report_types.contains(rt),
                    "{name}: report_type `{rt}` not in manifest"
                );
            }
            if let Some(oi) = r["operational_impact"].as_str() {
                assert!(
                    impacts.contains(oi),
                    "{name}: operational_impact `{oi}` not in manifest"
                );
            }
        }
        checked += 1;
    }
    assert!(
        checked >= 2,
        "expected at least the run-12 + run-13 fixtures"
    );
}

#[test]
fn legacy_map_and_labels_are_consistent_with_the_axis() {
    let m = manifest();
    let impacts: BTreeSet<String> = axis(&m, "operational_impact").into_iter().collect();
    for (legacy, mapped) in m["legacy_operational_impact"].as_object().unwrap() {
        assert!(
            !impacts.contains(legacy.as_str()),
            "`{legacy}` is both legacy and canonical"
        );
        if let Some(t) = mapped.as_str() {
            assert!(
                impacts.contains(t),
                "legacy `{legacy}` maps to unknown token `{t}`"
            );
        }
    }
    for token in m["operational_impact_labels"].as_object().unwrap().keys() {
        assert!(
            impacts.contains(token.as_str()),
            "label key `{token}` not a canonical impact token"
        );
    }
}
