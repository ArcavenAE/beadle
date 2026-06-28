---
bibliography: ../references.bib
---

# Guiding philosophy

beadle is bound by three layered sources. Where they differ, the tension is
surfaced, not silently resolved.

## 1. aae-orc `SOUL.md`

The org's engineering philosophy ([`../../SOUL.md`](../../SOUL.md)). The tenets
that bind beadle most:

- **User sovereignty.** No phone-home, no lock-in, no hidden dependencies. beadle
  reads public GitHub state and a target's own declared intent; it does not
  exfiltrate or couple a project to a hosted service. Telemetry is opt-in.
- **Composability over frameworks; no component conscripts another.** Each beadle
  engine (validate, classify, investigate, score-intent, dashboard) is usable on
  its own. beadle runs as a skill without marvel; it can target a repo without
  sideshow. It composes the existing `arcavenai-issue-review` skill rather than
  absorbing it.
- **Observable by default, never mandatory.** Decisions are auditable; nothing
  core is gated on telemetry.
- **Parallel safety.** Every run/session is isolated; no shared mutable state
  (INC-001 made architectural).
- **Spec-driven development.** The intent manifest is the source of truth for
  *intent*; the code/issues are the source of truth for *behaviour*; when they
  diverge, investigate — don't silently reconcile. This is literally beadle's job.
- **Gradual elaboration.** Build what you need, not what you might need — the
  phasing in `DESIGN.md` §8.

## 2. The StrongDM Software Factory manifesto

`factory.strongdm.ai` [@strongdm]. A manifesto for non-interactive,
agent-built software. beadle does not adopt its cardinal prohibitions ("code must
not be written / reviewed by humans") wholesale — beadle is a triage assistant
under human supervision, not a lights-out factory — but it adopts the mechanisms
the manifesto uses to keep an autonomous system aimed at human intent:

- **Humans own intent, scenarios, and satisfaction scoring.** The agents do the
  rest. beadle's per-target intent manifest *is* this human-owned steering layer;
  the maintainer-engagement signal is the satisfaction read-out.
- **Information asymmetry by design.** The manifesto keeps evaluation scenarios
  *out of the coding agent's view* (an ML holdout set) so the system can't teach
  to the test. beadle's frontier F6 asks whether some alignment scenarios can be
  held out of the scoring agent's view for the same reason.
- **Probabilistic satisfaction over binary pass/fail.** "What fraction of
  trajectories likely satisfy the user?" beats a brittle green/red gate that
  invites reward-hacking (`return true`). beadle's alignment verdict is graded
  (advances / neutral / drifts with confidence), never a boolean label.
- **Deliberate naivety — "why am I doing this?"** Prune inherited ceremony that no
  longer earns its keep. For triage: if a task exists only because humans used to
  do it by hand, question whether it should exist at all. This is the antidote to
  the minutiae spiral beadle is built to detect.

## 3. The human-AI collaboration canon

The well-studied failure modes of human-AI teaming, and their mitigations
(`docs/research-notes.md` has the full set with sources):

- **Automation bias** produces both commission and omission errors; only felt
  accountability + independent verification suppress both [@skitka]. → Every
  beadle action is attributable to a named supervisor; high-stakes verdicts are
  re-checked by a fresh-context agent.
- **The irony of automation** [@bainbridge]: automating the easy work hands humans
  the hardest residual cases when least practiced. → Escalated cases get *more*
  context, not less; rotate a sample of bot-handled issues through full human
  review to keep maintainer triage skill alive.
- **Alarm fatigue**: high false-positive escalation desensitizes until the real
  alert is ignored. → Escalation precision beats recall; every human ping clears
  a high bar.
- **Meaningful human control** = *tracking* (the bot reflects the human's policy)
  + *tracing* (every outcome maps to a human who understood it) [@meaningfulcontrol];
  the EU AI Act Art. 14 oversight baseline — override, halt, reversibility,
  automation-bias awareness [@euaiact].
- **Calibrated trust, not maximal trust** [@leesee]: surface the bot's per-category
  track record so reliance tracks demonstrated reliability.
- **Goodhart / specification gaming** [@goodhart; @specgaming]: close-rate,
  time-to-triage, and label-coverage are proxies that break under optimization.
  beadle never optimizes them as success; it pairs every count with an outcome
  signal, re-anchors to the written intent each run, and keeps an independent
  held-out check (METR: penalizing visible gaming only teaches subtler gaming)
  [@metr].

## The synthesis

All three converge on one posture: **humans own intent and the right to override;
the system reads and analyzes freely but proposes for anything consequential or
public; evaluation is graded and partly held-out so it resists gaming; and the
project's direction — not the backlog's volume — is the thing being optimized.**
