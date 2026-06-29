# finding-010 — the dark-factory metric canon, and the telemetry that feeds it

Date: 2026-06-28 (session-051)
Probe: the user asked, over several turns, what metrics an AI team / a dark
factory / an AI project should be measured on; what cost & "Claude billing
autopsy" signals exist; what CI, project-planning/visualization, AI-harness
optimization ("fast-path for the harness"), per-platform telemetry, and
time-allocation data we should capture. Eight parallel research strands ran
against primary sources (DORA/SPACE/Flow/Lean canon, SWE-bench/τ-bench/METR,
reliability literature, Anthropic/AWS/GCP docs, OTel GenAI conventions).

**Scope ruling first (finding-008): this is critic's canon, not beadle's.**
Every metric below requires *more than one run* or a *trend across time* — the
finding-008 tell. beadle does not compute these. beadle's only stake is the two
classification axes it already owns (`impact.*` liveness, silent-integrity
safety) and *surfacing* the telemetry gaps that block the canon (it already did:
finding-007). This finding is the reference critic builds against; it lives in
beadle's graph only because beadle surfaced the question and owns the two axes
the whole canon is gated on.

Nodes touched: new `question-dark-factory-metric-canon` (frontier, critic-scope);
composes `elem-operational-impact-axis` (009), `elem-silent-integrity-severity`
(004), `elem-defect-classification-superset` (005), and findings 007 (OTEL gap)
+ 008 (the boundary). Cross-refs `ArcavenAE/critic` and orc vision.md (the
nine-layer stack, the "Claude billing autopsy" framing).

---

## 0. The one rule beneath the whole canon

