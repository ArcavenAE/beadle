# Targets — per-project intent anchors

beadle is generic; nothing about a target is hardcoded. Each target repo declares
its intent in `targets/<project>.intent.yaml`. beadle scores issues/PRs against
that manifest and weights them by what the named **maintainers** actually act on.

## Files

| File | Target | Status |
|---|---|---|
| `vsdd-factory.intent.yaml` | `drbothen/vsdd-factory` | populated (v0) |
| `prism.intent.yaml` | DrBothen's Prism | **draft — unpopulated** |

## Adding a target

1. Copy `vsdd-factory.intent.yaml` as a template.
2. Set `target` (repo, default branch, local checkout path).
3. List `maintainers` (the compass) and `measured_contributors` (who's being
   measured).
4. Point `intent_sources` at the repo's real canonical docs (composite, weighted).
   Prefer reading what exists over inventing a new doc — minimize the
   maintenance-rot risk (the GORE lesson).
5. Fill `goals` / `non_goals` / `scope_boundaries` from those sources.
6. Write the `alignment_rubric` (advances / neutral / drifts) with concrete,
   project-specific criteria.

The schema is frontier (charter F1) — expect it to firm up as more targets are
onboarded. Never score against a manifest marked `status: draft-unpopulated`.
