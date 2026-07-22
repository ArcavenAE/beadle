# 📋 beadle — Triage Dashboard · drbothen/vsdd-factory

<!-- beadle-state:v1
{"schema":1,"last_run":"2026-07-22","watermark":753,"store":"jsonl","intent_version":"vsdd-factory@0.3","warmup":"cold-start","run":16,"digest":"run16-0-new-13-merges-9-closes-third-p0b-close-623-ramp-prediction-FAILED-a4-7run-drifting-stands-2026-07-22","counts":{"open":479,"arcavenai_open":427,"maintainer_engaged":58,"arcavenai_closed_alltime":15},"attn":{"governance":[510],"direction":[410,671],"evidence_brief":[463,686,710]},"axis_model":"finding-005 superset + finding-009: report-type, defect-nature, reproducibility, triage-state, leverage, alignment, provenance, integrity(safety), operational-impact(liveness)","p0_data_loss":[342,358,365,523,588,635],"p0_integrity":[313,314,330,331,332,333,337,339,341,348,355,356,370,372,373,374,379,381,399,404,412,421,425,427,428,430,433,434,437,440,441,448,452,465,468,470,474,475,477,479,481,483,485,486,488,491,492,494,496,517,535,537,538,544,546,547,576,589,592,599,600,614,623,637,638,647,650,663,664,672,673,680,685,692,704,710,724,747,750],"new_this_run":[],"prior_run_burst":[724,732,733,734,741,746,747,748,749,750,751,752,753],"operational_impact":{"halt":[321,326,343,346,347,349,357,380,386,411,414,451,457,458,461,469,472,473,474,516,543,555,601,604,626,637,651,658,681,684,696,702,724,732],"panic":[541,548,569,647],"data_loss":[342,365,358,523],"degraded_new":[734,741,746]},"keystone":336,"keystone_new":[],"keystone_prior":[406,410,413,415,416,419,426,432,462,463,470,487,488,507,513,576,671],"quick_wins_new":[732,734,741],"quick_wins_prior":[436,444,447,450,453,466,469,471,472,476,478,482,495,498,499,501,633,636,641,643,645,653,654,655,656,660,661,666,667,677,678,682,684,690,703,705,707,709,712],"clusters":{"silent-data-loss":[342,358,365,412,479,523,588,635,645],"source-of-truth-integrity":[399,404,412,421,425,427,428,430,433,434,437,440,441,448,452,465,468,470,474,475,477,479,481,483,485,486,488,491,492,494,496,517,535,537,538,544,546,547,589,600,614,623,637,638,647,650,663,664,672,673,680,685,692,704,709,724,747,750],"false-green-convergence":[305,309,310,322,327,330,331,332,333,337,339,348,353,355,356,360,364,370,373,381,390,391,393,397,398,399,421,425,429,433,434,440,441,442,448,452,462,465,467,468,470,474,475,477,479,480,481,484,485,486,488,490,492,494,496,497,500,513,533,535,543,553,563,576,591,592,599,614,618,620,627,634,643,664,672,673,674,675,676,677,685,686,687,690,694,698,699,700,704,705,710,733,747,748,752],"operational-halt":[321,326,343,346,347,349,357,380,386,411,414,451,457,458,461,469,472,473,474,503,539,541,548,555,569,604,626,637,642,681,696,724,732],"platform-envelope-mismatch":[410,411,412,413,414,415,416,417,458,461,473,474,516,601,614,625,649,651,656],"test-writer-gaps":[329,335,354,359,363,364,373,381,383,393,394,398,402,418,419,420,422,423,424,426,436,442,460,466,471,475,477,479,480,481,485,490,494,496,498,499,500,502,520,556,571,575,598,607,609,633,653,665,679,682,684,697,699,700,701,702,713],"spec-propagation-drift":[361,362,367,369,376,382,387,388,390,391,392,395,396,400,401,403,404,406,418,419,422,427,428,429,430,432,435,443,444,445,446,447,449,450,453,459,471,476,478,482,484,488,489,491,493,519,521,522,537,550,552,554,559,561,564,565,573,574,580,581,582,594,600,615,622,638,647,653,656,660,668,671,680,681,693,695,708,709,711,749,751],"adversary-sweep-enumeration":[504,505,506,507,511,519,520,534,548,563,564,573,574,606,615,633,652,686,733,748,752,753],"verification-methodology":[509,512,517,518,522,525,533,534,535,536,539,540,542,543,544,545,546,549,550,553,555,556,557,559,560,562,566,567,585,586,587,590,593,595,596,599,601,602,603,605,607,608,616,617,618,620,621,627,633,634,641,643,645,648,649,652,654,658,660,661,662,663,664,665,667,668,669,671,675,676,677,678,679,682,687,690,693,694,695,697,698,699,700,701,702,703,704,705,710,713,724,746,748,749,751,752,753],"artifact-containment":[263,341,515,589,597,630,636,650,734,741],"scratch-isolation":[508,624,701],"contribution-governance":[510,551,651,696],"dispatch-contract":[328,368,386,405,409,448,457,459,463,468,473,482,516,518,521,525,540,545,549,551,558,569,589,619,630,655,658,661,662,666,674,683,696,706,707],"authority-substrate":[372,374,379,426,483,508,605,644],"observability":[317,318,319,320,324,325,415,463,464,495,501,583,628,629,635,644,648,667],"plugin-pack-ci":[349,351,357,469,472,476,492,625,694,698,732],"orchestrator-continuation":[343,347,380,408,409,446,457,486,495,604,635,647,692,707],"branch-protection":[346,348,349,357,408,626],"dispatch-race":[345,350,355,368,451,547,570,592,594,619,703],"worktree-factory-split":[341,342,451,452,483,503,508,523,588,624,631,655],"state-manager-hygiene":[517,538,547,572,629,631,641,692,706,711,712],"variants-gallery-review":[568],"convergence-tuning":[577,578,579],"learning-loop":[584]},"maintainer_actions":{"comments_prior":{"290":"2026-07-19","298":"2026-07-08","305":"2026-07-19","396":"2026-07-19","418":"2026-07-19","424":"2026-07-19","428":"2026-07-08","434":"2026-07-19","443":"2026-07-15","461":"2026-07-19","465":"2026-07-19","472":"2026-07-19","473":"2026-07-19","475":"2026-07-19","494":"2026-07-19","507":"2026-07-15","515":"2026-07-19","521":"2026-07-15","582":"2026-07-19","649":"2026-07-15","651":"2026-07-15","654":"2026-07-19","655":"2026-07-19","682":"2026-07-19","686":"2026-07-19","724":"2026-07-21"},"comments":{},"filings":[635,636,637,638,652,653,654,655,656,671,690,695],"merges":{"524":"2026-07-19","526":"2026-07-19","527":"2026-07-19","528":"2026-07-19","529":"2026-07-19","530":"2026-07-19","531":"2026-07-19","532":"2026-07-19","725":"2026-07-21","715":"2026-07-22","716":"2026-07-22","717":"2026-07-22","718":"2026-07-22","719":"2026-07-22","721":"2026-07-22","722":"2026-07-22","723":"2026-07-22","726":"2026-07-22","730":"2026-07-22","731":"2026-07-22","736":"2026-07-22","739":"2026-07-22"},"closes":{"418":"2026-07-19","465":"2026-07-19","472":"2026-07-19","724":"2026-07-21","229":"2026-07-22","243":"2026-07-22","244":"2026-07-22","296":"2026-07-22","300":"2026-07-22","566":"2026-07-22","623":"2026-07-22","658":"2026-07-22","660":"2026-07-22"},"corrections":{"691":"corrects #531 (merged PR) — Go/Node stdin diagnosis"},"close_caveats":{"244":"keyword auto-close via #731 squash (5648164) — math-test criterion residual, comment posted"}}}
beadle-state -->

