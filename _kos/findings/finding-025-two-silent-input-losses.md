# finding-025: two inputs beadle drops without a word: sync past 500 issues, and flow-sequence maintainers

Date: 2026-09-30 (measured), filed 2026-10-02
Status: observed during the shadow comparison (beadle#94, draft), each tested
against the binary at bc8c1c5 and re-checked at origin/main 94d9f15.
Subject: beadle's own fetch and intent parsing.

## The finding

1. `sync` reads at most 500 open issues and does not say so. `sync.rs:72-73`
   hard-codes `--limit 500`. vsdd-factory had 519 open issues, so 19 issues'
   comments were skipped on every sync with no warning. `enum` got a guard for
   the same class in beadle#83 (`enumerate.rs:68`); `sync` is the remainder.
   The orc's finding-200 records the same cap on GitHub search.
2. A flow-sequence `maintainers: [a, b]` loads as an empty list. `sequence()`
   (`intent.rs:313-338`) only starts at a key line ending in `:`, so every
   maintainer event then counts as "other". A flow-map `target` already fails
   loudly (`no repo: scalar`); an ownerless `repo` loads and fails only at the
   GitHub call.

Both turn a wrong board into a plausible board, which is the class
elem-silent-integrity-severity ranks highest when beadle grades others.

## What would close it

A truncated-fetch guard in `sync` as in `enum`, and a refusal (or a YAML
parser) for flow forms in intents. Neither is built here.
