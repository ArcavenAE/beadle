# The beadle agent team

beadle absorbs the multiclaude **envoy** role and reuses the team shape, agent
format, label discipline, and incident-hardened guardrails from
`ArcavenAE/multiclaude-enhancements`. This document is the build reference.

## Phase 0 → Phase 2

- **Phase 0:** one Claude Code session runs the five engines as sub-routines (the
  `beadle-triage` skill).
- **Phase 2:** a marvel-orchestrated team (Workspace `triage`, one Team, several
  Roles). The orchestrator serializes writes; everyone else fans out as reads.

## Roles

| Role | Persistent? | Writes? | Absorbs / new | Responsibility |
|---|---|---|---|---|
| **orchestrator** | persistent | **the only writer** | new (≈ supervisor) | dispatch, serialize all GitHub/store writes, escalate to human |
| **envoy** | persistent | proposes only | absorbed | greet/screen/label reporters, cross-check merged PRs, relay |
| **classifier** | ephemeral (×N) | read-only | new | taxonomy + severity/priority + leverage axis |
| **intent-scorer** | ephemeral (×N) | read-only | new | graded alignment verdict vs the target intent manifest |
| **validator** | ephemeral | read-only | new | already-fixed / fix-not-fixed / citation-exists / reproducibility |
| **investigator** | ephemeral | read-only | new | selective deep-dive memo for high-stakes/contested issues |
| **verifier** | ephemeral | read-only | new (fresh-context adversary) | re-check high-stakes verdicts against ground truth |
| **dashboard-keeper** | persistent | writes the dashboard issue only | new | regenerate the projection from the store |

Reused unchanged from multiclaude where a target already runs them:
project-watchdog (PM governance, the serialized **number-allocation chokepoint** —
INC-003), arch-watchdog (architecture drift), retrospector (improvement loop),
research-supervisor (external research).

## Division of labor (write domains — non-overlapping)

| Domain | Owner | Read-only consumers |
|---|---|---|
| GitHub issue comments + labels | orchestrator (envoy proposes) | everyone |
| the dashboard issue body | dashboard-keeper | everyone |
| the out-of-band store | orchestrator | classifier, scorer, investigator (read) |
| target repo code/specs | (beadle never writes target code) | all (read for cross-check) |

No overlapping write domains → no inter-agent merge conflicts. Worktree isolation
is architectural, not a definition-level guardrail (INC-001).

## Agent-definition format (9 sections)

Author every agent in `agents/` in the multiclaude shape:

1. **Responsibility** — 1–2 sentence mission.
2. **WHY this role exists** — the problem it solves.
3. **Incident-Hardened Guardrails** — `### INC-NNN` rules carried verbatim.
4. **Authority** — a table: **CAN** (autonomous) / **CANNOT** (forbidden) /
   **ESCALATE** (requires human).
5. **Interaction Protocols** — `### With <role>` subsections.
6. **Operational sections** — Polling Loop, HEARTBEAT Response, Session Handoff
   (persistent roles only).
7. **Communication** — the INC-004 warning **verbatim**: use the platform channel
   (`multiclaude message send` / the marvel transport), never the in-process
   `SendMessage` tool.
8. **Confidence & evidence** — surface confidence as frequency; cite cheap-to-verify
   links, not prose.
9. **What you do NOT do** — explicit forbidden actions.

## Label authority (aae-orc schema)

Use `../../labels/schema.yaml`. Scoped, with **mutual exclusivity enforced by
convention** (GitHub doesn't enforce it). Before applying a label in an exclusive
scope (`type.*`, `priority.*`, `triage.*`, `scope.*`, `resolution.*`): query
current labels → remove the existing one in that scope → apply the new one.

| Scope | beadle authority |
|---|---|
| `type.*`, `priority.*`, `triage.*`, `contrib.*`, `status.stale`, `agent.*` | set autonomously |
| `scope.*`, `resolution.*` | **propose only** — human/supervisor confirms |

## Incident-hardened rules (carried from multiclaude)

- **INC-001** — worktree isolation is architectural; no shared mutable checkout.
- **INC-002** — never cargo-cult procedure; state WHAT + WHY, let the implementer
  choose HOW; validate infra assumptions against real tool behaviour.
- **INC-003** — cross-artifact identity allocation is a serialized chokepoint;
  advisory registries don't prevent races; sequential dispatch with pre-assigned
  IDs.
- **INC-004** — inter-agent comms via the platform channel only; enforce
  mechanically with a PreToolUse block on `SendMessage` to known agent names.
- **Context exhaustion** — persistent agents lock up after ~12h / ~20 cycles;
  restart every 4–6h; supervisor health-check every 30 min; session-handoff
  preserves state.
- **Hot-reload** — agents can't hot-reload prompts; kill + respawn after editing a
  definition. Never tell a running agent to "re-read your definition."

## Multi-agent design rules (cross-vendor consensus)

- Parallelize reads/research/review; **serialize all writes through one thread**
  (Cognition: conflicting parallel decisions produce irreconcilable output)
  [@cognitionmulti].
- Give every sub-agent an objective, output format, source guidance, and explicit
  boundaries — vague delegation causes duplicate work [@anthropicmulti].
- Verification is a **dedicated fresh-context** agent checking output against
  ground truth, never the producer's memory [@cognitionmulti].
- Invest in decision-trace observability — minor changes cascade.
