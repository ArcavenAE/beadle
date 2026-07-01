# finding-011 — Phase-0 JSONL store shape, and the compass falls out of it

Date: 2026-07-01 (session 052)
Probe: `_kos/probes/brief-store-shape-phase-0.md` — what concrete shape does
the out-of-band store take at Phase 0, and can maintainer engagement be
derived from data rather than asserted from inspection?

## Summary

The Phase-0 JSONL shape is settled enough to build the renderer against, and
one non-obvious thing fell out that the probe brief did not predict: **once
the store exists, the dashboard's assertions about maintainer engagement stop
being editorial and start being queries**. The claim `filed_vs_acted_gap:
161:0` that run 9 hand-authored on inspection is reproduced *exactly* by
counting rows in the store, and the reasons why maintainer engagement is zero
are individually addressable (one row per comment, actor classified against
the intent manifest).

That reframes the improvement plan: item B (cluster decay) and item C
(body-budget guard) are the *cheap* parts of the renderer. The load-bearing
part is drawing the line between what the machine derives and what the
editor still writes — and doing it in a way that the editor cannot
accidentally launder a machine claim.

## What the probe proved

- **Append-only JSONL round-trips cleanly** across a Perl writer (one-shot
  seed) and a Rust reader/writer (ongoing pipeline). Canonical-JSON
  serialization with sorted keys keeps diffs stable.
- **Watermark-as-derivation works.** The enumerator's watermark is `max(issue.number)`
  observed in the store, not a stored scalar. This survived a real
  disagreement gracefully: the seed captured up to #381 (run-9 sentinel), the
  first live enumerate saw #382 and #383 (filed between run 9 close and this
  session), and the store simply took the new max. No reconciliation code.
- **The `Record::Other` catch-all** for unknown `kind` fields lets us evolve
  the schema without version handshakes. Old readers preserve unknown rows
  verbatim; a rewrite is byte-identical.
- **Comment fingerprinting `(number, ts, actor)` is idempotent.** Re-running
  sync appended zero rows. This is the invariant that makes the delta-sweep
  cheap to schedule (a cron can just run it).

## The non-obvious thing

Once the store existed, one `gh issue list --json number,comments` call over
180 open issues emitted **157 comment events**. Every single event's actor
was `arcavenai` (149) or `arcaven` (8) — both `measured` per the intent
manifest. Zero `maintainer` events. Zero `other` events.

This is the exact number the dashboard was asserting. But now:

- The number is a query, not an editorial position. Which means it is
  auditable.
- The number is *decomposable*: 157 events across N unique issues, oldest
  event T, newest event T', highest event-density issue #X. None of these
  were computable before the store existed.
- The claim "no maintainer has acted" is no longer a claim — it is the
  arithmetic result of a filter over the store. A run of the dashboard that
  said "maintainer_engaged: 3" would have to be a lie about the data or a
  labeling bug, not a difference of opinion.

This is the compass (elem-maintainer-compass) becoming *load-bearing* in the
literal sense: it now supports weight it did not before.

## What the probe did not settle

- **Render-side editorial boundary.** The renderer can materialize the
  sentinel block, the counts table, the per-cluster member lists, freshness
  timestamps, and cluster-decay state. It **cannot** materialize the
  direction verdict paragraph, quick-win eligibility, or per-issue verdict
  chips — those depend on classifications the store does not yet hold.
  The clean move is: renderer produces a body with editorial slots marked as
  such, editor fills them in, and the sentinel digest is computed over the
  full merged body so hand-edits to editorial slots are detected on re-run.
  Opens: `question-renderer-editorial-boundary` (frontier).
- **Where classifications come from.** The current session did not add any
  `classification` records. The next probe is whether classifications get
  authored by hand (per-issue chip in the dashboard body → row in store),
  scripted from labels (github label → axis value), or lifted from finding-005/009
  by a classifier agent. The store *accepts* them either way.
- **Backfill of historical runs.** The seed only captured run 9's frame.
  Runs 1–8 are lost to the store. If cross-run trend is going to matter
  (finding-008 says it lives in critic, not beadle), the store does not need
  them. If some in-beadle "over the last N runs" logic emerges, backfill
  becomes a probe.

## Nodes touched

- **Refines** `elem-state-out-of-band` (Phase-0 shape now concrete).
- **Strengthens** `elem-maintainer-compass` — the compass is now data, not
  assertion. Node body updated to name the derivation.
- **Opens** `question-renderer-editorial-boundary` (frontier) — where does
  the line between machine-derived and editor-authored sit in the rendered
  body, and how does the sentinel digest catch hand-edits that cross the
  line?
- **Bounds** `question-dolt-state-engine` — the Phase-2 migration inherits
  the record kinds established here. Dolt loads JSONL directly, so migration
  is a schema-transcription exercise, not a data-transformation exercise.
- **Composes** finding-008 (critic-owns-measurement) — beadle owns the
  *observations* (rows in the store); cross-run rate/trend analysis over
  those rows still routes to critic.

## Confidence promotions this cycle

None yet. The Rust crates stay frontier. The JSONL schema stays frontier.
Promotion to bedrock is gated on `render` shipping (proves the record shape
supports at least one useful materialization) and on one more independent
target being seeded (proves the shape is not vsdd-factory-specific).

## Provenance

Probe medium: code (Rust workspace, Perl seed) — ADR-006.
Cited artifacts:
- `crates/beadle-store/src/lib.rs` — record kinds and canonical serializer.
- `crates/beadle/src/enumerate.rs` — watermark-as-derivation.
- `crates/beadle/src/sync.rs` — comment fingerprint and role classifier.
- `scripts/seed-store.pl` — Phase-0 seed for run-9-established targets.
- `store/vsdd-factory/state.jsonl` (local, gitignored) — 351 rows, 180 issue
  observations + 157 comment events + 13 clusters + 1 run.
