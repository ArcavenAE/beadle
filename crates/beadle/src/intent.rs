//! Minimal `targets/<name>.intent.yaml` reader.
//!
//! We only need a handful of top-level scalars and two YAML sequences for
//! Phase 2. Rather than pull a full YAML parser, we grep for `key: value`
//! and for sequence-under-key blocks — matching what the Perl seed does.
//! When the manifest structure grows past this shape, swap in `serde_yaml`.

use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::Path;

pub struct Intent {
    pub repo: String,
    #[allow(dead_code)] // wired into the run record once render() lands
    pub version: Option<String>,
    pub maintainers: Vec<String>,
    pub measured_contributors: Vec<String>,
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
    let text = fs::read_to_string(&path)
        .with_context(|| format!("read {}", path.display()))?;

    let repo = scalar(&text, "repo")
        .ok_or_else(|| anyhow!("no `repo:` scalar in {}", path.display()))?;
    let version = scalar(&text, "schema_version");
    let maintainers = sequence(&text, "maintainers");
    let measured_contributors = sequence(&text, "measured_contributors");

    Ok(Intent {
        repo,
        version,
        maintainers,
        measured_contributors,
    })
}

/// Grep for a `key: value` line anywhere (any indentation).
/// Comments after `#` are stripped.
fn scalar(text: &str, key: &str) -> Option<String> {
    let needle = format!("{key}:");
    for line in text.lines() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix(&needle) {
            let v = rest
                .split('#')
                .next()
                .unwrap_or("")
                .trim()
                .trim_matches(|c: char| c == '"' || c == '\'');
            if !v.is_empty() {
                return Some(v.to_string());
            }
        }
    }
    None
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
            let v = item
                .split('#')
                .next()
                .unwrap_or("")
                .trim()
                .trim_matches(|c: char| c == '"' || c == '\'');
            if !v.is_empty() {
                items.push(v.to_string());
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
        };
        assert_eq!(i.actor_role("drbothen"), "maintainer");
        assert_eq!(i.actor_role("arcavenai"), "measured");
        assert_eq!(i.actor_role("random"), "other");
    }
}
