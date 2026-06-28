# The issue-as-dashboard

beadle maintains one pinned GitHub issue per target repo as a living dashboard.
The mechanics emulate Renovate's Dependency Dashboard [@renovatedash] while fixing
its one fatal flaw.

## The fatal flaw we fix: state-in-Markdown

Renovate parses its own rendered issue body as machine state; clearing or editing
the body leaves it "in a broken state" (renovatebot/renovate#19563) [@renovate19563].
Every bot-maintained dashboard studied shares this failure mode. **beadle's rule
(B1 / ADR-001 / charter G1):**

> Durable state lives out-of-band. The issue body is a **regenerated projection**.
> beadle never trusts a hand-edit and tolerates a wiped body by regenerating
> cleanly from the store.

## Dual-audience rendering: LLM-first, human-supported

The dashboard is **mainly consumed by LLM/agent sessions** (other Claude Code runs
orienting on the target) and **secondarily by humans**. These two audiences have
*inverted visibility*, so one document serves both via three channels:

- **Human channel — the rendered surface.** Humans will not hover, will not expand
  `<details>`, will not read HTML comments. So the visible render must be scannable:
  **short human title first** (a few words — *what the issue is*), then minimal chips.
- **Agent channel — folded / embedded, free to the LLM.** An agent reads the **raw
  markdown source**, so collapsed `<details>`, link titles, and HTML-comment payloads
  are *instantly visible at zero cost* — folding is not hiding for an agent. This
  channel carries the **beadle-computed judgments that exist nowhere else**: the axis
  vector (report-type, defect-nature, reproducibility, leverage, alignment, priority),
  the verdict rationale with citations, and the integrity flags. Put it in a per-issue
  `<details>` block or a structured comment payload so it's out of the human's way but
  one parse away for the agent.
- **Reference channel — links the LLM follows.** Issue numbers, PR refs, and commit
  shas are **fetchable resources**: an agent can follow `#314` and read the full body
  on demand. So the dashboard **never replicates issue detail** — that would violate
  B1 (the body is a projection, not a replica) and waste the 65 k budget. Anything
  recoverable by following a reference stays a reference; the dashboard carries only
  the *differentiating* payload an agent needs to decide whether to follow it.

The rule: **surface short titles for humans; embed/fold the axis vector + rationale
for agents; link (never copy) anything an agent can fetch.** Each audience gets its
ideal view from the same artifact — not a compromise between them.

## Discovery & idempotency

Discovery is **sentinel-first, title-second** — keep both, but the machine-stable
key wins. The goal is one dashboard per target repo, never a duplicate:

- **Primary key — the `<!-- beadle-state -->` sentinel.** Search open issues by the
  beadle identity for a body containing the `beadle-state` sentinel block (and,
  inside its JSON, a matching target). This survives a hand-edited title, which the
  bare title does not.
- **Secondary key — exact title.** `📋 beadle — Triage Dashboard` (the bare string;
  the `· owner/name` suffix lives only in the body H1, never the issue title).
  Used to find a dashboard whose body sentinel was wiped, and as the create-time
  title.
- **Candidate set = union of both, filtered to the beadle identity.** Then:
  - **exactly one** authored by the beadle identity → rewrite it in place;
  - **>1** → **STOP, report all candidates, request human consolidation.** Never
    pick one silently; never create another. (Drift has already happened — a
    second dashboard exists — and only a human should collapse them.)
  - **one, wrong author** → STOP (do not edit another author's issue, do not fork);
  - **none** → **re-check immediately before** `gh issue create` (narrows the
    create race), then create + pin it (`pinIssue`, ≤3/repo).
- **Closed dashboards are intentionally out of scope** (`--state open`). A closed
  same-title issue frees the title for a fresh dashboard (this is how
  vsdd-factory#311, mis-authored by `arcaven`, was retired). This is by design, not
  a miss.
- **Concurrency caveat:** re-check-before-create narrows but does not close the
  TOCTOU window. Two concurrent runs against one target can still both create. A
  real guarantee needs a serialized chokepoint (INC-003 class) — frontier
  (`question-dashboard-discovery-robustness`), relevant at Phase 1 (scheduled
  gh-aw) and Phase 2 (marvel single-writer orchestrator), not Phase 0.
- **Rewrite the whole body each run**; never append comments. History is the
  issue's edit history.
- Stay under the **65,536-char body limit**; offload detail to linked sub-issues
  or the store; use `<details>` for ergonomics (collapsing does not reduce the
  char count).

## State block (digest only, not the record)

A single sentinel-delimited HTML comment carries a *digest + pointer*, never the
authoritative data:

```
<!-- beadle-state:v1
{"schema":1,"last_run":"<ISO8601>","watermark":<n>,"store":"<jsonl|dolt-ref>",
 "digest":"sha256:...","counts":{"open":N,"acted":N},"trend_ref":"<id>"}
beadle-state -->
```

The real records (issues, classifications, alignment verdicts, maintainer-event
log, historical stats) live in the store. Phase 0: append-only JSONL in the repo
or a gist. Phase 2: Dolt (versioned SQL — diffable, branchable, auditable;
frontier F2).

## Body sections (rendered top-to-bottom)

1. **Header** — last updated, watermark, run count, target intent version, a
   one-line **direction verdict**. The verdict is **gated on warm-up state**
   (ADR-005): until a baseline exists (≥1 completed maintainer triage cycle, or a
   configured threshold) it is **`COLD START / BASELINE`**, never `DRIFTING` —
   an average rate over a pre-engagement window is initialization bias, not drift.
   After the process has turned, the verdict becomes `on-course` / `watch` /
   `drifting`.
2. **Progress** — open/closed by author (arcavenai vs maintainers vs others), by
   type, by priority, by validation verdict; **trend deltas** since last run;
   net-flow (intake − close); filed-vs-acted ratio; backlog age distribution.
   Every count paired with an outcome signal (no vanity metrics).
3. **Action plan** — P0/P1/P2 tables. **Each row leads with a short human title**
   (a few words — *what the issue is*), then `#NN` (the agent's fetch reference), then
   recommended action + owner, with type / reproducibility / **alignment-verdict as
   trailing chips** (finding-005). The verdict is *status* (drill-in detail), never the
   row headline — a human scans for what an issue is and will not hover over a bare
   number. Top actionable items only. **Agent channel:** the per-row verdict *rationale*
   (cited reason the action is recommended, integrity flags) rides in a folded
   `<details>` under the row — instantly visible to an LLM, out of the human's way —
   and never restates the issue body, which the agent fetches via `#NN`.
4. **Direction health** (the differentiator) — minutiae ratio, scope-drift
   candidates, issues contradicting declared intent, over-engineering smells
   (premature abstraction, speculative config, build-what-you-might-need),
   self-referential-spiral flags, and the maintainer-engagement-vs-volume gap.
   **In cold-start (ADR-005), this panel reports structure to make the first pass
   cheap (high-leverage items, cluster candidates) and explicitly marks
   rate/drift signals as "not yet measurable — process not turned," never as a
   drift indictment.** Rate-based drift appears only once a baseline exists.
5. **Classification index** — human-readable table of all tracked issues. **Each
   row leads with the short human title**, then `#NN`, then classification chips:
   **report-type** (Issue Type / kind/*) · **defect-nature** (mechanical→conceptual,
   IEEE 1044 / ODC) · **reproducibility** (Bohr/Mandel/Heisen badge) · priority ·
   leverage · alignment · verdict (finding-005). Defect-nature and report-type are
   the two primary facets; reproducibility is a badge feeding the effort estimate.
   **Agent channel:** the full axis vector for each issue (every scored axis, not just
   the charted facets) lives in the folded `beadle-state` digest / store reference, so
   an agent reading raw body has the complete classification without fetching, while the
   human table stays scannable. The index is an *index* (B1) — it points, it does not
   replicate the issue.
6. **Maintainer progress** — outcome-paired progress, **not a leaderboard**
   (`question-maintainer-progress-gamification`). Make resolution against
   beadle-discovered defects visible and rewarding, but reward the **verified
   fix-outcome** (flagged → fixed → validates), never close-rate / time-to-triage /
   volume (B3 + no-Goodhart). E.g. "N integrity defects closed with a load-bearing
   fix," "longest-standing P0 resolved," "cycles since last drift" — each count
   paired with its outcome. No gameable per-human ranking. Cold-start (ADR-005):
   show structure, withhold streak/rate claims until the process has turned.
7. **Controls** — checkbox lines the next run reads, acts on, and resets. **Two
   tiers** (F5 + `question-maintenance-request-controls`):
   ```
   # Tier 1 — per-issue verbs (bounded to one artifact)
   - [ ] <!-- verb=fast-track;id=#NN --> Fast-track #NN
   - [ ] <!-- verb=investigate;id=#NN --> Deep-investigate #NN
   - [ ] <!-- verb=accept-deferral;id=#NN --> Accept deferral of #NN

   # Tier 2 — board-level maintenance requests (whole-corpus, expensive, on-demand)
   - [ ] <!-- verb=reprioritize;id=board --> Re-rank the action plan
   - [ ] <!-- verb=full-refresh;id=board --> Re-enumerate & re-triage every open issue
   - [ ] <!-- verb=revalidate;id=board --> Re-run validate across the whole corpus
   - [ ] <!-- verb=rescore-intent;id=board --> Re-run score-intent corpus-wide (after a manifest change)
   ```
   Checkboxes are **derived from authoritative state** each render, never authored
   by hand. On the next scheduled run, parse raw body for `- [x] <!-- verb=...;id=... -->`,
   dispatch the verb, then rewrite with the box cleared (eventually-consistent;
   never read-and-act in the same instant a human clicks).

   Tier 2 is the **maintainer-triggered maintenance-request** surface: a human checks
   a box to pull an expensive routine forward without waiting for the next full
   scheduled run. Resetting the box on dispatch is the de-bounce (a box left checked
   across passes must not re-run). Tier-2 routines are read/analyze/regenerate only —
   an irreversible public action (close / resolve / free-text comment) still escalates
   per B2, never a checkbox. A **cheap-poll pass** (Phase 1, gh-aw cron) can fetch only
   this body and parse Tier-2 boxes to decide whether any costly routine was requested,
   escalating to the expensive act pass only on a hit. Frontier F5 + the maintenance-
   request question.

## Identity, cadence, hardening

- Runs **as the `arcavenai` identity** (Phase 1+: a GitHub App for a clean audit
  trail, 15k/hr limits, and to break the `GITHUB_TOKEN` re-trigger guard) [@ghaw].
- Driven by a **schedule** (cron is UTC; avoid the top of the hour). No internal
  timers — heartbeat-via-cron, the multiclaude pattern.
- **Throttle below GitHub secondary rate limits**; cap comments per issue / per
  run with backoff on 403/429 [@ghacceptableuse].
- A wiped or vandalized body is non-fatal: regenerate from the store.

## Why an issue, not a Project

A single rewritten Markdown issue is the right human-readable surface with
interactive checkbox controls (Projects v2 can't do checkbox-as-control in a
body). Projects v2 is a *complement* for structured cross-repo rollups via
GraphQL custom fields — reach for it only when that need is concrete (gradual
elaboration), never as the interactive surface.
