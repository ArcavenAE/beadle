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
Re-classify on every axis; **don't trust the body's self-label.** The model is a
superset of research-grounded axes (finding-005) + beadle-original axes. Default
priority low, escalate on evidence.

**Research-grounded axes** (deep-research `we6yrrcba`; "defect" is the industry
term — IEEE 1044 / ODC):
- **report-type** — the surface form. Primary carrier: GitHub **Issue Types**
  (Task / Bug / Feature, org-level, distinct from labels). Long tail: a `kind/*`-style
  family (regression, security, dependency/CVE, ci/build, flaky-test, tech-debt,
  perf, docs, question, RFC). Submitter-assertable, cheap to auto-classify.
- **defect-nature** — *what is actually wrong*, on the mechanical→conceptual spectrum
  (IEEE 1044 anomaly classes / ODC Defect Type): syntax/typo/wrong-variable →
  off-by-one/boundary → null/resource/lifecycle → concurrency/race → logic →
  algorithmic → spec/requirements → design/architectural → **directional /
  intent-misalignment** (code correct, wrong thing built). Mechanical end =
  auto-classifiable, decide-in-seconds, demand a one-line repro + fix sketch.
  Conceptual end (design/spec/directional) = needs a human architectural decision,
  demand cited rationale against the intent anchor, **escalate — never auto-resolve.**
