# finding-005 — reconciling beadle's classify model with the defect-taxonomy research

Date: 2026-06-28
Probe: deep-research workflow `we6yrrcba` (issue/defect classification taxonomies for
triage), commissioned to ground beadle's `classify` engine. Reconciled against the
eight signals beadle already carries, then the superset wired into the skill + schema.
Nodes touched: `classify` engine (skill step 3 + step 7), `question-intent-manifest-schema`
(F1), new `question-defect-classification-axes`. Composes finding-004 (severity model).

## What the research established (primary-sourced)

- **Report-type axis** is first-class and now native: GitHub ships an org-level
  **Issue Types** feature (Task/Bug/Feature, distinct from labels, assigned via the
  sidebar Type field, filterable). beadle runs under the `ArcavenAE` org → fits, and
  it is the natural **primary grouping pivot** for the dashboard. Long tail handled by
  a `kind/*`-style label family (Kubernetes) / orthogonal `C-*/T-*/A-*/P-*` families
  (Rust) — both real production precedents for an N-axis scheme where families co-occur.
- **Defect-nature axis** ("defect" is THE industry term — IEEE 1044 classifies
  *anomalies* with *defect* as the facet; **O**rthogonal **D**efect **C**lassification;
  "defect taxonomy"). IEEE 1044-2009 + ODC **Defect Type** (nature of the fix) ground a
  mechanical→conceptual spectrum: syntax/typo/wrong-variable → off-by-one/boundary →
  null/resource/lifecycle → concurrency/race → logic → algorithmic → spec/requirements →
  design/architectural → **directional/intent-misalignment** (code correct, wrong thing
  built). ODC **Defect Trigger** (what surfaced it) is an orthogonal second attribute.
- **Reproducibility class** — Bohrbug | Mandelbug | Heisenbug (Grottke-Trivedi). The
  research calls this *"the single most decision-relevant axis for evaluation cost."*
  Bohrbug = decide-in-seconds/auto-triage; Mandelbug/Heisenbug = deep investigation,
  escalate. Feeds the **level-of-effort** half of priority. Real precedent: Kubernetes
  `kind/failing-test` (consistent) vs `kind/flake` (intermittent) is this axis in a label.
- **Severity ≠ priority** — severity = impact; priority = urgency **+ level-of-effort**.
  Confirms beadle's existing split and gives the LOE term a source (which the
  reproducibility class now estimates instead of gut-feel).
- **Triage-state axis** — `needs-triage → triage/accepted → triage/needs-information`
  (Kubernetes); Rust `regression-untriaged` ("nature unclear, needs further triage") is
  the escalation hook for unknown-nature reports.

## What maps, what's net-new, what's beadle-original

