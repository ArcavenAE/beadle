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
    tree = ast.parse(GATE.read_text())
    for node in tree.body:
        if isinstance(node, ast.Assign) and any(
            isinstance(t, ast.Name) and t.id == "DECLARED_REMOVALS" for t in node.targets
        ):
            return ast.literal_eval(node.value)
    return None


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


def run_gate(before, cand):
    with tempfile.TemporaryDirectory() as d:
        b, c = pathlib.Path(d, "before.md"), pathlib.Path(d, "cand.md")
        b.write_text(before)
        c.write_text(cand)
        p = subprocess.run(
            [sys.executable, str(GATE), str(b), str(c)], capture_output=True, text=True
        )
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
        s = {"run": 1, "watermark": 10, "tracked": [5]}
        rc, out = run_gate(body(s, heads), body({**s, "run": 2, "watermark": 11}))
        self.assertEqual(rc, 0, out)
        self.assertEqual(out.count("WARN 2: declared removal"), len(heads), out)

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


if __name__ == "__main__":
    unittest.main()
