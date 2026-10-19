//! Minimal `targets/<name>.intent.yaml` reader.
//!
//! We need a handful of top-level scalars, two YAML sequences, and, for
//! instrument v2 (SKILL §6b), the `maintainer_capacity:` block with its
//! nested `observed:` sub-block. Rather than pull a full YAML parser, we grep
//! for `key: value`, for sequence-under-key blocks, and walk indent-delimited
//! sub-blocks. When the manifest structure grows past this shape, swap in
//! `serde_yaml`.

use std::{fs, path::Path};

use anyhow::{Context, Result, anyhow};

pub struct Intent {
    pub repo: String,
    #[allow(dead_code)] // wired into the run record once render() lands
    pub version: Option<String>,
    pub maintainers: Vec<String>,
    pub measured_contributors: Vec<String>,
    /// `maintainer_capacity:`: present from manifest schema v0.4. Its
    /// presence is what switches a direction verdict from instrument v1 to
    /// v2; absent, the crate computes v1 and says so out loud (SKILL §6b).
    pub maintainer_capacity: Option<MaintainerCapacity>,
}

/// The capacity model a v2 verdict is conditioned on (SKILL §6b).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MaintainerCapacity {
    pub class: String,
    pub team_size: Option<u32>,
    /// The unit an attention window is measured in: `action-day` today.
    pub attention_window: String,
    /// Window dates the run observed, including maintainer actions the store
    /// never sees (merges and commits on their own PRs). Refreshed each run.
    pub observed_windows: Vec<String>,
    /// `observed.pr_acceptance`: the store holds no PR records, so the
    /// acceptance band has no store-side input.
    pub pr_acceptance: Option<PrAcceptance>,
    /// `observed.measured_prs`: measured-side PRs with their readiness
    /// dates. Feeds the merge-latency band and §6b rule 4(b).
    pub measured_prs: Vec<MeasuredPr>,
    /// `observed.acted_with_diff`: §6b rule 3's cost covariate: the share of
    /// acted-on items that arrived with a mergeable diff.
    pub acted_with_diff: Option<ActedWithDiff>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrAcceptance {
    pub merged: u32,
    pub decided: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeasuredPr {
    pub number: u32,
    /// `YYYY-MM-DD` the PR became ready for a maintainer decision.
    pub ready: String,
    /// `YYYY-MM-DD` it merged, when it has.
    pub merged: Option<String>,
    /// Issue numbers this PR actually repairs: what §6b rule 4(b) tests
    /// against the silent lane.
    pub fixes: Vec<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActedWithDiff {
    pub with_diff: u32,
    pub acted: u32,
}

impl Intent {
    /// Classify an author login: "maintainer", "measured", or "other".
    /// This is the axis the delta-sweep needs: everything else is analysis.
    pub fn actor_role(&self, login: &str) -> &'static str {
        if self.maintainers.iter().any(|m| m == login) {
            "maintainer"
        } else if self.measured_contributors.iter().any(|c| c == login) {
            "measured"
        } else {
            "other"
        }
    }
}

pub fn load(root: &Path, target: &str) -> Result<Intent> {
    let path = root.join(format!("targets/{target}.intent.yaml"));
    let text = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
    // This reader is not a YAML parser. A line it would read one way and a real
    // parser another is refused here, so a manifest edit cannot change meaning
    // silently (the loader fails closed).
    if let Some((n, what)) = colon_space_hazards(&text).into_iter().next() {
        return Err(anyhow!("{}:{n}: {what}", path.display()));
    }
    parse(&text).ok_or_else(|| anyhow!("no `repo:` scalar in {}", path.display()))
}

/// Lines this reader accepts but a real YAML parser would read differently.
/// Returns (line, reason). Each is a way a manifest edit could change meaning
/// without this reader noticing:
///
/// - an unquoted scalar holding a colon-space (a map key, to a parser);
/// - text after the closing quote of a quoted scalar (a parse error);
/// - a quoted scalar left open at the end of the text;
/// - a list item that reads as a one-key map among plain items. `- path: x`
///   in a list of such items is a map list and is fine; one such item among
///   prose items is what a colon-space edit of a prose item produces.
///
/// Comments, quoted scalars (also across lines), flow collections and block
/// scalars are skipped otherwise.
pub(crate) fn colon_space_hazards(text: &str) -> Vec<(usize, String)> {
    struct Block {
        indent: usize,
        maps: Vec<usize>,
        plain: usize,
    }
    fn flush(b: Block, out: &mut Vec<(usize, String)>) {
        if b.plain > 0 {
            for n in b.maps {
                out.push((
                    n,
                    "list item reads as a map among plain items; \
                     a colon-space in prose makes a key"
                        .to_string(),
                ));
            }
        }
    }
    // Index just past the closing quote of `s`, which starts with quote `q`.
    fn close_at(s: &str, q: char) -> Option<usize> {
        let mut it = s.char_indices().skip(1).peekable();
        while let Some((i, c)) = it.next() {
            if q == '"' && c == '\\' {
                it.next();
            } else if c == q {
                if q == '\'' && matches!(it.peek(), Some((_, '\''))) {
                    it.next();
                } else {
                    return Some(i + c.len_utf8());
                }
            }
        }
        None
    }
    // Open minus close brackets on a line, outside quotes.
    fn bracket_delta(s: &str) -> i32 {
        let (mut d, mut q) = (0, None);
        for c in s.split(" #").next().unwrap_or(s).chars() {
            match (q, c) {
                (None, '"' | '\'') => q = Some(c),
                (Some(x), c) if c == x => q = None,
                (None, '[' | '{') => d += 1,
                (None, ']' | '}') => d -= 1,
                _ => {}
            }
        }
        d
    }
    let strip_comment = |s: &str| -> String {
        s.split(" #").next().unwrap_or(s).trim().to_string()
    };

    let mut out = Vec::new();
    let mut blocks: Vec<Block> = Vec::new();
    let mut open_quote: Option<(usize, char)> = None;
    let mut block_indent: Option<usize> = None;
    let mut flow_depth: i32 = 0;
    let bare_key = |k: &str| {
        let mut c = k.chars();
        matches!(c.next(), Some(f) if f.is_ascii_alphabetic() || f == '_')
            && k.chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.'))
    };
    for (i, line) in text.lines().enumerate() {
        let n = i + 1;
        let indent = line.len() - line.trim_start().len();
        if let Some((_, q)) = open_quote {
            if let Some(at) = line.find(q) {
                open_quote = None;
                let rest = strip_comment(&line[at + q.len_utf8()..]);
                if !rest.is_empty() {
                    out.push((n, format!("text after a closing quote: {rest}")));
                }
            }
            continue;
        }
        if flow_depth > 0 {
            flow_depth += bracket_delta(line);
            continue;
        }
        if let Some(b) = block_indent {
            if line.trim().is_empty() || indent > b {
                continue;
            }
            block_indent = None;
        }
        let t = line.trim_start();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let item = t.strip_prefix("- ");
        // Close the lists this line ends.
        while let Some(b) = blocks.last() {
            let ends = if item.is_some() {
                b.indent > indent
            } else {
                b.indent >= indent
            };
            if !ends {
                break;
            }
            if let Some(b) = blocks.pop() {
                flush(b, &mut out);
            }
        }
        if item.is_some() && blocks.last().is_none_or(|b| b.indent != indent) {
            blocks.push(Block {
                indent,
                maps: Vec::new(),
                plain: 0,
            });
        }

        let body = item.unwrap_or(t);
        // The scalar to judge, and whether this item reads as a map.
        let mut map_item = false;
        let scalar: &str = match body.split_once(": ") {
            Some((k, v)) if bare_key(k.trim()) => {
                map_item = item.is_some();
                v
            }
            Some((k, _)) if item.is_some() && matches!(k.chars().next(), Some('"' | '\'')) => {
                body
            }
            Some(_) if item.is_some() => body,
            Some((_, v)) => v,
            None => body.trim_end_matches(':'),
        };
        let scalar = strip_comment(scalar);
        let scalar = scalar.as_str();
        if scalar.starts_with(['[', '{']) {
            flow_depth = bracket_delta(scalar).max(0);
        }
        if scalar.starts_with('|') || scalar.starts_with('>') {
            block_indent = Some(indent);
            continue;
        }
        if let Some(q) = scalar.chars().next().filter(|c| matches!(c, '"' | '\'')) {
            match close_at(scalar, q) {
                None => open_quote = Some((n, q)),
                Some(end) => {
                    let rest = scalar[end..].trim();
                    if rest.starts_with(':') && item.is_some() && !map_item {
                        // `- "key": value` is a map item.
                        map_item = true;
                    } else if !rest.is_empty() {
                        out.push((n, format!("text after a closing quote: {rest}")));
                    }
                }
            }
        } else if !(scalar.starts_with('[') || scalar.starts_with('{'))
            && scalar.contains(": ")
            && !map_item
        {
            out.push((n, format!("unquoted value holds a colon-space: {scalar}")));
        } else if map_item && !(scalar.starts_with('[') || scalar.starts_with('{'))
            && !scalar.starts_with(['"', '\''])
            && scalar.contains(": ")
        {
            out.push((n, format!("unquoted value holds a colon-space: {scalar}")));
        }
        if item.is_some() {
            if let Some(b) = blocks.last_mut() {
                if map_item {
                    b.maps.push(n);
                } else {
                    b.plain += 1;
                }
            }
        }
    }
    while let Some(b) = blocks.pop() {
        flush(b, &mut out);
    }
    if let Some((n, _)) = open_quote {
        out.push((n, "quoted value is never closed".to_string()));
    }
    out.sort();
    out
}

/// Parse manifest text. Separated from `load` so the capacity block is
/// testable without a file on disk.
pub fn parse(text: &str) -> Option<Intent> {
    Some(Intent {
        repo: scalar(text, "repo")?,
        version: scalar(text, "schema_version"),
        maintainers: sequence(text, "maintainers"),
        measured_contributors: sequence(text, "measured_contributors"),
        maintainer_capacity: parse_capacity(text),
    })
}

/// Parse the `maintainer_capacity:` block, including its nested `observed:`
/// sub-block (manifest v0.6). `None` means the target declares no capacity
/// model, which is the signal to score it under v1.
fn parse_capacity(text: &str) -> Option<MaintainerCapacity> {
    let cap = block(text, "maintainer_capacity")?;
    let observed = block(&cap, "observed").unwrap_or_default();

    Some(MaintainerCapacity {
        class: scalar(&cap, "class").unwrap_or_else(|| "unknown".to_string()),
        team_size: scalar(&cap, "team_size").and_then(|s| s.parse().ok()),
        attention_window: scalar(&cap, "attention_window")
            .unwrap_or_else(|| "action-day".to_string()),
        observed_windows: list(&cap, "observed_windows"),
        pr_acceptance: parse_pr_acceptance(&observed),
        measured_prs: parse_measured_prs(&observed),
        acted_with_diff: parse_acted_with_diff(&observed),
    })
}

fn parse_pr_acceptance(observed: &str) -> Option<PrAcceptance> {
    let b = block(observed, "pr_acceptance")?;
    Some(PrAcceptance {
        merged: scalar(&b, "merged").and_then(|s| s.parse().ok())?,
        decided: scalar(&b, "decided").and_then(|s| s.parse().ok())?,
    })
}

fn parse_acted_with_diff(observed: &str) -> Option<ActedWithDiff> {
    let b = block(observed, "acted_with_diff")?;
    Some(ActedWithDiff {
        with_diff: scalar(&b, "with_diff").and_then(|s| s.parse().ok())?,
        acted: scalar(&b, "acted").and_then(|s| s.parse().ok())?,
    })
}

fn parse_measured_prs(observed: &str) -> Vec<MeasuredPr> {
    let Some(b) = block(observed, "measured_prs") else {
        return Vec::new();
    };
    dash_items(&b)
        .iter()
        .filter_map(|item| {
            Some(MeasuredPr {
                number: scalar(item, "number").and_then(|s| s.parse().ok())?,
                ready: scalar(item, "ready")?,
                merged: scalar(item, "merged"),
                fixes: list(item, "fixes")
                    .iter()
                    .filter_map(|s| s.parse().ok())
                    .collect(),
            })
        })
        .collect()
}

fn indent_of(line: &str) -> usize {
    line.len() - line.trim_start().len()
}

/// Strip a trailing `# comment`, surrounding quotes, and whitespace.
fn clean(raw: &str) -> String {
    raw.split('#')
        .next()
        .unwrap_or("")
        .trim()
        .trim_matches(|c: char| c == '"' || c == '\'')
        .trim()
        .to_string()
}

/// Grep for a `key: value` line anywhere (any indentation).
/// Comments after `#` are stripped.
fn scalar(text: &str, key: &str) -> Option<String> {
    let needle = format!("{key}:");
    for line in text.lines() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix(&needle) {
            let v = clean(rest);
            if !v.is_empty() {
                return Some(v);
            }
        }
    }
    None
}

