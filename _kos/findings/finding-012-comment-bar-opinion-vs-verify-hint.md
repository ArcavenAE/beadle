# finding-012 — the comment-bar's opinion / verify-hint distinction

Date: 2026-07-01
Probe: item D from the Phase-4 retrospective — extend the comment-bar rubric to
explicitly distinguish "opinion" (which fails) from "disambiguation /
verify-hint" (which can pass). Ground the rule in real drbothen/vsdd-factory
issues (#370, #380, #350, #381).
Nodes touched: `elem-propose-not-act` (bedrock, updated with the four lenses +
sharpening tests); `skills/beadle-triage/SKILL.md` §9 (Comment).

## What the four lenses now say

A comment posted by beadle on an issue in the target repo passes the bar iff
it fits one of four lenses (`elem-propose-not-act`, §9 of the skill):

1. **fixed-pending-release** — cite the merge commit + the tests that
   demonstrate the fix. Verifiable against the repo in one click.
2. **hallucinated-citation / fix-not-fixed** — the issue cites a file, line,
   function, or spec section that does not exist, or was edited to something
   that no longer supports the claim. Cite the exact drift.
3. **scope-drift** — the issue proposes work outside the target's declared
   intent. Cite the intent anchor.
4. **clear sibling cross-ref** — a sibling issue is measurably about the
   same defect or a strictly-adjacent one. Cite both numbers, name the
   delta in one sentence.

## The item-D extension — opinion vs verify-hint

The rubric before this finding was the four lenses plus "never agreement-only,
soft never hard, disclose bot authorship." That was correct but under-defined:
it did not close the door on well-intentioned *opinion* comments that a careful
reviewer might mistake for adding value.

Item D closes that door with a *behaviour-trigger sharpening test*, same shape
as `~/.claude/CLAUDE.md`'s force-push abort signal and orc's
`.claude/rules/tooling-friction.md`. Every draft comment is audited against
three tests before it is posted:

1. **Verifiable-in-60-seconds** — would a maintainer be able to check the
   claim against the code, spec, or issue graph in under a minute? If no,
   the comment is opinion. Do not post.
2. **Names-a-specific-artifact** — does the comment cite `file:line`, a
   commit sha, an issue number, an intent anchor? If no, the comment is
   opinion. Do not post.
3. **Adds-a-pointer-the-maintainer-did-not-have** — even if the maintainer
   read the issue, does the comment add a piece of orientation they did
   not already have? "This is important" always fails. A verify-hint that
   names *where to look* passes.

The rule collapses to: *disambiguation names an artifact; opinion names a
feeling.* If the draft names a feeling, throw it away.

## Grounding on four vsdd-factory cases

The retrospective specifically flagged #370, #380 vs #350, #381 as the
comparison set. Applying the extension:

- **#370** — "CI build-config verification job emits static PASS instead of
  runtime-computed scan count." The body carries the fix shape (compute and
  print scan count, warn on zero). *Opinion* would be "well-scoped, ship
  it" — names a feeling. *Verify-hint* would be pointing at the exact
  workflow file and line where the static echo lives, so the maintainer can
  eyeball the fix shape in one click — names an artifact. Post the second,
  not the first. In practice beadle **should not comment on #370** because
  the fix shape is already spelled out in the body; adding a pointer the
  maintainer did not already have is not possible here without inventing.
- **#380** and **#350** — the body of #380 already distinguishes itself
  from #350 in a paragraph: "This is distinct from #350. #350 is about the
  *harness classifier* blocking a relayed signed commit. This issue is
  about the *subagent's own decision logic*." That is the delta already in
  the graph. A beadle sibling cross-ref that adds "these are companions;
  address the layer both share (relayed-authorization semantics) before
  either individually" would clear lens 4 — it names a specific delta the
  maintainer might miss when triaging one without the other. A comment
  that just says "these are real problems" is opinion.
- **#381** — "reference oracle duplicates production mapping." The author
  (arcavenai) added a follow-up comment on 2026-07-01 broadening scope to
  "test doc-comment overclaim." That is a *self-annotation* pattern —
  beadle records it as a sibling signal (arcavenai is talking to itself
  and broadening scope on an issue it already filed), but beadle does not
  imitate the broadening in its own voice. The right beadle behaviour is
  to file/surface the broadened pattern as a *new* issue (or a follow-up
  in the dashboard), not to post an opinion on #381 saying "yes, and
  also …".

## The self-annotation pattern (sibling class)

An emergent pattern the store now surfaces: `arcavenai` commenting on
`arcavenai`-authored issues. Run 10 of the vsdd-factory store shows
`arcavenai` = 81 comments, `arcaven` = 76 comments — combined ≥ 157 = all
comments in the current window. This is one measured contributor talking
to another measured contributor (or itself), with zero maintainer
engagement in that thread. beadle must count this as *contribution
volume* per `elem-maintainer-compass`, never as maintainer-engagement.
The comment-bar rubric now says: self-annotations are a sibling signal
we *notice*; they are never a shape we *imitate*.

## What shipped

- `skills/beadle-triage/SKILL.md` §9 — the four lenses, the opinion /
  verify-hint distinction, the three sharpening tests, four grounded
  examples.
- `_kos/nodes/bedrock/elem-propose-not-act.yaml` — extended with the
  four-lens rubric and the three-test audit.
- This finding.

## What stays open

- Comment authorship is a candidate for a per-artifact store record type
  (`comment_draft` / `comment_posted`) so that the audit is *itself* an
  auditable object. Frontier: whether draft comments and their audit
  outcomes should be first-class in the JSONL store. Deferred until the
  first live posting cycle produces enough drafts to make the audit
  worth persisting.
- The self-annotation pattern (`arcavenai` self-thread) may deserve its
  own axis in `elem-defect-classification-superset` — a `filer-loop`
  signal orthogonal to defect-nature. Deferred until a second target
  reproduces the pattern (vsdd-factory is our only current target).
