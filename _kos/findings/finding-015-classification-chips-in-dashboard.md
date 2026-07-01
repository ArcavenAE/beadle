---
id: finding-015
title: "Dashboard renders classification chips + summary block"
tags: [beadle, render, classifier, ingest-contract, phase-4-followon]
date: 2026-07-01
provenance:
  created_by: agent
  probe: retrospective-followon
---

# What shipped

`beadle render <target>` now consumes `ClassificationRecord` rows and surfaces
them in two places:

1. **Per-issue chip column** in the Open-issues table.
   Compact form: `<report_type> · <defect_nature> · <priority>[ ⚠][▲][★]`.
   Glyphs: `⚠` integrity=true, `▲` operational_impact=silent-data-loss,
   `★` quick_win_eligible=true. Unclassified issues render `_unclassified_`.

2. **Classification summary block** immediately above Open-issues.
   Row totals (classified count / total observed, integrity count,
   silent-data-loss count, quick-win-eligible count, P0/P1 counts) plus a
   report-type breakdown table. When no classifications exist, the block
   emits the propose-not-act pending disclosure verbatim — same rationale
   as `SignalOrPending::Pending` in direction.

## Superseding rule

If the skill reclassifies an issue (new evidence arrives, or an earlier
class was wrong), the later `ClassificationRecord` — higher `run` first,
then later `ts` as tiebreaker — wins. Earlier records stay in the store
as history but do not render.

## End-to-end validation

- Scratch target with 3 issues + 3 classifications (one integrity+SDL P0,
  one docs P3 quick-win, one perf P2). Chip column shows
  `bug · spec-requirements · P0 ⚠▲` / `docs · spec-requirements · P3 ★` /
  `perf · algorithmic · P2`. Summary rows tally correctly. Direction
  verdict on the same store still returns `drifting` on B (integrity=33%)
  and C (SDL=33%), matching the row counts one-to-one.
- Real `vsdd-factory` (run 9, 180 issues, 0 classifications) renders
  chip column as `_unclassified_` on every row + the pending disclosure
  in the summary block. Body size 45799 bytes = 81% of the 55 KiB
  budget — the added column is affordable at real target scale.

## Why chips live in the derived zone (not editor slots)

The chip is a deterministic projection of `ClassificationRecord` — no
interpretation, same output from the same input. Sentinel `derived_digest`
covers it, so a hand-edit to a chip is a detectable event on the next
render (question-renderer-editorial-boundary). The editor still owns the
verdict paragraph and per-issue prose notes; chips are the machine's
half of the split.

## What this unblocks

- Actually invoking the Phase-0 classifier skill against
  `vsdd-factory`: the render→push loop now has a visible payoff for
  every classification the skill produces. Empty chips are the loudest
  possible "the skill hasn't run against this target yet" signal.
- The first live push of the dashboard: chips make the difference
  between "beadle wrote something" and "beadle wrote something a
  human maintainer can act on" legible.
- Editor-slot rot detection with signal: once chips populate, sentinel
  drift on chip content = classifier updated a record = worth showing
  in the run-over-run comparison (future item, out of scope here).

## Deliberately open

- **Chip length under load.** vsdd-factory has 180 issues; if every one
  classifies with three flags each, chip column adds ~30 chars × 180 =
  ~5.4 KiB. The body-budget currently has ~10 KiB of headroom. If
  populated dashboards start bumping the ceiling, item C rollup
  (linked backlog issue) becomes needed. Not blocking today.
- **Ordering.** Issues render by number descending. A future variant
  could sort by classification severity (P0-integrity first). Not
  shipped — sort stability matters more than severity ordering for
  hand-edit diff clarity.
- **Cluster ↔ classification cross-refs.** Both `ClassificationRecord`
  and `ClusterRecord` carry cluster names; the render doesn't join
  them yet (e.g., "silent-data-loss cluster contains N classified
  items, K carry silent-data-loss impact"). Small future patch;
  no invariant depends on it.
