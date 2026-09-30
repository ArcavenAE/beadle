#!/usr/bin/env python3
"""Regression gate for /dashboard-refresh step 3. Exit 0 = pass."""
import json, re, sys

before_p, cand_p = sys.argv[1], sys.argv[2]
before, cand = open(before_p).read(), open(cand_p).read()
fails, warns = [], []

def sentinel(t):
    m = re.search(r'<!--\s*beadle-state:v1\s*\n(.*?)\nbeadle-state\s*-->', t, re.S)
    return json.loads(m.group(1)) if m else None

sb, sc = sentinel(before), sentinel(cand)

# ---- 1. section presence -------------------------------------------------
if sc is None:
    fails.append("1: candidate sentinel missing or unparseable")
else:
    if sc.get("run") != sb["run"] + 1:
        fails.append(f"1: run must be {sb['run']+1}, got {sc.get('run')}")
    if not (sc.get("watermark", 0) > sb["watermark"]):
        fails.append(f"1: watermark must exceed {sb['watermark']}, got {sc.get('watermark')}")

required = [
    (r'^## Baseline', "## Baseline"),
    (r'^## 👤 Needs human reading', "## Needs human reading (attn lane)"),
    (r'^## Action plan', "## Action plan"),
    (r'^### 🔴 P0a', "### P0a"),
    (r'^### 🔴 P0b', "### P0b"),
    (r'^### 🟠 P1', "### P1 (>=1)"),
    (r'^### 🟢 P', "### P2/P3 (>=1)"),
    (r'^## 🟦 Quick wins', "## Quick wins"),
    (r'^## Direction Health', "## Direction Health"),
    (r'^## Classification index', "## Classification index"),
    (r'^## Maintainer progress', "## Maintainer progress"),
    (r'^## Controls', "## Controls"),
]
for pat, label in required:
    if not re.search(pat, cand, re.M):
        fails.append(f"1: required section missing -> {label}")

# direction verdict paragraph near the top, bolded
_post = cand.split('beadle-state -->', 1)[-1]
if not re.search(r'\*\*Direction verdict:', _post[:3000]):
    fails.append("1: bolded 'Direction verdict:' line missing from the head")

# carried-forward prior index(es) must survive
prior_idx = re.findall(r'^### Run-(\d+) index', before, re.M)
cand_idx = re.findall(r'^### Run-(\d+) index', cand, re.M)
for r in prior_idx:
    if r not in cand_idx:
        fails.append(f"1/2: prior classification index for run-{r} dropped")
if sc and str(sc["run"]) not in cand_idx:
    fails.append(f"1: new Run-{sc['run']} index missing")

# ---- 2. no section loss --------------------------------------------------
# Each rename maps the before heading to (candidate heading, reason); the
# reason is printed with the warning, as `beadle verify` prints its `why`.
_RUN18 = "run-18 carry precedent"
# One-time ISO/IEC 25010 transition entries, not a category schema: they let
# the boards retire today's grouping once. They do not fix which categories
# a board carries, and new categories need no entry here.
_T25010 = "one-time ISO/IEC 25010 transition: the index stops naming the retired grouping"
DECLARED_RENAMES = {
 "### Run-18 index (NEW)":
   ("### Run-18 index (carried forward — folded)", _RUN18),
 "### 🟢 P1/P2/P3 — Run-18 findings (NEW · 13 P2 rows + 12 P3 by cluster; P0b/P1 rows ride their lanes above)":
   ("### 🟢 P1/P2/P3 — Run-18 findings (carried — folded · 13 P2 rows + 12 P3 by cluster; P0b/P1 rows ride their lanes above)", _RUN18),
 "## Classification index (finding-005 + finding-009 + attn facet + IEEE 1044 + ODC + ISO/IEC 25010)":
   ("## Classification index (finding-005 + finding-009 + attn facet + IEEE 1044 + ODC)", _T25010),
 "## Classification index (report type + defect nature + reproducibility + IEEE 1044 + ODC + ISO/IEC 25010)":
   ("## Classification index (report type + defect nature + reproducibility + IEEE 1044 + ODC)", _T25010),
}
# A removal the run intends, with the reason. Headings are H2/H3 lines, exact.
# Axes are top-level sentinel keys: one covers its `key.sub` sub-axes too. A
# declared axis only excuses the axis vanishing (3b); its issue numbers must
# still survive somewhere in state (3). The list serves every board, so an
# entry a board never carried warns as stale rather than failing.
_G25010 = "ISO/IEC 25010 grouping retired from the board; its rows stay in the P2/P3 lanes"
DECLARED_REMOVALS = {
 "headings": {
  "### 🟢 P2: Correctness and operability defects, grouped by ISO/IEC 25010 characteristic": _G25010,
  "### 🟢 P3: Papercuts, docs, tests, and design questions, grouped by ISO/IEC 25010 characteristic": _G25010,
  "### 🟢 P2: Correctness and operability, grouped by ISO/IEC 25010 characteristic": _G25010,
  "### 🟢 P3: Papercuts, docs, diagnostics and design questions, grouped by ISO/IEC 25010 characteristic": _G25010,
  "### 🟢 P3: Papercuts, docs, design questions, and dependency bumps, grouped by ISO/IEC 25010 characteristic": _G25010,
  "## Quality characteristic groups (ISO/IEC 25010:2023)": _G25010,
 },
 "axes": {
  "iso25010": "ISO/IEC 25010 grouping retired from the board; issues stay tracked on their other axes",
  "taxonomy_group": "the taxonomy group axis went with the ISO/IEC 25010 grouping it indexed",
 },
}
hb = re.findall(r'^#{2,3} .*$', before, re.M)
hc = set(re.findall(r'^#{2,3} .*$', cand, re.M))
for h in DECLARED_REMOVALS["headings"]:
    if h not in hb:
        warns.append(f"2: stale declared removal, heading not in before-snapshot -> {h[:60]!r}")
    elif h in hc:
        warns.append(f"2: declared removal still present in candidate -> {h[:60]!r}")
