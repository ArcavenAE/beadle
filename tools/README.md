# tools/

## `dashboard-gate.py`

The `/dashboard-refresh` §3 regression gate, executable. Written during run-19
and preserved here because it is the reference implementation for **P0-2** in
`docs/proposals/binary-skill-convergence.md` (port to `beadle verify`).

```sh
python3 tools/dashboard-gate.py <before-snapshot.md> <candidate.md>
```

Exit 0 = pass. Checks: section presence, no section loss (declared renames
allowlisted), no coverage shrinkage **per axis** (not by union — the union form
misses any loss confined to one axis), roll-forward of per-run `*_new` axes into
their `*_prior` counterpart, no `_unclassified_` rows, and markdown
render-integrity **relative to the before-snapshot** — a pre-existing violation
warns, a newly introduced one fails.

Run-19 found two things with it that the command doc's own wording would have
missed: `operational_impact.degraded_new` silently dropping 11 members on reset,
and the fact that the live body carries an inherited render-integrity violation
(one `<details>` block holding one 30-row table; the gate reports per row, so it
prints 30 entries for one defect — count the table, not the rows)
that an absolute check would block forever.
