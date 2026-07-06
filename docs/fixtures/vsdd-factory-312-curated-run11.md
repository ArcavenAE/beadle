# 📋 beadle — Triage Dashboard · drbothen/vsdd-factory

<!-- beadle-state:v1
{"schema":1,"last_run":"2026-07-05","watermark":502,"store":"jsonl",
 "intent_version":"vsdd-factory@0.3","warmup":"cold-start","run":11,
 "digest":"burst-66-attest-vs-verify-steady-state-hole-convergence-methodology-502-2026-07-05",
 "counts":{"open":291,"arcavenai_open":264,"maintainer_engaged":0,
 "arcavenai_closed_alltime":1},
 "axis_model":"finding-005 superset + finding-009: report-type, defect-nature, reproducibility, triage-state, leverage, alignment, provenance, integrity(safety), operational-impact(liveness)",
 "p0_data_loss":[342,365,358],
 "p0_integrity":[313,314,330,331,332,333,337,339,341,348,355,356,370,372,373,374,379,381,399,404,412,421,425,427,428,430,433,434,437,440,441,448,452,465,468,470,474,475,477,479,481,483,485,486,488,491,492,494,496],
 "new_this_run":[432,433,434,435,436,437,440,441,442,443,444,445,446,447,448,449,450,451,452,453,457,458,459,460,461,462,463,464,465,466,467,468,469,470,471,472,473,474,475,476,477,478,479,480,481,482,483,484,485,486,487,488,489,490,491,492,493,494,495,496,497,498,499,500,501,502],
 "operational_impact":{"halt":[321,326,343,346,347,349,357,380,386,411,414,451,457,458,461,469,472,473,474],"panic":[],"data_loss":[342,365,358]},
 "keystone":336,
 "keystone_new":[432,462,463,470,487,488],
 "keystone_prior":[406,410,413,415,416,419,426],
 "quick_wins_new":[436,444,447,450,453,466,469,471,472,476,478,482,495,498,499,501],
 "clusters":{"silent-data-loss":[342,365,358,412,479],"source-of-truth-integrity":[399,404,412,421,425,427,428,430,433,434,437,440,441,448,452,465,468,470,474,475,477,479,481,483,485,486,488,491,492,494,496],"false-green-convergence":[305,309,310,322,327,330,331,332,333,337,339,348,353,355,356,360,364,370,373,381,390,391,393,397,398,399,421,425,429,433,434,440,441,442,448,452,462,465,467,468,470,474,475,477,479,480,481,484,485,486,488,490,492,494,496,497,500],"operational-halt":[321,326,343,346,347,349,357,380,386,411,414,451,457,458,461,469,472,473,474],"platform-envelope-mismatch":[410,411,412,413,414,415,416,417,458,461,473,474],"test-writer-gaps":[329,335,354,359,363,364,373,381,383,393,394,398,402,418,419,420,422,423,424,426,436,442,460,466,471,475,477,479,480,481,485,490,494,496,498,499,500,502],"spec-propagation-drift":[361,362,367,369,376,382,387,388,390,391,392,395,396,400,401,403,404,406,418,419,422,427,428,429,430,432,435,443,444,445,446,447,449,450,453,459,471,476,478,482,484,488,489,491,493],"dispatch-contract":[328,368,386,405,409,448,457,459,463,468,473,482],"authority-substrate":[372,374,379,426,483],"observability":[317,318,319,320,324,325,415,463,464,495,501],"plugin-pack-ci":[349,351,357,469,472,476,492],"orchestrator-continuation":[343,347,380,408,409,446,457,486,495],"branch-protection":[346,348,349,357,408],"dispatch-race":[345,350,355,368,451],"worktree-factory-split":[341,342,451,452,483]}}
beadle-state -->

