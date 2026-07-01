//! Thin wrapper around the `gh` CLI. We shell out for MVP; `octocrab` is
//! the natural upgrade when we need pagination-past-500 or GraphQL.

use anyhow::{anyhow, Context, Result};
use serde::de::DeserializeOwned;
use std::process::Command;

/// Run `gh <args>` and parse stdout as JSON into `T`. Stderr is surfaced on
/// non-zero exit; stdout is captured either way.
pub fn json<T: DeserializeOwned>(args: &[&str]) -> Result<T> {
    let out = Command::new("gh")
        .args(args)
        .output()
        .with_context(|| format!("spawn gh {:?}", args))?;
    if !out.status.success() {
        return Err(anyhow!(
            "gh {:?} failed (exit {}): {}",
            args,
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    let stdout = String::from_utf8(out.stdout).context("gh stdout not utf-8")?;
    serde_json::from_str(&stdout).with_context(|| format!("parse gh {:?} JSON output", args))
}
