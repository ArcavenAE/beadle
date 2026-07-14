# finding-020 — run-13: the discipline became a command, and the binary's validator drifted from the spec

**Status:** confirmed (run-13 executed end-to-end, 2026-07-13)
**Scope:** beadle Phase-0 — `/dashboard-refresh` command + `beadle classify ingest`
**Follows:** finding-019 (analysis quality lives in the discipline, not the plumbing)

## What happened

The run-10/run-11 workflow (backup → skills-based refresh → section-regression
gate → post-or-restore) was distilled from the session prompts that produced it
into a project command, `.claude/commands/dashboard-refresh.md`, and executed
for the first time as run-13 against drbothen/vsdd-factory#312:

- **Backup:** pre-refresh snapshot verified byte-identical to the committed
  run-12 fixture before anything else ran.
- **Classify:** 104 new issues (#516–#631) graded by 8 parallel subagents
  (13/batch), each given the intent manifest + the finding-005/009 rubric +
  attn facet. 104/104 accounted for.
- **Gate:** machine-checked section-regression verification against BOTH the
  before-snapshot and the fixture canon — zero fails, zero warns on the second
  pass (the first pass caught 67 dropped drill-in issue references in my own
  fold-downs; the gate exists precisely to catch the composer).
- **Post:** 57,754 bytes (65,536 cap), run 13 / watermark 631, round-tripped
  byte-clean. Fixtures committed (beadle PR #31).

The binary's `render`/`push` path was not used at any point (finding-019).

## F1: the gate catches the curator, not just the binary

The run-12→run-13 compression work (folding run-11's index, abbreviating prior
tables to protect the body budget) silently dropped 67 `#NN` drill-in
references on the first composition pass. The regression gate's
issue-reference-coverage check caught it; the enumerations were restored as
compact number strings. Lesson: **body-budget compression is itself a
regression vector** — the gate must diff issue-reference coverage, not just
section headings, and it must run against the before-snapshot, not only the
canon.

## F2: `classify ingest` operational_impact enum contradicts finding-009 (the attest-vs-verify class inside beadle itself)

The binary validates `operational_impact` against a severity-flavored enum
(`silent-data-loss|silent-corruption|false-verdict|user-visible-error|
degraded-performance|none`) while SKILL §3 / finding-009 define it as the
LIVENESS axis (`panic|halt|data_loss|degraded|none`), deliberately orthogonal
to the safety cluster the record already carries in `integrity`/
`integrity_anchor`/`silent_data_loss`. Run-13 ingested with a lossy mapping;
the corpus's first three `impact.panic` items (#541/#548/#569) are
indistinguishable from halts in the store, and the direction-signal machinery
(finding-014/016) reads the store. Ground truth preserved in
`docs/fixtures/vsdd-factory-312-run13-classifications-rich.json`.

This is a self-referential instance of the class the dashboard tracks as
attest-vs-verify: **the validator's vocabulary diverged from the spec's, and
validation green-lit the divergence.** Same shape as aae-orc's finding-074
(kos loaders silently skipping nodes whose vocabulary drifted): the checker
and the spec need a contract test or they part ways silently.

Tracked: ArcavenAE/beadle#32 (authoritative defect record) +
ArcavenAE/aae-orc#58 (bd-request work-queue anchor).

## F3: operational patterns worth keeping

- **Grader-stall recovery:** 2 of 8 subagents stalled ~1 hour (same failure
  mode as the 2026-07-05 session's Bedrock 429/503 kills). A resume ping
  recovered both; no batch was re-run from scratch. Rule already encoded in
  the command: retried/resumed, never silently dropped — every issue number
  must appear.
- **Curator overrides grader on lane admission:** three grader-eligible quick
  wins were excluded by rule-judgment (#523 SDL — the highest-severity class
  never rides the safe lane even when the fix is mechanically bounded;
  #541/#548 panic — checkpoint/resume architecture is not a one-liner). The
  exclusion audit lives in the body's agent channel. Candidate SKILL §7a
  amendment: SDL and panic are lane-excluded regardless of mechanical
  eligibility.

## Open

- beadle#32 fix + store migration (runs 10+13 records) + SKILL↔binary enum
  contract test.
- Maintainer first actions landed this run (3 Zious11 comments + 9 filings) —
  next run tests whether engagement converts to a close/label (the ADR-005
  turn) and whether the A4 clock on #342/#365/#358 breaks.
