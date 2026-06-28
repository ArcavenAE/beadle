# finding-001 — dashboard discovery robustness

Date: 2026-06-28
Probe: design review of dashboard create-vs-update (no code spike — review of the
shipped Phase-0 skill + launcher + schema).
Node: `_kos/nodes/frontier/question-dashboard-discovery-robustness.yaml`

## Question

beadle is generic over targets and rewrites a single pinned dashboard issue per
target in place. How does it know which project to act on, and can it create a
duplicate dashboard?

## What's already correct

- **Project selection is explicit.** The target is an input key in `targets/`;
  the manifest's `target.repo` is the GitHub coordinate. Nothing is inferred from
  cwd/env. `status: draft-unpopulated` manifests are refused (guards `prism`).
- **Create-vs-update is title-keyed** (Renovate's pattern): exact-title open issue
  authored by the beadle identity → rewrite in place; wrong author → STOP (don't
  fork); none → create + pin. The identity-mismatch STOP is the right behavior and
  is unchanged (it already prevented a fork after vsdd-factory#311 was mis-authored
  by `arcaven`).

## Four edges found

1. **Title-as-only-key fragility.** Discovery was exact-title only. GitHub allows
   duplicate titles and titles can be hand-edited; any drift → miss → duplicate.
2. **No multi-match guard.** If a duplicate already exists, exact-title search
   could return >1 and the spec didn't say what to do — risk of silently picking
   one or creating another.
3. **Identity mismatch.** Correct as-is (STOP, don't fork). No change.
4. **Creation race (TOCTOU).** Two runs can both see "no dashboard" and both
   create. Not closable by a check alone.

## Fixes applied (obvious hardening — no spike needed)

Specialist operational docs (CLAUDE.md/DESIGN.md/README are the human-edit-only meta-docs; charter.md is NOT — it is kos harvest output):

- **`docs/dashboard-schema.md`** — discovery now keys PRIMARILY on the
  `<!-- beadle-state -->` sentinel (machine-stable, survives title edits); exact
  title is a secondary/fallback signal. Candidate set = union of both. >1 candidate
  by the beadle identity → STOP + report + request consolidation. Re-check
  immediately before create (narrows the race). Clarified TITLE = bare string;
  the `· owner/name` suffix is body-H1 only.
- **`skills/beadle-triage/SKILL.md`** — step 7 discovery updated to match.
- **`prompts/create-dashboard.md`** — STEP 2 decision tree updated: sentinel-first
  discovery, multi-match STOP branch, re-check-before-create.

## Deferred (genuinely needs design — stays frontier)

The **concurrency guarantee**. Re-check-before-create narrows but does not close
the TOCTOU window. A real guarantee requires a SERIALIZED CHOKEPOINT for dashboard
creation — the INC-003 lesson (advisory checks don't prevent races). Low risk in
Phase 0 (single session); becomes real at Phase 1 (scheduled gh-aw) and Phase 2
(marvel single-writer orchestrator). Tracked in the frontier node; resolve with
F3 (marvel team shape) and the Phase-1 cron design.

## Charter delta (kos harvest — update the nodes, then the charter projection)

Add to `charter.md` Frontier:

> ### F7: Dashboard discovery robustness
>
> Discovery keys on the `<!-- beadle-state -->` sentinel first, exact title
> second; >1 candidate → STOP and request consolidation; re-check before create.
> Open: the concurrency guarantee — dashboard creation needs a serialized
> chokepoint (INC-003 class) once Phase 1 (scheduled gh-aw) or Phase 2 (marvel)
> can run concurrent passes against one target. See
> `_kos/nodes/frontier/question-dashboard-discovery-robustness.yaml`, finding-001.