**Direction verdict: 🔴 DRIFTING (machine and now unqualified) — the pre-registered ramp prediction was evaluated this run and FAILED.**
**The test (aae-orc#65, criteria fixed 2026-07-20, blind to this data): hard acted-on ceiling
past P0b by run-16 — i.e. maintainer action on a P0a/SDL item they did not file — else
"ramping stops explaining the data and the drifting read stands." Result: the ceiling is
P0b for the third consecutive window (this window's rung-setter: #623, input-hash upsert,
closed via merged PR #718). No P0a/SDL item was touched; A4 extends to a 7-run
zero-engagement streak on #342/#365/#358. Per the pre-registration, the RAMPING qualifier is
retired — reported plainly, no re-derivation.** The finding is sharpened, not contradicted,
by this being the **biggest engagement window on record**: 13 arcavenai PRs merged and 9
tracked defects closed in one day (#296 #229 #243 #244 #300 #566 #623 #658 #660 — a third
P0b close, plus review comments driving two more rebases). The maintainer is draining the
backlog hard, *and* the drain has a stable selection function whose ceiling sits at P0b:
every acted item arrived with a mergeable fix PR attached; the six SDL items have none.
One close carries a caveat: **#244 was auto-closed by a stale `Closes` keyword in #731's
squash commit** (the PR body had been corrected to `Refs` in review) — its deferred
math-test criterion is residual; comment posted offering reopen-or-refile. **Watermark
unchanged at #753 — the first zero-new-issue window on record** (the measured filer paused
while the maintainer drained).
_Updated 2026-07-22 · watermark #753 · run 16 · intent vsdd-factory@0.3
· finding-005/009 axes + attn facet · vocabulary.json contract (beadle PRs #34/#35) ·
skills-based refresh (binary render retired per run-10 regression)_

> **vsdd-factory is a self-referential factory — engine == product.** Engine process-gaps and
> meta findings are **on-mission**, scored on leverage + provenance, not dismissed as
> self-reference. Run-15's intake held **13/13 `advances`, 13/13 pilot-derived** (second
> consecutive zero-speculative burst). Run-11's integrity-block reading, run-12's sweep-chain
> reading, run-13's checkers-lie reading, and run-14's maintainer-files-into-the-corpus
> reading are unchanged and carried below.

> **THIS RUN — zero new issues; the window was all maintainer turn.** 13 PR merges + 9
> defect closes in one day. The drained set spans the board's lanes: P0b integrity (#623
> input-hash upsert), operational halt (#658 story-ID regex), quick wins (#660 v-prefix,
> #229 corverax placeholder, #296 emit-event doc), dashboards (#243 #244), registry (#300),
> and BC-title scoping (#566). Run-15's 13-issue intake (two P0b: #747 #750; themes:
> narrated-verification, anchor-resolution, downstream-pilot axis proposals, repo hygiene)
> is folded to its index below.

## Baseline (counts — a starting line, first rates unlocked)

| Metric | Value | Outcome pairing |
|---|---|---|
| Open issues | 479 | arcavenai 427 · Zious11 27 · arcaven 16 · drbothen 8 · slabgorb 1 |
| Filer concentration | arcavenai ~89% | the measured filer paused this window (0 new); the maintainer drained (−9) |
| New this run | **0** (watermark holds at #753) | first zero-new-issue window on record |
| Maintainer engagement | **58 actions (+22 this run — the biggest window on record)** | 13 arcavenai PR merges + 9 tracked-defect closes in one day (2026-07-22) |
| Closes against tracked defects | **13** (+9: #229 #243 #244 #296 #300 #566 #623 #658 #660) | every close paired with a merged fix PR; #244 alone was a keyword auto-close with residual scope (caveat below). Three more fixes merged with `Refs` — #637 #690 #473 stay open pending reporter confirmation |
| filed-vs-acted gap | 427 open : 58 acted | trend: 12.9:1 (run-14) → 12.1:1 (run-15) → **7.4:1** — the sharpest narrowing yet, all from the acted side |
| Δ since last run (#753→#753) | **−9 net open** (0 new − 9 closed) | drained set spans P0b, halt, quick-win, dashboard, registry, BC-title lanes |

## 👤 Needs human reading — direct attention (NEW lane · orthogonal to priority)
> A small group whose **special nature benefits from the maintainer reading them directly and
> in full** — the value is in the prose and cannot survive summarization into chips. Orthogonal
> to priority: an item here can be P2 on impact yet first in your reading queue. Ordered by
> reading priority: **reply-needed** (a counterparty is blocked on your answer) →
> **gates-work** (the decision unblocks other open work) → **standing** (context that improves
> everything downstream). Never quick-win-eligible; never folded into cluster rollups.
| What it is | # | attn · order · why direct reading |
|---|---|---|
| **External contribution flow proposal (ArcavenAE)** — first external contributor asks how to route artifact-bearing changes; proposes artifact-content-via-issue backed by a 2000-trial simulation (tables + charts in comments) | **#510** | `attn.governance` · **reply-needed** · consent/relationship decision only maintainers can make; ArcavenAE holds artifact-bearing PRs until your ruling (also P2 · **no response after 17 days** — 22 actions landed this window, none here; PR #524 even references #510 without ruling on it) |
| **Platform-envelope mismatch (tracking)** — the dark-factory requirements profile vs the Claude Code plugin envelope; seven named limits (#411–#417), now generating live gate failures (#458 #461 #473 #474) | **#410** | `attn.direction` · **gates-work** · re-platform / amend-constitution / risk-accept is an architectural fork only you can take; 11 open issues wait on it (also P1-KEYSTONE) |
| **Fleet-mining evidence brief** — 4,602 dispatches across 3 factories, 169 bad-outcome chains hand-classified: 84% of real bad outcomes are NOT model capability; four engine asks derived | **#463** | `attn.evidence-brief` · **standing** · external empirical corpus you cannot re-derive from this repo; weighing adoption of the four asks is a judgment over the full method + numbers (also P1-KEYSTONE) |
<details><summary>🤖 agent channel — attn.* facet notes (debut run)</summary>
```
Facet defined in beadle SKILL.md step 3b (2026-07-06). Initial subtypes grown from these
three exemplars; growth rule = codify a new subtype on its SECOND instance, generic
attn.other until then. Candidates screened this run and NOT tagged: #508 (worktree config
poisoning — serious but a defect, chips carry it), #507 (consolidation terminus — engineering
judgment, not relationship/direction), #464 (evidence volume but derivable from this repo's
own telemetry). The bar: would the maintainer LOSE something decision-relevant if they only
read the chip row? Keep the lane in single digits or the facet stops meaning anything.
```
```
Run-13 screen: 104 new issues, ZERO new attn.* tags. Nearest candidates and why not:
#584 (learning-loop, maintainer-authored — the maintainer wrote it, direct-reading is moot);
#568 (variants-gallery proposal — speculative, chips carry it); #583 (cost ledger — bounded
enhancement). The lane stays at 3 items, all carried, all still unanswered. reply-needed #510
ages to 7 days with maintainer activity landing elsewhere — the first conversion target
remains unchanged.
```
| Factory-graph: derived traceability graph rehydrated from `.factory/` markdown | #671 | 👤 direction · gates-work · **your co-maintainer's keystone proposal** — one ruling reframes the traceability lane; graded P1 KEYSTONE this run |
| Reviewer-calibration divergence: severity vs code-freeze state | #686 | 👤 evidence-brief · standing · third calibration facet, Zious11 field-data already attached — the cross-project evidence lives only here |
| V-model verification-planning gap: N-consecutive-clean counts absence-of-findings | #710 | 👤 evidence-brief · standing · 🛑 also P0b — carries the empirical convergence dataset the repo cannot re-derive |

```
Run-14 screen: 72 new issues, 3 new attn.* tags (lane 3 → 6, at the ~7 bar — tighten next run).
Admitted: #671 (direction — maintainer-authored, but it is a PROPOSAL awaiting the other
maintainer's ruling: direct reading is the decision path, not moot per the run-13
maintainer-authored screen); #686, #710 (evidence-brief — cross-project field data not
re-derivable from this repo). Nearest misses: #687 (citation-coherence class — chips carry
it); #695 (release CHANGELOG enumeration — bounded process ask); #634 (drbothen's own
holdout-gate feature spec — author == reader, moot). External-signal note: an outside
analysis flags #681 as the strongest filing of this batch. B3: outside analysis is demand to
reconcile, never equate — recorded here as provenance-annotated signal; #681's board
placement (P1 operational-halt, dual-cluster) derives from the grader axes + curator read,
which happen to agree with it.
```
```
Run-15 screen: 13 new issues, ZERO new attn.* tags (lane holds at 6 — the run-14 "tighten
next run" note is honored by admitting nothing). Nearest candidates and why not: #747
(convergence-seal bypass — highest-stakes new item, but it is a defect with a cited cheap
remedy; chips + the P0b row carry it); #750 (errata-artifact gap — process defect, chips
carry it); #752/#753 (axis proposals — engineering adoption calls, not judgment-over-prose).
The six arcaven fact-check comments are thread-level signal, recorded in the classification
rationales, not an attn trigger. reply-needed #510 ages to 17 days.
```
```
Run-16 screen: 0 new issues, no candidates to screen — lane holds at 6, all carried. The
notable negative: 22 maintainer actions landed this window and NONE touched this lane.
reply-needed #510 stays unanswered at 17 days while the merge wave passed it by; #410's
11-issue dependency cone and the two evidence briefs (#463 #686 #710) likewise untouched.
The lane is now the cleanest witness of the selection function: items needing a READ +
RULING sit; items arriving with a mergeable diff move.
```
</details>

## Action plan — grouped by INTENT, integrity-first (finding-004 precedence)

Rows lead with **what the issue is**; `#NN` is the agent's fetch reference; trailing chips are
status. `repro` badge feeds the effort estimate; `⏸ halt`/`💥 panic` = operational-impact
(finding-009); `🛑` = source-of-truth integrity (finding-004); `💾` = data-loss. Verdicts cited
against the v0.3 rubric. Board rows verified against live state this run; the nine issues
closed in this window's maintainer wave (#229 #243 #244 #296 #300 #566 #623 #658 #660) are
struck where they appear or noted in their lane — see Maintainer progress for the
close-by-close pairing.


### 🔴 P0a — Silent data-loss (highest severity: irreplaceable state destroyed/stranded with no signal)

> Each is **integrity × data-loss** (or pure SDL): a system-of-record silently diverges from
> reality AND destroys or strands state behind a green check. Sits **above** everything else
> by precedence. **A4 alarm (finding-016) — DRIFTING, machine-computed:** #342/#365/#358 now
> at a **7-run zero-engagement streak**, and this window makes the silence maximally
> load-bearing: **22 maintainer actions landed in one day — merges, closes, a third P0b kill —
> and not one touched this lane.** #523/#588 continue their own clock (4 runs); #635 (the
> maintainer's own SDL filing) is at 3. **This lane is what the ramp prediction needed and did
> not get — the prediction came due this run and FAILED (see Direction Health).** No new SDL
> this run (0 new issues).

| What it is | # | Type · repro · verdict |
|---|---|---|
| Product-branch merge silently `rm`s a `.factory` file the nested worktree was serving | #342 | bug · Bohr · **🛑💾 INTEGRITY + DATA-LOSS · ADVANCES · systemic** |
| Rebase auto-merge silently drops 4 production lines (no conflict markers, clean `--continue`) | #365 | bug · Mandel · **🛑💾 INTEGRITY + DATA-LOSS · ADVANCES · systemic** |
| PR base not locked to trunk → orphan merge: `state=MERGED` while `origin/main` lacks the commit | #358 | bug · Bohr · **🛑💾 INTEGRITY + DATA-LOSS(orphan) · ADVANCES · systemic** |
| Story-worktree `.factory` artifacts silently lost at teardown — worktree removed with unsynced state | **#523** | bug · Bohr · **🛑💾 INTEGRITY + DATA-LOSS · ADVANCES · systemic** (NEW) |
| Factory-side PR strands the shared `.factory` worktree on a chore branch — served state silently wrong for every consumer | **#588** | process-gap · Bohr · **🛑▲ INTEGRITY + SDL(strand) · ADVANCES · systemic** (NEW) |
| CLEAN passes/streak never persisted mid-gate; crash loses position | **#635** | process-gap · Bohr · **🛑💾 DATA-LOSS · ADVANCES · systemic** (NEW) |

### 🔴 P0b — Source-of-truth integrity (gates EVERY functional verdict, incl. convergence)

> A PASS/"converged" verdict computed over a corrupt substrate is **unfalsifiable**
> (finding-004). **The class got its THIRD kill this window: #623 closed via merged arcavenai
> PR #718 — input-hash records now upsert idempotently instead of appending duplicate rows**
> (following #465/PR #532 in run-14 and #724/PR #725 in run-15 — the P0b lane is where fixes
> with PRs attached reliably land). Run-15's two new P0b carry: **#747** — production edits
> merged after the 3-CLEAN convergence seal with no gate re-entry (nothing binds the seal to
> the merged HEAD; the #313 false-certification family at story scope; thread suggests a
> `converged_sha` mirroring BC-5.42.001's `covered_sha`), and **#750** — errata cited as
> normative authority by five sibling artifacts exist only as changelog rows, unresolvable by
> any grep/listing/graph-walk (the anchor-integrity family, except the authority existed as
> an event, not an artifact). Note on #750: its grade is contested between graders
> (P0b/integrity vs P2/none) — held at P0b per the safety-conservative rule, flagged for
> re-grade when new evidence lands.

| What it is | # | Type · repro · verdict |
|---|---|---|
| Post-convergence production edits merge without gate re-entry — seal not bound to merged HEAD | **#747** | process-gap · Bohr · **🛑 INTEGRITY (convergence seal) · ADVANCES · systemic** (NEW) |
| Errata cited as normative authority exist only as changelog rows — unresolvable by construction | **#750** | process-gap · Bohr · **🛑 INTEGRITY (learning) · ADVANCES · systemic** (NEW) |
| Live sprint-state assertions gated every PR on base drift — data + gate both fixed | ~~#724~~ | ci-build · Bohr · **🛑 INTEGRITY (spec_process) · RESOLVED ✓ 2026-07-21** (NEW+CLOSED) |
| $(cat) strips trailing newlines — input-hash false-drift hard-block | **#637** | bug · Bohr · **🛑 INTEGRITY (spec_process) · fix MERGED (PR #715) — open pending reporter verify** |
| Finding-ID scheme not canonical — G-less form collides, enables fakes | **#638** | process-gap · Bohr · **🛑 INTEGRITY (learning) · ADVANCES · systemic** (NEW) |
| Writer crash mid-burst leaves changelog overclaiming unedited body | **#647** | process-gap · Mandel · **🛑 INTEGRITY (spec_process) · ADVANCES · systemic** (NEW) |
| Full-file Write leaks stray </content> tag into committed spec artifact | **#650** | bug · Bohr · **🛑 INTEGRITY (spec_process) · ADVANCES · systemic** (NEW) |
| Identifiers retyped from memory silently corrupt records | **#663** | process-gap · Mandel · **🛑 INTEGRITY (learning) · ADVANCES · systemic** (NEW) |
| Review verdicts accepted without ground-truthing components | **#664** | process-gap · Mandel · **🛑 INTEGRITY (learning) · ADVANCES · systemic** (NEW) |
| Decompose omits terminal input-hash --update; hash wrong from birth | **#672** | process-gap · Bohr · **🛑 INTEGRITY (spec_process) · ADVANCES · systemic** (NEW) |
| Blocking validators' fix-instructions coerce fabricated record text | **#673** | process-gap · Bohr · **🛑 INTEGRITY (spec_process) · ADVANCES · systemic** (NEW) |
| Spec's false API claim steers code toward invariant violation | **#680** | process-gap · Bohr · **🛑 INTEGRITY (spec_process) · ADVANCES · systemic** (NEW) |
| Fix-phase self-attestation: DONE report false, guard test neutered | **#685** | process-gap · Mandel · **🛑 INTEGRITY (learning) · ADVANCES · systemic** (NEW) |
| Interleaved delivery leaves STATE.md next-pointer aimed at already-merged story | **#692** | process-gap · Mandel · **🛑 INTEGRITY (spec_process) · ADVANCES · systemic** (NEW) |
| Pipe to grep masks runner exit code; post-summary crash reads green | **#704** | process-gap · Bohr · **🛑 INTEGRITY (spec_process) · ADVANCES · systemic** (NEW) |
| Convergence counts clean passes without vector-coverage novelty | **#710** | process-gap · Mandel · **🛑 INTEGRITY (spec_process) · ADVANCES · systemic** (NEW) |

<details><summary>🤖 agent channel — P0b prior detail (49 issues, runs 9–12) + run-13 reading</summary>

Prior P0b: **#465 CLOSED with a validated fix (PR #532 — the first P0b close)**; the rest open. Maintainer comments beyond #428 (Zious11 evidence-add
2026-07-08). Full rows in the run-11/12 renders (docs/fixtures in ArcavenAE/beadle) and the
store; fetch `#NN` for bodies (B1): #313 #314 #330 #331 #332 #333 #337 #339 #341 #348 #355 #356 #370 #372 #373 #374 #379 #381 #399 #404 #412 #421 #425 #427 #428 #430 #433 #434 #437 #440 #441 #448 #452 #465 #468 #470 #474 #475 #477 #479 #481 #483 #485 #486 #488 #491 #492 #494 #496.

```
Run-13 reading (14 new):
- The gate layer is now the dominant corruption target: #599 (agent-authored lint gates ignore
  their args and under-scan — vacuous green), #614 (GNU BRE inverse detectors false-GREEN on
  macOS), #600 (index sweep lands partial, report asserts full), #592 (async adversary verdicts
  dropped or bound to superseded HEAD), #517 (Drift Items marked satisfied advance with no
  verification gate). finding-004's "the green check IS the defect" arc: run-11 proved records
  lie, run-12 proved prose survives sweeps, run-13 proves the checkers lie.
- #576 (maintainer-authored) + #513 (run-12 keystone) now bracket the same fix from both sides:
  verdicts must derive mechanically from evidence (findings count / pasted gate output), never
  from narrative. Land these two and the largest false-green subclass loses its mechanism.
- Write-protection sub-family: #589 (frozen artifacts writable — 27 misdirected appends,
  forked v1.97), #537/#538/#544/#546/#547/~~#623~~ (state/attestation/protocol corruption
  paths; #623 CLOSED 2026-07-22 via PR #718 — the third P0b close).
- All 14 HARD-EXCLUDED from quick wins per finding-004.
```
Fetch each `#NN` for the full body (B1).
</details>


### 🟠 P1 — Platform-envelope mismatch (#410–#417 + new evidence)

> arcaven's 8-issue constitutional critique (#410 tracker) — decision still open: re-platform /
> amend constitution / record risk-acceptance. **This run adds four evidence issues from the
> pilots:** #458 (Phase-4 gate arithmetic assumes machine-observable ACs; 6 of 7 akey holdout
> scenarios need hardware/humans), #461 (auto-mode classifier vetoes the "standing merge
> authorization" the doctrine promises — permission-laundering workarounds appearing), #473
> (workflow step tasks the write-denied orchestrator with a write — constitution-vs-workflow
> contradiction), #474 (routed to P0b — the gate that can only pass by waiver). The
> envelope-mismatch cluster is no longer just an argument; it is generating gate failures.

| What it is | # | Type · repro · verdict |
|---|---|---|
| Tracking issue — dark-factory requirements exceed Claude Code plugin envelope | **#410** | tech-debt · n/a · **P1-KEYSTONE · ADVANCES** (prior) |
| No external supervisor — orchestrator and orchestrated share one failure domain | **#411** | tech-debt · Mandel · **⏸ HALT** (prior) |
| CAP-012 loss bound unverifiable · session lifecycle = REPL · no introspection/bulkheads · actor-topology mismatch · resource opacity | **#413 #414 #415 #416 #417** | (prior — see run-10 detail) |
| **Hardware/human-gated holdout criteria need a first-class category + resume-queue** — Phase-4 halts on physical products | **#458** | enhancement · Bohr · **⏸ HALT · ADVANCES · systemic** (NEW) |
| **Auto-mode classifier blocks agent merge — doctrine promises an affordance the substrate denies** (self-flagged scope doubt: factory vs Claude Code ownership) | **#461** | possibly-out-of-scope · Bohr · **⏸ HALT · NEUTRAL · systemic** (NEW) |
| **Scenario-rotation step tasks the write-denied orchestrator with writing a file** — unexecutable as declared | **#473** | bug · Bohr · **⏸ HALT · fix MERGED (PR #717) — open pending reporter verify** |
| Research-agent declares MCP tools absent at spawn; halts split-dispatch | #516 | bug · Bohr · **ADVANCES · systemic** (NEW) |
| Phase-4 holdout budgets frozen without target-build capability canary | #601 | process-gap · Bohr · **ADVANCES · systemic** (NEW) |
| Validate-pr-review-posted unreachable on single-identity project | #651 | bug · Bohr · **ADVANCES · systemic** (NEW) |


### 🟠 P1 — Operational: halts the autonomous pipeline (`⏸ impact.halt` · finding-009)

> The liveness axis. **This window took a kill: #658 (STORY_ID regex) closed after arcavenai
> PR #719 merged** — the halt lane's first close. #681 (dual-validator deadlock — following
> either shipped contract hard-blocks the agent; external analysis flags it
> strongest-of-batch, see agent channel) remains the natural first pull. Panic #647 rides
> P0b as integrity-primary; run-13's panic trio (#541 #548 #569) carries unchanged.

| What it is | # | Type · repro · verdict |
|---|---|---|
| Lock RENEW still repo-relative in state-burst/state-manager — unreachable in installed-plugin context (completes merged #526; the #472 class) | **#732** | bug · Bohr · **⏸ HALT · ADVANCES · systemic · ★** (NEW run-15 · arcaven) |
| Validate-pr-review-posted unreachable on single-identity project | #651 | bug · Bohr · **ADVANCES · systemic** (NEW) |
| Validate-pr-merge-prerequisites STORY_ID regex misses named story IDs | ~~#658~~ | bug · Bohr · **CLOSED ✓ 2026-07-22 via PR #719** |
| Validate-burst-log vs shipped template: dual-validator deadlock | #681 | bug · Bohr · **ADVANCES · systemic** (NEW) |
| Red-gate tests asserting stub artifacts are un-greenable by design | #684 | process-gap · Bohr · **ADVANCES · systemic** (NEW) |
| Single-account deployments can't post formal PR approval the 9-step process assumes | #696 | process-gap · Bohr · **ADVANCES · systemic** (NEW) |
| Guard removal breaks tests that depended on guard's no-op | #702 | process-gap · Bohr · **ADVANCES · systemic** (NEW) |
| Run-13 lane (panic trio #541 #548 #569 · halts #543 #555 #604 #626) + run-12/11 lanes (#503 #508 #513 · #457 #451 #469) + older halt tail (#321 #326 #343 #346 #347 #349 #357 #380 #386) | | (carried — #472 CLOSED ✓ this window; see index + maintainer progress) |

<details><summary>🤖 agent channel — #681 external signal + halt-lane reading</summary>

```
#681 (validate-burst-log dual-validator deadlock): an outside analysis of this batch flags
#681 as its strongest filing. Recorded per B3 as signal-to-reconcile: the grader
independently scored it systemic/P1/halt with dual cluster (operational-halt +
spec-propagation-drift) and the curator concurs — both shipped contracts hard-block the
agent, the escape hatch is validator-source archaeology, and the body carries four concrete
fixes. It is the natural first pull of the halt lane. Panic #647 is P0b-primary (integrity)
and rides both lists. #637 renders in P0b (integrity outranks halt on the same row).
```
</details>

### 🟠 P1 — KEYSTONE consolidations

> Single highest-leverage actionable items — each closes a whole class upstream.

| What it is | # | Type · repro · verdict |
|---|---|---|
| Mandatory deterministic pre-review lint layer (~63-pair external pilot) — **#466 (adversary runs the linter) is a direct subset; fold it in** | **#336** | enhancement · Bohr · **KEYSTONE** (prior) |
| **Product-level defect register** — no mode has a post-ship defect log; the shipped-broken-demo class (#432/#433) has nowhere to land | **#432** | gap · Bohr · **KEYSTONE · ADVANCES · systemic** (NEW) |
| **Probe variance, not pass count, drives convergence quality** — hostile-input structure axis + path-component enumeration; the 3-clean-pass certificate is vacuous on a narrow probe distribution | **#462** | process-gap · Mandel · **KEYSTONE · ADVANCES · systemic** (NEW) |
| **Fleet-mining engine asks** — model_rationale frontmatter, per-dispatch outcome telemetry, escalation ladders, uniform lessons registry (4,602 dispatches, 84% of bad outcomes not model capability) | **#463** | proposal · n/a · **KEYSTONE · ADVANCES · systemic** (NEW · arcaven) |
| **Remediation disposition-sweep contract** (7-pass sibling-miss pattern + wrong-value-propagated-with-fidelity) — also P0b above | **#470** | (see P0b) |
| **Smoke sentinel gate: quality-green ≠ operator-green** — nothing between `git add` and `gh pr merge` executes the artifact at the operator boundary | **#487** | enhancement · Bohr · **KEYSTONE · ADVANCES · systemic** (NEW) |
| **Steady-state enforcement surface** — also P0b above; #489/#491/#493 are its facets | **#488** | (see P0b) |
| POL-003 dispatch tables · impl-adds-API lint · defense-in-depth input provenance | #406 #419 #426 | (prior keystones — see run-10 detail) |
| Adversary CLEAN verdict can contradict non-empty findings list | **#576** | process-gap · Mandel · **🛑 INTEGRITY (spec_process) · ADVANCES · systemic · KEYSTONE — pairs with #513: mechanical verdict derivation from both sides** (NEW) |
| Factory-graph: derived traceability graph from .factory/ markdown | **#671** | proposal · n/a · **ADVANCES · systemic · KEYSTONE — maintainer-authored; 👤 attn.direction; one ruling reframes the traceability lane** (NEW) |


### 🟠 P1 — Convergence soundness (methodology + propagation)

> The methodology wave matures from cataloging failures to specifying the missing
> verification steps: quantity-assertions on drain loops (#697), mutation-verified
> security invariants (#676), full-suite semantics (#677), per-state-class fix sweeps
> (#679), pipeline-authored gate independence (#698). Read #710 (👤) as this run's
> consolidation anchor — it carries the empirical dataset for WHY absence-of-findings
> convergence under-measures.

| What it is | # | Type · repro · verdict |
|---|---|---|
| Canonicalize SAP-3 + SID-2 probes from prism into agent specs | #633 | proposal · n/a · **ADVANCES · systemic** (NEW) |
| Story-level holdout gate — three-tier holdout in per-story delivery | #634 | enhancement · n/a · **ADVANCES · systemic** (NEW) |
| Mutation restore hits last commit, silently drops uncommitted fix | #645 | process-gap · Bohr · **ADVANCES · systemic** (NEW) |
| Red Gate stub panic on shared path regresses existing tests | #662 | process-gap · Bohr · **ADVANCES · systemic** (NEW) |
| Step (f) silently degrades to self-review without spawn capability | #674 | process-gap · Bohr · **ADVANCES · systemic** (NEW) |
| Seam-covered ACs can mask unwired production entrypoint | #675 | process-gap · Bohr · **ADVANCES · systemic** (NEW) |
| Security-invariant ACs need mutation verification of the guard | #676 | process-gap · Bohr · **ADVANCES · systemic** (NEW) |
| 'Full suite green' claims require --no-fail-fast enumeration | #677 | process-gap · Bohr · **ADVANCES · systemic** (NEW) |
| Class-scoped fix verified only on representative members: false green | #679 | process-gap · Bohr · **ADVANCES · systemic** (NEW) |
| Internally-unsatisfiable spec (signature vs sibling postcondition) passes all consistency audits | #693 | process-gap · Mandel · **ADVANCES · systemic** (NEW) |
| Feature-gated verification test silently excluded from CI — headline proof never runs on gate | #694 | process-gap · Bohr · **ADVANCES · systemic** (NEW) |
| Error-return negative tests must assert discriminator; timeout backstop fakes non-nil error | #699 | proposal · Bohr · **ADVANCES · systemic** (NEW) |
| AC obligation names multiple paths but its test exercises only one — sibling path unimplemented, green | #700 | process-gap · Bohr · **ADVANCES · systemic** (NEW) |
| Prior methodology core (#593 #462 #497 #500 #467) + cascade pair (#445 #446 #449) + prior cluster (#390 #391 #429 #327 #322 #305 #309 #310 #297 ~~#300~~) + run-13 wave | | (carried — **#300 CLOSED ✓ 2026-07-22 via PR #723**; see run-13 fold + indexes) |

### 🟢 P2 — Run-15 findings (NEW · advances, no SDL, no panic)

> 9 new P2s (+ P1 #732 above). The downstream-pilot burst dominates: two adversary-axis
> proposals (#752 boundary-sentinel inputs, #753 channel-lifecycle asymmetry — both grounded
> in concrete S-3.02 instances, not speculation), the narrated-verification class (#746 —
> read the thread: the root cause was corrected in-flight, the surviving defect is sharper
> than the filing), version-qualifier drift (#749 — thread points at generalizing
> BC-5.39.003 rather than new machinery), and reverse symbol→anchor coverage (#751 —
> overlaps #419, thread suggests scoping together). Repo-hygiene pair #734/#741 are ★
> quick-win-eligible. #733 (arcaven) reconciles the #532 clause with the adversary's
> read-only toolset — two prose-only shapes offered.

| What it is | # | Type · repro · verdict |
|---|---|---|
| Adversary compile-state clause unsatisfiable for read-only agent — false-CLEAN edge on the #532 class | #733 | process-gap · unknown · **ADVANCES · systemic** (NEW · arcaven) |
| Author-absolute /Users/jmagady paths in shipped skill examples (13+ sites; class guard proposed) | #734 | bug · Bohr · **ADVANCES · ★** (NEW) |
| Self-referential .lazyclaude gitlink on develop — git 128 warning on every CI job, breaks --recurse-submodules | #741 | bug · Bohr · **ADVANCES · systemic · ★** (NEW) |
| pr-manager step-9 cleanup verified by narration, not output — step 9 inherits step 8's conclusion unverified | #746 | process-gap · Mandel · **ADVANCES · systemic** (NEW) |
| Adversary coherence-lens verifies status flags but not description/call-path columns — wrong prose survived 5 passes | #748 | process-gap · Bohr · **ADVANCES · systemic** (NEW) |
| Version-qualifier drift in prose citations invisible to input-hash detection — 2 instances in one run | #749 | process-gap · Bohr · **ADVANCES · systemic** (NEW) |
| Exported symbols added during rework escape BC/architecture anchoring — gate never walks symbol→anchor | #751 | process-gap · Bohr · **ADVANCES · systemic** (NEW) |
| Axis proposal: boundary-sentinel inputs — out-of-range test params fake coverage of the production path | #752 | proposal · Bohr · **ADVANCES · systemic** (NEW) |
| Axis proposal: channel-lifecycle asymmetry — divergent teardown sets across graceful/exceptional paths | #753 | proposal · Bohr · **ADVANCES · systemic** (NEW) |

### 🟢 P2 — Run-14 process/policy/test-writer findings (advances, no integrity, no halt)

> 36 run-14 P2s, grouped by primary cluster — act by lever, not one-by-one. Per-issue chips in
> the run-14 classification index below; ★ items also appear in the quick-wins lane.

| Cluster (count) — representative | Issues |
|---|---|
| **verification-methodology** (15) — e.g. Base-branch fixes invisible to PR CI without actual branch update | #641 #643 #648 #649 #652 #654 #661 #667 #668 #669 #678 #687 #695 #705 #713 |
| **spec-propagation-drift** (4) — e.g. Story ACs must enumerate per-variant test-function names (recurrence 3) | #653 #656 ~~#660~~✓ #708 |
| **dispatch-contract** (4) — e.g. Resume prompt must include worktree path + branch + pre-commit assertion | #655 #666 #683 #707 |
| **test-writer-gaps** (4) — e.g. Stale RED-gate docstrings survive Red→Green; no gate catches them | #665 #682 #697 #701 |
| **state-manager-hygiene** (2) — e.g. Story index row accumulates unbounded per-revision changelog prose | #711 #712 |
| **artifact-containment** (1) — e.g. Demo-recorder protocol lacks host-path scrub; /Users paths committed | #636 |
| **operational-halt** (1) — e.g. PR-close guard can't distinguish abandon vs close-reopen-for-CI | #642 |
| **authority-substrate** (1) — e.g. Shared git identity defeats per-role commit attribution forensics | #644 |
| **adversary-sweep-enumeration** (1) — e.g. Finding decay is non-monotone; only N-consecutive-CLEAN is sound | #686 |
| **false-green-convergence** (1) — e.g. validate-count-propagation mis-parses epic ID E-11 as count '11 stories' | #690 |
| **plugin-pack-ci** (1) — e.g. Pipeline-authored coverage gate: maximal-coverage default + hand-rolled YAML scan ignores list-form | #698 |
| **dispatch-race** (1) — e.g. Fixed attestation port makes concurrent lanes' port tests flake | #703 |

### 🟢 P2 — Prior clusters (unchanged this run)

Run-12 P2 detail (24 rows incl. the #504→#507 sweep chain + #435 #442 #443 #459 #480 #484
#490) carried in the run-12 render (docs/fixtures) + indexes below. Older tail unchanged —
abbreviated to protect the body budget: dispatch/test/spec tail (#368 #369 #371 #375 #376
#377 #378 #328 #329 #334 #335 #338 #344 #345 #350 #351 #352 #353 #354 #359 #360 #361 #362
#363 #364 #366 #367 #382 #383 #387 #388 #392 #393 #394 #395 #398 #400 #405 #407 #417 #420
#422 #423 #424), observability thread (#317–#325, root cause #415), CI-cost cluster (#308,
read WITH #336), trust-scope reminder (#316).


## 🟦 Quick wins — safe to act on (low-caution lane · orthogonal to impact)

> **This lane is now the proven channel:** this window closed **#660** (v-prefix normalize →
> PR #722) and **#566** (BC-title scoping → PR #721) from this lane, merged the fix for #690
> (PR #716, open pending verify), and prior conversions #472/#724-fix hold. Eligibility is
> **derived** (mechanical + bounded blast + cited fix + alignment ≠ drifts). **Lane exclusion
> is RULE (SKILL §7a, beadle PR #34):** integrity anchors (#747 #750), SDL, and panic never
> ride here regardless of mechanical eligibility. Remaining on-ramp: **#732** (completes
> #526's own conversion — two missed sites), then #741, #734.

| What it is | # | Type · repro · priority |
|---|---|---|
| Lock RENEW repo-relative in state-burst/state-manager — apply #526's own conversion to 2 missed sites | **#732** | bug · Bohr · **⏸ P1** (NEW run-15 · arcaven) |
| Author-absolute /Users/jmagady paths in shipped examples — replace + /Users/-literal content guard | **#734** | bug · Bohr · **P2** (NEW run-15) |
| .lazyclaude gitlink: `git rm --cached` + .gitignore + mode-160000 health check (covers #263 too) | **#741** | bug · Bohr · **P2** (NEW run-15) |
| Canonicalize SAP-3 + SID-2 probes from prism into agent specs | #633 | proposal · n/a · **P1** (NEW) |
| Demo-recorder protocol lacks host-path scrub; /Users paths committed | #636 | process-gap · Bohr · **P2** (NEW) |
| Decision-citation hook regex false-positives on look-alike IDs | #641 | bug · Bohr · **P2** (NEW) |
| Base-branch fixes invisible to PR CI without actual branch update | #643 | process-gap · Bohr · **P2** (NEW) |
| Mutation restore hits last commit, silently drops uncommitted fix | #645 | process-gap · Bohr · **P1** (NEW) |
| Story ACs must enumerate per-variant test-function names (recurrence 3) | #653 | process-gap · n/a · **P2** (NEW) |
| Bundle-scoped mutation runs need --timeout 480 / --jobs 2 heuristic | #654 | process-gap · n/a · **P2** (NEW) |
| Resume prompt must include worktree path + branch + pre-commit assertion | #655 | process-gap · n/a · **P2** (NEW) |
| Story-writer must count params on pinned signatures vs lint threshold | #656 | process-gap · n/a · **P2** (NEW) |
| Validate-changelog-monotonicity doesn't normalize 'v' version prefix | ~~#660~~ | bug · Bohr · **CLOSED ✓ 2026-07-22 via PR #722** |
| Red Gate: uncommitted stubs then tests-alone = non-compiling commit | #661 | process-gap · n/a · **P2** (NEW) |
| Dispatches recommend safer form instead of mandating it | #666 | process-gap · Mandel · **P2** (NEW) |
| MUST-before-X obligations lack explicit discharge records | #667 | process-gap · Bohr · **P2** (NEW) |
| 'Full suite green' claims require --no-fail-fast enumeration | #677 | process-gap · Bohr · **P1** (NEW) |
| String-grep seam-exclusion check blind to control-flow-only seams | #678 | process-gap · Bohr · **P2** (NEW) |
| Stale RED-gate docstrings survive Red→Green; no gate catches them | #682 | process-gap · Bohr · **P2** (NEW) |
| Red-gate tests asserting stub artifacts are un-greenable by design | #684 | process-gap · Bohr · **⏸** · **P1** (NEW) |
| Validate-count-propagation mis-parses epic ID E-11 as count '11 stories' | #690 | bug · Bohr · **fix MERGED (PR #716) — open pending reporter verify** |
| Fixed attestation port makes concurrent lanes' port tests flake | #703 | process-gap · Mandel · **P2** (NEW) |
| Overlapping attestation controls make pixels contradict code | #705 | process-gap · Bohr · **P2** (NEW) |
| Step guard counts STEP_COMPLETE per stop-event, forces re-emission | #707 | process-gap · Bohr · **P2** (NEW) |
| New BC error code not registered in canonical error-taxonomy | #709 | process-gap · Bohr · **P1** (NEW) |
| Story index row accumulates unbounded per-revision changelog prose | #712 | process-gap · Bohr · **P2** (NEW) |
| Prior lane (run-13: #533 #539 #545 #549 #557 #558 ~~#566~~ #567 #580 #581 #582 #583 #591 #593 #594 #597 #598 #622 #628 #629 #630 #631 · run-12: #515 (partial fix landed, PR #524) · run-11: #436 #444 #447 #450 #453 #466 #469 #471 #476 #478 #482 #495 #498 #499 #501 · older: #396 #397 #401 #402 #403 #389 #407 #408 #409 #352 #323 #320 #209 #239 #256 #280 #282) | | (**#472 #418 #566 CLOSED ✓** — #566 this window via PR #721 · rest open) |

<details><summary>🤖 agent channel — quick-win §7a rule check + exclusion audit (runs 14–15)</summary>

```
§7a RULE CHECK (run-15):
- 3 admitted: #732 #734 #741 — all mechanical/bounded with cited fixes, none flagged.
- integrity HARD-EXCLUDE (3): #724 #747 #750 — none grader-admitted (contract enforces);
  #724 additionally already resolved.
- SDL / panic rule-exclude: none this run (0 SDL, 0 panic in the batch).
- attn never-QW: no attn tags this run.
- Curator overrides: NONE in either direction. Grader-vs-rule agreement on all 13.
- Note: #746's proposed fix was refuted in-thread (would regress the exact-line match the
  shipped check already has) — a reminder the lane requires the CITED fix to be correct,
  not just small; #746 stays out on that basis (quick_win_disqualification recorded).
- Maintainer on-ramp order: #732 (your own #526 conversion, two missed sites) → #741
  (two-command repo hygiene) → #734 (path scrub + guard).
```
```
§7a RULE CHECK (run-14, first run under the rule, beadle PR #34):
- 21 admitted: #633 #636 #641 #643 #645 #653 #654 #655 #656 #660 #661 #666 #667 #677 #678 #682 #684 #690 #703 #705 #707 #709 #712
- integrity HARD-EXCLUDE (13): #637 #638 #647 #650 #663 #664 #672 #673 #680 #685 #692 #704 #710 — none were grader-admitted (contract enforces).
- SDL rule-exclude: #635 — grader did not admit (silent_data_loss=true blocks eligibility at grading).
- panic rule-exclude: #647 — grader did not admit (integrity + panic).
- attn never-QW: #671 #686 #710 — none grader-admitted.
- Curator overrides this run: NONE in either direction. Rule and grader agreed on every lane
  decision; the audit exists because §7a now requires it whenever they disagree.
- #684 (halt + grader-eligible, un-greenable stub-artifact tests) admitted WITH ⏸ chip per
  run-11/13 precedent (#469/#472-class: bounded fix to a halting surface — the #472 close
  validates the pattern).
- Maintainer on-ramp order: #690 (your own count-parser bug) → #653/#654/#655/#656 (your own
  process asks) → #661 (v-prefix normalize) → #660 (changelog monotonicity) → rest.
```
</details>

## Direction Health — corpus-level (the led-by-the-backlog check)

Machine-computed this run (`beadle direction`, finding-014/016/017 signals; vocabulary
contract PR #35). Verdict: **🔴 DRIFTING** · top signal: **A4 silent-data-loss
zero-engagement**. **Falsification meter (aae-orc#65, pre-registered 2026-07-20 — reported
every run): PREDICTION EVALUATED THIS RUN — FAILED.** The registered criterion: by run-16
the HARD acted-on ceiling moves past P0b (maintainer merge/close-with-resolution/commit on
a P0a/SDL item they did not file, discharging A4) — else "ramping stops explaining the data
and the drifting read stands." Observed: hard ceiling = **P0b for the third consecutive
window** (this window's rung actions: #623 close via PR #718 merge, #637 fix-PR #715 merge)
· soft ceiling = P0b · A4 streak = **7 runs** · P0a/SDL actions = **0**, against the largest
action volume ever recorded (22 in one day). Per the pre-registration: the RAMPING
qualifier is retired; no criteria re-derivation. Re-qualification requires a new
pre-registered prediction — not a narrative read.

| Signal | Verdict | Reading |
|---|---|---|
| `maintainer_action` (the compass) | **turning — strongest window on record** | +22 actions (13 merges · 9 closes); cumulative 58. Third consecutive window with hard P0b actions on non-self-filed items |
| `A4 silent-data-loss zero-engagement` (finding-016) | **drifting** | #342/#365/#358 at a **7-run zero-engagement streak**; #523/#588 at 4; #635 (maintainer-authored SDL) at 3. 22 actions landed in one day; zero touched this lane |
| `acted-on severity ceiling` (ramp meter, aae-orc#65) | **flat at P0b — prediction FAILED** | P0b → P0b → P0b across three windows. The volume rose an order of magnitude; the ceiling did not move. "Past P0b" ⇔ P0a/SDL action ⇔ A4 discharge — did not occur by the registered deadline |
| `integrity-density` (B) | improving (open class) | 0 new; open P0b class net −1 (#623 closed) with #637's fix merged pending verify |
| `silent-data-loss share` (C) | on-course / stalled | 0 new; the open lane is unchanged at 6 — and now the meter's headline residue |
| `filing_density` | quiet | **0 this run** — first zero-new-issue window; the measured filer paused while the maintainer drained |
| `filed_vs_acted_gap` | narrowing sharply | 427:58 = **7.4:1** (12.9 → 12.1 → 7.4) — this window's movement is entirely on the acted side |
| `convert_finding_to_guard_unbounded` | n/a | no new filings to screen |
| `over_engineering_fingerprints` | n/a | no new filings to screen |

- **What the failed prediction does and does not say:** it retires RAMPING as the
  qualifier — it does not diminish the engagement. The maintainer drained 9 tracked defects
  and merged 13 fix PRs in a day; the corpus is visibly useful to him. What the meter
  isolates is the **selection function**: every acted item arrived with a mergeable fix
  attached; the SDL lane (whose items require reading a mechanism analysis and making a
  worktree/merge-discipline call, no cheap diff available) has waited 7 runs. Stable
  small-and-easy selection at high volume is still drift with respect to the corpus's own
  severity ordering — that is precisely the read the pre-registration protected from being
  softened by a big engagement day.
- **Run-15 addendum, kept (fix-delivery-cost hypothesis):** the acted-on ceiling may be as
  much a function of **fix-delivery cost** as of severity — #724 resolved in <36h because
  the fix arrived as a mergeable PR. This window strengthens that observation (13/13 merges
  were measured-side PRs). The falsifiable follow-on: if the measured side ships fix PRs
  against the SDL lane and they still sit, cost stops explaining the ceiling; if they merge,
  A4 discharges and the drift read falls. Either way the next meter movement is decisive —
  worth registering as the next prediction (operator slot; not self-registered here).

## Classification index (finding-005 + finding-009 + attn facet)

**Run-16: zero new issues — nothing to classify.** State changes this run are closes/merges,
recorded in Maintainer progress and the struck board rows above.

### Run-15 index (carried forward — most recent detail)

report-type · defect-nature · repro · alignment · flags. Axis vectors ingested verbatim
(vocabulary.json contract). Rich fixture:
docs/fixtures/vsdd-factory-312-run15-classifications-rich.json. Six rows carry in-thread
arcaven fact-checks (recorded in the store rationales) — read the thread before acting on
#746 (root cause corrected) and #741 (gitlink is self-referential, not dangling; impact
unchanged). Follow `#NN` for bodies (B1).

| What it is | # | chips |
|---|---|---|
| Live sprint-state assertions gated every PR on base drift | ~~#724~~ | ci-build · design · Bohr · adv · **🛑** **⏸** · **RESOLVED ✓** |
| Lock RENEW repo-relative in state-burst/state-manager (installed-context) | **#732** | bug · logic · Bohr · adv · **⏸** **★** *(arcaven)* |
| Adversary compile-state clause unsatisfiable for read-only agent | #733 | process-gap · spec · unknown · adv *(arcaven)* |
| Author-absolute /Users/jmagady paths in shipped skill examples | #734 | bug · syntax · Bohr · adv · **★** |
| Self-referential .lazyclaude gitlink — git 128 on every CI job | #741 | bug · lifecycle · Bohr · adv · **★** |
| pr-manager step-9 cleanup verified by narration, not output | #746 | process-gap · spec · Mandel · adv |
| Post-convergence production edits merge without gate re-entry | **#747** | process-gap · design · Bohr · adv · **🛑** |
| Coherence-lens verifies status flags, not description/call-path columns | #748 | process-gap · design · Bohr · adv |
| Version-qualifier drift invisible to input-hash detection | #749 | process-gap · design · Bohr · adv |
| Errata cited as authority exist only as changelog rows | **#750** | process-gap · lifecycle · Bohr · adv · **🛑** |
| Exported rework symbols escape BC/architecture anchoring | #751 | process-gap · design · Bohr · adv |
| Axis proposal: boundary-sentinel inputs (false coverage) | #752 | proposal · design · Bohr · adv |
| Axis proposal: channel-lifecycle asymmetry (teardown leaks) | #753 | proposal · lifecycle · Bohr · adv |

### Run-14 index (carried forward — folded)

72 issues (#633–#713), chips unchanged — full per-issue rows in the run-14 body
(docs/fixtures/vsdd-factory-312-curated-run14.md) + store records (verbatim vocabulary
ingest, beadle PR #35; rich fixture vsdd-factory-312-run14-classifications-rich.json):
#633 #634 #635 #636 #637 #638 #641 #642 #643 #644 #645 #647 #648 #649 #650 #651 #652 #653
#654 #655 #656 #658 #660 #661 #662 #663 #664 #665 #666 #667 #668 #669 #671 #672 #673 #674
#675 #676 #677 #678 #679 #680 #681 #682 #683 #684 #685 #686 #687 #690 #692 #693 #694 #695
#696 #697 #698 #699 #700 #701 #702 #703 #704 #705 #706 #707 #708 #709 #710 #711 #712 #713.
Cluster rollup: integrity 13 (🛑) · SDL #635 (▲) · panic #647 (💥) · keystones #671 #576 ·
quick wins 21 (★) · attn #671 #686 #710 (👤) · Zious11 filings 12 · neutral pair #642 #707.

### Run-13 index (carried forward — folded)

104 issues (#516–#631), chips unchanged — full per-issue rows in the run-13 body
(docs/fixtures/vsdd-factory-312-curated-run13.md) + store records (impact re-canonicalized
by migrate-impact): #516 #517 #518 #519 #520 #521 #522 #523 #525 #533 #534 #535 #536 #537
#538 #539 #540 #541 #542 #543 #544 #545 #546 #547 #548 #549 #550 #551 #552 #553 #554 #555
#556 #557 #558 #559 #560 #561 #562 #563 #564 #565 #566 #567 #568 #569 #570 #571 #572 #573
#574 #575 #576 #577 #578 #579 #580 #581 #582 #583 #584 #585 #586 #587 #588 #589 #590 #591
#592 #593 #594 #595 #596 #597 #598 #599 #600 #601 #602 #603 #604 #605 #606 #607 #608 #609
#614 #615 #616 #617 #618 #619 #620 #621 #622 #623 #624 #625 #626 #627 #628 #629 #630 #631.
Cluster rollup: integrity 14 (🛑) · SDL pair #523/#588 (▲) · panic trio #541/#548/#569 (💥) ·
keystone #576 · quick wins 22 (★) · Zious11 filings #576–#584.

### Run-12 index (carried forward — folded)

13 issues (#503–#515), all still open, chips unchanged — full per-issue rows in the run-15
body (docs/fixtures/vsdd-factory-312-curated-run15.md) + store records: #503 #504 #505 #506
#507 #508 #509 #510 #511 #512 #513 #514 #515. Cluster rollup: sweep-chain quartet
#504/#505/#506/#507 (#507 KEYSTONE-adjacent terminus) · degraded pair #503/#508 (worktree) ·
keystone #513 (evidence-paste, pairs with #576) · 👤 #510 (governance / reply-needed) ·
★ #515 (partial fix PR #524, .gitignore half open) · minutiae #514.


### Run-11 index (carried forward — folded)

66 issues (#432–#502), all still open, chips unchanged — full per-issue rows in the run-12
body (docs/fixtures/vsdd-factory-312-curated-run12.md) + store classification records:
#432 #433 #434 #435 #436 #437 #440 #441 #442 #443 #444 #445 #446 #447 #448 #449 #450 #451
#452 #453 #457 #458 #459 #460 #461 #462 #463 #464 #465 #466 #467 #468 #469 #470 #471 #472
#473 #474 #475 #476 #477 #478 #479 #480 #481 #482 #483 #484 #485 #486 #487 #488 #489 #490
#491 #492 #493 #494 #495 #496 #497 #498 #499 #500 #501 #502. Cluster rollup: integrity
block 23 (🛑) · methodology trio #462/#497/#500 · keystones #432 #462 #463 #470 #487 #488 ·
quick wins 16 (★) · halts 8 (⏸) · neutral scope-doubt pair #460/#461 · #263 gitlink
containment precedent (with #341/#515).

## Maintainer progress (outcome-paired — NOT a leaderboard)

Surfaces resolution **against beadle-discovered defects**, rewarding the verified fix-outcome
(flagged → fixed → validates), never close-rate / time-to-triage / volume (B3 + no-Goodhart).
**This window is the largest single-day drain on record.**

| Outcome signal | Count | State |
|---|---|---|
| **Defects closed with a validated fix** | **13** | prior 4: #418 (PR #528) · #472 (PR #526) · #465 (PR #532) · #724 (PR #725) — **+9 this window (all 2026-07-22):** #623 (PR #718 — **THIRD P0b close**: input-hash upsert) · #658 (PR #719 — first halt-lane close) · #660 (PR #722) · #566 (PR #721) · #300 (PR #723) · #296 (PR #726) · #243 (PR #730) · #229 (PR #736) · **#244 (PR #731 — ⚠ keyword auto-close: the squash commit's stale `Closes` fired though the body was corrected to `Refs` in review; the deferred math-test criterion is residual — reopen-or-refile offered in-thread)** |
| **Fixes merged, close pending reporter verify** | **3** | #637 (🛑 P0b — PR #715) · #690 (PR #716) · #473 (⏸ — PR #717, completing PR #527's partial) — all merged with `Refs` per the reporter-confirm discipline |
| **Partial fixes landed + engaged** | 4 | #515 (PR #524, fix 1 of 2) · #434 (PR #529, part 1) · #424 (PR #531 + correction #691) · #475 (PR #530, part 1) |
| **arcavenai PRs merged** | **22 / 31** | 8 (2026-07-19) · #725 (2026-07-21) · **13 (2026-07-22): #715 #716 #717 #718 #719 #721 #722 #723 #726 #730 #731 #736 #739** · 9 still open |
| **Correction/review cycle** | active | #691 corrects #531 (run-14) · this window: post-merge body-count correction requested on #726 · #728 merge conditioned on executing the documented conflict resolution · substantive review rounds on #737/#738 — the channel reviews hard and merges fast |
| Field-data comment-events on measured issues | 23 | unchanged (run-13 trio #298 #428 #507 pattern holds) — this window's maintainer prose landed on PRs (reviews), not issue threads |
| Silent data-loss (P0a) closed with validated fix | **0 / 6** | #342 #365 #358 (**A4: 7-run streak**) + #523 #588 (4 runs) + #635 (3 runs, maintainer-authored). **The ramp prediction came due here this run and was NOT discharged — FAILED (see Direction Health)** |
| Source-of-truth integrity (P0b) closed | **3 / 79** | #465 ✓ #724 ✓ **#623 ✓** · 76 open (incl. #747 #750; #637 fix merged pending verify) |
| Halt/panic (P1 operational) closed + validated | **3 / 40** | #472 ✓ #724 ✓ **#658 ✓** · panic 4 open (#541 #548 #569 #647) · #473 fix merged pending verify |
| Envelope decision (#410) taken | 0 / 1 | 13+ issues wait on it (👤 attn.direction) — untouched by the wave |
| **Needs-human-reading items answered** | **0 / 6** | #510 (reply-needed — **17 days**, counterparty still holding PRs) · #410 · #463 · #671 #686 #710 — zero of 22 actions touched this lane |
| Keystone consolidations accepted + wired | 0 / 17 | 16 prior + #671 (your co-maintainer's own — accept it?) |
| Quick wins exercised | **3 / 79** | #472 ✓ **#660 ✓ #566 ✓** · #690 fix merged pending verify · on-ramp: #732 #741 #734 |
| **Measured-side hygiene** (not maintainer credit) | 3 | #226 self-closed not-reproducible (2026-07-20) · arcaven's 6 in-thread fact-checks (run-15) · #244 residual-scope comment posted same-day as the auto-close |

_Cold-start lifted for engagement metrics (the wheel completed verified turns); rate/streak
claims start accruing against this run's baseline. Still no per-human ranking — the metric
moves only when a real defect gets a real, validated fix._

## Controls

Derived from authoritative state each render — never hand-authored. The next run parses
`- [x] <!-- verb=...;id=... -->`, dispatches, then resets the box (eventually consistent).

```
# Tier 1 — per-issue verbs (bounded to one artifact)
```
- [ ] <!-- verb=investigate;id=#510 --> Read & rule on #510 (👤 reply-needed — **17 days**; external contribution flow; counterparty holding PRs on your answer)
- [ ] <!-- verb=fast-track;id=#635 --> Fast-track #635 (🛑💾 NEW P0a — **your own filing**: mid-gate streak counter wipes unpersisted CLEAN passes)
- [ ] <!-- verb=fast-track;id=#342 --> Fast-track #342 (🛑💾 P0a — merge silently deletes a served .factory file · A4 **7-run streak** · the meter’s next decisive movement)
- [ ] <!-- verb=fast-track;id=#365 --> Fast-track #365 (🛑💾 P0a — rebase silently drops production code · A4 **7-run streak** · the meter’s next decisive movement)
- [ ] <!-- verb=fast-track;id=#358 --> Fast-track #358 (🛑💾 P0a — orphan merge; merge-base --is-ancestor assertion · A4 **7-run streak** · the meter’s next decisive movement)
- [ ] <!-- verb=fast-track;id=#523 --> Fast-track #523 (🛑💾 P0a — story-worktree .factory artifacts lost at teardown)
- [ ] <!-- verb=fast-track;id=#588 --> Fast-track #588 (🛑▲ P0a — factory-side PR strands shared .factory worktree)
- [ ] <!-- verb=investigate;id=#671 --> Rule on #671 (👤 KEYSTONE — your co-maintainer's factory-graph proposal; gates the traceability lane)
- [ ] <!-- verb=investigate;id=#681 --> Deep-investigate #681 (⏸ P1 — dual-validator deadlock; strongest-of-batch external signal; four cited fixes)
- [ ] <!-- verb=investigate;id=#647 --> Deep-investigate #647 (🛑💥 P0b+panic — mid-edit burst death leaves corrupted continuation baseline)
- [ ] <!-- verb=investigate;id=#710 --> Read #710 (👤 evidence-brief + 🛑 P0b — the convergence under-measurement dataset)
- [ ] <!-- verb=investigate;id=#576 --> Deep-investigate #576 (🛑 KEYSTONE, your own filing — mechanical verdict derivation; pairs with #513)
- [ ] <!-- verb=investigate;id=#513 --> Deep-investigate #513 (KEYSTONE — evidence-paste requirement kills the false-green class)
- [ ] <!-- verb=investigate;id=#599 --> Deep-investigate #599 (🛑 P0b — vacuous agent-authored lint gates)
- [ ] <!-- verb=investigate;id=#614 --> Deep-investigate #614 (🛑 P0b — GNU-BRE gates false-GREEN on macOS)
- [ ] <!-- verb=investigate;id=#592 --> Deep-investigate #592 (🛑 P0b — async adversary verdicts dropped/stale-bound)
- [ ] <!-- verb=investigate;id=#488 --> Deep-investigate #488 (🛑 KEYSTONE — steady-state bypasses every gate)
- [ ] <!-- verb=investigate;id=#410 --> Deep-investigate #410 (KEYSTONE tracker — envelope-mismatch; 13 issues wait on it)
- [ ] <!-- verb=investigate;id=#463 --> Deep-investigate #463 (KEYSTONE — fleet-mining engine asks · 👤 standing)
- [ ] <!-- verb=investigate;id=#747 --> Deep-investigate #747 (🛑 NEW P0b — production edits merge past the convergence seal; thread offers the cheap converged_sha shape)
- [ ] <!-- verb=investigate;id=#750 --> Investigate #750 (🛑 NEW P0b — errata cited as authority with no resolvable artifact; retrofit path is cheap)
- [ ] <!-- verb=fast-track;id=#732 --> Fast-track #732 (quick-win ⏸ P1 — your own #526 conversion, two missed sites; lock renew unreachable installed)
- [ ] <!-- verb=fast-track;id=#741 --> Fast-track #741 (quick-win — two-command gitlink removal + mode-160000 health check, covers #263 too)
- [ ] <!-- verb=fast-track;id=#653 --> Fast-track #653 (quick-win — your own per-variant test-name enumeration ask)
- [ ] <!-- verb=fast-track;id=#661 --> Fast-track #661 (quick-win — Red Gate non-compiling middle state)
- [ ] <!-- verb=fast-track;id=#515 --> Fast-track #515 fix 2 (quick-win — the .gitignore half; fix 1 landed in PR #524)
- [ ] <!-- verb=fast-track;id=#469 --> Fast-track #469 (quick-win ⏸ — traceability regex one-liner)

```
# Tier 2 — board-level maintenance requests (read/analyze/regenerate only — B2)
```
- [ ] <!-- verb=reprioritize;id=board --> Reprioritize the whole board (fresh pass over the full open corpus, 479)
- [ ] <!-- verb=full-refresh;id=board --> Full refresh (re-enumerate, re-validate, re-render)
- [ ] <!-- verb=revalidate;id=board --> Re-validate open/closed state of every listed issue
- [ ] <!-- verb=rescore-intent;id=board --> Re-score intent alignment (manifest v0.3) across the corpus

_Boxes are parsed on the next scheduled run (cheap-poll-then-act). Irreversible/public actions
still escalate per B2 — these verbs only read, analyze, and regenerate._

