//! Minimal `targets/<name>.intent.yaml` reader.
//!
//! We need a handful of top-level scalars, two YAML sequences, and — for
//! instrument v2 (SKILL §6b) — the `maintainer_capacity:` block with its
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
    /// `maintainer_capacity:` — present from manifest schema v0.4. Its
    /// presence is what switches a direction verdict from instrument v1 to
    /// v2; absent, the crate computes v1 and says so out loud (SKILL §6b).
    pub maintainer_capacity: Option<MaintainerCapacity>,
}

/// The capacity model a v2 verdict is conditioned on (SKILL §6b).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MaintainerCapacity {
    pub class: String,
    pub team_size: Option<u32>,
    /// The unit an attention window is measured in — `action-day` today.
    pub attention_window: String,
    /// Window dates the run observed, including maintainer actions the store
    /// never sees (merges and commits on their own PRs). Refreshed each run.
    pub observed_windows: Vec<String>,
    /// `observed.pr_acceptance` — the store holds no PR records, so the
    /// acceptance band has no store-side input.
    pub pr_acceptance: Option<PrAcceptance>,
    /// `observed.measured_prs` — measured-side PRs with their readiness
    /// dates. Feeds the merge-latency band and §6b rule 4(b).
    pub measured_prs: Vec<MeasuredPr>,
    /// `observed.acted_with_diff` — §6b rule 3's cost covariate: the share of
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
    /// Issue numbers this PR actually repairs — what §6b rule 4(b) tests
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
    /// This is the axis the delta-sweep needs — everything else is analysis.
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
    parse(&text).ok_or_else(|| anyhow!("no `repo:` scalar in {}", path.display()))
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

/// Return the indented body under a bare `key:` header line — every following
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
        // observed pr_acceptance — the observed block is scoped separately.
        let cap = parse_capacity(CAPACITY_YAML).expect("capacity block present");
        assert_eq!(cap.pr_acceptance.map(|p| p.decided), Some(31));
    }
}
