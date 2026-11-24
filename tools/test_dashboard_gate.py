#!/usr/bin/env python3
"""Tests for dashboard-gate.py's declared removals (beadle#87).

Run: python3 -m unittest tools/test_dashboard_gate.py

The gate is a script, not a module, so each case writes a before/candidate
pair to a temp dir and runs it. The gate's own allow-list is empty, so the
mechanism tests run a copy of the script whose DECLARED_REMOVALS literal is
replaced by a test-only list (see `patched_gate`); the 25010 tests run the
script as shipped.
"""
import ast
import json
import pathlib
import subprocess
import sys
import tempfile
import unittest

GATE = pathlib.Path(__file__).with_name("dashboard-gate.py")


def patched_gate(dest, decl):
    """Write a copy of the gate to `dest` with DECLARED_REMOVALS set to `decl`."""
    src = GATE.read_text()
    node = next(
        n for n in ast.parse(src).body
        if isinstance(n, ast.Assign)
        and any(isinstance(t, ast.Name) and t.id == "DECLARED_REMOVALS" for t in n.targets)
    )
    lines = src.split("\n")
    lines[node.lineno - 1:node.end_lineno] = ["DECLARED_REMOVALS = " + repr(decl)]
    dest.write_text("\n".join(lines))
    return dest


# A test-only allow-list for the mechanism tests; the shipped one is empty.
TEST_DECL = {
    "headings": {
        "### Retired lane": "test: lane retired",
        "## Retired section": "test: section retired",
    },
    "axes": {
        "retired_axis": "test: axis retired",
        "retired_group": "test: group retired",
    },
}


REQUIRED = [
    "## Baseline",
    "## 👤 Needs human reading",
    "## Action plan",
    "### 🔴 P0a",
    "### 🔴 P0b",
    "### 🟠 P1",
    "### 🟢 P2",
    "## 🟦 Quick wins",
    "## Direction Health",
    "## Classification index",
    "## Maintainer progress",
    "## Controls",
]


def body(state, extra_headings=()):
    sentinel = "<!-- beadle-state:v1\n" + json.dumps(state) + "\nbeadle-state -->"
    # every run's index is carried forward, as the gate requires
    idx = [f"### Run-{r} index" for r in range(1, state["run"] + 1)]
    heads = REQUIRED + idx + list(extra_headings)
    return "\n\n".join([sentinel, "**Direction verdict:** steady."] + heads) + "\n"


def run_gate(before, cand, titles=None, decl=None):
    """Run the gate. `titles` is a list written to a --titles file, or None.

    `decl` runs a copy of the gate with that DECLARED_REMOVALS; None runs it as shipped.
    """
    with tempfile.TemporaryDirectory() as d:
        b, c = pathlib.Path(d, "before.md"), pathlib.Path(d, "cand.md")
        b.write_text(before)
        c.write_text(cand)
        gate = GATE if decl is None else patched_gate(pathlib.Path(d, "gate.py"), decl)
        cmd = [sys.executable, str(gate), str(b), str(c)]
        if titles is not None:
            t = pathlib.Path(d, "titles.txt")
            t.write_text("".join(x + "\n" for x in titles))
            cmd += ["--titles", str(t)]
        p = subprocess.run(cmd, capture_output=True, text=True)
        return p.returncode, p.stdout + p.stderr


class Harness(unittest.TestCase):
    def test_positive_control_identical_bodies_pass(self):
        s = {"run": 1, "watermark": 10, "tracked": [5, 6]}
        rc, out = run_gate(body(s), body({**s, "run": 2, "watermark": 11}))
        self.assertEqual(rc, 0, out)

    def test_negative_control_an_undeclared_loss_fails_today(self):
        s = {"run": 1, "watermark": 10, "tracked": [5]}
        rc, out = run_gate(body(s, ["### Gone"]), body({**s, "run": 2, "watermark": 11}))
        self.assertEqual(rc, 1, out)
        self.assertIn("2: HEADING LOST -> ### Gone", out)


