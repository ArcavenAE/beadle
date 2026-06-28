# finding-008 — measurement/comparison is critic's job, not beadle's

Date: 2026-06-28 (session-051)
Probe: a beadle triage session against vsdd-factory repeatedly drifted into
death-spiral detection, per-run velocity, $/commit drift, and "why does akey
look different" — work that is properly **critic's** (the run registry + arena).
Nodes touched: beadle scope (B-layer); cross-refs critic
(`ArcavenAE/critic`, `docs/analysis/boundary-beadle.md`).

## The boundary

| | **critic** | **beadle** |
|---|---|---|
| Verb | measure, compare, compete, **detect** | record, report, prioritize, **surface** |
| Unit | a run / a cohort of runs | a GitHub issue / a target backlog |
| Output | quantitative comparison, regression/breakdown | dashboard, filed/deduped issues, alignment verdict |
| GitHub mutation | **no** (emits findings inward) | yes (propose-not-act, B2) |
| Cross-run statistics | **yes** (core) | no |
| Per-artifact intent scoring | no | **yes** (B4) |

**The tell:** if answering needs **more than one run** or a **trend across
time**, it's critic. If it's "is this one issue real, aligned, prioritized,"
it's beadle.

## Why beadle still surfaces measurement-shaped things

beadle's intent-grounded triage legitimately *surfaces* defects a volume-counter
misses — finding-007 (OTEL has no per-run dimension) was a beadle byproduct. But
*quantifying* a spiral or comparing runs is critic's arena. The same OTEL gap is
read from both ends: beadle **surfaced** it; critic **depends** on it (it cannot
attribute metrics to a run without that dimension — drbothen/vsdd-factory#324).

## How they compose

critic detects a breakdown/regression → if defect-shaped and in a governed
target, the finding flows to beadle → beadle applies propose-not-act, dedupes
against existing issues, classifies, and reports (or, as with the resolver-storm
this session, verifies an issue already exists — #242 — and stays silent).

## What this session built in critic (not beadle)

Per the user's "clone critic and scaffold the analysis there":
`ArcavenAE/critic` PR #1 — `docs/analysis/` (measurement engine + this boundary)
and `skills/critic-compare/` with a runnable per-run health extractor, verified
against three live rc.21 pilots (all in a load-storm breakdown, ~66% error rate,
`registry_loaded=0`). critic prefers inherently per-run on-disk telemetry so it
works *despite* the #324 attribution gap.

## Consequence for beadle

When a beadle pass turns up a cross-run / trend / comparison question, beadle
**routes it to critic** rather than answering it inline. beadle keeps the
record, the report, the priority, and the intent verdict; critic keeps the
measurement.
