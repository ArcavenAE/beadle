//! `beadle verify` — the `/dashboard-refresh` §3 regression gate, as a verb.
//!
//! Ported from `tools/dashboard-gate.py` (run-19), which is the reference
//! implementation. Pure mechanism, no judgment: it compares a candidate
//! dashboard body against the before-snapshot it would replace and refuses
//! the post when the candidate lost something the snapshot carried.
//!
//! Four semantics are load-bearing and were each learned the hard way in
//! run-19 — do not "simplify" them back out:
//!
//! 1. **Per-axis preservation, not union.** The command doc's original check 3
//!    compares the UNION of tracked issue numbers across the whole sentinel, so
//!    a loss confined to one axis is invisible whenever those same numbers
//!    appear under another axis. Run-19 dropped 11 members from
//!    `operational_impact.degraded_new` and the union check passed it.
//! 2. **Per-run axes must roll forward.** An axis named `*_new` is *expected*
//!    to reset each run, so it is exempt from (1) — but its previous members
//!    must reappear in a `*_prior` counterpart or the classification was
//!    silently discarded.
//! 3. **Render-integrity checks are REGRESSION-RELATIVE.** A violation already
//!    present in the before-snapshot warns; a newly introduced one fails. The
//!    live body carries an inherited violation (one `<details>` holding one
//!    30-row table), so an absolute check would block every future post.
//! 4. **Declared renames are allowlisted with a justification string**, read
//!    from `targets/<target>.verify.json` (or `--renames`). An entry without a
//!    stated `why` is a configuration error, not a silent pass.
//!
//! One parsing trap is worth naming: the sentinel is a single ~9 000-char JSON
//! line. Dict INT VALUES in it are counters (`a4_windows` window counts,
//! `counts` totals) and must never be read as issue numbers — only list
//! elements and digit-string KEYS are.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail, Context, Result};
use serde::Deserialize;
use serde_json::{Map, Value};

/// Axes that reset every run by design, and so are exempt from per-axis
/// preservation. Matched on the *leaf* key, so `operational_impact.degraded_new`
/// is exempt via `degraded_new`.
const PER_RUN: &[&str] = &[
    "new_this_run",
    "quick_wins_new",
    "keystone_new",
    "prior_run_burst",
    "degraded_new",
];

/// Per-run axes that must roll their members into a `*_prior` counterpart:
/// (label, `*_new` dotted path, `*_prior` dotted path).
const ROLL_FORWARD: &[(&str, &str, &str)] = &[
    ("quick_wins", "quick_wins_new", "quick_wins_prior"),
    (
        "degraded",
        "operational_impact.degraded_new",
        "operational_impact.degraded_prior",
    ),
];

/// Sections the candidate must carry (line-prefix, human label).
const REQUIRED_SECTIONS: &[(&str, &str)] = &[
    ("## Baseline", "## Baseline"),
    (
        "## 👤 Needs human reading",
        "## Needs human reading (attn lane)",
    ),
    ("## Action plan", "## Action plan"),
    ("### 🔴 P0a", "### P0a"),
    ("### 🔴 P0b", "### P0b"),
    ("### 🟠 P1", "### P1 (>=1)"),
    ("### 🟢 P", "### P2/P3 (>=1)"),
    ("## 🟦 Quick wins", "## Quick wins"),
    ("## Direction Health", "## Direction Health"),
    ("## Classification index", "## Classification index"),
    ("## Maintainer progress", "## Maintainer progress"),
    ("## Controls", "## Controls"),
];

// ---------------------------------------------------------------- config ---

/// A heading rename the operator has declared, with the justification that
/// makes it a rename rather than a loss.
#[derive(Debug, Clone, Deserialize)]
pub struct Rename {
    pub from: String,
    pub to: String,
    #[serde(default)]
    pub why: String,
}

