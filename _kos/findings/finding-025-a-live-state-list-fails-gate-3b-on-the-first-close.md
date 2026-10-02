# finding-025: a "live now" list in the sentinel fails gate check 3b the first time a listed item closes

Date: 2026-10-02 (from the 2026-10-01 scheduled refresh)
Scope: tools/dashboard-gate.py check 3b, and how boards keep live state in the sentinel
Status: worked around on two boards; the gate change is proposed in beadle#94

## What I saw

The daily refresh of two client boards (A and B) on 2026-10-01 used the skills method
and the gate at a8b4b0b. On client board A, the first candidate failed:

```
  ✗ 3b: axis 'open_prs_now' lost 1 issue(s): [191]
```

(the gate's summary line, which reports one failed check, is omitted)

The run-2 sentinel had added `open_prs_now` and `open_now` to hold what is open at
render time, next to the older cumulative `open` and `prs` lists, with an
`axis_semantics` note saying so. PR #191 was open at run 2 and merged before run 3. The
candidate dropped it from `open_prs_now`, which was correct for a live list, and check 3b
refused it.

I reproduced it at beadle main 94d9f15: the published run-3 body passes, and the same
body with `open_prs_now` set back to live state (`[200]`) fails with the line above.

## Cause

Check 3b treats every list in the sentinel as cumulative unless its key is in the
hardcoded `PER_RUN` set (`new_this_run`, `quick_wins_new`, `keystone_new`,
`prior_run_burst`, `degraded_new`). A board has no way to declare a new per-run or live
axis. A note inside the sentinel (`axis_semantics`) is prose, and the gate does not read
it. So any list whose meaning is "what is true now" passes while it only grows, and fails
on the first close or merge.

Client board B had three such lists and four closures pending in the same run. It would
have failed the same way.

## Workaround applied (and why it is not a bypass)

On both boards the live lists were kept cumulative (nothing dropped), and live state
moved into a map (`prs_state` on A, `state_now` on B) keyed by issue number with a
status string. That is the shape the director board already used. The gate code was not
changed, and every number that was tracked stays tracked. Both boards then passed.

The cost: the names now mislead. `open_prs_now` on board A reads as live and holds
`[191, 200]`, one of which is merged. A reader has to know to look in the map.

## Relation to other work

beadle#94 (the shadow comparison) reaches the same gap from the Rust side and
recommends "a gate class for per-run headings and cumulative axes" (its Skills 2 and 3).
This finding is the observed failure that recommendation predicts, on a live board,
with the workaround that was used. It adds no new design. If the gate gains a declared
per-run axis class, `open_now`, `open_prs_now` and the B-board lists should be declared
in it and the maps can be retired.

## Evidence

- Gate output, candidates and before and after bodies for both boards: the private
  refresh notes for 2026-10-01 (client data, not published).
- Reproduction: `tools/dashboard-gate.py` at 94d9f15, before = board A's run-2 body,
  candidate = run-3 body with `open_prs_now` live: exit 1, the 3b line above.
  Control: the published run-3 body against the same before: GATE PASSED.
