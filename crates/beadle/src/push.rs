//! `beadle push <target>` — write the rendered body to the dashboard issue.
//!
//! Preserves editor-slot contents from the live body so the maintainer's
//! direction verdict and notes survive regeneration. Computes the final
//! `body_digest` after slot merge and updates the sentinel in place.
//!
//! Dry-run mode prints what would change without touching GitHub.

use anyhow::{anyhow, Context, Result};
use beadle_store::{NoteRecord, Record, Store};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

use crate::{controls, gh, intent, render};

const DIRECTION_OPEN: &str = "<!-- editor:direction-verdict -->";
const DIRECTION_CLOSE: &str = "<!-- /editor:direction-verdict -->";
const NOTES_OPEN: &str = "<!-- editor:notes -->";
const NOTES_CLOSE: &str = "<!-- /editor:notes -->";

#[derive(Deserialize)]
struct GhIssueRef {
    number: u32,
    body: String,
}

pub fn run(root: &Path, target: &str, dry_run: bool) -> Result<()> {
    let intent = intent::load(root, target)?;
    let dashboard = find_dashboard_issue(&intent.repo)?;
    eprintln!(
        "beadle push: target={target} repo={} dashboard=#{} dry_run={dry_run}",
        intent.repo, dashboard.number
    );

    // Scan the LIVE body for checked board-control boxes (F5 Tier 2).
    // Record one Note per verb before rendering; the fresh render always
    // emits unchecked boxes, so the next push through the same body
    // resets the box (natural de-bounce). Dispatch of the requested
    // routines is deferred — Phase-1 gh-aw will drain the notes queue.
    record_board_controls(root, target, &dashboard.body, dry_run)?;

    let rendered = render::run(root, target)?;

    // Extract editor slots from the LIVE body; preserve them across regen.
    let direction = extract_slot(&dashboard.body, DIRECTION_OPEN, DIRECTION_CLOSE)
        .unwrap_or_else(default_direction_slot);
    let notes =
        extract_slot(&dashboard.body, NOTES_OPEN, NOTES_CLOSE).unwrap_or_else(default_notes_slot);

    // Merge: substitute the fresh renderer's placeholder slots with the
    // live-body's editor content.
    let merged = merge_slots(&rendered, &direction, &notes);

    // Finalize sentinel: compute body_digest over the merged body (minus the
    // sentinel line itself — pre-image invariant so the digest isn't self-referential).
    let final_body = finalize_sentinel(&merged);

    if dry_run {
        eprintln!("beadle push: dry-run — writing to stdout, {} bytes", final_body.len());
        print!("{}", final_body);
        return Ok(());
    }

    // Edit the dashboard issue via `gh issue edit --body-file -`.
    let mut child = Command::new("gh")
        .args([
            "issue",
            "edit",
            &dashboard.number.to_string(),
            "--repo",
            intent.repo.as_str(),
            "--body-file",
            "-",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("spawn gh issue edit")?;
    child
        .stdin
        .as_mut()
        .ok_or_else(|| anyhow!("gh stdin unavailable"))?
        .write_all(final_body.as_bytes())
        .context("write body to gh stdin")?;
    let out = child.wait_with_output().context("wait for gh")?;
    if !out.status.success() {
        return Err(anyhow!(
            "gh issue edit failed (exit {}): {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    eprintln!(
        "beadle push: pushed {} bytes to {} issue #{}",
        final_body.len(),
        intent.repo,
        dashboard.number
    );
    Ok(())
}

/// Extract any checked board-verb boxes from the live dashboard body and
/// append one `control-request` note per verb to the store.
///
/// - Renderer never emits checked boxes, so an unchanged live body yields
///   zero notes (fast path).
/// - Dry-run does not append; it only reports what it would have recorded.
/// - `run` on the note references the store's latest run (0 if none).
fn record_board_controls(root: &Path, target: &str, live_body: &str, dry_run: bool) -> Result<()> {
    let verbs = controls::extract_checked_verbs(live_body);
    if verbs.is_empty() {
        return Ok(());
    }
    let store = Store::open(root.join("store"), target)?;
    let run = store
        .latest_run()
        .ok()
        .flatten()
        .map(|r| r.run)
        .unwrap_or(0);
    let ts = OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .unwrap_or_default();
    let recs: Vec<Record> = verbs
        .iter()
        .map(|v| {
            Record::Note(NoteRecord {
                ts: ts.clone(),
                target: target.to_string(),
                run,
                topic: "control-request".to_string(),
                text: format!("verb={};id=board", v),
            })
        })
        .collect();
    if dry_run {
        eprintln!(
            "beadle push: dry-run — {} board-control request(s) would be recorded ({})",
            recs.len(),
            verbs.join(", ")
        );
        return Ok(());
    }
    store.append(&recs).context("append control-request notes")?;
    eprintln!(
        "beadle push: recorded {} board-control request(s) ({})",
        recs.len(),
        verbs.join(", ")
    );
    Ok(())
}

fn find_dashboard_issue(repo: &str) -> Result<GhIssueRef> {
    let args = vec![
        "issue",
        "list",
        "--repo",
        repo,
        "--state",
        "open",
        "--author",
        "arcavenai",
        "--search",
        "beadle Triage Dashboard in:title",
        "--limit",
        "5",
        "--json",
        "number,body",
    ];
    let issues: Vec<GhIssueRef> = gh::json(&args)?;
    let hits: Vec<GhIssueRef> = issues
        .into_iter()
        .filter(|i| i.body.contains("<!-- beadle-state:v1"))
        .collect();
    match hits.len() {
        0 => Err(anyhow!("no dashboard issue found with beadle-state:v1 sentinel in {repo}")),
        1 => Ok(hits.into_iter().next().unwrap()),
        n => Err(anyhow!(
            "{n} dashboard candidates in {repo} — STOP for consolidation before push"
        )),
    }
}

fn extract_slot(body: &str, open: &str, close: &str) -> Option<String> {
    let start = body.find(open)?;
    let end = body[start..].find(close)?;
    Some(body[start..start + end + close.len()].to_string())
}

/// Fresh-target defaults: extract the marker-bracketed portion from the
/// renderer constants so the extract-and-splice shapes match.
fn default_direction_slot() -> String {
    extract_slot(render::EDITOR_SLOT_DIRECTION, DIRECTION_OPEN, DIRECTION_CLOSE)
        .unwrap_or_else(|| render::EDITOR_SLOT_DIRECTION.to_string())
}

fn default_notes_slot() -> String {
    extract_slot(render::EDITOR_SLOT_NOTES, NOTES_OPEN, NOTES_CLOSE)
        .unwrap_or_else(|| render::EDITOR_SLOT_NOTES.to_string())
}

/// Replace the placeholder editor slots in the rendered body with the live
/// slot contents.
fn merge_slots(rendered: &str, direction: &str, notes: &str) -> String {
    let mut out = replace_between(rendered, DIRECTION_OPEN, DIRECTION_CLOSE, direction);
    out = replace_between(&out, NOTES_OPEN, NOTES_CLOSE, notes);
    out
}

/// Replace the entire block from `open` through `close` (inclusive) with the
/// literal `replacement`. If the placement anchors aren't found, returns the
/// original string unchanged.
fn replace_between(text: &str, open: &str, close: &str, replacement: &str) -> String {
    let Some(start) = text.find(open) else {
        return text.to_string();
    };
    let after_open = start;
    let Some(rel_end) = text[after_open..].find(close) else {
        return text.to_string();
    };
    let end = after_open + rel_end + close.len();
    let mut out = String::with_capacity(text.len() + replacement.len());
    out.push_str(&text[..start]);
    out.push_str(replacement);
    out.push_str(&text[end..]);
    out
}

/// Compute and inject `body_digest` into the sentinel. The pre-image is the
/// merged body with `"body_digest": "pending"` unchanged, so re-running push
/// after an editor tweak produces a stable digest.
fn finalize_sentinel(body: &str) -> String {
    let placeholder = "\"body_digest\": \"pending\"";
    if !body.contains(placeholder) {
        return body.to_string();
    }
    let mut hasher = Sha256::new();
    hasher.update(body.as_bytes());
    let digest = format!("{:x}", hasher.finalize());
    body.replacen(
        placeholder,
        &format!("\"body_digest\": \"{}\"", digest),
        1,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_and_replace_slot() {
        let body = "before <!-- editor:notes -->\nhi\n<!-- /editor:notes --> after";
        let got = extract_slot(body, NOTES_OPEN, NOTES_CLOSE).unwrap();
        assert!(got.starts_with(NOTES_OPEN));
        assert!(got.ends_with(NOTES_CLOSE));
        assert!(got.contains("hi"));

        let repl = replace_between(body, NOTES_OPEN, NOTES_CLOSE, "REPLACED");
        assert_eq!(repl, "before REPLACED after");
    }

    #[test]
    fn finalize_replaces_pending_digest() {
        let body = "{\"body_digest\": \"pending\"} rest";
        let final_body = finalize_sentinel(body);
        assert!(!final_body.contains("\"pending\""));
        assert!(final_body.contains("\"body_digest\": \""));
    }

    #[test]
    fn finalize_is_idempotent_on_missing_placeholder() {
        let body = "no placeholder here";
        assert_eq!(finalize_sentinel(body), body);
    }

    #[test]
    fn record_board_controls_appends_notes() {
        let td = tempfile::TempDir::new().unwrap();
        let body = "\
- [x] `full-refresh` — foo <!-- verb=full-refresh;id=board -->
- [x] `revalidate` <!-- verb=revalidate;id=board -->
";
        record_board_controls(td.path(), "vsdd-factory", body, false).unwrap();
        let store = Store::open(td.path().join("store"), "vsdd-factory").unwrap();
        let recs = store.read_all().unwrap();
        let notes: Vec<_> = recs
            .iter()
            .filter_map(|r| match r {
                Record::Note(n) if n.topic == "control-request" => Some(n),
                _ => None,
            })
            .collect();
        assert_eq!(notes.len(), 2);
        assert!(notes.iter().any(|n| n.text == "verb=full-refresh;id=board"));
        assert!(notes.iter().any(|n| n.text == "verb=revalidate;id=board"));
    }

    #[test]
    fn record_board_controls_no_op_on_unchecked() {
        let td = tempfile::TempDir::new().unwrap();
        let body = "\
- [ ] `full-refresh` <!-- verb=full-refresh;id=board -->
- [ ] `revalidate` <!-- verb=revalidate;id=board -->
";
        record_board_controls(td.path(), "vsdd-factory", body, false).unwrap();
        // No store dir should have been created — extract_checked_verbs
        // returned empty, so `record_board_controls` returned before touching
        // the filesystem.
        assert!(!td.path().join("store").exists());
    }

    #[test]
    fn record_board_controls_dry_run_does_not_write() {
        let td = tempfile::TempDir::new().unwrap();
        let body = "- [x] `reprioritize` <!-- verb=reprioritize;id=board -->\n";
        record_board_controls(td.path(), "vsdd-factory", body, true).unwrap();
        // Dry-run may still open the store (which creates the dir) but must
        // not append. Read back and confirm zero note records.
        let store = Store::open(td.path().join("store"), "vsdd-factory").unwrap();
        let recs = store.read_all().unwrap();
        assert!(
            recs.iter()
                .all(|r| !matches!(r, Record::Note(n) if n.topic == "control-request")),
            "dry-run must not append control-request notes"
        );
    }
}
