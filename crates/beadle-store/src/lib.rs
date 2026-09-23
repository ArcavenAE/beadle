//! beadle-store — append-only JSONL store for a beadle target.
//!
//! Every record has `kind` + `ts` + `target`. See `store/README.md` for the
//! schema. This crate only cares about reading and writing records; the
//! semantics of a run (enumerate, classify, render) live one crate up.

use std::{
    collections::{BTreeMap, HashMap},
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

/// The 👤 attention lane (`attn`): the issue needs a human decision that no
/// amount of analysis can supply. `subtype` names the kind of decision
/// (`governance`, `direction`, …) with NO `attn.` prefix; `order` is the
/// reading order within the lane (`reply-needed`, `standing`, …); `why` is
/// the one-paragraph case for the human's attention.
///
/// Historical fixtures carry two older encodings of the same facet — a bare
/// string subtype with sibling `attn_reading_order` / `attn_why` keys
/// (run-12), and `{type, reason, reading_order}` (run-14). Neither is
/// accepted on ingest; `scripts/backfill-classification-fields.py` normalizes
/// them into this shape.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Attn {
    pub subtype: String,
    #[serde(default)]
    pub order: Option<String>,
    #[serde(default)]
    pub why: Option<String>,
}

/// A "this may already be fixed upstream" verdict: what the fix is believed to
/// be (`by`), how sure we are (`confidence`: low/medium/high), and what a
/// human or a later run must check to settle it (`verify`).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PossiblyFixed {
    pub by: String,
    pub confidence: String,
    #[serde(default)]
    pub verify: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
    /// Short human title for the issue — finding-005 row legibility ("title
    /// leads, verdict trails"). Absent on rows classified before the field
    /// existed; renderers fall back to `#<number>`.
    #[serde(default)]
    pub short_title: Option<String>,
    /// Where the issue sits in the maintainer's triage queue:
    /// `needs-triage` | `accepted` | `needs-information`.
    #[serde(default)]
    pub triage_state: Option<String>,
    /// The 👤 attention lane. `None` means the issue is not in the lane.
    #[serde(default)]
    pub attn: Option<Attn>,
    /// The fixed-but-open sweep verdict. `None` means not swept, or swept and
    /// found still open.
    #[serde(default)]
    pub possibly_fixed: Option<PossiblyFixed>,
}

