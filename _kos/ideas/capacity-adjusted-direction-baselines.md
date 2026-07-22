# Capacity-adjusted direction baselines — grounding beadle's expectations in OSS community data

**Status:** idea (research capture + retuning proposal, 2026-07-22)
**Trigger:** operator challenge after run-16 posted 🔴 DRIFTING (unqualified) on the
maintainers' most productive engagement day ever. The operator — who talks to these
maintainers as humans — flagged that vsdd-factory is one of hundreds of projects for
two people whose focus is mostly elsewhere, and asked that beadle's expectations and
success measures be re-derived from actual OSS community data rather than implicit
full-time-maintainer assumptions.
**Relates to:** finding-014/016/017 (direction signals), `ramp-vs-drift-phase-qualifier.md`
(the prediction protocol this would re-parameterize), targets/vsdd-factory.intent.yaml,
aae-orc#65 (falsification protocol).

## 1. What the community data says

### Maintainer capacity (the denominator we've been ignoring)

| Fact | Number | Source |
|---|---|---|
| Popular GitHub systems with truck factor ≤ 2 | **65%** (of 133 popular projects) | Avelino et al., "A Novel Approach for Estimating Truck Factors" (arXiv:1604.06766) |
| Maintainers who are unpaid hobbyists | **60%** | Tidelift State of the Open Source Maintainer, 2023 & 2024 |
| Maintainers identifying as professional (most/all income) | **12–13%** | Tidelift 2024 |
| Unpaid maintainers spending ≤ 10 h/week (across ALL their projects) | **78%** | Tidelift 2024 |
| Maintainers reporting burnout / feeling underappreciated | **44% / 48%** | Tidelift 2024 |

A two-person team maintaining a repo as one of hundreds of side projects is not a
degenerate case — it is the *modal* shape of open source. Any instrument that
implicitly assumes continuous attention measures against a fictional population.

### Pull-request baselines (human era, ACTIVE projects — the optimistic end)

Gousios, Pinzger & van Deursen, "An Exploratory Study of the Pull-based Software
Development Model" (ICSE 2014; GHTorrent, 166,884 PRs across 291 projects *selected
for high PR activity*):

- **84.73%** of PRs eventually merged in the curated active-project sample
  (**~73%** GitHub-wide by facilities detection).
- Merge is fast-or-never: **30%** merged in under an hour; **80%** within 3.7 days;
  **95%** within 26 days. Merged PRs close at median **434 minutes**; unmerged
  linger (median 2,250 min).
- Typical PR volume is TINY: mean **8.1 PRs per project** in 2013, median **2**,
  95th percentile **21 per year**. Only **14%** of active repos used PRs at all.
- Established requesters' median personal success rate: **0.78**.

Key bias note: these numbers come from the *most actively maintained* projects of
their era. Lightly-maintained side projects sit far below these response baselines.

### Bot- and AI-authored contribution baselines (the directly relevant population)

| Fact | Number | Source |
|---|---|---|
| Bot-authored PRs accepted vs human | **37.38% vs 72.53%** | "Bots Don't Mind Waiting, Do They?" (arXiv) |
| Code-review-agent-only PRs merged vs human-only | **45.20% vs 68.37%** (−23.17 pts) | "From Industry Claims to Empirical Reality: Code Review Agents in Pull Requests" (arXiv, 2025/26) |
| One-time contributors' PR merge-rate drop in the GenAI era (2025), vs counterfactual | **−18.18%** | "AI Slop is DDoSing Open Source" (arXiv:2607.04003, Jul 2026; 294 repos, 2M+ PRs/issues, BSTS analysis) |
| Documented maintainer response to AI contribution floods | "communities often default to low-effort **defensive strategies**" (bans, lockdowns) | same |

The AI-DDoS paper describes exactly the mechanism beadle's measured side embodies:
high-volume plausible AI-generated contributions against finite human review
capacity. The *documented modal outcome* of that pattern in 2025–26 is defensive
lockdown — locked issue trackers, AI-contribution bans — not engagement.

## 2. Our observed numbers against those baselines

vsdd-factory: 2 maintainers (truck-factor-2 modal case), side project among
hundreds, first external contributor is a disclosed agent identity (arcavenai)
filing at machine speed.

