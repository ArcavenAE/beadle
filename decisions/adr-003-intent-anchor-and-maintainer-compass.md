# ADR-003: Per-target intent anchor, weighted by maintainer signal

## Status
Accepted

## Date
2026-06-27

## Context
beadle's differentiator is scoring work against project intent. To stay generic
(many targets: vsdd-factory, Prism, other ArcavenAE repos) nothing about a target
may be hardcoded. The risk is two-fold: (a) an intent doc nobody maintains (the
GORE failure mode), and (b) measuring the wrong thing — equating contribution
volume or reaction-popularity with value.

## Decision Drivers
- Generic across repos; specialized per project.
- Intent sources should already exist in the repo, to minimize maintenance rot.
- The signal of value is what maintainers act on, not what is filed.

## Considered Options
1. A single dedicated north-star doc per repo.
2. The aae-orc SOUL.md alone.
3. A composite, weighted anchor (existing repo docs + dynamic maintainer signal).

## Decision Outcome
**Chosen:** option 3, declared in `targets/<project>.intent.yaml`. The anchor is
composite and weighted toward maintainer voice. For vsdd-factory: CLAUDE.md
(canonical principle) + .factory/STATE.md (cycle goals) + docs/ + git history +
DrBothen/Zious11 comments + a future SOUL.md. **Maintainer engagement is the
compass** (respond/label/close/reference/merge, role-filtered); reaction-popularity
is "demand to reconcile, never equate"; contribution volume is the cargo being
measured. The gap between filed and acted-upon is the primary drift signal.

### Positive Consequences
- No hardcoding; new targets onboard by writing a manifest.
- Measures direction (maintainer-valued) over volume — the anti-"led-by-the-backlog"
  lever.

### Negative Consequences
- The manifest schema is still frontier (F1); early targets carry v0 risk.
- Reconstructing maintainer-action signal cleanly is non-trivial.
