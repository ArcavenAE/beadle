# beadle Charter

Intent-aligned issue-triage and dashboard system. The next generation of the
multiclaude **envoy** role, extended with per-artifact intent-alignment scoring
weighted by maintainer-action signal, and a bot-maintained dashboard whose state
lives out-of-band.

Follows the kos process: Orient → Ideate → Question → Probe → Harvest → Promote.
Authoritative graph: `_kos/nodes/`. Cross-repo questions belong in the
orchestrator's charter.

Last updated: 2026-06-28 (harvest: findings 001–006 — silent-integrity severity,
defect-classification superset + dual-audience render, two-tier controls, quick-wins
lane, dashboard-discovery robustness, maintainer-progress. Detail in the nodes; the
charter carries the load-bearing decisions only).

---

## Bedrock

*Established. Evidence-based or decided with rationale.*

### B1: Durable state lives out-of-band; the dashboard is a projection

The single most repeated failure mode across bot-maintained dashboards is
**state-in-Markdown**: Renovate parses its own rendered issue body as machine
state and breaks when a human edits it (renovatebot/renovate#19563). beadle's
source of truth is an out-of-band store (Phase 0: JSONL; Phase 2: Dolt). The
GitHub dashboard issue is **regenerated each run as a read-only projection** of
that store. beadle never trusts hand-edits to the body, and tolerates a wiped
body by regenerating cleanly. ADR-001. `_kos/nodes/bedrock/elem-state-out-of-band.yaml`.

### B2: Propose-not-act for anything consequential or public

Automation level is set **per stage, independently** (acquire / analyze / decide
/ act — Parasuraman-Sheridan-Wickens). beadle reads and analyzes with high
autonomy; it **proposes** for anything consequential or reader-facing. Bounded,
reversible, additive actions (an allow-listed label, a dashboard rewrite) may run
autonomously; free-text public comments and any closing/resolution action default
to a review path with a high precision bar and volume caps. Every public output
must clear the same quality bar a careful human contributor meets, or it is not
posted (the curl "AI-slop" rule). ADR-002.
`_kos/nodes/bedrock/elem-propose-not-act.yaml`.

### B3: Maintainer engagement is the compass; volume is the cargo

beadle weights work by **what maintainers actually act on** (respond / label /
close / reference / merge), reconstructed from role-filtered behavioural events.
Reaction-popularity is "demand to reconcile, never equate." Contribution volume —
especially from an automated filer — is the thing being measured, never the
signal of value. This is the input that lets beadle detect "led-by-the-backlog"
drift. `_kos/nodes/bedrock/elem-maintainer-compass.yaml`.

### B4: Per-artifact intent-alignment scoring is the differentiator (white space)

A genuine prior-art survey found no shipping tool that semantically scores an
individual issue/PR against a *declared, versioned project intent* and emits a
graded verdict with cited rationale. PM tools score at roadmap granularity with
no published method; traceability-ML answers link-existence, not alignment
quality; GORE has the formalism but dies on manual modelling. beadle occupies the
intersection: per-artifact granularity + semantic scoring against a
machine-readable intent + automated/continuous. `docs/prior-art.md` §white-space.
`_kos/nodes/bedrock/elem-intent-alignment-whitespace.yaml`.

### B5: Gradual elaboration — ship the skill, earn the team

Per SOUL.md tenet 7, beadle is the simplest thing that works first: a Claude Code
skill that maintains the dashboard and runs the triage lenses today. CLI tools,
hooks, a marvel-orchestrated agent team, and the Dolt state engine are added when
a concrete need demands them, not pre-built. Each phase is independently useful.
ADR-004.

### B6: Reuse the multiclaude envoy team, don't reinvent it

beadle absorbs the envoy role and reuses the proven team shape from
`ArcavenAE/multiclaude-enhancements`: the 9-section agent-definition format
(Responsibility / WHY / Incident-Hardened Guardrails / Authority CAN-CANNOT-
ESCALATE / Interaction Protocols / operational sections / Communication / What
You Do NOT Do), the scoped mutually-exclusive label taxonomy, heartbeat-via-cron
(no internal timers), and session-handoff for context-exhaustion survival.
`docs/agent-team.md`.

### B7: Incident-hardened by inheritance

The four multiclaude incidents are load-bearing constraints, not history:
- **INC-001** (shared-checkout contamination) → worktree isolation is
  *architectural*, never a definition-level guardrail alone.
- **INC-002** (destructive git override) → never cargo-cult procedure; state the
  WHAT and WHY, let the implementing agent choose the HOW.
- **INC-003** (epic-number collision) → cross-artifact identity allocation is a
  *serialized chokepoint*; advisory registries don't prevent races.
- **INC-004** (`SendMessage` silent-drop) → inter-agent comms use the platform
  channel, mechanically enforced (PreToolUse block), never the in-process tool.

### B8: No rate/drift verdict before the process has turned (cold-start)

An average rate computed over a window before the triage process has engaged is
**initialization bias, not drift**. A young backlog whose maintainers are
busy-but-willing and haven't begun their first pass is in a cold-start / warm-up
transient — the wheel hasn't turned. beadle gates direction verdicts on warm-up
state: until a baseline exists (≥1 completed maintainer triage cycle, or a
configured threshold), the verdict is **COLD START / BASELINE**, never DRIFTING,
and the dashboard's job is to *establish* the baseline and make the first pass
cheap. Rate/trend/drift reporting begins only after the transient ends — the
metrics are "not meaningful *yet*," not meaningless. Distinct from genuine
steady-state drift (B3), which is a sustained divergence *after* the process has
demonstrably turned. ADR-005.
`_kos/nodes/bedrock/elem-cold-start-warmup.yaml`.

### B9: Silent integrity / source-of-truth corruption is top severity; integrity gates functional

A fault that makes a *system of record* (ratchet, spec, hash, index, decision log,
learning store) disagree with reality **without raising a signal**, and propagates,
is the **highest** severity class — *regardless of how small the triggering bug
looks*. Severity comes from invisibility + compounding, not immediate loss.
Recoverability tier is a severity input (regenerable output < spec/process <
irreplaceable learning). And integrity **gates** functional: a functional verdict —
including "converged / PASS" — is unfalsifiable on a substrate that can't be trusted,
so an open integrity defect doesn't merely outrank a functional one, it gates the
validity of every functional verdict. P0 "Source-of-truth integrity" always sits
above convergence-soundness. finding-004.
`_kos/nodes/bedrock/elem-silent-integrity-severity.yaml`.

### B10: The classify model is a research-grounded superset + beadle-original axes

beadle re-classifies every artifact (never trusting its self-label) on research-grounded
axes — report-type, defect-nature (mechanical→conceptual, IEEE 1044 / ODC),
reproducibility (Bohr/Mandel/Heisen, feeding priority's effort term), triage-state,
severity-vs-priority — plus beadle-original axes (leverage, alignment B4, provenance,
the B9 integrity cluster). The dashboard renders this **title-leads / verdict-trails**
and **dual-audience** (LLM-first, human-supported): short titles for humans, the axis
vector + cited rationale folded/embedded for agents, references linked never replicated
(B1). A **quick-wins / safe-action lane** — derived from the axes, orthogonal to the
impact ordering, integrity-excluded (B9), never a scored metric (no-Goodhart) — surfaces
low-caution work to exercise the maintainers' process. finding-005, finding-006.
`_kos/nodes/bedrock/elem-defect-classification-superset.yaml`.

---

## Frontier

*Actively open. Expected to resolve through probes or design work.*

### F1: The intent-manifest schema

Each target declares its intent so beadle can score against it. Open: what is the
minimal, versioned, machine-readable schema? For vsdd-factory the anchor is a
*composite* — `CLAUDE.md` canonical principle + `STATE.md` cycle goals + `docs/` +
git history + maintainer (DrBothen/Zious11) comments + a future `SOUL.md`. The
generic form must express goals, non-goals, scope boundaries, and an alignment
rubric without becoming a doc nobody maintains (the GORE failure mode). First
concrete requirements answered (live at v0.3): `self_referential`,
`provenance_signal`, corpus-vs-per-issue minutiae scope (finding-002), and
`integrity_anchors` by recoverability tier (finding-004/005).
`_kos/nodes/frontier/question-intent-manifest-schema.yaml`.

### F2: State engine — Dolt vs DoltLab/DoltHub

Phase-2 durable state wants revision control: diffable, branchable, auditable
issue/stat history. Dolt (Git-for-data SQL) is the leading candidate; DoltLab
(self-hosted) / DoltHub (hosted remote) is the question for a shared remote. Not
in the MVP. `_kos/nodes/frontier/question-dolt-state-engine.yaml`.

### F3: The marvel team shape ("multiclaude v2")

How does the beadle team map onto marvel's resource model (Workspace / Team /
Role / Session)? Which roles are persistent, which ephemeral? Who serializes
writes (the orchestrator), who is read-only (classifiers, the fresh-context
verifier)? Blocked partly on marvel's pack-integration frontier (marvel F3).
`_kos/nodes/frontier/question-marvel-team-shape.yaml`.

### F4: Packaging — sideshow-pack contents and distribution

What does the beadle sideshow-pack distribute, at what scope? Skill bindings,
rules, hooks, the dashboard schema, per-target intent manifests. Reconcile
marvel ADR-002 ("packs live in marvel") with sideshow as the content-pack
manager. `pack.yaml` is the working draft.

### F5: Checkbox control surface

Renovate's `- [ ] <!-- verb=target -->` checkbox-as-control pattern, but with
state out-of-band: derive the checkboxes from authoritative state, parse `[x]` on
the next scheduled run, act, reset. Which beadle actions are safe to expose as
maintainer-triggered checkboxes (approve-fast-track, request-investigation,
accept-deferral)? **Two tiers** (finding-003): Tier 1 per-issue verbs (`id=#NN`) +
Tier 2 board-level maintenance requests (`id=board`: reprioritize / full-refresh /
revalidate / rescore-intent) with a cheap-poll-then-act discipline; Tier 2 is
read/analyze/regenerate only — irreversible public actions still escalate per B2.
`_kos/nodes/frontier/question-maintenance-request-controls.yaml`.

### F6: Intent-alignment as a held-out evaluation

Borrow StrongDM's holdout discipline: keep some alignment scenarios *out of the
scoring agent's view* so the verdict can't be taught-to-the-test, and score
satisfaction probabilistically rather than binary. Is this tractable for issue
triage, or only for the larger factory loop?

### F7: Dashboard discovery robustness

Discovery keys on the `<!-- beadle-state -->` sentinel first, exact title second;
union of both, filtered to the beadle identity; >1 candidate → STOP and request
consolidation; re-check before create. Open: the concurrency guarantee — creation
needs a serialized chokepoint (INC-003 class) once Phase 1 (scheduled gh-aw) or
Phase 2 (marvel) can run concurrent passes against one target. finding-001.
`_kos/nodes/frontier/question-dashboard-discovery-robustness.yaml`.

### F8: Maintainer-progress without a leaderboard

Make resolution against beadle-discovered defects visible and rewarding without
becoming a gameable metric. Reward the **verified fix-outcome** (flagged → fixed →
validates), never close-rate / time-to-triage / volume (B3 + no-Goodhart). Open: the
full reward design; Phase 0 renders a minimal honest version. finding-005.
`_kos/nodes/frontier/question-maintainer-progress-gamification.yaml`.

---

## Graveyard

*Ruled out, archived with rationale.*

### G1: State in the issue body (Markdown-as-truth)

Rejected. The universal failure mode of bot dashboards. State is out-of-band;
the body is a projection. `_kos/nodes/graveyard/grv-state-in-markdown.yaml`.

### G2: Auto-close on inactivity (stale-bot)

Rejected. Inactivity ≠ irrelevance ≠ resolution; it destroys the
highest-information reports and spawns duplicates (the documented stale-bot
backlash). Information density is a *protection* signal. beadle labels and
surfaces; it does not auto-close on a timer.