/// Load the declared-rename allowlist. `--renames` wins; otherwise
/// `targets/<target>.verify.json` is used when it exists; otherwise empty.
fn load_renames(root: &Path, target: &str, explicit: Option<&Path>) -> Result<Vec<Rename>> {
    let path: PathBuf = match explicit {
        Some(p) => p.to_path_buf(),
        None => {
            let default = root.join(format!("targets/{target}.verify.json"));
            if !default.exists() {
                return Ok(Vec::new());
            }
            default
        }
    };

    let raw = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
    let doc: Value =
        serde_json::from_str(&raw).with_context(|| format!("parse {} as JSON", path.display()))?;
    let arr = if doc.is_array() {
        doc
    } else {
        doc.get("renames").cloned().unwrap_or(Value::Array(vec![]))
    };
    let renames: Vec<Rename> = serde_json::from_value(arr)
        .with_context(|| format!("{}: expected a list of {{from,to,why}}", path.display()))?;

    for r in &renames {
        if r.from.trim().is_empty() || r.to.trim().is_empty() {
            bail!("{}: declared rename with an empty from/to", path.display());
        }
        if r.why.trim().is_empty() {
            bail!(
                "{}: declared rename {:?} has no `why` — an allowlisted rename \
                 must carry a justification",
                path.display(),
                clip(&r.from, 60)
            );
        }
    }
    Ok(renames)
}

// --------------------------------------------------------------- sentinel ---

/// Extract the raw JSON payload between `<!-- beadle-state:v1` and
/// `beadle-state -->`.
fn sentinel_raw(text: &str) -> Option<&str> {
    const MARK: &str = "beadle-state:v1";
    let mut cursor = 0usize;
    while let Some(rel) = text[cursor..].find("<!--") {
        let open = cursor + rel;
        cursor = open + 4;

        let after = &text[cursor..];
        let lead = after.len() - after.trim_start().len();
        if !after[lead..].starts_with(MARK) {
            continue;
        }
        let head_end = cursor + lead + MARK.len();

        // `\s*\n`: the payload starts just past the last newline of the
        // whitespace run that follows the marker.
        let tail = &text[head_end..];
        let ws_len = tail.len() - tail.trim_start().len();
        let Some(last_nl) = tail[..ws_len].rfind('\n') else {
            continue;
        };
        let start = head_end + last_nl + 1;

        // `\nbeadle-state\s*-->`, first occurrence (the regex is non-greedy).
        let mut scan = start;
        while let Some(rel2) = text[scan..].find("\nbeadle-state") {
            let close = scan + rel2;
            let rest = &text[close + 1 + "beadle-state".len()..];
            let ws2 = rest.len() - rest.trim_start().len();
            if rest[ws2..].starts_with("-->") {
                return Some(&text[start..close]);
            }
            scan = close + 1;
        }
    }
    None
}

/// Parse the sentinel into its top-level object, or `None` when it is missing
/// or unparseable.
fn sentinel(text: &str) -> Option<Map<String, Value>> {
    let raw = sentinel_raw(text)?;
    match serde_json::from_str::<Value>(raw) {
        Ok(Value::Object(m)) => Some(m),
        _ => None,
    }
}

fn as_int(v: &Value) -> Option<i64> {
    match v {
        Value::Number(n) if n.is_i64() || n.is_u64() => n.as_i64(),
        _ => None,
    }
}

fn ints_in(list: &[Value]) -> BTreeSet<i64> {
    list.iter().filter_map(as_int).collect()
}

/// Every issue number the sentinel tracks, as a union.
///
/// Only list ELEMENTS and digit-string KEYS count. Dict int VALUES are counters
/// (`a4_windows` window counts, `counts` totals) and must never be read as
/// issue numbers — that was a real bug in an earlier draft of the gate.
fn nums(state: &Map<String, Value>) -> BTreeSet<i64> {
    let mut out = BTreeSet::new();
    for value in state.values() {
        match value {
            Value::Array(list) => out.extend(ints_in(list)),
            Value::Object(inner) => {
                for (k, v) in inner {
                    if let Value::Array(list) = v {
                        out.extend(ints_in(list));
                    }
                    if is_digits(k) {
                        if let Ok(n) = k.parse::<i64>() {
                            out.insert(n);
                        }
                    }
                }
            }
            _ => {}
        }
    }
    out
}

fn is_digits(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
}

/// Every cumulative axis, keyed by dotted path. Per-run axes are excluded by
/// leaf-key name: they are meant to reset, and rule 3c covers them instead.
fn axes(state: &Map<String, Value>) -> BTreeMap<String, BTreeSet<i64>> {
    let mut out = BTreeMap::new();
    for (k, value) in state {
        match value {
            Value::Array(list) if !PER_RUN.contains(&k.as_str()) => {
                out.insert(k.clone(), ints_in(list));
            }
            Value::Object(inner) => {
                for (kk, vv) in inner {
                    if let Value::Array(list) = vv {
                        if !PER_RUN.contains(&kk.as_str()) {
                            out.insert(format!("{k}.{kk}"), ints_in(list));
                        }
                    }
                }
            }
            _ => {}
        }
    }
    out
}

