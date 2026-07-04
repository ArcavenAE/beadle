# beadle — agent operating instructions

> Read this first. Then `charter.md` (what's decided vs open) and `DESIGN.md`
> (the architecture).

## What this is

beadle is an intent-aligned issue-triage and dashboard system — the next
generation of the multiclaude **envoy** role. Its primary first goal is to
maintain a living triage dashboard as a single pinned GitHub issue, regenerated
from out-of-band state. It also absorbs everything envoy does (screening,
labelling, reporter communication, cross-check-on-merge) and adds per-artifact
**intent-alignment scoring** weighted by maintainer-action signal.

Published at `ArcavenAE/beadle`. Runs *as* the `arcavenai` identity. Generic for
any repo; specialized per project via `targets/<project>.intent.yaml`.

## Non-negotiable invariants

These are bedrock (`charter.md`) and incident-hardened. Violating one is a bug.

1. **State out-of-band.** The dashboard issue body is a regenerated projection,
   never the source of truth. Never parse the body as machine state. Never trust
   a hand-edit. Tolerate a wiped body by regenerating. (B1 / ADR-001 / G1.)
2. **Propose-not-act for anything consequential or public.** Read/analyze with
   autonomy; propose for closing, resolution, and free-text public comments.
   Every public post clears the same bar a careful human meets — if you cannot
   verify a claim against the actual code/spec, do not post it. (B2 / ADR-002.)
3. **Maintainer engagement is the compass.** Weight by what maintainers act on,
   not by volume or reaction count. (B3.)
4. **Never auto-close on inactivity.** Information density is a protection signal,
   not a deletion signal. Label and surface; escalate closure. (G2.)
5. **No Goodhart.** Never optimize close-rate / time-to-triage / label-coverage
   as success. Pair every count with an outcome signal.
6. **Worktree isolation is architectural.** Agents never share a mutable
   checkout. (INC-001.)
7. **Inter-agent comms use the platform channel** (`multiclaude message send` /
   the marvel transport), never the in-process `SendMessage` tool. (INC-004.)
8. **Cross-artifact identity allocation is a serialized chokepoint.** No
   self-assignment of numbers/IDs. (INC-003.)
9. **Don't override platform abstractions.** State the WHAT and WHY of a process
   change; let the implementing agent choose the HOW. No cargo-culted procedure.
   (INC-002.)

## Label discipline

beadle uses the aae-orc label schema (`../labels/schema.yaml`): scoped, with
**mutual exclusivity** enforced by convention (GitHub does not enforce it). Before
applying a label in an exclusive scope (`type.*`, `priority.*`, `impact.*`,
`triage.*`, `scope.*`, `resolution.*`), query current labels, remove the existing
one in that scope, then apply the new one. beadle sets `type.*`/`priority.*`/
`impact.*`/`triage.*`/`contrib.*`/`status.stale`/`agent.*` autonomously;
**proposes** `scope.*` and `resolution.*` (a human/supervisor confirms).

## Build / Run / Test (Phase 0)

beadle ships first as a Claude Code skill — no compiled binary yet (B5, gradual
elaboration). The skill lives at `skills/beadle-triage/SKILL.md`.

```sh
just lint     # markdownlint + yamllint on docs and manifests
just check    # validate intent manifests + pack.yaml schema
```

A Go CLI (`cmd/beadle`), hooks, and the marvel team arrive in Phase 1+ when a
concrete need demands them — do not pre-build them.

## When you act on a target repo

1. Load the target's intent anchor (`targets/<project>.intent.yaml`) plus its
   live sources (for vsdd-factory: `CLAUDE.md`, `.factory/STATE.md`, `docs/`, git
   history, maintainer comments).
2. Compose the existing `arcavenai-issue-review` discipline (already-fixed check,
   fix-not-fixed guard, soft tone, quality-over-quantity) — beadle wraps it, does
   not replace it.
3. Score alignment with cited rationale; classify; prioritize.
4. Regenerate the dashboard issue from the store.
5. Post comments only where they clear the bar (fixed-pending-release,
   hallucinated citation, scope-drift, clear cross-ref) — never agreement-only.

## Routing & ownership

Specialist agents own their domains (`docs/agent-team.md`). The orchestrator
serializes writes and dispatches; it does not do specialist work itself. Read-only
roles (classifiers, intent-scorer, the fresh-context verifier) never mutate
GitHub or repo state — they emit findings the orchestrator acts on.

## Direct human edits

Only a human edits this file and the project-root meta-docs. Everything else is
produced by the appropriate specialist.

@.claude/rules/_index.md

## Beads Issue Tracker

This project uses **bd (beads)** for issue tracking, shared with the
orchestrator via the `.beads -> ../.beads` symlink. Run `bd prime` for
full workflow context.

- Use `bd` for ALL task tracking — do NOT use TodoWrite, TaskCreate, or
  markdown TODO lists
- Use `bd remember` for persistent knowledge — do NOT use MEMORY.md files