class DeclaredRemovals(unittest.TestCase):
    """The declared-removal mechanism, run against a test-only allow-list."""

    def setUp(self):
        self.decl = TEST_DECL

    def gate(self, before, cand):
        return run_gate(before, cand, decl=self.decl)

    def test_every_shipped_declared_removal_carries_a_why(self):
        shipped = gate_constants()["DECLARED_REMOVALS"]
        for kind in ("headings", "axes"):
            for name, why in shipped[kind].items():
                self.assertTrue(str(why).strip(), f"declared {kind} removal {name!r} has no why")

    def test_declared_heading_removal_warns_and_passes(self):
        heads = list(self.decl["headings"])
        s = {"run": 1, "watermark": 10, "tracked": [5]}
        rc, out = self.gate(body(s, heads), body({**s, "run": 2, "watermark": 11}))
        self.assertEqual(rc, 0, out)
        self.assertEqual(out.count("WARN 2: declared removal ->"), len(heads), out)
        self.assertNotIn("2: stale", out)

    def test_a_declared_removal_the_board_never_carried_warns_as_stale(self):
        s = {"run": 1, "watermark": 10, "tracked": [5]}
        rc, out = self.gate(body(s), body({**s, "run": 2, "watermark": 11}))
        self.assertEqual(rc, 0, out)
        self.assertEqual(
            out.count("WARN 2: stale declared removal"), len(self.decl["headings"]), out
        )
        self.assertEqual(out.count("WARN 3b: stale declared removal"), len(self.decl["axes"]), out)

    def test_a_declared_heading_still_in_the_candidate_warns(self):
        h = next(iter(self.decl["headings"]))
        s = {"run": 1, "watermark": 10, "tracked": [5]}
        rc, out = self.gate(body(s, [h]), body({**s, "run": 2, "watermark": 11}, [h]))
        self.assertEqual(rc, 0, out)
        self.assertIn("WARN 2: declared removal still present in candidate", out)

    def test_a_string_valued_key_counts_as_carried(self):
        # taxonomy_group is a string on the live boards, not an axis
        s = {"run": 1, "watermark": 10, "tracked": [5], "taxonomy_group": "ISO/IEC 25010:2023"}
        cand = {"run": 2, "watermark": 11, "tracked": [5]}
        rc, out = run_gate(body(s), body(cand))
        self.assertEqual(rc, 0, out)
        self.assertNotIn("key not in before-snapshot -> 'taxonomy_group'", out)

    def test_undeclared_heading_loss_still_fails(self):
        s = {"run": 1, "watermark": 10, "tracked": [5]}
        rc, out = self.gate(
            body(s, ["### Something undeclared"]), body({**s, "run": 2, "watermark": 11})
        )
        self.assertEqual(rc, 1, out)
        self.assertIn("2: HEADING LOST -> ### Something undeclared", out)

    def test_declared_axis_removal_covers_dict_sub_axes(self):
        before = {"run": 1, "watermark": 10, "tracked": [5, 6, 7]}
        for i, axis in enumerate(self.decl["axes"]):
            # one list-valued and one dict-valued shape, both must be exempt
            before[axis] = [5, 6] if i % 2 else {"groupA": [5], "groupB": [6, 7]}
        cand = {"run": 2, "watermark": 11, "tracked": [5, 6, 7]}
        rc, out = self.gate(body(before), body(cand))
        self.assertEqual(rc, 0, out)
        self.assertIn("WARN 3b: declared removal", out)

    def test_undeclared_axis_disappearance_still_fails(self):
        before = {"run": 1, "watermark": 10, "tracked": [5], "not_declared": [5]}
        cand = {"run": 2, "watermark": 11, "tracked": [5]}
        rc, out = self.gate(body(before), body(cand))
        self.assertEqual(rc, 1, out)
        self.assertIn("3b: cumulative axis 'not_declared' disappeared from state", out)

    def test_declared_axis_removal_does_not_excuse_dropped_issues(self):
        axis = next(iter(self.decl["axes"]))
        before = {"run": 1, "watermark": 10, "tracked": [5], axis: [5, 99]}
        cand = {"run": 2, "watermark": 11, "tracked": [5]}
        rc, out = self.gate(body(before), body(cand))
        self.assertEqual(rc, 1, out)
        self.assertIn("3: 1 tracked issue(s) dropped from state: [99]", out)


