//! `beadle sync <target>` — comment delta-sweep across open issues.
//!
//! Purpose: capture maintainer-engagement events (the compass) without
//! re-reading whole issue bodies. We fetch open issues once with
//! `gh issue list ... --json comments`, then emit a `CommentEvent` for every
//! comment we haven't seen before (per (issue, ts, actor) tuple).
//!
//! Cost: one `gh` call regardless of open-issue count, thanks to bulk fields.

use anyhow::Result;
use beadle_store::{CommentEventRecord, Record, Store};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::path::Path;

use crate::{gh, intent};

#[derive(Deserialize)]
struct GhCommentAuthor {
    #[serde(default)]
    login: Option<String>,
}

#[derive(Deserialize)]
struct GhComment {
    #[serde(default)]
    author: Option<GhCommentAuthor>,
    #[serde(rename = "createdAt")]
    created_at: String,
    #[serde(default)]
    body: String,
}

#[derive(Deserialize)]
struct GhIssueForSync {
    number: u32,
    #[serde(default)]
    comments: Vec<GhComment>,
}

pub fn run(root: &Path, target: &str) -> Result<()> {
    let intent = intent::load(root, target)?;
    let store = Store::open(root.join("store"), target)?;

    // Load existing comment fingerprints so we can skip them.
    let seen: HashSet<(u32, String, String)> = store
        .read_all()?
        .into_iter()
        .filter_map(|r| match r {
            Record::CommentEvent(c) => Some((c.number, c.ts.clone(), c.actor.clone())),
            _ => None,
        })
        .collect();

    let latest_run = store.latest_run()?;
    let next_run = latest_run.as_ref().map(|r| r.run + 1).unwrap_or(1);

    eprintln!(
        "beadle sync: target={target} repo={} run={next_run} known-comments={}",
        intent.repo,
        seen.len()
    );

    let args = vec![
        "issue",
        "list",
        "--repo",
        intent.repo.as_str(),
        "--state",
        "open",
        "--limit",
        "500",
        "--json",
        "number,comments",
    ];
    let issues: Vec<GhIssueForSync> = gh::json(&args)?;

    let mut recs: Vec<Record> = Vec::new();
    let mut maintainer_events = 0u32;
    let mut measured_events = 0u32;
    let mut other_events = 0u32;

    for iss in &issues {
        for c in &iss.comments {
            let actor = c
                .author
                .as_ref()
                .and_then(|a| a.login.clone())
                .unwrap_or_else(|| "unknown".to_string());
            let key = (iss.number, c.created_at.clone(), actor.clone());
            if seen.contains(&key) {
                continue;
            }
            let role = intent.actor_role(&actor);
            match role {
                "maintainer" => maintainer_events += 1,
                "measured" => measured_events += 1,
                _ => other_events += 1,
            }

            let body_bytes = c.body.as_bytes();
            let mut hasher = Sha256::new();
            hasher.update(body_bytes);
            let body_sha = format!("{:x}", hasher.finalize());

            recs.push(Record::CommentEvent(CommentEventRecord {
                ts: c.created_at.clone(),
                target: target.to_string(),
                number: iss.number,
                event: "comment".to_string(),
                actor,
                actor_role: role.to_string(),
                body_len: Some(body_bytes.len()),
                body_sha256: Some(body_sha),
                observed_in_run: next_run,
            }));
        }
    }

    if !recs.is_empty() {
        store.append(&recs)?;
    }
    eprintln!(
        "beadle sync: appended {} events (maintainer={} measured={} other={})",
        recs.len(),
        maintainer_events,
        measured_events,
        other_events
    );

    let summary = serde_json::json!({
        "target": target,
        "run": next_run,
        "new_events": recs.len(),
        "maintainer_events": maintainer_events,
        "measured_events": measured_events,
        "other_events": other_events,
        "open_issues_scanned": issues.len(),
    });
    println!("{}", summary);
    Ok(())
}