/// Resolve a dotted path to a set of ints, or `None` when the path is absent.
fn list_at(state: &Map<String, Value>, dotted: &str) -> Option<BTreeSet<i64>> {
    let mut cursor: &Value = &Value::Null;
    let mut obj: Option<&Map<String, Value>> = Some(state);
    for part in dotted.split('.') {
        let map = obj?;
        cursor = map.get(part)?;
        obj = cursor.as_object();
    }
    match cursor {
        Value::Array(list) => Some(ints_in(list)),
        _ => None,
    }
}

// -------------------------------------------------------- render integrity ---

#[derive(Debug, Clone, PartialEq, Eq)]
struct Violation {
    code: &'static str,
    what: &'static str,
    ctx: String,
}

fn details_blocks(text: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut pos = 0usize;
    while let Some(rel) = text[pos..].find("<details>") {
        let start = pos + rel;
        match text[start..].find("</details>") {
            Some(rel2) => {
                let end = start + rel2 + "</details>".len();
                out.push(&text[start..end]);
                pos = end;
            }
            None => break,
        }
    }
    out
}

/// Drop fenced code blocks, matching ```` ```.*?``` ```` non-greedily. An
/// unterminated fence is left in place.
fn strip_fences(block: &str) -> String {
    let mut out = String::new();
    let mut pos = 0usize;
    while let Some(rel) = block[pos..].find("```") {
        let open = pos + rel;
        match block[open + 3..].find("```") {
            Some(rel2) => {
                out.push_str(&block[pos..open]);
                pos = open + 3 + rel2 + 3;
            }
            None => break,
        }
    }
    out.push_str(&block[pos..]);
    out
}

/// A markdown table row: `^\s*\|.*\|\s*$`.
fn is_table_row(line: &str) -> bool {
    let t = line.trim();
    t.len() >= 2 && t.starts_with('|') && t.ends_with('|')
}

fn render_violations(text: &str) -> Vec<Violation> {
    let mut v = Vec::new();
    let lines: Vec<&str> = text.lines().collect();

    // 5a — GFM lazy continuation swallows a table that hugs a blockquote.
    for i in 1..lines.len() {
        if lines[i].trim_start().starts_with('|') && lines[i - 1].trim_start().starts_with('>') {
            v.push(Violation {
                code: "5a",
                what: "table directly follows blockquote",
                ctx: clip(lines[i].trim(), 80),
            });
        }
    }

    // 5b — <details> needs blank lines around its markdown body.
    for (i, line) in lines.iter().enumerate() {
        if line.contains("<details>") && line.contains("<summary>") {
            if let Some(next) = lines.get(i + 1) {
                if !next.trim().is_empty() {
                    v.push(Violation {
                        code: "5b",
                        what: "no blank line after <details><summary>",
                        ctx: clip(line.trim(), 80),
                    });
                }
            }
        }
        if line.trim() == "</details>" && i > 0 && !lines[i - 1].trim().is_empty() {
            v.push(Violation {
                code: "5b",
                what: "no blank line before </details>",
                ctx: clip(lines[i - 1].trim(), 80),
            });
        }
    }

    // 5d — pipe tables do not render inside <details>.
    for block in details_blocks(text) {
        let body = strip_fences(block);
        for row in body.lines().filter(|l| is_table_row(l)) {
            v.push(Violation {
                code: "5d",
                what: "pipe-table row inside <details>",
                ctx: clip(row.trim(), 80),
            });
        }
    }
    v
}

// ----------------------------------------------------------------- report ---

/// The gate's verdict: the stats it prints, plus warnings and failures.
#[derive(Debug, Default)]
pub struct Report {
    before_chars: usize,
    cand_chars: usize,
    before_headings: usize,
    cand_headings: usize,
    runs: Option<(i64, String)>,
    watermarks: Option<(i64, String)>,
    tracked: Option<(usize, usize)>,
    pub warns: Vec<String>,
    pub fails: Vec<String>,
}

impl Report {
    pub fn passed(&self) -> bool {
        self.fails.is_empty()
    }

