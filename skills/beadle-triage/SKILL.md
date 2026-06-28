---
name: beadle-triage
description: Triage a target repo's GitHub issues against its declared intent and maintain a living dashboard issue. Use to run a triage pass on a beadle target (e.g. vsdd-factory), score issues for intent-alignment weighted by maintainer engagement, regenerate the pinned dashboard issue from out-of-band state, and post comments only where they clear the bar. The Phase-0 entry point for beadle; composes arcavenai-issue-review for per-issue discipline. Quality over quantity; propose-not-act for anything consequential.
---

# beadle — triage pass (Phase 0)

The minimal, useful form of beadle: one session runs the five engines as
sub-routines and maintains the dashboard. Read `../../charter.md` invariants and
`../../CLAUDE.md` before acting. **Every invariant there binds this skill.**

## Inputs

- **Target** — a key in `../../targets/` (default `vsdd-factory`). Load its
  `<target>.intent.yaml`. Refuse to score a manifest marked `draft-unpopulated`.
- **Watermark** — read from the dashboard issue's `<!-- beadle-state -->` block;
  if no dashboard exists yet, derive from arcaven-commented issues (bootstrap).

## Procedure

### 0. Load & sync
- Load the target intent manifest + its live `intent_sources` (for vsdd-factory:
  `CLAUDE.md`, `.factory/STATE.md`, `docs/`, git history, maintainer comments).
- Sync the local checkout to the true remote tip; if `git fetch` fails, read the
  true tip via the API and do **not** trust the local tree (the
  `arcavenai-issue-review` sync discipline). Note which commits you're behind.

### 1. Enumerate
- New since watermark + reopened + recently-maintainer-touched. Not only
  arcavenai — all, but weight arcavenai/arcaven highest (they reflect on the user).

### 2. validate  (read-only)
For each candidate: already-fixed-on-default? fix-not-fixed (is there a
load-bearing test, or is it a paper-fix)? do the cited symbols/files actually
exist (catch hallucinated citations)? reproducible? → a verdict.

### 3. classify  (read-only)
Two axes — severity (impact) and priority (urgency); a leverage axis
(systemic ↔ minutiae); default priority low, escalate on evidence. Re-classify
type; don't trust the body's self-label.

### 4. score-intent  (read-only)
Grade alignment against the manifest's `alignment_rubric` (advances / neutral /
drifts) **with cited rationale**. Confidence as frequency, never fluent certainty.

### 5. investigate  (read-only, selective)
Only for ambiguous / high-stakes / contested issues: a short memo (what's true,
what a fix touches, risks, recommended action).

### 6. Update the store
Append the run's records (issue, verdicts, classification, alignment, maintainer
events) to the out-of-band store (Phase 0: JSONL). The store is the source of
truth — never the dashboard body.

### 7. Regenerate the dashboard  (the primary goal)
Per `../../docs/dashboard-schema.md`: find the pinned `📋 beadle — Triage
Dashboard` issue by title (create+pin if absent), rewrite the whole body from the
store — Header (direction verdict), Progress (stats + trend deltas, every count
paired with an outcome), Action plan, **Direction Health** (minutiae ratio,
filed-vs-acted gap, scope-drift candidates), Classification index, Controls
(derived checkboxes). Embed only a digest in `<!-- beadle-state -->`. Never parse
the body as state; tolerate a wiped body.

### 8. Read controls from the prior body
Parse `- [x] <!-- verb=...;id=... -->` lines, dispatch the verb (fast-track /
investigate / accept-deferral), then reset the box on regeneration. Eventually
consistent — never read-and-act in the same instant.

### 9. Comment  (propose-not-act; high bar)
Post on an individual issue **only** where it clears the bar: fixed-pending-release
(cite commit + tests), hallucinated-citation / fix-not-fixed, a scope-drift
concern, or a clear sibling cross-ref. Compose `arcavenai-issue-review` for tone
and the already-fixed discipline. Never agreement-only. Soft, never
hard/critical. Disclose bot authorship. Anything irreversible (close / resolve) →
escalate, never auto.

## Output

Report: target + watermark→new-watermark; the dashboard issue URL; which issues
got comments (one-line rationale each); which were intentionally silent and why;
and the run's direction verdict (on-course / watch / drifting) with the
top contributing signal.

## Guardrails (from charter / incidents)

- State out-of-band; body is a projection (B1/G1).
- Propose-not-act for consequential/public; every public post clears the
  human bar (B2).
- Never auto-close on inactivity (G2). Information density is a protection signal.
- No Goodhart — never chase close-rate; pair counts with outcomes.
- Maintainer engagement is the compass (B3).
