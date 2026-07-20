//! beadle-store — append-only JSONL store for a beadle target.
//!
//! Every record has `kind` + `ts` + `target`. See `store/README.md` for the
//! schema. This crate only cares about reading and writing records; the
//! semantics of a run (enumerate, classify, render) live one crate up.

use std::{
    collections::BTreeMap,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// One row in `state.jsonl`. We keep this as a tagged enum for the well-known
/// kinds, plus an `Other` catch-all so forward-compat rows survive round-trips.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Record {
    Run(RunRecord),
    Issue(IssueRecord),
    Classification(Box<ClassificationRecord>),
    CommentEvent(CommentEventRecord),
    Cluster(ClusterRecord),
    Note(NoteRecord),
    /// Any kind we don't recognize — preserved verbatim on rewrite.
    #[serde(untagged)]
    Other(Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunRecord {
    pub ts: String,
    pub target: String,
    pub run: u32,
    #[serde(default)]
    pub watermark_before: u32,
    pub watermark_after: u32,
    pub counts: Counts,
    pub digest: String,
    #[serde(default)]
    pub warmup: Option<String>,
    #[serde(default)]
    pub intent_version: Option<String>,
    #[serde(default)]
    pub new_this_run: Vec<u32>,
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Counts {
    #[serde(default)]
    pub open: u32,
    #[serde(default)]
    pub arcavenai_open: u32,
    #[serde(default)]
    pub maintainer_engaged: u32,
    #[serde(default)]
    pub arcavenai_closed_alltime: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueRecord {
    pub ts: String,
    pub target: String,
    pub number: u32,
    pub observed_in_run: u32,
    pub title: String,
    pub author: String,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default)]
    pub closed_at: Option<String>,
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(default)]
    pub assignees: Vec<String>,
    pub body_len: usize,
    pub body_sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationRecord {
    pub ts: String,
    pub target: String,
    pub number: u32,
    pub run: u32,
    pub report_type: String,
    pub defect_nature: String,
    pub reproducibility: String,
    pub leverage: String,
    pub alignment: String,
    pub provenance: String,
    pub integrity: bool,
    #[serde(default)]
    pub integrity_anchor: Option<String>,
    #[serde(default)]
    pub operational_impact: Option<String>,
    /// Safety-cluster flag (finding-004): the issue is in the silent-data-loss
    /// class. Orthogonal to `operational_impact`, which is the finding-009
    /// liveness axis. Records ingested before the finding-009 alignment
    /// carried SDL inside `operational_impact`; read via
    /// [`ClassificationRecord::is_silent_data_loss`], which covers both.
    #[serde(default)]
    pub silent_data_loss: bool,
    pub priority: String,
    #[serde(default)]
    pub cluster: Vec<String>,
    #[serde(default)]
    pub quick_win_eligible: bool,
    pub rationale: String,
    #[serde(default)]
    pub cited_evidence: Option<String>,
    #[serde(default)]
    pub quick_win_disqualification: Option<String>,
}

impl ClassificationRecord {
    /// True when the record is in the silent-data-loss safety class — either
    /// via the dedicated `silent_data_loss` field or via the legacy
    /// pre-finding-009 encoding inside `operational_impact` (stores that have
    /// not yet run `beadle classify migrate-impact`).
    pub fn is_silent_data_loss(&self) -> bool {
        self.silent_data_loss || self.operational_impact.as_deref() == Some("silent-data-loss")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentEventRecord {
    pub ts: String,
    pub target: String,
    pub number: u32,
    pub event: String,
    pub actor: String,
    pub actor_role: String,
    #[serde(default)]
    pub body_len: Option<usize>,
    #[serde(default)]
    pub body_sha256: Option<String>,
    pub observed_in_run: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterRecord {
    pub ts: String,
    pub target: String,
    pub name: String,
    pub run: u32,
    #[serde(default)]
    pub description: Option<String>,
    pub members: Vec<u32>,
    pub last_added_run: u32,
    pub decay: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteRecord {
    pub ts: String,
    pub target: String,
    pub run: u32,
    pub topic: String,
    pub text: String,
}

/// A store rooted at `store/<target>/`.
pub struct Store {
    root: PathBuf,
    target: String,
}

impl Store {
    pub fn open(root: impl AsRef<Path>, target: &str) -> Result<Self> {
        let root = root.as_ref().join(target);
        std::fs::create_dir_all(&root)
            .with_context(|| format!("create store dir {}", root.display()))?;
        Ok(Self {
            root,
            target: target.to_string(),
        })
    }

    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn state_path(&self) -> PathBuf {
        self.root.join("state.jsonl")
    }

    /// Read every record in order. Unknown-kind rows come back as `Record::Other`.
    pub fn read_all(&self) -> Result<Vec<Record>> {
        let path = self.state_path();
        if !path.exists() {
            return Ok(vec![]);
        }
        let file = File::open(&path).with_context(|| format!("open {}", path.display()))?;
        let reader = BufReader::new(file);
        let mut out = Vec::new();
        for (i, line) in reader.lines().enumerate() {
            let line =
                line.with_context(|| format!("read line {} of {}", i + 1, path.display()))?;
            if line.trim().is_empty() {
                continue;
            }
            let rec: Record = serde_json::from_str(&line)
                .with_context(|| format!("parse line {} of {}", i + 1, path.display()))?;
            out.push(rec);
        }
        Ok(out)
    }

    /// Append records. Writes are line-buffered and flushed before return.
    /// Not atomic across a crash — but each line is atomic (JSONL invariant).
    pub fn append(&self, recs: &[Record]) -> Result<()> {
        let path = self.state_path();
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .with_context(|| format!("open for append {}", path.display()))?;
        let mut w = BufWriter::new(file);
        for rec in recs {
            let s = to_canonical_json(rec)?;
            writeln!(w, "{}", s)?;
        }
        w.flush()?;
        Ok(())
    }

    /// Atomically replace the entire store contents: write a temp file, back
    /// up any existing state to `state.jsonl.bak-<label>`, rename the temp
    /// into place. Migration-only path — `append` remains the normal write.
    /// Returns the backup path when a prior state file existed.
    pub fn rewrite(&self, recs: &[Record], backup_label: &str) -> Result<Option<PathBuf>> {
        let path = self.state_path();
        let tmp = self.root.join("state.jsonl.tmp");
        {
            let file = File::create(&tmp).with_context(|| format!("create {}", tmp.display()))?;
            let mut w = BufWriter::new(file);
            for rec in recs {
                let s = to_canonical_json(rec)?;
                writeln!(w, "{}", s)?;
            }
            w.flush()?;
        }
        let backup = if path.exists() {
            let bak = self.root.join(format!("state.jsonl.bak-{backup_label}"));
            std::fs::rename(&path, &bak)
                .with_context(|| format!("back up {} to {}", path.display(), bak.display()))?;
            Some(bak)
        } else {
            None
        };
        std::fs::rename(&tmp, &path)
            .with_context(|| format!("move {} into place", tmp.display()))?;
        Ok(backup)
    }

    /// Latest RunRecord, or None if the store has none.
    pub fn latest_run(&self) -> Result<Option<RunRecord>> {
        let recs = self.read_all()?;
        Ok(recs.into_iter().rev().find_map(|r| {
            if let Record::Run(rr) = r {
                Some(rr)
            } else {
                None
            }
        }))
    }

    /// Watermark = highest issue number ever observed. Used by the enumerator
    /// to bound its `gh issue list` query.
    pub fn watermark(&self) -> Result<u32> {
        let recs = self.read_all()?;
        Ok(recs
            .iter()
            .filter_map(|r| match r {
                Record::Issue(i) => Some(i.number),
                _ => None,
            })
            .max()
            .unwrap_or(0))
    }
}

/// Canonical JSON: sorted keys, no trailing newline. Matches the Perl seed.
pub fn to_canonical_json(rec: &Record) -> Result<String> {
    let v: Value = serde_json::to_value(rec)?;
    let sorted = canonicalize(v);
    Ok(serde_json::to_string(&sorted)?)
}

fn canonicalize(v: Value) -> Value {
    match v {
        Value::Object(m) => {
            let sorted: BTreeMap<String, Value> =
                m.into_iter().map(|(k, v)| (k, canonicalize(v))).collect();
            Value::Object(sorted.into_iter().collect())
        }
        Value::Array(a) => Value::Array(a.into_iter().map(canonicalize).collect()),
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn roundtrip_run_record() {
        let td = TempDir::new().unwrap();
        let store = Store::open(td.path(), "test").unwrap();
        let rec = Record::Run(RunRecord {
            ts: "2026-07-01T00:00:00Z".into(),
            target: "test".into(),
            run: 1,
            watermark_before: 0,
            watermark_after: 10,
            counts: Counts {
                open: 5,
                ..Default::default()
            },
            digest: "abc".into(),
            warmup: Some("cold-start".into()),
            intent_version: Some("test@0.1".into()),
            new_this_run: vec![1, 2, 3],
            notes: None,
        });
        store.append(&[rec]).unwrap();
        let read = store.read_all().unwrap();
        assert_eq!(read.len(), 1);
        matches!(&read[0], Record::Run(r) if r.run == 1);
    }

    #[test]
    fn watermark_from_issues() {
        let td = TempDir::new().unwrap();
        let store = Store::open(td.path(), "test").unwrap();
        for n in [42, 7, 100, 3] {
            store
                .append(&[Record::Issue(IssueRecord {
                    ts: "2026-07-01T00:00:00Z".into(),
                    target: "test".into(),
                    number: n,
                    observed_in_run: 1,
                    title: "t".into(),
                    author: "a".into(),
                    state: "open".into(),
                    created_at: "2026-07-01T00:00:00Z".into(),
                    updated_at: "2026-07-01T00:00:00Z".into(),
                    closed_at: None,
                    labels: vec![],
                    assignees: vec![],
                    body_len: 0,
                    body_sha256: "sha".into(),
                })])
                .unwrap();
        }
        assert_eq!(store.watermark().unwrap(), 100);
    }

    #[test]
    fn preserves_unknown_kinds() {
        let td = TempDir::new().unwrap();
        let path = td.path().join("test").join("state.jsonl");
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(
            &path,
            r#"{"kind":"future-thing","payload":{"x":1},"ts":"2026-07-01T00:00:00Z"}
"#,
        )
        .unwrap();
        let store = Store::open(td.path(), "test").unwrap();
        let read = store.read_all().unwrap();
        assert_eq!(read.len(), 1);
        matches!(&read[0], Record::Other(_));
    }
}
