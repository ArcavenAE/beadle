# ADR-005: No rate/drift verdict before the process has turned (cold-start)

## Status
Accepted

## Date
2026-06-27

## Context
The Phase-0 dry run against drbothen/vsdd-factory (2026-06-27) found 94/107 open
issues from one automated filer, 0 maintainer comments on them, and 1 lifetime
close. The first-draft dashboard rendered a "DRIFTING" verdict from those numbers.
That is wrong: the maintainers are busy-but-willing and have not yet begun their
first triage pass. Computing an average rate (resolution rate, engagement rate,
net-flow) over a window *before the process engaged* is **initialization bias**
(the warm-up transient in queueing/simulation; the cold-start problem in ML) — not
drift. Presenting it as drift indicts busy people for a denominator covering time
before they engaged, and is itself a Goodhart-style proxy abuse (ADR-002 forbids
optimizing/abusing proxy metrics).

## Decision Drivers
- A young backlog is pre-turn, not drifting.
- The metrics are "not meaningful *yet*," not meaningless — beadle keeps measuring.
- The dashboard must help, not indict, during cold start.

## Considered Options
1. Report rate/drift verdicts immediately (the first-draft behavior).
2. Suppress rate metrics entirely until steady state.
3. Gate the *verdict* on warm-up state: show baseline counts always; withhold
   rate/drift *interpretation* until the process has demonstrably turned.

## Decision Outcome
**Chosen:** option 3.
- Until a baseline exists (≥1 completed maintainer triage cycle, or a configured
  warm-up threshold), the direction verdict is **COLD START / BASELINE**, never
  DRIFTING.
- In cold-start the dashboard's job is to *establish the baseline* and make the
  maintainers' first pass cheap (surface high-leverage items + cluster structure),
  not to compute averages.
- Each rate metric is tagged with whether the system has reached steady state.
- Rate/trend/drift reporting (and the B3 maintainer-compass drift logic) begins
  only after the transient ends.

### Positive Consequences
- Honest first dashboards; busy maintainers aren't indicted by a cold-start metric.
- Cleanly separates cold-start from genuine steady-state drift.

### Negative Consequences
- Requires a defined warm-up threshold / "process has turned" detector (frontier).
- The most striking number (1% resolution) is deliberately *not* the headline early
  — by design.
