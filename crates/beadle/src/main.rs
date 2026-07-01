//! beadle — CLI entrypoint.
//!
//! Phase 2 shape:
//!   beadle enum   <target>   → fetch open issues via `gh`, append new observations
//!   beadle sync   <target>   → delta-sweep comments on open issues (item E)
//!   beadle render <target>   → materialize dashboard body to stdout
//!   beadle push   <target>   → write rendered body to the dashboard issue
//!                              (preserves editor slots, finalizes body_digest)

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod classify;
mod direction;
mod enumerate;
mod gh;
mod intent;
mod push;
mod render;
mod sync;

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
    Sync {
        target: String,
    },
    /// Render dashboard body from the store (item B/C).
    Render {
        target: String,
    },
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
    }
    .with_context(|| "beadle command failed")
}
