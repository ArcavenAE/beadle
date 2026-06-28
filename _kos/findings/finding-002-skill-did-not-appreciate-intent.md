# finding-002 — the skill did not appreciate intent

Date: 2026-06-28
Probe: first live triage of 5 new vsdd-factory issues (#308–#310, #313, #314)
during the run-2 dashboard refresh. Surfaced by the user: "I think maybe our skill
does not appreciate the intent."
Nodes touched: `question-intent-manifest-schema` (F1); manifest v0.1→v0.2.

## What happened

On the first refresh I slotted the new issues into the dashboard by **title
keyword** ("CI", "artifact" → a "CI/CD cluster"; "process-gap" → Watch) and never
ran beadle's `score-intent` engine on their bodies. score-intent against the
`alignment_rubric` with cited rationale is beadle's entire differentiator (B4) —
and it was skipped. The board looked triaged; the issues weren't.

## Four intent-blind misses (all four trace to one root: pattern-matched titles)

1. **Ignored provenance.** All five are dogfooding findings from an external pilot
   (ftc-blue) *running* vsdd-factory, with cited commits/files/reproductions. For a
   self-referential factory (engine == product) these are the highest-value input,
   not backlog. No provenance signal existed in the manifest or skill.

2. **Would mis-fire the self-reference rule.** The rubric lists "a self-referential
   process-gap that doesn't change product outcomes" as `drifts`. A naive read flags
   every `process-gap(...)` issue as drift. But for THIS target the engine IS the
   product, so engine process-gaps DO change product outcomes — the rule is
   inverted. Nothing told the skill that.

3. **Missed that #308 is beadle's own thesis reflected back.** #308 documents the
   factory's reflex of converting every adversarial finding into a permanent CI lint
   with no decay term — exactly `over_engineering_fingerprints` /
   `low_leverage_ratio_rising`. beadle should elevate it as a strategic ally, not
   bucket it by the word "CI".

4. **Missed the corpus-level recursion (the real prize).** 100 arcavenai issues,
   mostly granular engine process-gaps, 0 maintainer engagement = the precise
   `filed_vs_acted_gap_widening` + "led-by-the-backlog" pattern the manifest's
   `minutiae_signals` exist to catch — and the measured filer is generating the very
   over-machinery #308 independently documents. The detectors were only ever
   conceived per-issue; the drift is a property of the whole filing pattern, and
   holds even when each issue scores "advances."

## Correct triage (after reading the bodies)

| # | type · leverage | provenance | intent verdict | ROI |
|---|---|---|---|---|
| #314 | bug · systemic | pilot-derived (commit 4235b76, 11 specs, real hashes) | ADVANCES (strong) — removes a drift surface / convergence soundness | highest |
| #313 | bug/process-gap · moderate | pilot-derived (git log empty, 404 protection) | ADVANCES — tightens a gate (PASS must require committed artifacts) | high (data-loss) |
| #310 | bug/process-gap · moderate | pilot-derived (lint script, TOTAL 75→78, inverse of #277) | ADVANCES — tightens an existing gate | medium |
| #309 | process-gap · systemic | pilot-derived (analogue of #259/#298) | ADVANCES — convergence soundness; sibling of #259/#298 | medium-high |
| #308 | meta/strategic | pilot-derived (extensive ftc-blue metrics) | NEUTRAL→ADVANCES — study request, surfaces a real drift surface | roadmap-level |

Correct grouping is by **intent semantics**: drift-soundness (#314), data-safety
(#313), gate-ordering (#310), convergence-parity (#309 → fold into #259/#298),
framework-cost-strategy (#308) — NOT a keyword "CI cluster".

## Fixes applied (manifest + skill + template — PR)

Manifest `vsdd-factory.intent.yaml` → **v0.2**:
- `self_referential: true` with the inversion explained.
- `provenance_signal` axis (pilot-derived vs speculative markers) as a *ranking*
  signal.
- rubric sharpened: self-referential pilot-derived engine gaps are `advances`;
  SPECULATIVE self-reference is `drifts`; study requests are `neutral→advances`.
- `minutiae_signals` gain `convert_finding_to_guard_unbounded` (#308) + a comment
  that they run at the CORPUS level against the filer.

`skills/beadle-triage/SKILL.md`:
- score-intent is **MANDATORY per artifact**, reads target semantics before the
  rubric, groups by intent not keyword.
- new step 4b: corpus-level minutiae against the measured filer.
- guardrail added: **Intent fidelity (B4)** — title-level slotting is a defect.

`targets/_template.intent.yaml`: carries `self_referential` (default false),
`provenance_signal`, and the corpus-level minutiae note so future targets inherit it.

## Charter delta (kos harvest — update the nodes, then the charter projection)

F1 (intent-manifest schema) should record that the schema now needs: a
`self_referential` flag, a `provenance_signal` ranking axis, and explicit
corpus-vs-per-issue scope for minutiae signals. These are the first concrete
schema requirements F1 was asking for.
