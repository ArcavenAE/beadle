# Create / refresh the beadle dashboard — run AS arcavenai

beadle discovers and **rewrites its own** dashboard issue every run (discovery by
exact title; full body rewrite; state out-of-band). An issue's author cannot be
reassigned — so the dashboard must be **created and maintained by the same identity
that will keep editing it: `arcavenai`**, the managing bot. Posting it as any other
identity (e.g. `arcaven`, the human review account) means that identity, not the
bot, owns the post and beadle cannot manage it. This file is the launcher that
enforces that.

## How `arcavenai` is the active identity on the run system

On the system where you run this, the GitHub identity `gh` uses must be arcavenai.
Pick one:

- **Dedicated account login (simplest):** `gh auth login` as the `arcavenai` account
  (or `gh auth switch --user arcavenai` if already added). Verify:
  `gh api user --jq .login` → `arcavenai`.
- **GitHub App installation token (Phase-1 target, recommended):** export an
  installation token for the `arcavenai` app, e.g.
  `GH_TOKEN=$(gh auth token --app arcavenai ...)` or via
  `actions/create-github-app-token`. The author then shows as `arcavenai[bot]`.
  Note: `gh api user` won't return `arcavenai` for an app token — in that case set
  `BEADLE_EXPECTED_IDENTITY=arcavenai[bot]` and the guard below checks the issue's
  author after creation instead.

## The prompt

Run from a checkout of `ArcavenAE/beadle` with **cwd = the beadle repo root**.
Paste into Claude Code (or `claude -p "$(cat prompts/create-dashboard.md | sed -n '/^BEGIN PROMPT/,/^END PROMPT/p')"` — copy the block between the markers).

```
BEGIN PROMPT
You are running beadle (ArcavenAE/beadle) to create and maintain the triage
dashboard for drbothen/vsdd-factory, AS the arcavenai identity. cwd is the beadle
repo root.

STEP 0 — HARD IDENTITY GUARD (do this first; abort on mismatch, post nothing):
  - Run: gh api user --jq .login
  - The active identity MUST be `arcavenai` (or, for a GitHub App token,
    `arcavenai[bot]` — accept either).
  - If it is anything else (e.g. `arcaven`), STOP immediately and report:
      "Aborting: active gh identity is <X>, expected arcavenai. The dashboard must
       be owned by arcavenai so beadle can rewrite its own post. Switch identity
       (gh auth switch --user arcavenai, or export the arcavenai app token) and
       re-run." 
    Do NOT create or edit any issue under another identity.

STEP 1 — Load target + invariants:
  - Read targets/vsdd-factory.intent.yaml, CLAUDE.md, charter.md, and
    docs/dashboard-schema.md. Honor every bedrock invariant (state out-of-band;
    propose-not-act; never auto-close; no Goodhart; cold-start gating ADR-005).

STEP 2 — Discover the dashboard issue (sentinel-first, title-second). The body
  `<!-- beadle-state -->` sentinel is the machine-stable primary key; the exact
  title is the secondary/fallback key (it can be hand-edited; the sentinel can't):
    TITLE='📋 beadle — Triage Dashboard'
    # candidate set = union of both, by arcavenai, open:
    gh issue list --repo drbothen/vsdd-factory --state open --author arcavenai \
      --search "beadle-state in:body" --json number,author,title,body
    gh issue list --repo drbothen/vsdd-factory --state open \
      --search "\"$TITLE\" in:title" --json number,author,title
    # merge by issue number; filter to author arcavenai.
  - EXACTLY ONE candidate authored by arcavenai:
      → rewrite its body in place: gh issue edit <n> --repo drbothen/vsdd-factory
        --body-file <generated>. Never open a second. Preserve any human-toggled
        checkbox selections you act on, then reset them.
  - MORE THAN ONE candidate authored by arcavenai:
      → STOP and report ALL of them (numbers + titles). A duplicate dashboard
        already exists. Do NOT pick one silently and do NOT create another — ask a
        human to consolidate (close the extras) first.
  - One candidate authored by someone else:
      → STOP and report. Do NOT edit another author's issue and do NOT create a
        duplicate title. Ask a human to close/hand it off first.
  - NONE:
      → re-run the discovery query one more time IMMEDIATELY before creating (a
        concurrent run may have just created it — this narrows, but does not fully
        close, the create race). If it now exists, branch above.
      → otherwise create it: gh issue create --repo drbothen/vsdd-factory
        --title "$TITLE" --body-file <generated>
      → pin it: get the node id (gh issue view <n> --json id) and call the GraphQL
        pinIssue mutation. If it returns FORBIDDEN, note that arcavenai lacks
        triage permission and a maintainer must pin — then continue (non-fatal).

STEP 3 — Generate the body per docs/dashboard-schema.md:
  - Run the beadle-triage engines (skills/beadle-triage/SKILL.md): enumerate open
    issues, validate (already-fixed / fix-not-fixed / citation-exists), classify,
    score-intent against the manifest rubric, detect clusters.
  - Compute the WARM-UP state. Until >=1 completed maintainer triage cycle exists
    (no maintainer comments/closes on the bot's issues yet → cold start), the
    direction verdict is COLD START / BASELINE — NEVER DRIFTING. Present counts as
    a baseline, mark rate/drift as "not yet measurable," and surface high-leverage
    items + cluster candidates to make the maintainers' first pass cheap.
  - Embed only a digest in the <!-- beadle-state ... --> sentinel; the body is a
    projection. Bot-disclose authorship in the footer.

STEP 4 — Comments (optional, high bar): post per-issue comments only where they
  clear the bar (compose the arcavenai-issue-review discipline: already-fixed,
  fix-not-fixed, soft tone, quality-over-quantity). Propose-not-act for anything
  consequential; never auto-close.

REPORT: the dashboard issue URL, created-vs-rewritten, the direction verdict, and
any action (e.g. pin) that needs a maintainer.
END PROMPT
```

## Notes

- A worked reference of the rendered body is in `docs/example-dashboard.md` (the
  cold-start baseline). The first arcavenai run should produce the same shape.
- The previously mis-authored dashboard (drbothen/vsdd-factory#311, posted by
  `arcaven`) has been **closed** so the title is free for arcavenai to create its
  own. If you see an open same-title issue authored by anyone other than
  arcavenai, the guard in STEP 2 will stop rather than fight over it.
- Phase 1 replaces this manual launcher with a scheduled `gh-aw` workflow running
  as the arcavenai GitHub App on a cron — same identity, same idempotent rewrite.
