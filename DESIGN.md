# beadle — Design

Status: initial architecture, bootstrapped 2026-06-27. This document is the
synthesis; `charter.md` records what is bedrock vs frontier, and the `decisions/`
ADRs record the load-bearing choices. Sources cited inline resolve via
`references.bib`.

---

## 1. Problem

A prolific contributor — increasingly an AI agent (arcavenai/Skippy) — can file
sound, well-written issues faster than maintainers can absorb them. Each is
individually defensible. In aggregate they:

- pull a project off its roadmap ("led by the backlog");
- bury high-leverage work under low-leverage minutiae (death by a thousand
  papercuts);
- invert the cost of being helpful — the maintainer spends more time triaging
  than the contribution saves.

The contributions are public and reflect on their author. The maintainers
(DrBothen, Zious11) need to assess, triage, and action improvements **without the
project going off the rails**, and to catch when they are being led around by
details to net-negative effect.

## 2. The white space beadle owns

A genuine survey of prior art (`docs/prior-art.md`) places every existing tool on
one axis — *how much it acts, how reversibly, on how cheap a signal* — and finds
one cell empty:

> No shipping tool semantically scores an **individual issue / PR / commit**
> against a **declared, versioned project intent**, emitting a graded verdict with
> cited rationale, **weighted by reconstructed maintainer-action signal**, with a
> **bot-maintained dashboard whose state lives out-of-band**.

The adjacent cells are occupied: PM tools name "strategic drift" but score at
roadmap granularity with no published method [@airfocus]; engineering-intelligence
platforms measure time-allocation, not work-vs-mission [@jellyfish]; spec-conformance
checks code-vs-spec, not work-vs-intent; goal-oriented RE (GORE/KAOS) has the
graded-satisfaction formalism but dies on manual modelling [@gore]. beadle needs
all three at once: per-artifact granularity, semantic scoring against a
machine-readable intent, and continuous automation. That intersection is open.

## 3. Lineage: the next-generation envoy

beadle is built on `ArcavenAE/multiclaude-enhancements`. It **absorbs the envoy
role** (community go-between: greet, screen, label, cross-check, relay — never
authorize) and reuses the team's proven scaffolding. The new capability layered on
top is intent-alignment scoring + the dashboard. The envoy's "three-layer
firewall" becomes beadle's intake pipeline:

- **Layer 1 — deterministic gates** (no AI): spam, duplicate, already-fixed,
  previously-decided. (This is where the `arcavenai-issue-review` already-fixed /
  fix-not-fixed checks live.)
- **Layer 2 — lightweight AI screening**: intent-alignment, classification,
  scope, priority.
- **Layer 3 — escalate**: architecturally complex / direction-changing / systemic
  issues go to a human or a deliberation, never decided by beadle.

## 4. The five engines (composable skills → agents)

The system decomposes into composable units. In Phase 0 they are sub-routines of
one skill; in Phase 2 they are marvel-orchestrated agents.

1. **validate** — the already-fixed + fix-not-fixed + reproducibility + citation
   engine. Syncs the target checkout, verifies cited symbols actually exist
   (catch hallucinated citations), confirms a candidate fix is load-bearing (a
   real test exercises the failure, not a paper-fix). Verdict ∈ {real-as-filed,
   fixed-on-default, fixed-pending-release, already-addressed, unreproducible,
   hallucinated-citation, needs-info}.
2. **classify** — taxonomy + prioritization + **alignment**. Two independent axes
   (severity = impact, priority = urgency, per the testing canon [@severitypriority]);
   a leverage axis (systemic ↔ minutiae); and the alignment axis (advances ↔
   neutral ↔ drifts) scored against the target intent. Priority defaults low and
   escalates only on evidence (Mozilla's P3-default; Kubernetes' "no label ≠ low")
   [@k8striage; @moztriage].
3. **investigate** — selective deep-dive for ambiguous / high-stakes / contested
   issues. Fans out reads across code, specs, related issues, maintainer comments,
   git history; produces a memo: what's true, what a fix touches, risks,
   recommended action.
4. **score-intent** — the differentiator. Reads the target's intent anchor and
   emits a graded, cited alignment verdict. Borrows GORE's partial-satisfaction
   grading and StrongDM's held-out / probabilistic-satisfaction discipline
   (`docs/PHILOSOPHY.md`).
5. **dashboard** — renders the issue-as-dashboard projection from the store and
   maintains it idempotently (`docs/dashboard-schema.md`).

The orchestrator serializes all writes through one thread; engines 1–4 fan out as
reads/analysis; a fresh-context verifier checks high-stakes verdicts against
ground truth (the Anthropic/Cognition multi-agent consensus: parallelize reads,
serialize writes, verify with independent context) [@anthropicmulti; @cognitionmulti].