/// Return the indented body under a bare `key:` header line; every following
/// line indented deeper than the header, up to the first that is not. Blank
/// lines pass through; relative indentation is preserved so the result can be
/// fed back into `block`, `scalar` and `list`.
fn block(text: &str, key: &str) -> Option<String> {
    let needle = format!("{key}:");
    let mut base: Option<usize> = None;
    let mut out: Vec<&str> = Vec::new();

    for line in text.lines() {
        match base {
            None => {
                let trimmed = line.trim_start();
                if let Some(rest) = trimmed.strip_prefix(&needle) {
                    // A header, not a `key: value` scalar.
                    if clean(rest).is_empty() {
                        base = Some(indent_of(line));
                    }
                }
            }
            Some(base_indent) => {
                // Blank lines pass through; a line indented no deeper than
                // the header has left the block.
                if line.trim().is_empty() || indent_of(line) > base_indent {
                    out.push(line);
                } else {
                    break;
                }
            }
        }
    }
    base?;
    Some(out.join("\n"))
}

/// Read a list under `key` in either the flow form (`key: ["a", "b"]`,
/// possibly wrapped across lines) or the block form (`key:` + `- a` lines).
fn list(text: &str, key: &str) -> Vec<String> {
    let needle = format!("{key}:");
    let lines: Vec<&str> = text.lines().collect();
    let Some(start) = lines
        .iter()
        .position(|l| l.trim_start().starts_with(&needle))
    else {
        return Vec::new();
    };
    let head_indent = indent_of(lines[start]);
    let head_rest = lines[start].trim_start()[needle.len()..].trim().to_string();

    if head_rest.starts_with('[') {
        let mut buf = head_rest;
        let mut i = start + 1;
        while !buf.contains(']') && i < lines.len() {
            buf.push(' ');
            buf.push_str(lines[i].trim());
            i += 1;
        }
        let inner = buf
            .trim_start_matches('[')
            .split(']')
            .next()
            .unwrap_or("")
            .to_string();
        return inner
            .split(',')
            .map(clean)
            .filter(|s| !s.is_empty())
            .collect();
    }

    let mut out = Vec::new();
    for line in &lines[start + 1..] {
        if line.trim().is_empty() {
            continue;
        }
        if indent_of(line) <= head_indent {
            break;
        }
        let trimmed = line.trim();
        if let Some(item) = trimmed.strip_prefix("- ") {
            let v = clean(item);
            if !v.is_empty() {
                out.push(v);
            }
        } else if !trimmed.starts_with('#') {
            break;
        }
    }
    out
}

