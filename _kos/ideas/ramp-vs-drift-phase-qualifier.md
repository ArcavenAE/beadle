# Ramp vs drift — the direction model needs an engagement-calibration phase

**Status:** idea (pre-hypothesis) · candidate frontier question
**Born:** 2026-07-20, run-14 post-publication editorial correction (operator)
**Feeds:** direction-signal model (finding-013/014/016/017), B8 cold-start, SKILL §7a/§7b

## The gap

The direction verdict enum (on-course / watch / drifting) has no value for a
maintainer who is **engaged but deliberately calibrating**. Run-14's data: 33
maintainer actions, every one a bounded, cheaply-verifiable, recoverable change
(8 merges of prompt/template/path fixes, 3 closes, a correction cycle #691, field-data
comments), while the SDL lane sat untouched at a 5-run zero-engagement streak.
A4 correctly measured the streak; the machine verdict fell through to `drifting`
for want of a `ramping` value — reading a confidence-building sequence as
misalignment.

The operator's correction (verbatim intent): the maintainer is *getting his
machine running* — developing confidence on low-stakes items that acting on this
corpus won't take the project in unintended directions, **before** touching the
serious issues. Sampling cheap items first is how a careful maintainer estimates
the quality of a 427-issue agent-filed backlog. Expecting severity-ordered
engagement was the model's error, not the maintainer's.

## The internal inconsistency (motivating instance)

**beadle's quick-wins lane exists to invite exactly this behavior** (SKILL §7a:
"the cheapest first maintainer pass", "exercise their process") — and then the
direction machinery scored the invited behavior's natural consequence as
`drifting`. The lane and the verdict disagree about what good early engagement
looks like. The fix is phase-awareness in the verdict, not a different invitation.

## The discriminator (what separates the two worlds)

Ramp and drift produce **identical single-run distributions**. They differ in
trajectory:

- **Ramp:** the *acted-on severity ceiling* climbs monotonically as trust
  accrues (run-13 → run-14 it already did: nothing → P0b close #465).
- **Drift:** the ceiling stays flat/falls; the small-and-easy selection function
  is stable, not transitional.

Candidate signal: track `max(severity of maintainer-acted items)` per run.
Verdict logic sketch: ceiling climbing → `ramping` (soften A4's contribution to
top_signal; keep the clock visible as the ramp's falsification meter); ceiling
flat ≥ N runs with A4 firing → `drifting` earns its name. Run-14's board now
carries this framing in the editor slot with a declared prediction: if the
ceiling has not moved past P0b by run-16, ramping stops explaining the data.

## Sub-questions (for the eventual frontier node)

1. Severity ordering for the ceiling: P2 < P1 < P0b < P0a(SDL)? Where do
   halts/panic sit vs integrity?
2. What counts as "acted-on" for the ceiling — close-with-fix only, or do
   partial fixes and substantive comments raise it fractionally?
3. N for "flat ceiling" before ramp → drift flips (relate to A4's own ≥3-run
   threshold; avoid double-counting the same silence).
4. Should `ramping` be a verdict value or a phase *qualifier* on any verdict
   (analogous to ADR-005 cold-start annotating rates)?
5. Does the phase model generalize to other engagement-distribution claims
   (filed-vs-acted gap, quick-wins conversion rate)?

## Provenance note (for the orc graph)

This correction came from the operator mid-conversation, not from the graph or
the instrument — the third documented instance of the F25/F19 pattern (every
prop existed: B8, §7a's on-ramp rationale, ADR-005; no backdrop connected them
into "engagement has phases"). Cross-ref orc charter F25/F19 when harvesting.
