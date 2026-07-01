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

**Operational impact — does the defect stop the factory? Its own axis, set
autonomously (finding-009).** ORTHOGONAL to defect-nature: a one-character typo that
crashes the session is maximum operational impact at minimum nature-complexity, so it
**cannot** be inferred from nature or priority. Grounded in ODC's *Impact* attribute
(its `Reliability` value verbatim covers "ABEND and WAIT" = crash and hang); we split
ODC-Reliability along the failure-model spine. Set the `impact.*` label autonomously
(it is a classification, like `type.*`):
- **`impact.panic`** — the agent (Claude Code) session terminates abnormally; context
  is **lost**, recovery required. Crash / fail-stop. Worst recovery cost, but
  self-announcing.
- **`impact.halt`** — the run **pauses/blocks awaiting manual human intervention**;
  state is **preserved** (no crash). Liveness violation / ODC-WAIT. *Less* severe than
  panic on recovery cost, but **harder to detect** — the process looks healthy while
  making no progress (the gray-failure trap). This is the case the user flagged as
  under-tracked.
- **`impact.data-loss`** — destroys/corrupts irreplaceable state; the ODC-Impact analog
  of the silent-integrity recoverability tier. May co-occur with panic/halt.
- **`impact.degraded`** — runs but impaired (fail-slow/fail-stutter); not a stoppage.

This is the **liveness** top-axis; silent-integrity (above) is the **safety** top-axis.
They are independent — score BOTH, never let one mask the other. A defect can be
silent-integrity without stopping the factory (worst: invisible AND running), or
`impact.panic` without corrupting anything (loud but clean). `impact.panic`/`impact.halt`
are P0/P1 candidates by default (they stop autonomous operation) but priority stays a
distinct judgment. Distinct from `status.blocked`/`status.needs-human`, which are
triage-workflow states of the *issue*, not the runtime liveness of the *factory*. When
the factory itself reports a defect it should emit this signal (it has ground truth
beadle can only infer — finding-009 upstream contract); until it does, beadle infers
`impact.*` from the issue body + dispatcher/run telemetry.

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

For classification records specifically, use `beadle classify ingest <target>` —
pipe a JSON array of ClassificationRecord objects on stdin (or `--file <path>`).
The binary validates the four bounded enums (`report_type`, `defect_nature`,
`reproducibility`, `operational_impact`), enforces the HARD invariants
(`integrity=true` requires `integrity_anchor`; `quick_win_eligible=true` is
invalid on integrity items), and appends. If validation fails, fix the payload
and retry — the taxonomy holds by construction, not by discipline.

Direction signals B (integrity-density) and C (silent-data-loss share) derive
from these classifications for the current run — if you don't ingest, they emit
`pending` with a `reason` naming the gap (finding-014).

Signal A4 (silent-data-loss zero-engagement, finding-016) joins classifications
against comment events across **all** runs in the store. An issue classified
SDL that persists across ≥ 2 consecutive runs with zero `actor_role="maintainer"`
comments fires `watch`; ≥ 3 fires `drifting`. A single maintainer comment
drops the issue from the alarm entirely — silence is only silent when the
maintainer has not spoken. This is the signal that catches the exact case
beadle exists for; when it fires drifting alongside SDL-share drifting, it
takes precedence in `top_signal` because it names *which issues*, not just
how many.

`beadle render <target>` also consumes ingested classifications (finding-015):
each Open-issues row gains a chip column (`report_type · defect_nature ·
priority` + `⚠` integrity / `▲` silent-data-loss / `★` quick-win-eligible
flags), and a summary block above the table tallies integrity / SDL /
quick-win / P0-P1 counts + a per-report-type breakdown. Unclassified issues
render `_unclassified_` and the summary emits the propose-not-act pending
disclosure verbatim. The chip is a deterministic projection covered by the
sentinel's `derived_digest`; don't hand-edit it.

### 7. Regenerate the dashboard  (the primary goal)
Per `../../docs/dashboard-schema.md`: discover the pinned dashboard **sentinel-first,
title-second** — search the beadle identity's open issues for the `<!-- beadle-state -->`
block (machine-stable key) and the exact title `📋 beadle — Triage Dashboard`; take
the union. Exactly one (your authorship) → rewrite in place. **>1 → STOP and request
human consolidation** (a duplicate already exists; never pick one silently, never
create a third). Wrong author → STOP, don't fork. None → re-check immediately, then
create+pin.

`beadle render <target>` now emits a **Direction verdict** section as the first
derived-zone block (finding-017). It contains a deterministic projection of the
current run's `DirectionReport`: header line `<glyph> <verdict>` (🟢/🟡/🔴) +
top signal, plus a 4-row table with per-signal verdicts + rationale for
filing-density, integrity-density (B), silent-data-loss-share (C), and
silent-data-loss-zero-engagement (A4). When A4 fires, the row's detail cell
names the affected issues (`· drifting: #42, #87` or `· watch: #12`). The
editor still owns the free-text paragraph above the table — it goes in the
`<!-- editor:direction-verdict -->` slot and is preserved verbatim across
regens. Whole-section ownership: renderer owns the numbers/table, editor owns
the interpretation (`question-renderer-editorial-boundary` sub-question A).

Then rewrite the whole body from the store — Header (direction verdict, per
finding-017), Progress (stats + trend deltas, every count
paired with an outcome), Action plan, **Quick wins — safe to act on** (see step 7a),
**Direction Health** (minutiae ratio,
filed-vs-acted gap, scope-drift candidates), Classification index, **Maintainer
progress** (outcome-paired, not a leaderboard — see step 7b), Controls
(derived checkboxes), **Legend & references** (one collapsed footer — see step 7c).
Embed only a digest in `<!-- beadle-state -->`. Never parse
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