    pub fn render(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "before: {} chars, {} headings\n",
            commas(self.before_chars),
            self.before_headings
        ));
        s.push_str(&format!(
            "cand  : {} chars, {} headings\n",
            commas(self.cand_chars),
            self.cand_headings
        ));
        if let (Some((rb, rc)), Some((wb, wc))) = (&self.runs, &self.watermarks) {
            s.push_str(&format!("run {rb} -> {rc} | watermark {wb} -> {wc}\n"));
        }
        if let Some((nb, nc)) = self.tracked {
            s.push_str(&format!("tracked issue numbers: {nb} -> {nc}\n"));
        }
        s.push('\n');
        for w in &self.warns {
            s.push_str(&format!("WARN {w}\n"));
        }
        if self.fails.is_empty() {
            s.push_str("GATE PASSED — all checks green\n");
        } else {
            s.push_str(&format!("GATE FAILED — {} check(s):\n", self.fails.len()));
            for f in &self.fails {
                s.push_str(&format!("  ✗ {f}\n"));
            }
        }
        s
    }
}

// ------------------------------------------------------------------- gate ---

/// Run every check. Errors only when the gate itself cannot operate (a missing
/// before-snapshot sentinel); everything else lands in the report.
pub fn evaluate(before: &str, candidate: &str, renames: &[Rename]) -> Result<Report> {
    let sb = sentinel(before)
        .ok_or_else(|| anyhow!("before-snapshot sentinel missing or unparseable"))?;
    let sc = sentinel(candidate);
    let mut rep = Report::default();

    let headings_before: Vec<&str> = headings(before);
    let headings_cand: BTreeSet<&str> = headings(candidate).into_iter().collect();
    rep.before_chars = before.chars().count();
    rep.cand_chars = candidate.chars().count();
    rep.before_headings = headings_before.len();
    rep.cand_headings = headings_cand.len();

    // ---- 1. section presence --------------------------------------------
    let run_before = sb
        .get("run")
        .and_then(as_int)
        .ok_or_else(|| anyhow!("before-snapshot sentinel has no integer `run`"))?;
    let wm_before = sb
        .get("watermark")
        .and_then(as_int)
        .ok_or_else(|| anyhow!("before-snapshot sentinel has no integer `watermark`"))?;

    match &sc {
        None => rep
            .fails
            .push("1: candidate sentinel missing or unparseable".into()),
        Some(sc) => {
            let run_cand = sc.get("run").and_then(as_int);
            if run_cand != Some(run_before + 1) {
                rep.fails.push(format!(
                    "1: run must be {}, got {}",
                    run_before + 1,
                    show(run_cand)
                ));
            }
            let wm_cand = sc.get("watermark").and_then(as_int).unwrap_or(0);
            if wm_cand <= wm_before {
                rep.fails.push(format!(
                    "1: watermark must exceed {wm_before}, got {}",
                    show(sc.get("watermark").and_then(as_int))
                ));
            }
            rep.runs = Some((run_before, show(run_cand)));
            rep.watermarks = Some((wm_before, show(sc.get("watermark").and_then(as_int))));
        }
    }

    for (prefix, label) in REQUIRED_SECTIONS {
        if !candidate.lines().any(|l| l.starts_with(prefix)) {
            rep.fails
                .push(format!("1: required section missing -> {label}"));
        }
    }

    // Direction verdict paragraph near the top, bolded.
    let post = match candidate.split_once("beadle-state -->") {
        Some((_, tail)) => tail,
        None => candidate,
    };
    let head: String = post.chars().take(3000).collect();
    if !head.contains("**Direction verdict:") {
        rep.fails
            .push("1: bolded 'Direction verdict:' line missing from the head".into());
    }

    // Carried-forward prior index(es) must survive.
    let prior_idx = run_indexes(before);
    let cand_idx = run_indexes(candidate);
    for r in &prior_idx {
        if !cand_idx.contains(r) {
            rep.fails.push(format!(
                "1/2: prior classification index for run-{r} dropped"
            ));
        }
    }
    if let Some(sc) = &sc {
        if let Some(run) = sc.get("run").and_then(as_int) {
            if !cand_idx.contains(&run.to_string()) {
                rep.fails.push(format!("1: new Run-{run} index missing"));
            }
        }
    }

    // ---- 2. no section loss (declared renames allowlisted) ---------------
    for h in &headings_before {
        if headings_cand.contains(h) {
            continue;
        }
        match renames.iter().find(|r| r.from == *h) {
            Some(r) if headings_cand.contains(r.to.as_str()) => rep.warns.push(format!(
                "2: declared rename -> {:?} => {:?} ({})",
                clip(h, 60),
                clip(&r.to, 60),
                r.why
            )),
            _ => rep
                .fails
                .push(format!("2: HEADING LOST -> {}", clip(h, 95))),
        }
    }

    // ---- 3. no coverage shrinkage ---------------------------------------
    if let Some(sc) = &sc {
        let nb = nums(&sb);
        let nc = nums(sc);
        rep.tracked = Some((nb.len(), nc.len()));
        let missing: Vec<i64> = nb.difference(&nc).copied().collect();
        if !missing.is_empty() {
            rep.fails.push(format!(
                "3: {} tracked issue(s) dropped from state: {:?}",
                missing.len(),
                &missing[..missing.len().min(25)]
            ));
        }

        // 3b. PER-AXIS preservation. The union check above is masked whenever a
        // number dropped from one axis still appears in another, so every
        // cumulative axis is checked on its own.
        let ab = axes(&sb);
        let ac = axes(sc);
        for (k, vb) in &ab {
            match ac.get(k) {
                None => rep
                    .fails
                    .push(format!("3b: cumulative axis '{k}' disappeared from state")),
                Some(vc) => {
                    let lost: Vec<i64> = vb.difference(vc).copied().collect();
                    if !lost.is_empty() {
                        rep.fails.push(format!(
                            "3b: axis '{k}' lost {} issue(s): {:?}",
                            lost.len(),
                            &lost[..lost.len().min(20)]
                        ));
                    }
                }
            }
        }

        // 3c. A per-run axis being reset must have a `*_prior` counterpart that
        // absorbs its members, or the classification is silently discarded.
        for (_, newk, prik) in ROLL_FORWARD {
            let Some(old_new) = list_at(&sb, newk) else {
                continue;
            };
            if old_new.is_empty() {
                continue;
            }
            let new_prior = list_at(sc, prik).unwrap_or_default();
            let still_new = list_at(sc, newk).unwrap_or_default();
            let lost: Vec<i64> = old_new
                .iter()
                .filter(|n| !new_prior.contains(n) && !still_new.contains(n))
                .copied()
                .collect();
            if !lost.is_empty() {
                rep.fails.push(format!(
                    "3c: '{newk}' reset without rolling {:?} into '{prik}'",
                    &lost[..lost.len().min(20)]
                ));
            }
        }
    }

    // ---- 4. classification completeness ----------------------------------
    if candidate.contains("_unclassified_") {
        rep.fails
            .push("4: '_unclassified_' present in candidate".into());
    }

    // ---- 5. markdown render integrity (REGRESSION-relative) --------------
    // The live body already carries violations of 5d. A gate that fails on
    // inherited debt blocks every future post, so these compare candidate vs
    // before: a NEW violation fails; a pre-existing one warns.
    let vb = render_violations(before);
    let vc = render_violations(candidate);
    let mut budget: Vec<(&str, &str)> = vb.iter().map(|v| (v.code, v.what)).collect();
    for v in &vc {
        match budget.iter().position(|k| *k == (v.code, v.what)) {
            Some(i) => {
                budget.remove(i);
            }
            None => rep.fails.push(format!(
                "{}: NEW render-integrity violation — {} :: {}",
                v.code, v.what, v.ctx
            )),
        }
    }
    if !vb.is_empty() {
        let kinds: BTreeSet<String> = vb
            .iter()
            .map(|v| format!("{} {}", v.code, v.what))
            .collect();
        rep.warns.push(format!(
            "{} pre-existing render-integrity violation(s) inherited from the \
             before-snapshot (not introduced here): {}",
            vb.len(),
            kinds.into_iter().collect::<Vec<_>>().join("; ")
        ));
    }

    // 5c. Every attn issue in the sentinel is a VISIBLE row in the lane table,
    // ahead of that section's <details>, never only inside the agent channel.
    if let Some(sc) = &sc {
        if let Some(Value::Object(attn)) = sc.get("attn") {
            let mut attn_nums: BTreeSet<i64> = BTreeSet::new();
            for v in attn.values() {
                if let Value::Array(list) = v {
                    attn_nums.extend(ints_in(list));
                }
            }
            match section(candidate, "## 👤 Needs human reading") {
                None => rep.fails.push("5c: attn section not found".into()),
                Some(sec) => {
                    let visible = sec.split("<details>").next().unwrap_or(&sec).to_string();
                    for n in &attn_nums {
                        if !mentions_issue(&visible, *n) {
                            rep.fails.push(format!(
                                "5c: attn issue #{n} in sentinel but not a visible row in the lane table"
                            ));
                        }
                    }
                }
            }
        }
    }

    Ok(rep)
}

