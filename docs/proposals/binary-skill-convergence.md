# Binary ↔ skill convergence — what `beadle` does, what `/dashboard-refresh` does, and what should move

Date: 2026-09-18
Author: run-19 session
Status: proposal — no code changed by this document
Evidence base: run-19 (`docs/fixtures/vsdd-factory-312-curated-run19.md`), executed
end to end on 2026-09-18, plus `beadle render vsdd-factory` against the same store
on the same day.

---

## 1. The headline

Both artifacts were produced from **one store, on one day, at one watermark**:

| | skill (`/dashboard-refresh`) | binary (`beadle render`) |
|---|---|---|
| Body | **85,420 chars · 28 sections** | **4,983 chars · 8 sections** |
| Direction verdict | **🟡 WATCH** | **🔴 drifting** |
| A4 residue | #523 #588 #635 | #479 #523 #588 #635 |
| A4 streak units | attention windows (30/30/28) | runs (11) |
| Open issues | 514 | 529 |
| Action plan | P0a/P0b/P1/P2 lanes, 40+ rows | *absent* |
| 👤 Needs-human-reading lane | 7 rows | *absent* |
| Maintainer progress | 13 outcome-paired rows | *absent* |
| Per-issue classification rows | 8 run indexes, carried | counts only |

The binary is at **6% of the body** and **disagrees with the board's verdict on
identical data**. That disagreement is not a rounding difference — it is a red
headline against a yellow one, and either could be read as authoritative by
someone who runs the command.

**The single most consequential fact in this assessment:** the binary cannot
close most of that gap today *even if someone wrote the rendering code*, because
**the store does not contain the data**. `classify ingest` silently discards four
fields on every record (§3.1). The skill's richest output has never reached the
store, which charter B1 designates the source of truth.

---

## 2. Method

This is not a code read against a spec. Run-19 was executed by hand under the
skill, and the binary was then pointed at the resulting store. Every number above
is from one of those two artifacts. Where this document claims the binary lacks
something, the claim is "it did not appear in the rendered output and the field is
absent from the struct," not "I did not find the function."

---

## 3. Root causes, in dependency order

The capability gap is not 20 missing features. It is **four structural facts**,
and most of the missing surface is downstream of the first.

### 3.1 The store is lossy — `classify ingest` drops four fields (ROOT CAUSE)

`ClassificationRecord` (`crates/beadle-store/src/lib.rs:88`) has no
`short_title`, `attn`, `triage_state`, or `possibly_fixed`. Serde ignores unknown
fields, ingest re-serialises the struct, and the extras are gone. Verified on
both runs that used the path:

```
run-19 sent  : short_title, attn, triage_state, possibly_fixed  (+21 known)
run-19 stored: 21 known fields
run-18 fixture: 25 fields   run-18 store: 21 fields
lost on ingest: attn, possibly_fixed, short_title, triage_state
```

Consequences, each of which reads as a separate "missing feature" until you trace
it here:

- **The 👤 lane cannot be rendered from the store.** `attn` is the entire §3b/§7d
  facet. The binary has no way to know an issue is in the lane.
- **Title-led rows are impossible.** finding-005's row-legibility rule ("title
  leads, verdict trails") depends on `short_title`. Without it, the binary can
  only lead with `#NN`, which is the exact defect finding-005 named.
- **The fixed-but-open sweep cannot be mechanised.** `possibly_fixed` is where
  that verdict lives.
- **B1 is inverted for these fields.** The git-tracked *fixture* is the only
  durable carrier; the store is not. The charter says the opposite.

There is a second-order irony worth stating plainly, because it is also the
strongest argument for priority: **a tool whose top severity class is
"a system of record disagrees with reality without raising a signal" has that
exact defect in its own ingest path.** It is issue-class #313 applied to beadle.

### 3.2 The renderer implements v1; the board runs v2

