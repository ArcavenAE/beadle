//! `beadle enum <target>` — watermark-bounded issue enumeration.
//!
//! Fetches open issues from the target repo via `gh`, filters to numbers
//! strictly greater than the current watermark (or --full = ignore watermark),
//! and appends `IssueRecord` rows to the store. Writes a summary to stderr;
//! JSON summary of new numbers to stdout so it composes with a shell pipe.

use anyhow::{Context, Result};
use beadle_store::{IssueRecord, Record, Store};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::path::Path;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

use crate::{gh, intent};

#[derive(Deserialize)]
struct GhAuthor {
    #[serde(default)]
    login: Option<String>,
}

#[derive(Deserialize)]
struct GhLabel {
    name: String,
}

#[derive(Deserialize)]
struct GhAssignee {
    login: String,
}

#[derive(Deserialize)]
struct GhIssue {
    number: u32,
    title: String,
    #[serde(default)]
    author: Option<GhAuthor>,
    state: String,
    #[serde(rename = "createdAt")]
    created_at: String,
    #[serde(rename = "updatedAt")]
    updated_at: String,
    #[serde(rename = "closedAt", default)]
    closed_at: Option<String>,
    #[serde(default)]
    labels: Vec<GhLabel>,
    #[serde(default)]
    assignees: Vec<GhAssignee>,
    #[serde(default)]
    body: String,
}

pub fn run(root: &Path, target: &str, full: bool) -> Result<()> {
    let intent = intent::load(root, target)?;
    let store = Store::open(root.join("store"), target)?;

    let watermark = if full { 0 } else { store.watermark()? };
    let latest_run = store.latest_run()?;
    let next_run = latest_run.as_ref().map(|r| r.run + 1).unwrap_or(1);

    eprintln!(
        "beadle enum: target={target} repo={} watermark={watermark} run={next_run}",
        intent.repo
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
        "number,title,author,state,createdAt,updatedAt,closedAt,labels,assignees,body",
    ];
    let issues: Vec<GhIssue> = gh::json(&args)?;
    eprintln!("beadle enum: fetched {} open issues", issues.len());

    let now = OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .context("format RFC3339 timestamp")?;

    let mut new_numbers: Vec<u32> = Vec::new();
    let mut recs: Vec<Record> = Vec::new();
    for iss in &issues {
        if iss.number <= watermark {
            continue;
        }
        let body_bytes = iss.body.as_bytes();
        let mut hasher = Sha256::new();
        hasher.update(body_bytes);
        let body_sha = format!("{:x}", hasher.finalize());

        recs.push(Record::Issue(IssueRecord {
            ts: iss.updated_at.clone(),
            target: target.to_string(),
            number: iss.number,
            observed_in_run: next_run,
            title: iss.title.clone(),
            author: iss
                .author
                .as_ref()
                .and_then(|a| a.login.clone())
                .unwrap_or_else(|| "unknown".to_string()),
            state: iss.state.to_lowercase(),
            created_at: iss.created_at.clone(),
            updated_at: iss.updated_at.clone(),
            closed_at: iss.closed_at.clone(),
            labels: iss.labels.iter().map(|l| l.name.clone()).collect(),
            assignees: iss.assignees.iter().map(|a| a.login.clone()).collect(),
            body_len: body_bytes.len(),
            body_sha256: body_sha,
        }));
        new_numbers.push(iss.number);
    }
    new_numbers.sort_unstable();

    if recs.is_empty() {
        eprintln!("beadle enum: no new issues above watermark {watermark}");
    } else {
        store.append(&recs)?;
        eprintln!(
            "beadle enum: appended {} new issue observations (min={} max={})",
            recs.len(),
            new_numbers.first().copied().unwrap_or(0),
            new_numbers.last().copied().unwrap_or(0)
        );
    }

    // stdout: machine-readable summary; drives the next step (sync/render).
    let summary = serde_json::json!({
        "target": target,
        "run": next_run,
        "watermark_before": watermark,
        "watermark_after": new_numbers.last().copied().unwrap_or(watermark),
        "new_issue_numbers": new_numbers,
        "open_issue_count": issues.len(),
        "timestamp": now,
    });
    println!("{}", summary);
    Ok(())
}
