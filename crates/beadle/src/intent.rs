//! Minimal `targets/<name>.intent.yaml` reader.
//!
//! We only need a handful of top-level scalars for Phase 2 (repo, version).
//! Rather than pull a full YAML parser, we grep for `key: value` at column 0
//! — matching what the Perl seed does. When the manifest structure grows,
//! swap in `serde_yaml`.

use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::Path;

pub struct Intent {
    pub repo: String,
    #[allow(dead_code)] // wired into the run record once render() lands
    pub version: Option<String>,
}

pub fn load(root: &Path, target: &str) -> Result<Intent> {
    let path = root.join(format!("targets/{target}.intent.yaml"));
    let text = fs::read_to_string(&path)
        .with_context(|| format!("read {}", path.display()))?;

    let repo = scalar(&text, "repo")
        .ok_or_else(|| anyhow!("no `repo:` scalar in {}", path.display()))?;
    let version = scalar(&text, "schema_version");

    Ok(Intent { repo, version })
}

/// Grep for a `key: value` line anywhere (any indentation), like the seed's
/// Perl equivalent. We accept the first hit — good enough while the manifest
/// has a single scalar per key. Comments after `#` are stripped.
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
