//! Per-target classification axes, the `Requirements:` line, the evidence axis,
//! the propose-only policy and the board sections that read them (I-508).
//!
//! Every axis name, value, requirement id and owner below is invented for the
//! test. No product value is read from, or written to, the code under test.

use std::{
    fs,
    path::Path,
    process::{Command, Output},
};

use serde_json::{Value, json};
use tempfile::TempDir;

const AXES: &str = "\
axes:
  - name: lane
    scoped: true
    propose: label
    values: [alpha, beta, gamma]
  - name: stage
    scoped: true
    propose: row_prefix
    values: [early, late]
    prefixes:
      EA: early
      LA: late
  - name: theme
    scoped: false
    propose: label
    values: [t1, t2, t3]
  - name: pace
    scoped: false
    propose: marker
    values: [fast]
";

const REQS: &str = "\
requirements:
  pattern: \"{a}-{n}{l?}\"
  list: requirements.txt
  version: 3
";

const LIST: &str = "# version 3, ids are never renumbered\nEA-1\nEA-2\nEA-2a\nLA-1\nLA-2\n";

fn intent(repo: &str, extra: &str) -> String {
    format!("schema_version: 0.7\ntarget:\n  repo: {repo}\nmaintainers:\n  - alice\n{extra}")
}

fn run_row(run: u32) -> Value {
    json!({"kind": "run", "ts": "2026-09-09T00:00:00Z", "target": "t", "run": run,
           "watermark_before": 0, "watermark_after": 9, "counts": {}, "digest": "", "new_this_run": []})
}

fn issue_row(number: u32, run: u32) -> Value {
    json!({"kind": "issue", "ts": "2026-09-09T00:00:00Z", "target": "t", "number": number,
           "observed_in_run": run, "title": format!("issue {number}"), "author": "bob", "state": "open",
           "created_at": "2026-09-01T00:00:00Z", "updated_at": "2026-09-02T00:00:00Z",
           "body_len": 1, "body_sha256": "s"})
}

fn class_row(number: u32, run: u32) -> Value {
    json!({"kind": "classification", "ts": "2026-09-23T00:00:00Z", "target": "t", "number": number,
           "run": run, "report_type": "bug", "defect_nature": "logic", "reproducibility": "bohrbug",
           "leverage": "systemic", "alignment": "advances", "provenance": "pilot-derived",
           "integrity": false, "priority": "P2", "rationale": "fixture"})
}

fn workspace(intent_text: &str, rows: &[Value]) -> TempDir {
    let td = TempDir::new().unwrap();
    fs::create_dir_all(td.path().join("targets")).unwrap();
    fs::write(td.path().join("targets/t.intent.yaml"), intent_text).unwrap();
    let store = td.path().join("store/t");
    fs::create_dir_all(&store).unwrap();
    let jsonl: String = rows
        .iter()
        .map(|r| format!("{}\n", serde_json::to_string(r).unwrap()))
        .collect();
    fs::write(store.join("state.jsonl"), jsonl).unwrap();
    td
}

fn beadle(td: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_beadle"))
        .args(["--root".as_ref(), td.as_os_str()])
        .args(args)
        .output()
        .expect("spawn beadle")
}

fn ingest(td: &TempDir, payload: &Value) -> Output {
    let f = td.path().join("payload.json");
    fs::write(&f, serde_json::to_string(payload).unwrap()).unwrap();
    beadle(
        td.path(),
        &["classify", "ingest", "t", "--file", f.to_str().unwrap()],
    )
}

fn stderr(o: &Output) -> String {
    String::from_utf8_lossy(&o.stderr).to_string()
}

fn stored(td: &TempDir) -> Vec<Value> {
    fs::read_to_string(td.path().join("store/t/state.jsonl"))
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}

fn last_class(td: &TempDir) -> Value {
    stored(td)
        .into_iter()
        .rev()
        .find(|r| r["kind"] == "classification")
        .unwrap()
}

fn rec(number: u32, extra: Value) -> Value {
    let mut base = class_row(number, 1);
    base.as_object_mut().unwrap().remove("kind");
    for (k, v) in extra.as_object().unwrap() {
        base[k] = v.clone();
    }
    base
}

