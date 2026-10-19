//! `beadle verify` end-to-end, against the real run-18 → run-19 fixture pair.
//!
//! The acceptance contract for aae-orc-agwo8: the historical pair PASSES, the
//! inherited `<details>` table WARNS rather than fails, and three named
//! negative controls each FAIL: deleting a cluster member, resetting a `*_new`
//! axis without roll-forward, and introducing a blockquote-adjacent table.
//!
//! The controls are produced by mutating the real run-19 body here rather than
//! by checking in hand-written bad fixtures, so they stay honest: each one is
//! exactly the shipped body plus one defect.

use std::{
    path::{Path, PathBuf},
    process::Command,
};

use serde_json::Value;

mod support;

const BEFORE: &str = "docs/fixtures/vsdd-factory-312-curated-run18.md";
const CANDIDATE: &str = "docs/fixtures/vsdd-factory-312-curated-run19.md";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(rel: &str) -> String {
    let p = repo_root().join(rel);
    std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("read {}: {e}", p.display()))
}

struct Outcome {
    passed: bool,
    out: String,
}

impl Outcome {
    fn says(&self, needle: &str) -> bool {
        self.out.contains(needle)
    }
}

/// The candidate as the first dash-free run would carry it: a spaced em dash
/// in a heading becomes a colon and a space (P4), one elsewhere becomes a
/// semicolon (P1), and any other em dash a semicolon. The shipped fixtures are
/// history and keep their dashes, so the tests mutate them here, in the house
/// style of the other controls.
fn dash_free(body: &str) -> String {
    let d = support::em_dash().to_string();
    let spaced = format!(" {d} ");
    body.split('\n')
        .map(|l| {
            let heading = l.starts_with("## ") || l.starts_with("### ");
            let l = if heading {
                l.replace(&spaced, ": ")
            } else {
                l.replace(&spaced, "; ")
            };
            l.replace(&d, ";")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Run the real binary over `before` and a dash-free form of the candidate.
fn gate(before_rel: &str, candidate_body: &str) -> Outcome {
    gate_raw(before_rel, &dash_free(candidate_body))
}

/// Run the real binary over `before` and the candidate exactly as given.
fn gate_raw(before_rel: &str, candidate_body: &str) -> Outcome {
    let dir = tempfile::tempdir().expect("tempdir");
    let cand_path = dir.path().join("candidate.md");
    std::fs::write(&cand_path, candidate_body).expect("write candidate");

    let output = Command::new(env!("CARGO_BIN_EXE_beadle"))
        .arg("--root")
        .arg(repo_root())
        .args(["verify", "vsdd-factory", "--before"])
        .arg(repo_root().join(before_rel))
        .arg("--candidate")
        .arg(&cand_path)
        .output()
        .expect("run beadle verify");

    let mut out = String::from_utf8_lossy(&output.stdout).into_owned();
    out.push_str(&String::from_utf8_lossy(&output.stderr));
    Outcome {
        passed: output.status.success(),
        out,
    }
}

// ---------------------------------------------------------------- mutation ---

const OPEN: &str = "<!-- beadle-state:v1\n";
const CLOSE: &str = "\nbeadle-state -->";

/// Rewrite the candidate's sentinel through `f`, leaving the prose untouched.
fn with_sentinel(body: &str, f: impl FnOnce(&mut Value)) -> String {
    let start = body.find(OPEN).expect("sentinel open") + OPEN.len();
    let end = start + body[start..].find(CLOSE).expect("sentinel close");
    let mut state: Value = serde_json::from_str(&body[start..end]).expect("sentinel JSON");
    f(&mut state);
    format!(
        "{}{}{}",
        &body[..start],
        serde_json::to_string(&state).expect("reserialize"),
        &body[end..]
    )
}

fn drop_from_list(state: &mut Value, path: &[&str], victim: i64) {
    let mut cur = state;
    for p in path {
        cur = cur.get_mut(*p).unwrap_or_else(|| panic!("missing {p}"));
    }
    let list = cur.as_array_mut().expect("array");
    let before = list.len();
    list.retain(|v| v.as_i64() != Some(victim));
    assert_eq!(
        before - 1,
        list.len(),
        "fixture drifted: {victim} not in {path:?}"
    );
}

// --------------------------------------------------------------- the pair ---

#[test]
fn the_run18_to_run19_pair_passes() {
    let r = gate(BEFORE, &read(CANDIDATE));
    assert!(r.passed, "the shipped pair must pass:\n{}", r.out);
    assert!(r.says("GATE PASSED"), "{}", r.out);
    assert!(r.says("run 18 -> 19 | watermark 830 -> 836"), "{}", r.out);
    assert!(r.says("tracked issue numbers: 407 -> 411"), "{}", r.out);
}

#[test]
fn the_first_dash_free_run_passes_with_retirement_warnings() {
    // run19 mutated so headings lose their dash by P4 and prose by P1: the
    // first dash-free run against a dash-bearing before-snapshot. Without the
    // retirement fold every retired heading is HEADING LOST.
    let r = gate(BEFORE, &read(CANDIDATE));
    assert!(r.passed, "the dash-free run must pass:\n{}", r.out);
    assert!(r.says("2: em dash retired -> "), "{}", r.out);
    assert!(!r.says("HEADING LOST"), "{}", r.out);
}

#[test]
fn a_candidate_that_keeps_the_old_dashes_is_refused() {
    let r = gate_raw(BEFORE, &read(CANDIDATE));
    assert!(!r.passed, "{}", r.out);
    assert!(r.says("9: em dash in candidate"), "{}", r.out);
}

/// `beadle verify` over a scratch root whose store may hold one issue title.
fn gate_with_store_title(title: Option<&str>, candidate: &str) -> Outcome {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    std::fs::create_dir_all(root.join("targets")).unwrap();
    std::fs::copy(
        repo_root().join("targets/vsdd-factory.verify.json"),
        root.join("targets/vsdd-factory.verify.json"),
    )
    .unwrap();
    if let Some(title) = title {
        let store = root.join("store/vsdd-factory");
        std::fs::create_dir_all(&store).unwrap();
        let row = serde_json::json!({
            "kind": "issue", "ts": "2026-09-09T00:00:00Z", "target": "vsdd-factory",
            "number": 9, "observed_in_run": 19, "title": title, "author": "x",
            "state": "open", "created_at": "2026-09-01T00:00:00Z",
            "updated_at": "2026-09-02T00:00:00Z", "body_len": 1, "body_sha256": "s",
        });
        std::fs::write(store.join("state.jsonl"), format!("{row}\n")).unwrap();
    }
    let cand_path = root.join("candidate.md");
    std::fs::write(&cand_path, candidate).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_beadle"))
        .arg("--root")
        .arg(root)
        .args(["verify", "vsdd-factory", "--before"])
        .arg(repo_root().join(BEFORE))
        .arg("--candidate")
        .arg(&cand_path)
        .output()
        .expect("run beadle verify");
    let mut out = String::from_utf8_lossy(&output.stdout).into_owned();
    out.push_str(&String::from_utf8_lossy(&output.stderr));
    Outcome {
        passed: output.status.success(),
        out,
    }
}

#[test]
fn verify_exempts_a_title_held_in_the_store_and_only_that() {
    let title = format!("Fix parser {} handle eof", support::em_dash());
    let cand = format!("{}\n- #9 {title}\n", dash_free(&read(CANDIDATE)));
    let with = gate_with_store_title(Some(&title), &cand);
    assert!(with.passed, "a stored title is quoted as written:\n{}", with.out);
    // Control: the same body with no such title in the store is refused.
    let without = gate_with_store_title(None, &cand);
    assert!(!without.passed, "{}", without.out);
    assert!(without.says("9: em dash in candidate"), "{}", without.out);
}

#[test]
fn the_inherited_details_table_warns_rather_than_failing() {
    // The live body carries one <details> holding one 30-row table. An absolute
    // render-integrity check would block every future post on it; the gate is
    // regression-relative, so it warns.
    let r = gate(BEFORE, &read(CANDIDATE));
    assert!(r.passed, "{}", r.out);
    assert!(
        r.says("WARN 30 pre-existing render-integrity violation(s)"),
        "inherited debt must surface as a warning:\n{}",
        r.out
    );
    assert!(r.says("5d pipe-table row inside <details>"), "{}", r.out);
    assert!(
        !r.says("NEW render-integrity violation"),
        "nothing new was introduced:\n{}",
        r.out
    );
}

#[test]
fn declared_renames_warn_with_their_justification() {
    let r = gate(BEFORE, &read(CANDIDATE));
    assert!(r.says("declared rename"), "{}", r.out);
    assert!(
        r.says("run-18 carry precedent"),
        "the allowlist justification must be printed, not just the rename:\n{}",
        r.out
    );
}

// -------------------------------------------------------- negative control ---

#[test]
fn control_a_deleting_a_cluster_member_fails() {
    let bad = with_sentinel(&read(CANDIDATE), |s| {
        drop_from_list(s, &["clusters", "scratch-isolation"], 830)
    });
    let r = gate(BEFORE, &bad);
    assert!(
        !r.passed,
        "a dropped cluster member must fail the gate:\n{}",
        r.out
    );
    assert!(
        r.says("3b: axis 'clusters.scratch-isolation' lost 1 issue(s): [830]"),
        "{}",
        r.out
    );
}

#[test]
fn control_a2_a_loss_masked_by_another_axis_still_fails() {
    // The run-19 bug in miniature: the number survives the UNION check because
    // it reappears elsewhere, so only the per-axis check can see the loss.
    let bad = with_sentinel(&read(CANDIDATE), |s| {
        drop_from_list(s, &["clusters", "scratch-isolation"], 830);
        s["clusters"]["observability"]
            .as_array_mut()
            .expect("array")
            .push(Value::from(830));
    });
    let r = gate(BEFORE, &bad);
    assert!(
        !r.passed,
        "per-axis preservation must catch this:\n{}",
        r.out
    );
    assert!(
        !r.says("dropped from state"),
        "the union check is blind here by construction; if it fired, the \
         control no longer proves per-axis is doing the work:\n{}",
        r.out
    );
    assert!(
        r.says("3b: axis 'clusters.scratch-isolation' lost 1 issue(s): [830]"),
        "{}",
        r.out
    );
}

#[test]
fn control_b_resetting_a_per_run_axis_without_roll_forward_fails() {
    // run-18's operational_impact.degraded_new held 11 members; run-19 rolled
    // them into degraded_prior. Empty that counterpart and the 11 vanish.
    let bad = with_sentinel(&read(CANDIDATE), |s| {
        s["operational_impact"]["degraded_prior"] = Value::Array(vec![]);
    });
    let r = gate(BEFORE, &bad);
    assert!(!r.passed, "a silent per-run reset must fail:\n{}", r.out);
    assert!(
        r.says(
            "3c: 'operational_impact.degraded_new' reset without rolling \
             [762, 788, 793, 794, 799, 809, 810, 811, 812, 822, 828] into \
             'operational_impact.degraded_prior'"
        ),
        "{}",
        r.out
    );
}

#[test]
fn control_b2_quick_wins_reset_without_roll_forward_fails() {
    let bad = with_sentinel(&read(CANDIDATE), |s| {
        let list = s["quick_wins_prior"].as_array_mut().expect("array");
        list.retain(|v| !matches!(v.as_i64(), Some(791) | Some(812) | Some(822)));
    });
    let r = gate(BEFORE, &bad);
    assert!(!r.passed, "{}", r.out);
    assert!(
        r.says(
            "3c: 'quick_wins_new' reset without rolling [791, 812, 822] into 'quick_wins_prior'"
        ),
        "{}",
        r.out
    );
}

#[test]
fn control_c_introducing_a_blockquote_adjacent_table_fails() {
    // GFM lazy continuation swallows a table that hugs a blockquote; the
    // run-14→16 attn-lane regression.
    let bad = read(CANDIDATE).replace(
        "## Controls",
        "> a fresh pull-quote\n| col | col |\n| --- | --- |\n| a | b |\n\n## Controls",
    );
    let r = gate(BEFORE, &bad);
    assert!(
        !r.passed,
        "a new blockquote-adjacent table must fail:\n{}",
        r.out
    );
    assert!(
        r.says("5a: NEW render-integrity violation: table directly follows blockquote"),
        "{}",
        r.out
    );
}

#[test]
fn control_d_an_undeclared_heading_rename_fails() {
    let bad = read(CANDIDATE).replace("## Controls", "## Control Surface");
    let r = gate(BEFORE, &bad);
    assert!(!r.passed, "{}", r.out);
    assert!(r.says("2: HEADING LOST -> ## Controls"), "{}", r.out);
}

#[test]
fn control_e_a_counter_bump_is_not_an_issue_loss() {
    // a4_windows maps issue number -> window count. Bumping the VALUES must not
    // read as tracked issues appearing and disappearing.
    let bad = with_sentinel(&read(CANDIDATE), |s| {
        let windows = s["a4_windows"].as_object_mut().expect("object");
        for v in windows.values_mut() {
            if let Some(n) = v.as_i64() {
                *v = Value::from(n + 100);
            }
        }
    });
    let r = gate(BEFORE, &bad);
    assert!(r.passed, "counter values are not issue numbers:\n{}", r.out);
}
