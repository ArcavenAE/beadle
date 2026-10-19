# Brief: kill em dashes in beadle's rendered boards and triage skill

Status: design for review, 2026-10-19. Owner: the architect role. Docs only.
Issue: #117.
Commission: the operator's "kill emdashes" (2026-10-19), relayed by director.
Checked against beadle origin/main c1f95a5.

## Why

Every board beadle publishes carries em dashes, and the operator has ruled them
out of published text. The live vsdd-factory#312 body (run 21) holds 278 of
them (measured: `gh issue view 312 --json body`, counted for U+2014). Fixing the
sources once keeps every future board clean without hand-patching each run.

The cost is small but not zero: the first dash-free run fails today's
regression gate on every heading that loses its dash. This brief names the
smallest gate change that lets that run through without loosening the gate.

Measured below means a command was run or a file was read at c1f95a5; inferred
means reasoning from what was read.

## Premise check: which path reaches the live board

Two corrections to the commission's framing, both measured.

1. **The live board is curated, not rendered.** `.claude/commands/dashboard-refresh.md:2`
   and `:16-19` forbid `beadle render` and `beadle push` in the refresh; the body
   is written by the curating agent following the skill, with the highest
   committed fixture as "shape canon" (`dashboard-refresh.md:53-54`) and "carry
   forward all prior curated content" (`:92`). So the renderer's dashes reach a
   board only when the binary path is used. The live board's dashes come from
   the skill, the canon fixture, and content carried forward from the prior body.
2. **The issue title and H1 are already dash-free on the live board.** The live
   title is `📋 beadle · Triage Dashboard` and the run 21 H1 is
   `# 📋 beadle · Triage Dashboard · BOHICA-LABS/vsdd-factory` (measured). The
   skill (`SKILL.md:378`), `docs/dashboard-schema.md:55` and
   `prompts/create-dashboard.md:57` still name the old title with an em dash, so
   their "exact title" discovery key is already stale against the live issue.

Of run 21's 27 dash-bearing headings, 5 trace to skill text (Quick wins, Needs
human reading, Maintainer progress, Tier 1, Tier 2); the other 22 come from the
canon and carry-forward (measured by phrase search in the run 21 body).

## 1. render.rs and the rest of crates/

**render.rs: 31 em dashes, confirmed** (measured). Classified:

| class | count | lines |
|---|---|---|
| literal, reaches the rendered body | 7 | 245, 430, 617, 623, 660, 714, 780 |
| literal, stderr or error only | 4 | 105 (bail), 190 (WARN), 209, 216 (size report) |
| literal in test code | 1 | 1304 |
| comment or doc comment | 19 | 1, 5, 6, 34, 43, 54, 58, 83, 84, 95, 112, 158, 302, 329, 355, 412, 425, 1268, 1473 |

Director's "11 literals, 7 reaching a board" is right for non-test source; the
twelfth literal is a test assertion. Where the 7 land:

- 780: the H1 (`compose`).
- 245: `warmup` of the provisional run, serialized into the `beadle-state`
  sentinel (`build_sentinel`, render.rs:803), so it sits in the body inside an
  HTML comment. It reaches a board only through the binary path, which the
  curated refresh does not use (`dashboard-refresh.md:202`).
- 430: the Decay cell of the Clusters table (`decay_display`).
- 617, 623: Baseline table, Provenance column (617 only when a state is unreadable).
- 660: Top commenters list items.
- 714: Classification summary when the store holds no classifications. Inferred
  unreachable through `run()`, which bails first when the run has none
  (render.rs:103-109); reachable through `render_derived` directly.

**Other modules render too** (measured, non-comment lines only):

- `controls.rs:45, 49`: the Board controls block, emitted into every rendered
  body (render.rs `render_derived`). Two literals, 5 comments, 2 test literals.
  The extractor anchors on the `verb=` marker, not the dash (controls.rs:69-80),
  so old checked boxes with a dash still parse.
- `direction.rs`: 14 source literals, 1 test literal, 43 comments. Only
  `direction.rs:320` (filing-density rationale) reaches the rendered body,
  through `render_signal_list`. The other 13 (725 to 1431) are in the v2
  capacity-adjusted path (`compute_outcome`), printed by `beadle direction` as
  JSON; the curating agent reads that output and may quote it into the
  "Why this verdict" fold. Indirect, but still published text.