if sb:
    for a in DECLARED_REMOVALS["axes"]:
        if a not in sb:
            warns.append(f"3b: stale declared removal, key not in before-snapshot -> '{a}'")
for h in hb:
    if h in hc:
        continue
    tgt, why = DECLARED_RENAMES.get(h, (None, None))
    if tgt and tgt in hc:
        warns.append(f"2: declared rename -> {h[:60]!r} => {tgt[:60]!r} ({why})")
    elif h in DECLARED_REMOVALS["headings"]:
        warns.append(f"2: declared removal -> {h[:60]!r} ({DECLARED_REMOVALS['headings'][h]})")
    else:
        fails.append(f"2: HEADING LOST -> {h[:95]}")

# ---- 3. no coverage shrinkage -------------------------------------------
def nums(state):
    out = set()
    for k, v in state.items():
        if isinstance(v, list):
            out |= {x for x in v if isinstance(x, int)}
        elif isinstance(v, dict):
            # Only list ELEMENTS and digit-string KEYS are issue numbers.
            # Dict int VALUES are counters (a4_windows window-counts, `counts`
            # totals) and must never be read as tracked issues.
            for vv in v.values():
                if isinstance(vv, list):
                    out |= {x for x in vv if isinstance(x, int)}
            out |= {int(k2) for k2 in v if str(k2).isdigit()}
    return out

if sb and sc:
    missing = sorted(nums(sb) - nums(sc))
    if missing:
        fails.append(f"3: {len(missing)} tracked issue(s) dropped from state: {missing[:25]}")

    # 3b. PER-AXIS preservation. The union check above is masked whenever a number
    # dropped from one axis still appears in another, so every cumulative axis is
    # checked on its own. Per-run axes are exempt by name (they are meant to reset).
    PER_RUN = {'new_this_run', 'quick_wins_new', 'keystone_new', 'prior_run_burst', 'degraded_new'}
    def axes(state):
        out = {}
        for k, v in state.items():
            if isinstance(v, list) and k not in PER_RUN:
                out[k] = {x for x in v if isinstance(x, int)}
            elif isinstance(v, dict):
                for kk, vv in v.items():
                    if isinstance(vv, list) and kk not in PER_RUN:
                        out[f"{k}.{kk}"] = {x for x in vv if isinstance(x, int)}
        return out
    ab, ac = axes(sb), axes(sc)
    for k, vb_ in ab.items():
        vc_ = ac.get(k)
        why = DECLARED_REMOVALS["axes"].get(k.split('.', 1)[0])
        if vc_ is None and why:
            warns.append(f"3b: declared removal -> axis '{k}' ({why})")
        elif vc_ is None:
            fails.append(f"3b: cumulative axis '{k}' disappeared from state")
        elif vb_ - vc_:
            fails.append(f"3b: axis '{k}' lost {len(vb_-vc_)} issue(s): {sorted(vb_-vc_)[:20]}")

    # 3c. a per-run axis that is being reset must have a _prior counterpart that
    # absorbs its members, or the classification is silently discarded.
    for k in ('quick_wins', 'degraded'):
        newk = {'quick_wins': 'quick_wins_new', 'degraded': 'operational_impact.degraded_new'}[k]
        prik = {'quick_wins': 'quick_wins_prior', 'degraded': 'operational_impact.degraded_prior'}[k]
        def getlist(state, dotted):
            cur = state
            for part in dotted.split('.'):
                cur = cur.get(part) if isinstance(cur, dict) else None
                if cur is None: return None
            return set(cur)
        oldnew = getlist(sb, newk)
        if not oldnew: continue
        newpri = getlist(sc, prik) or set()
        lost = oldnew - newpri - (getlist(sc, newk) or set())
        if lost:
            fails.append(f"3c: '{newk}' reset without rolling {sorted(lost)[:20]} into '{prik}'")