CI_RENAMES = [
    (
        "## Classification index (finding-005 + finding-009 + attn facet + IEEE 1044 + ODC + ISO/IEC 25010)",
        "## Classification index (finding-005 + finding-009 + attn facet + IEEE 1044 + ODC)",
    ),
    (
        "## Classification index (report type + defect nature + reproducibility + IEEE 1044 + ODC + ISO/IEC 25010)",
        "## Classification index (report type + defect nature + reproducibility + IEEE 1044 + ODC)",
    ),
]
P3_DEPENDENCY_VARIANT = (
    "### 🟢 P3: Papercuts, docs, design questions, and dependency bumps, "
    "grouped by ISO/IEC 25010 characteristic"
)


def gate_constants():
    """The allow-list constants of the gate, by name, without running the gate."""
    names = {"_RUN18", "_T25010", "_G25010", "DECLARED_RENAMES", "DECLARED_REMOVALS"}
    tree = ast.parse(GATE.read_text())
    keep = [
        n for n in tree.body
        if isinstance(n, ast.Assign)
        and any(isinstance(t, ast.Name) and t.id in names for t in n.targets)
    ]
    ns = {}
    exec(compile(ast.Module(body=keep, type_ignores=[]), str(GATE), "exec"), ns)
    return {k: v for k, v in ns.items() if k in ("DECLARED_RENAMES", "DECLARED_REMOVALS")}


QC_SECTION = "## Quality characteristic groups (ISO/IEC 25010:2023)"
GROUPED_LANES = [
    "### 🟢 P2: Correctness and operability defects, grouped by ISO/IEC 25010 characteristic",
    "### 🟢 P3: Papercuts, docs, tests, and design questions, grouped by ISO/IEC 25010 characteristic",
    "### 🟢 P2: Correctness and operability, grouped by ISO/IEC 25010 characteristic",
    "### 🟢 P3: Papercuts, docs, diagnostics and design questions, grouped by ISO/IEC 25010 characteristic",
    P3_DEPENDENCY_VARIANT,
]
ISO_MAP = {
    "Functional suitability": [5],
    "Reliability": [6, 7],
    "Security": [8],
    "Safety": [],  # a 2023 characteristic, empty until used, still carried
}


