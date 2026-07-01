---
id: finding-014
title: "Classifier ingest contract + direction signals B & C wired"
tags: [beadle, classifier, direction-verdict, phase-4-followon, ingest-contract]
date: 2026-07-01
provenance:
  created_by: agent
  probe: retrospective-followon
---

# What shipped

Two pieces of the Phase-0 classifier surface land as concrete Rust code:

1. **`beadle classify ingest <target> [--file <path>]`** — accepts a JSON
   payload (single object or array) matching the `ClassificationRecord`
   schema, validates the four bounded enums (`report_type`, `defect_nature`,
   `reproducibility`, `operational_impact`), enforces the two HARD invariants
   from `elem-defect-classification-superset` (`integrity=true` requires
   `integrity_anchor`; `quick_win_eligible=true` is invalid on integrity
   items), and appends the records to the target's store. Timestamps default
   to now; an explicit `ts` in the payload wins so the skill can replay
   historical classifications deterministically. Stdin is the default input
   channel; `--file` is a convenience for testing.

2. **`beadle direction <target>`** consumes classification records for the
   current run and computes signals B (integrity-density) and C
   (silent-data-loss share) — no longer hardcoded `pending`. When the store
   contains no classifications for the current run, both signals emit
   `pending` with a specific `reason` naming the gap; when classifications
   exist, they emit `on-course` / `watch` / `drifting` from the thresholds
   in `question-direction-verdict-escalation` (15%/30% for integrity;
   5%/10% for SDL). The `SignalOrPending` enum makes the JSON output
   honest: a pending signal is a live schema variant, not a placeholder
   string masquerading as data.

## What did NOT ship, and why

**No Rust classifier.** A keyword-matching heuristic classifier would
launder guesses as `ClassificationRecord` data and defeat the whole point
of the superset (finding-005 / finding-006 grounded classification is what
gives the compass and the quick-wins lane their teeth). Phase 0's classifier
is the Claude Code skill — `skills/beadle-triage/SKILL.md` — reasoning
from the intent manifest and the artifact body against the taxonomy. The
Rust binary owns the *contract* (validate + append + compute) so the
skill's output is machine-checkable and its downstream signals derive
from data, not prose.

## Why the propose-not-act invariant survives

`elem-propose-not-act` requires that pending signals be surfaced by name
with a `reason` field, never silently omitted. The `SignalOrPending::Pending`
variant serializes with `verdict: "pending"` + `reason: "<specific gap>"` —
the direction report cannot lie about whether B/C are grounded in data or
still waiting for the classifier to produce records for this run. When the
overall verdict is computed as `max`, pending signals contribute nothing
(they're treated as `on-course` for the max), so an unclassified run
cannot escalate on B or C by omission.

## End-to-end validation

Ran a scratch target with 1 seeded run (5 new issues) and 5 ingested
classifications (2 integrity, 1 silent-data-loss). Result:

- filing-density: `on-course` (single run, no history to derive against)
- integrity-density: `drifting` (2/5 = 40% > 30% threshold)
- silent-data-loss: `drifting` (1/5 = 20% > 10% threshold)
- overall verdict: `drifting`
- top_signal: `integrity-density drifting: 2/5 classified issues carry integrity=true → 40.0%`

The pending→live transition works. The `top_signal` selector picks the
first-hit live signal at the target verdict — filing was on-course, so
integrity wins the top slot.

Against the real vsdd-factory store (run 9, 14 new issues, 0
classifications), the report correctly shows filing on-course + both B/C
`pending` with `reason=no classification records for run 9`.

## Threshold calibration status

Same as finding-013: the numbers (15%/30% for integrity; 5%/10% for SDL)
are educated guesses from the vsdd-factory drift, not calibrated. A1
(calibration corpus in `question-direction-verdict-escalation`) requires
≥2 targets × ≥3 runs with real classifier output before the numbers stop
being placeholders. The thresholds live in `direction.rs` as named
constants so recalibration is a single-line change.

## What this unblocks

- **A4 (silent-data-loss zero-engagement alarm)** — needs comment-event
  data joined against classification records. `comment_event` rows already
  exist in the store (finding-011). The join is direction's next feature.
- **The dashboard's classification-index rows** — render.rs currently
  emits the store's issues without classification chips because there were
  no classifications to emit. Once the skill produces its first run of
  classifications, render.rs can pick them up.
- **critic's coupling** — the taxonomy is the shared boundary
  (`elem-defect-classification-superset`); the classifier's output is the
  substrate critic consumes for class-weighted defect-detection efficiency.
  No shared schema/library extraction yet (session-051 decision); the
  store's JSONL is the interchange format.

## Deliberately open

- **Second-order-verdict-changes-classifier-behavior feedback loop.**
  If the skill sees the previous run's verdict was `drifting`, does it
  classify the next batch more cautiously? The current design has NO such
  coupling — the skill classifies from the intent + body, not from prior
  verdicts. This is deliberate; Goodhart avoidance
  (`elem-no-goodhart`). Worth revisiting only if we see the skill's
  classification distribution drifting run-over-run with no artifact-level
  cause.

- **`run` scoping on classifications.** Today B/C look only at
  classifications where `run == latest_run`. A stronger signal might be
  "trailing-3-run integrity share" mirroring signal A's window (A3).
  Deferred to the same calibration pass as A3.