| beadle axis | research analog | verdict |
|---|---|---|
| severity (impact) | severity doctrine | validated, now citable |
| priority (urgency) | priority = urgency + LOE | validated; LOE term sourced |
| **report-type** | GitHub Issue Types + kind/* | **NET-NEW — adopt** |
| **defect-nature** | IEEE 1044 + ODC Defect Type | **NET-NEW — adopt** |
| **reproducibility class** | Bohrbug/Mandelbug/Heisenbug | **NET-NEW — adopt, feeds priority** |
| **triage-state** | k8s needs-triage→accepted | **NET-NEW — adopt** |
| leverage (systemic↔minutiae) | — none — | beadle-original (research flagged it unsourced) |
| alignment (advances↔drifts, B4) | "directional/intent" — **open question, no standard names it** | beadle-original, *ahead of the field* |
| provenance (pilot vs speculative) | adjacent ODC Source/Age | beadle-original |
| blast-radius visibility × compounding (finding-004) | nearest is **FMEA detectability** (research did NOT surface it) | beadle-original; FMEA is the citable anchor |
| recoverability tier (finding-004) | adjacent ODC Impact | beadle-original |
| integrity-gates-functional precedence (finding-004) | — none — | beadle-original (a precedence rule, not an axis) |

## What the research does NOT cover (the asymmetry that matters)

The research is entirely about **software defects on an ordinary product**. It is silent
on the three things most specific to a self-referential factory — exactly the finding-004
cluster:

- **Process-failure / source-of-truth integrity** (silent-integrity class, recoverability
  hierarchy learning > spec > code, integrity-gates-functional precedence). No surveyed
  standard reaches it; ODC Impact/Trigger are the nearest neighbours and fall short. The
  user surfaced this **ahead of** the research and it stands — it is beadle's contribution,
  not a gap to backfill.
- **The directional/intent axis** — research logs it as an explicit **open question**
  ("no surveyed standard names this class; how to operationalize it is unknown"). beadle's
  `alignment` axis (B4) is the project's bet on precisely this.
- **Detectability as a severity multiplier** — FMEA (RPN = severity × occurrence ×
  detectability) is the citable home for finding-004's blast-radius-visibility axis, but
  the research never surfaced FMEA. Pull it in as the reference if/when we want a standard.

**Conclusion: the unified model is a superset** — research's multi-axis frame (report-type,
defect-nature, reproducibility, triage-state, severity, priority) PLUS beadle's four
originals (leverage, alignment, provenance, the finding-004 integrity cluster), with
reproducibility-class feeding priority's effort term and the integrity cluster sitting
*above* the defect-nature axis as a precedence gate (finding-004).

## Two open research questions worth tracking (not blocking)

1. No empirical data correlating reproducibility class with *actual* maintainer fix-time —
   so "Bohrbug = cheap" is a heuristic, not calibrated. (research openQuestion 1)
2. No canonical citable ODC Defect-Type value-set (the fixed-7-types claim was REFUTED) —
   beadle adopts the *spectrum*, not a bounded enum, until a version is chosen.
   (research openQuestion 2)

## Fixes applied (this PR)

**`skills/beadle-triage/SKILL.md` step 3 (classify):** the axis line gains, as named
**defect** axes: **report-type** (GitHub Issue Types primary + kind/* long-tail),
**defect-nature** (IEEE 1044 / ODC mechanical→conceptual spectrum), **reproducibility
class** (Bohrbug/Mandelbug/Heisenbug → feeds priority's effort term + escalation), and
**triage-state** (needs-triage → accepted → needs-info). The finding-004 integrity blocks
are unchanged and remain the precedence gate above all of these.

**`skills/beadle-triage/SKILL.md` step 7 (regenerate) + `docs/dashboard-schema.md` body
sections:** dashboard legibility fix — every Action-plan and Classification-index row
leads with a **short human title** (a few words: what the issue IS), `#NN` second; the
verdict/status is a trailing chip, not the headline. (See the dashboard-legibility note
below — separate user observation, same render path, fixed together.)

**`targets/vsdd-factory.intent.yaml` + `_template.intent.yaml` → schema v0.3:** add
`integrity_anchors` (target-declared systems-of-record by recoverability tier) so the
finding-004 silent-integrity classifier has grounding instead of inference. The
defect-nature / reproducibility / report-type / triage-state axes are engine-level
(target-agnostic) and live in the skill, NOT the manifest — only *where a target's
source-of-truth lives* is target-shaped.

## Dashboard legibility (user observation, 2026-06-28)

> "humans are unable to orient to these issues by number reference alone and need
> something that short-summary the title ... they will not [hover]. We are
> overrepresenting the 'verdict' and misusing that screen real-estate over what is a
> status ... we lack the context/basic understanding of what that issue is that any
> human would expect."

Root cause: the run-2 board rendered rows as `#NN — VERDICT`, treating the alignment
verdict as the headline. Verdict is **status** (drill-in detail); the headline a human
needs first is **what the issue is**. Fix: row format becomes
`#NN <short human title> · <type chip> · <verdict chip>` — title leads, chips trail.
Recoverability: zero — pure presentation. Baked into the schema + skill render step.

## Dual-audience rendering: LLM-first, human-supported (user, 2026-06-28)

> "we expect this dashboard to MAINLY be consumed by LLM, by other claude code agent
> sessions ... data intended for llm / agents can be folded/embedded in links because
> they will be instantly visible without clicking or hovering to the llm" ... "the PR
> references the llm can follow or register easily so we don't need to replicate all the
> issues in detail here for the llm."

This *resolves* the legibility tension rather than trading against it. The two audiences
have **inverted visibility**: a human reads only the rendered surface (won't hover, won't
expand `<details>`, won't read HTML comments); an LLM reads the raw markdown source (sees
folded `<details>`, link titles, comment payloads instantly, at zero cost). So one body
serves both via **three channels**:

1. **Human channel** — visible render: short title + minimal chips.
2. **Agent channel** — folded/embedded: the beadle-computed judgments that exist nowhere
   else (full axis vector, cited verdict rationale, integrity flags). Free to the LLM.
3. **Reference channel** — links the LLM follows: `#NN`, PR refs, shas. The dashboard
   **never replicates issue detail** — an agent fetches it. This is B1 restated (the body
   is a projection/index, not a replica) and also protects the 65 k budget.

Rule: surface short titles for humans; embed/fold the axis vector + rationale for agents;
link (never copy) anything an agent can fetch. Each audience gets its ideal view from one
artifact — not a compromise. Baked into the schema (new "Dual-audience rendering" section)
and skill step 7.

## Charter delta (kos harvest — update the nodes, then the charter projection)

- **F1 (intent-manifest schema)** now also requires `integrity_anchors` (systems-of-record
  by recoverability tier) — the second concrete schema requirement after finding-002's
  `self_referential`/`provenance_signal`.
- **New bedrock candidate:** beadle's classify model is the documented superset above —
  six research-grounded axes (report-type, defect-nature, reproducibility, triage-state,
  severity, priority) + four beadle-original (leverage, alignment, provenance, the
  finding-004 integrity cluster). "Defect" is the canonical term for the nature axis
  (IEEE 1044 / ODC). Reproducibility class feeds priority's effort term; the integrity
  cluster gates the validity of every functional verdict (finding-004 precedence).
- A **dashboard-render rule:** rows lead with a short human title; verdict/status is a
  trailing chip, never the headline. **Dual-audience corollary:** the dashboard is
  LLM-first (mainly read by agent sessions) and human-supported — surface short titles
  for humans, fold/embed the axis vector + rationale for agents, and link (never replicate)
  anything an agent can fetch via `#NN`/PR ref. This is B1 restated.
- A **maintainer-progress section** (outcome-paired, not a leaderboard) is added to the
  dashboard — frontier `question-maintainer-progress-gamification`. Reward verified
  fix-outcomes, never close-rate/volume (B3 + no-Goodhart).
- Defer to **`question-defect-classification-axes`** the two open research questions
  (reproducibility↔fix-time calibration; canonical ODC value-set).
