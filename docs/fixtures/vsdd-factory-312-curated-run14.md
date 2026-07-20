# 📋 beadle — Triage Dashboard · drbothen/vsdd-factory

<!-- beadle-state:v1
{"schema":1,"last_run":"2026-07-20","watermark":713,"store":"jsonl","intent_version":"vsdd-factory@0.3","warmup":"cold-start","run":14,"digest":"run14-72-maintainer-turn-8-merges-3-closes-first-p0b-close-465-a4-drifting-5run-sdl-silence-zious-sdl-debut-635-2026-07-20","counts":{"open":477,"arcavenai_open":427,"maintainer_engaged":33,"arcavenai_closed_alltime":4},"attn":{"governance":[510],"direction":[410,671],"evidence_brief":[463,686,710]},"axis_model":"finding-005 superset + finding-009: report-type, defect-nature, reproducibility, triage-state, leverage, alignment, provenance, integrity(safety), operational-impact(liveness)","p0_data_loss":[342,358,365,523,588,635],"p0_integrity":[313,314,330,331,332,333,337,339,341,348,355,356,370,372,373,374,379,381,399,404,412,421,425,427,428,430,433,434,437,440,441,448,452,465,468,470,474,475,477,479,481,483,485,486,488,491,492,494,496,517,535,537,538,544,546,547,576,589,592,599,600,614,623,637,638,647,650,663,664,672,673,680,685,692,704,710],"new_this_run":[633,634,635,636,637,638,641,642,643,644,645,647,648,649,650,651,652,653,654,655,656,658,660,661,662,663,664,665,666,667,668,669,671,672,673,674,675,676,677,678,679,680,681,682,683,684,685,686,687,690,692,693,694,695,696,697,698,699,700,701,702,703,704,705,706,707,708,709,710,711,712,713],"prior_run_burst":[432,433,434,435,436,437,440,441,442,443,444,445,446,447,448,449,450,451,452,453,457,458,459,460,461,462,463,464,465,466,467,468,469,470,471,472,473,474,475,476,477,478,479,480,481,482,483,484,485,486,487,488,489,490,491,492,493,494,495,496,497,498,499,500,501,502],"operational_impact":{"halt":[321,326,343,346,347,349,357,380,386,411,414,451,457,458,461,469,472,473,474,516,543,555,601,604,626,637,651,658,681,684,696,702],"panic":[541,548,569,647],"data_loss":[342,365,358,523],"degraded_new":[638,641,642,643,644,645,660,662,663,664,665,666,667,668,669,671,672,673,674,675,676,677,679,682,685,687,690,692,693,694,695,697,698,699,700,701,703,705,706,707,708,710,711,712,713]},"keystone":336,"keystone_new":[671],"keystone_prior":[406,410,413,415,416,419,426,432,462,463,470,487,488,507,513,576],"quick_wins_new":[633,636,641,643,645,653,654,655,656,660,661,666,667,677,678,682,684,690,703,705,707,709,712],"quick_wins_prior":[436,444,447,450,453,466,469,471,472,476,478,482,495,498,499,501],"clusters":{"silent-data-loss":[342,358,365,412,479,523,588,635,645],"source-of-truth-integrity":[399,404,412,421,425,427,428,430,433,434,437,440,441,448,452,465,468,470,474,475,477,479,481,483,485,486,488,491,492,494,496,517,535,537,538,544,546,547,589,600,614,623,637,638,647,650,663,664,672,673,680,685,692,704,709],"false-green-convergence":[305,309,310,322,327,330,331,332,333,337,339,348,353,355,356,360,364,370,373,381,390,391,393,397,398,399,421,425,429,433,434,440,441,442,448,452,462,465,467,468,470,474,475,477,479,480,481,484,485,486,488,490,492,494,496,497,500,513,533,535,543,553,563,576,591,592,599,614,618,620,627,634,643,664,672,673,674,675,676,677,685,686,687,690,694,698,699,700,704,705,710],"operational-halt":[321,326,343,346,347,349,357,380,386,411,414,451,457,458,461,469,472,473,474,503,539,541,548,555,569,604,626,637,642,681,696],"platform-envelope-mismatch":[410,411,412,413,414,415,416,417,458,461,473,474,516,601,614,625,649,651,656],"test-writer-gaps":[329,335,354,359,363,364,373,381,383,393,394,398,402,418,419,420,422,423,424,426,436,442,460,466,471,475,477,479,480,481,485,490,494,496,498,499,500,502,520,556,571,575,598,607,609,633,653,665,679,682,684,697,699,700,701,702,713],"spec-propagation-drift":[361,362,367,369,376,382,387,388,390,391,392,395,396,400,401,403,404,406,418,419,422,427,428,429,430,432,435,443,444,445,446,447,449,450,453,459,471,476,478,482,484,488,489,491,493,519,521,522,537,550,552,554,559,561,564,565,573,574,580,581,582,594,600,615,622,638,647,653,656,660,668,671,680,681,693,695,708,709,711],"adversary-sweep-enumeration":[504,505,506,507,511,519,520,534,548,563,564,573,574,606,615,633,652,686],"verification-methodology":[509,512,517,518,522,525,533,534,535,536,539,540,542,543,544,545,546,549,550,553,555,556,557,559,560,562,566,567,585,586,587,590,593,595,596,599,601,602,603,605,607,608,616,617,618,620,621,627,633,634,641,643,645,648,649,652,654,658,660,661,662,663,664,665,667,668,669,671,675,676,677,678,679,682,687,690,693,694,695,697,698,699,700,701,702,703,704,705,710,713],"artifact-containment":[263,341,515,589,597,630,636,650],"scratch-isolation":[508,624,701],"contribution-governance":[510,551,651,696],"dispatch-contract":[328,368,386,405,409,448,457,459,463,468,473,482,516,518,521,525,540,545,549,551,558,569,589,619,630,655,658,661,662,666,674,683,696,706,707],"authority-substrate":[372,374,379,426,483,508,605,644],"observability":[317,318,319,320,324,325,415,463,464,495,501,583,628,629,635,644,648,667],"plugin-pack-ci":[349,351,357,469,472,476,492,625,694,698],"orchestrator-continuation":[343,347,380,408,409,446,457,486,495,604,635,647,692,707],"branch-protection":[346,348,349,357,408,626],"dispatch-race":[345,350,355,368,451,547,570,592,594,619,703],"worktree-factory-split":[341,342,451,452,483,503,508,523,588,624,631,655],"state-manager-hygiene":[517,538,547,572,629,631,641,692,706,711,712],"variants-gallery-review":[568],"convergence-tuning":[577,578,579],"learning-loop":[584]},"maintainer_actions":{"comments_prior":{"298":"2026-07-08","428":"2026-07-08","507":"2026-07-08"},"comments":{"507":"2026-07-15","521":"2026-07-15","649":"2026-07-15","651":"2026-07-15","443":"2026-07-15","494":"2026-07-19","461":"2026-07-19","686":"2026-07-19","682":"2026-07-19","305":"2026-07-19","655":"2026-07-19","396":"2026-07-19","582":"2026-07-19","654":"2026-07-19","290":"2026-07-19","418":"2026-07-19","472":"2026-07-19","465":"2026-07-19","473":"2026-07-19","515":"2026-07-19","434":"2026-07-19","424":"2026-07-19","475":"2026-07-19"},"filings":[635,636,637,638,652,653,654,655,656,671,690,695],"merges":{"524":"2026-07-19","526":"2026-07-19","527":"2026-07-19","528":"2026-07-19","529":"2026-07-19","530":"2026-07-19","531":"2026-07-19","532":"2026-07-19"},"closes":{"418":"2026-07-19","465":"2026-07-19","472":"2026-07-19"},"corrections":{"691":"corrects #531 (merged PR) \u2014 Go/Node stdin diagnosis"}}}
beadle-state -->

