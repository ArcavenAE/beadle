# finding-003 — maintenance-request controls (the cheap-poll-then-act tier)

Date: 2026-06-28
Probe: user request for a new development target — board-level controls that let
maintainers request expensive routines on demand, discovered via a cheap poll.
Nodes touched: new `question-maintenance-request-controls` (frontier), extends F5.

## What the user asked for

> "a control that when selected / a checkbox or series of checkboxes requests
> various actions more quickly. then we can poll this and act on demand, such as if
> the maintainers want us to reprioritize the list, perform a full refresh or
> re-evalidation or some like maintenance tasks ... it can allow us to run quick
> checks to see if any human has asked for more costly routines. I think we have the
> intent already on 'Controls' and they just are not wired up yet."

Correct read: the intent IS already in F5 / dashboard-schema §6, but only as a
**per-issue** verb surface (`fast-track`/`investigate`/`accept-deferral`, keyed to
one `id=#NN`). The new target is a **second tier** and a **polling discipline**.

## The two tiers

- **Tier 1 (existing, F5):** per-issue verbs. "Do X to issue #NN." Bounded.
- **Tier 2 (this finding):** board-level maintenance requests. "Run an expensive
  routine over the whole target." `reprioritize`, `full-refresh`, `revalidate`,
  `rescore-intent` (the set is deliberately open). No `id`, or `id=board`.

## The cost asymmetry it solves

Tier-2 routines are exactly the ones a maintainer occasionally wants on demand but
beadle must NOT run every scheduled tick. Split the work:

- **Poll pass (cheap, frequent):** fetch only the dashboard body, parse Tier-2 `[x]`
  boxes, decide if any human requested a costly routine. One issue read, no
  enumeration, no scoring. Safe on a tight cadence.
- **Act pass (expensive, on-demand):** only if the poll found a checked request, run
  it, then reset the box.

A request queue rendered as checkboxes: humans express demand cheaply, beadle
discovers demand cheaply, costly work happens only when asked. The on-demand
complement to the scheduled cadence.

## What was wired up now (Phase 0, obvious + safe)

- `skills/beadle-triage/SKILL.md` step 8 — split into Tier 1 / Tier 2; Tier 2
  dispatches whole-corpus routines and resets the box (de-bounce); read-only/regen
  only, irreversible public actions still escalate per B2; cheap-poll pattern noted
  as Phase-1 schedule work.
- `docs/dashboard-schema.md` §6 — Controls now documents both tiers with rendered
  Tier-2 examples + the cheap-poll pass.
- `_kos/nodes/frontier/question-maintenance-request-controls.yaml` — the open design
  questions (which routines are safe, poll-vs-act cadence, de-bounce/idempotency,
  cost-guard/abuse, the INC-003 concurrency chokepoint shared with discovery).

## What stays frontier

The cheap-poll *cadence* itself (a separate frequent poll schedule) is Phase-1 gh-aw
cron work, resolved alongside the discovery chokepoint and the gh-aw schedule design.
Phase 0 is single-session, so rendering the Tier-2 boxes is enough today.

## Charter delta (kos harvest — update the nodes, then the charter projection)

F5 (checkbox control surface) should record the **two-tier** model: per-issue verbs
(existing) + board-level maintenance-request controls with a cheap-poll-then-act
discipline. The safe-routine boundary (read/analyze/regenerate yes; irreversible
public action no) is the B2-derived constraint that keeps Tier 2 inside
propose-not-act.