impl ClassificationRecord {
    /// True when the record is in the 👤 attention lane.
    pub fn is_in_attn_lane(&self) -> bool {
        self.attn.is_some()
    }

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

impl Record {
    /// The run this record belongs to, whatever its kind.
    ///
    /// `Other` has no run on purpose: a row that failed its struct must not
    /// get a vote on which run a pass is working in. See [`working_run`].
    pub fn run(&self) -> Option<u32> {
        match self {
            Record::Run(r) => Some(r.run),
            Record::Issue(i) => Some(i.observed_in_run),
            Record::Classification(c) => Some(c.run),
            Record::CommentEvent(e) => Some(e.observed_in_run),
            Record::Cluster(c) => Some(c.run),
            Record::Note(n) => Some(n.run),
            Record::Other(_) => None,
        }
    }
}

/// The run a pass is working in: the highest run ANY record claims, not the
/// highest run that has a `Run` record.
///
/// The two differ for the whole length of a pass, and that gap is a defect
/// (finding-019 root cause 4, observed in runs 10 and 11). `beadle enum`
/// appends observations tagged with the new run immediately; the `Run` record
/// is only written later, at push. Anything keying on `Record::Run` therefore
/// computes against the PREVIOUS run for the entire pass it is supposed to
/// inform — direction reported `run: 9` and "no classification records for
/// run 9" while 66 fresh run-10 records sat in the same store.
///
/// Max, not last: file order is not run order, and a resumed pass can append
/// an older run's record after a newer one.
pub fn working_run(records: &[Record]) -> u32 {
    records.iter().filter_map(Record::run).max().unwrap_or(0)
}

/// For each issue number, the most-recent observation in the store.
///
/// "Most recent" is by `observed_in_run`, not file order: a resumed pass can
/// append an older run's observation after a newer one. Lives here rather than
/// in `render` because enumeration needs the same view — it has to know what
/// the store currently believes is open in order to notice that GitHub has
/// closed something (ArcavenAE/beadle#81).
pub fn latest_issue_observations(records: &[Record]) -> Vec<IssueRecord> {
    let mut by_number: HashMap<u32, IssueRecord> = HashMap::new();
    for rec in records {
        if let Record::Issue(i) = rec {
            let keep = by_number
                .get(&i.number)
                .map(|prev| prev.observed_in_run <= i.observed_in_run)
                .unwrap_or(true);
            if keep {
                by_number.insert(i.number, i.clone());
            }
        }
    }
    let mut out: Vec<IssueRecord> = by_number.into_values().collect();
    out.sort_by_key(|i| std::cmp::Reverse(i.number));
    out
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

    fn full_classification() -> ClassificationRecord {
        ClassificationRecord {
            ts: "2026-09-09T19:13:30Z".into(),
            target: "t".into(),
            number: 762,
            run: 18,
            report_type: "enhancement".into(),
            defect_nature: "design-architectural".into(),
            reproducibility: "mandelbug".into(),
            leverage: "systemic".into(),
            alignment: "advances".into(),
            provenance: "maintainer-authored".into(),
            integrity: false,
            integrity_anchor: None,
            operational_impact: Some("degraded".into()),
            silent_data_loss: false,
            priority: "P2".into(),
            cluster: vec!["convergence-tuning".into()],
            quick_win_eligible: false,
            rationale: "r".into(),
            cited_evidence: Some("e".into()),
            quick_win_disqualification: Some("d".into()),
            short_title: Some("Records-tier treadmill".into()),
            triage_state: Some("accepted".into()),
            attn: Some(Attn {
                subtype: "governance".into(),
                order: Some("reply-needed".into()),
                why: Some("a consent loop only a human can close".into()),
            }),
            possibly_fixed: Some(PossiblyFixed {
                by: "PR #776".into(),
                confidence: "low".into(),
                verify: Some("treat as open".into()),
            }),
        }
    }

    /// beadle#66: the four late-added fields must survive the store, and the
    /// serialized row must carry all 25 keys (21 + the four).
    #[test]
    fn classification_roundtrips_late_added_fields() {
        let td = TempDir::new().unwrap();
        let store = Store::open(td.path(), "t").unwrap();
        let rec = full_classification();
        store
            .append(&[Record::Classification(Box::new(rec.clone()))])
            .unwrap();

        let line = std::fs::read_to_string(store.state_path()).unwrap();
        let v: Value = serde_json::from_str(line.trim()).unwrap();
        assert_eq!(v.as_object().unwrap().len(), 25, "row must carry 25 keys");

        match &store.read_all().unwrap()[0] {
            Record::Classification(c) => {
                assert_eq!(c.short_title.as_deref(), Some("Records-tier treadmill"));
                assert_eq!(c.triage_state.as_deref(), Some("accepted"));
                assert_eq!(c.attn, rec.attn);
                assert_eq!(c.possibly_fixed, rec.possibly_fixed);
                assert!(c.is_in_attn_lane());
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    /// Rows written before the fields existed must still load — ~300 of them
    /// are in the live vsdd-factory store.
    #[test]
    fn classification_without_late_added_fields_still_loads() {
        let line = r#"{"kind":"classification","ts":"t","target":"x","number":1,"run":9,"report_type":"bug","defect_nature":"logic","reproducibility":"bohrbug","leverage":"a","alignment":"b","provenance":"c","integrity":false,"integrity_anchor":null,"operational_impact":null,"silent_data_loss":false,"priority":"P2","cluster":[],"quick_win_eligible":false,"rationale":"r","cited_evidence":null,"quick_win_disqualification":null}"#;
        match serde_json::from_str::<Record>(line).unwrap() {
            Record::Classification(c) => {
                assert!(c.short_title.is_none());
                assert!(c.attn.is_none());
                assert!(!c.is_in_attn_lane());
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    /// Why `#[serde(deny_unknown_fields)]` is NOT the schema-drift alarm here,
    /// kept as an executable note (beadle#66 / aae-orc-l5b5i).
    ///
    /// `Record` is internally tagged with an untagged `Other` catch-all. With
    /// the attribute on `ClassificationRecord`, a drifted row does not error —
    /// the variant declines it and the row lands in `Record::Other`, where it
    /// round-trips happily while vanishing from every consumer that matches on
    /// `Record::Classification`. That converts a loud failure into exactly the
    /// silent system-of-record disagreement beadle#66 is about. The gate lives
    /// in `classify::ingest` instead, which owns the payload contract; the
    /// store stays permissive so historical rows keep loading.
    #[test]
    fn unknown_key_on_a_classification_row_is_not_caught_by_serde() {
        let line = r#"{"kind":"classification","ts":"t","target":"x","number":1,"run":9,"report_type":"bug","defect_nature":"logic","reproducibility":"bohrbug","leverage":"a","alignment":"b","provenance":"c","integrity":false,"priority":"P2","rationale":"r","alignment_rationale":"DRIFT"}"#;
        match serde_json::from_str::<Record>(line).unwrap() {
            // Today: parsed as a Classification, the extra key dropped.
            Record::Classification(c) => assert_eq!(c.number, 1),
            // With deny_unknown_fields it would land here instead — accepted,
            // invisible, and unclassified. Worse, not better.
            Record::Other(v) => panic!("fell through to Other: {v}"),
            other => panic!("unexpected {other:?}"),
        }
    }

    /// Records written during a pass, before the run record exists.
    fn jsonl_mid_pass() -> &'static str {
        concat!(
            r#"{"kind":"run","ts":"t","target":"x","run":19,"watermark_after":830,"#,
            r#""counts":{},"digest":"d"}"#,
            "\n",
            r#"{"kind":"issue","ts":"t","target":"x","number":901,"observed_in_run":20,"#,
            r#""title":"a","author":"b","state":"open","created_at":"t","updated_at":"t","#,
            r#""body_len":1,"body_sha256":"s"}"#,
            "\n",
            r#"{"kind":"note","ts":"t","target":"x","run":20,"topic":"perf","text":"t"}"#,
            "\n",
        )
    }

    fn read_jsonl(body: &str) -> (TempDir, Vec<Record>) {
        let td = TempDir::new().unwrap();
        let path = td.path().join("t").join("state.jsonl");
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, body).unwrap();
        let recs = Store::open(td.path(), "t").unwrap().read_all().unwrap();
        (td, recs)
    }

    #[test]
    fn working_run_is_the_open_run_not_the_last_finalized_one() {
        let (_td, recs) = read_jsonl(jsonl_mid_pass());
        // The defect this guards: the newest `Run` RECORD still says 19 while
        // the pass is demonstrably working in 20. Anything keying on
        // `Record::Run` reads 19 for the whole pass.
        let newest_run_record = recs
            .iter()
            .rev()
            .find_map(|r| match r {
                Record::Run(rr) => Some(rr.run),
                _ => None,
            })
            .unwrap();
        assert_eq!(newest_run_record, 19, "the stale value the defect reads");
        assert_eq!(working_run(&recs), 20, "the run the pass is actually in");
    }

    #[test]
    fn working_run_takes_the_max_not_the_last() {
        // A resumed pass can append an older run's record after a newer one, so
        // file order is not run order.
        let body = format!(
            "{}{}",
            jsonl_mid_pass(),
            concat!(
                r#"{"kind":"note","ts":"t","target":"x","run":3,"topic":"x","text":"late"}"#,
                "\n"
            )
        );
        let (_td, recs) = read_jsonl(&body);
        assert_eq!(recs.last().unwrap().run(), Some(3));
        assert_eq!(working_run(&recs), 20);
    }

    #[test]
    fn a_malformed_row_gets_no_vote_on_the_working_run() {
        // `Other` is where a row that claims a known kind and fails its struct
        // lands. It must not be able to drag a pass into a run nothing else is
        // working in.
        let body = format!(
            "{}{}",
            jsonl_mid_pass(),
            concat!(
                r#"{"kind":"issue","number":999,"observed_in_run":99}"#,
                "\n"
            )
        );
        let (_td, recs) = read_jsonl(&body);
        assert!(matches!(recs.last().unwrap(), Record::Other(_)));
        assert_eq!(recs.last().unwrap().run(), None);
        assert_eq!(working_run(&recs), 20);
    }

    #[test]
    fn working_run_of_an_empty_store_is_zero() {
        assert_eq!(working_run(&[]), 0);
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
