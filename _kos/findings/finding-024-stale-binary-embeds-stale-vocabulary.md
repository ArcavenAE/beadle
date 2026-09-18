# finding-024: a stale `beadle` binary silently carries an old embedded vocabulary, and only `classify ingest` finds out

Date: 2026-09-18 (run-19)
Scope: ambient tooling (beadle binary / vocabulary contract)
Status: fixed by rebuild; the detection gap is open
Captured per `../../.claude/rules/tooling-friction.md` before the workaround was applied.

## Symptom

`classify ingest` refused a valid run-19 payload:

```
Error: beadle command failed
Caused by:
    0: payload item 0
    1: `report_type` value `process-gap` not in ["task", "bug", "feature",
       "regression", "security", "dependency", "ci-build", "flaky-test",
       "tech-debt", "perf", "docs", "question", "rfc"]
```

The rejected value is **canonical**. `skills/beadle-triage/vocabulary.json`
carries 20 `report_type` values; the binary enforced 13. The seven it did not
know — `process-gap`, `enhancement`, `policy`, `proposal`, `lesson`, `gap`,
`possibly-out-of-scope` — are exactly the beadle-original tokens, and
`process-gap` is the single most common report type on this board.

## Cause

**Two build artifacts exist at different vintages, and nothing says which one
the workflow uses.** `target/release/beadle` was built **2026-07-01**;
`target/debug/beadle` was newer. `/dashboard-refresh` names neither, so the
operator picks — and "release" is the natural pick for a production run. That
is the stale one.

Verified against the store: runs **17 and 18 both ingested records carrying the
post-`0724014` tokens** (run-17: #755 #756; run-18: 12 of 32, including #762
#794 #796), which the 07-01 release binary would have rejected exactly as it
rejected run-19's. So those runs used the debug artifact. The failure is not
that beadle's contract drifted — it is that the same repo offers two enforcers
of two different contracts and the command does not choose.

`vocabulary.json` was last
changed **2026-07-20** (`0724014`, the finding-009 operational_impact
alignment). The manifest is embedded **at build time**, so a binary older than
the manifest enforces a vocabulary that no longer exists. Nothing on the binary
announces its vintage, and `--version` would not have helped: the version
string did not change.

`tests/vocabulary_contract.rs` holds the skill and the binary together, but it
holds them together **in CI, against the source tree** — it cannot say anything
about a stale artifact sitting in `target/`.

## Why this is worth a finding rather than a rebuild-and-move-on

The failure is loud, which is the good case. The bad case is the one this
almost became: the error names the *payload* (`payload item 0`), not the
*binary*, so the natural first reading is "the grader emitted a bad token" —
and the natural first fix is to edit the payload down to the 13 values the
binary will accept. That silently re-introduces the exact enum drift
finding-020 F2 recorded, and it would have looked like a clean ingest.

The generalisable shape: **a build-time-embedded contract turns any stale
artifact into a silent old-contract enforcer**, and the error surfaces at the
consumer, blaming the input.

## Workaround applied

Rebuild (`ax build beadle`), then re-ingest — appended 4/4. This is the correct
fix, not a patch; the payload was never wrong.

## Open

- No staleness check. A cheap one: have the binary compare a hash of its
  embedded manifest against the on-disk `vocabulary.json` at startup and warn
  on mismatch — it already knows both.
- `/dashboard-refresh` does not build before ingesting. Any run that uses a
  checked-out `target/` inherits whatever vintage is there.
- `/dashboard-refresh` step 2 should name the artifact (or build it). Right now
  the correct behaviour is reached by luck: runs 17 and 18 happened to use
  `target/debug`, run-19 reached for `target/release` and failed. Cheapest fix
  is a build step before ingest, which also closes the staleness question above.
- The stale `target/release/beadle` (2026-07-01) is still on disk and still
  enforces the 13-value vocabulary. Anything that reaches for it gets the old
  contract.

## References

- `.claude/rules/tooling-friction.md` (capture before workaround)
- finding-020 F2 (the pre-alignment enum drift this would have re-created)
- `skills/beadle-triage/SKILL.md` §6 (the manifest-wins rule)
- `crates/beadle/tests/vocabulary_contract.rs`
