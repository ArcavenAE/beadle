---
description: Backup → skills-based triage refresh → verify no section regression → post. Never uses the beadle binary.
argument-hint: [target] (default vsdd-factory)
---

# /dashboard-refresh — skills-based dashboard refresh with regression gate

Target: `$ARGUMENTS` (default `vsdd-factory`, dashboard issue `#312` on
`drbothen/vsdd-factory` — resolve other targets via `targets/<target>.intent.yaml`).

Distilled from the sessions of 2026-07-02 (run 10 — binary-render regression,
skill restoration) and 2026-07-05 (run 11 — curated restoration + fail/restore
rule). This command exists because the `beadle` Rust binary's render path is
**not load-bearing and currently unreliable**: it produces a flat unclassified
table and strips the analysis (P0a/P0b/P1/P2 intent grouping, quick wins,
direction health, classification index, maintainer progress). The Claude Code
skill carries the discipline. **Do not run `beadle render` / `beadle push` at
any point in this workflow.**

## 1. Backup (never skip, never proceed on failure)

1. `mkdir -p tmp/dashboard-snapshots`
2. Fetch the live dashboard body and metadata:
   - body → `tmp/dashboard-snapshots/<target>-312-before-<UTC yyyy-mm-ddTHH-MM-SS>.md`
   - `gh issue view 312 -R drbothen/vsdd-factory --json body,updatedAt,title,number` →
     same path + `.json`
3. Sanity-check the snapshot: non-empty, contains the `<!-- beadle-state:v1` sentinel,
   and its `"run":N` / `"watermark":W` parse. Record N and W.
4. If any of this fails: **STOP**. No refresh without a verified backup.

## 2. Refresh — run the SKILL, not the binary

Follow `skills/beadle-triage/SKILL.md` (the load-bearing analysis discipline),
with `docs/fixtures/<target>-312-curated-run12.md` (latest committed fixture)
as the **shape canon**. Key operational notes from the sessions:

- Enumerate open issues with number > W (the sentinel watermark).
- Classify new issues in **batches of ~12–15 via parallel subagents**; each
  grader gets the intent manifest (`targets/<target>.intent.yaml`) + the full
  issue bodies, and returns per-issue axis records (report-type, defect-nature,
  reproducibility, triage-state, leverage, alignment + cited rationale,
  provenance, integrity, operational-impact, attn facet, quick-win flag).
  Subagent API failures (429/503/timeout) are retried/resumed, never silently
  dropped — every issue number in the batch must appear in the returned records.
- score-intent is MANDATORY per artifact (SKILL §4); corpus-level minutiae
  detectors run against the filer (§4b).
- Update the store (`store/<target>/state.jsonl`): append run record,
  issue observations, classification records (SKILL §6).
- Regenerate the FULL body (SKILL §7): carry forward all prior curated
  content (prior-run classification indexes, unchanged P2 clusters, keystone
  history); merge the new run's analysis into every section; bump
  `"run"` and `"watermark"` in the sentinel state JSON; refresh the digest.
- Read controls from the prior body before regenerating (SKILL §8).
- Write the candidate body to
  `tmp/dashboard-snapshots/<target>-312-candidate-<UTC ts>.md`. **Do not post yet.**

## 3. Verify — the regression gate (fail = restore, from 2026-07-05)

Compare the candidate against BOTH the before-snapshot and the fixture canon.
ALL checks must pass:

1. **Section presence** — every one of these exists (heading match, order per canon):
   - `<!-- beadle-state:v1` sentinel with parseable JSON (run = N+1, watermark > W)
   - Direction verdict paragraph (top, bolded verdict line)
   - `## Baseline`
   - `## 👤 Needs human reading` (attn lane — first section after Baseline)
   - `## Action plan` with `### 🔴 P0a`, `### 🔴 P0b`, at least one `### 🟠 P1`,
     at least one `### 🟢 P2`
   - `## 🟦 Quick wins`
   - `## Direction Health`
   - `## Classification index` for the new run AND carried-forward prior index(es)
   - `## Maintainer progress`
   - `## Controls`
2. **No section loss** — every `##`/`###` heading present in the before-snapshot
   is present in the candidate (renames must be justified in the run summary).
3. **No coverage shrinkage** — every issue number referenced in the
   before-snapshot's state JSON (p0 lists, clusters, indexes) appears in the
   candidate state JSON unless it was closed on GitHub (verify closures).
4. **Classification completeness** — no `_unclassified_` rows; every
   new-since-watermark issue appears in the new run's classification index.

**On any failure:** do NOT post. Report the diff. The live body is untouched;
if a bad body was ever posted, restore it with the before-snapshot via
`gh issue edit 312 -R drbothen/vsdd-factory --body-file <before-snapshot>`.

## 4. Post + record (only after the gate passes)

1. Present a before/after delta summary (sections, counts, new P0/P1 items).
2. Update the dashboard: `gh issue edit 312 -R drbothen/vsdd-factory --body-file <candidate>`
   (allow-listed autonomous action per charter B2 — a bounded, reversible
   dashboard rewrite; free-text public comments stay propose-not-act).
3. Save the pushed body as `tmp/dashboard-snapshots/<target>-312-after-<UTC ts>.md`.
4. Commit the candidate as the new acceptance fixture
   `docs/fixtures/<target>-312-curated-run<N+1>.md` (+ rich classification JSON)
   on a feature branch → PR (repo forbids direct pushes to main).
5. Report: run number, watermark movement, issue deltas, verification results,
   snapshot paths.

## Guardrails (charter/incidents — non-negotiable)

- State out-of-band (B1): the issue body is a projection; never parse it as
  machine state beyond the sentinel/controls contract.
- Propose-not-act (B2): dashboard rewrite is allow-listed; public free-text
  comments and closures are NOT part of this command.
- Never auto-close on inactivity (G2).
- No Goodhart: counts always pair with outcome signals.
- The backup is sacred: keep every before-snapshot; never overwrite one.