# ---- 4. classification completeness -------------------------------------
if '_unclassified_' in cand:
    fails.append("4: '_unclassified_' present in candidate")

# ---- 5. markdown render integrity (REGRESSION-relative) ------------------
# The live run-18 body already carries violations of 5d. A gate that fails on
# inherited debt blocks every future post, so these compare candidate vs before:
# a NEW violation fails; a pre-existing one warns and is reported for follow-up.
def render_violations(t):
    v = []
    ls = t.split('\n')
    for i in range(1, len(ls)):
        if ls[i].lstrip().startswith('|') and ls[i-1].lstrip().startswith('>'):
            v.append(('5a', 'table directly follows blockquote', ls[i].strip()[:80]))
    for i, l in enumerate(ls):
        if '<details>' in l and '<summary>' in l and i+1 < len(ls) and ls[i+1].strip() != '':
            v.append(('5b', 'no blank line after <details><summary>', l.strip()[:80]))
        if l.strip() == '</details>' and i > 0 and ls[i-1].strip() != '':
            v.append(('5b', 'no blank line before </details>', ls[i-1].strip()[:80]))
    for blk in re.findall(r'<details>.*?</details>', t, re.S):
        body = re.sub(r'```.*?```', '', blk, flags=re.S)
        for row in re.findall(r'^\s*\|.*\|\s*$', body, re.M):
            v.append(('5d', 'pipe-table row inside <details>', row.strip()[:80]))
    return v

vb, vc = render_violations(before), render_violations(cand)
kb = [(a, b) for a, b, _ in vb]
for a, b, ctx in vc:
    if (a, b) in kb:
        kb.remove((a, b))          # inherited — accounted for below
    else:
        fails.append(f"{a}: NEW render-integrity violation — {b} :: {ctx}")
if vb:
    warns.append(f"{len(vb)} pre-existing render-integrity violation(s) inherited from run-18 "
                 f"(not introduced here): " + "; ".join(sorted({f'{a} {b}' for a, b, _ in vb})))

# 5c: every attn issue in the sentinel appears as a visible table row before the section's <details>
if sc and 'attn' in sc:
    attn_nums = {n for v in sc['attn'].values() for n in v}
    m = re.search(r'^## 👤 Needs human reading.*?(?=^## |\Z)', cand, re.S | re.M)
    if not m:
        fails.append("5c: attn section not found")
    else:
        sec = m.group(0)
        visible = sec.split('<details>')[0]
        for n in sorted(attn_nums):
            if not re.search(rf'#{n}\b', visible):
                fails.append(f"5c: attn issue #{n} in sentinel but not a visible row in the lane table")

# ---- report --------------------------------------------------------------
print(f"before: {len(before):,} chars, {len(hb)} headings")
print(f"cand  : {len(cand):,} chars, {len(hc)} headings")
if sb and sc:
    print(f"run {sb['run']} -> {sc.get('run')} | watermark {sb['watermark']} -> {sc.get('watermark')}")
    print(f"tracked issue numbers: {len(nums(sb))} -> {len(nums(sc))}")
print()
for w in warns: print("WARN", w)
if fails:
    print(f"GATE FAILED — {len(fails)} check(s):")
    for f in fails: print("  ✗", f)
    sys.exit(1)
print("GATE PASSED — all checks green")
