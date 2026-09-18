#!/usr/bin/env python3
"""Restore short_title / triage_state / attn / possibly_fixed to store rows.

beadle#66 (bd aae-orc-l5b5i): `classify ingest` dropped these four fields on
every classification it ever wrote, because `ClassificationRecord` did not
declare them and serde ignores unknown keys. The binary now carries them, but
the rows already in `store/<target>/state.jsonl` were written by the lossy
path. The git-tracked rich fixtures under `docs/fixtures/` still hold the data;
this script puts back what it can prove.

Guarantees
----------
* The store is backed up to `state.jsonl.bak-backfill-<utc>` before any write,
  and only when there is something to write.
* Rows are never reordered and never dropped. Output has exactly as many lines
  as input, in the same order.
* Lines the script does not repair are copied through byte-for-byte.
* Only the four fields are ever written. No other key is added, changed or
  removed — asserted per row before the line is emitted.
* Idempotent: a value already equal to the fixture's is not a repair, so a
  second run reports 0 and rewrites nothing.
* A record it cannot attribute with certainty is SKIPPED and reported by issue
  number. A mis-attributed classification is worse than a missing one.

Matching (`--match`, default `strict`)
--------------------------------------
`strict` requires each fixture record to carry its own `run`, and matches on
`number` + `run`. Only the run-17/18/19 fixtures qualify; the run-12/13/14/15
fixtures carry neither `run` nor `ts`, so those four are skipped wholesale and
their issue numbers are listed.

`number` opts into matching on `number` alone. It is gated on two facts that
the script re-derives and re-checks on every run, refusing to proceed if either
stops holding:

  1. no issue number appears in more than one fixture, and
  2. no issue number is classified in more than one store run,

which together make `number` a key. It also prints the fixture-to-store-run
mapping it derives, because the filename's run is NOT the store's run for the
older fixtures (run12 -> store 10, run13 -> store 9, run14 -> store 13).

attn normalization
------------------
The same facet appears three ways in the fixtures. All three are folded into
the store's `{subtype, order, why}` — but only when the full triple is present
and unambiguous. Anything else skips the record and reports it by number,
because a wrong `attn` puts an issue into or out of the human reading lane.

    run 12:     "governance" + sibling attn_reading_order / attn_why keys
    run 14:     {"type": ..., "reason": ..., "reading_order": ...}
    run 18/19:  {"subtype": ..., "order": ..., "why": ...}
"""

from __future__ import annotations

import argparse
import datetime as dt
import glob
import json
import os
import re
import shutil
import sys
import textwrap
from collections import Counter, defaultdict

