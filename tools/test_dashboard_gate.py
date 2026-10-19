#!/usr/bin/env python3
"""Tests for dashboard-gate.py's declared removals (beadle#87).

Run: python3 -m unittest tools/test_dashboard_gate.py

The gate is a script, not a module, so each case writes a before/candidate
pair to a temp dir and runs it. The declared entries are read from the
script's own DECLARED_REMOVALS literal, so these tests exercise whatever the
script declares rather than a copy of it.
"""
import ast
import json
import pathlib
import subprocess
import sys
import tempfile
import unittest

GATE = pathlib.Path(__file__).with_name("dashboard-gate.py")


def declared_removals():
    # Run only the constant assignments the allowlist is built from, not the
    # gate itself (which reads argv at import).
    tree = ast.parse(GATE.read_text())
    keep = [
        n for n in tree.body
        if isinstance(n, ast.Assign)
        and any(isinstance(t, ast.Name) and t.id in ("_G25010", "DECLARED_REMOVALS") for t in n.targets)
    ]
    ns = {}
    exec(compile(ast.Module(body=keep, type_ignores=[]), str(GATE), "exec"), ns)
    return ns.get("DECLARED_REMOVALS")


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


def run_gate(before, cand, titles=None):
    """Run the gate. `titles` is a list written to a --titles file, or None."""
    with tempfile.TemporaryDirectory() as d:
        b, c = pathlib.Path(d, "before.md"), pathlib.Path(d, "cand.md")
        b.write_text(before)
        c.write_text(cand)
        cmd = [sys.executable, str(GATE), str(b), str(c)]
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
    def setUp(self):
        self.decl = declared_removals()
        self.assertIsNotNone(self.decl, "DECLARED_REMOVALS is not defined in dashboard-gate.py")

    def test_every_declared_removal_carries_a_why(self):
        for kind in ("headings", "axes"):
            self.assertTrue(self.decl.get(kind), f"no declared {kind}")
            for name, why in self.decl[kind].items():
                self.assertTrue(str(why).strip(), f"declared {kind} removal {name!r} has no why")

    def test_declared_heading_removal_warns_and_passes(self):
        heads = list(self.decl["headings"])
        self.assertTrue(heads, "no declared headings; this test would pass vacuously")
        s = {"run": 1, "watermark": 10, "tracked": [5]}
        rc, out = run_gate(body(s, heads), body({**s, "run": 2, "watermark": 11}))
        self.assertEqual(rc, 0, out)
        self.assertEqual(out.count("WARN 2: declared removal ->"), len(heads), out)
        self.assertNotIn("2: stale", out)

    def test_a_declared_removal_the_board_never_carried_warns_as_stale(self):
        s = {"run": 1, "watermark": 10, "tracked": [5]}
        rc, out = run_gate(body(s), body({**s, "run": 2, "watermark": 11}))
        self.assertEqual(rc, 0, out)
        self.assertEqual(
            out.count("WARN 2: stale declared removal"), len(self.decl["headings"]), out
        )
        self.assertEqual(out.count("WARN 3b: stale declared removal"), len(self.decl["axes"]), out)

    def test_a_declared_heading_still_in_the_candidate_warns(self):
        h = next(iter(self.decl["headings"]))
        s = {"run": 1, "watermark": 10, "tracked": [5]}
        rc, out = run_gate(body(s, [h]), body({**s, "run": 2, "watermark": 11}, [h]))
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
        rc, out = run_gate(
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
        rc, out = run_gate(body(before), body(cand))
        self.assertEqual(rc, 0, out)
        self.assertIn("WARN 3b: declared removal", out)

    def test_undeclared_axis_disappearance_still_fails(self):
        before = {"run": 1, "watermark": 10, "tracked": [5], "not_declared": [5]}
        cand = {"run": 2, "watermark": 11, "tracked": [5]}
        rc, out = run_gate(body(before), body(cand))
        self.assertEqual(rc, 1, out)
        self.assertIn("3b: cumulative axis 'not_declared' disappeared from state", out)

    def test_declared_axis_removal_does_not_excuse_dropped_issues(self):
        axis = next(iter(self.decl["axes"]))
        before = {"run": 1, "watermark": 10, "tracked": [5], axis: [5, 99]}
        cand = {"run": 2, "watermark": 11, "tracked": [5]}
        rc, out = run_gate(body(before), body(cand))
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


class Transition25010(unittest.TestCase):
    """One-time ISO/IEC 25010 transition entries, pinned by their exact text."""

    def setUp(self):
        self.s = {"run": 1, "watermark": 10, "tracked": [5]}
        self.c = {**self.s, "run": 2, "watermark": 11}

    def test_the_dependency_bumps_p3_variant_is_a_declared_removal(self):
        rc, out = run_gate(body(self.s, [P3_DEPENDENCY_VARIANT]), body(self.c))
        self.assertEqual(rc, 0, out)
        self.assertIn("WARN 2: declared removal -> '### 🟢 P3: Papercuts, docs, design questions, and d", out)

    def test_each_classification_index_retitle_is_a_declared_rename(self):
        for old, new in CI_RENAMES:
            with self.subTest(old=old):
                rc, out = run_gate(body(self.s, [old]), body(self.c, [new]))
                self.assertEqual(rc, 0, out)
                self.assertIn("WARN 2: declared rename", out)

    def test_a_retitle_to_anything_else_still_fails(self):
        old, _ = CI_RENAMES[0]
        rc, out = run_gate(body(self.s, [old]), body(self.c, ["## Classification index (other)"]))
        self.assertEqual(rc, 1, out)
        self.assertIn("2: HEADING LOST -> " + old[:60], out)

    def test_the_rename_warning_does_not_claim_the_run18_precedent(self):
        old, new = CI_RENAMES[0]
        rc, out = run_gate(body(self.s, [old]), body(self.c, [new]))
        line = next(l for l in out.splitlines() if l.startswith("WARN 2: declared rename"))
        self.assertNotIn("run-18", line)


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


if __name__ == "__main__":
    unittest.main()
