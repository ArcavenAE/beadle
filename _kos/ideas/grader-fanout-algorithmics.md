# Grader fan-out algorithmics — the refresh pass is an unstudied distributed system

**Status:** idea (pre-hypothesis) → extracted to
`question-grader-fanout-efficiency`
**Trigger:** run-14 (2026-07-20) — 17 parallel grader subagents, > 43 minutes
wall clock and counting, for ~80 new issues. Run-13: 8 graders × 13
issues/batch, 2 of 8 stalled ~1 hour, recovered only by a manual resume ping.
The operator's read: "I suspect we're doing some basic and dumb things here."

## The recognition

The `/dashboard-refresh` classify step is a distributed map-reduce over
expensive, unreliable workers — and we designed it by feel, not by the
literature. Every one of its pain points is a *named, studied problem* in
computer engineering:

| What we observe | What the field calls it | Where it's studied |
|---|---|---|
| Run wall-clock = slowest grader; 2/8 stalled 1h | Tail latency / stragglers | Dean & Barroso, *The Tail at Scale* (CACM 2013); MapReduce backup tasks (Dean & Ghemawat, OSDI 2004) |
| Manual resume ping to recover a stall | Missing heartbeat + speculative re-execution | Same; also work-stealing schedulers (Blumofe & Leiserson, Cilk) |
| Batch of 13 re-runs because one issue flaked | Retry granularity / unit-of-work mismatch | Task-parallel runtime design |
| 17 agents each loading the same manifest + rubric + repo context | Fixed-cost amortization; shared-prefix batching | LLM prompt-cache economics; setup-cost batch sizing (EOQ-shaped tradeoff) |
| 429/503 kills at spawn (2026-07-05, run-13) | Thundering herd / burst synchronization | Client-side rate shaping, jittered start |
| Full-body regeneration every run, carrying all prior content through an LLM composer | Full recomputation of a materialized view | Incremental view maintenance; differential dataflow (McSherry); self-adjusting computation (Acar) |
| Composer dropped 67 refs; gate caught it; recompose | The renderer is a regression vector — because it recomputes what didn't change | Same — incrementality is also a *correctness* device |
| Gate check 3 re-verifies closure of every issue ever referenced | O(corpus) verification per run, O(n²) cumulative | Amortized/incremental invariant checking |
| Every issue gets the full expensive grader | Missing cheap-first cascade | Classifier cascades (Viola-Jones lineage); modern LLM model-cascade routing |

None of this is exotic. It is TAOCP-grade discipline applied to a new
substrate: *measure first* (Knuth's analysis-of-algorithms stance, vol. 1),
then choose the schedule, the batch size, and the incremental structure from
the measured cost model — not from vibes.

## What the artifacts alone reveal (no profiling needed)

From `.claude/commands/dashboard-refresh.md` + SKILL §3–§7 + finding-020:

1. **Parallelism is between agents, serial within.** A grader session
   processes its 12–15 issues sequentially (one LLM turn stream, per-issue
   tool calls). Wall clock per grader ≥ batch_size × per-issue latency.
   The batch exists to amortize the fixed prompt (manifest + rubric), which
   is a *prompt-caching* concern being solved with a *scheduling* decision.
2. **No straggler policy.** Nothing watches grader liveness; nothing
   re-dispatches a stalled batch's remaining issues. The command says
   "retried/resumed, never silently dropped" — the mechanism is a human
   noticing.
3. **The retry/accounting unit is the batch, but the correctness unit is the
   issue** ("every issue number in the batch must appear"). Unit mismatch.
4. **Read amplification.** Each grader independently reads issue bodies (and,
   for validate-flavored checks, the target repo tip). One orchestrator-side
   snapshot (bodies + tip SHA + relevant file excerpts) handed to graders
   would cut both API traffic and the 429-herd risk.
5. **No difficulty routing.** Run-13 was 85% `process-gap`; report-type is
   "cheap to auto-classify" per the model. A cheap pre-pass (or store priors)
   could pre-fill easy axes and route only ambiguous / integrity-suspect /
   attn-flagged issues to the deep grader. The quality gate for such a
   cascade needs design (spot-check sampling; never cascade integrity calls).
6. **Render is full-recompute.** §7 regenerates everything each run. The
   store is append-only; the dashboard is a view. The studied answer is
   derived-sections + authored slots (exactly where `u47p` fail-loud render,
   `iouu` curated zones, and `question-renderer-editorial-boundary` were
   already pointing) — recompute only what the new run touches.
7. **Nothing is instrumented.** The store records runs/issues/classifications
   but no timings, token counts, tool-call counts, retries, or stall events.
   We cannot distinguish "batch too big" from "stragglers" from "serial tool
   loops" without data. beadle finding-007 (OTEL attribution) already has a
   branch; the cheap alternative is per-grader perf `note` records in the
   store.

## What only profiling can tell us (the ask to Skippy)

- Per-grader: spawn ts, first-token ts, end ts, #issues, #tool calls,
  #gh API calls, tokens in/out, retries, stalls. Per-issue within grader:
  start/end ts. Orchestrator: enumerate/fetch time, merge time, compose
  time, gate time, recompose count.
- Run-14 specifics: why 17 agents for ~80 issues (batch size ~5? extra
  roles — validate/score-intent split out?); what the 43+ min is actually
  spent on.

## Candidate reading list (for the deep-research probe)

- Knuth, TAOCP vol. 1 (analysis discipline; measure before optimizing),
  vol. 3 (the sorting/searching machinery behind indexes and merges).
- Dean & Ghemawat, *MapReduce* (OSDI 2004) — backup tasks.
- Dean & Barroso, *The Tail at Scale* (CACM 2013) — hedged requests.
- Blumofe & Leiserson, *Scheduling Multithreaded Computations by Work
  Stealing* (JACM 1999).
- McSherry et al., *Differential Dataflow* (CIDR 2013); Acar,
  *Self-Adjusting Computation* (thesis, 2005) — incremental view
  maintenance for the render side.
- Viola & Jones (2001) cascades → modern LLM cascade/routing papers
  (FrugalGPT lineage) — cheap-first classification with escalation.
- Queueing basics (M/G/c) for choosing worker count vs batch size under a
  provider rate limit.

## Guardrails the optimization must respect

- Classification QUALITY is the product; wall clock is a constraint.
  No cascade or batch change ships without a quality gate (sampled
  double-grade agreement vs the deep grader; integrity/attn axes never
  cascade to the cheap tier).
- No-Goodhart: perf metrics are diagnostic, not gates
  (aae-orc ADR-007 / diagnostic-not-gate).
- Propose-not-act and the regression gate stay; incrementality must make
  the gate *cheaper*, never weaker.
