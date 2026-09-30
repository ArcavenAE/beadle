# beadle shadow comparison, 2026-09-30

The binary cannot yet stand in for the skills method on any board: from one
store it renders 4.3% of what the skill posts, and it will not render at all
without the skill's classifications. On four of the five boards it has no input at
all: the skills runs write no store, so their state lives only in the board's
own sentinel. The cheapest wins are in the store, the gate and the sync path,
not the renderer.

Measured at ArcavenAE/beadle main `bc8c1c5`, 2026-09-30, from kinu. Every count
below comes from a command run in the shadow workspace; the raw outputs sit
beside this file. This extends the method in `aae-orc-5buzr` and
`docs/proposals/binary-skill-convergence.md` (first run 2026-09-18) from one
board to five.

## Scope and method

| board | class | method published by | live run | live size |
|---|---|---|---|---|
| vsdd-factory#312 | public, third-party | skills (errand) | 20 | 93,402 B |
| marvel#401 | own | skills (errand) | 2 | 57,107 B |
| director#158 | own | skills (errand) | 2 | 32,119 B |
| client board A | client | skills (errand) | 2 | 30,161 B |
| client board B | client | skills (errand) | 2 | 23,801 B |

- **Rust shadow:** `beadle enum --full`, `beadle sync`, `beadle render`, each
  with `--root` in the shadow workspace. vsdd-factory ran twice: once from a fresh
  store, once from the store the skills shadow built (same store, same day, same
  watermark). The other four ran from fresh stores with repo-only intents;
  errand's intents were not reachable from kinu, and errand reports it holds no
  store for them (see "State sources"). For those boards the live
  `beadle-state` sentinel stands in for the store.
- **Skills shadow:** vsdd-factory only, one agent following
  `skills/beadle-triage/SKILL.md` and `prompts/create-dashboard.md`, with every
  GitHub write replaced by a log line.
- **Live bodies:** fetched read-only at 07:06Z. All five show `updatedAt` between
  04:54Z and 05:05Z.

### No-write proof

- Rust: the only subprocess in `crates/beadle` that is not a read is
  `push.rs:77` (`gh issue edit --body-file -`), reached only from `beadle push`,
  and `push.rs:70-74` returns before it under `--dry-run`. `render`
  (`main.rs:138-142`) reads the store and prints. `enum` (`enumerate.rs:110`) and
  `sync` (`sync.rs:77`) call only `gh issue list`. The shadow never invoked
  `push`.
- Skills: every `gh` call went through a wrapper that passes reads and refuses
  anything else with exit 3, logging it. Controls before the run: a read passed;
  `issue edit` and a GraphQL mutation were refused. The run's refusal log is
  empty: no write was attempted.

## Observations

### vsdd-factory (same store)