FIELDS = ("short_title", "triage_state", "attn", "possibly_fixed")
ATTN_CANON = ("subtype", "order", "why")
ATTN_ALIASES = {"subtype": "type", "order": "reading_order", "why": "reason"}
REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def canonical(obj) -> str:
    """Match `beadle_store::to_canonical_json`: sorted keys, compact, UTF-8."""
    return json.dumps(obj, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


class Ambiguous(Exception):
    """The record's attn cannot be normalized without guessing."""


def normalize_attn(rec: dict):
    """Fold the three historical attn encodings into {subtype, order, why}.

    Returns None when the record is not in the lane. Raises `Ambiguous` rather
    than filling a gap with a guess.
    """
    a = rec.get("attn")
    if a is None:
        return None

    if isinstance(a, str):
        # run-12 shape: subtype inline, the other two as sibling keys.
        order, why = rec.get("attn_reading_order"), rec.get("attn_why")
        missing = [k for k, v in (("attn_reading_order", order), ("attn_why", why)) if not v]
        if missing:
            raise Ambiguous(f"attn is the bare string {a!r} but {', '.join(missing)} is absent")
        return {"subtype": a, "order": order, "why": why}

    if isinstance(a, dict):
        unknown = sorted(set(a) - set(ATTN_CANON) - set(ATTN_ALIASES.values()))
        if unknown:
            raise Ambiguous(f"attn object carries unrecognized key(s) {unknown}")
        out = {}
        for key in ATTN_CANON:
            alias = ATTN_ALIASES[key]
            if key in a and alias in a and a[key] != a[alias]:
                raise Ambiguous(f"attn carries both `{key}` and `{alias}` with different values")
            value = a.get(key, a.get(alias))
            if not value:
                raise Ambiguous(f"attn object has no `{key}` (nor `{alias}`)")
            out[key] = value
        return out

    raise Ambiguous(f"attn must be null, a string or an object, got {type(a).__name__}")


def load_fixtures(fixtures_dir: str):
    """Parse every rich fixture into (fixture_run, records, has_run)."""
    pattern = os.path.join(fixtures_dir, "vsdd-factory-312-run*-classifications-rich.json")
    out = []
    for path in sorted(glob.glob(pattern)):
        m = re.search(r"run(\d+)-classifications-rich\.json$", path)
        if not m:
            continue
        with open(path, encoding="utf-8") as fh:
            records = json.load(fh)
        if isinstance(records, dict):
            records = [records]
        has_run = all("run" in r for r in records)
        out.append((int(m.group(1)), os.path.basename(path), records, has_run))
    return sorted(out)


def main() -> int:
    ap = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    ap.add_argument("--store", default=os.path.join(REPO_ROOT, "store/vsdd-factory/state.jsonl"))
    ap.add_argument("--fixtures-dir", default=os.path.join(REPO_ROOT, "docs/fixtures"))
    ap.add_argument("--dry-run", action="store_true", help="report, write nothing")
    ap.add_argument(
        "--match",
        choices=("strict", "number"),
        default="strict",
        help="strict (default): require the fixture to carry `run`, match on number+run. "
        "number: match on issue number alone, gated on proven uniqueness.",
    )
    args = ap.parse_args()

    fixtures = load_fixtures(args.fixtures_dir)
    with open(args.store, encoding="utf-8") as fh:
        lines = fh.readlines()

    # ---- store index -------------------------------------------------------
    store_runs = defaultdict(set)  # number -> {run, ...}
    for line in lines:
        if not line.strip():
            continue
        rec = json.loads(line)
        if rec.get("kind") == "classification":
            store_runs[rec["number"]].add(rec["run"])

    multi_run = {n: sorted(r) for n, r in store_runs.items() if len(r) > 1}

    # ---- plan: number -> (fixture_run, field values), plus skips -----------
    plan: dict[int, tuple[int, dict]] = {}
    skips: dict[int, list[tuple[int, str]]] = defaultdict(list)  # fixture_run -> [(number, why)]
    dropped_fixtures = []
    seen_in_fixture: dict[int, int] = {}

    for frun, fname, records, has_run in fixtures:
        if args.match == "strict" and not has_run:
            dropped_fixtures.append((frun, fname, len(records)))
            for r in records:
                skips[frun].append((r["number"], "fixture carries no `run` field"))
            continue
        for rec in records:
            number = rec["number"]
            if number in seen_in_fixture and seen_in_fixture[number] != frun:
                raise SystemExit(
                    f"issue #{number} appears in fixture run{seen_in_fixture[number]} "
                    f"and run{frun}; refusing to guess"
                )
            seen_in_fixture[number] = frun

            try:
                attn = normalize_attn(rec)
            except Ambiguous as exc:
                skips[frun].append((number, f"ambiguous attn — {exc}"))
                continue

            vals = {}
            if attn is not None:
                vals["attn"] = attn
            for field in ("short_title", "triage_state", "possibly_fixed"):
                if rec.get(field) is not None:
                    vals[field] = rec[field]
            if not vals:
                skips[frun].append((number, "fixture carries none of the four fields"))
                continue

            if number not in store_runs:
                skips[frun].append((number, "no classification row in the store"))
                continue
            if args.match == "strict" and rec["run"] not in store_runs[number]:
                skips[frun].append(
                    (number, f"fixture run {rec['run']} != store run(s) {sorted(store_runs[number])}")
                )
                continue
            if args.match == "number" and number in multi_run:
                skips[frun].append(
                    (number, f"classified in several store runs {multi_run[number]}; number is not a key")
                )
                continue
            plan[number] = (frun, vals)

    if args.match == "number" and multi_run:
        print(f"warning: {len(multi_run)} issue number(s) span several store runs and were skipped")

    # ---- apply -------------------------------------------------------------
    repaired_rows = Counter()
    repaired_fields = defaultdict(Counter)
    already = Counter()
    remap: dict[int, int] = {}
    out = []

    for line in lines:
        if not line.strip():
            out.append(line)
            continue
        rec = json.loads(line)
        if rec.get("kind") != "classification" or rec["number"] not in plan:
            out.append(line)
            continue

        frun, vals = plan[rec["number"]]
        srun = rec["run"]
        if remap.setdefault(frun, srun) != srun:
            raise SystemExit(f"fixture run{frun} maps to store runs {remap[frun]} and {srun}")

        changed = False
        for field, value in vals.items():
            if rec.get(field) != value:
                rec[field] = value
                repaired_fields[srun][field] += 1
                changed = True
        if not changed:
            already[srun] += 1
            out.append(line)
            continue

        repaired_rows[srun] += 1
        new = canonical(rec) + "\n"
        before, after = json.loads(line), json.loads(new)
        for key in before:
            if key not in FIELDS and before[key] != after.get(key):
                raise SystemExit(f"row #{rec['number']}: re-serialising changed `{key}`")
        if set(after) - set(before) - set(FIELDS):
            raise SystemExit(f"row #{rec['number']}: unexpected new key(s)")
        out.append(new)

    if len(out) != len(lines):
        raise SystemExit("row count changed — refusing to write")

    # ---- report ------------------------------------------------------------
    print(f"match mode: {args.match}")
    print(f"fixtures:   {len(fixtures)} files, {sum(len(r) for _, _, r, _ in fixtures)} records")
    if dropped_fixtures:
        print("\nfixtures skipped wholesale (no `run` field; --match number would use them):")
        for frun, fname, n in dropped_fixtures:
            print(f"  run{frun:<3} {n:>3} records  {fname}")
    if remap:
        print("\nfixture run -> store run: " + ", ".join(
            f"{f}->{s}" + (" (REMAPPED)" if f != s else "") for f, s in sorted(remap.items())))

    print("\nrepairs by store run:")
    total = 0
    for srun in sorted(set(repaired_rows) | set(already)):
        fields = ", ".join(f"{k}={v}" for k, v in sorted(repaired_fields[srun].items())) or "-"
        print(f"  store run {srun:>2}: repaired {repaired_rows[srun]:>3} rows "
              f"(already correct {already[srun]:>3})  [{fields}]")
        total += repaired_rows[srun]
    print(f"  {'TOTAL':>13}: {total} rows repaired, {len(lines)} lines in and out")

    n_skipped = sum(len(v) for v in skips.values())
    print(f"\nskipped: {n_skipped} fixture record(s), enumerated by issue number")
    for frun in sorted(skips):
        by_reason = defaultdict(list)
        for number, why in skips[frun]:
            by_reason[why].append(number)
        print(f"  fixture run{frun}:")
        for why, numbers in sorted(by_reason.items()):
            nums = ", ".join(f"#{n}" for n in sorted(numbers))
            print(f"    {len(numbers):>3} — {why}")
            print(textwrap.fill(nums, width=92, initial_indent=" " * 10,
                                subsequent_indent=" " * 10, break_long_words=False))

    if total == 0:
        print("\nnothing to write")
        return 0
    if args.dry_run:
        print("\n--dry-run: nothing written")
        return 0

    stamp = dt.datetime.now(dt.timezone.utc).strftime("%Y-%m-%dT%H-%M-%SZ")
    backup = f"{args.store}.bak-backfill-{stamp}"
    shutil.copy2(args.store, backup)
    tmp = f"{args.store}.tmp"
    with open(tmp, "w", encoding="utf-8") as fh:
        fh.writelines(out)
    os.replace(tmp, args.store)
    print(f"\nbacked up  {backup}")
    print(f"rewrote    {args.store}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
