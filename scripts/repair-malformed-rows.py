#!/usr/bin/env python3
"""Repair store rows that claim a known `kind` but fail that kind's struct.

ArcavenAE/beadle#73. `Record` is a serde internally-tagged enum with an
untagged `Other` catch-all, so such a row does not error — it lands in
`Record::Other`, round-trips happily, and vanishes from every consumer that
matches the specific variant. `beadle render` now WARNS about these rows; it
cannot repair them. This does.

Guarantees (same contract as backfill-classification-fields.py)
--------------------------------------------------------------
* Backed up to `state.jsonl.bak-repair-<utc>` before any write, and only when
  there is something to write.
* Rows are never reordered and never dropped; output has exactly as many lines
  as input, in the same order.
* Lines not repaired are copied through byte-for-byte.
* Only MISSING fields are ever added. An existing value is never changed and no
  key is ever removed — asserted per row before the line is emitted. The extra
  `note` key on the #365 row is provenance and is preserved.
* Idempotent: a repaired row is well-formed, so a second run finds nothing.
* A row it cannot repair with proof is SKIPPED and reported. A wrong value in
  the system of record is worse than a known hole.

Repair rules — both general, neither hardcoded to a row
-------------------------------------------------------
note missing `ts`
    Recovered from the `Run` record of the same run. A note is written during
    its run, so the run's timestamp is the defensible bound. Skipped if that
    run has no `Run` record.

issue missing stable observational fields
    Inherited from the EARLIEST well-formed observation of the same issue
    number — but only after proving the body has not changed since, by
    re-hashing the live GitHub body and matching it against that observation's
    `body_sha256`. `updated_at` is not stable and comes from GitHub.

    `body_len` is deliberately NOT recomputed from the GitHub body. Measured
    across four issues, beadle's stored `body_len` exceeds `len(gh body)` by
    exactly the CRLF count (deltas 135, 11, 35, 0) while `body_sha256` matches
    on every one: the collector hashes the LF-normalised body and measures the
    raw one. `gh --json body` cannot reproduce the raw length, so the earlier
    observation's own value is the only valid source.

Requires --verify-github for the issue rule. It fails closed: without the
proof, the row is skipped.
"""

import argparse
import hashlib
import json
import shutil
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

# Non-`#[serde(default)]` fields, mirroring crates/beadle-store/src/lib.rs.
REQUIRED = {
    "issue": {"ts", "target", "number", "observed_in_run", "title", "author",
              "state", "created_at", "updated_at", "body_len", "body_sha256"},
    "note": {"ts", "target", "run", "topic", "text"},
    "run": {"ts", "target", "run", "watermark_after", "counts", "digest"},
    "cluster": {"run", "last_added_run"},
    "comment_event": {"ts", "target", "number", "event", "actor", "actor_role",
                      "observed_in_run"},
    "classification": {"ts", "target", "number", "run", "report_type",
                       "defect_nature", "reproducibility", "leverage",
                       "alignment", "provenance", "integrity", "priority",
                       "rationale"},
}
# Stable per-body; safe to inherit from an earlier observation once the body is
# proven unchanged. `updated_at` is excluded on purpose.
INHERITABLE = {"created_at", "body_len", "body_sha256"}