class Reversal25010(unittest.TestCase):
    """The operator reversed the 2026-09-30 drop: 25010 grouping is carried again.

    Dropping the section, a grouped lane, the index suffix or the `iso25010`
    state axis is a loss the gate fails, not a declared removal it excuses.
    """

    def setUp(self):
        self.s = {"run": 1, "watermark": 10, "tracked": [5, 6, 7, 8], "iso25010": ISO_MAP}
        self.c = {**self.s, "run": 2, "watermark": 11}

    def lost(self, out, heading):
        self.assertIn("2: HEADING LOST -> " + heading[:95], out)

    def test_dropping_the_quality_characteristic_section_fails(self):
        rc, out = run_gate(body(self.s, [QC_SECTION]), body(self.c))
        self.assertEqual(rc, 1, out)
        self.lost(out, QC_SECTION)

    def test_dropping_a_grouped_lane_fails_for_every_spelling(self):
        for lane in GROUPED_LANES:
            with self.subTest(lane=lane):
                rc, out = run_gate(body(self.s, [lane]), body(self.c))
                self.assertEqual(rc, 1, out)
                self.lost(out, lane)

    def test_retitling_the_index_without_the_25010_suffix_fails(self):
        for old, new in CI_RENAMES:
            with self.subTest(old=old):
                rc, out = run_gate(body(self.s, [old]), body(self.c, [new]))
                self.assertEqual(rc, 1, out)
                self.lost(out, old)

    def test_keeping_every_25010_heading_passes_without_an_allow_list_warning(self):
        heads = [QC_SECTION, *GROUPED_LANES, CI_RENAMES[0][0]]
        rc, out = run_gate(body(self.s, heads), body(self.c, heads))
        self.assertEqual(rc, 0, out)
        self.assertNotIn("declared removal", out)
        self.assertNotIn("declared rename", out)
        self.assertNotIn("stale declared removal", out)

    def test_dropping_the_iso25010_axis_fails_per_characteristic(self):
        cand = {k: v for k, v in self.c.items() if k != "iso25010"}
        rc, out = run_gate(body(self.s), body(cand))
        self.assertEqual(rc, 1, out)
        self.assertIn("3b: cumulative axis 'iso25010.Reliability' disappeared from state", out)
        self.assertNotIn("declared removal", out)

    def test_control_losing_one_issue_from_a_characteristic_fails(self):
        cand = {**self.c, "iso25010": {**ISO_MAP, "Reliability": [6]}}
        rc, out = run_gate(body(self.s), body(cand))
        self.assertEqual(rc, 1, out)
        self.assertIn("3b: axis 'iso25010.Reliability' lost 1 issue(s): [7]", out)

    def test_control_dropping_the_string_valued_taxonomy_group_is_not_an_axis_loss(self):
        # Pins today's behavior, unchanged here: a string key is not a cumulative axis.
        before = {**self.s, "taxonomy_group": "ISO/IEC 25010:2023 product quality characteristics"}
        rc, out = run_gate(body(before), body(self.c))
        self.assertEqual(rc, 0, out)

    def test_control_carrying_the_axis_whole_passes(self):
        rc, out = run_gate(body(self.s), body(self.c))
        self.assertEqual(rc, 0, out)

    def test_the_allow_lists_name_nothing_25010(self):
        consts = gate_constants()
        self.assertEqual(set(consts), {"DECLARED_RENAMES", "DECLARED_REMOVALS"})
        text = json.dumps(consts, ensure_ascii=False).lower()
        for needle in ("25010", "iso25010", "taxonomy_group"):
            self.assertNotIn(needle, text, f"the gate still declares {needle!r} as retired")


SKILL = pathlib.Path(__file__).resolve().parent.parent / "skills" / "beadle-triage" / "SKILL.md"
SLUG_TO_KEY = {
    "functional-suitability": "Functional suitability",
    "performance-efficiency": "Performance efficiency",
    "compatibility": "Compatibility",
    "interaction-capability": "Interaction capability",
    "reliability": "Reliability",
    "security": "Security",
    "maintainability": "Maintainability",
    "flexibility": "Flexibility",
    "safety": "Safety",
}


class SkillPinsTheSentinelSpelling(unittest.TestCase):
    """SKILL.md is read by the run that writes the state map; the gate (3b)
    compares keys by exact spelling, so the skill must pin the key for each
    slug, and must say an index title without the suffix is kept as is."""

    def setUp(self):
        text = SKILL.read_text()
        start = text.index("**Quality characteristic groups (ISO/IEC 25010:2023).**")
        self.para = text[start:text.index("\n\n**Row legibility", start)]

    def test_each_slug_maps_to_its_sentinel_key(self):
        for slug, key in SLUG_TO_KEY.items():
            with self.subTest(slug=slug):
                self.assertIn(f"`{slug}` -> `{key}`", self.para)

    def test_a_plain_classification_index_title_stays_as_it_is(self):
        self.assertIn("without the \"+ ISO/IEC 25010\" suffix", self.para)
        self.assertIn("keep it as it is", self.para)


# The em dash is built at run time so this file carries no raw character.
D = chr(0x2013 + 1)


def dashed(s):
    return s.replace("{D}", D)