- **reproducibility class** — Bohrbug (consistent, isolatable → cheap) | Mandelbug
  (complex activation/propagation, not systematically reproducible → investigate) |
  Heisenbug (changes under observation → investigate). *The single most
  decision-relevant axis for evaluation cost* — it **feeds priority's
  level-of-effort term** (don't estimate effort by gut) and routes escalation. Demand
  a reliable repro + environment + trace for Mandelbug/Heisenbug; flag unknown-nature
  reports for further triage (Rust `regression-untriaged` pattern).
- **triage-state** — needs-triage → accepted → needs-information. The lifecycle lane,
  distinct from the validate-verdict.

**beadle-original axes** (ahead of / beside the literature, finding-005):
- **leverage** (systemic ↔ minutiae), **alignment** (advances ↔ drifts — B4, the
  "directional/intent" class the research logs as an unsolved open question),
  **provenance** (pilot-derived ↑ vs speculative ↓), and **blast-radius visibility ×
  compounding** (finding-004; FMEA-detectability is its citable analog).

Surface report-type and defect-nature as the two primary facets; render
reproducibility as a badge; keep severity (impact) and priority (urgency+effort)
distinct. The finding-004 integrity blocks below **gate** all of these — an open
source-of-truth integrity defect outranks any functional defect on the same substrate
regardless of its report-type or defect-nature.

**Silent integrity / source-of-truth corruption — top severity, always escalate
(finding-004).** A fault that makes a *system of record* (ratchet, spec, hash,
index, decision log, learning store) disagree with reality **without raising a
signal**, and propagates to dependent cycles/artifacts, is the **highest** severity
class **regardless of how small the triggering bug looks**. The severity comes from
invisibility + compounding, not from the size of the immediate loss. A green check
or a one-line diff must never mask it. (vsdd-factory #313/#314 are this class — the
ratchet certifies PASS against state that doesn't exist on disk; the hash disagrees
with reality. They are source-of-truth corruption, not a "CI cluster.")

**Recoverability is a severity input.** Rank what is at risk by recoverability, not
byte count: regenerable **output** (re-run the generator — cheapest) < **spec /
process** (the authority code is judged against; only a human amends it) <
**learning** (adversarial findings, decision log, convergence history, what was
ruled out — *irreplaceable*; cannot be cheaply re-derived). A fault threatening the
irreplaceable tier outranks one risking only regenerable output, even at equal size.
For a self-referential factory, code is the cheapest thing it produces; the learning
and spec are the expensive, irreplaceable output — and are exactly what
"produced-but-not-committed" / silent-overwrite bugs destroy.

**Integrity gates functional — a precedence rule, not just an ordering
(finding-004).** Integrity is *foundational*; convergence, gate-correctness, feature
behavior are *functional*. Functional properties are **computed over** the substrate
(ratchet state, spec, hash, learning store). If the substrate can't be trusted, a
functional verdict — including a "converged / PASS" — is **unfalsifiable**: correct
code on a false substrate still yields a false result, and a green check certifies
nothing. So a source-of-truth integrity defect does not merely *outrank* a
convergence-soundness defect on impact — it **gates the validity of every functional
verdict, convergence included.** In the action plan, the **Source-of-truth
integrity** group is always P0 and sits **above** convergence-soundness; never let a
functional item (however important) be ranked above an open integrity defect on the
same substrate it depends on.

### 4. score-intent  (read-only) — MANDATORY PER ARTIFACT, never skipped
This is beadle's differentiator (B4). You MUST read each enumerated issue's BODY
and grade it — title-level slotting is a defect, not a shortcut. For every issue:

1. **Read the manifest's target semantics FIRST**, before applying the rubric:
   - `self_referential: true` → engine/process/meta issues are **on-mission**;
     do NOT flag `process-gap(...)`/`meta(...)` as off-mission self-reference. Judge
     on leverage + provenance. (For vsdd-factory the engine IS the product.)
   - `provenance_signal` → infer **pilot-derived** (cites a real commit/file/run +
     reproduction) vs **speculative** (invented machinery, no observed failure).
     Pilot-derived ranks ABOVE; speculative ranks toward `drifts`.
2. Grade alignment against `alignment_rubric` (advances / neutral / drifts) **with
   cited rationale** — quote the rubric line and cite the issue's evidence.
3. Confidence as frequency, never fluent certainty.

Group by **intent semantics, not surface keywords**: a drift-soundness bug, a
data-safety bug, and a framework-cost study are three different buckets even if all
three say "CI" in the title.

### 4b. corpus-level minutiae  (read-only) — run the detectors against the FILER
Beyond per-issue scoring, compute the manifest's `minutiae_signals` across the
whole measured-contributor corpus. The "led-by-the-backlog" pattern (N granular
issues, M<<N maintainer actions) is a property of the *filing pattern*, not any one
issue — and it holds **even when every individual issue scores "advances."** Surface
it on Direction Health. (Cold-start ADR-005: report the structural ratio as
baseline; withhold the rate/trend claim until the process has turned.)

### 5. investigate  (read-only, selective)
Only for ambiguous / high-stakes / contested issues: a short memo (what's true,
what a fix touches, risks, recommended action).

### 6. Update the store
Append the run's records (issue, verdicts, classification, alignment, maintainer
events) to the out-of-band store (Phase 0: JSONL). The store is the source of
truth — never the dashboard body.

### 7. Regenerate the dashboard  (the primary goal)
Per `../../docs/dashboard-schema.md`: discover the pinned dashboard **sentinel-first,
title-second** — search the beadle identity's open issues for the `<!-- beadle-state -->`
block (machine-stable key) and the exact title `📋 beadle — Triage Dashboard`; take
the union. Exactly one (your authorship) → rewrite in place. **>1 → STOP and request
human consolidation** (a duplicate already exists; never pick one silently, never
create a third). Wrong author → STOP, don't fork. None → re-check immediately, then
create+pin. Then rewrite the whole body from the store — Header (direction verdict), Progress (stats + trend deltas, every count
paired with an outcome), Action plan, **Direction Health** (minutiae ratio,
filed-vs-acted gap, scope-drift candidates), Classification index, **Maintainer
progress** (outcome-paired, not a leaderboard — see step 7b), Controls
(derived checkboxes). Embed only a digest in `<!-- beadle-state -->`. Never parse
the body as state; tolerate a wiped body.

