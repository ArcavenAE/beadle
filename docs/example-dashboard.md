# Example dashboard (worked reference)

A real Phase-0 dry run against `drbothen/vsdd-factory`, computed from live data on
2026-06-27. This is the rendered issue body beadle would maintain. The verdict is
**gated on warm-up state** (ADR-005): the process has not turned yet, so this is a
**cold-start / baseline** dashboard — it establishes the baseline and makes the
maintainers' first pass cheap, and explicitly marks rate/drift signals as *not yet
measurable*, never as a drift indictment.

---

```markdown
# 📋 beadle — Triage Dashboard · drbothen/vsdd-factory

<!-- beadle-state:v1
{"schema":1,"last_run":"2026-06-27","watermark":306,"store":"jsonl",
 "intent_version":"vsdd-factory@0.1","warmup":"cold-start",
 "counts":{"open":107,"arcavenai_open":94,"maintainer_engaged":0,
 "arcavenai_closed_alltime":1}} beadle-state -->

**Direction verdict: 🟡 COLD START (baseline).** The wheel hasn't turned yet — the
maintainers are busy and haven't begun their first triage pass. So the rate-style
numbers below (resolution rate, maintainer engagement) are a **baseline, not a
drift signal**: an average computed over a window before the process engaged is
initialization bias, not meaningful *yet*. This dashboard's job today is to
establish that baseline and make the first pass cheap. Once the first maintainer
triage cycle completes, beadle switches to trend/drift reporting. _Updated
2026-06-27 · watermark #306 · intent vsdd-factory@0.1_

## Baseline (counts — interpret as a starting line, not a rate)

| Metric | Value | Note |
|---|---|---|
| Open issues | 107 | arcavenai 94 · Zious11 6 · drbothen 6 · slabgorb 1 |
| Filer concentration | arcavenai 88% | one prolific (AI) filer — expected at startup |
| Reviewed by `arcaven` | 52 / 94 | review-identity passes (not a maintainer decision) |
| Awaiting first read | 21 | zero comments yet |
| Maintainer engagement | — | _not yet measurable (cold start)_ |
| Resolution / net-flow | — | _not yet measurable (cold start)_ |

## First-pass action plan (make the maintainers' first read cheap)

High-leverage items worth a maintainer decision first — verified where noted:

| # | Issue | Verdict | Suggested first-pass action |
|---|---|---|---|
| #300 | registry missing L1 brief | advances · verified hard-block | accept + 1-line registry add |
| #305 | unbuildable stories (AC-collapse) | advances · systemic | tighten decompose gate to AC-granularity |
| #297 | agents relax governance config | advances · integrity | add governance-edit guard |
| #299 | cross-surface set-equality | advances | accept; prefer derive-over-duplicate |
| #241/#242 | observability broken (Loki/resolver) | advances · verified | confirm fixed-pending-release vs open |

## Structure (cluster candidates — group as tracking issues, don't close)

Same-lever issues filed N times → fold into one tracking issue each, so the first
pass reads 6 threads, not 40:

- 🧵 Observability/Grafana/Loki (~12): #203 #206 #207 #208 #235 #237–#239 #241 #243–#245
- 🧵 factory-health / orphan-branch / STATE template (~7): #204 #205 #209 #229 #230 #234 #236
- 🧵 preflight / env / OS-detection (~6): #225 #226 #227 #228 #253 #255
- 🧵 orchestrator parallel-burst / race / push (~6): #210 #250 #258 #273 #275 #293
- 🧵 post-merge process-gaps (~5): #289 #290 #291 #292 #294
- 🧵 scaffold-claude-md (2): #295 #302

## Watch (revisit after the process turns — NOT drift yet)

- Filer concentration + clustering are normal at startup; they become a drift
  signal only if they persist *after* maintainers begin engaging.
- Self-referential candidates to re-examine later: #306 (whole-corpus reviewer
  thrash), #278 (calibration-drift observation).

## Controls

- [ ] <!-- verb=group-cluster;id=observability --> Create tracking issue for the observability cluster
- [ ] <!-- verb=route-decision;id=#300,#305,#297 --> Route first-pass items to a maintainer decision
- [ ] <!-- verb=mark-process-turned;id=baseline --> Mark the first maintainer pass complete (ends cold-start; enables trend reporting)
```

---

Note the `mark-process-turned` control: checking it records that the first
maintainer pass happened, which ends the warm-up transient and lets beadle begin
rate/trend/drift reporting (B3 maintainer-compass, ADR-005).