- Not board text: `push.rs` 3 literals (stderr), `verify.rs` 5 (gate report),
  `enumerate.rs` 2 (stderr), `vocab.rs:50` (error). `beadle-store`, `main.rs`,
  `migrate.rs`, `note.rs`, `sync.rs`, `classify.rs`, `intent.rs`: comments only.

**En dashes (U+2013) in crates/:** one, `direction.rs:1113`, a number range in a
v2 rationale (`~68` to `73%`). None in render.rs. The live board holds 18 and
SKILL.md 8, all ranges. The ruling names em dashes; see Open questions.

Adjacent, out of scope: render.rs:693 emits a unicode ellipsis in long cluster
member lists, which AGENTS.md lists as a tell to sweep.

## 2. The skill and the files that shape the body

**skills/beadle-triage/SKILL.md: 132, confirmed** (measured), one per line
except lines 170, 185, 337, 355 (two each).

| class | count | lines | must change |
|---|---|---|---|
| A. text that lands in the body or title | 7 | 378, 419, 449, 483, 526, 560, 562 | yes |
| B1. prose in steps 7 and 8, the body-writing instructions | 44 | 377 to 570, less class A | yes, they model the style |
| B2. prose elsewhere in the skill | 81 | the rest | for consistency |

Class A: 378 is the title; 419 and 483 name "Quick wins" with a dash, as the
board heading does; 449 names the lane; 526 is the Maintainer progress heading
the board carries verbatim; 560 and 562 are the Tier 1 and Tier 2 headings the
board carries verbatim (run 21 lines 778, 820).

The skill has no rule about dashes, and that gap matters more than any one
line: the curator copies the canon and carries forward the prior body, both
dash-bearing. Step 7 (after SKILL.md:415) needs a new rule:

> No em dash (U+2014) anywhere in the body, including text carried forward from
> the prior body and text taken from the shape canon. In a `##` or `###`
> heading, replace each spaced em dash with a colon and a space. Elsewhere use a
> period, semicolon, colon, comma or parentheses, never an en dash or a double
> hyphen.

Other files (measured counts):

- `.claude/commands/dashboard-refresh.md`: 28. None copied into the body, but
  lines 53-54 (shape canon) and 92 (carry forward) need the same rule by
  pointer, since the canon stays dash-bearing until the new fixture lands.
- `docs/dashboard-schema.md`: 52. Lines 55 (title), 197 and 202 (Tier heading
  templates) are class A; the rest is prose.
- `docs/example-dashboard.md`: 12, all in an example body, so class A in effect.
- `prompts/create-dashboard.md`: 16. Line 57 (`TITLE=`) is class A; it sets the
  create-time title.

Title: every file switches to the live form, `📋 beadle · Triage Dashboard`, and
the discovery text accepts the old em-dash title as a second spelling until no
open dashboard carries it. The binary is unaffected: its search is
`beadle Triage Dashboard in:title` filtered on the sentinel (push.rs:169-178).

## 3. Replacements

One rule per pattern, applied everywhere. Before-lines show the character as
`\u{2014}`.

| pattern | before | after |
|---|---|---|
| P1. clause break in a sentence or table cell | `count(comment_event) \u{2014} {} issues touched` | `count(comment_event); {} issues touched` |
| P2. label, then its value or description | ``- `{}` \u{2014} {}`` (render.rs:660, controls.rs:49) | ``- `{}`: {}`` |
| P3. state word, then why | `unmaintained \u{2014} no cluster record since run {}` | `unmaintained: no cluster record since run {}` |
| P4. heading separator | `## 🟦 Quick wins \u{2014} safe to act on (...)` | `## 🟦 Quick wins: safe to act on (...)` |
| P5. paired dashes around an aside, prose only | `issues \u{2014} even most P0s \u{2014} are` | `issues (even most P0s) are` |
| P6. title and H1 | `# 📋 beadle \u{2014} Triage Dashboard · {}` | `# 📋 beadle · Triage Dashboard · {}` |

