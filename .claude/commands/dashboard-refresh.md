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
with the **highest-numbered** committed fixture
`docs/fixtures/<target>-312-curated-run<N>.md` as the **shape canon**
(run-14 at time of writing). Key operational notes from the sessions:

- Enumerate open issues with number > W (the sentinel watermark).
- Classify new issues in **batches of ~10–12 via parallel subagents dispatched
  SYNCHRONOUSLY — never background**; each grader gets the intent manifest
  (`targets/<target>.intent.yaml`) + the full issue bodies (orchestrator
  pre-builds the input file; graders Read input → Write output, no fetching),
  and returns per-issue axis records (report-type, defect-nature,
  reproducibility, triage-state, leverage, alignment + cited rationale,
  provenance, integrity, operational-impact, attn facet, quick-win flag).
  - **Dispatch policy (run-14 evidence, 2026-07-20 — MITIGATION with a sunset,
    not permanent lore):** background-dispatched graders starved ~10–20×
    slower than sync for identical work, then died to the session timeout
    reaper (11/12 deaths in one cluster) — orc finding-083. Until that
    harness fault is resolved (sunset anchor: bd `aae-orc-0nud` — when it
    closes, re-test background dispatch and retire this rule), graders run
    sync-only, and each dispatch is sized so its expected wall clock sits
    well under the reaper: healthy grading measured ~26–50 s/issue, so a
    10–12-issue batch ≈ 5–10 min. Stagger spawns by a few seconds (no
    thundering herd).
  - **Recovery = hedged gap-fill, not resume.** If a grader dies or exceeds
    ~2× the median batch wall clock, dispatch fresh **sync** gap-filler
    agents covering ONLY its missing issue numbers (run-14: 6/6 hedges
    succeeded, zero redundant tokens because hedges never race live work).
    Resume-from-transcript does NOT recover terminal API deaths (0/3,
    run-14) — don't burn time on it. Every issue number in every batch must
    appear in the returned records; merge keep-first.
  - **Dispatch ledger (self-profiling — diagnostic, never a gate):** record
    per dispatch: grader id, mode, batch size, issue numbers, spawn/end
    timestamps, outcome (ok / timeout / hedged), tokens if the harness
    reports them — plus per-phase orchestrator timings (enumerate, merge,
    compose, gate, recompose count). Publish the ledger in the run report,
    and append it to the store as `beadle note <target> --topic perf`
    records (one per dispatch + one per phase summary).
- score-intent is MANDATORY per artifact (SKILL §4); corpus-level minutiae
  detectors run against the filer (§4b).
- Update the store (`store/<target>/state.jsonl`): append run record,
  issue observations, classification records (SKILL §6).
- Regenerate the FULL body (SKILL §7): carry forward all prior curated
  content (prior-run classification indexes, unchanged P2 clusters, keystone
  history); merge the new run's analysis into every section; bump
  `"run"` and `"watermark"` in the sentinel state JSON; refresh the digest.
- Read controls from the prior body before regenerating (SKILL §8).
- Direction Health carries the **falsification meter every run** (aae-orc#65):
  A4 SDL-engagement streak + the acted-on severity ceiling (hard/soft, per the
  pre-registered criteria in `_kos/ideas/ramp-vs-drift-phase-qualifier.md`) —
  reported whichever way it points, never only when it flatters a hypothesis.
- **Calibration is instrument v2 (SKILL §6b, capacity-adjusted):** verdicts,
  streaks, and latencies are conditioned on the manifest's
  `maintainer_capacity` block; streak denominators are attention windows
  first (runs/calendar as shadow); severity ceiling reports as selection
  depth with its cost covariate; a 🔴 headline requires a red
  capacity-adjusted band AND a falsified cost explanation.
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
5. **Markdown render integrity** (SKILL §7d contract; added after the
   run-14→16 attn-lane regression) — all mechanical:
   - no table line (`|…`) directly follows a blockquote line (`>…`) without
     a blank line between (GFM lazy continuation swallows the table);
   - every `<details><summary>…</summary>` is followed by a blank line;
   - every issue number in the sentinel's `attn` lists appears as a row in
     the visible Needs-human-reading table (before that section's
     `<details>`), never only inside the agent channel;
   - no `|`-table rows inside any `<details>` block without a header row.

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
   snapshot paths, and the falsification meter (A4 streak + hard/soft
   acted-on severity ceiling).
6. Append the run record (`kind: run`) to the store — the run-14 pass missed
   this; it is part of SKILL §6, not optional.
7. **Prediction protocol:** any due pre-registered prediction (store record
   `kind: note, topic: prediction`) is evaluated against its registered
   criteria exactly as written — report pass or fail in the run summary and
   on the board; do not defer, soften, or re-derive the criteria post-hoc.
   Evaluated results are permanent under the instrument that produced them
   (the run-16 ramp prediction FAILED under v1 and stays failed). NEW
   predictions must follow SKILL §6b rule 5: horizons in attention windows
   (default 4–6), single-window actions only; a horizon spanning < 2
   expected windows is invalid at registration.

## Guardrails (charter/incidents — non-negotiable)

- State out-of-band (B1): the issue body is a projection; never parse it as
  machine state beyond the sentinel/controls contract.
- Propose-not-act (B2): dashboard rewrite is allow-listed; public free-text
  comments and closures are NOT part of this command.
- Never auto-close on inactivity (G2).
- No Goodhart: counts always pair with outcome signals.
- The backup is sacred: keep every before-snapshot; never overwrite one.