`direction.rs` contains no reference to `maintainer_capacity`, `attention_window`,
or any part of SKILL §6b. Every streak is run-denominated. The skill post-processes
to v2 by hand each run — SKILL §6b says so explicitly ("the crate's
`DirectionReport` still computes v1; post-process per this section until it catches
up").

That standing workaround has now produced a live contradiction: **🔴 vs 🟡 on the
same store**. The v2 rule that resolves it is not subtle — §6b rule 4 requires
*both* a red capacity-adjusted band *and* a falsified cost explanation before a red
headline, and the second condition is unmet. The binary does not know the rule
exists.

The A4 set also diverges (#479 included by the binary, excluded by the board) and
the denominators differ (runs vs attention windows), so the two disagree on *which
issues*, *how long*, and *what it means*.

### 3.3 Only two editor slots exist, so `push` is unsafe

`push.rs` preserves exactly two slots — `editor:direction-verdict` and
`editor:notes` — via `extract_slot`/`merge_slots`. Everything else is regenerated.
Since the binary generates 6% of the board, running `beadle push` today would
**destroy the other 94%**. This is why `/dashboard-refresh` forbids the verb
outright and the skill pushes via `gh issue edit`.

`grep -rn curated crates/` returns only comments. The five curated zones that
`aae-orc-iouu` specifies do not exist. That ticket's premise is intact.

### 3.4 Derived state ignores stored state

Small, but it is the kind of error that discredits the rest of the board:

- **Open issues: 529.** The renderer counts distinct issue numbers ever observed.
  The `IssueRecord` struct *has* a `state` field, and 19 issues in the store are
  closed on GitHub (#204 #229 #243 #244 #296 #300 #365 #418 #465 #472 …). The
  data is present and unused.
- **Clusters all read `archived (10 runs quiet)`** with truncated membership —
  `silent-data-loss` shows 3 members where the board tracks 9. The 13 `cluster`
  records are fossils from ~run 9; the skill has maintained cluster state in the
  dashboard sentinel ever since, which the binary never reads.

---

## 4. The boundary: what must NOT move into the binary

This is the load-bearing constraint on everything below, and it is why "make the
binary do `/dashboard-refresh`" is the wrong goal statement.

SOUL §8 / ADR-007: **automate reminding, checking and proposing — not judging.**
finding-019 put it in this repo's own words: *analysis lives in the discipline, not
the plumbing.* Run-10 is the demonstration — a binary render replaced a curated
board with a flat table and the analysis was lost.

Stays in the skill, permanently:

- **Alignment grading with cited rationale** (B4, the differentiator).
- **attn-lane admission.** Run-19's own override is the argument: the grader
  tagged #834 `attn.governance/reply-needed`; the curator declined it because the
  ask reduced to two sentences without loss and the lane sat at its ceiling. No
  rule computes that. What the binary *can* do is enforce that the decision was
  recorded.
- **The direction-verdict paragraph** — the interpretation above the table.
- **Priority assignment**, and every curator override.
- **Whether a comment clears the §9 bar.**

The correct goal is narrower and much more achievable: **the binary should own
everything mechanical, so the skill spends its whole budget on judgment.** Today
the skill hand-carries a great deal of pure mechanism — carry-forward, sentinel
arithmetic, window counting, render-integrity checking — and that is where the
time and the risk actually go.

---

## 5. Prioritised recommendations

Ordered by (unblocks-others × risk-reduced) ÷ effort. Effort is rough:
S ≈ a sitting, M ≈ a day, L ≈ multi-day.

### P0-1 · Stop the ingest from dropping fields — **S**

Add `short_title`, `attn`, `triage_state`, `possibly_fixed` to
`ClassificationRecord`. Add `#[serde(deny_unknown_fields)]` so the *next* schema
drift fails loud instead of silently. Backfill run-13→19 from the git-tracked rich
fixtures, which still hold the data.

*Why first:* it is small, it is a silent-data-loss defect in the SDL detector, and
**P1-1, P1-3, P2-2 and P2-3 are all impossible until it lands.** Nothing else on
this list has that property.

*Acceptance:* round-trip a run-18 fixture record through ingest and read back all
25 fields; an unknown field is rejected with a named error.

### P0-2 · Ship the regression gate as `beadle verify` — **S**

Run-19 wrote a gate (`tools/dashboard-gate.py`, 196 lines, preserved with this proposal) implementing the five
checks in `/dashboard-refresh` §3. It is pure mechanism, it needs no judgment, and
**it caught a real regression this run**: `operational_impact.degraded_new` reset
and dropped 11 run-18 members. It also proved the command doc's own check 3 is
too weak — a union comparison misses any loss confined to one axis, because the
numbers reappear elsewhere.

Port it to `beadle verify <target> --before <snapshot> --candidate <file>`, with
the strengthened semantics run-19 landed on:

1. Per-axis preservation, not union (this is the bug).
2. Per-run axes (`*_new`) must roll members into a `*_prior` counterpart.
3. Render-integrity checks are **regression-relative** — a pre-existing violation
   warns, a new one fails. Absolute checks would block every post forever: the
   live body carries an inherited `5d` violation from run-18 — one `<details>`
   block holding one 30-row table (the gate reports it per row, so it surfaces
   as 30 entries; the defect is one table).
4. Declared renames allowlisted with justification.

*Why so high:* the gate is the only thing standing between a bad render and the
public board, it currently exists as a throwaway file in `tmp/`, and it is the
cheapest large risk reduction available. It also makes every later change safe to
attempt.

*Acceptance:* negative-control tests — deleting a cluster member, resetting a
`*_new` axis without roll-forward, and introducing a new blockquote-table adjacency
must each fail; the run-18→19 pair must pass.

### P1-1 · Curated zones — generalise slots from 2 to N (**`aae-orc-iouu`**) — **M**

Exactly as that ticket specifies: extend `extract_slot`/`merge_slots` into named
curated zones (`curated:action-plan`, `curated:quick-wins`,
`curated:direction-health`, `curated:classification-index`,
`curated:maintainer-progress`). Binary owns the frame — sentinel, derived digest,
controls, discovery; skill owns the zones.

*Why here and not first:* it is the keystone that makes `beadle push` usable and
retires the `gh issue edit` interim, but it is worth more once P0-2 can prove a
push preserved everything, and its zone list should be settled after P0-1 fixes
what the store can hold.

*Acceptance (from the ticket, unchanged):* byte-identical round trip;
`derived_digest` scoped to binary-owned sections only; the run-11 fixture
expressible as frame + zones with no content loss.

### P1-2 · Direction v2 in the crate — **M**

Teach `direction.rs` the `maintainer_capacity` block: attention-window
denominators, the success bands, rule 4's two-condition red gate, and the
descriptive severity ceiling. Keep v1 computable and **stamp every
`DirectionReport` with its instrument version**, because §6b forbids re-scoring
evaluated predictions under a new instrument.

*Why:* it removes a standing hand-post-process step, and it ends the state where
two commands give two verdicts on one store. Until it lands, `beadle direction`
should say out loud that it computes v1 and is not the board's verdict.

*Acceptance:* run-19's store yields 🟡 WATCH with `merge_latency` red and rule 4
unmet; A4 reports 30/30/28 in windows; the run-16 prediction stays FAILED.

### P1-3 · Carry-forward as a store query — **M**

The skill hand-carries every prior run index, unchanged P2 cluster and keystone
history. The store has all of it. Give the renderer a "carried" projection so
prior-run indexes render from data rather than from copy-paste.

*Why:* this is the single largest block of mechanical work the skill does by hand
each run, and hand-carry is precisely how content gets dropped.

### P2-1 · Fix derived counts and cluster decay — **S**

Honour `IssueRecord.state` in the open count (529 → 514). Either maintain cluster
records each run or stop rendering a decay verdict computed from fossils. Today
the section says `archived (10 runs quiet)` about clusters the board actively
uses.

### P2-2 · Tier-1 controls from data — **S**

The binary emits only board-level Tier-2 boxes. The skill hand-writes ~30 Tier-1
per-issue verbs, every one of which is derivable once P0-1 lands (`possibly_fixed`
→ `accept-deferral`/close; `attn` → `investigate`; quick-win → `fast-track`).

### P2-3 · Mechanical section scaffolds — **M**

Emit the *structure* of the action plan, quick-wins lane and 👤 lane — correct
lanes, correct ordering, correct chips, rows placed by stored axes — and leave the
prose to curated zones. The binary should never author the analysis; it should
make it impossible to lose a row. Depends on P0-1 and P1-1.

### P3 · Smaller items

- **Build-before-ingest** in `/dashboard-refresh`, plus an embedded-vocabulary
  staleness warning (`finding-024` / ArcavenAE/beadle#64). Run-19 hit this: the
  2026-07-01 release artifact enforced a 13-value vocabulary against the
  manifest's 20.
- **Legend & references footer** (§7c) — fully mechanical: emit only the chips
  that appeared this run.
- **Dispatch-ledger verb** so the perf ledger is not hand-assembled prose.
- **Body-budget telemetry.** `BODY_BUDGET_BYTES = 55 * 1024` (56,320) in
  `render.rs:29`. The live board is **87,232 bytes** — 1.55× the configured
  budget — and GitHub stores it without complaint. Today that only mis-reports
  ("8% of budget"), because exceeding it merely warns; the risk is the comment
  above the constant, which promises future work that "triggers rollup of oldest
  editorial detail." A rollup driven by a budget 1.55× below the observed-good
  size would evict carried content that is fine. Re-derive the constant from
  what GitHub actually accepts before anything acts on it.

---

## 6. Sequencing

```
P0-1 store fidelity ──┬─→ P1-1 curated zones ──→ P2-3 section scaffolds
                      ├─→ P1-3 carry-forward ──────────┘
                      └─→ P2-2 tier-1 controls
P0-2 beadle verify ───── (independent; makes every step above safe to attempt)
P1-2 direction v2 ────── (independent)
P2-1 counts/decay ────── (independent)
```

P0-1 and P0-2 are both a sitting each, are independent of one another, and
together convert this from "the binary is 6% of the board" into "the binary can
be grown safely." Do them first and in either order.

## 7. What *not* to do

- **Do not re-enable `beadle push` before P1-1.** It would repeat run-10.
- **Do not move grading, attn admission, priority or the verdict paragraph into
  the binary.** finding-019 and ADR-007 both rule this out, and run-19's #834
  override is a live example of judgment no rule reproduces.
- **Do not make the gate absolute rather than regression-relative.** The live body
  carries an inherited violation (one 30-row table inside a `<details>`); an
  absolute gate blocks every future post.
- **Do not trim carried content to fit a 65,536 budget** that empirically is not
  the limit. Fix the constant instead.

## 8. Defects found while writing this

All filed 2026-09-18 against `a10c158`, counts verified under the claim gate.

| | Where | GitHub | bd |
|---|---|---|---|
| Ingest drops 4 fields from every record | `beadle-store/src/lib.rs:88` | **#66** | `aae-orc-l5b5i` (P0) |
| Open count ignores `IssueRecord.state` (529 vs 514) | `render.rs` baseline | **#67** | `aae-orc-nhlyq` (P2) |
| Cluster decay computed from run-9 fossils | `render.rs` clusters | **#68** | `aae-orc-nhlyq` (P2) |
| `direction` verdict contradicts the board (🔴 vs 🟡) | `direction.rs` | **#69** | `aae-orc-it9r3` (P1) |
| Body-budget constant 1.55× below observed-good | `render.rs:29` | **#70** | `aae-orc-veq8a` (P3) |
| Stale release binary embeds stale vocabulary | build/workflow | #64 | `aae-orc-ety14` (P3) |
| §3 check 3 is union-based, blind to per-axis loss | command doc | — | `aae-orc-agwo8` (P0) |

### Work items

| Rec | bd | Blocked on |
|---|---|---|
| P0-1 store fidelity | `aae-orc-l5b5i` | — |
| P0-2 `beadle verify` | `aae-orc-agwo8` | — |
| P1-1 curated zones | `aae-orc-iouu` (pre-existing) | P0-1 |
| P1-2 direction v2 | `aae-orc-it9r3` | — |
| P1-3 carry-forward from store | `aae-orc-lle2s` | P0-1 |
| P2-1 counts + cluster decay | `aae-orc-nhlyq` | — |
| P2-2 Tier-1 controls from data | `aae-orc-89eaq` | P0-1 |
| P2-3 section scaffolds | `aae-orc-2n15r` | P0-1, P1-1 |
| P3 build-before-ingest | `aae-orc-ety14` | — |
| P3 body budget | `aae-orc-veq8a` | — |
| P3 render tail (legend, ledger) | `aae-orc-r1w1n` | — |
| **Re-run this assessment** | `aae-orc-5buzr` | P0-1, P0-2 |

`aae-orc-5buzr` carries the method, so the next pass re-measures rather than
trusting the numbers above. They are a measurement at a timestamp, not standing
facts — the open count, A4 streaks, body size and verdict pair will all expire.

## 9. Honest summary

The binary is not a weak implementation of the skill. It is a **correct
implementation of a much smaller thing** — a store projector with a verdict
engine — that has drifted out of sync with a board the skill grew past it. The
gap is mostly one root cause (a lossy ingest) plus one deliberate constraint that
should stay (judgment belongs to the skill).

Two sittings — fix the ingest, ship the gate — change the shape of the problem.
Everything else on this list is then ordinary work.