P1 covers render.rs 105, 190, 209, 216, 617, 623, 714, controls.rs:45 and
direction.rs:320. P3 covers render.rs 245 and 430 and the direction.rs v2
prefixes (`falsified:`, `intact:`, `none:`, `pending:`). P4 is fixed at
colon-space because the gate change in section 4 accepts exactly that, which
keeps the gate mechanical; inside headings it applies even to an aside
(`(carried forward: folded)`).

P6 is the one place the replacement is not on the five-mark list. The middle
dot is the board's own separator, and the live title and H1 already use it
(measured), so a colon would change a heading the live board has settled. The
operator may overrule; if so, a colon goes in all title and H1 sites at once.

One wording trap, measured: render.rs:1443 asserts `advisory only` with exact
case. Writing render.rs:216 as `. Advisory only;` failed that test; `; advisory
only:` passes.

## 4. Tests and fixtures

Measured in a scratch worktree at c1f95a5: baseline `cargo test --workspace`
passes (204 tests). With the replacements applied to the 7 rendered render.rs
literals, the 4 stderr ones, controls.rs (2), all 14 direction.rs source
literals, and every em dash in SKILL.md, it still passes.

| test | reads | verdict |
|---|---|---|
| `tests/verify_gate.rs` | run18 and run19 fixtures (lines 19-20) | passes unchanged; fixtures untouched |
| `tests/vocabulary_contract.rs` | SKILL.md text; only `*-classifications-rich.json` in docs/fixtures (lines 123-131) | passes; `impact.` token scan unaffected |
| `tests/classification_fidelity.rs` | run18 rich JSON only | passes; no renderer or skill input |
| `tests/direction_v2.rs` | v2 outcome fields | passes; no assertion on a dashed string |
| render.rs unit tests | rendered body, decay text | pass; 1304 is a negative check on a killed table, leave it |
| controls.rs:119, 126 and push.rs:298 | dash-bearing checked boxes as input | pass; keep them, they prove old boxes still parse |
| `tools/test_dashboard_gate.py` | the Python gate | passes (`python3 -m unittest`) |

The fixtures run11 to run19 are not edited. Nothing in the suite compares
rendered output to a curated fixture.

**What does fail is the gate, on the first dash-free run.** Measured on the
run18 to run19 pair only: run18 as before, run19 with every em dash replaced as
candidate. Both `beadle verify` and
`tools/dashboard-gate.py` fail with 24 `2: HEADING LOST` lines and nothing else.
Sentinel, coverage, render-integrity and attn checks all pass. Heading check 2
is exact-line (verify.rs:594-616, dashboard-gate.py:105-114). The 24 include the
declared rename `### Run-18 index (NEW)`, whose `to` carries a dash in
`targets/vsdd-factory.verify.json` and `DECLARED_RENAMES`. The run 22 refresh
would hit the same wall against the run 21 body. That is inferred from the
same mechanism; run 21 to run 22 was not measured, since run 22 does not exist.

Options:

- (a) Declare each retired heading as a rename. About 24 entries per target, in
  two places, each a one-off; the allowlist turns into a changelog.
- (b) Strip dashes from both sides before comparing. Too loose: a candidate
  that keeps its dashes would also pass, and so would any separator.
- (c) A one-way retirement fold. A before-heading absent from the candidate is
  matched when the candidate holds it with every ` \u{2014} ` replaced by `: `.
  It warns `2: em dash retired -> <heading>` instead of failing. The same fold
  applies to a declared rename's `to`. Fixtures and allowlists stay byte-exact.
- (d) Leave the gate alone and pin new expectations. Does not get run 22 posted.

**Recommendation: (c), in both gates.** Prototyped in scratch (measured): the
dash-retired run19 candidate gives `GATE PASSED` with 22 retirement warnings in
each gate. A candidate using `; ` in headings still fails all 24, so the fold
admits only P4. All Rust and Python tests still pass. Once before-snapshots are
dash-free the fold is a no-op; it stays for any other target's first dash-free
run (prism).

## 5. Guards against regression

