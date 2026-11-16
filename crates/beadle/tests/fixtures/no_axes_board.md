# 📋 beadle · Triage Dashboard · acme/widget

<!-- beadle-state:v1
{
  "body_digest": "pending",
  "counts": {
    "arcavenai_closed_alltime": 0,
    "arcavenai_open": 0,
    "maintainer_engaged": 0,
    "open": 0
  },
  "intent_version": null,
  "run": 1,
  "schema": 1,
  "target": "t",
  "warmup": null,
  "watermark": 9
}
beadle-state -->

**Direction verdict: 🟢 ON-COURSE** · run 1 · integrity-density on-course: 0/2 classified issues carry integrity=true → 0.0% · next: no open issue waits on a human reading

| At a glance | Result |
|---|---|
| Open issues | **2** |
| Closed (observed) | **0** |
| Integrity (⚠) | **0** |
| Silent data loss (▲) | **0** |
| Needs human reading | **0** |

<details><summary>Why this verdict</summary>

<!-- editor:direction-verdict -->
_Editor: fill in the direction verdict paragraph. Renderer only surfaces the numbers; the interpretation is yours. The sentinel's derived_digest tracks whether any of these numbers changed since the last run._
<!-- /editor:direction-verdict -->

Signal detail, one item per direction signal:

- filing-density: 🟢 on-course · single run (0 new); no history to derive against
- integrity-density (B): 🟢 on-course · 0/2 classified issues carry integrity=true → 0.0%
- silent-data-loss-share (C): 🟢 on-course · 0/2 classified issues flagged silent-data-loss → 0.0%
- silent-data-loss-zero-engagement (A4): ⚪ pending · pending: no silent-data-loss classifications in store

</details>

## Baseline (derived from store)

| Metric | Value | Provenance |
|---|---|---|
| Open issues | 2 | issue records whose latest observation is `state=open` |
| Closed (observed) | 0 | latest observation is `state=closed` |
| Observed issues | 2 | distinct numbers ever observed (open + closed) |
| Comment events | 0 | count(comment_event); 0 issues touched |
| Maintainer comments | 0 | actor_role=maintainer |
| Measured comments | 0 | actor_role=measured |
| Other comments | 0 | actor_role=other |
| filed_vs_acted_gap | 2 : 0 | open issues : maintainer actions |
| Watermark | #9 | max(issue.number) |

_State is each issue's **last observation**, not a live read. `beadle enum` fetches only `--state open` and only above the watermark, so an issue closed after it was last observed still reads open here until something re-observes it (ArcavenAE/beadle#67)._

### Top commenters (per-actor totals, all runs)

_no comments observed_

## Clusters (with decay)

_no clusters recorded_

## Classification summary

| Metric | Value | Notes |
|---|---|---|
| Classified issues | 2 | of 2 observed (open + closed) |
| Integrity (⚠) | 0 | HARD: requires `integrity_anchor` |
| Silent-data-loss (▲) | 0 | operational_impact axis |
| Quick-win eligible (★) | 0 | HARD: never on integrity=true |
| P0 / P1 | 0 / 0 | priority axis |

| Report type | Count |
|---|---|
| `bug` | 2 |

## Board controls

_Tier-2 maintenance requests. Check a box, wait for the next `beadle push`; beadle records the request as a `control-request` note and resets the box. Dispatch of the routines lands with the Phase-1 gh-aw cron (deferred, per `question-maintenance-request-controls`)._

- [ ] `reprioritize`: re-rank the action plan without a full re-enumerate <!-- verb=reprioritize;id=board -->
- [ ] `full-refresh`: re-enumerate every open issue from scratch <!-- verb=full-refresh;id=board -->
- [ ] `revalidate`: re-run validate across the corpus (already-fixed / fix-not-fixed / hallucinated-citation) <!-- verb=revalidate;id=board -->
- [ ] `rescore-intent`: re-run score-intent corpus-wide after an intent-manifest change <!-- verb=rescore-intent;id=board -->


## Editor notes

<!-- editor:notes -->
_Editor: free-form notes, per-issue verdict chips, quick-win analysis, escalations, whatever the compass tells you to say this run._
<!-- /editor:notes -->