/// Split a block of `- ` sequence entries into one chunk per entry, with the
/// leading dash rewritten to whitespace so the chunk parses as a plain map.
fn dash_items(text: &str) -> Vec<String> {
    let mut items: Vec<String> = Vec::new();
    let mut cur: Option<Vec<String>> = None;
    let mut item_indent = 0usize;

    for line in text.lines() {
        if let Some(rest) = line.trim_start().strip_prefix("- ") {
            if let Some(done) = cur.take() {
                items.push(done.join("\n"));
            }
            item_indent = indent_of(line);
            cur = Some(vec![format!("{}{}", " ".repeat(item_indent + 2), rest)]);
        } else if let Some(open) = cur.as_mut() {
            if line.trim().is_empty() {
                continue;
            }
            if indent_of(line) > item_indent {
                open.push(line.to_string());
            } else {
                break;
            }
        }
    }
    if let Some(done) = cur {
        items.push(done.join("\n"));
    }
    items
}

/// Parse a top-level sequence: `key:\n  - item1\n  - item2`.
/// Stops at the first non-blank line that isn't a `- ` sequence entry.
fn sequence(text: &str, key: &str) -> Vec<String> {
    let needle = format!("{key}:");
    let mut lines = text.lines();
    let mut items = Vec::new();

    // Advance to the key.
    for line in lines.by_ref() {
        if line.trim_start().starts_with(&needle) && line.trim().ends_with(':') {
            break;
        }
    }

    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(item) = trimmed.strip_prefix("- ") {
            let v = clean(item);
            if !v.is_empty() {
                items.push(v);
            }
        } else {
            // Left the sequence block.
            break;
        }
    }
    items
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intent_files_parse_after_retirement() {
        // Every shipped manifest loads, and no unquoted value gained a `: `
        // (a colon-space would end the scalar). A regression pin: the files
        // parse on main too, and this guards the dash replacement step.
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let mut seen = 0;
        for e in std::fs::read_dir(root.join("targets")).unwrap().flatten() {
            let name = e.file_name().to_string_lossy().into_owned();
            let Some(target) = name.strip_suffix(".intent.yaml") else {
                continue;
            };
            let text = std::fs::read_to_string(e.path()).unwrap();
            seen += 1;
            if !target.starts_with('_') {
                let i = load(&root, target).unwrap_or_else(|err| panic!("{name}: {err}"));
                assert!(!i.repo.is_empty(), "{name}: empty repo");
            }
            assert_eq!(
                colon_space_hazards(&text),
                vec![],
                "{name}: unquoted value holds a colon-space"
            );
        }
        assert!(seen >= 3, "expected the intent files, saw {seen}");
    }

    #[test]
    fn colon_space_in_a_list_item_is_a_hazard() {
        // A parser reads `- build output (re-run: cheapest)` as a one-key map.
        let yaml = "tier:\n  - generated code / build output (re-run: cheapest)\n";
        let h = colon_space_hazards(yaml);
        assert_eq!(h.len(), 1, "{h:?}");
        assert_eq!(h[0].0, 2);
        // Controls: a semicolon is fine, and so are one-key map items, quoted
        // items and comments.
        for ok in [
            "tier:\n  - generated code (re-run; cheapest)\n",
            "tier:\n  - path: README.md\n  - source: git-history\n",
            "tier:\n  - \"quoted (re-run: cheapest)\"\n",
            "tier:\n  - fine # a comment: with a colon\n",
            "# a comment: with a colon\nkey: value\n",
        ] {
            assert_eq!(colon_space_hazards(ok), vec![], "{ok:?}");
        }
    }

    #[test]
    fn colon_space_in_a_value_is_a_hazard() {
        let h = colon_space_hazards("repo: acme/widget\nnote: canonical principle: no defer\n");
        assert_eq!(h.len(), 1, "{h:?}");
        assert_eq!(h[0].0, 2);
        assert_eq!(
            colon_space_hazards("note: canonical principle; no defer\n"),
            vec![]
        );
    }

    #[test]
    fn a_quoted_value_may_span_lines_but_must_close() {
        let ok = "x:\n  - \"first line (e.g. a: b)\n    second line: still quoted\"\n";
        assert_eq!(colon_space_hazards(ok), vec![]);
        let open = "x:\n  - \"never closed\n";
        let h = colon_space_hazards(open);
        assert_eq!(h.len(), 1, "{h:?}");
        assert!(h[0].1.contains("never closed"));
    }

    #[test]
    fn load_fails_closed_on_a_line_it_would_misread() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("targets")).unwrap();
        let p = dir.path().join("targets/t.intent.yaml");
        std::fs::write(&p, "repo: acme/widget\ntier:\n  - a thing (see: this)\n").unwrap();
        let err = match load(dir.path(), "t") {
            Err(e) => e.to_string(),
            Ok(_) => panic!("a misread line must not load"),
        };
        assert!(err.contains(":3:") && err.contains("colon-space"), "{err}");
        // Control: the same manifest with a semicolon loads.
        std::fs::write(&p, "repo: acme/widget\ntier:\n  - a thing (see; this)\n").unwrap();
        assert_eq!(
            load(dir.path(), "t").ok().map(|i| i.repo),
            Some("acme/widget".to_string())
        );
    }

    #[test]
    fn a_bare_word_lead_among_plain_items_is_a_hazard() {
        // The shape a colon-space retirement of a prose item produces: a parser
        // reads `- Paper-fixes: rename` as a map among prose items.
        let yaml = "non_goals:\n  - MVP deferrals\n  - Paper-fixes: rename or doc-comment only\n  - Scope expansion\n";
        let h = colon_space_hazards(yaml);
        assert_eq!(h.len(), 1, "{h:?}");
        assert_eq!(h[0].0, 3);
        assert!(h[0].1.contains("reads as a map"), "{h:?}");
        // A lowercase single word is the same shape.
        assert_eq!(
            colon_space_hazards("g:\n  - one\n  - fixes: two\n").len(),
            1
        );
        // Controls: a list that is all map items is a map list, a lone map
        // item is too, and the semicolon form is plain.
        for ok in [
            "t:\n  - path: README.md\n  - glob: docs/**\n  - source: git-history\n",
            "t:\n  - source: git-history\n",
            "non_goals:\n  - MVP deferrals\n  - Paper-fixes; rename or doc-comment only\n",
            "a:\n  - x\nb:\n  - k: v\n",
        ] {
            assert_eq!(colon_space_hazards(ok), vec![], "{ok:?}");
        }
    }

    #[test]
    fn text_after_a_closing_quote_is_a_hazard() {
        // A parser rejects all of these.
        for bad in [
            "note: \"abc\": def\n",
            "note: \"abc\" def\n",
            "note: 'abc' def\n",
            "t:\n  - \"abc\" def\n",
            "t:\n  - \"first\n    second\" tail\n",
        ] {
            let h = colon_space_hazards(bad);
            assert_eq!(h.len(), 1, "{bad:?} -> {h:?}");
            assert!(h[0].1.contains("after a closing quote"), "{h:?}");
        }
        // Controls: a trailing comment, an escaped quote, a doubled single
        // quote and a closed multi-line value are fine.
        for ok in [
            "note: \"abc\" # a comment: here\n",
            "note: \"a \\\" b: c\"\n",
            "note: 'it''s: fine'\n",
            "t:\n  - \"first\n    second\"\n",
        ] {
            assert_eq!(colon_space_hazards(ok), vec![], "{ok:?}");
        }
    }

    #[test]
    fn parses_sequence() {
        let yaml = "maintainers:\n  - drbothen\n  - Zious11\nother_key: hello\n";
        assert_eq!(
            sequence(yaml, "maintainers"),
            vec!["drbothen".to_string(), "Zious11".to_string()]
        );
    }

    #[test]
    fn actor_role_classifies() {
        let i = Intent {
            repo: "x/y".into(),
            version: None,
            maintainers: vec!["drbothen".into()],
            measured_contributors: vec!["arcavenai".into()],
            maintainer_capacity: None,
        };
        assert_eq!(i.actor_role("drbothen"), "maintainer");
        assert_eq!(i.actor_role("arcavenai"), "measured");
        assert_eq!(i.actor_role("random"), "other");
    }

    const CAPACITY_YAML: &str = r#"schema_version: 0.6