### 7a. Quick wins — safe to act on  (the low-caution lane)
Render a group **orthogonal to the P0/P1/P2 impact ordering** — *not* a fourth
priority tier. Its job: give maintainers the **obviously-broken, easy-fix,
low-blast-radius** issues they can use to **exercise their process** without the
caution the high-impact items demand. Reward the safe + obvious, not only the
critical. **Eligibility is DERIVED from the axes you already scored** (never a
hand-curated "easy" tag):
- **defect-nature** at the mechanical end (typo / wrong-variable / docs / one-line
  config / a bounded agent-prompt warning) — exclude design / spec / directional;
- **reproducibility** Bohrbug (consistent, isolatable) or a pure docs/wording change;
- **bounded blast radius** + **high fix-confidence** (the fix is cited + obvious);
- **alignment ≠ drifts** (never fast-track something that shouldn't be built).

**HARD EXCLUSION (finding-004) — non-negotiable.** An issue touching a
system-of-record `integrity_anchor` is **never** a quick win, *no matter how small
the triggering bug looks*. finding-004 is explicit: "a green check or a one-line diff
must never mask" a silent-integrity defect. The easy lane is precisely that trap;
integrity-gated items stay in the P0 source-of-truth cluster. Before placing any
issue here, check it against the manifest's `integrity_anchors` and the integrity
flag from step 3 — if either fires, it is disqualified.

**NO-GOODHART.** This is a *surfacing* device, not a metric. Never count "quick wins
closed" as success — that rewards trivial churn over real fixes. Maintainer progress
(7b) still moves only on verified fix-outcomes. Row format matches the action plan
(title leads, `#NN`, trailing chips); a folded agent-channel `<details>` may carry
the one-line fix sketch. **Cold-start (ADR-005):** this lane is the **cheapest first
maintainer pass** — resolving one is the lowest-risk way to turn the process and
start the baseline. Surface a handful (not the whole tail); link, never replicate.

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

### 7c. Legend & references  (one collapsed footer — light touch)
Decode the chip vocabulary **once**, at the foot, in a single collapsed
`<details><summary>Legend & references</summary>` — this is *why* §3–§5 rows stay
terse. **Do NOT** scatter footnote marks, superscripts, or `[1]`-style callouts
through the rows; stray symbols make the board harder to read, not easier. The footer
has two short parts:
- **Legend** — map only the chips/glyphs that **actually appeared this run** to a
  one-line meaning (verdict 🟢/🟡/🔴, the `Bohr/Mandel/Heisen` repro badge, `🛑 integrity`).
  Omit any row whose chip wasn't used.
- **References** — link only the **load-bearing industry standards** so a maintainer
  *can* read more: defect-nature → IEEE 1044 / ODC; reproducibility → Grottke–Trivedi;
  severity-vs-priority. Plain inline links in the footer, the citation given **once** —
  never repeated on the rows that use the term. Keep it to a few; don't catalog every
  source (`references.bib` is the full bibliography).

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
Post on an individual issue **only** where it clears the bar. Compose
`arcavenai-issue-review` for tone and the already-fixed discipline. Never
agreement-only. Soft, never hard/critical. Disclose bot authorship. Anything
irreversible (close / resolve) → escalate, never auto.

**The four passing lenses** (any one is sufficient):

1. **fixed-pending-release** — cite the merge commit + the tests that
   demonstrate the fix. The claim is verifiable against the repo.
2. **hallucinated-citation / fix-not-fixed** — the issue cites a file, line,
   function, or spec section that does not exist, or was edited to something
   that no longer supports the claim. Cite the exact drift.
3. **scope-drift** — the issue proposes work outside the target's declared
   intent (or duplicates existing scope). Cite the intent anchor.
4. **clear sibling cross-ref** — a sibling issue is measurably about the
   same defect or a strictly-adjacent one. Cite both numbers, name the
   difference in one sentence. `arcavenai`'s own follow-up comments on
   an issue it filed (self-annotations broadening scope) count as a
   sibling signal, not a comment we should imitate.

**The opinion / disambiguation distinction (item D).** A comment that
*expresses a view on the issue's worth, priority, or design merit* is
opinion — it does not clear the bar, no matter how correct it feels. A
comment that *disambiguates against cited artifacts* or *points at a
verifiable check the reader can run themselves* is disambiguation /
verify-hint — it can clear the bar as an instance of lens 2 or lens 4.

Sharpening test — apply to the draft before posting:

- **Would a maintainer be able to verify the claim in under 60 seconds
  against the code, spec, or issue graph?** If no, it is opinion. Do not
  post.
- **Does the comment name a specific artifact (file:line, commit sha,
  issue number, intent anchor)?** If no, it is opinion. Do not post.
- **Would the comment survive the fact that the maintainer likely already
  saw the issue?** Agreement-only ("this is important") always fails this.
  A verify-hint that adds one piece of *pointer* the maintainer did not
  already have passes.

Grounded examples on `drbothen/vsdd-factory` (finding-012):

- `#370` (CI static PASS): *opinion* would be "well-scoped, ship it";
  *verify-hint* would be pointing at the exact workflow file and line
  where the static echo lives so the maintainer can eyeball the fix
  shape in one click. Post the second, not the first.
- `#350`/`#380` (harness classifier vs subagent-decision): a comment
  that cross-refs the two with a one-sentence delta clears lens 4;
  a comment repeating "these are real problems" is opinion.
- `#381` (reference-oracle duplication): the author's own follow-up
  comment (2026-07-01) broadening scope to "test doc-comment overclaim"
  is exactly the *self-annotation* pattern — beadle notes it as a
  sibling signal (`arcavenai` self-thread) but does not imitate the
  broadening in beadle's voice.

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
