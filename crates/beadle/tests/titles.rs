//! `beadle titles <target>`: the stored issue titles, one per line, for the
//! Python gate's `--titles FILE`. It only reads the store.

use std::{fs, path::Path, process::Command};

use serde_json::{json, Value};
use tempfile::TempDir;

const INTENT: &str = "schema_version: 0.3\nrepo: acme/widget\nmaintainers:\n  - alice\n";

fn issue(number: u32, run: u32, title: &str) -> Value {
    json!({
        "kind": "issue", "ts": "2026-09-09T00:00:00Z", "target": "t",
        "number": number, "observed_in_run": run, "title": title,
        "author": "bob", "state": "open",
        "created_at": "2026-09-01T00:00:00Z", "updated_at": "2026-09-02T00:00:00Z",
        "body_len": 1, "body_sha256": "s",
    })
}

fn workspace(rows: &[Value]) -> TempDir {
    let td = TempDir::new().unwrap();
    fs::create_dir_all(td.path().join("targets")).unwrap();
    fs::write(td.path().join("targets/t.intent.yaml"), INTENT).unwrap();
    let store = td.path().join("store/t");
    fs::create_dir_all(&store).unwrap();
    let jsonl: String = rows
        .iter()
        .map(|r| format!("{}\n", serde_json::to_string(r).unwrap()))
        .collect();
    fs::write(store.join("state.jsonl"), jsonl).unwrap();
    td
}

/// name, inode, size, mtime and ctime of every file and directory under `dir`,
/// sorted. A rewrite changes the mtime or the inode, a rename the inode.
fn snapshot(dir: &Path) -> Vec<String> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for e in fs::read_dir(&d).unwrap().flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else {
                let m = fs::metadata(&p).unwrap();
                out.push(format!(
                    "{} {} {:?}",
                    p.display(),
                    m.len(),
                    m.modified().unwrap()
                ));
            }
        }
    }
    out.sort();
    out
}

#[test]
fn beadle_titles_is_read_only() {
    let td = workspace(&[
        issue(1, 1, "first title"),
        issue(1, 2, "first title, edited"),
        issue(2, 2, "  second title  "),
        issue(3, 2, ""),
    ]);
    let before = snapshot(td.path());

    // A gh stub first on PATH logs every call; the command must make none.
    let bin = TempDir::new().unwrap();
    let log = bin.path().join("gh.log");
    let stub = bin.path().join("gh");
    fs::write(
        &stub,
        format!("#!/bin/sh\necho \"$@\" >> '{}'\nexit 1\n", log.display()),
    )
    .unwrap();
    fs::set_permissions(&stub, std::os::unix::fs::PermissionsExt::from_mode(0o755)).unwrap();
    let path = format!(
        "{}:{}",
        bin.path().display(),
        std::env::var("PATH").unwrap_or_default()
    );

    // Control: the stub is reachable by name, so an unlogged run means no call.
    let probe = Command::new("gh")
        .env("PATH", &path)
        .arg("control")
        .output()
        .expect("run the gh stub");
    assert!(!probe.status.success() && fs::read_to_string(&log).unwrap().contains("control"));
    fs::remove_file(&log).unwrap();
    let before_stub = snapshot(td.path());
    assert_eq!(before, before_stub, "the control must not touch the store");

    let out = Command::new(env!("CARGO_BIN_EXE_beadle"))
        .env("PATH", &path)
        .args(["--root".as_ref(), td.path().as_os_str()])
        .args(["titles", "t"])
        .output()
        .expect("run beadle titles");
    assert!(
        out.status.success(),
        "beadle titles failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8(out.stdout).unwrap();
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(
        lines,
        vec!["first title, edited", "second title"],
        "latest observation per issue, trimmed, blanks dropped"
    );
    assert!(
        !log.exists(),
        "beadle titles called gh: {}",
        fs::read_to_string(&log).unwrap_or_default()
    );
    assert_eq!(
        before,
        snapshot(td.path()),
        "the store must not change: no file rewritten, replaced or added"
    );
}
