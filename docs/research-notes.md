# Research notes

Condensed, de-duplicated findings that shaped the design. Full sources in
`../references.bib`. Organized by topic; each principle has a one-line rationale.

## Triage frameworks that hold up

- **Separate severity (impact) from priority (urgency)** as two independent axes —
  collapsing them destroys signal [@severitypriority].
- **Layer frameworks by altitude:** coarse MoSCoW filter → RICE/WSJF score the
  shortlist. Cap "Musts" ≤60% of capacity or the classification self-destructs.
- **Default new items low; escalate only on evidence** (Mozilla P3-default tied to
  a release/OKR) [@moztriage]. **Treat "no priority label" as a discrete
  not-yet-triaged state, not "low"** (Kubernetes) [@k8striage].
- **Don't trust the reporter's kind label — re-classify on triage** (support filed
  as bug is the classic; ~1/3 mislabeled at source).
- **Needs-info has a hard timeout** (K8s 20d; Mozilla needinfo→close-incomplete);
  unbounded needs-info is pure carrying cost.
- **Duplicate ≠ related cluster:** close+link the dupe; group the cluster under a
  meta/tracking issue (same lever filed N times is a tracking issue, not N
  closures). Auto-close only on high-confidence exact match.

## Minutiae overload & "led by the backlog"

- **Cap the active backlog**; when full, force a prioritization decision, don't
  grow capacity. A wishlist-as-backlog drowns high-leverage work [@backloghealth].
- **YAGNI as a triage filter:** default answer to a speculative request is "not
  yet," gated on a concrete present need. Ask "does this deliver value *today*?"
- **Over-engineering fingerprints = drift signals:** premature abstraction,
  speculative generics, "might-be-useful" utilities, config flags with no
  consumer, feature anticipation ahead of demand.
- **Reserve forethought for infrastructure, public APIs, security** — retrofitting
  these is disruptive; YAGNI is "delay implementation, not avoid design."
- **Redirect support off the tracker; close out-of-scope kindly with a reason** —
  an honest "no" keeps the queue aligned to intent.

## The automated-filer risk (AI slop)

- **Demand human accountability + disclosure**; the filer remains responsible for
  quality (OpenSSF / Seth Larson consensus).
- **The specific risk: surface-plausible, technically fabricated reports that cost
  real time to refute** — curl: ~5% genuine, ~20% AI slop; referenced functions
  that don't exist [@curlslop]. → triage on *substance* (real code refs?
  reproduces?), not "vibes"; there is no reliable AI-detector.
- **Don't let anti-slop policy discourage good contribution** — the target is
  misuse, not AI or newcomers.

## Dashboard mechanics (Renovate + the failure mode)

- One title-discovered issue, full body rewrite each run, checkbox-as-control via
  `- [ ] <!-- verb=target -->` lines parsed on the next scheduled run
  [@renovatedash].
- **State-in-Markdown is the universal failure mode** — Renovate breaks on
  hand-edit (#19563) [@renovate19563]. Store state out-of-band; the body is a
  projection; embed only a digest in a `<!-- -->` sentinel block.
- 65,536-char body limit (HTTP 422 over it); offload + `<details>`.
- Run as a **GitHub App** on a **cron schedule**; break the `GITHUB_TOKEN`
  re-trigger guard; 15k/hr limits; throttle below secondary limits [@ghaw;
  @ghacceptableuse].
- **Issues vs Projects:** a single rewritten issue is the right interactive
  surface; Projects v2 is a complement for structured cross-repo rollups only.

## Human-AI collaboration (the canon)

- **Automation bias** → commission *and* omission errors; felt accountability +
  independent verification suppress both [@skitka].
- **Complacency** rises with workload, afflicts experts, can't be trained away →
  reduce monitoring load; force explicit decisions [@complacency].
- **Out-of-the-loop** — full automation degrades situation awareness more than
  partial → prefer propose-with-reasoning [@oolp].
- **Irony of automation** — automating the easy work hands humans the hardest
  residual cases when least practiced [@bainbridge].
- **Mode confusion** from ambiguous state → tag every action with its mode
  (`[beadle: proposal]` vs `[beadle: auto]`) [@modeconfusion].
- **Alarm fatigue** — escalation precision beats recall.
- **Set automation level per stage** (acquire/analyze/decide/act) independently
  [@pswlevels].
- **Calibrated, not maximal, trust** [@leesee]; **meaningful human control** =
  tracking + tracing [@meaningfulcontrol]; EU AI Act Art. 14 oversight baseline
  [@euaiact]; confidence-as-frequency [@confcalib]; cheap-to-verify evidence, not
  prose [@vasconcelos].

## Keeping aimed at intent (anti-Goodhart)

- **Write rules against intended outcome, never a literal proxy** — assume any
  loophole is found [@specgaming].
- **Close-rate / time-to-triage / label-coverage are proxies that break under
  optimization** (Goodhart); a "tickets closed" target produces a feature-factory
  — exactly how a tracker leads a project off its roadmap [@goodhart]. Pair every
  count with an outcome signal.
- **Re-anchor to the written goal each run; independent held-out check** the agent
  never sees — penalizing visible gaming only teaches subtler gaming [@metr].

## Multi-agent design

- Multi-agent wins on parallelizable read/research/review; loses on tightly
  interdependent writes [@anthropicmulti]. **Serialize writes through one thread**
  [@cognitionmulti]. Give each sub-agent objective + format + boundaries.
  **Verify with a dedicated fresh-context agent** against ground truth.

## StrongDM Software Factory (intent-steering mechanisms)

- Humans own intent + held-out scenarios + satisfaction scoring; agents do the
  rest [@strongdm].
- **Information asymmetry** (evaluation kept out of the agent's view, ML-holdout
  style) resists teaching-to-the-test.
- **Probabilistic satisfaction over binary pass/fail** resists `return true`
  reward-hacking.
- **Deliberate naivety** — prune ceremony that no longer earns its keep ("why am I
  doing this?").