class EmDashRetirement(unittest.TestCase):
    """Check 2's one-way retirement fold and check 9 (beadle-1dkpg)."""

    def setUp(self):
        self.s = {"run": 1, "watermark": 10, "tracked": [5]}
        self.c = {**self.s, "run": 2, "watermark": 11}

    def nine(self, out):
        return [l for l in out.splitlines() if "9: em dash in candidate" in l]

    def test_em_dash_retirement_warns_and_passes(self):
        rc, out = run_gate(
            body(self.s, [dashed("## Extras {D} safe to act on")]),
            body(self.c, ["## Extras: safe to act on"]),
        )
        self.assertEqual(rc, 0, out)
        self.assertIn("WARN 2: em dash retired -> ", out)

    def test_heading_retired_with_another_mark_fails(self):
        # Regression pin: the fold admits colon-space and nothing else.
        rc, out = run_gate(
            body(self.s, [dashed("## Extras {D} safe to act on")]),
            body(self.c, ["## Extras; safe to act on"]),
        )
        self.assertEqual(rc, 1, out)
        self.assertIn("2: HEADING LOST -> ## Extras", out)

    def test_declared_rename_target_matches_after_retirement(self):
        rc, out = run_gate(
            body(self.s, ["### Run-18 index (NEW)"]),
            body(self.c, ["### Run-18 index (carried forward: folded)"]),
        )
        self.assertEqual(rc, 0, out)
        self.assertIn("WARN 2: declared rename", out)

    def test_candidate_with_em_dash_is_refused(self):
        rc, out = run_gate(body(self.s), body(self.c) + dashed("Some prose {D} here\n"))
        self.assertEqual(rc, 1, out)
        self.assertEqual(len(self.nine(out)), 1, out)

    def test_candidate_without_em_dash_passes(self):
        rc, out = run_gate(body(self.s), body(self.c) + "Some prose; here\n")
        self.assertEqual(rc, 0, out)

    def test_quoted_title_with_em_dash_is_exempt(self):
        title = dashed("Fix parser {D} handle eof")
        cand = body(self.c) + "- #12 " + title + "\n"
        rc, out = run_gate(body(self.s), cand, [title])
        self.assertEqual(rc, 0, out)
        # Control: the same body without the title is refused.
        rc, out = run_gate(body(self.s), cand, [])
        self.assertEqual(rc, 1, out)

    def test_titles_file_absent_exempts_nothing(self):
        title = dashed("Fix parser {D} handle eof")
        cand = body(self.c) + "- #12 " + title + "\n"
        rc, out = run_gate(body(self.s), cand)
        self.assertEqual(rc, 1, out)
        self.assertEqual(len(self.nine(out)), 1, out)

    def test_dash_beside_quoted_title_is_refused(self):
        title = dashed("Fix parser {D} handle eof")
        cand = body(self.c) + "- #12 " + title + dashed(" {D} more\n")
        rc, out = run_gate(body(self.s), cand, [title])
        self.assertEqual(rc, 1, out)
        self.assertEqual(len(self.nine(out)), 1, out)

    def test_degenerate_titles_exempt_nothing(self):
        strings = [dashed(x) for x in ("{D}", " {D} ", "abcd {D} efgh", "x {D} y {D} z !!")]
        cand = body(self.c) + "".join(x + "\n" for x in strings)
        rc, out = run_gate(body(self.s), cand, strings + [""])
        self.assertEqual(rc, 1, out)
        self.assertEqual(len(self.nine(out)), 5, out)

    def test_dash_between_two_titles_is_refused(self):
        a, b = dashed("first {D} title one"), dashed("second {D} title two")
        cand = body(self.c) + a + dashed(" {D} ") + b + "\n"
        rc, out = run_gate(body(self.s), cand, [a, b])
        self.assertEqual(rc, 1, out)
        self.assertEqual(len(self.nine(out)), 1, out)

    def test_overlapping_title_occurrences_do_not_chain(self):
        title = dashed("ab {D} ab {D} ab")
        cand = body(self.c) + dashed("ab {D} ab {D} ab {D} ab\n")
        rc, out = run_gate(body(self.s), cand, [title])
        self.assertEqual(rc, 1, out)
        self.assertEqual(len(self.nine(out)), 1, out)


