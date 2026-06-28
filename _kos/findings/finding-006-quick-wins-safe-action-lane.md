# finding-006 — the quick-wins safe-action lane (reward the safe, not only the critical)

Date: 2026-06-28
Probe: user request during run-3/4 review — "we should also reward with safe action:
low hanging fruit. Not all work should be impact/critical first ... a group like this
for our maintainers, something they can test their processes on, not approach with as
much caution. the obviously broken, easy fix issues."
Nodes touched: `classify`/render model (skill step 7a + dashboard-schema §3b); new
bedrock `elem-dashboard-render-model`. Composes finding-004 (integrity exclusion) and
finding-005 (the axes the lane is derived from).

## What the user asked for

A dashboard group that is **orthogonal to the impact ordering** — not a fourth priority
tier below P2. The action plan ranks by impact (integrity → convergence → gate → cost);
this lane cross-cuts it to surface the *obviously-broken, easy-fix, low-blast-radius*
issues a maintainer can act on **without** the caution the high-impact items demand. The
motivating principle: reward the safe + obvious, give maintainers a low-risk surface to
**exercise their process** on. It is also the cheapest possible first maintainer pass.

## The design — eligibility is DERIVED, never a hand label

The lane is not a curated "good first issue" tag (which drifts and gets stale). It is
**computed from the axes already scored** in finding-005:

- **defect-nature** at the mechanical end (typo / wrong-variable / docs / one-line
  config / a bounded agent-prompt warning) — exclude design / spec / directional;
- **reproducibility** Bohrbug (consistent, isolatable) or a pure docs/wording change;
- **bounded blast radius** + **high fix-confidence** (the fix is cited and obvious);
- **alignment ≠ drifts** (never fast-track something that shouldn't be built at all).

This is the inverse selection of the P0 integrity cluster: P0 collects high-blast-radius,
invisible, compounding faults; the quick-wins lane collects low-blast-radius, visible,
self-contained ones. Same axes, opposite ends.

## The two guards (why this is safe, not a Goodhart trap)

1. **Integrity hard-exclusion (finding-004) — non-negotiable.** An issue touching a
   system-of-record `integrity_anchor` is **never** a quick win, *no matter how small the
   triggering bug looks.* finding-004 is explicit: "a green check or a one-line diff must
   never mask" a silent-integrity defect. The easy lane is *precisely that trap* — the
   place a one-line diff with catastrophic invisible blast radius would hide. So the lane
   checks each candidate against the manifest's `integrity_anchors` and the step-3
   integrity flag; either firing disqualifies it. Integrity items stay in the P0 cluster.

2. **No-Goodhart — surfacing device, not a metric.** "Quick wins closed" is **never**
   counted as success. Counting it would reward trivial churn (file-and-close easy issues
   to move a number) over real fixes — the exact dynamic B3 + the no-Goodhart rule forbid.
   Maintainer-progress (the outcome-paired section) still moves only on verified
   fix-outcomes. The lane *shows* opportunities; it does not *score* their consumption.

## Cold-start tie-in (ADR-005)

In cold-start the wheel hasn't turned and there's no rate to report. The quick-wins lane
is the lowest-risk way to **turn it**: a maintainer resolves one safe item, the first
verified fix-outcome lands, and the baseline begins. The lane is the cheap on-ramp the
cold-start doctrine (B8) said the dashboard's job was to provide.

## What shipped (Phase 0)

- `docs/dashboard-schema.md` §3b — the lane as a body section, orthogonal to the action
  plan, with the derivation rule, the finding-004 exclusion, and the no-Goodhart caveat.
- `skills/beadle-triage/SKILL.md` step 7a — render procedure with the explicit
  integrity-exclusion audit and the no-Goodhart constraint.
- Live render: #312 run 4 surfaced five verified candidates (#209 one-line signing, #239
  docs runtimes, #256 trunk-branch default, #280/#282 agent-prompt warnings), each
  audited clear of the vsdd-factory integrity_anchors.
- PR #6 (`f05d89f`), merged to main.

## What stays open

No new frontier question — the lane is decided and shipped. The open calibration it
*shares* with finding-005 (`question-defect-classification-axes`): the precise
defect-nature / reproducibility threshold that auto-qualifies an issue as a quick win vs.
needs-a-human-look. Tracked there, not duplicated.