// ------------------------------------------------------------- text utils ---

/// Every `##`/`###` heading line, in order. Matches `^#{2,3} .*$`: exactly two
/// or three hashes followed by a space, at the start of a line.
fn headings(text: &str) -> Vec<&str> {
    text.lines()
        .filter(|l| l.starts_with("## ") || l.starts_with("### "))
        .collect()
}

/// Run numbers carrying a `### Run-<n> index` heading.
fn run_indexes(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in text.lines() {
        let Some(rest) = line.strip_prefix("### Run-") else {
            continue;
        };
        let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
        if !digits.is_empty() && rest[digits.len()..].starts_with(" index") {
            out.push(digits);
        }
    }
    out
}

/// The slice from a `## ` heading up to the next `## ` heading (or the end).
fn section(text: &str, heading_prefix: &str) -> Option<String> {
    let mut lines = text.lines();
    let mut out: Vec<&str> = Vec::new();
    for line in lines.by_ref() {
        if line.starts_with(heading_prefix) {
            out.push(line);
            break;
        }
    }
    if out.is_empty() {
        return None;
    }
    for line in lines {
        if line.starts_with("## ") {
            break;
        }
        out.push(line);
    }
    Some(out.join("\n"))
}

/// `#<n>` present with a word boundary after it — `#31` must not match `#312`.
fn mentions_issue(text: &str, n: i64) -> bool {
    let needle = format!("#{n}");
    let mut pos = 0usize;
    while let Some(rel) = text[pos..].find(&needle) {
        let at = pos + rel;
        let after = &text[at + needle.len()..];
        match after.chars().next() {
            None => return true,
            Some(c) if !(c.is_alphanumeric() || c == '_') => return true,
            _ => pos = at + needle.len(),
        }
    }
    false
}