## 5. The dashboard (issue-as-dashboard)

Emulate Renovate's Dependency Dashboard mechanics, **fix its fragility**
(`docs/dashboard-schema.md`):

- **One title-discovered, pinned issue**, body fully rewritten each run.
- **State lives out-of-band** (Phase 0: JSONL; Phase 2: Dolt). The body is a
  projection; a `<!-- beadle-state ... -->` sentinel block carries only a digest +
  pointer, never the authoritative record. Hand-edits are ignored; a wiped body
  regenerates. (Renovate breaks here — renovatebot/renovate#19563 — beadle does
  not.) [@renovatedash; @renovate19563]
- Body sections: **Progress** (stats + trend deltas), **Action plan** (P0/P1/P2
  with recommended action + linked issue), **Direction health** (the
  intent/drift panel: minutiae ratio, filed-vs-acted, net-flow, scope-drift
  candidates), **Classification index** (human-readable table), and **Controls**
  (checkbox lines `- [ ] <!-- verb=target -->` that the next run reads, acts on,
  and resets — F5).
- Stay under the 65,536-char body limit; offload detail to linked sub-issues or
  the store; collapse with `<details>`.

## 6. Per-target intent anchor

Nothing is hardcoded. Each target declares its intent in
`targets/<project>.intent.yaml` (schema is frontier F1). The anchor is **composite
and weighted toward maintainer voice**:

- For **vsdd-factory**: `CLAUDE.md` canonical principle (production-grade default,
  no MVP-defer) + `.factory/STATE.md` cycle goals + `docs/` + git history +
  comments/decisions by DrBothen and Zious11 + a future `SOUL.md`.
- Generically: declared goals, non-goals, scope boundaries, and an alignment
  rubric — kept minimal so it doesn't rot (the GORE lesson).

Maintainer engagement is reconstructed from role-filtered behavioural events (who
responded / labelled / closed / referenced / merged), with reactions treated as
"demand to reconcile, never equate." This is the signal that separates
*on-roadmap* from *led-by-the-backlog*.

## 7. Autonomy & guardrails

Per-stage automation (Parasuraman-Sheridan-Wickens [@pswlevels]):

| Stage | Autonomy | Rationale |
|---|---|---|
| acquire (poll, fetch) | high (auto) | reversible, read-only |
| analyze (validate, classify, score) | high (auto) | read-only; emits findings |
| decide (priority, fast-track) | medium | reversible labels auto; scope/resolution proposed |
| act (label) | auto for bounded/allow-listed/reversible | mutual-exclusivity enforced |
| act (public comment) | **propose / high-bar auto** | curl AI-slop rule; volume-capped; soft tone |
| act (close / resolve) | **never autonomous** | irreversible; escalate (no stale-bot) |

Confidence is surfaced as frequency ("right in ~8 of 10 similar cases"), never
fluent certainty [@confcalib]; evidence is cheap-to-verify links, not prose
[@vasconcelos]; every action is reversible with an audit trail and named-supervisor
attribution [@euaiact; @meaningfulcontrol]. Escalation precision beats recall
(alarm-fatigue) — every human ping clears a high bar.

## 8. Phasing (gradual elaboration)

| Phase | Trigger | Agents | State | Packaging |
|---|---|---|---|---|
| **0 now** | a Claude Code skill (`skills/beadle-triage`) | one session, sub-dispatched | JSONL + dashboard issue | skill files |
| **1** | scheduled `gh-aw` workflow run *as arcavenai* | orchestrator + classifier | embedded store | a sideshow-pack (`pack.yaml`) |
| **2** | a marvel-orchestrated team | supervisor + N classifiers + investigators + fresh-context verifier | **Dolt** (versioned SQL) | pack runs on marvel ("multiclaude v2") |

Build on the hardened `gh-aw` path, not the retiring GitHub Models; run as a
GitHub App / bot identity for a clean audit trail and higher rate limits; throttle
below secondary rate limits [@ghaw; @ghacceptableuse].

## 9. What beadle is NOT

- Not a stale-bot. It never auto-closes on inactivity.
- Not a label-slapper. Every classification carries cited rationale.
- Not an authorizer. Like envoy, it relays and proposes; humans/supervisor decide
  and authorize work.
- Not a metric-optimizer. It reports counts paired with outcomes; it never chases
  close-rate.
- Not a replacement for `arcavenai-issue-review` — it wraps and operationalizes
  that discipline at scale.

## 10. Open questions

Tracked in `charter.md` Frontier and `_kos/nodes/frontier/`: the intent-manifest
schema (F1), the Dolt vs DoltLab/DoltHub state engine (F2), the marvel team shape
(F3), the sideshow-pack contents (F4), the checkbox control surface (F5), and
intent-alignment-as-held-out-evaluation (F6).
