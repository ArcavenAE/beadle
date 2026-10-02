# Label proposals from beadle's analysis: what its classification could do for GitHub labels

**Status:** idea (pre-hypothesis, no commitment)
**Born:** 2026-10-02, operator ask, relayed by director
**Feeds:** elem-defect-classification-superset, elem-propose-not-act, elem-maintainer-compass, the Label discipline section of CLAUDE.md

## The operator's words

> "kos idea beadle how might beadle's analysis and issue assessment improve or benefit gh labels?"

## Director's reading (not findings)

Everything in this section is director's reading of the ask, relayed as
context. None of it was measured in beadle.

**The evidence it cites.** A hand review of 136 items from the last two days
in another org's repository, under that org's label scheme, found:

- 3 PRs with no labels, and 13 issues with no priority;
- 3 misfits: a wrong type, a spec-domain label on a feature, and a fan-out
  area label added by a fleet seat;
- an agent-origin label applied one way in one batch and another way in the
  next;
- most labels applied by the item's own author.

The cost was reading every body and timeline by hand.

**The angles it suggests.**

- **Proposals.** beadle proposes type, priority and area labels from the
  analysis it already does, as confirm-or-override under ADR-007, never
  applied on its own.
- **Drift.** The content and the label disagree; a scoped label is missing;
  or a pick-one dimension carries two values.
- **Labeler provenance.** Who applied each label, read from the timeline,
  including origin labels an author applied to their own item.
- **Per-org vocabulary.** The label scheme is an input, not hard-coded.

## What beadle already does (checked 2026-10-02)

- beadle re-classifies every artifact on its own axes and never trusts the
  body's self-label (elem-defect-classification-superset). Most of a label
  proposal is that classification, rendered in a target's vocabulary.
- CLAUDE.md's Label discipline section: beadle uses the aae-orc label schema
  (`../labels/schema.yaml`), enforces mutual exclusivity by convention, sets
  `type.*`, `priority.*`, `impact.*`, `triage.*`, `contrib.*`, `status.stale`
  and `agent.*` **autonomously**, and proposes `scope.*` and `resolution.*`.
  DESIGN.md section 7 says the same ("reversible labels auto").
- The compass already counts a maintainer applying a label as engagement
  (elem-maintainer-compass), so labeler identity is already a signal beadle
  reads.

## Where the reading and beadle disagree

The "never applied on its own" angle conflicts with beadle's bedrock, which
lets beadle apply bounded, reversible, allow-listed labels autonomously on
the repositories it targets (elem-propose-not-act). One way to reconcile the
two: autonomy holds only where the scheme is beadle's own and the target has
an intent manifest, and on another org's repository, under its own scheme,
beadle only proposes. That is a reading, not a ruling.

## What would be new

1. **The scheme as an input.** Today the schema is one path. Another org's
   repository has another vocabulary and other exclusive scopes. A per-target
   scheme reference (beside the intent manifest) would let the same
   classification map onto either.
2. **Drift as a report, not a fix.** For each item, the gap between beadle's
   classification and the labels present: missing in a required scope, two
   values in an exclusive scope, or a label the content contradicts. Reported
   with the cited rationale beadle already writes, never a bare label.
3. **Labeler provenance.** From `labeled` timeline events: who applied each
   label, and whether the author applied it to their own item. A self-applied
   origin or priority label is a claim, not a triage outcome. This also shows
   when two batches of the same origin were labelled differently.

## Diagnostic, not a gate

Label coverage is on beadle's own No Goodhart list (CLAUDE.md principle 5).
Drift counts and coverage inform a maintainer; they never block a merge, and
they never become a target. Each count is paired with an outcome, for
example how many proposed labels a maintainer kept.

## Open questions

- Does beadle's classification map onto label dimensions directly, or does
  each scheme need its own mapping (and who writes it)?
- Who owns label policy: the operator's gh-labels skill, which holds the
  vocabularies, or beadle, which applies them? A split might be: the skill
  owns the vocabulary, and beadle owns the mapping and the drift report.
- Does autonomous labeling stay limited to beadle's own targets, with
  proposals only on any repository under another org's scheme?
- How is provenance read cheaply? Timeline events per item cost one call
  each, which is the hand review's cost moved to an API.
- What would show this is worth building: proposals kept by maintainers at a
  useful rate, or drift found that a hand review missed?
