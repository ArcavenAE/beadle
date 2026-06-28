# finding-007 — vsdd-factory OTEL telemetry cannot attribute metrics to a project or to parallel instances

Date: 2026-06-28
Probe: user asked whether Grafana/factory logs show a compaction "death spiral"
(shrinking inter-compaction intervals, rising context-fill, falling velocity)
across the three live pilots (ftc-blue, switchboard-blue, akey). Investigating that
question surfaced a prior, blocking defect: the telemetry has no project dimension.
Nodes touched: vsdd-factory observability (engine); cross-refs beadle's own pilots.
Filed as: drbothen/vsdd-factory issue (see below).

## What the question was, and why it couldn't be answered from Grafana

The death-spiral question is legitimate and the signals to answer it *should* live in
the obs stack. They don't:

- **No compaction telemetry on disk.** None of the three pilots
  (`~/work/ftc-blue`, `~/work/switchboard-blue`, `~/work/aae-orc/akey`) has a
  `precompact-flush-log` or `postcompact-reanchor-*.jsonl`. The cadence signal
  vsdd-factory#318 wants to read isn't being written in pilots — the E-18 hooks
  aren't deployed there. (Confirms #317/#318/#319's premise from the outside.)
- **No context-occupancy metric in Prometheus.** The only Claude-Code series is
  `claude_code_token_usage_tokens_total` (a cumulative counter); there is no
  context-window gauge and no `context.*` compaction event. (Confirms #319.)

But the deeper blocker — the one this finding is about — is that **even the metrics
that DO exist cannot be sliced by project.**

## The defect

Every `claude_code_*` metric in Prometheus carries `session_id` and machine/model
labels, but **no `project` / `cwd` / `repo` / `workspace` / `branch` / `instance`
label.** Verified across all five emitted metrics:

```
claude_code_token_usage_tokens_total   project-ish labels: NONE
claude_code_cost_usage_USD_total       project-ish labels: NONE
claude_code_commit_count_total         project-ish labels: NONE
claude_code_session_count_total        project-ish labels: NONE
claude_code_active_time_seconds_total  project-ish labels: NONE
```

`session_id` is the *only* disambiguator (9 distinct over 7d), and it is an opaque
UUID. Consequences:

1. **No per-project view.** Cost, tokens, commits, active-time for ftc-blue vs
   switchboard-blue vs akey are summed into one undifferentiated series. The
   per-day velocity numbers I computed ($/commit worsening 06-27→06-28,
   cacheRead:input ≈ 2700:1) are **machine-wide aggregates**, not attributable to a
   project — so a per-project spiral is invisible *by construction*.
2. **No parallel-instance view.** ftc-blue ran 4 distinct sessions on 06-28,
   switchboard 2, akey 2. Even if a `project` label existed, two concurrent runs of
   the *same* project would still collapse together — there is no
   `instance` / `run_id` / `worktree` dimension. (Today's same-project sessions
   were sequential, so no overlap was observed; the structural gap stands.)

## Attribution IS recoverable offline — which proves the data exists, just isn't labelled

The factory's own `dispatcher-internal-*.jsonl` records `session_id` alongside the
project (by log file path). Joining that against OTEL's `session_id` recovers
attribution. **Proven:** switchboard-blue's dispatcher session `4016dfcc-…` is the
same UUID present on `claude_code_cost_usage_USD_total{session_id=~"4016dfcc.*"}` in
Prometheus.

So the fix is cheap in principle: emit the resource attribute already known at
session start. But the offline join is not a substitute — it breaks once logs
rotate/prune, requires a manual two-source merge, and still can't separate parallel
instances. Grafana needs the label natively.

## Recommended emit (in the issue)

`OTEL_RESOURCE_ATTRIBUTES` (or the factory's session-start telemetry hook) should set,
per session: `project` (product/repo name from STATE.md or git toplevel), `cwd`,
`worktree`/`instance` id, and ideally `cycle`/`wave`. These become Prometheus labels
and Grafana gets per-project, per-instance breakdowns — the precondition for *any*
velocity-drift or death-spiral panel.

## Relationship to the compaction cluster (#317–#320)

This is the **substrate** under that cluster. #317 emits a compaction event; #318
detects cadence; #319 infers it from telemetry; #320 shows it on the statusline. All
four are unsliceable per-project until this label exists. Filing order doesn't gate
them, but a death-spiral dashboard is not buildable without project attribution first.

## Why this is a beadle finding (not just a target-repo bug)

beadle discovered it by doing its job — orienting on the target's intent
(self-referential factory, observability is on-mission) and following a maintainer
question into the live telemetry. It is the second time (after the #313/#314 integrity
read) that beadle's intent-grounded triage surfaced a defect the volume-counters
would never flag: **a missing dimension is invisible to anything that only counts what
is present.** Reinforces B3 (engagement/intent is the compass, not volume) and the
B9 family (absence-of-signal is itself a signal).
