# finding-023 — run-18: a fixed-but-open sweep at a pinned SHA belongs in every refresh after a gap

**Status:** confirmed (run-18 executed end-to-end, 2026-09-09)
**Scope:** beadle Phase-0 — `/dashboard-refresh` command, skills-based refresh, store notes
**Follows:** finding-019 (analysis lives in the discipline), finding-020 (command-encoded refresh)

## What happened

Run 18 refreshed drbothen/vsdd-factory#312 seven weeks after run 17 (2026-07-24
→ 2026-09-09). The window held 23 attention windows, 32 maintainer PRs, two
releases (v1.0.0-rc.24, rc.25), 32 new issues (#759–#830) and only seven corpus
actions, all on 07-24/25. The command ran as written — backup verified
byte-identical to the run-17 fixture, three sync grader batches, regression
gate FAILS 0 / WARNS 0 against both the before-snapshot and the canon, post,
after-snapshot byte-identical to the candidate. The binary `render`/`push`
path stayed unused (finding-019).

The new step was a **fixed-but-open sweep**: 28 open issues whose fix might have
landed were verified by five verifier agents at the pinned upstream SHA
(`upstream/develop` fff5e4cc, 2026-09-07) on a read-only worktree — ancestry
against the release tags, executed tests where they existed, mechanism read
from source. Verdicts: 7 FIXED (#358 #637 #690 #580 #757 in rc.24; #279 in
rc.25; #641 unreleased), 9 PARTIAL (#342 #378 #473 #515 #648 #687 #789 #794
#827), 12 NOT-FIXED. Fixture:
`docs/fixtures/vsdd-factory-312-run18-fixed-but-open-sweep.json`.

## F1: a fix PR that says "closes" without the keyword leaves a verified-fixed issue open — and the instrument counts it as untouched

#358 (P0a, A4 lane) was fixed by maintainer-authored PR #761 and shipped in
rc.24. The PR body names the issue in prose without a closing keyword, so
GitHub never closed it, and the capacity-adjusted A4 streak kept counting it
as an untouched P0a across every window since. The same shape held for #637,
#690, #580 and #757. Without the sweep, run 18 would have reported the A4
residue as unchanged when three of its five members were shipped fixes.

Consequence for the instrument: **untouched-window counts are only as good as
the closure signal**, and the closure signal on an episodic side project is
the maintainer remembering the keyword. The sweep is the correction; the
close proposals go on the board as Tier-1 `accept-deferral` controls plus
comment drafts, never as closes (propose-not-act, G2).

## F2: the sweep is also a self-audit

Verifying at the source found four claims of ours wrong: the #473 comment
(the artifact-path registry has supported `{placeholder}` segments since
2c97cb00), the #799 comment (`git -C <worktree>` is handled by target-aware
branch detection, not evaded — blocked by executed tests), the #793 body (the
minimal repro needs `-f`; the guard requires both a `git clean` and a
`-f`/`--force` substring), and the #809 body (the dispatcher has applied a
per-entry `fuel_cap` since S-1.2). All four are queued as corrections under
the upstream-claim-gate (rules 8/9) and recorded in the store
(`kind: note, topic: corrections`); none was posted this run.

## F3: what the verification protocol needs to hold

- Pin the SHA and record it in every verdict (`pinned_sha` in the fixture).
- Distinguish *merged* from *released*: ancestry against each `v*` tag, not
  just `HEAD`. #641 is fixed at HEAD and unreleased; #279 is rc.25 only.
- PARTIAL is a first-class verdict with a named residual. Nine of 28 landed
  there, and collapsing them to either side would have been wrong.
- A sibling PR that "validates" a class does not close an issue outside its
  arm — #749 must not be closed against PR #776 (Arm A2 covers table rows
  only).
- Every candidate carries an explicit verdict (upstream-claim-gate rule 12).

## F4: operational notes (tooling friction, captured before workaround)

- `gh issue list --limit 500` returned exactly 500 rows for a 510-issue corpus
  with no truncation signal; `--limit 1000` is now the enumeration default.
- `forks/vsdd-factory` had no `upstream` remote, so `git log upstream/main`
  silently returned nothing; added per the documented forks/ shape.
- The harness `Agent` tool ran every grader/verifier dispatch in the
  background despite the sync-only policy (aae-orc-0nud still open). All nine
  returned; no starvation observed (null observation for the sunset review).
- Body budget: 72,713 → 77,033 bytes. A `gh issue edit` failure is atomic, so
  the live body is never at risk from an oversize candidate; the cap is a
  readability budget, not a safety one.

## Recommendation

Add the sweep to `/dashboard-refresh` §2 as a standing step whenever the gap
since the last run exceeds one attention window: enumerate open issues whose
number appears in any merged PR body or release note since the watermark,
verify each at the pinned SHA, and table the verdicts under Maintainer
progress. Proposed here, not yet edited into the command.
