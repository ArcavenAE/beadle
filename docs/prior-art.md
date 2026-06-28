# Prior art & the white space

A survey of automated / AI-assisted issue triage, backlog management, and
bot-maintained dashboards. Every system falls on one axis — *how much it acts, how
reversibly, on how cheap a signal.* The cell beadle occupies is empty.

## The landscape

| System | What it does | Interaction model | Core failure mode |
|---|---|---|---|
| **Triage Party** (Google) | view-only triage dashboard; expressive rule/collection queries | pure view — never writes | config rots; read-only ceiling |
| **Dosu** | AI label / dedup / answer / stale | graduated dial: Mention → Auto-Draft → Auto-Reply | confident noise pollutes the tracker at scale (LangChain #25153) |
| **Copilot coding agent** | issue → draft PR | human-triggered, always a draft, separation-of-duties | review-burden shift |
| **GitHub native triage AI** | suggest labels/assignees; semantic search | suggest-and-confirm (no always-on auto-labeler) | summaries degrade on long threads |
| **Sweep** (YC S23) | issue → PR | discontinued / pivoted to IDE plugin | "a 500-line PR makes you a reviewer for code you didn't write" |
| **actions/stale** | inactivity → label → close | auto-act on cron | the backlash: inactivity ≠ irrelevance; destroys rich reports |
| **probot/no-response** | unanswered question → close | human precondition + reopens on reply | narrow; the *defensible* auto-close |
| **Renovate Dependency Dashboard** | bot-owned issue; checkbox = control | checkbox-as-control + re-render loop | **crashes if a human edits the body** (#19563) |
| **Mergify** | rules engine + merge queue | off-GitHub dashboard + per-PR checks | durable state lives off-platform; not glanceable |
| **Sentry Seer** | root-cause → solution → PR | propose with operator-set stop points | hallucination + run-to-run non-determinism (vendor-admitted) |
| **Linear Agents** | triage intelligence; agents-as-users | delegation ≠ reassignment (human stays accountable) | preview APIs; thin rule automation |
| **Rust / K8s tracking issues** | human umbrella/checklist issue | manual curation; code↔issue linkage | the checklist **rots** |

## Transferable lessons (stolen into beadle)

1. **Triage Party's judgment-encoding rule vocabulary** (conversation-direction:
   who owes whom a reply; response-latency) and its "every rule has a documented
   resolution / the page is empty when triage is done" definition-of-done.
2. **Dosu's graduated, per-action autonomy dial** with **bounded write-scope**
   (labels/reactions/comments only) — but default reader-facing free text to a
   review path, with volume caps, after the LangChain "dosubot pollutes the
   tracker" backlash.
3. **The stale-bot anti-pattern**: never let an irreversible action fire on a
   single cheap signal; **information density is a *protection* signal**; prefer
   additive/reversible actions; require a human precondition for anything
   destructive (no-response, not stale). The Nix model: label, never close.
4. **Renovate's dashboard mechanics** (title-discovered single issue, full body
   rewrite, checkbox-as-control) — but with **state out-of-band** (the universal
   state-in-Markdown failure; derive checkboxes, don't trust hand-edits).
5. **BugBug's confidence gating** (Mozilla's only long-running production triage
   ML acts only on the >60%-confidence subset) — ship the confident subset,
   recommend elsewhere.
6. **Greptile's learned noise suppression** (block a comment too similar to
   downvoted ones) and **Linear's delegation-not-reassignment** (a named human
   stays accountable) — so beadle doesn't become the next "dosubot."
7. Build on the hardened **`gh-aw`** path, not the retiring GitHub Models; run as
   a bot identity; prompt-injection-harden untrusted issue/PR text.

## Research-evidence ceilings (calibration)

- ~**60% precision ceiling** for auto-assignment has held ~20 years (Anvik et al.
  ICSE 2006; Bhattacharya ~86% on 856k reports; both framed as recommenders).
- **~1/3 of issue labels are misclassified at the source** (Herzig-Just-Zeller
  ICSE 2013) — never trust existing labels as ground truth.
- **Duplicate detection** tops out ~68–74% recall@k, and tuned classical
  retrieval (REP) beats deep learning on most projects — auto-merge is unsafe;
  surface ranked candidates for human confirmation.
- **Nominal features (reporter, component) can beat NLP embeddings** for
  assignment (Li et al. 2024) — sophistication isn't the lever; data scale +
  realistic evaluation are.

## The white space (confirmed empty after genuine search)

> **No shipping tool semantically scores an individual issue / PR / commit against
> a declared, versioned project *intent*, emits a graded verdict with cited
> rationale, weighted by reconstructed *maintainer-action* signal, with a
> bot-maintained dashboard whose state lives out-of-band.**

Adjacent cells are occupied but answer the wrong question:

- **PM tools** (airfocus "strategic drift", Umaku "vision misalignment") name the
  concept but disclose no mechanism, and score at roadmap-initiative granularity,
  not per-artifact.
- **Engineering-intelligence** (Jellyfish, Swarmia "Investment Balance") measures
  *time-allocation across buckets* — portfolio accounting, not work-vs-mission.
- **Spec-conformance** (drift skills, SmartBear) checks code-vs-spec — fidelity,
  not work-vs-intent.
- **Goal-oriented RE** (GORE / KAOS / i\*) has the right graded partial-satisfaction
  *formalism* but is manual modelling nobody sustains; traceability-ML automates
  ingestion but answers link-existence, not alignment quality.

Two adjacent confirmed gaps beadle also fills: **no tool cleanly separates
reaction-popularity from maintainer-acted-upon signal** (the weighting input), and
**"drift" has no vocabulary for the human-work-vs-intent meaning** (it resolves to
infra/ML drift). beadle needs all three at once — per-artifact granularity,
semantic scoring against a machine-readable intent, continuous automation — which
is exactly the empty intersection.

Full source URLs in `../references.bib`.