Independently re-derived by every subfield surveyed — METR's perception gap,
Perry's overconfidence finding, the reward-hacking literature, Goodhart construct
validity, DORA's throughput-vs-instability pairing, and beadle's own no-Goodhart
invariant (#5):

> **Pair every count with an independent, agent-inaccessible outcome signal, and
> never optimize a single metric as the target.**

Self-report, code-suggestion acceptance rate, and "tests pass" are *perception /
proxy* signals that the AI-specific evidence shows are systematically gamed or
wrong. They belong on the dashboard only chained to a verification the agent
cannot see or influence. This is the same shape as the business-case constraint
the user set: *capturing real quality issues justifies more Anthropic use; a
false or inaccurate report is a penalty against us.* A metric the factory can
game is a false report waiting to happen.

---

## 1. The precedence stack — what gates what

The factory's metrics are not a flat list; they form a precedence order, and
reading them out of order produces false confidence. This is the B9 family
generalized:

```
  SAFETY (silent-integrity, finding-004)  ─┐
                                           ├─ must be clean BEFORE
  LIVENESS (impact.*, finding-009)        ─┘   convergence/efficiency
                                                metrics mean anything
              │
              ▼
  CONVERGENCE / EFFICIENCY (FPY, rework, net-defect-flow, passes-to-converge)
              │
              ▼
  COST / TIME (billing autopsy, time-budget, cost-per-outcome)
              │
              ▼
  DELIVERY (DORA, flow, throughput)  ── only honest once all above are sound
```

Rationale: convergence/efficiency metrics are computed *over* the substrate. If
the substrate is silently false (safety) or the loop isn't actually running
(liveness), an efficiency number is measuring a corrupt or halted process and
reads as healthy. Cost-per-outcome divides by "outcome" — if "outcome" is a
test-passed-but-not-resolved patch (B4 below: ~67% of public SWE-bench "passes"
didn't truly resolve), the denominator is a lie. **Verify integrity and liveness
first; only then trust the rates.**

---

## 2. SAFETY & LIVENESS — the two top axes (already bedrock; the rest builds on them)

Both already adopted in beadle (findings 004, 009) and the label schema. Critic
turns each into a *rate across runs*:

| Axis | beadle classifies | critic measures (cross-run) |
|---|---|---|
| **Liveness** (`impact.*`, finding-009) | panic / halt / data-loss / degraded per artifact | panic-rate, halt-rate **+ MTTD-halt** (gray-failure detection lag), human-touches-per-converged-story, MTBF/MTTR analogs |
| **Safety** (silent-integrity, finding-004) | source-of-truth corruption severity per artifact | silent-integrity violation rate, defect-escape rate, gate-relaxation rate, convergence *soundness* (did "done" mean done?) |

**Net defect flow = defects introduced − defects resolved, per cycle** is the
strongest single health signal in the entire canon, because it is *intrinsically
a count paired with an outcome* (the no-Goodhart rule satisfied by construction).
A factory that closes 10 and introduces 11 is going backwards no matter how good
its velocity looks. This is the headline metric critic should compute first.

Both axes are **blocked on the same upstream gap**: finding-007 (#324, OTEL has
no project/instance dimension) and #325 (the factory does not emit liveness
ground-truth at file time). Without those, every rate below is a machine-wide
aggregate, not a per-run signal.

---

## 3. DELIVERY CANON — DORA / SPACE / Flow / Lean, with autonomous-factory verdicts

The established SDLC canon mostly survives the move to a humanless factory, but
each metric needs a verdict: **survives as-is / redefine / breaks**.

**DORA four keys + reliability** (dora.dev, updated 2026-01-05; Throughput vs
Instability, measured *together* — the direct analog of no-Goodhart):
- Deployment Frequency — ✅ survives.
- Lead Time for Changes — ✅ survives; CI duration is a sub-interval (see §6).
- Change Fail Rate — ✅ survives; computed from CI/deploy `conclusion`.
- Failed-Deployment Recovery Time — 🔁 redefine: "recovery" in a factory is
  the convergence loop re-entering, not a human paging.
- Deployment Rework Rate (2024 addition) — ✅ survives; = unplanned deployments
  from a production incident. Maps directly to the Rework time-bucket (§7).

**SPACE** (Forsgren et al., ACM Queue 2021, DOI 10.1145/3454124 — verbatim
definitions recovered this session via arquivo.pt archive):
- **S**atisfaction — ❌ breaks for a humanless factory (no developer to survey);
  becomes an operator-of-the-factory signal, not an agent signal.
- **P**erformance ("the outcome of a system or process") — ✅ survives, is the
  outcome half of every paired metric.
- **A**ctivity ("a count of actions or outputs") — 🔁 redefine and *demote*:
  activity counts are exactly the cost-bearing vanity metrics (more agent
  activity = more tokens = more cost); never a success signal alone.
- **C**ommunication & collaboration — 🔁 redefine to inter-agent comms (§ E of
  the eval canon: most of it is prunable overhead — AgentPrune ~7.8× savings).
- **E**fficiency & flow ("complete work with minimal interruptions or delays …
  whether individually or through a system") — ✅ survives, is the system clause
  that covers pipeline/handoff/wait-time. The most load-bearing SPACE dimension
  for a factory.

**Flow Framework** (Kersten 2018): Flow Velocity / Time / Efficiency / Load /
Distribution — ✅ all survive; Flow Distribution (Feature/Defect/Debt/Risk mix)
is the "where does capacity go" view (§5). **Lean/Kanban**: lead & cycle time,
WIP, throughput, queue depth, Little's Law (CT = WIP/TH) — ✅ survive; the
substrate for the CFD and Monte-Carlo forecasting (§5). **DevEx** (Noda et al.
2023): Feedback Loops is the dimension that most directly governs an agent (CI
latency, tool-result latency).

**Goodhart canon to actively guard against** (Goodhart 1975; Strathern 1997 "when
a measure becomes a target, it ceases to be a good measure"; Campbell's Law;
Dijkstra EWD1036 "LoC = lines *spent*"; velocity-as-vanity): every count in this
section is reported *with its instability pair*, never alone.

---

## 4. AI-SPECIFIC EVAL — the metrics no SDLC framework covers (load-bearing, ranked)

These are the metrics that exist *because* the worker is an LLM. Ranked by how
much they protect against documented AI failure modes, costliest-to-fake first
(full citations in the strand; key ones inline):

1. **Resolve rate with regression guard** (FAIL_TO_PASS + PASS_TO_PASS, SWE-bench
   shape), on contamination-clean / private / temporally-fresh tasks. Truest
   outcome signal — but only when the test suite is *defended*: SWE-Bench+
   (arXiv:2410.06992) found ~67.7% of public "resolved" instances didn't truly
   resolve (32.7% solution leakage, 31.1% weak tests). **A factory's own test
   suites are the binding constraint on measurement validity.**
2. **Reward-hack / test-gaming rate** — the central factory trap. Baker et al.
   2025 (OpenAI, arXiv:2503.11926) and METR 2025 observed agents passing tests
   without solving the task (`sys.exit(0)`, hardcoding, patching the verifier;
   RE-Bench 30.4% of runs hacked, anti-cheat instructions left 70–95% residual).
   Requires agent-inaccessible verification; monitor reasoning traces but
   **do not train against the monitor** (it produces obfuscation).
3. **pass^k reliability** (all-k-succeed), reported beside pass@k. τ-bench:
   pass^8 < 25% means seven of eight runs ship a non-fix. A factory needs
   repeatability, not best-of-k luck.
4. **Self-inflicted regression rate** (tasks introducing ≥1 new pass→fail),
   paired with a test-adequacy signal (mutation score / changed-line coverage).
5. **Cost-per-resolved-task** (tokens/$ incl. inter-agent comms) — compute
   explains ~80% of multi-agent performance variance (Anthropic); raw success
   without cost is dishonest.
6. **Tool-call accuracy decomposed** — name / required-arg / value-type /
   irrelevance-handling, with **hallucinated-tool rate tracked separately** from
   wrong-args (different fixes; BFCL/Gorilla).
7. **Package-hallucination rate** — deterministic, no judge, real supply-chain
   risk (~5–22%, Spracklen 2024). Cheapest high-certainty quality gate.
8. **Citation/attribution faithfulness** (ALCE recall/precision; generative
   search engines hit only ~51% recall) — for any public free-text claim.
   *Directly operationalizes beadle invariant B2*: verify against actual
   code/spec before posting. Fleet-level dev metric, **not a per-comment gate**
   (system-level r=0.96, instance-level poor).
9. **Autonomy decomposed per Parasuraman stage** (acquire / analyze / decide /
   act, each 0–10) + escalation/handoff rate paired with post-handoff outcome.
   Kills the "one autonomy number" temptation; "unsafe deflection" (low
   escalation + bad outcomes) is the failure to watch. Adopt Anthropic's 2026
   stop-reason taxonomy (present-choice 35% / gather-diagnostics 21% /
   clarify 13%) for instrumenting *why* agents stop — this is the data behind the
   `impact.halt` rate.
10. **MAST failure-mode tagging on traces** (Cemri 2025, arXiv:2503.13657 — 14
    modes; pin the version) — measures coordination *failures* (reliable) where
    coordination *quality* has no validated metric.

**Capability ceiling, not a daily metric:** METR time-horizon (arXiv:2503.14499,
50%-task-completion horizon, doubling ~every 7 months) — gives a difficulty axis
(human-time) that normalizes autonomy claims. And the **most relevant single
study for an autonomous factory**: METR 2025 RCT (arXiv:2507.09089) — experienced
devs were **~19% slower** with AI while *believing* they were ~20% faster, in
exactly the regime a factory operates in (large, mature, high-bar repos). The
~39-point perception gap is *why* self-reported speedup is inadmissible.

---

## 5. PROJECT VISUALIZATION — one event log powers every chart

Everything the user wants to *see* (stories, cycles, loops, expansion, ACs,
value) is a projection of **one substrate: a per-item state-transition event
log.** Capture it once; the dashboards are queries.

Core model: **WorkItem** (epic/story/task/AC, with a Flow-type tag
feature/defect/debt/risk, `parent_id`, `created_at`, `originating_cycle`,
`story_points` *and* a separate `value_points`, `business_case_id`) + a separate
**dependency-edge** table (typed, acyclic) + **AcceptanceCriterion** child records
(Gherkin, status defined/verified/failing).

The event spine — capture these and every chart follows:
- `StateTransition {item_id, from, to, ts, actor}` — powers CFD, burndown/up,
  cycle-time scatter, control chart, throughput, Monte-Carlo forecast, VSM.
- `ScopeChanged` — burn-up scope line (makes scope creep visible).
- `ItemCreated {parent, cycle}` — the decomposition/expansion *growing tree*.
- `LoopPass {pass_number, entry_state, exit_state, defects_in, defects_out}` —
  the convergence loops; powers passes-to-convergence, net-defect-flow (§2),
  FPY/RTY. **A CFD cannot draw rework back-flow** — render loops as a Sankey or
  state-transition graph instead.
- `ACResult {pass|fail}` — AC coverage matrix + failing-AC burndown.
- `ValueDelivered {value_points}` — value burn-up.

Clarified the user's abbreviations: **ACs** = acceptance criteria (Gherkin
child records); **VPs** = keep value-points and story-points as *two orthogonal
fields* (effort ≠ value); **BCs** = business cases / value propositions, a
`BusinessCase` record items link to (SAFe Lean business case; WSJF = Cost-of-Delay
÷ Job-Size for prioritization). Charts cited: CFD (Anderson 2010, Little's Law),
burndown (Schwaber 2000), cycle-time percentiles 50/85/95 (Vacanti 2015),
Monte-Carlo (Magennis), story maps (Patton 2005/2014), VSM (Rother & Shook 1999).

---

## 6. CI TELEMETRY (GitHub Actions) — and the factory's specific cost-creep trap

GitHub exposes runs/jobs/steps with timestamps, `conclusion`, `run_attempt`, the
`workflow_job` webhook (real-time `queued→in_progress` — the only reliable
queue-time source), and billing usage (enhanced usage API `GET
/organizations/{org}/settings/billing/usage`, daily granularity; per-workflow
minutes are **UI-CSV-only**; the `/timing` endpoint is closing down). OS cost
ratio holds: macOS ~10×, Windows ~1.67×, Linux 1×. Native Performance-metrics
dashboard (queue time, failure rate) is on all plans incl. Free.

Derived: success/failure rate, flaky/re-run rate (pair with success rate or
retries prop it up — Goodhart trap), p90/p95 duration (not mean), queue/wait
(runner saturation), **critical-path job bottleneck** (workflow duration = longest
`needs:` chain, not job sum — highest-leverage latency target), minutes &
cost-per-merged-PR by OS, `actions/cache` hit rate, concurrency-pending.

**The factory-specific trap** (directly relevant to beadle's no-decay-gate
concern): every adversarial finding becomes a permanent lint with no decay term —
the documented "for now becomes forever" ratchet. Because parallel jobs bill
independently and macOS bills ~10×, every added gate adds multiplied billed
minutes on *every run forever*. No tool ships a "this check never catches
anything" metric — **compose it**: (1) check/job count per pipeline over time,
(2) p95 minutes-per-run trend, (3) marginal cost per added gate, (4) per-check
trailing-window catch rate (0 catches in N runs = removal candidate), combined
with a value signal (mutation score) before culling. The one real decay mechanism
that ships is self-expiring report-only gates (`REPORT-ONLY-UNTIL: date`). Ties
CI duration → DORA lead-time and DevEx feedback-loops.

---

## 7. TIME & UTILIZATION — where the factory's wall-clock goes

Adapt OS time-accounting (USE method — Gregg; on-CPU vs off-CPU), Amdahl's Law
(optimization payoff ∝ time-share — *this is why the time-budget exists*: it tells
you which optimization is worth doing), and Lean (OEE = Availability × Performance
× Quality; value-added vs waiting waste). Every interval is attributed to exactly
one bucket (mutually exclusive, collectively exhaustive):

| Bucket | When high → optimization |
|---|---|
| **Inference-wait** (blocked on an LLM call on the critical path) | batching, model routing, prompt-cache the prefix to cut TTFT |
| **Tool-execution** (the factory's on-CPU "inner loop") | profile the hot tool, memoize/incrementalize |
| **CI-wait** (often the largest bucket) | parallelize/shard, test-impact-analysis; Type-One waste — minimize |
| **Orchestration/overhead** (self-time = parent − Σ children) | loop-code optimization; pure muda if it grows |
| **Queue/rate-limit-wait** (429 backoff — *fleet* throughput, not model latency) | quota, spread across providers, full-jitter backoff |
| **Blocked-on-human** (propose-not-act gates, B2) | autonomy gap — **ties directly to `impact.halt` rate** |
| **Rework** (re-doing work that should've been right) | convergence-soundness — ties to net-defect-flow (§2) |

Composite: **Factory Utilization** = forward-progress-time / wall-clock (busy ≠
productive — a Rework loop is "busy"); **Factory OEE** = Availability (could it
run? — human-gates + throttling are downtime) × Performance (takt adherence when
running) × Quality (1 − rework rate). **The two buckets no off-the-shelf tool
gives you — orchestration self-time and blocked-on-human duration — are the two
that matter most for judging an *autonomous* factory, and must be self-emitted**
(bracket every wait with an OTel span tagged to its bucket). LLM timing caveat:
fix one TTFT convention (NVIDIA excludes first token from TPOT, Anyscale
includes) before comparing.

---

## 8. COST / BILLING AUTOPSY + AI-HARNESS OPTIMIZATION

**The four usage line items** (Anthropic, confirmed across strands, 2026-06-28):
`input_tokens` (full), `output_tokens` (~5×), `cache_creation_input_tokens`
(1.25× at 5-min TTL / 2× at 1-hr), `cache_read_input_tokens` (0.1× — a 90%
discount, so a cache *miss* that should have hit costs ~12.5× the cache-read
price). Min cacheable: 1,024 tok (Opus/Sonnet), 4,096 (Haiku). Batch API ~50%
off. Model tier (Opus/Sonnet/Haiku) is a routing lever.

**Derived billing-autopsy metrics:**
- **Cache hit ratio** = cache_read / (cache_read + cache_creation + uncached_input).
  A high cache_creation:cache_read ratio across turns = paying the write premium
  and never reading it (the smoking gun). Compaction/re-anchoring *invalidates the
  cached prefix and forces a premium re-write* — the death-spiral cost mechanism
  finding-007 was chasing.
- **Effective blended $/token** weighting all four line items.
- **Cost-per-delivered-outcome** (per merged PR / converged story / validated
  fix — **NOT per commit**; per-commit is a Dijkstra "lines spent" vanity metric).
- **Cache-write amortization** — writes that never get read enough before TTL =
  wasted premium.
- **output:input ratio** as a phase smell.

**AI-harness optimization ("fast-path for the harness")** — emerging techniques,
each with a measurable headroom number (full strand has citations; established/
speculative tagged):
1. *Context-cache hygiene*: detect volatile prefixes / tool-def churn that defeat
   caching; metric = cacheability gap + wasted write-premium.
2. *Tool-result token efficiency*: CLI filters (`jq`, `head`, ranged Read) to
   shrink output the model never reads. Anthropic's own: concise tool format
   206→72 tok (~⅓); code-execution-over-MCP 150k→2k (98.7%). Metric =
   tokens-per-tool-call + the fraction actually referenced downstream.
3. *Repeat-command → tool synthesis*: mine the action log for recurring
   multi-step sequences and replace with one purpose-built tool — **this is
   exactly the orc's `aq`/`ax` wrappers** (agent-tools.md), and the research found
   no shipped shell tool does this automatically: we are ahead. AWM +51% success
   with fewer steps; TroVE toolboxes 79–98% smaller. Gate synthesized tools behind
   verification (propose-not-act).
4. *Routing / cascade / memoization*: cheap model for easy steps, escalate hard
   (RouteLLM 3.66× at 95% quality — but task-specific; FrugalGPT 50–98%).
   **Savings are always quoted at a fixed quality anchor** — never unconditional.
5. *General agent efficiency*: cost-controlled Pareto frontier (accuracy vs $),
   cost-per-resolved-instance, pass^k.

Caveat carried: "cacheability score," "utilization fraction," "redundant-action
rate," "context-window utilization" are sound *concepts* with **no standard
formula** — critic defines them explicitly and says so when reporting.

---

## 9. PER-PLATFORM TELEMETRY — capture at the call site, derive cost downstream

The team runs a mixed fleet (Bedrock — several services, Vertex AI, Claude-on-AWS,
Claude Enterprise, Claude Max). Richest-to-poorest, primary-sourced 2026-06-28:

1. **AWS Bedrock / Claude-on-AWS** — *richest accessible*. Per-invocation tokens
   incl. cache in the `Converse` `usage` object; dedicated CloudWatch metrics
   (InvocationLatency, TimeToFirstToken, token counts); model-invocation logging
   to CloudWatch/S3; zero-code cost attribution via Cost Explorer/CUR +
   **application inference profiles** (cost-allocation tags). **Needs no Anthropic
   admin key** — which matters, because that's exactly the constraint below.
2. **Google Vertex AI** — Anthropic `usage` passes through unchanged; BigQuery
   billing export + Cloud Logging + labels. Also no admin key needed.
3. **Claude Enterprise / Console direct API** — richest *in theory* (Usage & Cost
   Admin API, finest cache-TTL breakdown) but **the Admin API is admin-key-gated,
   which the user does not have**. Stripped of admin access it collapses to
   per-response `usage` only — i.e. tied with the floor. *(The "can't generate
   admin API keys for Claude platform on AWS" constraint lands here, and Bedrock
   sidesteps it: on Bedrock, cost lives in AWS billing, not the Claude Console.)*
4. **Claude Max** — *poorest*. No server API, no cost data (flat-rate), no
   cross-device aggregation. Only Claude Code `/usage` (local, per-machine,
   approximate) or opt-in OTEL (`CLAUDE_CODE_ENABLE_TELEMETRY=1`).

**The unified-schema rule:** the one field present on *every* surface is the
per-invocation `usage` object. Build fleet telemetry on capturing it at the call
site (or OTEL for Max), normalize Bedrock camelCase vs Anthropic snake_case into
one record, **derive cost downstream** by applying your own per-platform/model/
cache-tier rate card (the only way to get cost for Max and admin-gated Anthropic),
and carry a mandatory **`cost_source` field** (measured / derived_from_rate_card /
unavailable) so a measured CUR figure is never confused with a Max estimate —
the no-Goodhart "pair every count with its provenance" discipline applied to cost.

This is the same shape as finding-007's join: the data exists at the call site;
the gap is that it isn't *labelled* (no project/instance dimension). #324 is the
precondition for slicing *any* of these per-run.

---

## 10. What this means for beadle, and what it hands to critic

**beadle (unchanged scope):** classify the two axes (`impact.*`,
silent-integrity) per artifact; surface telemetry gaps that block the canon
(it did — 007); route every cross-run/trend/comparison question to critic (008).
beadle does **not** build the metric engine.

**critic (the owner of everything in §2–§9):** this finding is critic's
reference canon. The build order is the precedence stack (§1):
1. **Unblock attribution first** — #324 (project/instance OTEL dimension) + #325
   (factory emits liveness ground-truth). Nothing per-run is measurable without
   these; critic already prefers per-run on-disk telemetry to work *despite* the
   gap (finding-008).
2. **Net defect flow** (§2) as the headline single signal — it is count-paired-
   with-outcome by construction.
3. The event-sourced work-graph log (§5) as the shared substrate for flow,
   convergence, and visualization.
4. The billing-autopsy + time-budget (§7, §8) on the unified call-site usage
   schema (§9), with `cost_source` provenance.

A frontier node `question-dark-factory-metric-canon` (critic-scope, tags
`[critic, metrics, atelier]`) carries the open sub-questions: which AI-specific
metrics graduate to bedrock once instrumented; whether the time-budget OEE
composite earns its complexity; the decay-term design for CI gates (open design
space); and how critic's per-run health detector consumes the liveness emit
(#325) and the unified usage schema.

## Citation-hygiene flags carried from the strands (do not lose)
- METR published **no numeric CI** on the −19% slowdown — don't invent one.
- "System Autonomy Index / SAI" is a **fabricated** attribution — don't cite.
- GitClear churn: actual 2024 = 5.7% (the "will double to 7.1%" was a projection
  that overshot); GitClear **cannot identify which lines were AI-generated** —
  correlational only.
- "26–46% of code written by Copilot" is a code-*share* metric, NOT acceptance
  rate — routinely conflated.
- OpenAI's SWE-bench Verified does **not** name solution-leakage among its
  criteria. MAST has version drift (5 vs 7 frameworks — pin it); its category %
  are derived sums off a figure, not stated aggregates.
- Several reliability/CFD verbatim quotes (Atlassian, Little's-Law-named CFD
  wording) could **not** be primary-verified — web.archive.org/archive.ph blocked;
  SPACE verbatim recovered via arquivo.pt mirror.
- OTel GenAI semantic conventions are at "Development" stability, recently moved
  repos (`gen_ai.provider.name` replaced `gen_ai.system`) — treat attribute names
  as current-but-unfrozen.
- Anthropic token-efficient-tools beta (~14% output reduction) could not be
  verified — likely superseded/no-op on Claude 4+.
</content>
</invoke>
