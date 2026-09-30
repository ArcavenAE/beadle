# Declared category changes: let a run retire the headings it discovered

Date: 2026-09-30
Author: arcaven-architect-g5-0
Status: proposal. No code changed by this document.
Issue: #92. Builds on #88 (declared removals, for #87).

Operator, 2026-09-30, verbatim: "I worry this may lock in catagorizes as they
are now, which is not our intent, we will want to insert an LLM driven process
to discover what catagories are in use or beneficial given the body of issues,
which should already be supported in the skills-based version".

## 1. What is true today

| Claim | Where | Result |
|---|---|---|
| Check 2 fails every before-heading missing from the candidate unless it is a declared rename or removal | `crates/beadle/src/verify.rs`, "2. no section loss"; `tools/dashboard-gate.py` section 2 | true |
| Declarations are static: `DECLARED_REMOVALS` in the script, `removals` in `targets/<target>.verify.json` | #88 | true: a run cannot declare its own change without editing gate config |
| Check 3 compares issue numbers in the sentinel state JSON, as a union and per axis | `verify.rs` `nums`, `axes`, check 3 and 3b | true; it knows nothing about headings |
| The skills version discovers groupings | `skills/beadle-triage/SKILL.md` steps 3 and 7; the curated fixture `docs/fixtures/vsdd-factory-312-curated-run16.md` | **partly.** The classification axes are a bounded enum validated by `beadle classify ingest` against `vocabulary.json` ("the taxonomy holds by construction"). The P1 and P2 theme subsections are composed by the model each run: the fixture carries four P1 themes and three P2 groups whose names come from the corpus, not from a list |
| The skills flow expected the run to justify its own changes | `.claude/commands/dashboard-refresh.md`, gate item 2: "renames must be justified in the run summary" | true; the binary and script gates moved that justification into config |

So discovery already happens at the theme level, and the gate freezes it. The
bounded axes are a separate question: discovering a new classification axis
changes `vocabulary.json` and the ingest contract, which this proposal does not
touch.

## 2. The distinction the gate needs

Check 2 exists to catch lost issues. Heading text is a proxy for that. A
discovered category change is safe when no issue under a retired heading is
lost. So the gate needs two things it lacks: which headings a run may change,
and which issues sat under each heading.

**Two kinds of heading.**

- **Schema headings**: the fixed sections check 1 requires
  (`REQUIRED_SECTIONS`, `verify.rs`: Baseline, Needs human reading, Action
  plan, the P0a and P0b blocks, Quick wins, Direction Health, Classification
  index, Maintainer progress, Controls), the carried-forward run indexes under
  Classification index, and any other heading the target's static config
  names. A run can never retire these; only the static declarations from #88
  can.
- **Category headings**: the theme subsections that start with the P1 or P2
  priority marker (`### 🟠 P1`, `### 🟢 P`), whose text after the marker is the
  theme. Check 1 requires only that at least one of each exists, so a run may
  change which ones exist by declaration, and check 1 still fails a board left
  with none.

**Which issues sat under a heading.** The gate already holds both bodies. The
issues under a heading are the `#N` references between that heading and the
next heading of the same or higher level, so a `####` sub-heading inside a
theme stays part of that theme. A category heading with table rows or list
items but no extracted `#N` warns (`2: <heading> has rows but no issue
references`), since the extraction then has nothing to protect.

## 3. The rule

A category heading missing from the candidate passes check 2 with a warning
when both hold:

1. **The run declared it.** The candidate's sentinel state JSON carries
   `category_changes` for this run: a list of entries, each with `kind`
   (`retired`, `merged`, `renamed` or `split`), `from` (the before-heading's
   theme text), `into` (candidate headings), and `why`. `into` is non-empty
   for `merged`, `renamed` and `split`, and every `into` names a candidate
   category heading; an empty `into` for those kinds, or an `into` naming a
   schema heading, is a config error. The
   discovery step writes it; the run summary shows it. `category_changes` is a
   per-run key, so it joins `PER_RUN` and resets each run.
