---
id: finding-018-board-control-checkboxes
type: finding
title: "F5 Tier-2 board-control checkboxes — renderer emits, push records, dispatch deferred"
tags: [beadle, dashboard, controls, tier-2, propose-not-act, phase-0]
provenance:
  created_by: agent
  created_at: "2026-07-01"
  answers:
    - question-maintenance-request-controls
edges:
  - target: elem-state-out-of-band
    type: derives_from
  - target: elem-propose-not-act
    type: bounded_by
---

## Context

Charter frontier F5 (`question-maintenance-request-controls`) splits the
dashboard checkbox surface into two tiers:

- **Tier 1** — per-issue verbs (`id=#NN`): `fast-track` / `investigate` /
  `accept-deferral`.
- **Tier 2** — board-level maintenance requests (`id=board`):
  `reprioritize` / `full-refresh` / `revalidate` / `rescore-intent`.

Motivating asymmetry: the maintainer wants those routines on demand, not
on every tick. A cheap-poll pass parses the dashboard body for checked
boxes; an expensive act pass runs the matching routine. This finding
covers the Phase-0 half of Tier 2 — record demand, defer dispatch.

## What shipped

- **`crates/beadle/src/controls.rs`** (new). Canonical vocabulary of four
  Tier-2 verbs + `render_board_controls_block()` + `extract_checked_verbs()`.
  Renderer always emits `- [ ]` (never `[x]`); extractor scans for `- [x]`
  or `- [X]` lines whose HTML-comment marker carries `verb=<v>` matching
  `BOARD_VERBS`. Extraction dedupes across the body and returns canonical
  order so a double-check queues the routine once.
- **`crates/beadle/src/render.rs`**. `render_derived()` calls
  `controls::render_board_controls_block()` between the Classification
  summary and the Open-issues detail. The block sits inside the derived
  zone so `derived_digest` covers it — hand-edits to the checkbox list
  are first-class detectable events on the next run.
- **`crates/beadle/src/push.rs`**. `push::run()` calls
  `record_board_controls()` BEFORE `render::run()`, on the LIVE body just
  fetched from GitHub. If any checkbox is checked, one `NoteRecord` per
  verb lands in the store with `topic="control-request"` and
  `text="verb=<v>;id=board"`. The fresh render then emits unchecked boxes;
  a subsequent `beadle push` overwrites the live body with the reset —
  that IS the de-bounce (concern #4 in the question node).
- **Tests: +11 (39 → 50).** Renderer emits every verb unchecked and
  never `[x]`; extractor handles `[x]`/`[X]`, canonical ordering, dedup,
  unknown verbs, Tier-1 mis-matches; push writes notes, no-ops on
  unchecked bodies, dry-run does not write.

## Which sub-questions this answers

The frontier node poses several design questions; this finding answers
the ones that are resolvable at Phase 0:

1. **Safety scope.** ✅ Every Tier-2 verb we shipped is read/analyze/
   regenerate only. None irreversibly touches GitHub. Anything that
   would (label sweeps, cross-issue re-scoring against maintainer
   comments) is left to Tier 1 escalation per B2. Concretely: the
   extractor's whitelist IS `BOARD_VERBS`; a maintainer typing
   `verb=close-all` produces zero notes.

2. **Poll vs act cadence.** ✅ Split. Poll (record) is cheap and happens
   on every `beadle push`. Act (dispatch) is deferred: the notes are a
   request queue that Phase-1 gh-aw will drain. The two cadences are
   independent by construction.

3. **Eventually-consistent semantics.** ✅ The renderer's zone always
   emits unchecked boxes. That is the "will get to it soon" hint — the
   next push through the same body publishes the fresh unchecked
   render, which visually confirms the request landed. If Phase-1
   dispatch is slow, the box comes back unchecked but the outcome
   hasn't yet appeared elsewhere on the dashboard; the maintainer can
   re-check.

4. **De-bounce.** ✅ Reset-on-render, structurally. There is no way to
   accidentally re-run: `push` records the request THEN calls `render`,
   and render's board-controls block is always unchecked. Re-checking
   the box on the next tick is an explicit signal.

## Which sub-questions remain open

- **Concurrency chokepoint.** Two `beadle push` invocations within the
  same interval both parse the checked box and both record — the
  request duplicates. Same shape as INC-003 (cross-artifact identity)
  and F7 (dashboard discovery). Phase 1 needs a serialization primitive
  when gh-aw drains the queue; today the Phase-0 single-session
  invariant means at most one push happens at a time (see
  `question-shared-state-timing`).
- **Cost guard.** Nothing yet limits how often the same verb can be
  requested. Phase 1 dispatch should coalesce identical `control-request`
  notes over some window before firing the expensive routine.
- **Tier 1 verbs.** Per-issue `fast-track` / `investigate` /
  `accept-deferral` are not yet rendered anywhere. They're a separate
  cut — they need per-issue rows to sit against, and the "Open issues"
  detail list is currently a bulk table, not a per-issue-owned row set.

## Ownership divider (per finding-017)

The board-controls block sits inside the renderer's whole-section
ownership zone. Compare against finding-017: the Direction verdict has
two whole sections — renderer owns the numbers, editor owns the prose.
Here there is no editor prose: the maintainer's authored artifact IS
the check, and the check is legible to code. So the whole block is
renderer-owned. The maintainer's "edit" (checking a box) is a
regen-detected event, not a paragraph that must survive verbatim.

That means `derived_digest` on the next run will differ from the last
run's `derived_digest` because the body-as-observed-just-before-render
had a checked box while the freshly-rendered body has an unchecked one
— exactly the first-class event we want.

## Not this cut

- Actual dispatch of `full-refresh` / `revalidate` / `rescore-intent` /
  `reprioritize` — Phase-1 gh-aw cron work.
- Tier 1 per-issue checkbox surface (`id=#NN`).
- A `beadle controls list` CLI to inspect the pending queue.
- Coalescing / cost guards.
- Escalation shape for irreversible verbs (whitelist stays code-side,
  so this is not urgent).