fn full() -> TempDir {
    let td = workspace(
        &intent("acme/widget", &format!("{AXES}{REQS}")),
        &[run_row(1)],
    );
    fs::write(td.path().join("requirements.txt"), LIST).unwrap();
    td
}

#[test]
fn ingest_stores_declared_axis_values() {
    let td = full();
    let o = ingest(
        &td,
        &rec(1, json!({"axes": {"lane": ["alpha"], "theme": ["t1"]}})),
    );
    assert!(o.status.success(), "{}", stderr(&o));
    assert_eq!(
        last_class(&td)["axes"],
        json!({"lane": ["alpha"], "theme": ["t1"]})
    );
}

#[test]
fn ingest_rejects_an_axis_the_target_does_not_declare() {
    let td = full();
    let o = ingest(&td, &rec(1, json!({"axes": {"nope": ["alpha"]}})));
    assert!(!o.status.success());
    assert!(stderr(&o).contains("nope"), "{}", stderr(&o));
}

#[test]
fn ingest_rejects_a_value_outside_the_declared_list() {
    let td = full();
    let o = ingest(&td, &rec(1, json!({"axes": {"lane": ["delta"]}})));
    assert!(!o.status.success());
    assert!(stderr(&o).contains("delta"), "{}", stderr(&o));
}

#[test]
fn ingest_rejects_two_values_on_a_scoped_axis() {
    let td = full();
    let o = ingest(&td, &rec(1, json!({"axes": {"lane": ["alpha", "beta"]}})));
    assert!(!o.status.success());
    assert!(stderr(&o).contains("lane"), "{}", stderr(&o));
}

#[test]
fn ingest_accepts_several_values_on_an_unscoped_axis() {
    let td = full();
    let o = ingest(&td, &rec(1, json!({"axes": {"theme": ["t1", "t3"]}})));
    assert!(o.status.success(), "{}", stderr(&o));
    assert_eq!(last_class(&td)["axes"]["theme"], json!(["t1", "t3"]));
}

#[test]
fn a_row_prefix_axis_takes_its_value_from_the_cited_row_when_none_is_given() {
    let td = full();
    let o = ingest(&td, &rec(1, json!({"requirements": ["LA-2"]})));
    assert!(o.status.success(), "{}", stderr(&o));
    assert_eq!(last_class(&td)["axes"]["stage"], json!(["late"]));
}

#[test]
fn a_row_prefix_axis_stays_unset_when_the_cited_prefix_is_not_declared() {
    let td = full();
    let o = ingest(&td, &rec(1, json!({"requirements": ["ZZ-9"]})));
    assert!(o.status.success(), "{}", stderr(&o));
    assert!(last_class(&td)["axes"].get("stage").is_none());
    assert_eq!(last_class(&td)["requirements"], json!(["ZZ-9"]));
}

#[test]
fn ingest_rejects_evidence_values_beadle_may_not_propose() {
    for bad in ["live", "soc-confirmed"] {
        let td = full();
        let o = ingest(&td, &rec(1, json!({"evidence": bad})));
        assert!(!o.status.success(), "{bad} was accepted");
        assert!(stderr(&o).contains(bad), "{}", stderr(&o));
        assert!(stored(&td).iter().all(|r| r["kind"] != "classification"));
    }
}

#[test]
fn ingest_accepts_none_and_test_evidence() {
    for ok in ["none", "test"] {
        let td = full();
        let o = ingest(&td, &rec(1, json!({"evidence": ok})));
        assert!(o.status.success(), "{}", stderr(&o));
        assert_eq!(last_class(&td)["evidence"], json!(ok));
    }
}

#[test]
fn ingest_rejects_a_requirement_id_that_does_not_fit_the_pattern() {
    let td = full();
    let o = ingest(&td, &rec(1, json!({"requirements": ["EA-1", "bad id"]})));
    assert!(!o.status.success());
    assert!(stderr(&o).contains("bad id"), "{}", stderr(&o));
}

#[test]
fn a_split_row_id_and_its_plain_sibling_both_fit() {
    let td = full();
    let o = ingest(&td, &rec(1, json!({"requirements": ["EA-2", "EA-2a"]})));
    assert!(o.status.success(), "{}", stderr(&o));
    assert_eq!(last_class(&td)["requirements"], json!(["EA-2", "EA-2a"]));
}