**Direction verdict: 🔴 DRIFTING — the compass turned for real this window, and the alarm that fires is precisely the one built for this moment.**
The triage wheel completed its **first full turns**: Zious11 merged **all 8 arcavenai PRs**
(#524 #526–#532, 2026-07-19), closed **3 tracked defects with validated fixes** — #472 (⏸
quick-win, lock-helper paths), #418 (demo-tape relocation), and **#465, the first
source-of-truth-integrity (P0b) close** (adversary ground-truth verification, PR #532) — left
partial-fix engagement on 5 more (#515 #434 #424 #473 #475), added field-data comments on 10
measured issues, and shipped one correction cycle (#691 fixing #531's Go/Node stdin diagnosis —
reviewed, corrected, and merged rather than reverted). ADR-005's cold-start condition is
**lifted for engagement metrics**. And yet the machine verdict is **drifting**, because A4
(finding-016) now fires at its sharpest: **#342/#365/#358 reach a 5-run zero-engagement streak
while 33 maintainer actions landed everywhere else** — silence on the highest-severity class is
no longer explainable by general inattention. The run also took +72 issues (watermark #631 →
#713, 58 arcavenai · 12 Zious11 · 2 drbothen), including the corpus's **first
maintainer-authored silent-data-loss filing (#635)** and a maintainer keystone proposal
(#671, factory-graph). _Updated 2026-07-20 · watermark #713 · run 14 · intent vsdd-factory@0.3
· finding-005/009 axes + attn facet · vocabulary.json contract (beadle PRs #34/#35) ·
skills-based refresh (binary render retired per run-10 regression)_

> **vsdd-factory is a self-referential factory — engine == product.** Engine process-gaps and
> meta findings are **on-mission**, scored on leverage + provenance, not dismissed as
> self-reference. **70 of 72 new issues score `advances`** (2 neutral: #642 guard-semantics
> scope-doubt, #707 self-flagged bounded lint ask); **72 of 72 pilot-derived** with cited
> commits/runs/instances — the first zero-speculative burst on record. Run-11's
> integrity-block reading, run-12's sweep-chain reading, and run-13's checkers-lie reading are
> unchanged and carried below.

> **NEW this run — 72 issues, four themes.** **(1) Verification-methodology continues as
> center of mass** (42 issues): quantity-blind drain loops (#697), coverage gates authored by
> the pipeline they gate (#698), full-suite-green claims without --no-fail-fast (#677),
> fix-scope verification across state classes (#679). **(2) The maintainer is now filing INTO
> the corpus's own classes:** 12 Zious11 filings including a **silent-data-loss instance
> (#635 — mid-gate streak-counter wipe)**, three integrity items (#637 #638 #652), and the
> factory-graph keystone proposal (#671). **(3) TDD/red-gate hardening wave:** stale RED-gate
> docstrings surviving Green (#682), un-greenable stub-artifact tests (#684), seam-masked
> unwired entrypoints (#675), security-invariant mutation checks (#676). **(4) Liveness:**
> panic #647 (writing-agent burst dies mid-edit leaving partial artifacts + a corrupted
> continuation baseline — also P0b) alongside 7 new halts including **#681 (dual-validator
> deadlock — external analysis flags it the strongest filing of the batch; see agent
> channel)**.

## Baseline (counts — a starting line, first rates unlocked)

| Metric | Value | Outcome pairing |
|---|---|---|
| Open issues | 477 | arcavenai 427 · Zious11 27 · arcaven 14 · drbothen 8 · slabgorb 1 |
| Filer concentration | arcavenai ~90% | Zious11 now 27 open own filings (12 this run) — the process-gap lens is co-owned |
| New this run | +72 (#633–#713) | 58 arcavenai · 12 Zious11 · 2 drbothen · 72/72 pilot-derived · 14 P0 · 22 P1 · 36 P2 |
| Maintainer engagement | **33 actions — the wheel's first full turns** | 8 PR merges + 3 closes-with-validated-fix + 22 comment-events (2026-07-13..19) |
| Closes against tracked defects | **3** (#418 #465 #472) | each cites the fixing PR + merge commit; #465 is the **first P0b close** |
| filed-vs-acted gap | 427 open : 33 acted | first real denominator — structural ratio now reportable with a trend baseline next run |
| Δ since last run (#631→#713) | +72 open | integrity share 18% (13/72) · methodology share 58% (42/72) · SDL 1 (maintainer-authored) |

## 👤 Needs human reading — direct attention (NEW lane · orthogonal to priority)
> A small group whose **special nature benefits from the maintainer reading them directly and
> in full** — the value is in the prose and cannot survive summarization into chips. Orthogonal
> to priority: an item here can be P2 on impact yet first in your reading queue. Ordered by
> reading priority: **reply-needed** (a counterparty is blocked on your answer) →
> **gates-work** (the decision unblocks other open work) → **standing** (context that improves
> everything downstream). Never quick-win-eligible; never folded into cluster rollups.
| What it is | # | attn · order · why direct reading |
|---|---|---|
| **External contribution flow proposal (ArcavenAE)** — first external contributor asks how to route artifact-bearing changes; proposes artifact-content-via-issue backed by a 2000-trial simulation (tables + charts in comments) | **#510** | `attn.governance` · **reply-needed** · consent/relationship decision only maintainers can make; ArcavenAE holds artifact-bearing PRs until your ruling (also P2 · **no response after 15 days** — engagement landed on 30+ other artifacts this window; PR #524 even references #510 without ruling on it) |
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
</details>

## Action plan — grouped by INTENT, integrity-first (finding-004 precedence)

Rows lead with **what the issue is**; `#NN` is the agent's fetch reference; trailing chips are
status. `repro` badge feeds the effort estimate; `⏸ halt`/`💥 panic` = operational-impact
(finding-009); `🛑` = source-of-truth integrity (finding-004); `💾` = data-loss. Verdicts cited
against the v0.3 rubric. All issues verified OPEN this run.


### 🔴 P0a — Silent data-loss (highest severity: irreplaceable state destroyed/stranded with no signal)

> Each is **integrity × data-loss** (or pure SDL): a system-of-record silently diverges from
> reality AND destroys or strands state behind a green check. Sits **above** everything else
> by precedence. **A4 alarm (finding-016) — DRIFTING, machine-computed:** #342/#365/#358 now
> at a **5-run zero-engagement streak**, and this run the silence is load-bearing: 33
> maintainer actions landed elsewhere while the SDL lane stayed untouched. #523/#588 continue
> their own clock (2 runs). **NEW and notable: #635 is the corpus's first maintainer-authored
> SDL** — Zious11 independently documents the mid-gate adversarial streak counter wiping CLEAN
> passes it never persisted.

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
> (finding-004). **The class got its first kill this window: #465 closed via PR #532 —
> adversary verdicts now require ground-truth verification.** 13 new this run keep the
> pressure on the same mechanism from new angles: hook-layer deadlock fixtures (#647 — also
> the run's panic), self-contradictory specs surviving consistency gates (#693), input-hash
> recorded at decomposition never re-verified (#672), story-index accumulating unreconciled
> revisions (#712 — via #710's dataset), and the burst-log schema the validator enforces
> against its own shipped template (#681's sibling facet, see P1).

| What it is | # | Type · repro · verdict |
|---|---|---|
| $(cat) strips trailing newlines — input-hash false-drift hard-block | **#637** | bug · Bohr · **🛑 INTEGRITY (spec_process) · ADVANCES · systemic** (NEW) |
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
  forked v1.97), #537/#538/#544/#546/#547/#623 (state/attestation/protocol corruption paths).
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
| **Scenario-rotation step tasks the write-denied orchestrator with writing a file** — unexecutable as declared | **#473** | bug · Bohr · **⏸ HALT · ADVANCES · systemic** (NEW) |
| Research-agent declares MCP tools absent at spawn; halts split-dispatch | #516 | bug · Bohr · **ADVANCES · systemic** (NEW) |
| Phase-4 holdout budgets frozen without target-build capability canary | #601 | process-gap · Bohr · **ADVANCES · systemic** (NEW) |
| Validate-pr-review-posted unreachable on single-identity project | #651 | bug · Bohr · **ADVANCES · systemic** (NEW) |


### 🟠 P1 — Operational: halts the autonomous pipeline (`⏸ impact.halt` · finding-009)

> The liveness axis. One new **panic** (#647 — writing-agent burst dies mid-edit; partial
> artifact + corrupted continuation baseline; rides P0b as integrity-primary) and **7 new
> halts**, led by **#681 (dual-validator deadlock — following either shipped contract
> hard-blocks the agent; external analysis flags it strongest-of-batch, see agent channel)**.
> Run-13's panic trio (#541 #548 #569) and halt lane carry unchanged.

| What it is | # | Type · repro · verdict |
|---|---|---|
| Validate-pr-review-posted unreachable on single-identity project | #651 | bug · Bohr · **ADVANCES · systemic** (NEW) |
| Validate-pr-merge-prerequisites STORY_ID regex misses named story IDs | #658 | bug · Bohr · **ADVANCES · systemic** (NEW) |
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
| Prior methodology core (#593 #462 #497 #500 #467) + cascade pair (#445 #446 #449) + prior cluster (#390 #391 #429 #327 #322 #305 #309 #310 #297 #300) + run-13 wave | | (carried — see run-13 fold + indexes) |

### 🟢 P2 — Process/policy/test-writer findings (NEW · advances, no integrity, no halt)

> 36 new P2s, grouped by primary cluster — act by lever, not one-by-one. Per-issue chips in
> the run-14 classification index below; ★ items also appear in the quick-wins lane.

| Cluster (count) — representative | Issues |
|---|---|
| **verification-methodology** (15) — e.g. Base-branch fixes invisible to PR CI without actual branch update | #641 #643 #648 #649 #652 #654 #661 #667 #668 #669 #678 #687 #695 #705 #713 |
| **spec-propagation-drift** (4) — e.g. Story ACs must enumerate per-variant test-function names (recurrence 3) | #653 #656 #660 #708 |
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

> **This lane converted:** #472 (⏸ lock-helper paths) was fixed and closed via PR #526 — the
> first quick-win exercised, exactly the process-turning on-ramp this lane exists for.
> Eligibility is **derived** (mechanical + bounded blast + cited fix + alignment ≠ drifts).
> **Lane exclusion is now RULE (SKILL §7a, beadle PR #34):** integrity anchors (13 new P0b),
> SDL (#635), and panic (#647) never ride here regardless of mechanical eligibility. **21 new
> quick wins this run — 3 maintainer-authored (#653 #654 #655 #656 family, #690), continuing
> the fix-your-own-filings on-ramp that worked.**

| What it is | # | Type · repro · priority |
|---|---|---|
| Canonicalize SAP-3 + SID-2 probes from prism into agent specs | #633 | proposal · n/a · **P1** (NEW) |
| Demo-recorder protocol lacks host-path scrub; /Users paths committed | #636 | process-gap · Bohr · **P2** (NEW) |
| Decision-citation hook regex false-positives on look-alike IDs | #641 | bug · Bohr · **P2** (NEW) |
| Base-branch fixes invisible to PR CI without actual branch update | #643 | process-gap · Bohr · **P2** (NEW) |
| Mutation restore hits last commit, silently drops uncommitted fix | #645 | process-gap · Bohr · **P1** (NEW) |
| Story ACs must enumerate per-variant test-function names (recurrence 3) | #653 | process-gap · n/a · **P2** (NEW) |
| Bundle-scoped mutation runs need --timeout 480 / --jobs 2 heuristic | #654 | process-gap · n/a · **P2** (NEW) |
| Resume prompt must include worktree path + branch + pre-commit assertion | #655 | process-gap · n/a · **P2** (NEW) |
| Story-writer must count params on pinned signatures vs lint threshold | #656 | process-gap · n/a · **P2** (NEW) |
| Validate-changelog-monotonicity doesn't normalize 'v' version prefix | #660 | bug · Bohr · **P2** (NEW) |
| Red Gate: uncommitted stubs then tests-alone = non-compiling commit | #661 | process-gap · n/a · **P2** (NEW) |
| Dispatches recommend safer form instead of mandating it | #666 | process-gap · Mandel · **P2** (NEW) |
| MUST-before-X obligations lack explicit discharge records | #667 | process-gap · Bohr · **P2** (NEW) |
| 'Full suite green' claims require --no-fail-fast enumeration | #677 | process-gap · Bohr · **P1** (NEW) |
| String-grep seam-exclusion check blind to control-flow-only seams | #678 | process-gap · Bohr · **P2** (NEW) |
| Stale RED-gate docstrings survive Red→Green; no gate catches them | #682 | process-gap · Bohr · **P2** (NEW) |
| Red-gate tests asserting stub artifacts are un-greenable by design | #684 | process-gap · Bohr · **⏸** · **P1** (NEW) |
| Validate-count-propagation mis-parses epic ID E-11 as count '11 stories' | #690 | bug · Bohr · **P2** (NEW) |
| Fixed attestation port makes concurrent lanes' port tests flake | #703 | process-gap · Mandel · **P2** (NEW) |
| Overlapping attestation controls make pixels contradict code | #705 | process-gap · Bohr · **P2** (NEW) |
| Step guard counts STEP_COMPLETE per stop-event, forces re-emission | #707 | process-gap · Bohr · **P2** (NEW) |
| New BC error code not registered in canonical error-taxonomy | #709 | process-gap · Bohr · **P1** (NEW) |
| Story index row accumulates unbounded per-revision changelog prose | #712 | process-gap · Bohr · **P2** (NEW) |
| Prior lane (run-13: #533 #539 #545 #549 #557 #558 #566 #567 #580 #581 #582 #583 #591 #593 #594 #597 #598 #622 #628 #629 #630 #631 · run-12: #515 (partial fix landed, PR #524) · run-11: #436 #444 #447 #450 #453 #466 #469 #471 #476 #478 #482 #495 #498 #499 #501 · older: #396 #397 #401 #402 #403 #389 #407 #408 #409 #352 #323 #320 #209 #239 #256 #280 #282) | | (**#472 CLOSED ✓** · #418 CLOSED ✓ · rest open) |

<details><summary>🤖 agent channel — quick-win §7a rule check + exclusion audit (run-14)</summary>

```
§7a RULE CHECK (first run under the rule, beadle PR #34):
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
zero-engagement**.

| Signal | Verdict | Reading |
|---|---|---|
| `maintainer_action` (the compass) | **turned** | 33 actions: 8 merges + 3 validated closes (#418 #465 #472) + 22 comment-events. ADR-005 cold-start **lifted** for engagement metrics; rates get a baseline next run |
| `A4 silent-data-loss zero-engagement` (finding-016) | **drifting** | #342/#365/#358 at a **5-run zero-engagement streak**; #523/#588 at 2. The sharpest form: silence-amid-engagement. #635 (maintainer-authored SDL) starts its own clock |
| `integrity-density` (B) | watch | 13/72 = 18.1% (run-13: 13.5%) — the false-green class keeps being found faster than it is closed (1 close vs 13 new) |
| `silent-data-loss share` (C) | on-course | 1/72 = 1.4% — and the 1 is maintainer-authored, which is corroboration, not noise |
| `filing_density` | drifting (mechanical) | trailing-window math still dominated by run-13's +104; run-14's +72 continues the large-burst regime — composition remains consolidation-shaped (58% methodology) |
| `filed_vs_acted_gap` | first denominator | 427 open : 33 acted — reportable as a ratio with a trend baseline from run-15 |
| `convert_finding_to_guard_unbounded` | watch | antidote candidates: #698 (gate-independence), #679 (state-class sweeps), #671 (factory-graph subsumes per-citation guards) — prefer consolidations over ~15 new one-off guard asks |
| `over_engineering_fingerprints` | LOW | 72/72 pilot-derived, zero speculative — best on record; 2 neutral are honest scope-doubt |

- **Meta-signal (strongest yet):** the maintainer now *authors into* the corpus's own
  top-severity classes — an SDL filing (#635), three integrity filings, and a keystone
  consolidation proposal (#671). Independent convergence from the party with ground truth —
  now with commit history to back it (8 merges).
- **The asymmetry to watch:** engagement flowed to bounded, mechanical, agent-guidance fixes
  (the 8 merges are all prompt/template/path-discipline changes). The architectural lanes —
  P0a worktree/merge mechanics, the envelope decision (#410), the attn queue (#510 at 15
  days) — stayed untouched. The wheel turns where the fixes are small; the alarm fires where
  they are not.

## Classification index — run-14 new issues (finding-005 + finding-009 + attn facet)

report-type · defect-nature · repro · alignment · flags. Axis vectors ingested **verbatim**
this run (vocabulary.json contract, beadle PR #35 — the run-13 lossy-map era is over; store
runs 10–13 migrated via `classify migrate-impact`, aae-orc-57g6). Rich fixture:
docs/fixtures/vsdd-factory-312-run14-classifications-rich.json. Follow `#NN` for bodies (B1).

| What it is | # | chips |
|---|---|---|
| Canonicalize SAP-3 + SID-2 probes from prism into agent specs | #633 | proposal · spec · n/a · adv · **★** *(drbothen)* |
| Story-level holdout gate — three-tier holdout in per-story delivery | #634 | enhancement · design · n/a · adv · *(drbothen)* |
| CLEAN passes/streak never persisted mid-gate; crash loses position | **#635** | process-gap · design · Bohr · adv · **▲** *(Zious11)* |
| Demo-recorder protocol lacks host-path scrub; /Users paths committed | #636 | process-gap · spec · Bohr · adv · **★** *(Zious11)* |
| $(cat) strips trailing newlines — input-hash false-drift hard-block | **#637** | bug · logic · Bohr · adv · **🛑** **⏸** *(Zious11)* |
| Finding-ID scheme not canonical — G-less form collides, enables fakes | **#638** | process-gap · spec · Bohr · adv · **🛑** *(Zious11)* |
| Decision-citation hook regex false-positives on look-alike IDs | #641 | bug · logic · Bohr · adv · **★** |
| PR-close guard can't distinguish abandon vs close-reopen-for-CI | #642 | process-gap · design · Bohr · **neutral** |
| Base-branch fixes invisible to PR CI without actual branch update | #643 | process-gap · spec · Bohr · adv · **★** |
| Shared git identity defeats per-role commit attribution forensics | #644 | process-gap · design · Bohr · adv |
| Mutation restore hits last commit, silently drops uncommitted fix | #645 | process-gap · design · Bohr · adv · **★** |
| Writer crash mid-burst leaves changelog overclaiming unedited body | **#647** | process-gap · design · Mandel · adv · **🛑** **💥** |
| Plugin timeout emits fail-closed label but write lands (fail-open) | #648 | process-gap · design · n/a · adv |
| PostToolUse dispatcher blocks valid write on plugin timeout | #649 | bug · design · Mandel · adv |
| Full-file Write leaks stray </content> tag into committed spec artifact | **#650** | bug · logic · Bohr · adv · **🛑** |
| Validate-pr-review-posted unreachable on single-identity project | **#651** | bug · design · Bohr · adv · **⏸** |
| Adversary mutation-coverage claims must be backed by empirical cargo-mutants run | #652 | process-gap · spec · n/a · adv · *(Zious11)* |
| Story ACs must enumerate per-variant test-function names (recurrence 3) | #653 | process-gap · spec · n/a · adv · **★** *(Zious11)* |
| Bundle-scoped mutation runs need --timeout 480 / --jobs 2 heuristic | #654 | process-gap · config · n/a · adv · **★** *(Zious11)* |
| Resume prompt must include worktree path + branch + pre-commit assertion | #655 | process-gap · spec · n/a · adv · **★** *(Zious11)* |
| Story-writer must count params on pinned signatures vs lint threshold | #656 | process-gap · spec · n/a · adv · **★** *(Zious11)* |
| Validate-pr-merge-prerequisites STORY_ID regex misses named story IDs | **#658** | bug · logic · Bohr · adv · **⏸** |
| Validate-changelog-monotonicity doesn't normalize 'v' version prefix | #660 | bug · logic · Bohr · adv · **★** |
| Red Gate: uncommitted stubs then tests-alone = non-compiling commit | #661 | process-gap · spec · n/a · adv · **★** |
| Red Gate stub panic on shared path regresses existing tests | #662 | process-gap · spec · Bohr · adv |
| Identifiers retyped from memory silently corrupt records | **#663** | process-gap · spec · Mandel · adv · **🛑** |
| Review verdicts accepted without ground-truthing components | **#664** | process-gap · spec · Mandel · adv · **🛑** |
| Regression guards freeze cells owned by other work axes | #665 | process-gap · design · Bohr · adv |
| Dispatches recommend safer form instead of mandating it | #666 | process-gap · spec · Mandel · adv · **★** |
| MUST-before-X obligations lack explicit discharge records | #667 | process-gap · spec · Bohr · adv · **★** |
| Expired freezes keep shielding stale artifact sections | #668 | process-gap · design · Bohr · adv |
| Polarity inversion survives coordinate-focused review | #669 | process-gap · design · Bohr · adv |
| Factory-graph: derived traceability graph from .factory/ markdown | **#671** | proposal · design · n/a · adv · **KEYSTONE** **👤** *(Zious11)* |
| Decompose omits terminal input-hash --update; hash wrong from birth | **#672** | process-gap · spec · Bohr · adv · **🛑** |
| Blocking validators' fix-instructions coerce fabricated record text | **#673** | process-gap · design · Bohr · adv · **🛑** |
| Step (f) silently degrades to self-review without spawn capability | #674 | process-gap · design · Bohr · adv |
| Seam-covered ACs can mask unwired production entrypoint | #675 | process-gap · spec · Bohr · adv |
| Security-invariant ACs need mutation verification of the guard | #676 | process-gap · spec · Bohr · adv |
| 'Full suite green' claims require --no-fail-fast enumeration | #677 | process-gap · spec · Bohr · adv · **★** |
| String-grep seam-exclusion check blind to control-flow-only seams | #678 | process-gap · spec · Bohr · adv · **★** |
| Class-scoped fix verified only on representative members: false green | #679 | process-gap · spec · Bohr · adv |
| Spec's false API claim steers code toward invariant violation | **#680** | process-gap · spec · Bohr · adv · **🛑** |
| Validate-burst-log vs shipped template: dual-validator deadlock | **#681** | bug · design · Bohr · adv · **⏸** |
| Stale RED-gate docstrings survive Red→Green; no gate catches them | #682 | process-gap · spec · Bohr · adv · **★** |
| Reviewer declares read-only toolset; runtime does not enforce it | #683 | process-gap · design · Bohr · adv |
| Red-gate tests asserting stub artifacts are un-greenable by design | **#684** | process-gap · spec · Bohr · adv · **⏸** **★** |
| Fix-phase self-attestation: DONE report false, guard test neutered | **#685** | process-gap · design · Mandel · adv · **🛑** |
| Finding decay is non-monotone; only N-consecutive-CLEAN is sound | #686 | process-gap · design · Mandel · adv · **👤** |
| Spec-artifact citation-coherence is ungated defect class stalling convergence | #687 | process-gap · spec · Mandel · adv |
| Validate-count-propagation mis-parses epic ID E-11 as count '11 stories' | #690 | bug · logic · Bohr · adv · **★** *(Zious11)* |
| Interleaved delivery leaves STATE.md next-pointer aimed at already-merged story | **#692** | process-gap · design · Mandel · adv · **🛑** |
| Internally-unsatisfiable spec (signature vs sibling postcondition) passes all consistency audits | #693 | process-gap · spec · Mandel · adv |
| Feature-gated verification test silently excluded from CI — headline proof never runs on gate | #694 | process-gap · config · Bohr · adv |
| CHANGELOG must be enumerated from git log first-parent, not hand-summarized — epic omitted | #695 | process-gap · spec · Mandel · adv · *(Zious11)* |
| Single-account deployments can't post formal PR approval the 9-step process assumes | **#696** | process-gap · design · Bohr · adv · **⏸** |
| Quantity-blind expected-error drain loops mask event multiplicity — double-emission passed green | #697 | process-gap · logic · Bohr · adv |
| Pipeline-authored coverage gate: maximal-coverage default + hand-rolled YAML scan ignores list-form | #698 | bug · logic · Bohr · adv |
| Error-return negative tests must assert discriminator; timeout backstop fakes non-nil error | #699 | proposal · logic · Bohr · adv |
| AC obligation names multiple paths but its test exercises only one — sibling path unimplemented, green | #700 | process-gap · spec · Bohr · adv |
| Singleton test-spy via deferred-free+rename corrupts singleton path; leak reads as pre-existing failures | #701 | process-gap · logic · Heisen · adv |
| Guard removal breaks tests that depended on guard's no-op | **#702** | process-gap · spec · Bohr · adv · **⏸** |
| Fixed attestation port makes concurrent lanes' port tests flake | #703 | process-gap · design · Mandel · adv · **★** |
| Pipe to grep masks runner exit code; post-summary crash reads green | **#704** | process-gap · spec · Bohr · adv · **🛑** |
| Overlapping attestation controls make pixels contradict code | #705 | process-gap · design · Bohr · adv · **★** |
| Verify-git-push non-deterministically blocks non-force git push | #706 | bug · logic · Mandel · adv |
| Step guard counts STEP_COMPLETE per stop-event, forces re-emission | #707 | process-gap · design · Bohr · **neutral** · **★** |
| Call-contract fix sweep misses godoc header and sibling BC specs | #708 | process-gap · design · Bohr · adv |
| New BC error code not registered in canonical error-taxonomy | #709 | process-gap · design · Bohr · adv · **★** |
| Convergence counts clean passes without vector-coverage novelty | **#710** | process-gap · spec · Mandel · adv · **🛑** **👤** |
| Self-flagged follow-on sweep creates no tracked obligation | #711 | process-gap · design · Bohr · adv |
| Story index row accumulates unbounded per-revision changelog prose | #712 | process-gap · design · Bohr · adv · **★** |
| RED gate ignores predicted-vs-observed failure signature mismatch | #713 | process-gap · design · Bohr · adv |

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

### Run-12 index (carried forward)

| What it is | # | chips |
|---|---|---|
| WorktreeCreate hook returns no path | **#503** | bug · logic · Bohr · adv · degraded |
| Sweep must span spec+VP artifacts | #504 | process-gap · spec · Bohr · adv |
| Sweep story pseudocode + arch tables | #505 | process-gap · spec · Bohr · adv |
| Sweep BC self-metadata | #506 | process-gap · spec · Bohr · adv |
| Peer-artifact sweep (consolidation terminus) | **#507** | process-gap · spec · Bohr · adv · **KEYSTONE-adjacent** |
| cp -r worktree poisons shared git config | **#508** | bug · design · Bohr · adv · degraded |
| Verify encoded artifacts in decoded form | #509 | process-gap · logic · Bohr · adv |
| External contribution flow proposal | **#510** | proposal · intent · n/a · adv · **👤 governance / reply-needed** |
| Assertion content vs BC prose drift | #511 | process-gap · spec · Bohr · adv |
| ARCH-11 reverse-trace machine check | #512 | process-gap · spec · Bohr · adv |
| Green claims need pasted evidence | **#513** | process-gap · spec · Mandel · adv · **KEYSTONE** |
| Combined-footnote coupling flag | #514 | enhancement · design · n/a · adv · minutiae |
| Root-level red-gate log on develop | #515 | bug · config · Bohr · adv · **★** |


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
**This section has real rows for the first time.**

| Outcome signal | Count | State |
|---|---|---|
| **Defects closed with a validated fix** | **3** | #418 (PR #528, demo-tape self-locating cd) · #472 (PR #526, lock-helper `${{CLAUDE_PLUGIN_ROOT}}` paths — the quick-wins lane's first conversion) · **#465 (PR #532 — FIRST P0b close: adversary ground-truth verification mandate)** |
| **Partial fixes landed + engaged** | **5** | #515 (PR #524, fix 1 of 2) · #434 (PR #529, part 1) · #424 (PR #531 + correction #691) · #473 (PR #527, both defects functionally fixed) · #475 (PR #530, part 1) — each has a Zious11 resolution comment citing the merge |
| **arcavenai PRs merged** | **8** | #524 #526 #527 #528 #529 #530 #531 #532 (all 2026-07-19, ten-minute window) |
| **Correction cycle** | 1 | #691 corrects #531's Go/Node stdin diagnosis — reviewed-and-corrected beats reverted; the contribution channel is functioning |
| Field-data comment-events on measured issues | 22 | SOH-cycle datapoints (#507 #521 #649 #651 #443, 07-15) + downstream confirmations (#290 #305 #396 #461 #494 #582 #654 #655 #682 #686, 07-19) — extending the run-13 trio (#298 #428 #507) |
| Silent data-loss (P0a) closed with validated fix | 0 / 6 | #342 #365 #358 (**A4: 5-run streak**) + #523 #588 + #635 (NEW, maintainer-authored) |
| Source-of-truth integrity (P0b) closed | **1 / 76** | #465 ✓ · 62 prior + 13 new open |
| Halt/panic (P1 operational) closed + validated | **1 / 38** | #472 ✓ · panic 4 open (#541 #548 #569 #647) |
| Envelope decision (#410) taken | 0 / 1 | 13+ issues wait on it (👤 attn.direction) |
| **Needs-human-reading items answered** | **0 / 6** | #510 (reply-needed — **15 days**, counterparty still holding PRs) · #410 · #463 · NEW: #671 #686 #710 |
| Keystone consolidations accepted + wired | 0 / 16 | 15 prior + #671 new (your co-maintainer's own — accept it?) |
| Quick wins exercised | **1 / 76** | #472 ✓ — the on-ramp works; 21 new + 54 prior remain |

_Cold-start lifted for engagement metrics (the wheel completed verified turns); rate/streak
claims start accruing against this run's baseline. Still no per-human ranking — the metric
moves only when a real defect gets a real, validated fix._

## Controls

Derived from authoritative state each render — never hand-authored. The next run parses
`- [x] <!-- verb=...;id=... -->`, dispatches, then resets the box (eventually consistent).

```
# Tier 1 — per-issue verbs (bounded to one artifact)
```
- [ ] <!-- verb=investigate;id=#510 --> Read & rule on #510 (👤 reply-needed — **15 days**; external contribution flow; counterparty holding PRs on your answer)
- [ ] <!-- verb=fast-track;id=#635 --> Fast-track #635 (🛑💾 NEW P0a — **your own filing**: mid-gate streak counter wipes unpersisted CLEAN passes)
- [ ] <!-- verb=fast-track;id=#342 --> Fast-track #342 (🛑💾 P0a — merge silently deletes a served .factory file · A4 **5-run streak**)
- [ ] <!-- verb=fast-track;id=#365 --> Fast-track #365 (🛑💾 P0a — rebase silently drops production code · A4 **5-run streak**)
- [ ] <!-- verb=fast-track;id=#358 --> Fast-track #358 (🛑💾 P0a — orphan merge; merge-base --is-ancestor assertion · A4 **5-run streak**)
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
- [ ] <!-- verb=fast-track;id=#690 --> Fast-track #690 (quick-win — your own count-parser bug: "E-11 stories" mis-parsed as count)
- [ ] <!-- verb=fast-track;id=#653 --> Fast-track #653 (quick-win — your own per-variant test-name enumeration ask)
- [ ] <!-- verb=fast-track;id=#661 --> Fast-track #661 (quick-win — Red Gate non-compiling middle state)
- [ ] <!-- verb=fast-track;id=#660 --> Fast-track #660 (quick-win — changelog-monotonicity v-prefix normalize)
- [ ] <!-- verb=fast-track;id=#515 --> Fast-track #515 fix 2 (quick-win — the .gitignore half; fix 1 landed in PR #524)
- [ ] <!-- verb=fast-track;id=#469 --> Fast-track #469 (quick-win ⏸ — traceability regex one-liner)

```
# Tier 2 — board-level maintenance requests (read/analyze/regenerate only — B2)
```
- [ ] <!-- verb=reprioritize;id=board --> Reprioritize the whole board (fresh pass over all 477)
- [ ] <!-- verb=full-refresh;id=board --> Full refresh (re-enumerate, re-validate, re-render)
- [ ] <!-- verb=revalidate;id=board --> Re-validate open/closed state of every listed issue
- [ ] <!-- verb=rescore-intent;id=board --> Re-score intent alignment (manifest v0.3) across the corpus

_Boxes are parsed on the next scheduled run (cheap-poll-then-act). Irreversible/public actions
still escalate per B2 — these verbs only read, analyze, and regenerate._
