//! beadle — CLI entrypoint.
//!
//! Phase 2 shape:
//!   beadle enum   <target>   → fetch open issues via `gh`, append new observations
//!   beadle sync   <target>   → delta-sweep comments on open issues (item E)
//!   beadle render <target>   → materialize dashboard body to stdout
//!   beadle push   <target>   → write rendered body to the dashboard issue
//!
//! Only `enum` is wired in this cut. The others are stubs with the intended
//! contract commented — we build them out as each phase demands them.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod enumerate;
mod gh;
mod intent;
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
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let root = cli.root.unwrap_or_else(|| PathBuf::from("."));

    match cli.cmd {
        Cmd::Enum { target, full } => enumerate::run(&root, &target, full),
        Cmd::Sync { target } => sync::run(&root, &target),
        Cmd::Render { target } => {
            anyhow::bail!("`render` (item B/C) not implemented yet — target={target}")
        }
        Cmd::Push { target, dry_run } => {
            anyhow::bail!(
                "`push` not implemented yet — target={target}, dry_run={dry_run}"
            )
        }
    }
    .with_context(|| "beadle command failed")
}