#[test]
fn requirements_are_refused_where_the_target_declares_no_pattern() {
    let td = workspace(&intent("acme/widget", AXES), &[run_row(1)]);
    let o = ingest(&td, &rec(1, json!({"requirements": ["EA-1"]})));
    assert!(!o.status.success());
    assert!(
        stderr(&o).contains("declares no requirements"),
        "{}",
        stderr(&o)
    );
}

fn board(td: &TempDir) -> String {
    let o = beadle(td.path(), &["render", "t"]);
    assert!(o.status.success(), "{}", stderr(&o));
    String::from_utf8(o.stdout).unwrap()
}

fn seeded(intent_text: &str) -> TempDir {
    let td = workspace(
        intent_text,
        &[
            run_row(1),
            issue_row(1, 1),
            issue_row(2, 1),
            issue_row(3, 1),
            class_row(3, 1),
        ],
    );
    fs::write(td.path().join("requirements.txt"), LIST).unwrap();
    for (n, extra) in [
        (
            1,
            json!({"axes": {"lane": ["alpha"]}, "evidence": "test", "requirements": ["EA-1", "EA-2a"]}),
        ),
        (
            2,
            json!({"axes": {"lane": ["alpha"]}, "evidence": "none", "requirements": ["EA-1", "QQ-7"]}),
        ),
    ] {
        let o = ingest(&td, &rec(n, extra));
        assert!(o.status.success(), "{}", stderr(&o));
    }
    td
}

#[test]
fn the_board_shows_effort_by_axis_and_by_evidence() {
    let b = board(&seeded(&intent("acme/widget", &format!("{AXES}{REQS}"))));
    assert!(b.contains("Effort by lane"), "{b}");
    assert!(b.contains("| alpha | 2 |"), "{b}");
    assert!(b.contains("| beta | 0 |"), "{b}");
    assert!(b.contains("Effort by evidence"), "{b}");
    assert!(b.contains("| test | 1 |"), "{b}");
    assert!(b.contains("| none | 1 |"), "{b}");
}

#[test]
fn the_board_shows_scope_coverage_against_the_listed_requirements() {
    let b = board(&seeded(&intent("acme/widget", &format!("{AXES}{REQS}"))));
    assert!(b.contains("Scope coverage"), "{b}");
    assert!(b.contains("version 3"), "{b}");
    assert!(b.contains("| listed | 5 |"), "{b}");
    assert!(b.contains("| cited and listed | 2 |"), "{b}");
    assert!(
        b.contains("| listed, not cited | EA-2, LA-1, LA-2 |"),
        "{b}"
    );
    assert!(b.contains("| cited, not listed | QQ-7 |"), "{b}");
}

#[test]
fn axes_without_a_requirements_block_render_without_scope_coverage() {
    let td = seeded(&intent("acme/widget", &format!("{AXES}{REQS}")));
    fs::write(
        td.path().join("targets/t.intent.yaml"),
        intent("acme/widget", AXES),
    )
    .unwrap();
    let b = board(&td);
    assert!(b.contains("Effort by lane"), "{b}");
    assert!(!b.contains("Scope coverage"), "{b}");
}

#[test]
fn rows_written_before_the_new_fields_still_load_and_count_as_unassigned() {
    let td = seeded(&intent("acme/widget", &format!("{AXES}{REQS}")));
    let b = board(&td);
    assert!(b.contains("| unassigned | 1 |"), "{b}");
}

