# ADR-002: Propose-not-act, per-stage autonomy, no auto-close

## Status
Accepted

## Date
2026-06-27

## Context
beadle runs autonomously, *as the arcavenai identity*, posting publicly. Its
contributions reflect on the user. The human-AI collaboration canon (automation
bias, alarm fatigue, the irony of automation, meaningful human control) and the
open-source AI-slop crisis (curl: ~5% genuine reports; fabricated references) both
constrain how much an autonomous agent should act publicly.

## Decision Drivers
- Reputational risk of public posts under an identity.
- Reversibility and stakes vary sharply by action type.
- Over-escalation desensitizes the human (alarm fatigue); under-escalation hides
  real problems.

## Considered Options
1. Fully autonomous (auto-post everything, auto-close stale).
2. Fully manual (propose everything, human approves each write).
3. Per-stage autonomy: high for acquire/analyze; graduated for act.

## Decision Outcome
**Chosen:** option 3 (Parasuraman-Sheridan-Wickens levels-of-automation).
- **acquire / analyze** (poll, validate, classify, score): autonomous, read-only.
- **act — labels**: autonomous for bounded, allow-listed, reversible labels, with
  mutual-exclusivity enforced.
- **act — public comments**: high-bar (must clear the careful-human bar; cannot
  verify a claim → must not post), volume-capped, soft-toned, bot-disclosed.
- **act — close / resolve**: **never autonomous** — escalate. No stale-bot;
  information density is a protection signal, not a deletion signal.
Confidence is surfaced as frequency; evidence is cheap-to-verify links; every
action is reversible with an audit trail and named-supervisor attribution
(EU AI Act Art. 14 oversight baseline). Never optimize a proxy metric (Goodhart):
counts are always paired with an outcome signal.

### Positive Consequences
- Public output meets the human bar; reputational risk contained.
- Reversibility matched to stakes; no irreversible action on a cheap signal.

### Negative Consequences
- Lower throughput on closures (by design).
- Requires a maintained escalation path and audit trail.
