# ADR-001: Durable state lives out-of-band; the dashboard is a projection

## Status
Accepted

## Date
2026-06-27

## Context
beadle maintains a living dashboard as a single GitHub issue, emulating Renovate's
Dependency Dashboard. A prior-art survey found that every bot-maintained dashboard
shares one failure mode: **state-in-Markdown**. Renovate parses its own rendered
issue body as machine state and is left "in a broken state" when a human edits or
clears the body (renovatebot/renovate#19563).

## Decision Drivers
- The dashboard must survive human hand-edits and a wiped body.
- State must be auditable and (eventually) diffable over time.
- The body must stay under GitHub's 65,536-char limit.

## Considered Options
1. State in the issue body (Markdown-as-truth) — the Renovate model.
2. State out-of-band; the body is a regenerated projection.

## Decision Outcome
**Chosen:** option 2. Durable state lives out-of-band — Phase 0 append-only JSONL,
Phase 2 Dolt (frontier F2). The issue body is regenerated each run as a read-only
projection; only a digest + pointer live in a `<!-- beadle-state -->` sentinel.
beadle never trusts a hand-edit and regenerates cleanly from the store.

### Positive Consequences
- Robust to vandalism / hand-edits; no broken-state failure.
- Auditable history independent of GitHub's issue edit log.
- Checkboxes are derived from authoritative state, not authored by hand.

### Negative Consequences
- A store to maintain and migrate (JSONL → Dolt).
- The body and store can momentarily diverge (eventually-consistent control loop).
