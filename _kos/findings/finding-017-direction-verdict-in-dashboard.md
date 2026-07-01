---
id: finding-017
title: "Direction verdict + signal table land in the derived zone of the dashboard body"
tags: [beadle, direction-verdict, dashboard, renderer, editorial-boundary, phase-4-followon]
date: 2026-07-01
provenance:
  created_by: agent
  probe: retrospective-followon
---

# What shipped

`beadle render <target>` now emits a **Direction verdict** section at the top
of the derived zone. Every regen writes:

- A header line: `**<glyph> <verdict>** — run <N> — top signal: <text>`
  where the glyph maps `on-course → 🟢`, `watch → 🟡`, `drifting → 🔴`.
- A 4-row table with per-signal verdict + rationale:
  `filing-density`, `integrity-density (B)`, `silent-data-loss-share (C)`,
  `silent-data-loss-zero-engagement (A4)`.
- For A4 specifically: when the signal is live and non-empty, the table's
  detail cell appends `· drifting: #42, #87` (or `· watch: #12`) so the
  affected issue numbers are visible without expanding a details block.

The block is emitted before the existing "Baseline (derived from store)"
section. `render_derived` — the function whose output feeds the sentinel's
`derived_digest` — now takes the `DirectionReport` as a parameter, so any
change in the verdict / top-signal / signal detail changes the digest.
Hand-editing the verdict block is now a first-class detectable event, same
as any other derived-zone change (invariant B1 / question-renderer-
editorial-boundary sub-question B).

## Answering question-renderer-editorial-boundary sub-question A

Sub-question A asked which of three divider formats to use. This finding
picks and demonstrates option 2: **whole sections owned by the renderer
sit next to whole sections owned by the editor.** The verdict block is a
whole renderer-owned section (numbers, glyphs, table, deterministic
projection of `DirectionReport`); the `<!-- editor:direction-verdict -->`
slot immediately above it is a whole editor-owned section (the free-text
paragraph explaining the numbers). They do not interleave — the editor
never edits *inside* the table; the renderer never rewrites *inside* the
editor slot.

The other two options were rejected:

- **Marked slots inside the derived zone (option 1).** Would let the editor
  hand-author a "verdict cell" while the renderer authored the rest of the
  same table. This inverts B1 — machine-derived numbers next to
  human-authored numbers, indistinguishable by position. Any editor edit
  desyncs `derived_digest` without a clean way to say "this cell is not
  under renderer authorship."
- **Editor overlay file (option 3).** Adds a second source of truth
  (`store/<target>/editorial.md`) whose merge semantics would need to be
  designed and enforced. The `<!-- editor:... -->` inline slot in the body
  already gives us round-tripping across regens (finding-011); adding a
  second file for the same purpose is premature elaboration (SOUL.md §7).

Sub-questions B (derived_digest tracks it — done, the block flows through
the existing digest), C (editor's paragraph survives regen — done, the
inline slot is preserved verbatim), D (empty slot on run 1 — the existing
placeholder text handles this), and E (unclassified surfacing — answered
by finding-015) are already addressed by the composed shape.

## Rationale — why the renderer owns the numbers

The direction verdict is a *deterministic projection* of the store:
`DirectionReport = compute(target, records)`. The signal thresholds,
top-signal precedence, and pending semantics are code, not editorial
judgment (finding-013 / finding-014 / finding-016). If a human wrote the
"drifting" verdict when the store said "watch," we would be laundering a
guess as data — the exact failure the propose-not-act invariant (B2)
exists to prevent.

The editor still owns the *paragraph* that explains what to do about the
numbers. That is the domain where B2 says "propose, don't act" — the
human decides which drifting issue matters most this week, which one is
pending-response versus abandoned, whether the maintainer needs a nudge.
The verdict is what; the paragraph is why + so-what.

## Top-signal wiring

`top_signal()`'s A4-first precedence (finding-016) matters most in the
rendered header line. When A4 fires drifting alongside SDL-share, the
header now says "silent-data-loss-zero-engagement drifting: 1 SDL
issue(s) with ≥ 3-run silence + zero maintainer engagement" rather than
the coarser SDL-share message. An editor scanning the dashboard sees
*which class of issue* is drifting on the first row of the table — the
A4 row's detail cell names #42 explicitly.

## Refactor — `direction::compute` extracted from `direction::run`

