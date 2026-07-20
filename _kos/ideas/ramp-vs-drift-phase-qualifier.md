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

## Pre-registration (2026-07-20, aae-orc#65 — fixed BEFORE run-15, blind to its data)

Per aae-orc#65 ask 2, the two definitions that decide the run-16 verdict are
fixed now, while we cannot see run-15/16 engagement. Changing them after run-15
posts voids the test.

**(a) Severity ordering for the ceiling:** `P2 < P1 < P0b < P0a`. An item's
rung is its **assigned priority at the time the action is observed** (re-grades
get noted in the run summary, they don't retroactively move past ceilings).
Liveness classes (`halt`/`panic`) carry **no separate rung** — they are loud,
self-announcing failures and already land at the priority they were graded
(typically P0b); the top rung stays reserved for silent integrity (SDL/P0a),
which is the class A4 guards. Consequence: **"past P0b" ⇔ an acted-on P0a/SDL
item ⇔ A4 discharge.** The prediction, the ordering, and A4 cohere by
construction.

**(b) "Acted-on" (two meters, no fractional credit):**

- **Hard ceiling** — moved only by: maintainer merges a fix, closes with a
  stated resolution, or lands a commit addressing the item. This is the meter
  the run-16 prediction is evaluated against.
- **Soft ceiling** — substantive maintainer comment (reproduction attempt,
  explicit triage decision, fix direction, question-to-reporter). Reported
  alongside every run; **never moves the hard ceiling**.
- **Self-filed exclusion:** action on an item the maintainer filed themselves
  moves neither meter — the ceiling measures engagement with the measured
  backlog, not the maintainer's own filings (run-14 edge that forced this:
  Zious11's own SDL filing #635 shows awareness, not backlog engagement).

Baseline at declaration: hard ceiling = **P0b** (close of #465, run-14).
Prediction: hard ceiling past P0b by run-16, else ramping stops explaining
the data. Discharge mechanism (ask 1): store record `kind: note,
topic: prediction` + `dashboard-refresh.md` step-4 checklist (run ≥ 16
evaluation line + every-run meter line, ask 4).

## Sub-questions (for the eventual frontier node)

1. ~~Severity ordering for the ceiling~~ — **pre-registered above** (a).
2. ~~What counts as "acted-on"~~ — **pre-registered above** (b).
3. N for "flat ceiling" before ramp → drift flips (relate to A4's own ≥3-run
   threshold; avoid double-counting the same silence).
4. ~~Verdict value or phase qualifier?~~ — **answered by aae-orc#65 ask 3:
   qualifier**, the ADR-005 cold-start-annotation shape. Two reasons adopted:
   no-Goodhart says annotate the alarm, never soften it (A4 stays loud with
   the qualifier explaining it); and ramping and drifting can coexist on
   orthogonal axes (e.g. scope), which a single enum value cannot express.
5. Does the phase model generalize to other engagement-distribution claims
   (filed-vs-acted gap, quick-wins conversion rate)?

## Provenance note (for the orc graph)

This correction came from the operator mid-conversation, not from the graph or
the instrument — the third documented instance of the F25/F19 pattern (every
prop existed: B8, §7a's on-ramp rationale, ADR-005; no backdrop connected them
into "engagement has phases"). Cross-ref orc charter F25/F19 when harvesting.
