# ADR-004: Skill → sideshow-pack → marvel team (gradual elaboration)

## Status
Accepted

## Date
2026-06-27

## Context
beadle is described as eventually being a team of agents with CLI tools, hooks,
state tracking, and a revision-controlled database. Building all of that before a
concrete need violates SOUL.md tenet 7 (gradual elaboration) and risks an
over-built system that is useful at none of its phases.

## Decision Drivers
- SOUL.md gradual elaboration: build what you need, not what you might need.
- Each phase must be independently useful.
- Reuse the proven multiclaude envoy team rather than reinvent it.

## Considered Options
1. Build the full marvel-orchestrated agent team + Dolt up front.
2. Phase it: skill now; pack next; team last.

## Decision Outcome
**Chosen:** option 2.
- **Phase 0 (now):** a Claude Code skill (`skills/beadle-triage`) runs the five
  engines and maintains the dashboard. Composes `arcavenai-issue-review`.
- **Phase 1:** packaged as a sideshow-pack (`pack.yaml`); a scheduled `gh-aw`
  workflow runs the pass *as arcavenai*; checkbox controls added.
- **Phase 2:** the pack is consumed by marvel as a triage Team ("multiclaude v2");
  durable state migrates to Dolt (frontier F2). Reuses the multiclaude agent
  format, label discipline, heartbeat-via-cron, session-handoff, and the four
  incident-hardened rules.

### Positive Consequences
- Useful on day one; no speculative machinery.
- Reuses battle-tested team scaffolding and its incident hardening.

### Negative Consequences
- Two migrations (state JSONL→Dolt; skill→pack→team) to execute later.
- Phase boundaries must be honored, not collapsed under enthusiasm.
