# Probe brief — what shape should Phase-0 out-of-band state take?

Date opened: 2026-07-01 (retroactive; work executed in-session before this brief
was written — captured now per ADR-006 harvest discipline).

## Question

`elem-state-out-of-band` establishes that beadle's durable state lives outside
the dashboard body, and `question-dolt-state-engine` names Dolt as the Phase-2
candidate. **What concrete shape does Phase 0 take** so that:

- the shape survives the Phase-0 → Phase-2 migration without a rewrite of the
  code that reads and writes it;
- every mutating action beadle performs is captured as an appendable event, so
  a wiped dashboard body can be regenerated purely from the store;
- the store is honestly diffable in a PR (append-only, canonical JSON);
- and the schema tolerates unknown future record kinds without breaking older
  readers.

Named cluster of open sub-questions:

- **A.** What record kinds are load-bearing at Phase 0? (run/issue/
  classification/comment/cluster/note — or fewer?)
- **B.** How is watermark derived — a stored scalar, or a derivation over
  observed issue records? (The latter is idempotent under partial rebuilds.)
- **C.** How do you seed an established target (already 9 dashboard runs deep)
  without re-running historical inference?
- **D.** How honestly can maintainer engagement be *derived* rather than
  hand-encoded? Prior runs authored `filed_vs_acted_gap: 161:0` from
  inspection; the store should be able to *prove* that number from data.

## Hypothesis

An append-only JSONL file with a tagged-enum record type — plus a
forward-compat `Other` catch-all — is the smallest thing that:

1. survives Dolt migration (Dolt loads JSONL directly);
2. keeps the "one row is one atomic observation" invariant even under a crash
   mid-write;
3. lets the enumerator and comment-sweep run independently without a schema
   coordinator (each appends its own record kinds);
4. lets a renderer materialize views by reading the full log once (O(N), N in
   the low thousands at Phase 0 scale).

The record kinds are: `run`, `issue`, `classification`, `comment_event`,
`cluster`, `note`. Watermark is derived (max of observed issue numbers).
Seeding is a one-shot script that (a) reads the current dashboard sentinel for
already-decided classifications/clusters, (b) fetches open issues fresh from
`gh` for the current observation frame.

## Timebox and success signal

Timebox: one session (the code is the probe medium — ADR-006).

Success signal — three things:

1. **End-to-end round-trip:** seed + enumerate + sync executes against
   vsdd-factory (a live target with 180 open issues, 9 runs of dashboard
   history) without hand-editing the resulting JSONL.
2. **Derived compass:** the maintainer-engagement number the dashboard was
   *asserting* falls out of the store as a query over `comment_event` rows.
   The number matches (or the discrepancy is illuminating).
3. **Migration-honest:** re-reading the JSONL after a schema addition
   (unknown kind) does not lose data — the reader preserves the row verbatim
   as `Record::Other` and a rewrite reproduces it byte-identical.

## Confidence tier of the probe artifacts

- The Rust crates (`beadle-store`, `beadle`) start at **frontier** confidence
  per ADR-006 — code as probe medium, not yet bedrock.
- The Perl seed is **placeholder** — a one-shot to bridge the "there is
  already a run-9 dashboard" state; not part of the ongoing pipeline.
- The record schema in `docs/store-schema.md` is **frontier** — the specific
  fields will iterate as `render` uses them.

## Related nodes and findings

Refines: `elem-state-out-of-band` (Phase 0 shape now concrete).
Composes: `elem-maintainer-compass` (the compass now derivable, not asserted).
Feeds: `question-dolt-state-engine` (Phase 2; this brief bounds the migration).
Opens (see finding): the render-side question about editorial vs derived zones.
