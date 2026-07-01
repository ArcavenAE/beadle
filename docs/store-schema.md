# beadle store — on-disk, append-only, JSONL

State projection for a beadle target. Layout:

```
store/
├── README.md                       (this file)
└── <target>/
    ├── state.jsonl                 (append-only event log — the source of truth)
    └── snapshots/
        └── run-NN.json             (optional per-run materialization for fast reads)
```

The dashboard issue body carries only a **digest pointer** in its `beadle-state:v1`
block; the store is authoritative (B1 — state out-of-band). A wiped or hand-edited
dashboard body regenerates from the store without loss.

## Design principles

- **Append-only.** New observations append; nothing rewrites. Rebuilds are
  materializations of the log, not replacements.
- **One target per subdirectory.** No cross-target coupling. Fresh checkout with a
  wiped `store/` regenerates from GitHub (the enumerator is idempotent).
- **JSONL, not YAML/TOML.** Line-oriented so a Perl one-liner, a Rust `BufReader`,
  or `jq` all handle it. Every line is a valid JSON object.
- **Schema in one place.** This file. If a `kind` isn't documented here, don't emit
  it.

## Record kinds

Every line has the shape `{"kind": "...", "ts": "<ISO8601>", ...}`. Timestamps are
UTC. Fields marked *optional* may be omitted; unknown fields must be preserved on
round-trip (forward-compat).

### `kind: "run"`

Emitted once per beadle run, at the *end* (after enumerate + score + render but
before push, so a failed push doesn't lose the delta).

```json
{"kind":"run","ts":"2026-07-01T00:00:00Z","target":"vsdd-factory","run":9,
 "watermark_before":367,"watermark_after":381,
 "counts":{"open":178,"arcavenai_open":161,"maintainer_engaged":0,
           "arcavenai_closed_alltime":1},
 "digest":"burst-14-integrity-halt-substrate-corruption-381-2026-07-01",
 "warmup":"cold-start",
 "intent_version":"vsdd-factory@0.3",
 "new_this_run":[368,369,370,371,372,373,374,375,376,377,378,379,380,381],
 "notes":"one-line human-readable summary"}
```

### `kind: "issue"`

Emitted the first time an issue is observed AND on any observable change
(state, labels, title). Beadle's *own* classification is a separate `kind:"classification"`
record so we can rescore without rewriting the raw observation.

```json
{"kind":"issue","ts":"2026-07-01T13:06:31Z","target":"vsdd-factory",
 "number":380,"observed_in_run":9,
 "title":"Write-capable subagent refuses orchestrator-relayed human approval on principle...",
 "author":"arcavenai","state":"open","created_at":"2026-07-01T13:06:31Z",
 "updated_at":"2026-07-01T13:06:31Z","closed_at":null,
 "labels":[],"assignees":[],"body_len":4820,
 "body_sha256":"<hex>"}
```

`body_sha256` is over the raw body — lets us detect edits without storing the
whole body per observation. When it changes, emit a new `issue` record with the
new sha; the old one stays in the log.

### `kind: "classification"`

Beadle's judgment for one issue, one run. Multiple `classification` records for the
same `(target, number)` accumulate over time — a rescore is a new record, not an
overwrite.

```json
{"kind":"classification","ts":"2026-07-01T00:00:00Z","target":"vsdd-factory",
 "number":380,"run":9,
 "report_type":"process-gap",
 "defect_nature":"design",
 "reproducibility":"Bohr",
 "leverage":"systemic",
 "alignment":"advances",
 "provenance":"pilot-derived",
 "integrity":false,
 "integrity_anchor":null,
 "operational_impact":"halt",
 "priority":"P1",
 "cluster":["orchestrator-continuation","pr-manager-approval-contract"],
 "quick_win_eligible":false,
 "rationale":"orchestrator's AskUserQuestion protocol does not have a subagent-facing channel; subagent refusal deadlocks human-gated steps. Distinct facet from #350 (harness classifier).",
 "cited_evidence":"akey Phase-3 deadlock; author cites divergence from #350",
 "quick_win_disqualification":"finding-009 halt hard-exclude"}
```

`quick_win_disqualification` is the *why not*, addressed to future-me. Rationale
records exist for **every** artifact, including quick-win-eligible ones — the
default is "captured judgment," never "silent skip."

### `kind: "comment_event"`

An observed comment/close/reopen/label event on an issue. Emitted by the
delta-sweep step (E). Records maintainer engagement (the compass, B3) *and*
tracks arcaven/arcavenai's own activity for filed-vs-acted math.

```json
{"kind":"comment_event","ts":"2026-06-29T22:56:59Z","target":"vsdd-factory",
 "number":336,"event":"commented",
 "actor":"arcaven","actor_role":"review-identity",
 "body_len":312,"body_sha256":"<hex>",
 "observed_in_run":9}
```

`event` values: `commented` | `closed` | `reopened` | `labeled` | `unlabeled` |
`assigned` | `merged`. `actor_role`: `maintainer` | `measured-contributor` |
`review-identity` | `other` (derived from the intent manifest — this is why the
manifest is loaded before enumeration).

### `kind: "cluster"`

A cluster definition. Beadle's clusters are named groupings that persist across
runs; membership is derived from `classification.cluster` fields. This record
tracks *cluster-level* metadata (last-added-member run, description, decay
state) that isn't a property of any single issue.

```json
{"kind":"cluster","ts":"2026-07-01T00:00:00Z","target":"vsdd-factory",
 "name":"authority-substrate","run":9,
 "description":"identity assertion → key validation → audit-ledger chain",
 "members":[372,374,379],"last_added_run":9,
 "decay":"active"}
```

`decay`: `active` (a member was added this run) | `warming` (1-2 runs since
add) | `rollup-candidate` (≥3 runs since add — renderer folds into rollup row) |
`archived` (renderer omits from body; store keeps).

### `kind: "note"`

Free-form provenance/reasoning trace. Never dashboard-facing; here so a future
run can read "why beadle did X" without re-deriving it.

```json
{"kind":"note","ts":"2026-07-01T00:00:00Z","target":"vsdd-factory","run":9,
 "topic":"comment-bar-recalibration",
 "text":"Considered posting on #370 (verification job emits static PASS) — the specific class 'verification-that-verifies-nothing' would be useful to disambiguate. Withheld pending rubric extension (Phase 4 D)."}
```

## Materialization

**Latest state of issue N** = collapse `issue` + `classification` + `comment_event`
records for `number == N` in ts order; take the last of each field. Missing
fields inherit from the prior record for that number.

**Cluster membership at run R** = for each open issue, its classification record
at run R (or the most recent prior) with `cluster` array; union across issues.

**Digest** = `sha256(canonical-json(materialization at run R))` — deterministic,
compact, embeddable in the dashboard body's sentinel block.

## Backward compat

- New `kind` values must be added to this doc **and** be safe to ignore for
  older readers (readers filter by known `kind`).
- Existing fields must never change meaning. Rename = new field + populate both
  until every consumer is updated + drop old.
- Schema version is implicit in the file's location under `store/`. If a breaking
  change is truly needed, bump the target subdir: `vsdd-factory/state.jsonl` →
  `vsdd-factory/state.v2.jsonl` and write a migration.

## Not stored here

- Full issue bodies (they live on GitHub; we cache the sha to detect drift).
- The rendered dashboard body (a projection, not state).
- Anything the target repo's own artifacts already carry (`.factory/STATE.md`,
  decision log, etc. — beadle *reads* them, doesn't shadow them).