DOC = GATE.parent.parent / ".claude" / "commands" / "dashboard-refresh.md"
LANES_P2_P3 = [GROUPED_LANES[0], GROUPED_LANES[1]]


class ReclassificationRun(unittest.TestCase):
    """A run that regroups the same issues: run = before + 1 at the same watermark.

    The payload declares it with gate_kind "reclassification". An undeclared run
    at an unchanged watermark is still refused by check 1.
    """

    def setUp(self):
        self.s = {"run": 1, "watermark": 10, "tracked": [5, 6, 7, 8], "iso25010": ISO_MAP}
        self.before = body(self.s, [QC_SECTION] + LANES_P2_P3)
        self.c = {**self.s, "run": 2, "gate_kind": "reclassification"}

    def cand(self, state=None, extra=None):
        extra = [QC_SECTION] + LANES_P2_P3 if extra is None else extra
        return body(self.c if state is None else state, extra)

    def test_a_declared_reclassification_at_the_same_watermark_passes(self):
        rc, out = run_gate(self.before, self.cand())
        self.assertEqual(rc, 0, out)

    def test_the_declaration_is_what_lets_it_through(self):
        undeclared = {k: v for k, v in self.c.items() if k != "gate_kind"}
        rc, out = run_gate(self.before, self.cand(undeclared))
        self.assertEqual(rc, 1, out)
        self.assertIn("1: watermark must exceed 10, got 10", out)

    def test_a_declared_run_must_keep_the_watermark(self):
        for wm in (11, 9):
            with self.subTest(watermark=wm):
                rc, out = run_gate(self.before, self.cand({**self.c, "watermark": wm}))
                self.assertEqual(rc, 1, out)
                self.assertIn("1: reclassification run must keep watermark 10, got " + str(wm), out)

    def test_a_declared_run_must_still_be_before_plus_one(self):
        rc, out = run_gate(self.before, self.cand({**self.c, "run": 3}))
        self.assertEqual(rc, 1, out)
        self.assertIn("1: run must be 2, got 3", out)

    def test_dropping_an_open_row_fails(self):
        c = {**self.c, "tracked": [5, 6, 7], "iso25010": {**ISO_MAP, "Security": []}}
        rc, out = run_gate(self.before, self.cand(c))
        self.assertEqual(rc, 1, out)
        self.assertIn("3: 1 tracked issue(s) dropped from state: [8]", out)

    def test_the_quality_characteristic_section_is_required(self):
        rc, out = run_gate(self.before, self.cand(extra=LANES_P2_P3))
        self.assertEqual(rc, 1, out)
        self.assertIn("1: reclassification run requires section -> " + QC_SECTION, out)

    def test_each_grouped_lane_is_required(self):
        for lane, label in ((GROUPED_LANES[0], "P2"), (GROUPED_LANES[1], "P3")):
            with self.subTest(lane=label):
                extra = [QC_SECTION] + [x for x in LANES_P2_P3 if x != lane]
                rc, out = run_gate(self.before, self.cand(extra=extra))
                self.assertEqual(rc, 1, out)
                self.assertIn(f"1: reclassification run requires a grouped {label} lane", out)

    def test_the_iso25010_map_is_required(self):
        c = {k: v for k, v in self.c.items() if k != "iso25010"}
        rc, out = run_gate(self.before, self.cand(c))
        self.assertEqual(rc, 1, out)
        self.assertIn("1: reclassification run requires the iso25010 map in state", out)

    def test_an_unknown_kind_is_refused(self):
        rc, out = run_gate(self.before, self.cand({**self.c, "gate_kind": "regroup"}))
        self.assertEqual(rc, 1, out)
        self.assertIn("1: unknown gate_kind 'regroup'", out)

    def test_the_next_run_needs_no_declaration(self):
        nxt = {**self.s, "run": 3, "watermark": 12}
        rc, out = run_gate(self.cand(), self.cand(nxt))
        self.assertEqual(rc, 0, out)