2. **No issue under it is lost.** Every issue referenced under the retired
   heading in the before body is referenced under a candidate P1 or P2
   category heading. A reference anywhere else does not count: not in the
   Classification index, the carried-forward run indexes, Baseline,
   Maintainer progress, any other schema heading, or the sentinel. For
   `merged`, `renamed` and `split`, each such issue must appear under one of
   the `into` headings. There is no exemption for an issue closed on GitHub:
   neither gate reads closure today (`verify.rs` and `dashboard-gate.py` have
   no closed state), and check 3 fails any number that leaves the state
   (section 7).

Otherwise it fails, as today:

- an undeclared missing heading fails `2: HEADING LOST`;
- a declared change that loses an issue fails with the issue numbers:
  `2: declared <kind> of <heading> lost issues [...]`;
- a declaration against a schema heading, in `from` or in `into`, is a
  config error, not a warning.

Declarations that name a heading the before-snapshot never carried warn as
stale, the same way #88's static removals do.

Checks 1, 3 and 3b are unchanged. A category change never excuses an issue
that leaves the state JSON.

## 4. Why this does not turn the gate into a formality

A run that declares its own changes could declare anything. The protection is
not the declaration, it is the coverage condition: a run can regroup freely
but cannot drop an issue by regrouping. Every declared change is printed in
the gate output and the run summary, so a reader sees each regroup and its
reason. The gate does not cap how many changes a run declares: a count limit
would be a health metric used as a gate (ADR-007), and a large regroup after
a discovery pass is the expected case.

## 5. Tests (red first on main)

1. A P1 theme renamed with a `renamed` declaration and all its issues under
   the new heading: pass, with one warning.
2. The same with no declaration: fails `HEADING LOST`.
3. Two P1 themes `merged` into one, one issue missing from the candidate:
   fails, naming that issue.
4. A `retired` theme whose issues all moved to other themes: passes. The same
   with one issue absent from every candidate category heading: fails, naming
   it, whether or not the issue is closed on GitHub.
5. A declaration against `## Controls`: config error. A `merged` whose `into`
   names `## Baseline`: config error. A `renamed` with an empty `into`: config
   error.
6. A `category_changes` entry naming a heading the before-snapshot lacks:
   stale warning.
7. `category_changes` from the previous run does not carry into this one
   (per-run key).
8. A retired theme whose issues survive only in the Classification index (or
   only in the sentinel): fails, naming them.
9. Extraction boundary: a theme with a `####` sub-heading holding issues
   counts those issues as the theme's; the next `###` theme ends it.
10. A category heading with rows but no `#N`: passes with the rows-without-
    references warning.
11. The Python gate and `beadle verify` agree on cases 1 to 10.

## 6. Edits, in order (none made by this PR)

| # | Edit | Depends on |
|---|---|---|
| C-1 | Schema versus category headings: the schema list from `REQUIRED_SECTIONS` plus static config; category headings by level and priority marker | none |
| C-2 | Issues-under-heading extraction from a body (same-or-higher-level boundary; the rows-without-references warning) | none |
| C-3 | `category_changes` read from the candidate state, added to `PER_RUN`; the rule in section 3 in `verify.rs` and `dashboard-gate.py` | C-1, C-2 |
| C-4 | The skills flow: the discovery step writes `category_changes` and the run summary shows it | C-3, and the discovery step itself |

The discovery step (which categories are in use or useful) is the operator's
planned LLM process and is not designed here; C-4 is only its interface.

## 7. Open questions

- Closure. An issue closed on GitHub has no gate-readable source today, for
  this rule or for check 3. If closed issues should be allowed to leave the
  board, the refresh would fetch a closed list into the candidate state and
  both checks would consult it. That is a separate change; until it exists,
  this rule grants no closure exemption.

- Whether a discovered change should also be proposed rather than applied the
  first time (the run posts, and a human confirms the regroup on the next
  run). Default offered: no; the coverage condition carries the safety, and
  the regroup is visible on the board.