fn clip(s: &str, n: usize) -> String {
    s.chars().take(n).collect()
}

fn show(v: Option<i64>) -> String {
    match v {
        Some(n) => n.to_string(),
        None => "None".into(),
    }
}

fn commas(n: usize) -> String {
    let digits = n.to_string();
    let mut out = String::new();
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    out
}

// ------------------------------------------------------------------ entry ---

pub fn run(
    root: &Path,
    target: &str,
    before_path: &Path,
    candidate_path: &Path,
    renames_path: Option<&Path>,
) -> Result<()> {
    let before = fs::read_to_string(before_path)
        .with_context(|| format!("read before-snapshot {}", before_path.display()))?;
    let candidate = fs::read_to_string(candidate_path)
        .with_context(|| format!("read candidate {}", candidate_path.display()))?;
    let renames = load_renames(root, target, renames_path)?;

    let report = evaluate(&before, &candidate, &renames)?;
    print!("{}", report.render());
    if !report.passed() {
        bail!(
            "gate failed — {} check(s); do NOT post this body",
            report.fails.len()
        );
    }
    Ok(())
}

// ------------------------------------------------------------------ tests ---

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    /// A minimal dashboard body carrying `state` as its sentinel: every
    /// required section, a bolded verdict, and one `### Run-N index` heading
    /// per entry in `indexes`.
    fn doc(state: Value, indexes: &[i64], extra: &str) -> String {
        let mut s = String::from("<!-- beadle-state:v1\n");
        s.push_str(&serde_json::to_string(&state).expect("serialize sentinel"));
        s.push_str("\nbeadle-state -->\n\n**Direction verdict:** steady.\n\n");
        s.push_str(
            "## Baseline\n\n\
             ## 👤 Needs human reading\n\n\
             | # | note |\n| --- | --- |\n| #410 | see thread |\n| #671 | see thread |\n\n\
             ## Action plan\n\n\
             ### 🔴 P0a\n\n### 🔴 P0b\n\n### 🟠 P1\n\n### 🟢 P2\n\n\
             ## 🟦 Quick wins\n\n\
             ## Direction Health\n\n\
             ## Classification index\n\n",
        );
        for r in indexes {
            s.push_str(&format!("### Run-{r} index\n\n"));
        }
        s.push_str("## Maintainer progress\n\n## Controls\n\n");
        s.push_str(extra);
        s
    }

    fn base_state(run: i64, watermark: i64) -> Value {
        json!({
            "run": run,
            "watermark": watermark,
            "attn": { "direction": [410, 671] },
            "clusters": { "scratch-isolation": [508, 624, 830] },
            "quick_wins_new": [],
            "quick_wins_prior": [],
            "operational_impact": { "degraded_new": [], "degraded_prior": [] },
        })
    }

    fn gate(before: &str, cand: &str) -> Report {
        evaluate(before, cand, &[]).expect("gate runs")
    }

    #[test]
    fn identical_bodies_pass_when_run_and_watermark_advance() {
        let before = doc(base_state(18, 830), &[18], "");
        let cand = doc(base_state(19, 836), &[18, 19], "");
        let r = gate(&before, &cand);
        assert!(r.passed(), "unexpected fails: {:?}", r.fails);
    }

    #[test]
    fn sentinel_survives_the_9kb_single_line_shape() {
        let filler: Vec<i64> = (1..2000).collect();
        let mut st = base_state(19, 836);
        st["clusters"]["filler"] = json!(filler);
        let body = doc(st, &[19], "");
        let parsed = sentinel(&body).expect("sentinel parses");
        assert_eq!(parsed.get("run").and_then(as_int), Some(19));
    }

    #[test]
    fn missing_candidate_sentinel_fails_rather_than_panicking() {
        let before = doc(base_state(18, 830), &[18], "");
        let r = gate(&before, "## Baseline\n");
        assert!(r
            .fails
            .iter()
            .any(|f| f.contains("candidate sentinel missing or unparseable")));
    }

    #[test]
    fn dict_int_values_are_counters_not_issue_numbers() {
        // `a4_windows` maps issue number -> window count. Only the KEYS are
        // issue numbers; reading the values would invent tracked issues and,
        // worse, make a counter change look like a dropped issue.
        let state = json!({ "a4_windows": { "523": 3, "588": 7, "358": "n/a" } })
            .as_object()
            .cloned()
            .expect("object");
        assert_eq!(nums(&state), BTreeSet::from([358, 523, 588]));

        let before = json!({ "a4_windows": { "523": 3 } })
            .as_object()
            .cloned()
            .expect("object");
        let after = json!({ "a4_windows": { "523": 9 } })
            .as_object()
            .cloned()
            .expect("object");
        assert_eq!(
            nums(&before),
            nums(&after),
            "a counter bump is not issue churn"
        );
    }

    #[test]
    fn per_axis_catches_a_loss_the_union_check_masks() {
        // 830 leaves scratch-isolation and reappears under observability, so
        // the union of tracked numbers is unchanged. This is the run-19 bug.
        let before = doc(base_state(18, 830), &[18], "");
        let mut st = base_state(19, 836);
        st["clusters"] = json!({ "scratch-isolation": [508, 624], "observability": [830] });
        let cand = doc(st, &[18, 19], "");

        let r = gate(&before, &cand);
        assert!(
            !r.fails.iter().any(|f| f.starts_with("3:")),
            "union check should be blind here, but fired: {:?}",
            r.fails
        );
        assert!(
            r.fails
                .iter()
                .any(|f| f.contains("3b: axis 'clusters.scratch-isolation' lost 1 issue(s): [830]")),
            "per-axis check missed it: {:?}",
            r.fails
        );
    }

    #[test]
    fn a_cumulative_axis_may_not_vanish() {
        let before = doc(base_state(18, 830), &[18], "");
        let mut st = base_state(19, 836);
        st["clusters"] = json!({});
        let cand = doc(st, &[18, 19], "");
        assert!(gate(&before, &cand)
            .fails
            .iter()
            .any(|f| f.contains("cumulative axis 'clusters.scratch-isolation' disappeared")));
    }

    #[test]
    fn per_run_axis_may_reset_when_members_roll_into_prior() {
        let mut b = base_state(18, 830);
        b["operational_impact"] = json!({ "degraded_new": [762, 788], "degraded_prior": [] });
        let before = doc(b, &[18], "");

        let mut c = base_state(19, 836);
        c["operational_impact"] = json!({ "degraded_new": [901], "degraded_prior": [762, 788] });
        let cand = doc(c, &[18, 19], "");

        let r = gate(&before, &cand);
        assert!(r.passed(), "roll-forward should pass: {:?}", r.fails);
    }

    #[test]
    fn per_run_axis_reset_without_roll_forward_fails() {
        let mut b = base_state(18, 830);
        b["operational_impact"] = json!({ "degraded_new": [762, 788], "degraded_prior": [] });
        let before = doc(b, &[18], "");

        let mut c = base_state(19, 836);
        c["operational_impact"] = json!({ "degraded_new": [], "degraded_prior": [] });
        let cand = doc(c, &[18, 19], "");

        assert!(gate(&before, &cand).fails.iter().any(|f| f.contains(
            "3c: 'operational_impact.degraded_new' reset without rolling [762, 788] into \
             'operational_impact.degraded_prior'"
        )));
    }

    #[test]
    fn inherited_render_violation_warns_and_a_new_one_fails() {
        let debt = "<details>\n<summary>rows</summary>\n\n| a | b |\n| - | - |\n\n</details>\n";
        let before = doc(base_state(18, 830), &[18], debt);
        let cand = doc(base_state(19, 836), &[18, 19], debt);

        let r = gate(&before, &cand);
        assert!(r.passed(), "inherited debt must not fail: {:?}", r.fails);
        assert!(
            r.warns
                .iter()
                .any(|w| w.contains("pre-existing render-integrity")),
            "inherited debt must warn: {:?}",
            r.warns
        );

        // One extra row on top of the inherited two is a regression.
        let worse =
            format!("{debt}\n<details>\n<summary>more</summary>\n\n| c | d |\n\n</details>\n");
        let r2 = gate(&before, &doc(base_state(19, 836), &[18, 19], &worse));
        assert!(r2.fails.iter().any(
            |f| f.contains("NEW render-integrity violation — pipe-table row inside <details>")
        ));
    }

    #[test]
    fn a_table_hugging_a_blockquote_is_a_new_violation() {
        let before = doc(base_state(18, 830), &[18], "");
        let cand = doc(base_state(19, 836), &[18, 19], "> quote\n| a | b |\n");
        assert!(gate(&before, &cand)
            .fails
            .iter()
            .any(|f| f.contains("table directly follows blockquote")));
    }

    #[test]
    fn heading_loss_fails_unless_the_rename_is_declared() {
        let before = doc(base_state(18, 830), &[18], "### Run-18 findings (NEW)\n");
        let cand = doc(
            base_state(19, 836),
            &[18, 19],
            "### Run-18 findings (carried)\n",
        );

        assert!(gate(&before, &cand)
            .fails
            .iter()
            .any(|f| f.contains("2: HEADING LOST -> ### Run-18 findings (NEW)")));

        let declared = [Rename {
            from: "### Run-18 findings (NEW)".into(),
            to: "### Run-18 findings (carried)".into(),
            why: "run-18 carry precedent".into(),
        }];
        let r = evaluate(&before, &cand, &declared).expect("gate runs");
        assert!(r.passed(), "declared rename should pass: {:?}", r.fails);
        assert!(r.warns.iter().any(|w| w.contains("declared rename")));
        assert!(r.warns.iter().any(|w| w.contains("run-18 carry precedent")));
    }

    #[test]
    fn attn_issue_must_be_visible_before_the_details_fold() {
        let before = doc(base_state(18, 830), &[18], "");
        let mut st = base_state(19, 836);
        st["attn"] = json!({ "direction": [410, 671, 999] });
        let cand = doc(st, &[18, 19], "");
        assert!(gate(&before, &cand).fails.iter().any(|f| f
            .contains("5c: attn issue #999 in sentinel but not a visible row in the lane table")));
    }

    #[test]
    fn issue_mentions_respect_word_boundaries() {
        assert!(mentions_issue("row for #410 here", 410));
        assert!(mentions_issue("ends at #410", 410));
        assert!(!mentions_issue("this is #4108", 410));
        assert!(mentions_issue("#4108 and #410.", 410));
    }

    #[test]
    fn run_index_headings_are_extracted_by_number() {
        let t = "### Run-18 index (carried forward — folded)\n### Run-19 index (NEW)\n### Run-20 notes\n";
        assert_eq!(run_indexes(t), vec!["18".to_string(), "19".to_string()]);
    }

    #[test]
    fn code_fences_inside_details_are_not_tables() {
        let blk = "<details>\n<summary>s</summary>\n\n```\n| not | a | table |\n```\n\n</details>";
        assert!(render_violations(blk).iter().all(|v| v.code != "5d"));
    }
}