| axis | skills shadow | Rust shadow | live |
|---|---|---|---|
| size | 91,806 B | 3,983 B | 93,402 B |
| h2 / h3 / table rows | 8 / 22 / 236 | 6 / 1 / 29 | 8 / 22 / 269 |
| sections | full skeleton | Direction verdict, Baseline, Clusters, Classification summary, Board controls, Editor notes | full skeleton |
| classifications | 5 new (#837 to #841), one corrected on re-ingest | reads the same 5 | 5 new this run |
| gate vs live (pinned `verify.json`) | 6 fails: 2 by design (run, watermark), 4 headings lost | 96 fails | n/a |
| gate vs run-19 fixture | 2 fails, both run-19 headings lost | 92 fails: 40 state axes, 28 headings, 14 sections or run, 8 prior-run indexes, 2 other | n/a |
| run time | 12m43s wall, 257k agent tokens | render under 0.01 s; enum 7.6 s, sync 14.8 s | n/a |
| would-be writes | 1 body edit, 2 comments, 5 labels, 1 fixture commit plus PR | none (no push) | n/a |

- `dashboard-gate.py` and `beadle verify` gave identical fail counts on all four
  candidate pairs: the Rust port of the gate is at parity on this sample.
- The binary's share of the skill's body is 4.3% (3,983 of 91,806 B), against
  5.8% on 2026-09-18 (4,983 of 85,420 characters). This store holds 1 run; the
  live store holds 20, so the binary's share is likely understated. Treat it as a
  lower bound.
- The 5 `impact.*` labels the skill would set do not exist on the repo, so all 5
  writes would fail.
- The skills run needed three departures from the literal method: the live board
  was already at run 20, so the shadow rebuilt run 20 from the committed run-19
  fixture; a fresh store made `beadle direction` report the wrong engagement band
  and A4 streaks, so those were recomputed by hand per SKILL §6b; five issues were
  graded in one pass, not by parallel graders.
- The shadow agent reported a gate pass after adding two rename entries to its
  own copy of `verify.json`. With the pinned file the same body fails.

### All five boards (fresh store, live body)

| axis | vsdd-factory | marvel | director | client A | client B |
|---|---|---|---|---|---|
| open per `enum` | 519 | 60 | 31 | 5 | 13 |
| open per board | 519 | 57 | 29 | 4 | 13 |
| explained by | exact | 3 opened after refresh | 1 opened after; self excluded | self excluded | self excluded; 1 closed after refresh |
| dashboard issue counted as open | yes | yes | no | no | no |
| sync scanned / open | 500 / 519 | 60 / 60 | 31 / 31 | 5 / 5 | 13 / 13 |
| h2 sections | 8 | 10 | 9 | 9 | 9 |
| gate on live vs itself | run, watermark only | same | same | same | same |
| warm-up verdict at run 2 | n/a (run 20) | COLD START | COLD START | ALIGNED | ALIGNED |
| state source | skills-shadow store (same store) | live sentinel only | live sentinel only | live sentinel only | live sentinel only |
| same-store render | done (above) | not possible: no store | not possible: no store | not possible: no store | not possible: no store |

- **Heading text drifts.** The same logical section is spelled three or four ways
  across boards ("Quick wins: safe to act on", with and without a parenthetical).
  The gate matches prefixes, so it cannot see drift.
- **Removed sections survive as tombstones.** All four run-2 boards keep an
  ISO/IEC 25010 section whose only line says the grouping was dropped by operator
  request on 2026-09-30. The gate fails any body that loses a heading, so a
  removal the operator ordered stays as an empty section.
- **State grows by rule.** The run-2 sentinels say list axes are cumulative
  "because the regression gate forbids shrinking them", and current state moves to
  parallel `*_now` keys. Run-scoped headings ("Run-19 findings", "Run-19 index")
  must likewise be kept or declared renamed forever. vsdd-factory grew from
  87,232 B at run 19 to 93,402 B at run 20.
- **GitHub accepted 93,402 B.** That is 7% above `BODY_OBSERVED_ACCEPTED_BYTES`
  (87,232, `render.rs:41`), which the code records as observed, not as a limit.
- **One schema label, five shapes.** Every sentinel says `schema: 1`; the
  vsdd-factory keys (`a4_windows`, `keystone`, `maintainer_actions`) and the
  run-2 keys (`prs`, `iso25010`, `close_proposed`, `main_sha`) barely overlap.
- **Refresh time.** The briefed refresh is 06:00Z; all five bodies were written
  04:54Z to 05:05Z.

## Recommendations

### Rust

1. **Guard `sync` against a truncated fetch.** `sync.rs:72-73` hard-codes
   `--limit 500` with no check; vsdd-factory has 519 open issues, so 19 issues'
   comments are skipped on every sync without a word. `enum` got the same guard
   in beadle#83 (`enumerate.rs:68`); `sync` is the remainder of that class.
2. **Commit the four run-2 intents to `targets/`.** The binary cannot run those
   boards from the repo; their intents exist only with errand. Without them no
   one but errand can reproduce a board.
3. **Emit the cumulative state axes first.** Of the Rust render's 92 gate
   failures against the run-19 fixture, 40 are state axes the binary never
   writes (`p0_integrity`, `quick_wins_prior`, `operational_impact.*`), the
   largest single class; 28 are headings and 14 are sections. The axes come from
   the store the binary already reads, so they are the cheapest class to close.
4. **Update `BODY_OBSERVED_ACCEPTED_BYTES` to 93,402** with this run as its
   evidence, keeping it an observation.
5. **Name the self-count rule.** Decide whether the dashboard issue counts as
   open and apply it in `enum`; two of five boards count it and three do not.

### Skills

1. **Ingest every run into a store.** Four of five boards keep their state only
   in the posted sentinel, and the one store that exists is a week stale. The
   binary cannot render, verify against, or reproduce a board whose skills run
   ends at the post. Running `beadle classify ingest` each run, as SKILL.md
   already describes for classifications, gives the binary its input.
2. **Let the gate distinguish per-run content from board structure.** Run-scoped
   headings and cumulative lists are what the gate forces to grow. A declared
   class of per-run headings that may drop without a rename entry would stop both
   the growth and the tombstones.
3. **Carry declared removals in the target, reviewed, not in the run.** A rename
   or removal a run can add to its own allowlist lets any run pass the gate. The
   `gate-declared-removals` branch is the place for this.
4. **Pin warm-up rules to counts, not judgment.** Four boards in the same run-2,
   two-day state gave two verdict classes.
5. **Check labels exist before planning label writes.** All 5 `impact.*` label
   writes on vsdd-factory would fail.
6. **Normalize heading text across targets** so a prefix gate and a reader see
   one board shape.

Judgment stays in the skill. None of the above moves grading, attention-lane
admission, priority, or the verdict paragraph into the binary (ADR-007,
finding-019).

## Proposed issues (not filed)

1. beadle: skills runs write no store on four of five boards (Skills 1). This
   blocks every other convergence item for those boards.
2. beadle: `sync` has no truncated-fetch guard (Rust 1). Live today: silent
   data loss today on the largest board.
3. beadle: commit run-2 target intents (Rust 2).
4. beadle: gate class for per-run headings and cumulative axes (Skills 2, 3).
5. beadle: emit cumulative state axes from the store (Rust 3).

## State sources

The errand operator's backups were not reachable from the measuring host. The
errand seat reports three gaps, relayed by the team supervisor and not verified
here:

1. Only vsdd-factory has a `store/state.jsonl`. The skills-method runs write no
   store, so the other four boards' state lives only in each board's
   `beadle-state` sentinel.
2. The vsdd-factory `state.jsonl` dates from 2026-09-23 and predates the skills
   refreshes, so it is stale.
3. The marvel, director and client board A intents are run-1 drafts from
   2026-09-28.

Per board, this report therefore used:

| board | state used | intent used |
|---|---|---|
| vsdd-factory | the store the skills shadow built today (same store for both sides) | `targets/vsdd-factory.intent.yaml` at `bc8c1c5` |
| marvel | live sentinel, fetched 07:06Z | repo-only (errand draft not reachable) |
| director | live sentinel, fetched 07:06Z | repo-only (errand draft not reachable) |
| client board A | live sentinel, fetched 07:06Z | repo-only (errand draft not reachable) |
| client board B | live sentinel, fetched 07:06Z | repo-only (errand intent not reachable) |

Repo-only intents carry no maintainer list, so `sync` counted every event on
those four boards as "other"; their maintainer and measured splits are not
comparable and are not reported.
