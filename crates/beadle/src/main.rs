//! beadle — CLI entrypoint.
//!
//! Phase 2 shape:
//!   beadle enum   <target>   → fetch open issues via `gh`, append new observations
//!   beadle sync   <target>   → delta-sweep comments on open issues (item E)
//!   beadle render <target>   → materialize dashboard body to stdout
//!   beadle push   <target>   → write rendered body to the dashboard issue
//!                              (preserves editor slots, finalizes body_digest)

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

mod classify;
mod controls;
mod direction;
mod enumerate;
mod gh;
mod intent;
mod migrate;
mod note;
mod push;
mod render;
mod sync;
mod vocab;

#[derive(Parser)]
#[command(name = "beadle", version, about)]
struct Cli {
    /// Path to the beadle workspace root (defaults to CWD).
    #[arg(long, global = true)]
    root: Option<PathBuf>,

    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Enumerate new-since-watermark open issues into the store.
    Enum {
        /// Target name; must match `targets/<name>.intent.yaml`.
        target: String,
        /// Force a full sweep instead of watermark-bounded delta.
        #[arg(long)]
        full: bool,
    },
    /// Delta-sweep comments on open issues (item E).
    Sync { target: String },
    /// Render dashboard body from the store (item B/C).
    Render { target: String },
    /// Push rendered body to the dashboard issue.
    Push {
        target: String,
        #[arg(long)]
        dry_run: bool,
    },
    /// Compute the run's direction verdict (item F).
    Direction {
        target: String,
        /// Also append a `note` record to the store as an audit trail.
        #[arg(long)]
        write_note: bool,
    },
    /// Ingest ClassificationRecord payload(s) produced by the classifier skill.
    #[command(subcommand)]
    Classify(ClassifyCmd),
    /// Append a note record to the store (perf ledger entries, audit trails,
    /// operator remarks). Diagnostic only — nothing gates on notes.
    Note {
        target: String,
        /// Note topic (e.g. `perf`, `migration`).
        #[arg(long)]
        topic: String,
        /// Note body; compact JSON welcome for machine-readable ledgers.
        #[arg(long)]
        text: String,
        /// Run number; defaults to the store's latest run record.
        #[arg(long)]
        run: Option<u32>,
    },
}

#[derive(Subcommand)]
enum ClassifyCmd {
    /// Append a validated ClassificationRecord (or JSON array of them) to the store.
    /// Reads from `--file` if given, else stdin.
    Ingest {
        target: String,
        #[arg(long)]
        file: Option<PathBuf>,
    },
    /// Re-map legacy (pre-finding-009) operational_impact values to the
    /// canonical liveness vocabulary, using rich classification fixtures as
    /// per-issue ground truth. Fails loud (writing nothing) on records no
    /// fixture or unambiguous mechanical map can resolve.
    MigrateImpact {
        target: String,
        /// Rich classification fixture(s) carrying ground truth (repeatable).
        #[arg(long = "fixture")]
        fixtures: Vec<PathBuf>,
        /// Report what would change without touching the store.
        #[arg(long)]
        dry_run: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let root = cli.root.unwrap_or_else(|| PathBuf::from("."));

    match cli.cmd {
        Cmd::Enum { target, full } => enumerate::run(&root, &target, full),
        Cmd::Sync { target } => sync::run(&root, &target),
        Cmd::Render { target } => {
            let body = render::run(&root, &target)?;
            print!("{}", body);
            Ok(())
        }
        Cmd::Push { target, dry_run } => push::run(&root, &target, dry_run),
        Cmd::Direction { target, write_note } => direction::run(&root, &target, write_note),
        Cmd::Classify(ClassifyCmd::Ingest { target, file }) => {
            classify::ingest(&root, &target, file.as_deref())
        }
        Cmd::Classify(ClassifyCmd::MigrateImpact {
            target,
            fixtures,
            dry_run,
        }) => migrate::migrate_impact(&root, &target, &fixtures, dry_run),
        Cmd::Note {
            target,
            topic,
            text,
            run,
        } => note::run(&root, &target, &topic, &text, run),
    }
    .with_context(|| "beadle command failed")
}
