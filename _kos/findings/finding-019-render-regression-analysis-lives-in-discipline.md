# finding-019 — the render regression: analysis quality lives in the discipline, not the plumbing

**Status:** confirmed (three sessions of evidence: 2026-07-01, 2026-07-02, 2026-07-05)
**Scope:** beadle Phase-0 — `beadle render`/`beadle push` vs the skill-curated dashboard
**Severity:** the exact class beadle exists to catch — a green-looking mechanical
success that silently destroyed the product's purpose. Self-referential instance
of finding-004.

## Symptom

PRs #18–#22 shipped a Rust projection pipeline (`enum → sync → classify ingest →
direction → render → push`). Run against vsdd-factory, `beadle render` produced a
57.7 KB body that was **objectively worse than the GitHub issues tab**: a flat
223-row `_unclassified_` table, no P0a/P0b/P1/P2 intent-grouped action plan, no
quick-wins lane, no Direction-Health narrative, no maintainer-progress pairing,
no Tier-1 controls. The user's verdict (2026-07-02, verbatim): "this is a
devistating regression … all the value, all the context, orientation,
prioritization, everything of value was lost." And: "this plumbing is not
loadbearing. It's hard to say what it does of value."

## Root cause — two distinct gaps, one compounding process failure

1. **The producing layer was never run.** PR #18 correctly refused to put an
   LLM-judgment heuristic in Rust ("keyword-matching would launder guesses") —
   classification stays skill-side. But nothing compels the skill classifier to
   run before render. PR #19's own test evidence showed "0 classifications,
   every row `_unclassified_`" — and merged anyway. We built the projection
   layer and never built (or ran) the layer that produces the thing projected.
2. **The curated projection is not in the binary at all.** Even with a fully
   ingested classification set, `render.rs` emits chips on a *flat list* plus a
   count summary. The intent-grouped action plan (P0a silent-data-loss → P0b
   integrity → P1 halt/keystone/convergence → P2), the quick-wins lane with the
   finding-004 hard-exclusion audit, the per-cluster agent-channel readings, the
   maintainer-progress outcome pairing — the *analysis* — exist only in the
   skill (SKILL.md §3–§7) and in the hand-curated bodies.

Compounding: each PR was individually defensible (mechanical validation ✓,
deterministic digest ✓, tests ✓) and the sequence Goodharted on "renderer
correctness" while the board's purpose — *organizing issues for maintainer
understanding, prioritization, and action* — regressed to zero. No invariant
gated "the rendered body must be at least as decision-useful as the last
curated body," so nothing failed loudly.

3. **Vocabulary drift (2026-07-05 instance).** classify.rs's bounded enums
   don't cover the vocabulary the skill's own §3 axes produce: no
   `process-gap`/`enhancement`/`policy` report-types (the dominant types in the
   corpus — ~30 of 47 in run 10), no `halt`/`panic` operational-impact (the
   finding-009 axis render itself documents). Ingesting real classifications
   requires a lossy shim (`tmp/run11/map_to_store.py`). The taxonomy "holds by
   construction" against a vocabulary the pipeline upstream doesn't speak.

4. **Run-ordering staleness.** `beadle direction` computes against the store's
   latest `run` record (run 9) even after `enum` has appended run-10+ issue
   observations — the run record is only written at render/push time, so
   direction is always one run behind during a pass. Observed 2026-07-02 and
   again 2026-07-05.

## What the binary DID get right (keep list)

- **enum/sync** — watermark-bounded GitHub sweeps into the JSONL store; cheap,
  correct, removed the hand-maintained sentinel-counts staleness (the run-9
  `open: 178` sentinel was stale minutes after hand-push).
- **Store as source of truth** (B1) with derived_digest over the derived zone —
  hand-edit detection works.
- **classify ingest validation** — bounded enums + HARD-invariant enforcement
  (integrity ⇒ anchor; integrity ⇏ quick-win) is the right shape, wrong
  vocabulary (gap 3).
- **push discovery discipline** — sentinel-first, exactly-one, wrong-author
  STOP; editor-slot preservation across regen.
- **Board-control scanning** (F5 poll half) — parse-then-reset de-bounce.

## What must never again be delegated to the renderer

The dashboard body's *analysis zones*: action-plan grouping and ordering
(precedence per finding-004), quick-wins eligibility narrative, cluster
readings, direction interpretation, maintainer-progress framing. These are
LLM judgment projected as prose. A deterministic renderer can *frame* them
(section skeleton, digests, controls) but not *author* them.

## Interventions (proposed, see charter F-next / bd)

a. **Fail-loud render:** `beadle render` refuses to emit (or emits with a
   giant UNPUBLISHABLE banner + nonzero exit) when the current run has zero
   classifications. A projection of nothing must not look like a dashboard.
b. **Curated-zone slots:** extend the editor-slot mechanism from 2 slots to a
   full curated zone — the skill authors the analysis sections; the binary
   owns sentinel, controls, digest, and slot preservation. Renderer never
   emits a flat all-issues table (kill it outright — it violates B1's
   "projection, not replica" and busts the 65 K budget at scale).
c. **Vocabulary reconciliation:** classify.rs enums must be a superset of the
   skill's §3 vocabulary (add process-gap/enhancement/policy report-types;
   add halt/panic to operational-impact per finding-009) — or the enums move
   to a shared manifest both sides load.
d. **Pipeline compulsion:** SKILL.md step 6 makes classify-then-ingest a
   gating precondition of step 7 (render/push), not an optional aside.
e. **Run-record-first ordering:** enum opens the run record; direction/render
   compute against it.

## Cross-references

- finding-004 (silent-integrity severity class — this finding is its
  self-referential instance: beadle's own green checks masked the corruption
  of beadle's own source of value)
- finding-011 (store shape), finding-015 (chips), finding-017 (direction
  verdict render)
- Session evidence: GOLDEN-do-not-delete-vsdd-factory-312-body.md,
  vsdd-factory-312-after-render.md (the regression artifact),
  vsdd-factory-312-curated-run10.md (the recovery artifact) in
  tmp/dashboard-snapshots/
