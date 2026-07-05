# Proposal: restore the curated board, keep the mechanical substrate

**Status:** draft for review (session 2026-07-05)
**Companion finding:** `_kos/findings/finding-019-render-regression-analysis-lives-in-discipline.md`

## The one-sentence diagnosis

The Rust move optimized the *plumbing* (store, digests, watermarks, validation)
and silently deleted the *product* (an intent-grouped, prioritized,
maintainer-actionable board) — because the analysis was never in the binary and
nothing forced the analysis to run before the projection published.

## Division of labor (the rule going forward)

| Zone | Owner | Rationale |
|---|---|---|
| Enumerate / comment-sweep (`enum`, `sync`) | **binary** | mechanical GitHub I/O; watermark discipline; already correct |
| Store append + validation (`classify ingest`) | **binary** | bounded enums, HARD invariants (integrity ⇒ anchor, integrity ⇏ quick-win) hold by construction |
| Classification + intent-scoring **judgment** | **skill (LLM)** | PR #18's own rationale: a Rust heuristic would launder guesses |
| Action-plan grouping, quick-wins lane, cluster readings, Direction-Health narrative, maintainer-progress framing | **skill (LLM)** | this IS the product; prose judgment, precedence per finding-004 |
| Sentinel, derived_digest, controls block, editor-slot preservation, push discovery (sentinel-first, exactly-one, wrong-author STOP) | **binary** | mechanical framing that must never drift |
| Direction signal **numbers** (A4/B/C) | **binary** | deterministic joins over the store |
| Direction **interpretation** | **skill/editor slot** | already the design (finding-017); keep |

The skill authors the analysis zones; the binary frames, digests, discovers,
and pushes. Neither side crosses the line.

## Concrete changes (proposed bd tickets)

1. **`beadle render` fails loud on zero classifications** for the current run —
   nonzero exit, no body emitted. A projection of nothing must never look like
   a dashboard. (P1 — this is the guard that was missing.)
2. **Kill the flat all-issues table** in render. It violates B1 (projection,
   not replica), busts the 65 K budget at ~300 open issues, and is the
   "poor man's copy of the issues tab" the user rejected. (P1)
3. **Curated-zone slots:** generalize the editor-slot mechanism —
   `<!-- curated:action-plan -->`, `<!-- curated:quick-wins -->`,
   `<!-- curated:direction-health -->`, `<!-- curated:classification-index -->`,
   `<!-- curated:maintainer-progress -->`. `beadle push` preserves them across
   regen exactly as it preserves editor slots today; the skill rewrites them
   each pass. Binary owns everything outside the slots. (P1)
4. **Vocabulary reconciliation:** classify.rs enums become a superset of the
   skill's §3 vocabulary — add `process-gap`/`enhancement`/`policy` to
   report-type (~30 of 47 run-10 issues are process-gap); add
   `halt`/`panic` to operational-impact (finding-009 is beadle's own axis and
   the binary cannot represent it). Until then every ingest needs a lossy
   shim (`tmp/run11/map_to_store.py` documents the mapping). (P2)
5. **Priority vocabulary:** accept `P0a`/`P0b` (finding-004 distinguishes
   them; the store currently flattens to `P0` and the distinction lives only
   in prose). (P2)
6. **Run-record-first ordering:** `enum` opens the run record; `direction`
   and `render` compute against the current run, not the previous one.
   (P2 — observed twice: direction reported run-9 during the run-10 pass and
   run-10 numbers during the run-11 pass.)
7. **SKILL.md step ordering:** classify-then-ingest becomes a stated
   precondition of step 7 (render/push), not an optional aside. (P2, docs)

## What we do NOT do

- Do not teach Rust to rank, group, or narrate. That is judgment laundering,
  the exact thing PR #18 refused.
- Do not delete the binary. `enum`/`sync`/`classify ingest`/push-discovery/
  slot-preservation are real wins (the hand-maintained sentinel was stale
  minutes after push on run 9).
- Do not hand-edit the derived zone (sentinel, controls) — that stays
  binary-owned and digest-covered.

## Interim operating procedure (until 1–3 land)

The skill composes the full body (as in run 10 / run 11), keeps the sentinel
JSON + controls format compatible, and pushes via `gh issue edit`. The store
is still fed (`enum`, `sync`, `classify ingest` with the shim) so signals
B/C/A4 and the watermark stay warm. `beadle push` is NOT used until the
curated-zone slots exist — it would overwrite the curated body with the flat
render (the run-10 near-miss).