target:
  repo: acme/widget
maintainers:
  - alice
measured_contributors:
  - bot
maintainer_capacity:
  class: episodic-side-project   # full-time | part-time-regular | episodic
  team_size: 2
  attention_window: action-day   # a calendar day with >=1 maintainer action
  # Derived from the store; refresh each run.
  observed_windows: ["2026-07-08", "2026-07-15",
    "2026-07-19"]
  success_bands:
    pr_acceptance:
      green: ">= 0.60"
  pr_channel:
    max_open_measured_prs: 10
  # run-scoped observations
  observed:
    as_of_run: 19
    pr_acceptance:
      merged: 30
      decided: 31
    measured_prs:
      - number: 729
        ready: "2026-08-01"      # re-review asked
        fixes: [515]
      - number: 768
        ready: "2026-08-04"
        merged: "2026-08-20"
        fixes: []

# a trailing top-level comment
goals:
  - ship it
"#;

    #[test]
    fn capacity_block_parses() {
        let cap = parse_capacity(CAPACITY_YAML).expect("capacity block present");
        assert_eq!(cap.class, "episodic-side-project");
        assert_eq!(cap.team_size, Some(2));
        assert_eq!(cap.attention_window, "action-day");
        assert_eq!(
            cap.observed_windows,
            vec!["2026-07-08", "2026-07-15", "2026-07-19"],
            "wrapped flow sequence reads across lines"
        );
    }

    #[test]
    fn observed_windows_is_not_confused_with_observed_block() {
        let cap = parse_capacity(CAPACITY_YAML).expect("capacity block present");
        assert_eq!(
            cap.pr_acceptance,
            Some(PrAcceptance {
                merged: 30,
                decided: 31
            })
        );
        assert_eq!(cap.observed_windows.len(), 3);
    }

    #[test]
    fn measured_prs_parse_with_fixes_and_merge_dates() {
        let cap = parse_capacity(CAPACITY_YAML).expect("capacity block present");
        assert_eq!(
            cap.measured_prs,
            vec![
                MeasuredPr {
                    number: 729,
                    ready: "2026-08-01".into(),
                    merged: None,
                    fixes: vec![515],
                },
                MeasuredPr {
                    number: 768,
                    ready: "2026-08-04".into(),
                    merged: Some("2026-08-20".into()),
                    fixes: vec![],
                },
            ]
        );
    }

    #[test]
    fn capacity_absent_yields_none() {
        let yaml = "schema_version: 0.1\nrepo: acme/widget\nmaintainers:\n  - alice\n";
        assert!(parse_capacity(yaml).is_none());
        let intent = parse(yaml).expect("parses");
        assert!(intent.maintainer_capacity.is_none());
    }

    #[test]
    fn success_bands_nested_scalar_does_not_leak_into_observed() {
        // `success_bands.pr_acceptance.green` must not be read as the
        // observed pr_acceptance: the observed block is scoped separately.
        let cap = parse_capacity(CAPACITY_YAML).expect("capacity block present");
        assert_eq!(cap.pr_acceptance.map(|p| p.decided), Some(31));
    }
}
