# 📋 beadle — Triage Dashboard · drbothen/vsdd-factory

<!-- beadle-state:v1
{"schema":1,"last_run":"2026-09-09","watermark":830,"store":"jsonl","intent_version":"vsdd-factory@0.5","warmup":"cold-start","run":18,"digest":"run18-32-new-2-p0b-794-806-365-closed-first-p0a-close-7-fixed-open-verified-0-measured-merges-21-dark-windows-watch-2026-09-09","counts":{"open":510,"arcavenai_open":453,"maintainer_engaged":79,"arcavenai_closed_alltime":20},"attn":{"governance":[510,766],"direction":[410,671],"evidence_brief":[463,686,710]},"axis_model":"finding-005 superset + finding-009: report-type, defect-nature, reproducibility, triage-state, leverage, alignment, provenance, integrity(safety), operational-impact(liveness)","p0_data_loss":[342,358,365,523,588,635],"p0_integrity":[313,314,330,331,332,333,337,339,341,348,355,356,370,372,373,374,379,381,399,404,412,421,425,427,428,430,433,434,437,440,441,448,452,465,468,470,474,475,477,479,481,483,485,486,488,491,492,494,496,517,535,537,538,544,546,547,576,589,592,599,600,614,623,637,638,647,650,663,664,672,673,680,685,692,704,710,724,747,750,756,794,806],"new_this_run":[762,764,765,766,785,788,789,790,791,792,793,794,795,796,797,799,806,809,810,811,812,819,820,821,822,823,825,826,827,828,829,830],"prior_run_burst":[755,756,757,758],"operational_impact":{"halt":[321,326,343,346,347,349,357,380,386,411,414,451,457,458,461,469,472,473,474,516,543,555,601,604,626,637,651,658,681,684,696,702,724,732,826],"panic":[541,548,569,647],"data_loss":[342,365,358,523],"degraded_new":[762,788,793,794,799,809,810,811,812,822,828]},"keystone":336,"keystone_new":[],"keystone_prior":[406,410,413,415,416,419,426,432,462,463,470,487,488,507,513,576,671],"quick_wins_new":[791,812,822],"quick_wins_prior":[436,444,447,450,453,466,469,471,472,476,478,482,495,498,499,501,633,636,641,643,645,653,654,655,656,660,661,666,667,677,678,682,684,690,703,705,707,709,712,732,734,741],"clusters":{"silent-data-loss":[342,358,365,412,479,523,588,635,645],"source-of-truth-integrity":[399,404,412,421,425,427,428,430,433,434,437,440,441,448,452,465,468,470,474,475,477,479,481,483,485,486,488,491,492,494,496,517,535,537,538,544,546,547,589,600,614,623,637,638,647,650,663,664,672,673,680,685,692,704,709,724,747,750,794,806],"false-green-convergence":[305,309,310,322,327,330,331,332,333,337,339,348,353,355,356,360,364,370,373,381,390,391,393,397,398,399,421,425,429,433,434,440,441,442,448,452,462,465,467,468,470,474,475,477,479,480,481,484,485,486,488,490,492,494,496,497,500,513,533,535,543,553,563,576,591,592,599,614,618,620,627,634,643,664,672,673,674,675,676,677,685,686,687,690,694,698,699,700,704,705,710,733,747,748,752,756,765,785,806,820,821,827,830],"operational-halt":[321,326,343,346,347,349,357,380,386,411,414,451,457,458,461,469,472,473,474,503,539,541,548,555,569,604,626,637,642,681,696,724,732,826],"platform-envelope-mismatch":[410,411,412,413,414,415,416,417,458,461,473,474,516,601,614,625,649,651,656,766,788,809,811,829],"test-writer-gaps":[329,335,354,359,363,364,373,381,383,393,394,398,402,418,419,420,422,423,424,426,436,442,460,466,471,475,477,479,480,481,485,490,494,496,498,499,500,502,520,556,571,575,598,607,609,633,653,665,679,682,684,697,699,700,701,702,713,757,790,829],"spec-propagation-drift":[361,362,367,369,376,382,387,388,390,391,392,395,396,400,401,403,404,406,418,419,422,427,428,429,430,432,435,443,444,445,446,447,449,450,453,459,471,476,478,482,484,488,489,491,493,519,521,522,537,550,552,554,559,561,564,565,573,574,580,581,582,594,600,615,622,638,647,653,656,660,668,671,680,681,693,695,708,709,711,749,751,758,762,806,810],"adversary-sweep-enumeration":[504,505,506,507,511,519,520,534,548,563,564,573,574,606,615,633,652,686,733,748,752,753],"verification-methodology":[509,512,517,518,522,525,533,534,535,536,539,540,542,543,544,545,546,549,550,553,555,556,557,559,560,562,566,567,585,586,587,590,593,595,596,599,601,602,603,605,607,608,616,617,618,620,621,627,633,634,641,643,645,648,649,652,654,658,660,661,662,663,664,665,667,668,669,671,675,676,677,678,679,682,687,690,693,694,695,697,698,699,700,701,702,703,704,705,710,713,724,746,748,749,751,752,753,762,765,785,819,820,821,823,825,829],"artifact-containment":[263,341,515,589,597,630,636,650,734,741,789,825],"scratch-isolation":[508,624,701,755,830],"contribution-governance":[510,551,651,696,766],"dispatch-contract":[328,368,386,405,409,448,457,459,463,468,473,482,516,518,521,525,540,545,549,551,558,569,589,619,630,655,658,661,662,666,674,683,696,706,707,756,764,811,827],"authority-substrate":[372,374,379,426,483,508,605,644,825],"observability":[317,318,319,320,324,325,415,463,464,495,501,583,628,629,635,644,648,667,755,791,792,795,797,809,822,827],"plugin-pack-ci":[349,351,357,469,472,476,492,625,694,698,732,789],"orchestrator-continuation":[343,347,380,408,409,446,457,486,495,604,635,647,692,707,755,764,794,796],"branch-protection":[346,348,349,357,408,626,790,826],"dispatch-race":[345,350,355,368,451,547,570,592,594,619,703],"worktree-factory-split":[341,342,451,452,483,503,508,523,588,624,631,655,799,826],"state-manager-hygiene":[517,538,547,572,629,631,641,692,706,711,712,794,795,810,830],"variants-gallery-review":[568],"convergence-tuning":[577,578,579,762,785,820,823],"learning-loop":[584,796,823],"plugin-upgrade-drift":[788,792],"guard-string-matching":[790,793],"hook-matcher-precision":[799,812,822,826,828]},"maintainer_actions":{"comments_prior":{"290":"2026-07-19","298":"2026-07-08","305":"2026-07-19","396":"2026-07-19","418":"2026-07-19","424":"2026-07-19","428":"2026-07-08","434":"2026-07-19","443":"2026-07-15","461":"2026-07-19","465":"2026-07-19","472":"2026-07-19","473":"2026-07-19","475":"2026-07-19","494":"2026-07-19","507":"2026-07-15","515":"2026-07-19","521":"2026-07-15","582":"2026-07-19","649":"2026-07-15","651":"2026-07-15","654":"2026-07-19","655":"2026-07-19","682":"2026-07-19","686":"2026-07-19","724":"2026-07-21"},"comments":{"749":"2026-07-25","681":"2026-07-25","663":"2026-07-25","626":"2026-07-25"},"filings":[762,764,765,785],"merges":{"524":"2026-07-19","526":"2026-07-19","527":"2026-07-19","528":"2026-07-19","529":"2026-07-19","530":"2026-07-19","531":"2026-07-19","532":"2026-07-19","725":"2026-07-21","715":"2026-07-22","716":"2026-07-22","717":"2026-07-22","718":"2026-07-22","719":"2026-07-22","721":"2026-07-22","722":"2026-07-22","723":"2026-07-22","726":"2026-07-22","730":"2026-07-22","731":"2026-07-22","736":"2026-07-22","739":"2026-07-22","728":"2026-07-22","754":"2026-07-22","714":"2026-07-23","720":"2026-07-23","727":"2026-07-23","735":"2026-07-23","737":"2026-07-23","738":"2026-07-23","740":"2026-07-23","759":"2026-07-23","760":"2026-07-24","761":"2026-07-24"},"closes":{"418":"2026-07-19","465":"2026-07-19","472":"2026-07-19","724":"2026-07-21","229":"2026-07-22","243":"2026-07-22","244":"2026-07-22","296":"2026-07-22","300":"2026-07-22","566":"2026-07-22","623":"2026-07-22","658":"2026-07-22","660":"2026-07-22","567":"2026-07-22","629":"2026-07-23","631":"2026-07-23","204":"2026-07-23","365":"2026-07-24"},"corrections":{"691":"corrects #531 (merged PR) — Go/Node stdin diagnosis"},"close_caveats":{"244":"keyword auto-close via #731 squash (5648164) — math-test criterion residual, comment posted"},"p0a_actions":{"342":"2026-07-23 via PR #759 (maintainer-authored S-21.01 WASM staging guard + orchestrator merge pre-check, E-21 W1) — issue open pending epic waves","365":"2026-07-24 CLOSED via PR #760 (S-21.02, maintainer-authored, E-21 W1; rc.24) — first P0a close","358":"2026-07-24 FIXED via PR #761 (S-21.03, maintainer-authored; no auto-close keyword) — verified run-18, close proposed"},"fixed_open_verified":{"358":"PR #761 rc.24","637":"PR #715 rc.24","690":"PR #716 rc.24 + #803 rc.25","580":"PR #714 rc.24 (maintainer-authored issue)","279":"PR #798 rc.25","757":"PR #783 rc.24","641":"PRs #813/#815 unreleased"},"partial_open_verified":{"342":"#759+#775 rc.24, #814 unreleased; nested mount + dual-track detector residual","473":"#527/#717 rc.24; registry pattern + doc row residual","515":"fix1 #524; fix2 == #729","789":"#786 rc.25; staged⊆declared blind spot","794":"#802 wrap, operator-invoked only","827":"#807 event only; fail-open stderr + gate residual","687":"#776 Arm A1/A2 table-row slice only","378":"#776 Class E date monotonicity; version-order rubric residual","648":"#807; 4 on_error=block PostToolUse hooks still exit 2 on epoch timeout"},"measured_prs_open":{"729":"rounds answered 07-23; re-review asked 08-01; 0 actions since","768":"arcaven; opened 08-04; 0 review"}},"instrument":"v2-capacity-adjusted","attention_windows":["2026-07-08","2026-07-15","2026-07-19","2026-07-21","2026-07-22","2026-07-23","2026-07-24","2026-07-25","2026-08-07","2026-08-10","2026-08-13","2026-08-15","2026-08-16","2026-08-17","2026-08-25","2026-08-26","2026-08-27","2026-08-28","2026-08-29","2026-08-30","2026-08-31","2026-09-01","2026-09-03","2026-09-04","2026-09-05","2026-09-06","2026-09-07","2026-09-08","2026-09-09"],"a4_windows":{"523":28,"588":28,"635":26,"358":"discharged (PR #761 reference, fixed)","365":"closed"},"releases":{"v1.0.0-rc.24":"2026-08-25","v1.0.0-rc.25":"2026-09-04"}}
beadle-state -->