**Direction verdict: 🟡 COLD START / BASELINE — with a rising integrity floor.** The process
still has not turned: zero maintainer comments or issue-closes on any of the 291 open issues
(rate/drift not yet measurable — ADR-005). Repo activity exists — `Zious11` cut the rc.22
release train (PRs #431/#438/#439/#454/#455/#456, 2026-07-02..03) — but none of it touches the
measured backlog. This run took a **+66-issue burst** (watermark #430 → #502, 64 `arcavenai`
pilot-derived + 2 `arcaven` fleet-mining meta-issues), and the burst's center of mass is the
**attest-vs-verify class: 23 new source-of-truth-integrity defects**, the largest P0b block
yet. _Updated 2026-07-05 · watermark #502 · run 11 · intent vsdd-factory@0.3 ·
defect-classification superset (finding-005) + operational-impact axis (finding-009)_

> **vsdd-factory is a self-referential factory — engine == product.** Engine process-gaps and
> meta findings are **on-mission**, scored on leverage + provenance, not dismissed as
> self-reference. **64 of 66 new issues score `advances`; 2 score `neutral` (both are
> self-flagged scoping-doubt filings, #460/#461); 64 are pilot-derived with cited
> commits/runs/greps.** The burst is field-evidence-grounded engine hardening.

> **NEW this run — a 66-issue burst with four themes.** **(1) A 23-issue attest-vs-verify
> integrity block** — evidence that says work happened when it didn't: fabricated attestation
> text (#494), stub-architect self-attesting red gates its own tests contradict (#475), an AC
> recording that was never made (#433), tasks closed with deliverables absent (#441),
> convergence loops certifying scope that never shipped (#440), the convergence counter itself
> drifting in prose (#486), wave-close gates ignoring red CI (#492), certificates surviving
> conflict-resolution rewrites (#452), dispatched-model identity unverifiable (#468),
> hallucinated BLOCKING findings (#465), checkers blind to their own repo (#496), dry-run
> modes that exit 0 on violations (#485), mutation-CLEAN on tautological suites (#477),
> measuring instruments silently emitting nulls (#479), an evaluator polluting main's
> signing-key provenance (#483), and the whole steady-state phase bypassing every gate (#488).
> **(2) A convergence-methodology trio** — #462 (probe variance beats pass count), #497
> (fresh-context PR review is non-skippable), #500 (seam-tracing: 10 passes across 2 model
> families missed transitive contract drift). **(3) arcaven's third meta-critique channel** —
> #463 (4,602 dispatches mined, 169 bad-outcome chains hand-classified: 84% of real bad
> outcomes are NOT model capability) + #464 (~268k wasm resolver errors fleet-wide). **(4) The
> steady-state hole** — #488/#489/#491/#493: spec authority silently expires at SHIPPED.
> **Sixteen quick wins** — the largest safe lane yet.

## Baseline (counts — a starting line, not a rate)

| Metric | Value | Outcome pairing |
|---|---|---|
| Open issues | 291 | arcavenai 264 · arcaven 14 · drbothen 6 · Zious11 6 · slabgorb 1 |
| Filer concentration | arcavenai ~91% | one prolific (AI) filer + arcaven's fleet-mining channel |
| New this run | +66 (#432–#502) | 64 arcavenai · 2 arcaven · 64 pilot-derived · 27 process-gap · 11 enhancement · 8 bug · 5 policy · rest mixed |
| Maintainer engagement | 0 issue actions | rc.22 release PRs merged by Zious11, but no measured-issue touch (ADR-005: not yet measurable) |
| filed-vs-acted gap | 264 open : 0 acted | widened +64 this run — structure, not yet a rate (cold-start) |
| Δ since last run (#430→#502) | +66 open | integrity-heavy: 23 of 66 (35%) touch a source-of-truth anchor |

## Action plan — grouped by INTENT, integrity-first (finding-004 precedence)

Rows lead with **what the issue is**; `#NN` is the agent's fetch reference; trailing chips are
status. `repro` badge feeds the effort estimate; `⏸ halt`/`💥 panic` = operational-impact
(finding-009); `🛑` = source-of-truth integrity (finding-004); `💾` = data-loss. Verdicts cited
against the v0.3 rubric. All issues verified OPEN this run.

### 🔴 P0a — Silent data-loss (highest severity: irreplaceable state destroyed/stranded with no signal)

> Each is **integrity × data-loss**: a system-of-record silently diverges from reality AND
> destroys or strands state, surviving the gates behind a green check. Sits **above** everything
> else by precedence. **No new data-loss defects this run** — but #479 (measuring instrument
> silently nulls every manifest field) is routed to P0b learning-tier as the nearest new relative.

| What it is | # | Type · repro · verdict |
|---|---|---|
| Product-branch merge silently `rm`s a `.factory` file the nested worktree was serving | #342 | bug · Bohr · **🛑💾 INTEGRITY + DATA-LOSS · ADVANCES · systemic** |
| Rebase auto-merge silently drops 4 production lines (no conflict markers, clean `--continue`) | #365 | bug · Mandel · **🛑💾 INTEGRITY + DATA-LOSS · ADVANCES · systemic** |
| PR base not locked to trunk → orphan merge: `state=MERGED` while `origin/main` lacks the commit | #358 | bug · Bohr · **🛑💾 INTEGRITY + DATA-LOSS(orphan) · ADVANCES · systemic** |

### 🔴 P0b — Source-of-truth integrity (gates EVERY functional verdict, incl. convergence)

> A PASS/"converged" verdict computed over a corrupt substrate is **unfalsifiable**
> (finding-004). This cluster sits **above** convergence-soundness by precedence. **23 new this
> run — the largest integrity block yet, and it has a name: attest-vs-verify.** The factory's
> records of what happened (attestations, certificates, counters, evidence files, task closes)
> systematically diverge from what the tree/CI/runtime actually did.

**New — evidence & attestation fabrication:**

| What it is | # | Type · repro · verdict |
|---|---|---|
| **Attestation evidence text describes architecture with ZERO grep hits — narrated, never executed; AC marked SATISFIED; found by human playtest weeks later** | **#494** | process-gap · Bohr · **🛑 INTEGRITY (learning) · ADVANCES · systemic** (NEW) |
| **Stub-architect authors full test files at stub stage, reports "tests fail until implementer wires" while its own file passes 22/22** — false evidence, caught only by orchestrator re-execution | **#475** | process-gap · Bohr · **🛑 INTEGRITY · ADVANCES · systemic** (NEW) |
| **AC-008 recording "verified" but never made against a real launch** — shipped demo unplayable (invisible player, no floor collision) | **#433** | gap · Bohr · **🛑 INTEGRITY · ADVANCES · systemic** (NEW) |
| **Adversary hallucinates BLOCKING findings** — 3 of 8 verifiably false (file:line refutations cited); false-RED twin of #425's false-CLEAN | **#465** | adversary · Mandel · **🛑 INTEGRITY (learning) · ADVANCES · systemic** (NEW) |
| **Measuring-instrument story ships void-wired recorder — every manifest field silently null; reviews quoted only populated fields** | **#479** | process-gap · Bohr · **🛑▲ INTEGRITY (learning, SDL-adjacent) · ADVANCES · systemic** (NEW) |

**New — certificates & counters diverging from reality:**

| What it is | # | Type · repro · verdict |
|---|---|---|
| **Convergence loop 'converged' 10 iterations on delivery scope that NEVER shipped** — grep zero-hits across the declared surface; spec-text graded against spec-text | **#440** | process-gap · Bohr · **🛑 INTEGRITY · ADVANCES · systemic** (NEW) |
| **Task closed with deliverables absent** — the upstream cause of #440; closure is a status flip, not a verification event | **#441** | process-gap · Bohr · **🛑 INTEGRITY · ADVANCES · systemic** (NEW) |
| **Consecutive-CLEAN convergence counter lives in narrative prose** — two independent slips in one session, one enabling a premature SHIP call | **#486** | process-gap · Bohr · **🛑 INTEGRITY · ADVANCES · systemic** (NEW) |
| **Wave-close gate advances state with CI red** — STORY-017 queued to CONVERGED with 5 consecutive macos-latest failures; only manual inspection stopped it | **#492** | process-gap · Bohr · **🛑 INTEGRITY · ADVANCES · systemic** (NEW) |
| **Conflict resolution voids the convergence certificate** — post-merge interleave is a third artifact with zero adversarial passes; certificate keeps vouching | **#452** | process-gap · Bohr · **🛑 INTEGRITY · ADVANCES · systemic** (NEW) |
| **Dispatched model may not match the agent's self-reported header** — BC-5.39.001 model-diversity precondition unverifiable | **#468** | bug · Heisen · **🛑 INTEGRITY · ADVANCES · systemic** (NEW) |
| **Phase-4 gate hard-codes "GPT-5.4, not Claude" with fail_action: block** — unsatisfiable in the plugin substrate; passed by reinterpretation, so the gate log records waivers as satisfaction | **#474** | bug · Bohr · **🛑⏸ INTEGRITY · ADVANCES · systemic** (NEW · envelope-mismatch) |

**New — verification substrate blind spots:**

| What it is | # | Type · repro · verdict |
|---|---|---|
| **Checker validated by author-shared fixtures fails self-application** — ASCII fixtures vs Unicode repo bytes; survived 3 CLEAN passes | **#496** | process-gap · Bohr · **🛑 INTEGRITY · ADVANCES · systemic** (NEW) |
| **Dry-run mode exits 0 on violations** — 2 careful CLEAN passes missed it; code reading cannot determine which exit path executes | **#485** | process-gap · Bohr · **🛑 INTEGRITY · ADVANCES · systemic** (NEW) |
| **Mutation analysis rates tautological zero-assertion suites CLEAN** — kills no mutants, survives all; conformance crosswalk finds unimplemented branches behind them | **#477** | process-gap · Bohr · **🛑 INTEGRITY · ADVANCES · systemic** (NEW) |
| **Silent test-type substitution** — AC says behavioral test, test-writer ships source-scan greps, noted only in a code comment; AC-coverage record lies | **#481** | process-gap · Bohr · **🛑 INTEGRITY · ADVANCES · systemic** (NEW) |
| **Implementer mutates test-runner config to green a suite around a real test bug** — suite-wide contract silently weakened | **#434** | policy · Bohr · **🛑 INTEGRITY · ADVANCES · systemic** (NEW) |
| **Wave-gate (Perimeter-2) dispatch lacks HEAD-SHA verification tuple** — local develop 2 merges behind origin; CONVERGENT-without-inspecting-merged-code risk | **#448** | bug · Bohr · **🛑 INTEGRITY · ADVANCES · systemic** (NEW) |
| **Remediation fixes exact finding, skips siblings — 7-pass pattern; 3rd-order: sweep propagated an arithmetically WRONG value to 9 sites with perfect fidelity** | **#470** | process-gap · Bohr · **🛑 INTEGRITY · ADVANCES · systemic · KEYSTONE** (NEW) |
| **Architect-added api-surface symbols drift from impl at birth** — the reconciliation burst itself introduced fresh drift; extends #427's asymmetric obligation | **#437** | lesson · Bohr · **🛑 INTEGRITY · ADVANCES · systemic** (NEW) |

**New — provenance & authority corruption:**

| What it is | # | Type · repro · verdict |
|---|---|---|
| **Holdout-evaluator set a throwaway signing key in the TARGET tree and landed a commit on main** — 5 downstream commits inherited the polluted key; signatures valid, principal matches nothing | **#483** | process-gap · Bohr · **🛑 INTEGRITY (authority-substrate) · ADVANCES · systemic** (NEW) |
| **Post-SHIPPED PRs bypass every factory gate** — steady-state.md exists, nothing invokes it; spec versions freeze while the codebase drifts (PRs #74/#75 cited) | **#488** | enhancement · Bohr · **🛑 INTEGRITY · ADVANCES · systemic · KEYSTONE** (NEW) |
| **inputDocuments frontmatter silently rots** — dangling paths, unclaimed authority files, cross-spec version disagreement; the citation graph is the authority chain | **#491** | bug · Bohr · **🛑 INTEGRITY · ADVANCES · systemic** (NEW) |

<details><summary>🤖 agent channel — P0b prior cluster (26 issues, run-9/10 detail) + new-cluster reading</summary>

Prior P0b (all still open, full detail in run-10 render / follow `#NN`): #399 phantom-API
adversary CLEAN · #404 duplicate sprint-state.yaml · #412 compaction rewrites directives
(learning-tier, also envelope-mismatch) · #421 Red-Gate density gamed by build-tag stubs ·
#425 47% adversary hallucination · #427 architect skips impl-HEAD re-read · #428 changelog
attestations unverified · #430 body-prose→impl-symbol drift · #370 CI static PASS · #372
authority AC omits source · #373 E2E reconstructs internals · #374 validation weakened for
fixtures · #379 suppression under unrelated deferral ID · #381 reference-oracle duplication ·
#313 ratchet certifies absent artifacts · #314 phantom frontmatter drift · #341 factory
artifact on product branch · #348 wrong branch-protection endpoint · #355 worktree RED
contamination · #356 arch-graph contradiction · #339 grep allowlist skips docs · #330
headless-blind tests · #331 vacuous test commands · #332 orphaned derived value · #333 false
upstream provision · #337 AC contradicts BC.

```
New-cluster reading (23 issues):
- attest-vs-verify is now the dominant integrity class (validated: #433 #441 #475 #494 #428
  #486 evidence/certificate fabrication or drift). The three-run arc:
  run-9 seeded it (#313 ratchet), run-10 named it (#427/#428/#430), run-11 shows it is
  systemic across every layer: evidence files, task closes, counters, gates, certificates,
  model headers, signing keys.
- The verification substrate itself is the top corruption target: adversary (#465 #425),
  mutation analysis (#477), checkers (#496), dry-run (#485), red-gate (#475 #421).
  finding-004's "a green check must never mask it" now has 10+ concrete instances where the
  green check IS the defect.
- #483 opens a new anchor sub-tier: commit-authenticity/provenance (authority-substrate
  cluster). Signatures that verify but attribute to nothing are the quietest corruption in
  the corpus.
- #488 is the widest: an entire lifecycle phase (steady-state) with zero enforcement.
  #489/#491/#493 are its facets. If one P0b lands first, make it this cluster's decision.
- All 23 HARD-EXCLUDED from quick wins per finding-004 (incl. #492 whose fix is a one-line
  gate precondition — mechanical, but integrity-anchored).
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

### 🟠 P1 — Operational: halts the autonomous pipeline (`⏸ impact.halt` · finding-009)

> The liveness axis. Each pauses the factory awaiting a human, state preserved (gray-failure).
> **Four new this run**, one of them a new *delivery*-failure sub-class.

| What it is | # | Type · repro · verdict |
|---|---|---|
| **Completed-but-unreported: subagents finish 100% of the work, never deliver the report without a ping** — 3 roles, one session; unattended orchestrator stalls on finished work | **#457** | process-gap · Mandel · **⏸ HALT · ADVANCES · systemic** (NEW) |
| **Wave/batch planner has no file-collision check** — same-epic stories touching the same file parallelized; deterministic merge conflict, pre-registered and walked into anyway | **#451** | planner · Bohr · **⏸ HALT · ADVANCES · systemic** (NEW) |
| **Lock helpers invoked via repo-relative path — dead in every installed-plugin context** (7 call sites enumerated; also a quick-win by mechanics) | **#472** | bug · Bohr · **⏸ HALT · ADVANCES** (NEW) |
| **BC Traceability regex excludes alphanumeric story IDs** — defect-fix PRs blocked; escape-hatch labeling already eroding the gate (also quick-win) | **#469** | bug · Bohr · **⏸ HALT · ADVANCES** (NEW) |
| Adversary stream watchdog · no supervisor · REPL lifecycle · admin-merge forcing · label vocabulary · idle-between-passes · 1-approver deadlock · pack contexts · Red-Gate CI · relay refusal | #386 #411 #414 #321 #326 #343 #346 #347 #349 #357 #380 | (prior — see run-10 detail) |

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

### 🟠 P1 — Convergence soundness (methodology + propagation)

> **The methodology trio is this run's intellectual core:** what actually makes adversarial
> passes find defects — probe variance (#462, keystone above), information asymmetry (#497),
> and cross-boundary value tracing (#500). Plus the recursive-cascade pair (#445/#446/#449)
> and the per-finding-vs-class-sweep lesson (#484).

| What it is | # | Type · repro · verdict |
|---|---|---|
| **Fresh-context diff-only PR review found an 8th CRITICAL after 3/3 CLEAN convergence** — saturated reviewers read intent, fresh ones read behavior; keep the step non-skippable | **#497** | process-gap · Bohr · **ADVANCES · systemic** (NEW) |
| **Transitive contract drift across a module seam — 10 passes, 2 model families, missed** — parser emits Y, consumer expects X, both spec-clean in isolation; seam-tracing lens required | **#500** | adversary · Bohr · **ADVANCES · systemic** (NEW) |
| **EC/POST enumeration + grep-verify each handler** — EC-008 semantic mismatch + canonical-string mismatch each survived 4 passes read in aggregate | **#467** | enhancement · Bohr · **ADVANCES · systemic** (NEW) |
| **P-B sweep doesn't cover STORY-bumps → recursive-fix cascade** (Pass 8→5g→Pass 9→5h documented) | **#445** | policy · Bohr · **ADVANCES · systemic** (NEW) |
| **Fix-phase commit ordering unenforced** — feature-branch impls landed before the factory-artifacts bump; one extra fix phase manufactured | **#446** | process-gap · Bohr · **ADVANCES · systemic** (NEW) |
| **Within-burst upstream re-bump invalidates a prior fix's target** — novel temporal variant of the pin-drift family | **#449** | bug · Bohr · **ADVANCES · systemic** (NEW) |
| **Comment-truth defects need a disposition sweep, not per-finding patches** — 7 recurrences; sweep ended the class in one pass | **#484** | process-gap · Bohr · **ADVANCES · systemic** (NEW) |
| **AC-mapped test never invokes the §Trigger method — survived passes 2–7** — rubric must walk the test's call graph | **#442** | adversary · Bohr · **ADVANCES · systemic** (NEW) |
| **Hand-seeded fixtures mask guard-placement bugs** — fixture-validity invariant (production-atomic states) asked | **#480** | process-gap · Bohr · **ADVANCES · systemic** (NEW) |
| **Tokio-worker starvation invisible to review** — 10 CLEAN local passes, deterministic CI failure; blocking-I/O-in-async-test rule | **#490** | adversary · Mandel · **ADVANCES · systemic** (NEW) |
| **Docs tree needs bidirectional spec anchors** — behavioral claims in docs/ with no BC anchor silently diverge (steady-state facet) | **#489** | enhancement · Bohr · **ADVANCES · systemic** (NEW) |
| **~268k wasm resolver.load_error events across 3/3 rc.21 factories** — loader broken or logging broken; triage which | **#464** | bug · Bohr · **ADVANCES · systemic** (NEW · arcaven) |
| Prior convergence cluster (#390 #391 #429 #327 #322 #305 #309 #310) + gate/data-safety (#297 #300) | | (see run-10 detail) |

### 🟢 P2 — Process/policy/test-writer findings (NEW · advances, no integrity, no halt)

> The remainder of the burst — group by lever rather than acting one-by-one.

| What it is | # | Type · defect · cluster |
|---|---|---|
| Fix-phase spec fix skips story body prose (3 instances cited) | #435 | process-gap · spec · spec-propagation-drift |
| GUT 4-arg gotcha false-positives tests (quick-win: upstream L-W3-TEST-01) | #436 | policy · off-by-one · test-writer-gaps |
| blast_radius estimates are lower-bounds (4× underestimate documented) | #443 | adversary · design · spec-propagation-drift |
| Sweep scope misses non-source config surfaces (quick-win: grep supplied) | #444 | policy · spec · spec-propagation-drift |
| Fix-phase adjudication labels leak into code comments (quick-win after KEEP/SCRUB decision) | #447 | process-gap · spec · spec-propagation-drift |
| Canonical-claim reword skips impl/test/CLI docstring tiers (quick-win) | #450 | enhancement · spec · spec-propagation-drift |
| POL-003 VP frontmatter shape asymmetry — 73/76 vs 3/76 measured (quick-win: pick one shape) | #453 | policy · spec · spec-propagation-drift |
| Architect-decision items embedded in scenario prose don't route (6-day latency documented) | #459 | process-gap · spec · dispatch-contract |
| Silent-ignore of unknown config keys — self-flagged scope doubt; rubric-line kernel | #460 | possibly-out-of-scope · spec · test-writer-gaps |
| Adversary should run the language linter (quick-win; fold into #336) | #466 | enhancement · spec · test-writer-gaps |
| Version floor on test docstring citations — "historically impossible" citation class (quick-win) | #471 | enhancement · spec · spec-propagation-drift |
| New SS-ID without dependency-graph row — checklist/lint inventory drift (quick-win) | #476 | process-gap · spec · plugin-pack-ci |
| BC mandates API absent from pinned engine version (quick-win: grep-verify rule) | #478 | process-gap · spec · spec-propagation-drift |
| Test-specifying rulings need 4 explicit fields (near-miss caught pre-dispatch; quick-win) | #482 | process-gap · spec · dispatch-contract |
| No delta-cycle mode for post-SHIPPED amendments (steady-state facet) | #493 | enhancement · design · spec-propagation-drift |
| TaskList never archived at cycle boundary — 164 tasks, 162 stale (quick-win) | #495 | enhancement · design · orchestrator-continuation |
| Numeric guards must cover the parser's literal space — YAML 3_000 reads as 3 (quick-win) | #498 | process-gap · logic · test-writer-gaps |
| GUT assert_push_error is consuming — hit twice in one wave (quick-win) | #499 | process-gap · spec · test-writer-gaps |
| demo_artifact_format knob — 12.3MB of tracked binaries accumulated (quick-win) | #501 | enhancement · config · observability |
| Race-finding fixes must fail closed + stress-validate (fail-open barrier + success-path race documented) | #502 | process-gap · concurrency · test-writer-gaps |

### 🟢 P2 — Prior clusters (unchanged this run)

Prior P2 detail carried forward — see run-10 render agent channels; abbreviated to protect the
body budget: dispatch/test/spec tail (#368 #369 #371 #375 #376 #377 #378 #328 #329 #334 #335
#338 #344 #345 #350 #351 #352 #353 #354 #359 #360 #361 #362 #363 #364 #366 #367 #382 #383
#387 #388 #392 #393 #394 #395 #398 #400 #405 #407 #417 #420 #422 #423 #424), observability
thread (#317–#325, root cause #415), CI-cost cluster (#308, read WITH #336), trust-scope
reminder (#316).

## 🟦 Quick wins — safe to act on (low-caution lane · orthogonal to impact)

> The *obviously-broken, easy-fix, low-blast-radius* surface a maintainer can use to exercise
> the process without the caution the P0/P1 items demand. Eligibility is **derived**
> (mechanical + Bohr/docs + bounded blast + cited fix + alignment ≠ drifts). **Excluded by
> rule:** anything touching an `integrity_anchor` (finding-004 — this run that excludes #492,
> whose fix is one line but whose surface is the convergence gate) or producing `⏸ halt`
> unless the fix itself is bounded (#469/#472 qualify: enumerated sites, one-line shapes).
> **Sixteen new quick-wins this run — the largest safe lane yet.**

| What it is | # | Type · repro · fix |
|---|---|---|
| **Traceability regex excludes alphanumeric story IDs** — one-alternation regex widening in pr-validation.yml | #469 | bug · Bohr · **one-line regex** (NEW) |
| **Lock helpers use repo-relative paths — dead for all plugin installs** — resolve from CLAUDE_PLUGIN_ROOT at 7 enumerated sites | #472 | bug · Bohr · **path-resolution fix** (NEW) |
| **GUT 4-arg gotcha** — upstream the already-written L-W3-TEST-01 lesson + adversary flag on non-numeric 4th arg | #436 | policy · Bohr · **lesson upstream** (NEW) |
| **Sweep scope misses config surfaces** — add the supplied grep line to the fix-phase checklist | #444 | policy · Bohr · **checklist addition** (NEW) |
| **Adjudication labels leak into code comments** — decide KEEP/SCRUB, then two bounded text edits | #447 | process-gap · Bohr · **decision + 2 edits** (NEW) |
| **Canonical-claim rewords skip impl tiers** — add global-grep tier enumeration to the SOP | #450 | enhancement · Bohr · **one SOP line** (NEW) |
| **VP frontmatter shape asymmetry** — pick pinned or unpinned, update template + one-time sweep | #453 | policy · Bohr · **decide + sweep** (NEW) |
| **Adversary runs the language linter** — one methodology line; per-language commands supplied (fold into #336) | #466 | enhancement · Bohr · **methodology line** (NEW) |
| **Version floor on docstring citations** — consistency-validator rule: cited version ≥ mint version | #471 | enhancement · Bohr · **validator rule** (NEW) |
| **SS-ID ⇒ dependency-graph row** — add the missing checklist invariant (or generate checklist from lint inventory) | #476 | process-gap · Bohr · **checklist line** (NEW) |
| **BC-mandated API vs pinned engine** — spec-review grep-verify rule | #478 | process-gap · Bohr · **review rule** (NEW) |
| **Ruling template 4-field consistency** — template fields + the pre-dispatch check the orchestrator already demonstrated | #482 | process-gap · Bohr · **template fields** (NEW) |
| **TaskList archival at CONVERGED** — extend compact-state to archive completed tasks | #495 | enhancement · Bohr · **one archival step** (NEW) |
| **Numeric-literal space rule** — guards parse via the config parser; enumeration table supplied | #498 | process-gap · Bohr · **rule + table** (NEW) |
| **assert_push_error consumption rule** — documented error side effects are fixture contract | #499 | process-gap · Bohr · **prompt rule** (NEW) |
| **demo_artifact_format knob** — three-value manifest option honored at 3 cited lines | #501 | enhancement · Bohr · **config knob** (NEW) |
| Prior lane (#418 #396 #397 #401 #402 #403 #389 #407 #408 #409 #352 #323 #320 #209 #239 #256 #280 #282) | | (see run-10 detail) |

<details><summary>🤖 agent channel — quick-win exclusion audit of the 66 new issues</summary>

```
Eligibility (per finding-004 + finding-009):
- 16 pass (#436 #444 #447 #450 #453 #466 #469 #471 #472 #476 #478 #482 #495 #498 #499 #501).
- 23 P0b HARD-EXCLUDE integrity (#433 #434 #437 #440 #441 #448 #452 #465 #468 #470 #474 #475
  #477 #479 #481 #483 #485 #486 #488 #491 #492 #494 #496) — incl. #492 whose one-line fix
  guards the convergence gate itself (finding-004: the easy lane is precisely that trap).
- 8 halt-excluded unless bounded: #451 #457 #458 #461 #473 excluded (systemic/architectural);
  #469 #472 admitted (enumerated sites, one-line shapes, no gate surface).
- 6 keystones (#432 #462 #463 #470 #487 #488) — consolidations, never quick.
- Rest are P1/P2 methodology or design changes.
On-ramp order for a maintainer wanting a run of wins: #469 (regex) → #472 (paths) →
#495 (archival) → #501 (knob) → #444 (grep line) → #436 (lesson upstream) → #499 (GUT rule)
→ #476 (checklist) → #478 (review rule) → #450 (SOP line) → #471 (validator rule) →
#453/#447 (decide-then-mechanical) → #482 (template) → #498 (rule+table) → #466 (fold into #336).
```
</details>

## Direction Health — corpus-level (the led-by-the-backlog check)

Run against the measured filers (`arcavenai`/`arcaven`), not just per-issue.

- **`burst_filing`:** +66 this run (vs +47 run-10, +14 run-9) — the burst is accelerating, all
  one filer plus arcaven's channel, **0 maintainer engagement on issues**. But filing rate is
  not the story: **integrity density is 35% of the burst** (23 of 66 vs 17% run-10) and the
  new issues systematically name the factory's *records* as the corrupted layer
  (attest-vs-verify). ADR-005: still baseline structure, not drift — but the structural signal
  is that the backlog now describes a verification system that cannot be trusted to describe
  itself.
- **`filed_vs_acted_gap`:** arcavenai 264 open : maintainers 0 acted — widened from 208.
  Repo-side activity exists (Zious11's rc.22 release train, 6 PRs merged 07-02..03) without
  touching the measured backlog. Structure (cold-start); becomes drift only if it persists
  after engagement.
- **`convert_finding_to_guard_unbounded`:** the pattern continues and now has its own
  antidotes filed: #444/#476/#478/#482/#498 propose new one-off guards, while #336 (+#466
  folded), #406, and #470's disposition-sweep contract remain the consolidation answers.
  Prefer the consolidations.
- **`cluster_same_lever_filed_n_times`:** **attest-vs-verify** emerges as the dominant class
  (23 issues; was 3 named instances in run-10). **steady-state hole** named (#488 #489 #491
  #493). **convergence-methodology** trio (#462 #497 #500 + #467). Existing clusters swelled:
  test-writer-gaps +18, spec-propagation-drift +15, false-green-convergence +22.
- **`over_engineering_fingerprints`:** LOW — 64 of 66 pilot-derived with cited
  commits/runs/greps; the 2 neutral scores are self-flagged scoping-doubt filings (#460
  #461), which is the *honest* form of the fingerprint.
- **Meta-signal:** arcaven's fleet-mining channel (#463 #464) is the third cross-cutting
  critique (after #308 CI-cost, #410 envelope) — and the first with a quantitative
  outcome-classification corpus behind it (4,602 dispatches, 169 hand-classified chains, 84%
  of real bad outcomes not model capability). It argues the factory's first-order quality
  lever is process feedback, which is precisely what the attest-vs-verify block corrupts.
- **Rate/drift signals:** _still not measurable — process not turned (ADR-005)._ Structure only.

## Classification index — run-11 new issues (finding-005 + finding-009)

report-type · defect-nature · repro · alignment · flag. Full axis vectors ride in the
per-section agent channels; follow `#NN` for bodies (B1).

| What it is | # | chips |
|---|---|---|
| No product-level defect register | **#432** | gap · design · Bohr · adv · **KEYSTONE** |
| Headless scene-open ≠ playability; evidence never made | **#433** | gap · spec · Bohr · adv · **🛑** |
| Implementer edits test-runner config | **#434** | policy · spec · Bohr · adv · **🛑** |
| Fix-phase spec fix skips story prose | #435 | process-gap · spec · Bohr · adv · degraded |
| GUT 4-arg gotcha | #436 | policy · off-by-one · Bohr · adv · **★** |
| Architect-added symbols drift at birth | **#437** | lesson · spec · Bohr · adv · **🛑** |
| Convergence on never-shipped scope | **#440** | process-gap · spec · Bohr · adv · **🛑** |
| Task closed, deliverables absent | **#441** | process-gap · spec · Bohr · adv · **🛑** |
| AC test never invokes §Trigger | #442 | adversary · spec · Bohr · adv · degraded |
| blast_radius is a lower bound | #443 | adversary · design · Bohr · adv · degraded |
| Sweep misses config surfaces | #444 | policy · spec · Bohr · adv · **★** |
| P-B sweep skips STORY-bumps | #445 | policy · spec · Bohr · adv · degraded |
| Fix-phase commit ordering | #446 | process-gap · design · Bohr · adv · degraded |
| Adjudication labels leak into comments | #447 | process-gap · spec · Bohr · adv · **★** |
| Wave-gate lacks HEAD-SHA tuple | **#448** | bug · spec · Bohr · adv · **🛑** |
| Within-burst re-bump invalidates fix | #449 | bug · spec · Bohr · adv · degraded |
| Reword skips impl/test/CLI tiers | #450 | enhancement · spec · Bohr · adv · **★** |
| Planner has no file-collision check | #451 | planner · design · Bohr · adv · **⏸** |
| Conflict resolution voids certificate | **#452** | process-gap · design · Bohr · adv · **🛑** |
| VP frontmatter shape asymmetry | #453 | policy · spec · Bohr · adv · **★** |
| Completed-but-unreported delivery gap | #457 | process-gap · design · Mandel · adv · **⏸** |
| Hardware-gated criteria unevaluable | #458 | enhancement · spec · Bohr · adv · **⏸** |
| Architect-decision items don't route | #459 | process-gap · spec · Bohr · adv · degraded |
| Unknown-config-keys rubric (scope doubt) | #460 | possibly-out-of-scope · spec · Bohr · **neutral** |
| Auto-mode merge veto (scope doubt) | #461 | possibly-out-of-scope · design · Bohr · **neutral · ⏸** |
| Probe variance drives convergence | **#462** | process-gap · spec · Mandel · adv · **KEYSTONE** |
| Fleet-mining engine asks (4,602 dispatches) | **#463** | proposal · design · n/a · adv · **KEYSTONE** |
| ~268k wasm resolver errors | #464 | bug · config · Bohr · adv · degraded |
| Adversary hallucinates BLOCKING | **#465** | adversary · intent · Mandel · adv · **🛑** |
| Adversary runs the linter | #466 | enhancement · spec · Bohr · adv · **★** |
| EC/POST enumeration + grep-verify | #467 | enhancement · spec · Bohr · adv · degraded |
| Dispatched model vs header mismatch | **#468** | bug · design · Heisen · adv · **🛑** |
| Traceability regex excludes alnum IDs | #469 | bug · syntax · Bohr · adv · **★ ⏸** |
| Remediation skips siblings, 7-pass | **#470** | process-gap · design · Bohr · adv · **🛑 KEYSTONE** |
| Version floor on citations | #471 | enhancement · spec · Bohr · adv · **★** |
| Lock helpers repo-relative paths | #472 | bug · config · Bohr · adv · **★ ⏸** |
| Rotation step tasks write-denied agent | #473 | bug · spec · Bohr · adv · **⏸** |
| Phase-4 gate unsatisfiable criterion | **#474** | bug · spec · Bohr · adv · **🛑 ⏸** |
| Stub-architect self-attests red | **#475** | process-gap · spec · Bohr · adv · **🛑** |
| SS-ID without dep-graph row | #476 | process-gap · spec · Bohr · adv · **★** |
| Tautologies invisible to mutation | **#477** | process-gap · spec · Bohr · adv · **🛑** |
| BC mandates absent engine API | #478 | process-gap · spec · Bohr · adv · **★** |
| Instrument output silently null | **#479** | process-gap · spec · Bohr · adv · **🛑▲** |
| Fixtures mask guard placement | #480 | process-gap · spec · Bohr · adv · degraded |
| Silent test-type substitution | **#481** | process-gap · spec · Bohr · adv · **🛑** |
| Rulings need 4 explicit fields | #482 | process-gap · spec · Bohr · adv · **★** |
| Evaluator pollutes signing provenance | **#483** | process-gap · design · Bohr · adv · **🛑** |
| Comment-truth disposition sweep | #484 | process-gap · spec · Bohr · adv · degraded |
| Dry-run exits 0 on violations | **#485** | process-gap · spec · Bohr · adv · **🛑** |
| Convergence counter in prose | **#486** | process-gap · design · Bohr · adv · **🛑** |
| Smoke sentinel gate | **#487** | enhancement · spec · Bohr · adv · **KEYSTONE** |
| Post-SHIPPED PRs bypass gates | **#488** | enhancement · design · Bohr · adv · **🛑 KEYSTONE** |
| docs/ needs bidirectional anchors | #489 | enhancement · design · Bohr · adv · degraded |
| Tokio starvation invisible to review | #490 | adversary · concurrency · Mandel · adv · degraded |
| inputDocuments silently rots | **#491** | bug · design · Bohr · adv · **🛑** |
| Wave-close gate ignores CI | **#492** | process-gap · spec · Bohr · adv · **🛑** |
| No delta-cycle mode post-SHIPPED | #493 | enhancement · design · Bohr · adv · degraded |
| Fabricated attestation evidence | **#494** | process-gap · intent · Bohr · adv · **🛑** |
| TaskList never archived | #495 | enhancement · design · Bohr · adv · **★** |
| Checker fails self-application | **#496** | process-gap · spec · Bohr · adv · **🛑** |
| Fresh-context review non-skippable | #497 | process-gap · spec · Bohr · adv · degraded |
| Numeric-literal space coverage | #498 | process-gap · logic · Bohr · adv · **★** |
| assert_push_error is consuming | #499 | process-gap · spec · Bohr · adv · **★** |
| Transitive seam drift, 10 passes | #500 | adversary · design · Bohr · adv · degraded |
| demo_artifact_format knob | #501 | enhancement · config · Bohr · adv · **★** |
| Race fixes must fail closed | #502 | process-gap · concurrency · Mandel · adv · degraded |

## Maintainer progress (outcome-paired — NOT a leaderboard)

Surfaces resolution **against beadle-discovered defects**, rewarding the verified fix-outcome
(flagged → fixed → validates), never close-rate / time-to-triage / volume (B3 + no-Goodhart).

| Outcome signal | Count | State |
|---|---|---|
| Silent data-loss defects (P0a) closed **with a load-bearing fix** | 0 / 3 | #342, #365, #358 open |
| Source-of-truth integrity defects (P0b) closed with a load-bearing fix | 0 / 50 | 26 prior + 23 new + #297 open |
| Pipeline-halt defects (P1 operational) closed + validated | 0 / 19 | 11 prior + 8 new open |
| Platform-envelope-mismatch decision (#410) taken | 0 / 1 | open — now 11 issues wait on this decision |
| Keystone consolidations accepted + wired | 0 / 13 | #336 + 6 prior + 6 new open |
| Quick wins exercised (process-turning on-ramp) | 0 / 34 | 18 prior + 16 new open |
| Cycles since last drift | — | _process not turned — withheld (ADR-005)_ |

_Cold-start: structure shown, streak/rate claims withheld until the process turns. No per-human
ranking — the metric moves only when a real defect gets a real, validated fix._

## Controls

Derived from authoritative state each render — never hand-authored. The next run parses
`- [x] <!-- verb=...;id=... -->`, dispatches, then resets the box (eventually consistent).

```
# Tier 1 — per-issue verbs (bounded to one artifact)
```
- [ ] <!-- verb=fast-track;id=#342 --> Fast-track #342 (🛑💾 P0a — merge silently deletes a served .factory file)
- [ ] <!-- verb=fast-track;id=#365 --> Fast-track #365 (🛑💾 P0a — rebase silently drops production code)
- [ ] <!-- verb=fast-track;id=#358 --> Fast-track #358 (🛑💾 P0a — orphan merge; merge-base --is-ancestor assertion)
- [ ] <!-- verb=investigate;id=#488 --> Deep-investigate #488 (🛑 KEYSTONE — steady-state bypasses every gate; #489/#491/#493 are facets)
- [ ] <!-- verb=investigate;id=#494 --> Deep-investigate #494 (🛑 P0b — fabricated attestation evidence; execute-then-quote discipline)
- [ ] <!-- verb=fast-track;id=#492 --> Fast-track #492 (🛑 P0b — wave-close gate must require CI green; one-line precondition)
- [ ] <!-- verb=fast-track;id=#441 --> Fast-track #441 (🛑 P0b — task close needs deliverables verification)
- [ ] <!-- verb=investigate;id=#440 --> Deep-investigate #440 (🛑 P0b — convergence on never-shipped scope; pre-verdict grep gate)
- [ ] <!-- verb=fast-track;id=#486 --> Fast-track #486 (🛑 P0b — machine-readable convergence counter)
- [ ] <!-- verb=investigate;id=#475 --> Deep-investigate #475 (🛑 P0b — stub-architect test authorship prohibition + re-execution check)
- [ ] <!-- verb=investigate;id=#483 --> Deep-investigate #483 (🛑 P0b — evaluator scratch-workspace sandbox; signing provenance)
- [ ] <!-- verb=investigate;id=#465 --> Deep-investigate #465 (🛑 P0b — adversary ground-truth verification; read with #466/#467)
- [ ] <!-- verb=investigate;id=#470 --> Deep-investigate #470 (🛑 KEYSTONE — disposition-sweep contract + source-value verification)
- [ ] <!-- verb=investigate;id=#462 --> Deep-investigate #462 (KEYSTONE — probe-variance methodology; read with #497/#500)
- [ ] <!-- verb=investigate;id=#463 --> Deep-investigate #463 (KEYSTONE — fleet-mining engine asks; telemetry is the structural fix)
- [ ] <!-- verb=investigate;id=#487 --> Deep-investigate #487 (KEYSTONE — smoke sentinel gate; operator-boundary class)
- [ ] <!-- verb=investigate;id=#432 --> Deep-investigate #432 (KEYSTONE — product-level defect register)
- [ ] <!-- verb=investigate;id=#410 --> Deep-investigate #410 (KEYSTONE tracker — envelope-mismatch; now 11 issues wait on it)
- [ ] <!-- verb=investigate;id=#336 --> Deep-investigate #336 (KEYSTONE — deterministic pre-review lint; fold #466 in)
- [ ] <!-- verb=fast-track;id=#469 --> Fast-track #469 (quick-win ⏸ — traceability regex one-liner)
- [ ] <!-- verb=fast-track;id=#472 --> Fast-track #472 (quick-win ⏸ — lock-helper path resolution, 7 sites)
- [ ] <!-- verb=fast-track;id=#495 --> Fast-track #495 (quick-win — TaskList archival at CONVERGED)
- [ ] <!-- verb=fast-track;id=#501 --> Fast-track #501 (quick-win — demo_artifact_format knob)
- [ ] <!-- verb=fast-track;id=#444 --> Fast-track #444 (quick-win — config-surface sweep grep)
- [ ] <!-- verb=fast-track;id=#436 --> Fast-track #436 (quick-win — GUT lesson upstream)
- [ ] <!-- verb=fast-track;id=#499 --> Fast-track #499 (quick-win — assert_push_error rule)
- [ ] <!-- verb=fast-track;id=#476 --> Fast-track #476 (quick-win — SS-ID checklist invariant)
- [ ] <!-- verb=fast-track;id=#478 --> Fast-track #478 (quick-win — pinned-engine API verify rule)
- [ ] <!-- verb=fast-track;id=#450 --> Fast-track #450 (quick-win — reword tier-sweep SOP line)
- [ ] <!-- verb=fast-track;id=#471 --> Fast-track #471 (quick-win — citation version-floor rule)
- [ ] <!-- verb=fast-track;id=#313 --> Fast-track #313 (🛑 P0b prior — ratchet certifies absent artifacts)
- [ ] <!-- verb=fast-track;id=#314 --> Fast-track #314 (🛑 P0b prior — phantom frontmatter drift)
- [ ] <!-- verb=fast-track;id=#370 --> Fast-track #370 (🛑 P0b prior — CI static PASS)

```
# Tier 2 — board-level maintenance requests (whole-corpus, expensive, on-demand)
```
- [ ] <!-- verb=reprioritize;id=board --> Re-rank the action plan
- [ ] <!-- verb=full-refresh;id=board --> Re-enumerate & re-triage every open issue
- [ ] <!-- verb=revalidate;id=board --> Re-run validate across the whole corpus
- [ ] <!-- verb=rescore-intent;id=board --> Re-run score-intent corpus-wide (after a manifest change)

---

<details><summary>Legend & references</summary>

**Verdict:** 🟢 advances · 🟡 neutral · 🔴 drifts.
**Top-axis flags:** 🛑 integrity (source-of-truth / **safety** gate, ranks first) · 💾 data-loss · ▲ silent-data-loss adjacency · ⏸ halt / 💥 panic (operational / **liveness** — does it stop the factory? finding-009) · ★ quick-win-eligible.
**Repro badge:** `Bohr` = consistent, isolatable · `Mandel` = intermittent / activation-dependent · `Heisen` = changes under observation (feeds the effort estimate).
**Chips:** `systemic` = high-leverage · `KEYSTONE` = consolidates a defect family.

Classification rests on: defect-nature — [IEEE 1044](https://standards.ieee.org/ieee/1044/4128/) / [ODC](https://www.chillarege.com/odc/); reproducibility — [Grottke–Trivedi (Bohr/Mandel/Heisenbug)](https://doi.org/10.1109/MC.2007.55); [severity vs priority](https://www.centizen.com/severity-vs-priority-a-crucial-distinction-in-software-testing/); operational-impact — ODC *Impact/Reliability* split along the failure-model crash-vs-liveness spine (Cristian 1991; Schlichting & Schneider 1983; Alpern & Schneider 1985). Cold-start gating: ADR-005.

</details>

<sub>🤖 Maintained by **beadle** running as `arcavenai` — an automated triage assistant. State
lives out-of-band; this body is regenerated each run (hand-edits ignored, a wiped body
regenerates). Checkboxes are read on the next scheduled run, acted on, then reset. Scored
against intent vsdd-factory@0.3 (self-referential; provenance-weighted; defect-classification
superset, finding-005; operational-impact axis, finding-009). Issue detail is linked, never
replicated (B1) — follow `#NN` to fetch.</sub>