To wire direction into render without stdout side-effects, `direction.rs`
now exposes `pub fn compute(target: &str, records: &[Record]) -> DirectionReport`.
The existing `direction::run(root, target, write_note)` calls `compute()`
under the hood and handles the CLI-side stdout/note-append. This is the
same shape as the existing `render::run` / `render_derived` split — a pure
computation function usable from anywhere, plus a CLI-shaped `run()` that
wraps it with I/O.

Nothing observable about `beadle direction <target>` changed. The
existing 9 direction unit tests still pass; the pure `compute()` is what
they always exercised.

## End-to-end validation

Fixture 1 — `/tmp/beadle-scratch-a4` (A4 fixture with maintainer comment):

- Header: `**🔴 drifting** — run 3 — top signal: silent-data-loss
  drifting: 1/1 classified issues flagged silent-data-loss → 100.0%`
- A4 row: `🟢 on-course · 1 SDL issue(s) tracked; none in a zero-engagement
  streak of ≥ 2 runs`
- SDL-share row: `🔴 drifting · 1/1 classified issues flagged
  silent-data-loss → 100.0%`

Escape hatch honored: A4 stayed on-course because the maintainer comment
dropped #42 from the alarm set, so SDL-share drove the overall verdict.

Fixture 2 — `/tmp/beadle-scratch-a4-nomaint` (same store minus the
maintainer comment):

- Header: `**🔴 drifting** — run 3 — top signal:
  silent-data-loss-zero-engagement drifting: 1 SDL issue(s) with ≥ 3-run
  silence + zero maintainer engagement (longest streak: 3 runs)`
- A4 row: `🔴 drifting · 1 SDL issue(s) with ≥ 3-run silence + zero
  maintainer engagement (longest streak: 3 runs) · drifting: #42`

Top-signal precedence works: with no maintainer comment, A4 fires drifting
and displaces SDL-share as the top signal because A4 names *which issue*
(#42) rather than just how many.

## What did NOT ship, and why

- **No renderer-authored paragraph attempt.** The paragraph *below* the
  table is still the editor slot with placeholder text. Attempting to
  auto-author it would launder judgment as data (per rationale above).
  When A4 fires drifting the numbers already carry the specific-issue
  reference; the paragraph's job is to add editorial *interpretation*
  the numbers cannot.
- **No cross-run trend visualization.** "This is run 3; last run was
  watch; two runs ago was on-course" is a legitimate editorial move, but
  it requires either replaying `compute()` against historical run slices
  (cheap, all data is in the store) or storing verdict history as
  `note` records (requires a schema decision). Deferred until a
  calibrated target has ≥ 5 runs and the trend line has real signal.
- **No `derived_digest` sub-partition.** The verdict block is inside the
  single derived zone. A future evolution could partition
  `derived_digest_verdict` / `derived_digest_baseline` / etc. for finer
  drift detection — deferred until we see hand-edits that would benefit
  from the finer granularity.

## Tests

`crates/beadle/src/render.rs::tests` gains:

- `render_includes_direction_verdict_block` — asserts the header, glyph,
  and all four signal rows are present when all signals are pending.
- `render_direction_drifting_names_issues` — asserts the drifting glyph,
  the `drifting: #42` detail, and the A4 top-signal are surfaced.
- `mk_direction_pending()` helper — the standard "no history" report shape
  used by every render test that doesn't otherwise care about direction.

Existing tests `render_includes_classification_zones` and
`render_pending_when_no_classifications` gained a `&mk_direction_pending()`
argument to match `render_derived`'s new 9-argument signature.

Total test count on `beadle`: 39 (was 37 after finding-016). Full
`cargo test` green.

## Cross-cutting

- Answers `question-renderer-editorial-boundary` sub-question A
  (whole-section ownership, option 2).
- Composes with finding-013 / finding-014 / finding-016 (the three
  direction signals) — this is the *rendering* half of what those
  findings computed.
- Preserves invariants B1 (state out-of-band; body is a projection),
  B2 (propose-not-act; renderer owns numbers, editor owns interpretation),
  and G1 (tolerate wiped body — regen produces a fresh verdict block).

## Files

- `crates/beadle/src/direction.rs` — extracted `pub fn compute()` from
  `run()`. No observable behavior change to the direction CLI command.
- `crates/beadle/src/render.rs` — new `render_direction_block()` helper,
  new `verdict_glyph()` helper, `render_derived()` takes
  `&DirectionReport` (now 9 args), `render::run()` calls
  `direction::compute()`. Two new tests + `mk_direction_pending()`
  helper.