| Observed (as of run-16, 2026-07-22) | Community baseline | Read |
|---|---|---|
| PR acceptance: **22/31 merged (71%), 0 rejected**, 9 open | human ~68–73%; **bot ~37–45%**; AI-era OTC declining | **~2× the bot/agent baseline; AT the human baseline.** Extreme positive outlier for an AI-identity contributor in the AI-DDoS era |
| Maintainer attention: **5 action-days in ~3 weeks** (Jul 8, 15, 19, 21, 22), volume per window: 3 → 5 → 31 → 2 → 22 actions | 78% of unpaid maintainers: ≤10 h/week across ALL projects | Rising cadence AND rising per-window volume. For a side project this is exceptional; many comparable projects would show zero windows |
| Review depth: correction PR #691, post-merge body-count audit (#726), conditional merge on documented conflict resolution (#728), substantive rounds on #737/#738 | integrator-overload literature: shallow review is the norm under load | They review **hard** and merge fast — high-trust, high-scrutiny. Best-case channel behavior |
| Issue drain: 13 closes + 3 fix-merges pending against ~450 filed (≈4% acted) | no human-era baseline exists for 450 issues/3 weeks from one filer; the AI-DDoS-era baseline for this volume is **getting banned** | The acted-share is small because OUR numerator is machine-scale. The counterfactual for this filing volume was lockdown, not engagement |
| Adoption signal: 12 maintainer-authored filings INTO our taxonomy incl. their own SDL item (#635) | — | They adopted the corpus's own classes. Strongest alignment signal available |

**Bottom line:** measured against the actual community distribution — lightly
maintained project, part-time maintainers, agent-identity contributor, 2025–26 AI
climate — this engagement is top-percentile. The run-16 board's 🔴 DRIFTING
(unqualified) verdict was produced by an instrument calibrated to a population
this project does not belong to, on a clock (our run cadence) the maintainers do
not share.

## 3. What was still right in run-16

The pre-registration discipline itself worked (a 22-action day did not get
narrated into "ramping confirmed"), and the selection-function observation
survives: within attention windows, items with mergeable diffs move and
read-and-rule items (SDL lane, #510, keystones) do not. What fails is the
*interpretive frame*: under capacity constraints, cost-ranked triage is rational
maintainer behavior, not "drift." The instrument conflated "misaligned attention"
with "scarce attention rationally allocated."

## 4. Retuning proposal (for operator ratification)

1. **Capacity model in the intent manifest.** New block, e.g.
   `maintainer_capacity: {class: episodic-side-project, observed_attention_cadence:
   action-days/month, source: observed}`. Direction verdicts and streak metrics
   MUST be conditioned on it. Classes: `full-time`, `part-time-regular`,
   `episodic-side-project`.
2. **Attention-window denominators.** A maintainer attention window = a calendar
   day with ≥1 maintainer action on the target. All streaks (A4 included) report
   in windows first, runs/calendar as shadow. A4 today = ~3–5 windows, not
   "7 runs."
3. **Baseline-anchored success bands** replacing absolute expectations:
   - PR acceptance: GREEN ≥ 60% (human band), YELLOW 40–60% (bot band), RED < 40%.
     Current: 71% → GREEN.
   - Engagement cadence: GREEN = ≥1 attention window/month with nonzero actions
     for `episodic-side-project`. Current: 5 windows/3 weeks, rising → GREEN.
   - Merge latency: merged within ≤2 attention windows of PR readiness (not
     calendar days). Current: nearly all within 1–2 windows → GREEN.
4. **Severity-ceiling meter reframed as descriptive, not alarmed.** Keep tracking
   the acted-on ceiling (it is real information about the selection function) but
   drop the pejorative DRIFTING/RAMPING dichotomy for capacity-limited projects;
   report "selection depth" with the fix-delivery-cost covariate stated. A red
   direction verdict requires BOTH capacity-adjusted underperformance AND a
   falsified cost explanation.
5. **Prediction horizons in attention windows.** Pre-registered predictions get
   due dates like "within 4–6 attention windows (~60–90 days)" and must test
   actions a maintainer could take *within one window* (e.g., review one SDL fix
   PR), never re-orderings of their whole triage. The run-16 prediction (2-day
   horizon ≈ 0–1 windows) was structurally near-unfalsifiable in the good
   direction; its failure carried far less information than the board implied.
6. **Filing-rate governance on the measured side.** The AI-DDoS mechanism is us.
   Budget: do not grow the open measured-issue count while acted-share sits below
   a threshold; prefer fix-PRs-attached (observed to be the entire selection
   function); keep the zero-new-issue pauses. Our 71% merge rate against a
   37–45% agent baseline is plausibly *caused by* the fact-check/correction/
   ground-truth discipline — protect it.
7. **A4 discharge path is ours.** The alarm stays (data-loss classes matter), but
   its designed discharge is the measured side shipping SDL fix PRs to lower
   activation energy — not the maintainers re-prioritizing prose analyses.

## 5. Open questions

- Does `beadle direction` need per-class signal weights, or is a single
  capacity multiplier on streak thresholds enough for Phase 0?
- Should attention windows be detected automatically from the store's
  maintainer_actions dates (they can be), and at what granularity (day vs
  session-burst)?
- Retro-annotation of the run-16 board: operator's editor slot — machine verdict
  stands per protocol, but an editorial addendum reframing against these
  baselines is available if the operator wants it.
- Baselines above mix eras (2014 active-project PR stats vs 2025–26 AI-era).
  Worth a periodic refresh; the AI-era numbers are the ones moving fast.