class FirstRunReplace(unittest.TestCase):
    """Run 1 on a board that already holds a body, as when its store is cold-started.

    The payload declares gate_kind "first-run-replace" and carries a board_history
    note. Only checks 1 (run and watermark against the old body) and 2 (headings
    of the old body) are relaxed; the rest of the gate applies.
    """

    NOTE = "The store was started fresh on 2026-11-24; earlier runs live only in the old body."

    def setUp(self):
        self.old = {"run": 4, "watermark": 500, "tracked": [5, 6]}
        self.before = body(self.old, ["### Old lane", "## Old section"])
        self.c = {
            "run": 1, "watermark": 3, "tracked": [5, 6],
            "gate_kind": "first-run-replace", "board_history": self.NOTE,
        }

    def test_a_declared_first_run_over_an_existing_body_passes(self):
        rc, out = run_gate(self.before, body(self.c))
        self.assertEqual(rc, 0, out)
        self.assertNotIn("HEADING LOST", out)

    def test_a_body_with_no_sentinel_can_be_replaced(self):
        plain = "# Old board\n\n## Notes\n\nWritten by hand.\n"
        rc, out = run_gate(plain, body({**self.c, "tracked": []}))
        self.assertEqual(rc, 0, out)

    def test_undeclared_run_1_over_a_body_is_still_refused(self):
        undeclared = {k: v for k, v in self.c.items() if k not in ("gate_kind", "board_history")}
        rc, out = run_gate(self.before, body(undeclared))
        self.assertEqual(rc, 1, out)
        self.assertIn("1: run must be 5, got 1", out)
        self.assertIn("2: HEADING LOST -> ### Old lane", out)

    def test_the_board_history_note_is_required(self):
        for c in ({k: v for k, v in self.c.items() if k != "board_history"},
                  {**self.c, "board_history": "  "}):
            with self.subTest(note=c.get("board_history")):
                rc, out = run_gate(self.before, body(c))
                self.assertEqual(rc, 1, out)
                self.assertIn("1: first-run-replace requires a board_history note", out)

    def test_a_declared_first_run_must_be_run_1(self):
        rc, out = run_gate(self.before, body({**self.c, "run": 2}))
        self.assertEqual(rc, 1, out)
        self.assertIn("1: first-run-replace must be run 1, got 2", out)

    def test_the_candidate_still_needs_its_required_sections(self):
        rc, out = run_gate(self.before, body(self.c).replace("## Controls", "## Other"))
        self.assertEqual(rc, 1, out)
        self.assertIn("1: required section missing -> ## Controls", out)

    def test_an_em_dash_in_the_candidate_is_still_refused(self):
        rc, out = run_gate(self.before, body(self.c, ["### Lane " + D + " x"]))
        self.assertEqual(rc, 1, out)
        self.assertIn("9: em dash in candidate", out)

    def test_the_next_run_gets_the_ordinary_rules_back(self):
        first = body(self.c, ["### Fresh lane"])
        nxt = {"run": 2, "watermark": 4, "tracked": [5, 6]}
        rc, out = run_gate(first, body(nxt))
        self.assertEqual(rc, 1, out)
        self.assertIn("2: HEADING LOST -> ### Fresh lane", out)


class DocTellsARunHowToDeclare(unittest.TestCase):
    def test_the_refresh_doc_names_both_kinds_and_the_key(self):
        text = DOC.read_text()
        for needle in ("gate_kind", "reclassification", "first-run-replace", "board_history"):
            with self.subTest(needle=needle):
                self.assertIn(needle, text)


if __name__ == "__main__":
    unittest.main()