def gh_issue(repo, number):
    out = subprocess.run(
        ["gh", "issue", "view", str(number), "-R", repo,
         "--json", "body,updatedAt,createdAt,state"],
        capture_output=True, text=True,
    )
    if out.returncode != 0:
        return None
    return json.loads(out.stdout)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--store", default="store/vsdd-factory/state.jsonl")
    ap.add_argument("--repo", default="BOHICA-LABS/vsdd-factory")
    ap.add_argument("--dry-run", action="store_true", help="report, write nothing")
    ap.add_argument("--verify-github", action="store_true",
                    help="required to repair issue rows; re-hashes the live body")
    args = ap.parse_args()

    store = Path(args.store)
    lines = store.read_text().splitlines()

    parsed = []
    for raw in lines:
        parsed.append(json.loads(raw) if raw.strip() else None)

    # Index the sound material we repair FROM.
    run_ts, earliest_issue = {}, {}
    for d in parsed:
        if not d:
            continue
        k = d.get("kind")
        if k == "run" and not (REQUIRED["run"] - set(d)):
            run_ts.setdefault(d["run"], d["ts"])
        if k == "issue" and not (REQUIRED["issue"] - set(d)):
            n = d["number"]
            if n not in earliest_issue or d["observed_in_run"] < earliest_issue[n]["observed_in_run"]:
                earliest_issue[n] = d

    repairs, skips, out = [], [], []
    for i, (raw, d) in enumerate(zip(lines, parsed), 1):
        if not d or d.get("kind") not in REQUIRED:
            out.append(raw)
            continue
        missing = REQUIRED[d["kind"]] - set(d)
        if not missing:
            out.append(raw)
            continue

        fixed, why = None, None
        if d["kind"] == "note" and missing == {"ts"}:
            ts = run_ts.get(d.get("run"))
            if ts:
                fixed, why = {"ts": ts}, f"ts from run-{d['run']} record"
            else:
                why = f"no well-formed Run record for run {d.get('run')}"

        elif d["kind"] == "issue" and missing <= (INHERITABLE | {"updated_at"}):
            src = earliest_issue.get(d.get("number"))
            if not src:
                why = "no well-formed earlier observation of this issue"
            elif not args.verify_github:
                why = "issue repair needs --verify-github (fails closed)"
            else:
                live = gh_issue(args.repo, d["number"])
                if live is None:
                    why = "GitHub fetch failed"
                else:
                    body = live.get("body") or ""
                    sha = hashlib.sha256(body.encode()).hexdigest()
                    if sha != src["body_sha256"]:
                        why = (f"body changed since run {src['observed_in_run']} "
                               f"({sha[:12]} != {src['body_sha256'][:12]}) — "
                               "cannot inherit")
                    else:
                        fixed = {k: src[k] for k in missing & INHERITABLE}
                        if "updated_at" in missing:
                            fixed["updated_at"] = live["updatedAt"]
                        why = (f"inherited {sorted(missing & INHERITABLE)} from run "
                               f"{src['observed_in_run']}, body sha verified against "
                               f"live GitHub; updated_at from GitHub")
        else:
            why = f"missing {sorted(missing)} — no rule covers this shape"

        if not fixed:
            skips.append((i, d.get("kind"), d.get("number") or d.get("run"), why))
            out.append(raw)
            continue

        merged = dict(d)
        for k, v in fixed.items():
            assert k not in merged, f"line {i}: would overwrite existing {k}"
            merged[k] = v
        assert set(merged) == set(d) | set(fixed), f"line {i}: key set drifted"
        assert all(merged[k] == d[k] for k in d), f"line {i}: existing value changed"
        assert not (REQUIRED[d["kind"]] - set(merged)), f"line {i}: still malformed"
        out.append(json.dumps(merged, sort_keys=True, separators=(",", ":")))
        repairs.append((i, d.get("kind"), d.get("number") or d.get("run"),
                        sorted(fixed), why))

    print(f"store: {store}  ({len(lines)} lines)")
    print(f"\nrepaired: {len(repairs)}")
    for i, k, n, fields, why in repairs:
        print(f"  line {i:>5}  {k:<14} {n}  +{fields}\n        {why}")
    print(f"\nskipped: {len(skips)}")
    for i, k, n, why in skips:
        print(f"  line {i:>5}  {k:<14} {n}  — {why}")

    if not repairs:
        print("\nnothing to write")
        return 0
    if args.dry_run:
        print("\n--dry-run: nothing written")
        return 0
    if len(out) != len(lines):
        raise SystemExit("row count changed — refusing to write")

    stamp = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H-%M-%SZ")
    backup = f"{store}.bak-repair-{stamp}"
    shutil.copy2(store, backup)
    store.write_text("\n".join(out) + "\n")
    print(f"\nbacked up  {backup}\nrewrote    {store}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
