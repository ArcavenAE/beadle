# beadle agents

Agent definitions in the multiclaude 9-section format (see
[`../docs/agent-team.md`](../docs/agent-team.md)). In Phase 0 these are the
sub-roles the `beadle-triage` skill plays; in Phase 2 they are spawned as a
marvel-orchestrated team.

| File | Role | Persistent? | Writes? |
|---|---|---|---|
| [`orchestrator.md`](orchestrator.md) | dispatch + serialize all writes + escalate | persistent | the only writer |
| [`envoy.md`](envoy.md) | greet/screen/label/cross-check/relay reporters | persistent | proposes only |
| `intent-scorer.md` | graded alignment verdict vs the target intent | ephemeral | read-only · *TODO* |
| `validator.md` | already-fixed / fix-not-fixed / citation-exists | ephemeral | read-only · *TODO* |
| `classifier.md` | type / severity / priority / leverage | ephemeral | read-only · *TODO* |
| `investigator.md` | selective deep-dive memo | ephemeral | read-only · *TODO* |
| `verifier.md` | fresh-context re-check of high-stakes verdicts | ephemeral | read-only · *TODO* |
| `dashboard-keeper.md` | regenerate the dashboard projection | persistent | dashboard issue only · *TODO* |

The two foundational roles (orchestrator, envoy) are written; the ephemeral
read-only engines are stubbed for Phase 2 and authored when the team is stood up
(gradual elaboration, ADR-004).