Structural, so they may gate (ADR-007 and the diagnostic-not-gate rule: they
ask "is this artifact well-formed against a written rule", not "is the project
healthy"):

**Which guard, and why.** Three review rounds each found new edges in a
hand-written Rust lexer that marks off test code. A source scan also has a
limit no lexer removes: code can build the character from arithmetic, which no
scan of spellings can see. So the design splits the job. The output and data
checks below carry the guarantee wherever a test can reach the text. The
source scan is a tripwire for the spellings people write by accident, with no
test-code boundary at all. Of the reviewer's three options it takes the third,
refuse everywhere and build the dash at run time in tests, because that
removes the lexer instead of growing it. The first option, checking outputs, is
used for the body and the data files. The second, a real parser through `syn`,
is not taken: it would add a dev-dependency to close edges that the run-time
construction makes moot.

- `tests/no_em_dash.rs::sources_have_no_em_dash_spelling`, the tripwire. It
  reads every `.rs` file under `crates/` and refuses, in code, comments and tests
  alike, with no test-code boundary:
  - the raw U+2014 character;
  - a `\u{...}` escape whose hex digits, with underscores removed, equal
    `2014` after any leading zeros, in either case (`\u{2014}`, `\u{02014}`,
    `\u{20_14}`, `\U{2014}` as text);
  - an integer literal whose value is 0x2014 (8212), whatever its base or
    underscores: `0x2014`, `0x02014`, `0x20_14`, `8212`, `8_212`, `0o20024`,
    `0b10000000010100`, with or without a type suffix;
  - the UTF-8 bytes as a byte escape (`\xe2\x80\x94`, either case) or as three
    adjacent integer literals of value 0xE2, 0x80, 0x94 in a list;
  - the HTML entities `&mdash;`, `&#8212;` and `&#x2014;`, in either case,
    with leading zeros.
  Tests that need the character get it from one helper,
  `tests/support/mod.rs::em_dash()`, which returns
  `char::from_u32(0x2013 + 1).unwrap()`. Each literal there is one off, so the
  value rules pass it. The helper is the only sanctioned construction, and the
  scan enforces that. Outside `tests/support/mod.rs` it refuses any call that
  builds a char from an integer: `from_u32`, `from_u32_unchecked`, and
  `char::try_from` or `char::from` applied to a `u32`, matched as text. That
  costs nothing on main, which has no `from_u32` under `crates/` (`git grep`);
  the `try_from` impls in `director-envelope` are not on `char` and do not match.
  What remains is bytes built at run time and passed to `String::from_utf8`.
  The scan cannot refuse `from_utf8`, since gh.rs:23 needs it, so that case
  stays a review matter, and the brief says so. The scan is a tripwire against
  accidental spellings, not proof against deliberate construction. The inputs that must carry a dash (the old checked boxes in
  controls.rs:119 and 126, push.rs:298, render.rs:1549) and the five test files
  that hold raw dashes today move to the helper, which needs `pub` access from
  unit tests; a `#[cfg(test)]` copy in `src/` is the fallback.
- `scan_catches_planted_spelling`: the scan is a function over (path, text),
  and this test runs it on in-memory sources, one case per form in the list
  above, including each named variant (leading zeros, upper case, an
  underscore in the escape, decimal, octal, binary, a suffix, the byte list).
  Each case must be refused, and the failure must name the form and the line.
  Controls that must pass: `&ndash;`, `&#8211;`, `&#x2013;`, `0x2013`, `8211`, `0x2015`, the helper's own
  expression, and the bytes `0xe2, 0x80, 0x93` (the en dash). The scan must
  also report at least one file read under each crate, so an empty walk fails
  instead of passing.
- `tests/no_em_dash.rs::body_data_has_no_em_dash`, an output-side check over
  the data that reaches the body and is not Rust:
  `skills/beadle-triage/vocabulary.json` (compiled in by `include_str!` at
  vocab.rs:15, gates `report_type` at classify.rs:202, and printed into the
  body at render.rs:731-737; 0 dashes at c1f95a5) and every
  `targets/*.intent.yaml` (its `repo` becomes the H1 through render.rs:132 and
  :780, and the skill scores the manifest, so the curator may quote its
  values). The intent files hold 33 characters on 31 lines at c1f95a5:
  `_template` 7 on 7 lines and `prism` 2 on 1, all in comments, and
  `vsdd-factory` 24 on 23 lines, 8 of those lines values. All 33 are clause
  breaks or asides; none is a range. The check reads each file as text, so
  comments count too.
  **YAML rule:** in these files only P1 (semicolon) or P5 (parentheses) may
  replace a dash, never colon-space. A colon-space in an unquoted value breaks
  the YAML: vsdd-factory.intent.yaml:124, `note: canonical principle \u{2014}
  production-grade default, ...`, fails to parse with `: ` and parses with
  `; ` (measured by the reviewer). After the replacement every intent file
  must still load through the manifest loader (`intent.rs`, `serde_yaml`).
  The test `intent_files_parse_after_retirement` loads each one, and that
  test runs in the same red/green sequence.
- **The candidate-body check, both gates:** the live path is the curated
  body, not the renderer, so the guarantee has to sit where that body is
  refused before it posts. `beadle verify` (verify.rs `evaluate`) and
  `tools/dashboard-gate.py` each gain a check, `9: em dash in candidate`. It
  fails the gate for any U+2014 in the candidate body, with a line number,
  except inside a quoted title (below). The before-snapshot is not checked,
  since history keeps its dashes. Tests: `candidate_with_em_dash_is_refused`
  and `candidate_without_em_dash_passes` in verify.rs, the same pair in
  `tools/test_dashboard_gate.py`, and `quoted_title_with_em_dash_is_exempt`.
  This check, not the later fixture test, is what stops curator prose,
  carry-forward text and canon text from posting with a dash.
- **Quoted titles.** An issue or PR title is someone else's words. The
  default rule: a title is quoted exactly as written and is exempt, but only
  as an exact substring match against a title the store holds for that target
  (`beadle-store` lib.rs:72, `title`). The exemption is mechanical and needs
  no markup, so a dash the curator writes beside a title is still refused.
  `beadle verify` reads the titles from the target's store. The Python gate
  takes `--titles FILE`, one title per line, written by a new read-only
  `beadle titles <target>`. Without the file it exempts nothing, so it fails
  closed. The alternative, rewriting dashes inside quoted titles, changes
  another author's words. It is listed for the operator under Open questions.
- `render::tests::rendered_body_has_no_em_dash`, the behavioral check on the
  body: renders a store that reaches the body branches (a stale cluster, an
  unreadable issue state, one commenter, a zero trailing mean for filing
  density, the provisional run's `warmup` in the sentinel, line 245) and asserts
  no U+2014 in the body. A second case calls `render_derived` on a store with
  no classifications, to reach line 714, which `run()` cannot (render.rs:103-109).
  Positive control, measured: a prototype failed with the dash put back in the
  H1, naming body line 0.
- `controls::tests::board_controls_block_has_no_em_dash`.
- `tests/no_em_dash.rs::body_shaping_text_has_no_em_dash`: SKILL.md,
  `.claude/commands/dashboard-refresh.md`, `docs/dashboard-schema.md`,
  `docs/example-dashboard.md`, `prompts/create-dashboard.md`. Exempt by name:
  `docs/fixtures/` (history), `targets/*.verify.json` and
  `tools/dashboard-gate.py` (they quote historical headings, which must keep
  the dash to match). Doc text that names the old title writes "an em dash
  (U+2014)" in words instead of the character.

The guards cover U+2014 only, matching the ruling. Comments and test code are
covered by the spelling tripwire. A test input that needs the character gets
it from the `em_dash()` helper.

## 6. The replacement fixture and #115

The fixture cannot exist until a run uses the new skill, so it is a second PR,
after the builder PR merges.

1. The next scheduled refresh (run 22, unless another posts first) runs the
   new skill against the live run 21 body, passes the gate through the fold,
   and posts.
2. Per `dashboard-refresh.md` section 4 step 4, its body is committed as
   `docs/fixtures/vsdd-factory-312-curated-run<N>.md`, N from its sentinel.
3. That PR moves the shape canon pointer (`dashboard-refresh.md:53-54`) to
   run<N>, and adds `tests/no_em_dash.rs::newest_curated_fixture_has_no_em_dash`
   (the highest-numbered curated fixture per target holds no U+2014; older ones
   are history and exempt).
4. `vocabulary_contract.rs` needs no change: it reads only rich JSON. A rich
   JSON committed with the run must be ingestable, as now.
5. A real-pair gate test needs the run 21 body as its before, since the gate
   requires `run = before + 1` (verify.rs:513). The run 21 body exists only
   in #115. I recommend the replacement PR carry it byte-identical to #115's
   file, so `verify_gate.rs::the_run21_to_run22_pair_passes` proves the first
   dash-free transition on real bodies. If the operator prefers to drop run 21,
   the mutation test from section 7 stands alone.
6. #115 is closed as superseded only after the replacement PR is open, with a
   comment naming it. Nobody closes it early.

## 7. Build items: one builder red/green draft PR

Red first; each test is run and seen failing before its fix.

1. **Red, source, data and renderer guards:** `sources_have_no_em_dash_spelling`,
   `scan_catches_planted_spelling` (red until the scan function exists),
   `body_data_has_no_em_dash`, `intent_files_parse_after_retirement`,
   `rendered_body_has_no_em_dash` (both cases), `board_controls_block_has_no_em_dash`.
   The direction and stderr literals are covered by the source scan, not by a
   behavioral test.
2. **Red, text guard:** `tests/no_em_dash.rs::body_shaping_text_has_no_em_dash`.
3. **Red, gate:** verify.rs unit tests `retired_em_dash_heading_warns_instead_of_failing`
   and `declared_rename_target_matches_after_retirement`; verify_gate.rs
   `the_first_dash_free_run_passes_with_retirement_warnings` (run18 before;
   run19 mutated in the test, headings by P4, prose by P1, in the house style of
   its other controls); Python `test_em_dash_retirement_warns_and_passes`.
   The candidate-body check: `candidate_with_em_dash_is_refused`,
   `candidate_without_em_dash_passes` (the positive control) and
   `quoted_title_with_em_dash_is_exempt`, in verify.rs and in the Python tests,
   with a Python case that `--titles` absent exempts nothing.
   Two regression pins go with them and pass on main, so they are not reds:
   `a_heading_retired_with_a_semicolon_is_still_lost` and Python
   `test_heading_retired_with_another_mark_fails`. They are kept so the fold
   cannot widen past colon-space without a test failing.
4. **Green, renderer:** render.rs 11 source literals, controls.rs 2,
   direction.rs 14, by section 3. Keep `advisory only` lowercase. Retire the
   comment dashes across `crates/` too, since the source scan covers them, and
   build any dash a test input needs with the `em_dash()` helper. At c1f95a5 five test files
   hold raw dashes: classification_fidelity.rs (2), direction_v2.rs (9),
   run_ordering.rs (1), verify_gate.rs (4) and vocabulary_contract.rs (3)
   (`git grep -c` for U+2014). Also the 33 in `targets/*.intent.yaml`, by P1 or P5 only.
5. **Green, gates:** the fold in `verify.rs` check 2 and `tools/dashboard-gate.py`
   check 2, with the rename-`to` case; check 9 (em dash in candidate) in both,
   with the store-title exemption; `beadle titles <target>`, and
   `dashboard-refresh.md` passing `--titles` to the Python gate. `verify.json` and `DECLARED_RENAMES`
   unchanged.
6. **Green, text:** SKILL.md (132, plus the step 7 rule), dashboard-refresh.md
   (28, plus the canon and carry-forward pointer), dashboard-schema.md (52),
   example-dashboard.md (12), create-dashboard.md (16). Title text to the live
   form, with the old spelling accepted for discovery.
7. **Verify:** `ax test beadle` and `python3 -m unittest tools/test_dashboard_gate.py`;
   `git diff --stat origin/main -- docs/fixtures` prints nothing.

The commit message and PR body carry no em dash, no banned vocabulary and no AI
attribution; the builder sweeps them before pushing. The PR uses `Refs: #117`,
since the fixture PR finishes it.

## Open questions

The design assumes both of these as written below until director rules. Both
are carded and not ruled.


- **En dashes (assumed: they stay).** 18 on the live board, 8 in SKILL.md, 1 in direction.rs, all
  number ranges. The ruling names em dashes. Writing "#410 to #417" is a
  one-line skill rule if the operator wants it; until then the guard ignores them.
- **Quoted titles (assumed: quoted as written, exempt by exact store match).**
  Rewriting them instead changes another author's words; that is the
  alternative, and it is the operator's call.
- **P6 separator (assumed: the middle dot).** Middle dot, matching the live board, or colon, matching
  the five-mark list. This brief recommends the middle dot.

This recommendation is valid until 2026-10-26 or the next vsdd-factory refresh,
whichever comes first; the architect role re-checks it then.