**Direction verdict: 🟡 WATCH** (instrument v2, capacity-adjusted) · top signal: **measured channel dark 21 windows** (0 measured-side merges; #729/#768 ready and unreviewed) · this window: **first P0a CLOSE on record (#365) + second P0a fix (#358), both maintainer-authored, both in the FIRST window**

Capacity-conditioned (episodic-side-project, two humans — operator-ratified r2/r3). Prior results
preserved, never re-scored. **48 days since run-17**: the maintainers shipped **32 own PRs, two
releases (rc.24 08-25, rc.25 09-04) and the E-21/S-17/S-24/S-25 waves** across **23 attention
windows**, and touched the measured corpus in exactly two of them.

| At a glance | Result | Detail |
|---|---|---|
| PR acceptance | 🟢 **97%** (30/31) — **stale** | no measured-side decision in 23 windows; #729 + #768 open, unreviewed since 08-01 / 08-04 |
| Engagement cadence | 🟢 **23 windows / 7 weeks** | 60 repo actions, top-percentile for an episodic project — **53 of them on the maintainers' own PRs** |
| Merge latency (measured channel) | 🔴 **≥ 21 windows** | #729 answered both rounds 07-23, re-review asked 08-01, silence since; #768 never reviewed. Band red; headline not — see Direction Health |
| Ramp prediction (aae-orc#65) | 🔴 **FAILED** (under v1 — permanent) | evaluated run-16; no prediction is pending this run (none registered since) |
| Selection depth | 🟢 **DEEP, then dark** | window 1 (07-24): **#365 CLOSED via PR #760** (first P0a close) + **#358 fixed via PR #761** (no keyword, sits open); window 2 (07-25): four sub-case comments (#749 #681 #663 #626); windows 3–23: zero corpus actions |
| Fixed but never closed | **7 verified + 9 partial** | #358 #637 #690 #580 #757 (rc.24) #279 (rc.25) #641 (unreleased) verified FIXED at fff5e4cc — close proposed; #342 #378 #473 #515 #648 #687 #789 #794 #827 partial — sweep table under Maintainer progress |
| Next decisive move | **theirs: re-review #729 · ours: pause filing, ship the SDL residue** | 28 filings vs 7 corpus actions (§6b rule 6) — intake outran drain for the first time since run-14 |

Carried caveat: #244 auto-closed by a stale `Closes` keyword in #731's squash (residual offered, no
reply). Watermark **#758 → #830**: intake **32** (#762–#830; 27 arcavenai, 1 arcaven, 2 Zious11, 2
drbothen) — 2 P0b (#794 #806), 5 P1, 13 P2, 12 P3; 0 SDL, 0 panic, 1 halt (#826), 0 speculative.
_Updated 2026-09-09 · watermark #830 · run 18 · intent vsdd-factory@0.5 · finding-005/009 axes + attn
facet · vocabulary.json contract · skills-based refresh (binary render retired per run-10 regression)_

> **vsdd-factory is a self-referential factory — engine == product.** Engine process-gaps are
> **on-mission**, scored on leverage + provenance. Run-18 intake: **27/32 `advances`, 5 `neutral`,
> 0 `drifts`; 28 pilot-derived + 4 maintainer-authored, 0 speculative** — 22 of 28 measured filings
> landed in the 13 days after rc.24 (release dogfooding). Runs 11–14 readings unchanged.

> **THIS RUN — two P0a actions in one window, then the corpus went quiet.** On 2026-07-24 the
> maintainer closed **#365** with PR #760 and fixed **#358** with PR #761 — both E-21 W1 stories they
> authored, both in rc.24: the ceiling the ramp prediction asked for, one window late; the evaluated
> result stands. Then they built (wasmtime + RUSTSEC, POLICY-15, cross-site correspondence,
> factory-lock, INDETERMINATE) — none of it against a measured issue, none of it wrong. **This run's
> job is reconciliation:** which shipped work closes which open filing (seven do, nine partly), and
> the honest note that the measured side filed 28 issues into that silence while its two PRs sat.

## Baseline (counts — a starting line, first rates unlocked)

| Metric | Value | Outcome pairing |
|---|---|---|
| Open issues | **510** (+31) | arcavenai 453 · Zious11 29 · arcaven 17 · drbothen 10 · slabgorb 1 |
| Filer concentration | arcavenai ~89% | 27 of 32 new are the measured filer; 4 are maintainer-authored (#762 #785 drbothen · #764 #765 Zious11) |
| New this run | **32** (#762–#830 · watermark 758→830) | 2 P0b integrity (#794 #806) · 5 P1 (#788 #790 #809 #820 #826) · 13 P2 · 12 P3 — 27/32 `advances`, 0 speculative |
| Maintainer engagement | **79 actions (+7 this run)** | 2 P0a merges (#760→#365, #761→#358) + 1 close (#365) + 4 field-data comments (07-25) — **all within the first two windows; 21 windows since with zero corpus actions** |
| Closes against tracked defects | **18** (+1: #365 — first P0a) | paired with maintainer-authored PR #760 (S-21.02), shipped rc.24. **Verified fixed but still open: #358 #637 #690 #580 #757** (rc.24) **#279** (rc.25) **#641** (unreleased) — close proposed below |
| filed-vs-acted gap | 453 open : 79 acted | ratio 5.9:1 → **5.7:1** (flat) — but the margin reversed: **28 filed : 7 acted** this window; open count grew for the first time since run-14 |
| Δ since last run (#758→#830) | **+31 open** (+32 new − 1 closed) | intake outran drain 32:1 |
| Releases in window | **rc.24** (08-25) · **rc.25** (09-04) | 32 maintainer PRs merged; 0 measured PRs merged; 2 measured PRs open ≥ 21 windows |
| Maintainer capacity (v2) | **episodic-side-project** — 2 humans, one of hundreds of projects | attention windows observed: **29** (6 prior + 23 this run — Jul 24 · 25 · Aug 7 · 10 · 13 · 15 · 16 · 17 · 25 · 26 · 27 · 28 · 29 · 30 · 31 · Sep 1 · 3 · 4 · 5 · 6 · 7 · 8 · 9) |

## 👤 Needs human reading — direct attention (NEW lane · orthogonal to priority)
> A small group whose **special nature benefits from the maintainer reading them directly and
> in full** — the value is in the prose and cannot survive summarization into chips. Orthogonal
> to priority: an item here can be P2 on impact yet first in your reading queue. Ordered by
> reading priority: **reply-needed** (a counterparty is blocked on your answer) →
> **gates-work** (the decision unblocks other open work) → **standing** (context that improves
> everything downstream). Never quick-win-eligible; never folded into cluster rollups.

| What it is | # | attn · order · why direct reading |
|---|---|---|
| **External contribution flow proposal (ArcavenAE)** — how to route artifact-bearing changes; artifact-content-via-issue backed by a 2000-trial simulation | **#510** | `attn.governance` · **reply-needed** · consent/relationship decision only maintainers can make; ArcavenAE holds artifact-bearing PRs until your ruling (also P2 · **no response after 66 days** — 96 maintainer actions across 29 windows, none here) |
| **Signed vsdd-factory pack heads-up (ArcavenAE)** — a derivative packaging of the plugin was published under the vsdd-factory name via sideshow-packs; the filer lists where it diverges from the plugin form and **offers to rename** on request | **#766** | `attn.governance` · **reply-needed** · naming/consent decision only the maintainers can make; the counterparty holds the rename on your answer (also P3 · neutral · NEW run-18 · arcaven) |
| **Platform-envelope mismatch (tracking)** — the dark-factory requirements profile vs the Claude Code plugin envelope; seven named limits (#411–#417), now generating live gate failures (#458 #461 #473 #474) | **#410** | `attn.direction` · **gates-work** · re-platform / amend-constitution / risk-accept is an architectural fork only you can take; 11 open issues wait on it (also P1-KEYSTONE) |
| **Fleet-mining evidence brief** — 4,602 dispatches across 3 factories, 169 bad-outcome chains hand-classified: 84% of real bad outcomes are NOT model capability; four engine asks derived | **#463** | `attn.evidence-brief` · **standing** · external empirical corpus you cannot re-derive from this repo; weighing adoption of the four asks is a judgment over the full method + numbers (also P1-KEYSTONE) |
| **Factory-graph proposal** — derived traceability graph rehydrated from `.factory/` markdown | **#671** | `attn.direction` · **gates-work** · **your co-maintainer's keystone proposal** — one ruling reframes the traceability lane (also P1-KEYSTONE) |
| **Reviewer-calibration divergence** — severity vs code-freeze state | **#686** | `attn.evidence-brief` · **standing** · third calibration facet, Zious11 field-data already attached — the cross-project evidence lives only here |
| **V-model verification-planning gap** — N-consecutive-clean counts absence-of-findings | **#710** | `attn.evidence-brief` · **standing** · 🛑 also P0b — carries the empirical convergence dataset the repo cannot re-derive |

<details><summary>🤖 agent channel — attn.* facet screens (runs 12–18)</summary>

```
Facet: SKILL.md step 3b (2026-07-06); subtypes grown from exemplars #510 #410 #463; new subtype on a
SECOND instance. Run-12 screen: #508 #507 #464 not tagged. Bar: would the maintainer LOSE something
decision-relevant reading only the chip row? Keep the lane in single digits.
```
```
Runs 13–16 screens (folded — full text in the run-17 fixture): run-13 0 of 104 admitted (#584 #568
#583 nearest); run-14 3 admitted (#671 #686 #710; misses #687 #695 #634; #681 external-signal note
recorded per B3); run-15 0 of 13 (#747 #750 #752 #753 nearest); run-16 0 new — 22 actions landed,
none on this lane; run-17 0 of 4 (#756 #755 #758 nearest) — #342 moved with no measured diff while
the lane sat. Through run-17: read-and-rule sits, read-the-mechanism-and-fix moves.
```

```
Run-18 screen: 32 new issues, ONE admitted (lane 6 → 7, at the bar). Admitted: #766 (governance,
reply-needed) — the arcaven heads-up that a signed pack of vsdd-factory exists under the project's
name with an offer to rename; consent/naming is a relationship decision only the maintainers can take
and a counterparty is waiting (the #510 pattern). Grader-tagged, curator-DECLINED: #823
(grader: attn.direction/standing; curator: an engineering proposal the P2 row carries — the #507
precedent). Nearest misses: #785 #762 (maintainer-authored, author == reader); #821 (a defect with a
cited fix). 21 windows, zero actions on this lane; #510 waits 66 days; #410's cone and the three
evidence briefs untouched. E-21 answered the SDL lane's top rows without visiting a thread.
```

</details>

## Action plan — grouped by INTENT, integrity-first (finding-004 precedence)

Rows lead with **what the issue is**; `#NN` is the agent's fetch reference; trailing chips are
status. `repro` badge feeds the effort estimate; `⏸ halt`/`💥 panic` = operational-impact
(finding-009); `🛑` = source-of-truth integrity (finding-004); `💾` = data-loss. Verdicts cited
against the v0.3 rubric. Board rows verified against live state this run: **one close** (#365, struck) and **seven verified
fixed-but-open** (#358 #637 #690 #580 #757 #279 #641 — marked `FIXED · close proposed`); prior closes
stay struck. The 28-candidate fixed-but-open sweep is tabled under Maintainer progress.

### 🔴 P0a — Silent data-loss (highest severity: irreplaceable state destroyed/stranded with no signal)

> Each is **integrity × data-loss** (or pure SDL): a system-of-record silently diverges from
> reality AND destroys or strands state behind a green check. Sits **above** everything else
> by precedence. **THE LANE TOOK ITS FIRST CLOSE:** #365 — rebase auto-merge silently drops
> production lines — **CLOSED 2026-07-24 via maintainer-authored PR #760 (S-21.02 post-rebase
> diff-integrity gate, E-21 W1), shipped rc.24.** Same window, **#358 was fixed by PR #761
> (S-21.03: explicit `--base`, post-create `baseRefName` assertion, post-merge `merge-base
> --is-ancestor`, bats 7/7 at fff5e4cc) — its body says it closes #358, GitHub saw no keyword, the
> issue sits open.** #342's W1 guard gained the S-21.09 artifact restore (#775, rc.24) and an
> unreleased PostToolUse mirror (#814); the nested `.factory` mount and the dual-track detector the
> issue asked for are still absent — PARTIAL. **A4 residue:** #523/#588 **28/28 windows**, #635
> (maintainer-authored) **26/26**; #358 discharged by the #761 reference; #365 closed. 0 new SDL in
> 32. Discharge path for the residue is still ours — though neither #365 nor #358 had a measured diff.

| What it is | # | Type · repro · verdict |
|---|---|---|
| Product-branch merge silently `rm`s a `.factory` file the nested worktree was serving | #342 | bug · Bohr · **🛑💾 INTEGRITY + DATA-LOSS · ADVANCES · systemic · PARTIAL — W1 guard (PR #759) + S-21.09 restore (#775, rc.24) + PostToolUse mirror (#814, unreleased); residual: nested mount unchanged, no dual-track detector (companion #341 open)** |
| Rebase auto-merge silently drops 4 production lines (no conflict markers, clean `--continue`) | ~~#365~~ | bug · Mandel · **🛑💾 CLOSED ✓ 2026-07-24 via PR #760 (S-21.02, maintainer-authored) — first P0a close** |
| PR base not locked to trunk → orphan merge: `state=MERGED` while `origin/main` lacks the commit | **#358** | bug · Bohr · **🛑💾 INTEGRITY + DATA-LOSS(orphan) · FIXED — PR #761 (S-21.03, rc.24), verified at fff5e4cc · close proposed** |
| Story-worktree `.factory` artifacts silently lost at teardown — worktree removed with unsynced state | **#523** | bug · Bohr · **🛑💾 INTEGRITY + DATA-LOSS · ADVANCES · systemic · A4 28 windows** |
| Factory-side PR strands the shared `.factory` worktree on a chore branch — served state silently wrong for every consumer | **#588** | process-gap · Bohr · **🛑▲ INTEGRITY + SDL(strand) · ADVANCES · systemic · A4 28 windows** |
| CLEAN passes/streak never persisted mid-gate; crash loses position | **#635** | process-gap · Bohr · **🛑💾 DATA-LOSS · ADVANCES · systemic · A4 26 windows (maintainer-authored)** |

### 🔴 P0b — Source-of-truth integrity (gates EVERY functional verdict, incl. convergence)

> A PASS/"converged" verdict computed over a corrupt substrate is **unfalsifiable**
> (finding-004). **Run-18 adds two:** **#806** — `compute-input-hash` has three silent
> false-clean paths (a flag-order mistake exits 1 not 2, `--scan NOINPUT` conflates input-less with
> missing-inputs, upstream-propagation drift is invisible to the per-file edit hook; 38 of 61
> artifacts stale-but-green in the filer's run) — the ratchet's own referent lying green, the #313
> family at the hash layer; and **#794** — a story parked mid-convergence leaves the STATE.md
> checkpoint silently predating the rounds that ran (staleness-by-missing-trigger, not a
> miscomputed record; rc.25's `/vsdd-factory:wrap` records the counter only when an operator
> invokes it — PARTIAL). **#637 is verified FIXED** (PR #715, rc.24; bats 2/2 at fff5e4cc) and sits
> open only for want of a close. Class: 3 closed of 82; #756 #747 #750 carry unchanged (#750's
> contested grade held at P0b per the safety-conservative rule).

| What it is | # | Type · repro · verdict |
|---|---|---|
| compute-input-hash silent false-clean paths — flag order, `--scan NOINPUT`, downstream-propagation drift invisible to the edit hook | **#806** | bug · Bohr · **🛑 INTEGRITY (convergence gate / ratchet referent) · ADVANCES · systemic** (NEW run-18) |
| STATE.md checkpoint silently predates parked convergence rounds — no machine-readable record of where the streak stopped | **#794** | process-gap · Bohr · **🛑 INTEGRITY (.factory/STATE.md) · ADVANCES · systemic · PARTIAL (rc.25 wrap skill, operator-invoked only)** (NEW run-18) |
| Creation-only dispatch ran the full 9-step lifecycle — merge authorized with NO convergence certificate in existence | **#756** | process-gap · Mandel · **🛑 INTEGRITY (convergence gate) · ADVANCES · systemic** (NEW run-17) |
| Post-convergence production edits merge without gate re-entry — seal not bound to merged HEAD | **#747** | process-gap · Bohr · **🛑 INTEGRITY (convergence seal) · ADVANCES · systemic** (NEW) |
| Errata cited as normative authority exist only as changelog rows — unresolvable by construction | **#750** | process-gap · Bohr · **🛑 INTEGRITY (learning) · ADVANCES · systemic** (NEW) |
| Live sprint-state assertions gated every PR on base drift — data + gate both fixed | ~~#724~~ | ci-build · Bohr · **🛑 INTEGRITY (spec_process) · RESOLVED ✓ 2026-07-21** (NEW+CLOSED) |
| $(cat) strips trailing newlines — input-hash false-drift hard-block | **#637** | bug · Bohr · **🛑 INTEGRITY (spec_process) · FIXED — PR #715 (rc.24), verified at fff5e4cc · close proposed** |
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

<details><summary>🤖 agent channel — P0b prior detail (49 issues, runs 9–12) + run-13 reading (folded)</summary>

Prior P0b: **#465 CLOSED (PR #532, the first P0b close)**; rest open. Rows in the run-11/12 fixtures + store; fetch `#NN` (B1): #313 #314 #330 #331 #332 #333 #337 #339 #341 #348 #355 #356 #370 #372 #373 #374 #379 #381 #399 #404 #412 #421 #425 #427 #428 #430 #433 #434 #437 #440 #441 #448 #452 #465 #468 #470 #474 #475 #477 #479 #481 #483 #485 #486 #488 #491 #492 #494 #496. Run-13 reading (folded — run-17 fixture has the text): the gate layer is the dominant corruption target (#599 #614 #600 #592 #517 — "the checkers lie"); #576 + #513 bracket the fix (mechanical verdict derivation); write-protection sub-family #589 #537 #538 #544 #546 #547 ~~#623~~ (closed PR #718). All HARD-EXCLUDED from quick wins.

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
> **Run-18 adds two P1s at the plugin-lifecycle edge:** #788 (an upgrade silently drops the generated
> `hooks/hooks.json` — all ten bindings gone, three projects observed) and #809 (`FUEL_EXHAUSTED`
> tells the operator to raise a field the registry comment calls unimplemented; premise check: the
> dispatcher DOES honor per-entry `fuel_cap` — the comment and bats AC-020 are the stale half).
> #473 re-verified PARTIAL (PRs #527/#717 rc.24; one registry pattern + one doc row remain).

| What it is | # | Type · repro · verdict |
|---|---|---|
| Tracking issue — dark-factory requirements exceed Claude Code plugin envelope | **#410** | tech-debt · n/a · **P1-KEYSTONE · ADVANCES** (prior) |
| No external supervisor — orchestrator and orchestrated share one failure domain | **#411** | tech-debt · Mandel · **⏸ HALT** (prior) |
| CAP-012 loss bound unverifiable · session lifecycle = REPL · no introspection/bulkheads · actor-topology mismatch · resource opacity | **#413 #414 #415 #416 #417** | (prior — see run-10 detail) |
| **Hardware/human-gated holdout criteria need a first-class category + resume-queue** — Phase-4 halts on physical products | **#458** | enhancement · Bohr · **⏸ HALT · ADVANCES · systemic** (NEW) |
| **Auto-mode classifier blocks agent merge — doctrine promises an affordance the substrate denies** (self-flagged scope doubt: factory vs Claude Code ownership) | **#461** | possibly-out-of-scope · Bohr · **⏸ HALT · NEUTRAL · systemic** (NEW) |
| **Scenario-rotation step tasks the write-denied orchestrator with writing a file** — unexecutable as declared | **#473** | bug · Bohr · **⏸ HALT · PARTIAL — both filed defects fixed (PRs #527/#717, rc.24; `cargo test issue_473` 2/2); residual: no registry entry for `holdout-scenarios/evaluations/<run>/`, phase-4 SKILL.md:21 stale** |
| Research-agent declares MCP tools absent at spawn; halts split-dispatch | #516 | bug · Bohr · **ADVANCES · systemic** (NEW) |
| Phase-4 holdout budgets frozen without target-build capability canary | #601 | process-gap · Bohr · **ADVANCES · systemic** (NEW) |
| Validate-pr-review-posted unreachable on single-identity project | #651 | bug · Bohr · **ADVANCES · systemic** (NEW) |
| Generated hooks.json lost on plugin upgrade; all 10 hook bindings silently gone | **#788** | bug · Bohr · **ADVANCES · systemic** (NEW run-18) |
| FUEL_EXHAUSTED message advises unimplemented fuel_cap registry field | **#809** | bug · Bohr · **ADVANCES · systemic** (NEW run-18) |

### 🟠 P1 — Operational: halts the autonomous pipeline (`⏸ impact.halt` · finding-009)

> The liveness axis. **Run-18 adds #826:** `validate-factory-path-staging` resolves the branch in
> `CLAUDE_PROJECT_DIR` for any command without `git -C` and never parses `cd`, so `cd .factory &&
> git add -A` is blocked with the parent's branch (source-confirmed: `lib.rs:607-609`,
> `exec_subprocess.rs:248-250`; the unreleased #814 companion inherits the cwd). No lane closes this
> window (#204 #658 remain its kills). #732 re-verified NOT-FIXED (state-burst :231/:283,
> state-manager.md:322 untouched). #681 (dual-validator deadlock — following
> either shipped contract hard-blocks the agent; external analysis flags it
> strongest-of-batch, see agent channel) remains the natural first pull. Panic #647 rides
> P0b as integrity-primary; run-13's panic trio (#541 #548 #569) carries unchanged.

| What it is | # | Type · repro · verdict |
|---|---|---|
| validate-factory-path-staging resolves branch/pathspecs in the product checkout — blocks legitimate staging inside the `.factory` worktree | **#826** | bug · Bohr · **⏸ HALT · ADVANCES · systemic** (NEW run-18) |
| Lock RENEW still repo-relative in state-burst/state-manager — unreachable in installed-plugin context (completes merged #526; the #472 class) | **#732** | bug · Bohr · **⏸ HALT · ADVANCES · systemic · ★ · re-verified NOT-FIXED at fff5e4cc** (run-15 · arcaven) |
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
| **Probe variance, not pass count, drives convergence quality** — the 3-clean certificate is vacuous on a narrow probe distribution | **#462** | process-gap · Mandel · **KEYSTONE · ADVANCES · systemic** (NEW) |
| **Fleet-mining engine asks** — four asks from 4,602 dispatches (84% of bad outcomes not model capability) | **#463** | proposal · n/a · **KEYSTONE · ADVANCES · systemic** (NEW · arcaven) |
| **Remediation disposition-sweep contract** (7-pass sibling-miss pattern + wrong-value-propagated-with-fidelity) — also P0b above | **#470** | (see P0b) |
| **Smoke sentinel gate: quality-green ≠ operator-green** — nothing executes the artifact at the operator boundary | **#487** | enhancement · Bohr · **KEYSTONE · ADVANCES · systemic** (NEW) |
| **Steady-state enforcement surface** — also P0b above; #489/#491/#493 are its facets | **#488** | (see P0b) |
| POL-003 dispatch tables · impl-adds-API lint · defense-in-depth input provenance | #406 #419 #426 | (prior keystones — see run-10 detail) |
| Adversary CLEAN verdict can contradict non-empty findings list | **#576** | process-gap · Mandel · **🛑 INTEGRITY (spec_process) · ADVANCES · systemic · KEYSTONE — pairs with #513: mechanical verdict derivation from both sides** (NEW) |
| Factory-graph: derived traceability graph from .factory/ markdown | **#671** | proposal · n/a · **ADVANCES · systemic · KEYSTONE — maintainer-authored; 👤 attn.direction; one ruling reframes the traceability lane** (NEW) |

### 🟠 P1 — Convergence soundness (methodology + propagation)

> The methodology wave matures from cataloging failures to specifying the missing
> verification steps: quantity-assertions on drain loops (#697), mutation-verified
> security invariants (#676), full-suite semantics (#677), per-state-class fix sweeps
> (#679), pipeline-authored gate independence (#698). Read #710 (👤) as the
> consolidation anchor — it carries the empirical dataset for WHY absence-of-findings
> convergence under-measures. **Run-18 adds #820 (P1):** the only machine-checked
> predicates are a round count and CRIT/HIGH — MEDIUMs never break the streak, coverage is never
> checked, monotonicity penalizes the fixing round. Methodology P2s ride the Run-18 section
> (#821 #829 #825).

| What it is | # | Type · repro · verdict |
|---|---|---|
| Convergence streak ignores MEDIUMs, checks no coverage, and monotonicity penalizes fixing it | **#820** | bug · Bohr · **ADVANCES · systemic** (NEW run-18) |
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

### 🟢 P1/P2/P3 — Run-18 findings (NEW · 13 P2 rows + 12 P3 by cluster; P0b/P1 rows ride their lanes above)

> Two themes. **Hook-matcher precision** (new cluster #799 #812 #822 #826 #828 + guard-string-matching
> #790 #793): validators matching on command *text* or whole-file scope block legitimate work or
> fail open (#790 `verify-git-push`, the intake's fifth P1, rides here — no thematic P1 lane fits).
> **Plugin-lifecycle drift** (new cluster #788 #792 + #789): what an upgrade silently drops or
> leaves stale. Maintainer-authored #762 #764 #765 #785 grade `advances`. Premise checks for the
> filer: #793's repro needs `-f`; #796 partly false (S-7.02 already requires the deferral); #799's
> second comment misreads `-C`.

| What it is | # | Type · repro · verdict |
|---|---|---|
| verify-git-push fails open: lease substring and flag-before-remote bypass | **#790** | bug · Bohr · **ADVANCES · systemic** (NEW run-18) |
| Records-tier treadmill: line-cite ban, mechanical records-lint, micro-burst | **#762** | enhancement · Mandel · **ADVANCES · systemic** (NEW run-18 · drbothen) |
| pr-manager completion guard pressures step-9 merge claim before merge exists | **#764** | process-gap · Bohr · **ADVANCES · systemic** (NEW run-18 · Zious11) |
| Behavioral-completeness scan as mandatory BC verification gate | **#785** | enhancement · Bohr · **ADVANCES · systemic** (NEW run-18 · drbothen) |
| validate-factory-path-staging blocks quoted text naming git add .factory/ | **#799** | bug · Bohr · **ADVANCES · systemic** (NEW run-18) |
| STATE.md declared as hashed input manufactures drift on every write | **#810** | bug · Bohr · **ADVANCES · systemic** (NEW run-18) |
| PostToolUse block reports failure but the write already landed | **#811** | bug · Bohr · **ADVANCES · systemic** (NEW run-18) |
| Adversarial verification is reading not running; non-compiling ruling survived nine rounds | **#821** | process-gap · Bohr · **ADVANCES · systemic** (NEW run-18) |
| validate-table-cell-count is file-scoped: one bad row blocks all later edits | **#822** | bug · Bohr · **ADVANCES · minutiae · ★** (NEW run-18) |
| Lessons Learned review rules have no trigger, artifact, or hook | **#823** | process-gap · Bohr · **ADVANCES · systemic** (NEW run-18) |
| Adversary declared read-only can write files but cannot execute | **#825** | bug · unknown · **ADVANCES · systemic** (NEW run-18) |
| Fuel exhaustion silently indistinguishable from validated for structural gates | **#827** | bug · Bohr · **ADVANCES · systemic** (NEW run-18) |
| Go panicking stubs abort test binary; Red Gate 2 unobservable | **#829** | process-gap · Bohr · **ADVANCES · systemic** (NEW run-18) |
| regression-gate records unscoped /tmp command as project's passing regression state | **#830** | bug · Bohr · **ADVANCES · systemic** (NEW run-18) |

| P3 cluster (count) — representative | Issues |
|---|---|
| **observability** (3) — e.g. factory-obs list reports [ok] for empty .factory/logs; ingests nothing silently | #791 ★ #795 #797 |
| **verification-methodology** (2) — e.g. Holdout report template lacks evaluability/data-availability caveat section | #765 #819 |
| **hook-matcher-precision** (2) — e.g. destructive-command-guard sot_delete blocks any rm plus SoT filename | #812 ★ #828 |
| **contribution-governance** (1) — e.g. Heads-up: signed sideshow pack of vsdd-factory and its divergence register | #766 |
| **plugin-pack-ci** (1) — e.g. rc.24 bundle ships policy15-attestation-gate.wasm unreferenced by any registry | #789 |
| **plugin-upgrade-drift** (1) — e.g. factory-obs up leaves stack bound to previous plugin version's compose config | #792 |
| **guard-string-matching** (1) — e.g. destructive-command-guard blocks heredoc writes that merely mention git clean | #793 |
| **learning-loop** (1) — e.g. S-7.02 cycle-close NEXT pointer outlives filed process-gaps | #796 |

### 🟢 P2/P3 — Run-17 findings (carried — folded)

#755 (probe hygiene vs stall recovery · process-gap · Mandel · P2) · #757 (async-hook sink e2e
flake · flaky-test · P3 — **verified FIXED**: rc.24's #783 shipped the load-tolerant timeout on an
event-polled wait the issue asked for; close proposed) · #758 (companion BC for the
story-CHANGELOG task · task · P2). #756 rides P0b. Full rows in the run-17 fixture.

### 🟢 P2 — Run-15 findings (carried — folded)

9 P2s + P1 #732 (halt lane above, re-verified NOT-FIXED). Two adversary-axis proposals (#752
boundary-sentinel inputs, #753 channel-lifecycle asymmetry), the narrated-verification class
(#746 — root cause corrected in-thread), version-qualifier drift (#749 — **re-verified NOT-FIXED:**
rc.24's cross-site validator Arm A2 gates BC-table cites only, the filed prose cite stays
unguarded), reverse symbol→anchor coverage (#751), coherence-lens gap (#748), #733 (arcaven,
adversary compile-state clause). Repo-hygiene pair **#734 #741 re-verified NOT-FIXED** (17
`/Users/jmagady` sites and the mode-160000 gitlink both present at fff5e4cc, both in rc.24 and
rc.25) — still ★ quick-win-eligible. Full rows in the run-15 fixture.

### 🟢 P2 — Run-14 process/policy/test-writer findings (advances, no integrity, no halt)

36 run-14 P2s by primary cluster (rows in the run-14 fixture; per-issue chips in the index): verification-methodology #641 #643 #648 #649 #652 #654 #661 #667 #668 #669 #678 #687 #695 #705 #713 · spec-propagation-drift #653 #656 ~~#660~~✓ #708 · dispatch-contract #655 #666 #683 #707 · test-writer-gaps #665 #682 #697 #701 · state-manager-hygiene #711 #712 · artifact-containment #636 · operational-halt #642 · authority-substrate #644 · adversary-sweep-enumeration #686 · false-green-convergence ~~#690~~ FIXED · plugin-pack-ci #698 · dispatch-race #703. Sweep this run: **#641 FIXED** (word-boundary `D-\d+` guard #813 + structured Decisions-Log scan #815, unreleased) · #648 #687 PARTIAL · #649 NOT-FIXED.

### 🟢 P2 — Prior clusters (unchanged this run)

Run-12 P2 detail (24 rows incl. the #504→#507 sweep chain + #435 #442 #443 #459 #480 #484
#490) carried in the run-12 render (docs/fixtures) + indexes below. Older tail unchanged —
abbreviated to protect the body budget: dispatch/test/spec tail (#368 #369 #371 #375 #376
#377 #378 #328 #329 #334 #335 #338 #344 #345 #350 #351 #352 #353 #354 #359 #360 #361 #362
#363 #364 #366 #367 #382 #383 #387 #388 #392 #393 #394 #395 #398 #400 #405 #407 #417 #420
#422 #423 #424), observability thread (#317–#325, root cause #415), CI-cost cluster (#308,
read WITH #336), trust-scope reminder (#316).

## 🟦 Quick wins — safe to act on (low-caution lane · orthogonal to impact)

> **No conversions this window** — the lane's six kills (#472 #660 #566 #567 #629 #631) stand;
> #690 (your own count-parser bug, fix merged run-16) is now **verified FIXED** at fff5e4cc (PR #716
> rc.24, re-implemented by #803 rc.25; live repro exits 0) and needs only a close. Run-18 admits
> **3 new** (#791 #812 #822 — all mechanical, Bohr, cited fix, bounded). Eligibility is **derived**
> (mechanical + bounded blast + cited fix + alignment ≠ drifts). **Lane exclusion is RULE (SKILL
> §7a):** integrity anchors (#794 #806 this run), SDL, and panic never ride here regardless of
> mechanical eligibility. On-ramp unchanged and re-verified still open: **#732** (two missed sites
> of your own #526 conversion), then **#741** (two-command gitlink removal), **#734** (17 path
> sites + a `/Users/` guard).

| What it is | # | Type · repro · priority |
|---|---|---|
| factory-obs list reports [ok] for empty .factory/logs; ingests nothing silently | **#791** | bug · Bohr · **P3** (NEW run-18) |
| destructive-command-guard sot_delete blocks any rm plus SoT filename | **#812** | bug · Bohr · **P3** (NEW run-18) |
| validate-table-cell-count is file-scoped: one bad row blocks all later edits | **#822** | bug · Bohr · **P2** (NEW run-18) |
| Lock RENEW repo-relative in state-burst/state-manager — apply #526's own conversion to 2 missed sites | **#732** | bug · Bohr · **⏸ P1** (run-15 · arcaven · re-verified open) |
| Author-absolute /Users/jmagady paths in shipped examples — replace + /Users/-literal content guard | **#734** | bug · Bohr · **P2** (run-15 · re-verified open) |
| .lazyclaude gitlink: `git rm --cached` + .gitignore + mode-160000 health check (covers #263 too) | **#741** | bug · Bohr · **P2** (run-15 · re-verified open) |
| Run-14 lane (21 admitted, run-14 fixture has the rows): #633 #636 ~~#641~~ FIXED #643 #645 #653 #654 #655 #656 ~~#660~~ #661 #666 #667 #677 #678 #682 #684 ⏸ ~~#690~~ FIXED #703 #705 #707 #709 #712 | | (#660 CLOSED ✓ · **#690 verified FIXED, close proposed** · rest open) |
| Prior lane (run-13: #533 #539 #545 #549 #557 #558 ~~#566~~ ~~#567~~ #580 #581 #582 #583 #591 #593 #594 #597 #598 #622 #628 ~~#629~~ #630 ~~#631~~ · run-12: #515 (partial fix landed, PR #524) · run-11: #436 #444 #447 #450 #453 #466 #469 #471 #476 #478 #482 #495 #498 #499 #501 · older: #396 #397 #401 #402 #403 #389 #407 #408 #409 #352 #323 #320 #209 #239 #256 #280 #282) | | (**#472 #418 #566 #567 #629 #631 CLOSED ✓** — #567/#629/#631 this window via PRs #728/#720/#727 · rest open) |

<details><summary>🤖 agent channel — quick-win §7a rule check + exclusion audit (runs 14–18)</summary>

```
§7a RULE CHECK (run-18):
- 3 admitted of 32 graded: #791 (factory-obs existence-only [ok] — one-function predicate fix),
  #812 (destructive-command-guard sot_delete arm — clause-bound regex, bats exists), #822
  (validate-table-cell-count — drop `head -1`, report all rows; the diff-scoping half is larger).
- integrity HARD-EXCLUDE (2): #794 #806 — grader-set integrity=true, neither grader-admitted.
- SDL / panic rule-exclude: none this run (0 SDL, 0 panic in the intake).
- attn never-QW: #766 (admitted to the lane) — not grader-admitted.
- Curator overrides: ONE, on the attn facet not the lane — #823 grader-tagged attn.direction,
  curator declined (see attn screen); lane decisions agree 32/32.
- Lane outcome: 0 conversions in 23 windows — the first window since run-13 with no lane movement.
  #690 verified fixed (close pending) is the lane's seventh kill in substance if not in state.
```
```
Runs 14–17 rule checks (folded — full text in the run-17 fixture): run-14 21 admitted / 13
integrity-excluded / SDL #635 / panic #647 / attn #671 #686 #710 excluded, zero overrides; run-15
3 admitted (#732 #734 #741), #746 refuted-fix note, zero overrides; run-16 no intake; run-17 0 of
4 admitted (#755 design, #757 Mandel, #758 spec), #756 integrity-excluded, zero overrides.
```

</details>

## Direction Health — corpus-level (the led-by-the-backlog check)

Computed under **instrument v2 (capacity-adjusted, SKILL §6b — operator-ratified 2026-07-22)**.
Verdict: **🟡 WATCH** · top signal: **measured channel dark 21 windows**. **v2 red gate, applied
honestly:** (a) one band IS red — measured-channel merge latency (#729 ready 08-01, #768 opened
08-04, 21 silent windows each); (b) the strict cost test wants *SDL-residue fix PRs* sitting ≥ 2
windows and the measured side shipped none (#729 is the P0a-adjacent leak scan) — though the weak
form ("diffs move") is contradicted: two diff-attached PRs sat while 32 maintainer PRs merged. One
of two → 🟡, not 🔴. **Falsification meter (aae-orc#65):** the run-14 ramp prediction stays
**FAILED** (evaluated run-16 under v1 — permanent). **Hard acted-on ceiling: P0a CLOSE** (#365 via
PR #760) + the P0a fix of #358 — both maintainer-authored, 2026-07-24, one window after run-17.
Soft ceiling (prose): P1/P2 issue-thread comments 07-25 (#681 #749 #663 #626); since then prose
landed only on their own PRs. **No prediction registered or pending** — the operator's move (§6b
rule 5: window-denominated, 4–6 windows).

| Signal (v2) | Verdict | Reading |
|---|---|---|
| `pr_acceptance` band | **GREEN (stale)** | 30/31 = **97%, zero rejections** — unchanged: no measured-side decision in 23 windows; #729 + #768 open |
| `engagement_cadence` band | **GREEN** | **23 windows in 7 weeks** (episodic band: ≥ 1/month) — 60 repo actions, 53 on the maintainers' own PRs, 7 on the corpus |
| `merge_latency` band (measured channel) | **RED** | #729: both review rounds answered 07-23, re-review asked 08-01, **21 windows** with no response; #768 (arcaven, CI action pinning): 21 windows, never reviewed. Prior windows merged ready PRs same-day |
| `maintainer_action` (the compass) | **deep, then absent** | +7 (2 P0a merges · 1 close · 4 comments), all in windows 1–2; cumulative 79. Pointed straight at the SDL lane, then left the corpus |
| `selection depth` | **DEEP — P0a close** | ceiling **P0a CLOSED** (#365) + P0a fixed (#358); cost covariate **0% diff-attached** (maintainer-authored epic stories). The cost hypothesis is contradicted both ways: no-diff moved, diff-attached sat |
| `A4 silent-data-loss zero-engagement` (finding-016) | **residue drifting (signal-level; headline-gated per §6b)** | **#365 CLOSED, #358 DISCHARGED** (#761 reference). Residue: #523/#588 **28/28 windows**; #635 **26/26**. Discharge path: measured-side SDL fix PRs — none shipped |
| `integrity-density` (B) | watch (2 new) | #794 #806 (2 of 32 intake, 6%); class 82 members, 3 closed, **#637 verified fixed awaiting close**; open P0b net +2 |
| `silent-data-loss share` (C) | on-course | 0 new in 32; lane 6 → 1 closed + 1 fixed-open + 1 partial + 3 untouched |
| `filing_density` | **WATCH — self-directed (§6b rule 6)** | **28 measured filings (22 in the 13 days after rc.24) against 7 corpus actions**; open measured count +27. Rule 6: do not grow the count while acted-share is low — the intake did. Mitigation: all pilot-derived release dogfooding, PR channel at 2 of the 10-cap; but volume is the AI-DDoS mechanism regardless of quality |
| `filed_vs_acted_gap` | flat ratio, reversed margin | 453:79 = **5.7:1** (12.9 → 12.1 → 7.4 → 5.9 → 5.7) — the ratio held only because the denominator grew by 7; the margin this window is 28 filed : 7 acted |
| `convert_finding_to_guard_unbounded` · `over_engineering_fingerprints` | clean | 32 filings screened — no unbounded guard conversions, no invented machinery; 28/28 measured filings cite an observed run, file, or hook source |

- **Reconciliation, not drift:** E-21 (Factory State Data-Loss Hardening) IS the P0a lane by another
  route — #365/#358 were its W1 stories. Reading the dark windows as disengagement repeats the run-14
  error; reading them as corpus engagement would be false. The maintainers work their own epic
  backlog; the corpus moves when a story coincides. **Cheapest lever on both sides:** seven verified
  fixes (#358 #637 #690 #580 #757 #279 #641) need only a close — each verifiable in under a minute at fff5e4cc.

## Classification index (finding-005 + finding-009 + attn facet)

### Run-18 index (NEW)

report-type · defect-nature · repro · alignment · flags. 32 records ingested (vocabulary.json
contract; rich fixture vsdd-factory-312-run18-classifications-rich.json). Graded against the pinned
upstream checkout fff5e4cc (rc.25 + 10) — live source, not a stale tree; premise corrections recorded
for #793 #796 #799 #806 #809 #828. Follow `#NN` for bodies (B1).

| What it is | # | chips |
|---|---|---|
| Records-tier treadmill: line-cite ban, mechanical records-lint, micro-burst | #762 | enhancement · design · Mandel · adv *(drbothen)* |
| pr-manager completion guard pressures step-9 merge claim before merge exists | #764 | process-gap · design · Bohr · adv *(Zious11)* |
| Holdout report template lacks evaluability/data-availability caveat section | #765 | bug · spec · Bohr · adv *(Zious11)* |
| Heads-up: signed sideshow pack of vsdd-factory and its divergence register | #766 | policy · design · unknown · neu · **👤** *(arcaven)* |
| Behavioral-completeness scan as mandatory BC verification gate | #785 | enhancement · design · Bohr · adv *(drbothen)* |
| Generated hooks.json lost on plugin upgrade; all 10 hook bindings silently gone | #788 | bug · lifecycle · Bohr · adv |
| rc.24 bundle ships policy15-attestation-gate.wasm unreferenced by any registry | #789 | ci-build · logic · Bohr · adv |
| verify-git-push fails open: lease substring and flag-before-remote bypass | #790 | bug · logic · Bohr · adv |
| factory-obs list reports [ok] for empty .factory/logs; ingests nothing silently | #791 | bug · logic · Bohr · adv · **★** |
| factory-obs up leaves stack bound to previous plugin version's compose config | #792 | bug · lifecycle · Bohr · adv |
| destructive-command-guard blocks heredoc writes that merely mention git clean | #793 | bug · logic · Bohr · adv |
| STATE.md checkpoint silently predates parked convergence rounds | **#794** | process-gap · spec · Bohr · adv · **🛑** |
| Holdout score in STATE.md goes stale silently after deliveries | #795 | enhancement · spec · Bohr · neu |
| S-7.02 cycle-close NEXT pointer outlives filed process-gaps | #796 | process-gap · spec · Bohr · neu |
| Session-end markers fire per Stop turn, not per session | #797 | bug · spec · Bohr · adv |
| validate-factory-path-staging blocks quoted text naming git add .factory/ | #799 | bug · logic · Bohr · adv |
| compute-input-hash silent false-clean paths: flag order, NOINPUT, downstream drift | **#806** | bug · logic · Bohr · adv · **🛑** |
| FUEL_EXHAUSTED message advises unimplemented fuel_cap registry field | #809 | bug · design · Bohr · adv |
| STATE.md declared as hashed input manufactures drift on every write | #810 | bug · logic · Bohr · adv |
| PostToolUse block reports failure but the write already landed | #811 | bug · design · Bohr · adv |
| destructive-command-guard sot_delete blocks any rm plus SoT filename | #812 | bug · logic · Bohr · adv · **★** |
| No artifact field hashes its own bytes; freeze tuples miss edits | #819 | enhancement · spec · Bohr · neu |
| Convergence streak ignores MEDIUMs, checks no coverage, monotonicity penalizes coverage | #820 | bug · spec · Bohr · adv |
| Adversarial verification is reading not running; non-compiling ruling survived nine rounds | #821 | process-gap · spec · Bohr · adv |
| validate-table-cell-count is file-scoped: one bad row blocks all later edits | #822 | bug · logic · Bohr · adv · **★** |
| Lessons Learned review rules have no trigger, artifact, or hook | #823 | process-gap · design · Bohr · adv · **👤** |
| Adversary declared read-only can write files but cannot execute | #825 | bug · design · unknown · adv |
| validate-factory-path-staging resolves branch in product checkout, blocks .factory staging | #826 | bug · logic · Bohr · adv · **⏸** |
| Fuel exhaustion silently indistinguishable from validated for structural gates | #827 | bug · design · Bohr · adv |
| validate-template-compliance blocks backlog stubs lacking full story template sections | #828 | enhancement · spec · Bohr · neu |
| Go panicking stubs abort test binary; Red Gate 2 unobservable | #829 | process-gap · spec · Bohr · adv |
| regression-gate records unscoped /tmp command as project's passing regression state | #830 | bug · logic · Bohr · adv |

### Run-17 index (carried forward — folded)

4 issues, chips unchanged — rows in the run-17 fixture + store: #755 process-gap·design·Mandel ·
**#756** 🛑 process-gap·design·Mandel · #757 flaky-test·concurrency·Mandel · #758 task·spec·Bohr.
Run-16: zero new issues.

### Run-15 index (carried forward — folded)

13 issues (#724–#753), chips unchanged — full per-issue rows in the run-15/16/17 fixtures + store
(rich fixture vsdd-factory-312-run15-classifications-rich.json): ~~#724~~ ✓ #732 ⏸★ #733 #734 ★
#741 ★ #746 **#747** 🛑 #748 #749 **#750** 🛑 #751 #752 #753. Six rows carry in-thread arcaven
fact-checks (read before acting on #746 and #741).

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
**This window: the deepest single action on record (a P0a close) and then the longest silence.**

| Outcome signal | Count | State |
|---|---|---|
| **Defects closed with a validated fix** | **18** | **+1: #365 (PR #760, S-21.02, maintainer-authored, rc.24) — the first P0a close.** Prior 17: #204 #567 #629 #631 · #418 #472 #465 #724 · #623 #658 #660 #566 #300 #296 #243 #229 #244 (⚠ keyword auto-close) |
| **Fixed and shipped, never closed — verified this run** | **7** | **#358** (PR #761 rc.24 — body says closes #358, no keyword) · **#637** (🛑 P0b — PR #715 rc.24) · **#690** (PR #716 rc.24 + #803 rc.25) · **#580** (Zious11's own — PR #714 rc.24) · **#757** (PR #783 rc.24) · **#279** (PR #798 rc.25) · **#641** (#813/#815, unreleased). Verified at fff5e4cc; close proposed |
| **P0a lane** | **1 closed / 1 fixed-open / 1 partial / 3 untouched** | #365 ✓ · #358 fixed · #342 partial (nested mount + dual-track detector residual) · #523 #588 #635 untouched (A4 28/28/26 windows) |
| **Partial fixes landed** | **12** | sweep: #342 #378 #473 #515 #648 #687 #789 #794 #827 (residuals below) · prior: #434 (PR #529) · #424 (PR #531 + #691) · #475 (PR #530) |
| **arcavenai PRs merged** | **30 / 31** (+0) | no measured merge in 23 windows · **#729** both rounds answered 07-23, re-review asked 08-01, silence · arcaven **#768** unreviewed since 08-04 · channel 2 of the 10-cap |
| **Correction/review cycle** | **corrections owed by US** | four of our claims wrong at the source: #473 comment (registry has had `{placeholder}` patterns since 2c97cb00), #799 comment (`-C` is target-aware, not evaded), #793 body (repro needs `-f`), #809 body (dispatcher DOES honor `fuel_cap`). Body/thread corrections, gated before posting |
| Field-data comment-events on measured issues | **27** (+4) | Zious11's wave-84/85 sub-case notes on #749 #681 #663 #626 (07-25) — the only issue-thread prose this window (run-13 trio #298 #428 #507 pattern holds); other prose landed on their own PRs |
| Silent data-loss (P0a) closed with validated fix | **1 / 6** | **#365 ✓ (first)** · #358 fixed awaiting close · residue #523 #588 #635. Ramp prediction stays FAILED (evaluated run-16, permanent) |
| Source-of-truth integrity (P0b) closed | **3 / 82** | #465 ✓ #724 ✓ #623 ✓ · #637 verified fixed awaiting close · 78 open incl. NEW #794 #806 |
| Halt/panic (P1 operational) closed + validated | **3 / 41** | #472 ✓ #724 ✓ #658 ✓ (+ #204 strand) · NEW halt #826 · panic 4 open (#541 #548 #569 #647) · #473 partial |
| Envelope decision (#410) taken | 0 / 1 | 13+ issues wait on it (👤 attn.direction) — untouched across 29 windows |
| **Needs-human-reading items answered** | **0 / 7** | #510 (reply-needed — **66 days**) · NEW #766 (reply-needed) · #410 · #463 · #671 #686 #710 — zero of 60 actions touched the lane |
| Keystone consolidations accepted + wired | 0 / 17 | 16 prior + #671 |
| Quick wins exercised | **6 / 82** | #472 #660 #566 #567 #629 #631 ✓ · #690 fixed (close pending) · 3 new eligible (#791 #812 #822) · on-ramp #732 #741 #734 re-verified open |
| **Measured-side hygiene** (not maintainer credit) | this run | fixed-but-open sweep: **28 candidates verified at the pinned SHA — 7 FIXED, 9 PARTIAL, 12 NOT-FIXED** (table below); 32-issue intake graded with premise checks; four self-corrections queued |

<details><summary>🤖 agent channel — fixed-but-open sweep (verified at upstream/develop fff5e4cc, 2026-09-07; rc.24 = 2026-08-25, rc.25 = 2026-09-04)</summary>

Every verdict came from reading the cited files or running the named test/repro at the pinned SHA, never from PR prose. Full evidence (file:line, commands, comment drafts) in the run-18 fixture's sweep JSON.

| # | verdict · released in | recommended action |
|---|---|---|
| #279 | **FIXED** · rc.25 | **close** — hook-authored stamping supersedes the agent's value; verified by unit test `cargo test … |
| #342 | **PARTIAL** · rc.24 | keep-open — residual = (1) `.factory` worktree is still mounted nested (`git worktree add .factory … |
| #358 | **FIXED** · rc.24 | **close** — comment cites merge ebf9fb6d + `bats … |
| #378 | **PARTIAL** · rc.24 | keep-open — the issue asks for a consistency-validator rubric check that `modified:` entries are in … |
| #407 | **NOT-FIXED** · rc.25 | keep-open — no shipped mechanism or template note decides whether INDEX changelogs must record … |
| #473 | **PARTIAL** · rc.24 | keep-open — residual = (1) no artifact-path-registry entry for … |
| #515 | **PARTIAL** · rc.24 | keep-open — residual is fix 2, the content-based (frontmatter/document_type) scan of tracked+staged … |
| #580 | **FIXED** · rc.24 | **close** — the exact proposal shipped in PR #714 (merge aa594c9a) across template + story-writer + … |
| #635 | **NOT-FIXED** · unreleased | keep-open — the wave-gate adversarial loop (wave-gate/SKILL.md:100 -> … |
| #637 | **FIXED** · rc.24 | **close** — comment cites merge e628b884 + `bats --filter '#637' … |
| #641 | **FIXED** · unreleased | **close** — the two named false-positive substrings are closed on Side A by the word-boundary guard … |
| #648 | **PARTIAL** · rc.25 | keep-open — for the four PostToolUse `on_error = "block"` Edit/Write hooks … |
| #649 | **NOT-FIXED** · rc.25 | keep-open — a sync `on_error="block"` epoch timeout still yields exit 2 with reason `fail-closed: … |
| #687 | **PARTIAL** · rc.24 | keep-open — Arm A1/A2 mechanically gate one narrow slice of nit-class 4 (stale BC-version tokens in … |
| #690 | **FIXED** · rc.24 | **close** — comment cites merges 7025537c (#716, rc.24) and 8b4b60e6 (#803, rc.25 — same strip … |
| #711 | **NOT-FIXED** · none | keep-open — no tracked-obligation primitive or pre-delivery stale-marker scan landed; candidate PRs … |
| #712 | **NOT-FIXED** · none | keep-open — no index-row status-cell budget, health check, or compaction op exists; #805/#803 … |
| #732 | **NOT-FIXED** · none | keep-open — the two state-burst bash-block sites (renew at :231, CAS push at :283) and the … |
| #734 | **NOT-FIXED** · none | keep-open — zero of the 17 shipped-prose sites changed since filing; no `/Users/` content-guard … |
| #741 | **NOT-FIXED** · none | keep-open — gitlink still tracked at HEAD; neither proposed fix landed (no `git rm --cached`, no … |
| #749 | **NOT-FIXED** · none | keep-open — Arm A2 gates story BC-table version cites only; the filed prose/architecture-section … |
| #757 | **FIXED** · rc.24 | **close** — the issue's second suggested direction (load-tolerant timeout on an event-polled wait) … |
| #789 | **PARTIAL** · rc.25 | keep-open — the named orphan is gone (rc.25) and the issue's 'is it CI-only?' question is answered … |
| #794 | **PARTIAL** · rc.25 | keep-open — /vsdd-factory:wrap (rc.25) gives an operator a way to record the parked state including … |
| #799 | **NOT-FIXED** · none | keep-open — false positive reproduces from source at fff5e4cc (whitespace-token `git`+`add` scan … |
| #809 | **NOT-FIXED** · none | keep-open — the message still names fuel_cap and the registry still documents it as unimplemented, … |
| #826 | **NOT-FIXED** · none | keep-open — the guard resolves the branch in CLAUDE_PROJECT_DIR (product checkout) for any command … |
| #827 | **PARTIAL** · rc.25 | keep-open — ask 2 is met for every plugin via the durable `plugin.indeterminate` JSONL event … |

</details>

_Cold-start lifted for engagement metrics (the wheel completed verified turns); rate/streak
claims accrue against the run-14 baseline. Still no per-human ranking — the metric moves only
when a real defect gets a real, validated fix._

## Controls

Derived from authoritative state each render — never hand-authored. The next run parses
`- [x] <!-- verb=...;id=... -->`, dispatches, then resets the box (eventually consistent).

```
# Tier 1 — per-issue verbs (bounded to one artifact)
```
- [ ] <!-- verb=investigate;id=#729 --> Re-review #729 (measured-side PR: both rounds answered 07-23, re-review asked 08-01, 21 windows waiting)
- [ ] <!-- verb=accept-deferral;id=#358 --> Close #358 (🛑💾 P0a — fixed by your own PR #761, shipped rc.24; verified at fff5e4cc)
- [ ] <!-- verb=accept-deferral;id=#637 --> Close #637 (🛑 P0b — fixed by PR #715, rc.24; bats 2/2)
- [ ] <!-- verb=accept-deferral;id=#580 --> Close #580 (your own — shipped in PR #714, rc.24; bats 16/16)
- [ ] <!-- verb=accept-deferral;id=#690 --> Close #690 (your own count-parser bug — PR #716 rc.24, re-implemented #803 rc.25)
- [ ] <!-- verb=accept-deferral;id=#757 --> Close #757 (flake — event-polled wait with load-tolerant timeout shipped in #783, rc.24)
- [ ] <!-- verb=accept-deferral;id=#279 --> Close #279 (STATE.md timestamp now hook-stamped — #798, rc.25)
- [ ] <!-- verb=investigate;id=#510 --> Read & rule on #510 (👤 reply-needed — 66 days; external contribution flow; counterparty holding PRs)
- [ ] <!-- verb=investigate;id=#766 --> Read & rule on #766 (👤 reply-needed — a signed pack exists under the project's name; rename offered)
- [ ] <!-- verb=investigate;id=#806 --> Investigate #806 (🛑 NEW P0b — compute-input-hash false-clean paths; 38/61 artifacts stale-but-green)
- [ ] <!-- verb=investigate;id=#794 --> Investigate #794 (🛑 NEW P0b — STATE.md checkpoint predates parked rounds; wrap covers the invoked case only)
- [ ] <!-- verb=fast-track;id=#788 --> Fast-track #788 (P1 — plugin upgrade silently drops all 10 hook bindings; three projects observed)
- [ ] <!-- verb=investigate;id=#790 --> Investigate #790 (P1 — verify-git-push fails open on a lease substring or flag-before-remote)
- [ ] <!-- verb=fast-track;id=#826 --> Fast-track #826 (⏸ P1 — staging guard resolves the branch in the product checkout; blocks `.factory` commits)
- [ ] <!-- verb=investigate;id=#820 --> Investigate #820 (P1 — convergence streak ignores MEDIUMs, never checks coverage)
- [ ] <!-- verb=investigate;id=#809 --> Investigate #809 (P1 — FUEL_EXHAUSTED message vs registry comment; the dispatcher already honors `fuel_cap`)
- [ ] <!-- verb=fast-track;id=#635 --> Fast-track #635 (🛑💾 P0a — your own filing: mid-gate streak counter wipes CLEAN passes; A4 26/26 windows)
- [ ] <!-- verb=investigate;id=#342 --> Scope #342's residual (🛑💾 P0a — W1 + S-21.09 shipped; nested mount + dual-track detector open; #814 unreleased)
- [ ] <!-- verb=fast-track;id=#523 --> Fast-track #523 (🛑💾 P0a — story-worktree .factory artifacts lost at teardown; A4 28/28 windows)
- [ ] <!-- verb=fast-track;id=#588 --> Fast-track #588 (🛑▲ P0a — factory-side PR strands the shared .factory worktree; A4 28/28 windows)
- [ ] <!-- verb=investigate;id=#671 --> Rule on #671 (👤 KEYSTONE — your co-maintainer's factory-graph proposal; gates the traceability lane)
- [ ] <!-- verb=investigate;id=#681 --> Deep-investigate #681 (⏸ P1 — dual-validator deadlock; you annotated it 07-25 as PG-W84-003)
- [ ] <!-- verb=investigate;id=#647 --> Deep-investigate #647 (🛑💥 P0b+panic — mid-edit burst death leaves a corrupted continuation baseline)
- [ ] <!-- verb=investigate;id=#710 --> Read #710 (👤 evidence-brief + 🛑 P0b — the convergence under-measurement dataset; #820 is its mechanism)
- [ ] <!-- verb=investigate;id=#410 --> Deep-investigate #410 (KEYSTONE tracker — envelope mismatch; #788 #809 join its cone)
- [ ] <!-- verb=fast-track;id=#732 --> Fast-track #732 (quick-win ⏸ P1 — your own #526 conversion, two missed sites; re-verified open)
- [ ] <!-- verb=fast-track;id=#741 --> Fast-track #741 (quick-win — gitlink still mode-160000 at HEAD; two-command removal, covers #263)
- [ ] <!-- verb=fast-track;id=#734 --> Fast-track #734 (quick-win — 17 `/Users/jmagady` sites still shipped)
- [ ] <!-- verb=fast-track;id=#791 --> Fast-track #791 (NEW quick-win — factory-obs `[ok]` on an empty logs dir; one-function predicate)
- [ ] <!-- verb=fast-track;id=#812 --> Fast-track #812 (NEW quick-win — sot_delete arm: clause-bound regex)
- [ ] <!-- verb=fast-track;id=#822 --> Fast-track #822 (NEW quick-win — table-cell-count: report all rows, not `head -1`)
- [ ] <!-- verb=fast-track;id=#515 --> Fast-track #515 fix 2 (quick-win — the content-scan half is #729's scope; fix 1 landed in PR #524)

```
# Tier 2 — board-level maintenance requests (read/analyze/regenerate only — B2)
```
- [ ] <!-- verb=reprioritize;id=board --> Reprioritize the whole board (fresh pass over the full open corpus, 510)
- [ ] <!-- verb=full-refresh;id=board --> Full refresh (re-enumerate, re-validate, re-render)
- [ ] <!-- verb=revalidate;id=board --> Re-validate open/closed state of every listed issue
- [ ] <!-- verb=rescore-intent;id=board --> Re-score intent alignment (manifest v0.5) across the corpus

_Boxes are parsed on the next scheduled run (cheap-poll-then-act). Irreversible/public actions
still escalate per B2 — these verbs only read, analyze, and regenerate._
