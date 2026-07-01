---
id: finding-016
title: "Silent-data-loss zero-engagement alarm (A4) wired into direction"
tags: [beadle, direction-verdict, phase-4-followon, silent-data-loss, escalation, invariant-b3]
date: 2026-07-01
provenance:
  created_by: agent
  probe: retrospective-followon
---

# What shipped

Signal A4 from `question-direction-verdict-escalation` — the harshest of the
frontier's proposed escalation clauses — is now live in `beadle direction`.
Where signals B and C measured *density* (how many of this run's classified
issues are integrity / SDL), A4 measures *silence itself*: SDL-classified
issues that have persisted across runs with zero maintainer engagement.

The alarm is a proper join across two record kinds already in the store:

- **`ClassificationRecord`** — the SDL classification lineage per issue.
- **`CommentEventRecord`** — the engagement audit trail per issue, with
  `actor_role ∈ {maintainer, measured, other}` sourced from
  `targets/<target>.intent.yaml`.

For each issue ever classified `operational_impact = "silent-data-loss"`:

1. Walk the classification history sorted by `(run, ts)`. For every run
   `r` from the issue's first classification through `latest_run`, compute
   the *effective label at run r* = the latest classification whose
   `run ≤ r`. This is the "which class was the current understanding at
   this point in time" reading — SDL persistence, not membership at a
   specific run.
2. Find the longest streak of consecutive runs ending at `latest_run`
   where the effective label is SDL.
3. Consult the escape hatch: if any `CommentEventRecord` on this issue
   has `actor_role == "maintainer"`, drop the issue from the alarm set
   entirely. This encodes invariant B3 (maintainer engagement is the
   compass) at the signal level — silence is only silent when the
   maintainer has not spoken, per finding-004.
4. Categorize eligible issues:
   - streak ≥ 3 → `drifting` set (frontier A4's harshest clause)
   - streak ≥ 2 → `watch` set
   - streak = 1 → contributes to `sdl_issues` count but no alarm

Verdict rules (mirror the existing signal shape):

- `pending` when no SDL classifications exist, OR when `latest_run < 2`
  (insufficient history to form even the watch streak). Pending carries
  a specific `reason` string so the JSON output is self-explanatory.
- `drifting` if any issue is in the drifting set.
- `watch` if any issue is in the watch set.
- `on-course` otherwise (including the "SDL-tracked but maintainer
  engaged on every one" case).

## Top-signal precedence change

`top_signal()` now prefers A4 when its verdict matches the overall
verdict, ahead of integrity-density and SDL-share. Rationale: A4 is the
signal that measures *the thing beadle exists to catch* — issues rotting
in silent-data-loss status while nobody looks. When A4 fires the same
verdict as integrity or SDL-share, A4 is the more actionable message
because it names *which issues* (`drifting_issues: [42, 87, ...]`)
rather than a share percentage.

## Live variant / pending variant

Same shape as `SignalOrPending`: a `ZeroEngagementOrPending` enum with
`#[serde(untagged)]`. The pending variant contributes `OnCourse` to the
max-verdict calculation. A signal that lacks the data it needs cannot
silently escalate — the invariant carried forward from finding-014.

## What did NOT ship, and why

- **No trailing-window scoping (A3).** The alarm currently looks at *all*
  runs in the store, not just the last N. On a large store this is
  acceptable because the streak is computed per-issue and the outer walk
  is O(runs × issues-ever-classified), not O(all-runs × all-comments-
  ever). If the calibration corpus (A1) reveals streak counts inflating
  after long histories accumulate, a trailing-window cap is a bolt-on.

- **No "silent since first classified" separate rank.** An issue that
  went SDL → non-SDL → SDL is currently reported as streak=1 at the
  latest run (breaks contiguity). That's correct for "the issue is
  currently in an unbroken silent-data-loss run" but *loses* the
  "cumulative silent-data-loss weeks" reading. Deferred until a target
  exhibits that pattern.

- **No auto-comment on drifting issues.** Per invariant B2 the signal
  emits *data*; the editor still owns the direction-verdict paragraph
  and any target-facing communication. The `drifting_issues` list is a
  reference for the editor / next classifier run, not a batch of
  outgoing messages.

## End-to-end validation

Scratch target at `/tmp/beadle-scratch-a4` with 3 runs, one issue (#42)
classified SDL at each run, no maintainer engagement:

```
verdict: drifting
top_signal: silent-data-loss-zero-engagement drifting: 1 SDL issue(s)
  with ≥ 3-run silence + zero maintainer engagement (longest streak: 3
  runs)
signals.silent_data_loss_zero_engagement:
  sdl_issues: 1
  watch_count: 1
  drifting_count: 1
  drifting_issues: [42]
  longest_streak: 3
```

Adding one `comment_event` with `actor_role="maintainer"` drops #42 from
the alarm entirely: `drifting_count=0`, `longest_streak=0`, verdict flips
to `watch` (driven by the SDL-share signal alone). Escape hatch works.

7 unit tests cover: pending-no-SDL, pending-insufficient-history,
on-course-when-maintainer-engaged, watch-on-2-streak, drifting-on-3-streak,
user-comment-does-not-bypass (only `actor_role="maintainer"` is the
hatch), and streak-is-contiguous-ending-at-latest-run (a broken streak
resets to the length of the current tail).

Total test count on `beadle`: 37 (was 30 after PR #19). Full `cargo test`
green.

## Cross-cutting

- Answers frontier A4 in `question-direction-verdict-escalation`.
- Extends the `ClassificationRecord × CommentEventRecord` join —
  same shape as the maintainer-compass work in finding-011 but with a
  temporal dimension.
- Preserves the propose-not-act boundary (B2 / ADR-002): the signal
  emits a machine-readable data structure; the editor and the human
  compass decide what to *do* about a drifting-set issue.

## Deliberately open

- **A1 calibration.** Thresholds `SDL_ZERO_ENGAGEMENT_WATCH_STREAK=2`
  and `DRIFTING_STREAK=3` are frontier values from the escalation
  question. They stay frontier until at least two targets have ≥ 3
  runs of real data and the streak distributions are legible.
- **First live invocation.** Neither the Phase-0 classifier skill nor
  the direction command has yet run against real vsdd-factory data.
  This finding closes the *code* gap for A4; the calibration gap
  stays open.

## Files

- `crates/beadle/src/direction.rs` — new `sdl_zero_engagement_signal()`,
  `live_verdict_zero()`, `ZeroEngagementOrPending` + `ZeroEngagementSignal`
  types, updated `top_signal()` signature and precedence.
- `crates/beadle/src/direction.rs::tests` — 7 new tests.
