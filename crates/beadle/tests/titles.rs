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

/// name, size and mtime of every file under `dir`, sorted.
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
    let out = Command::new(env!("CARGO_BIN_EXE_beadle"))
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
    assert_eq!(before, snapshot(td.path()), "the store must not change");
}
