# finding-013 — direction-verdict escalation, first cut (item F)

Date: 2026-07-01
Probe: item F from the Phase-4 retrospective — commit to concrete
escalation thresholds for the direction verdict (`on-course` / `watch` /
`drifting`) and ship what the store can compute today. Frontier
question: `question-direction-verdict-escalation`.
Nodes touched: new frontier `question-direction-verdict-escalation`;
`beadle direction <target>` subcommand.

## The three signals

The frontier question fixes three named signals:

- **A. Filing-density derivative.** Rate of new issues per run.
  Computable *today* from `run.new_this_run.len()` and the ordering of
  run records — no classification data required.
- **B. Integrity-density derivative.** Share of new issues carrying an
  integrity-class flag. Requires classification records.
- **C. Silent-data-loss share.** Silent-integrity issues (finding-004
  class) as fraction of open. Requires classification records.

The verdict is the *max* of the three (any one drifting → drifting;
any one watch → watch; else on-course). Rationale for max-not-vote:
drift on any axis is drift; a weighted vote would let volume drown
severity, which is exactly the compass-vs-cargo confusion
`elem-maintainer-compass` was built to prevent.

## Thresholds (frontier — recalibrate at ≥ 2 targets × ≥ 3 runs)

### A. Filing-density derivative

Reference: trailing-3-run mean.

- `on-course` : rise ≤ 25% vs trailing mean AND current ≤ 2× trailing mean
- `watch`     : rise 25–50% OR current ≥ 2× trailing mean
- `drifting`  : rise ≥ 50% vs trailing mean

Trailing-3 was picked over time-based windows for one honest reason:
Phase 0 has one target and 9 runs total. Time-based windows can't
distinguish "no runs happened" from "no new issues" until we have
observation cadence for at least two targets. Sub-question A3 in the
frontier node names this.

### B. Integrity-density derivative

Share of *new-this-run* issues classified integrity=true.

- `on-course` : integrity share ≤ 15% of new
- `watch`     : 15–30%
- `drifting`  : > 30% for any single run

### C. Silent-data-loss share

Share of *open* issues classified silent-integrity (finding-004 class).

- `on-course` : share < 5% of open
- `watch`     : 5–10%
- `drifting`  : > 10% OR any new silent-data-loss issue with zero
                maintainer engagement across 3+ runs

The zero-engagement alarm is the harshest clause and the one item F
should be *most* honest about: it fires the `drifting` verdict on
precisely the case beadle exists to catch. Sub-question A4 names the
escape hatch — "pending response" vs "absent" — that lives inside
finding-004.

## What shipped today

- New subcommand: `beadle direction <target> [--write-note]`.
  Computes signal A, emits a JSON `DirectionReport` on stdout, and
  optionally appends a `note` record with `topic=direction-verdict`
  to the store as an audit trail. Signals B and C are surfaced by
  name with `verdict=pending` and a `reason` string —
  `classification records not yet in store`.
- Six unit tests on `filing_density_signal` covering the corner
  cases named in the thresholds (flat, +25%, +50%, absolute
  multiplier, single-run, empty).
- Live emit against `drbothen/vsdd-factory` at run 9:
  `verdict=on-course, top_signal="single run (14 new); no history to
  derive against"`. Honest — we have one data point, so there is no
  filing-density derivative to compute yet. B and C are surfaced as
  pending, not silently omitted.

## Why the propose-not-act shape holds

The pending-signals design is item-F's answer to the trap `finding-004`
warned about. If the classifier isn't running yet, the command must not
*infer* B or C from prox signals ("lots of `integrity:` labels" ≠
classification-flag=true) and must not omit them (silent omission
reads as "I checked, all clear"). Naming them as `pending` with a
`reason` field discloses exactly what the automation cannot see.

The written note record (only when `--write-note` is passed) makes the
verdict-source-of-truth part of the store per
`elem-state-out-of-band`. Editor slot still owns the free-text
paragraph per `question-renderer-editorial-boundary`. Sub-question A5
in the frontier node named this split explicitly.

## What stays open (deferred to frontier)

- **Signal B/C wiring**: needs classifier records to exist. Every
  target currently has zero. First calibration pass unblocks when
  the classifier writes its first `classification` row.
- **Threshold calibration**: the thresholds are educated guesses; sub-
  question A1 names the "≥ 2 targets × ≥ 3 runs" bar.
- **Rendering the verdict**: `beadle render` today ships the editor
  slot for direction verdict empty. Item F stops short of injecting
  the computed verdict into the rendered body; that requires
  question-renderer-editorial-boundary's C sub-question (nudging vs
  overwriting the editor's paragraph) to be answered first.
- **The `arcavenai` self-annotation pattern (finding-012)**: could
  become a fourth direction signal (`filer-loop rise`). Deferred
  until a second target reproduces the pattern.