fn strip_clock(body: &str) -> String {
    // The clock and the digest that covers it are the only run-to-run differences.
    body.lines()
        .filter(|l| {
            let l = l.trim_start();
            !l.starts_with("\"generated_at\"")
                && !l.starts_with("\"derived_digest\"")
                && !l.starts_with("_derived at ")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn a_target_without_axes_renders_byte_for_byte_as_before() {
    let td = workspace(
        &intent("acme/widget", ""),
        &[
            run_row(1),
            issue_row(1, 1),
            issue_row(2, 1),
            class_row(1, 1),
            class_row(2, 1),
        ],
    );
    let got = strip_clock(&board(&td));
    let golden = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/no_axes_board.md");
    if let Ok(to) = std::env::var("BLESS_BOARD") {
        fs::write(to, &got).unwrap();
    }
    assert_eq!(got, fs::read_to_string(&golden).unwrap());
}

#[test]
fn the_repo_local_intent_file_is_read_when_the_targets_file_is_absent() {
    let td = workspace(&intent("acme/widget", ""), &[run_row(1), class_row(1, 1)]);
    fs::remove_file(td.path().join("targets/t.intent.yaml")).unwrap();
    fs::create_dir_all(td.path().join(".beadle")).unwrap();
    fs::write(
        td.path().join(".beadle/intent.yaml"),
        intent("local/only", ""),
    )
    .unwrap();
    assert!(board(&td).contains("local/only"));
}

#[test]
fn the_targets_file_wins_when_both_intent_files_exist() {
    let td = workspace(&intent("acme/widget", ""), &[run_row(1), class_row(1, 1)]);
    fs::create_dir_all(td.path().join(".beadle")).unwrap();
    fs::write(
        td.path().join(".beadle/intent.yaml"),
        intent("local/only", ""),
    )
    .unwrap();
    let b = board(&td);
    assert!(b.contains("acme/widget"), "{b}");
    assert!(!b.contains("local/only"), "{b}");
}

fn plan_lines(o: &Output) -> Vec<String> {
    stderr(o)
        .lines()
        .filter(|l| l.starts_with("label-plan:"))
        .map(String::from)
        .collect()
}

fn planned(config: Option<&str>, repo: &str) -> Vec<String> {
    let td = workspace(&intent(repo, ""), &[run_row(1)]);
    if let Some(c) = config {
        fs::create_dir_all(td.path().join(".beadle")).unwrap();
        fs::write(td.path().join(".beadle/propose-only.yaml"), c).unwrap();
    }
    let o = ingest(&td, &rec(1, json!({"evidence": "test"})));
    assert!(o.status.success(), "{}", stderr(&o));
    plan_lines(&o)
}

fn propose_only(repo: &str) -> Vec<String> {
    vec![format!("label-plan: evidence.test {repo} propose-only")]
}

fn apply(repo: &str) -> Vec<String> {
    vec![format!("label-plan: evidence.test {repo} apply")]
}

#[test]
fn a_missing_policy_file_means_propose_only_everywhere() {
    assert_eq!(planned(None, "acme/widget"), propose_only("acme/widget"));
}

#[test]
fn an_empty_policy_file_means_propose_only_everywhere() {
    assert_eq!(
        planned(Some(""), "acme/widget"),
        propose_only("acme/widget")
    );
    assert_eq!(
        planned(Some("apply_owners: []\n"), "acme/widget"),
        propose_only("acme/widget")
    );
}

#[test]
fn an_unparsable_policy_file_means_propose_only_everywhere() {
    for text in ["apply_owners: [unclosed\n", "apply_owners: 12\n"] {
        assert_eq!(
            planned(Some(text), "acme/widget"),
            propose_only("acme/widget"),
            "{text}"
        );
    }
}

#[test]
fn a_file_using_the_old_deny_list_key_means_propose_only_everywhere() {
    assert_eq!(
        planned(Some("owners:\n  - other\n"), "acme/widget"),
        propose_only("acme/widget")
    );
}

#[test]
fn only_an_owner_the_list_names_may_be_applied_to() {
    let cfg = "apply_owners:\n  - acme\n";
    assert_eq!(planned(Some(cfg), "acme/widget"), apply("acme/widget"));
    assert_eq!(
        planned(Some(cfg), "other/widget"),
        propose_only("other/widget")
    );
}

#[test]
fn an_owner_the_policy_has_never_heard_of_is_propose_only() {
    // control: the same file does allow the owners it names
    assert_eq!(
        planned(Some("apply_owners:\n  - acme\n  - second\n"), "second/tool"),
        apply("second/tool")
    );
    assert_eq!(
        planned(
            Some("apply_owners:\n  - acme\n  - second\n"),
            "brand-new-org/tool"
        ),
        propose_only("brand-new-org/tool")
    );
}

#[test]
fn owners_are_compared_without_regard_to_case() {
    assert_eq!(
        planned(Some("apply_owners: [ACME]\n"), "acme/widget"),
        apply("acme/widget")
    );
    assert_eq!(
        planned(Some("apply_owners: [acme]\n"), "ACME/widget"),
        apply("ACME/widget")
    );
    assert_eq!(
        planned(Some("apply_owners: [ACME]\n"), "other/widget"),
        propose_only("other/widget")
    );
}

#[test]
fn one_malformed_entry_makes_the_whole_file_fail_closed() {
    // control: without the bad entry the same file is in force
    assert_eq!(
        planned(Some("apply_owners:\n  - acme\n"), "acme/widget"),
        apply("acme/widget")
    );
    for entry in ["acme/widget", "a:b", "a b", "\"\"", "\" \""] {
        let cfg = format!("apply_owners:\n  - acme\n  - {entry}\n");
        assert_eq!(
            planned(Some(&cfg), "acme/widget"),
            propose_only("acme/widget"),
            "entry {entry} left the file in force"
        );
    }
}

#[test]
fn a_target_that_is_not_an_owner_and_repo_pair_is_propose_only() {
    // control: a real owner and name in the same file is applied to
    assert_eq!(
        planned(Some("apply_owners:\n  - jira\n  - acme\n"), "acme/widget"),
        apply("acme/widget")
    );
    assert_eq!(
        planned(Some("apply_owners:\n  - jira\n  - acme\n"), "jira:PROJ"),
        propose_only("jira:PROJ")
    );
}

#[test]
fn the_policy_and_the_repo_local_intent_files_are_not_committed_here() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let ignore = fs::read_to_string(root.join(".gitignore")).unwrap();
    let lines: Vec<&str> = ignore.lines().map(str::trim).collect();
    for f in [".beadle/propose-only.yaml", ".beadle/intent.yaml"] {
        assert!(
            lines.contains(&f) || lines.contains(&".beadle/") || lines.contains(&".beadle"),
            "{f} is not ignored"
        );
    }
}

#[test]
fn the_id_pattern_is_matched_whole_and_case_sensitively() {
    let td = full();
    for good in ["EA-1", "EA-12", "EA-1a", "QQ-007"] {
        let o = ingest(&td, &rec(1, json!({"requirements": [good]})));
        assert!(o.status.success(), "{good}: {}", stderr(&o));
    }
    for bad in [
        "ea-1", "EA-", "EA-1ab", "EA-1A", "EA1", "-1", "EA-1 ", "XEA-1x2",
    ] {
        let o = ingest(&td, &rec(1, json!({"requirements": [bad]})));
        assert!(!o.status.success(), "{bad:?} was accepted");
    }
}

#[test]
fn literals_may_be_dash_dot_and_underscore() {
    let text = intent(
        "acme/widget",
        "requirements:\n  pattern: \"S.{a}_{n}-{n}\"\n  list: requirements.txt\n  version: 1\n",
    );
    let td = workspace(&text, &[run_row(1)]);
    let ok = ingest(&td, &rec(1, json!({"requirements": ["S.AB_3-14"]})));
    assert!(ok.status.success(), "{}", stderr(&ok));
    let no = ingest(&td, &rec(1, json!({"requirements": ["S.AB_3"]})));
    assert!(!no.status.success());
}

fn parse_body(td: &TempDir, body: &str) -> Output {
    let f = td.path().join("body.md");
    fs::write(&f, body).unwrap();
    beadle(
        td.path(),
        &["requirements", "t", "--file", f.to_str().unwrap()],
    )
}

fn ids(o: &Output) -> Value {
    serde_json::from_slice(&o.stdout)
        .unwrap_or_else(|_| panic!("not JSON: {}", String::from_utf8_lossy(&o.stdout)))
}

#[test]
fn the_requirements_line_of_a_body_yields_its_ids_in_order_without_repeats() {
    let td = full();
    let o = parse_body(
        &td,
        "Fixes a thing.\n\nRequirements: LA-2, EA-1, EA-2a, EA-1\n\nMore text.\n",
    );
    assert!(o.status.success(), "{}", stderr(&o));
    assert_eq!(ids(&o), json!(["LA-2", "EA-1", "EA-2a"]));
}

#[test]
fn text_outside_a_requirements_line_is_not_read_and_a_misfit_is_reported_not_returned() {
    let td = full();
    let o = parse_body(
        &td,
        "See EA-9 in the text.\nRequirements: EA-1, nonsense, ea-2\n",
    );
    assert!(o.status.success(), "{}", stderr(&o));
    assert_eq!(ids(&o), json!(["EA-1"]));
    assert!(
        stderr(&o).contains("nonsense") && stderr(&o).contains("ea-2"),
        "{}",
        stderr(&o)
    );
}

#[test]
fn a_body_with_no_requirements_line_yields_an_empty_list() {
    let td = full();
    let o = parse_body(&td, "Nothing cited here.\n");
    assert!(o.status.success(), "{}", stderr(&o));
    assert_eq!(ids(&o), json!([]));
}

#[test]
fn the_parser_is_refused_for_a_target_that_declares_no_pattern() {
    let td = workspace(&intent("acme/widget", AXES), &[run_row(1)]);
    let o = parse_body(&td, "Requirements: EA-1\n");
    assert!(!o.status.success());
    assert!(
        stderr(&o).contains("declares no requirements"),
        "{}",
        stderr(&o)
    );
}

#[test]
fn cited_requirements_are_recorded_in_the_cited_evidence() {
    let td = full();
    let o = ingest(
        &td,
        &rec(
            1,
            json!({"requirements": ["EA-1", "LA-1"], "cited_evidence": "docs/a.md line 3"}),
        ),
    );
    assert!(o.status.success(), "{}", stderr(&o));
    let c = last_class(&td)["cited_evidence"]
        .as_str()
        .unwrap()
        .to_string();
    assert!(
        c.contains("docs/a.md line 3") && c.contains("Requirements: EA-1, LA-1"),
        "{c}"
    );
}

const FORMS: &str = "\
requirements:
  pattern:
    - \"{a}-{a}-{n}{l?}\"
    - \"{a}-{n}\"
  version: 1
";

fn forms() -> TempDir {
    workspace(
        &intent("acme/widget", &format!("{AXES}{FORMS}")),
        &[run_row(1)],
    )
}

#[test]
fn a_list_of_forms_matches_an_id_that_fits_any_one_of_them() {
    let td = forms();
    for good in ["AB-CD-12", "AB-CD-12a", "AB-7"] {
        let o = ingest(&td, &rec(1, json!({"requirements": [good]})));
        assert!(o.status.success(), "{good}: {}", stderr(&o));
    }
}

#[test]
fn a_list_of_forms_refuses_an_id_that_fits_none() {
    let td = forms();
    for bad in ["AB-CD", "AB-CD-12ab", "ab-7", "AB-7-"] {
        let o = ingest(&td, &rec(1, json!({"requirements": [bad]})));
        assert!(!o.status.success(), "{bad} was accepted");
        assert!(stderr(&o).contains(bad), "{}", stderr(&o));
    }
}

#[test]
fn the_requirements_command_reads_every_form_in_the_list() {
    let td = forms();
    let o = parse_body(&td, "Requirements: AB-CD-12a, AB-7, AB-CD\n");
    assert!(o.status.success(), "{}", stderr(&o));
    assert_eq!(ids(&o), json!(["AB-CD-12a", "AB-7"]));
}

#[test]
fn a_malformed_form_in_the_list_is_named_when_the_intent_loads() {
    let text = intent(
        "acme/widget",
        "requirements:\n  pattern:\n    - \"{a}-{n}\"\n    - \"{q}\"\n",
    );
    let td = workspace(&text, &[run_row(1)]);
    let o = ingest(&td, &rec(1, json!({"requirements": ["AB-7"]})));
    assert!(!o.status.success());
    assert!(stderr(&o).contains("{q}"), "{}", stderr(&o));
}

#[test]
fn an_empty_list_of_forms_is_refused() {
    let td = workspace(
        &intent("acme/widget", "requirements:\n  pattern: []\n"),
        &[run_row(1)],
    );
    let o = ingest(&td, &rec(1, json!({"requirements": ["AB-7"]})));
    assert!(!o.status.success());
    assert!(stderr(&o).contains("pattern"), "{}", stderr(&o));
}
