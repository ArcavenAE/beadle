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

## Discovery & idempotency

- **Find "its" issue by exact title**, not a stored number (Renovate's pattern):
  `📋 beadle — Triage Dashboard`. If none, create + pin it (`pinIssue`, ≤3/repo).
  Treat the title as a primary key.
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
3. **Action plan** — P0/P1/P2 tables: each row = issue link + recommended action
   + owner + alignment verdict chip. Top actionable items only.
4. **Direction health** (the differentiator) — minutiae ratio, scope-drift
   candidates, issues contradicting declared intent, over-engineering smells
   (premature abstraction, speculative config, build-what-you-might-need),
   self-referential-spiral flags, and the maintainer-engagement-vs-volume gap.
   **In cold-start (ADR-005), this panel reports structure to make the first pass
   cheap (high-leverage items, cluster candidates) and explicitly marks
   rate/drift signals as "not yet measurable — process not turned," never as a
   drift indictment.** Rate-based drift appears only once a baseline exists.
5. **Classification index** — human-readable table of all tracked issues with
   classification chips (type / priority / leverage / alignment / verdict).
6. **Controls** — checkbox lines the next run reads, acts on, and resets:
   ```
   - [ ] <!-- verb=fast-track;id=#NN --> Fast-track #NN
   - [ ] <!-- verb=investigate;id=#NN --> Deep-investigate #NN
   - [ ] <!-- verb=accept-deferral;id=#NN --> Accept deferral of #NN
   ```
   Checkboxes are **derived from authoritative state** each render, never authored
   by hand. On the next scheduled run, parse raw body for `- [x] <!-- verb=...;id=... -->`,
   dispatch the verb, then rewrite with the box cleared (eventually-consistent;
   never read-and-act in the same instant a human clicks). Frontier F5.

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