**Row legibility — title leads, verdict trails (finding-005).** A human scanning the
board orients on *what an issue is*, not its number, and will not hover. Every
Action-plan and Classification-index row MUST lead with a **short human title** (a
few words — what the issue IS), then `#NN`, then chips. The alignment **verdict is
status** (drill-in detail), rendered as a trailing chip — never the headline. Format:
`<short title> (#NN) · <type> · <repro badge> · <verdict chip>`. Do not spend the
row's first, most-scanned characters on a verdict.

**Dual-audience — LLM-first, human-supported (per `../../docs/dashboard-schema.md`).**
The dashboard is MAINLY read by LLM/agent sessions; humans are the secondary audience.
The two see *inverted visibility*, so render three channels in one body:
- **Human channel (visible render):** short title + minimal chips, as above.
- **Agent channel (folded/embedded — free to the LLM):** the beadle-computed judgments
  that exist nowhere else — the full axis vector, the cited verdict rationale, integrity
  flags — go in a per-row `<details>` block or the `beadle-state` payload. An LLM reading
  raw markdown sees them instantly; the human doesn't have to. Folding is NOT hiding for
  an agent.
- **Reference channel (links the agent fetches):** `#NN`, PR refs, commit shas. The
  dashboard **never replicates the issue body** — an agent follows the reference (B1: the
  board is a projection/index, not a replica). Carry only what differentiates; link
  everything an agent can fetch. This also protects the 65 k body budget.

### 7b. Maintainer progress  (outcome-paired — NOT a leaderboard)
Surface what maintainers have *resolved against beadle-discovered defects* to make
progress visible and rewarding (`question-maintainer-progress-gamification`). This
is **constrained by B3 + no-Goodhart**: reward the **verified fix-outcome** (a
defect beadle flagged → maintainer fixed → the fix validates), never raw close-rate,
time-to-triage, or volume. Concretely: "N source-of-truth-integrity defects closed
with a load-bearing fix," "longest-standing P0 resolved," cycles since last drift —
each a count **paired with its outcome signal**. No per-human ranking that could be
gamed by closing-without-fixing; the metric must move only when real defects get
real fixes. Cold-start (ADR-005): show structure now, withhold rate/streak claims
until the process has turned. Frontier — render a minimal honest version; the full
reward design is open.

### 8. Read controls from the prior body — two tiers
Parse `- [x] <!-- verb=...;id=... -->` lines, dispatch, then reset the box on
regeneration. Eventually consistent — never read-and-act in the same instant a
human clicks; act on the NEXT pass. Two tiers exist
(`question-maintenance-request-controls`):

- **Tier 1 — per-issue verbs** (`id=#NN`): `fast-track` / `investigate` /
  `accept-deferral`. Bounded to one artifact.
- **Tier 2 — board-level maintenance requests** (no `id`, or `id=board`): the
  maintainer is pulling an expensive whole-corpus routine forward on demand —
  `reprioritize`, `full-refresh`, `revalidate`, `rescore-intent`. Dispatch the
  requested routine over the whole target, then reset the box. **De-bounce: reset
  the box on dispatch so a box left checked across passes does not re-run.** These
  are read/analyze/regenerate only — never an irreversible public action (close /
  resolve / free-text comment), which still escalate per B2.

**Cheap-poll pattern (Phase 1, gh-aw cron — render now, schedule later):** a poll
pass may fetch ONLY the dashboard body and parse Tier-2 boxes to detect whether any
human has requested an expensive routine, escalating to the act pass only when one
is checked. Phase 0 is single-session, so render the Tier-2 controls today; wiring
the separate cheap poll cadence is frontier.

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
- **Intent fidelity (B4).** score-intent is mandatory per artifact and reads the
  manifest's target semantics (`self_referential`, `provenance_signal`) before the
  rubric. Title-level slotting without grading the body is a defect. Group by intent
  semantics, not surface keywords. Run the corpus-level minutiae detectors against
  the measured filer, not only per-issue.
