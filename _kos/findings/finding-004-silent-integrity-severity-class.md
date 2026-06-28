# finding-004 — we underweighted silent integrity / source-of-truth corruption

Date: 2026-06-28
Probe: user challenge during run-2 review — "are we correctly assessing the impact
of data loss?" then escalated: "we could be losing LEARNING, which is more damning
than losing code" and "or losing spec, losing process steps and more."
Nodes touched: `classify` engine (skill step 3); charter F-series severity model.

## What we got wrong

In the run-2 dashboard, #313 (Phase-1 CI artifacts produced but never committed) was
scored **"high (data-loss)"** and bucketed under "Gate integrity & data-safety,"
ranked *below* #314. That severity treated data-loss as "lose some output, regenerate
it" — true for an ordinary product, **wrong for a self-referential ratchet factory.**

The correct read: #313 and #314 are not a "CI cluster." They are the SAME disease —
**source-of-truth corruption** — from two sides:

- **#313:** the ratchet records a PASS against artifacts that exist in CI but not on
  disk/in-repo. The system of record asserts a state that isn't real. Every later
  cycle builds on a false ground truth.
- **#314:** the input-hash includes frontmatter, so the ratchet's notion of "changed"
  is fiction (phantom drift on 11 specs). The hash — a system of record — disagrees
  with reality.

Both make a **system of record disagree with reality without raising a signal**, and
the disagreement **propagates to dependent cycles.** That is categorically worse than
losing output.

## The recoverability hierarchy (why "losing learning is more damning than code")

Rank what a cycle produces by recoverability, not by byte count:

| Lost | Recoverable? | Cost |
|---|---|---|
| **Code / generated output** | Yes — re-run the generator | tokens, time (the *best* case) |
| **Spec / process steps** | Partially — but the spec is the AUTHORITY code is judged against (spec wins; only a human amends it). Lose it and code has no referent to be correct against | integrity — the ratchet loses its referent |
| **Learning** (adversarial findings, decision log D-NNN, convergence history, what was already ruled out) | **No** — accumulated uncertainty-reduction. You can re-run the generator; you cannot cheaply re-derive *why* a path was abandoned cycles ago | irreplaceable — the factory's whole reason to exist |

Code is the CHEAPEST thing a self-referential factory produces. The expensive,
irreplaceable output is the learning + spec + process record — and that is exactly
what lives in the `.factory/` artifacts most likely to be "produced but not committed"
or silently overwritten.

**Local evidence (2026-06-28):** the vsdd-factory `develop` checkout's `.factory/`
contains only `logs/` — no STATE.md, no decision log, no adversary findings, no
convergence record on disk. The irreplaceable tier is the tier at risk. #313 is not
hypothetical.

## The missing axis (the actual beadle lesson)

beadle's `classify` engine has **severity** (impact), **priority** (urgency), and
**leverage** (systemic↔minutiae). It has NO axis for **blast-radius visibility ×
compounding**. That is the gap that let #313 score merely "high."

A loud crash that loses a file is *low* in this frame — you see it, you re-run. A
silent integrity gap that the green checkmark CERTIFIES as fine while it corrupts the
ground truth for every future cycle is the **highest** class, because: (a) nobody is
alerted, (b) it compounds, (c) by the time symptoms surface the corrupted state is
baked into N cycles of history.

New defect class for the classify engine:

> **Silent integrity / source-of-truth corruption** — the fault makes a system of
> record (ratchet, spec, hash, learning/decision store, index) disagree with reality
> *without raising a signal*, and the disagreement propagates to dependent cycles or
> artifacts. **Severity = highest, regardless of how small the triggering bug looks**,
> because the cost is unbounded and discovery is delayed. The severity comes from
> invisibility + compounding, not from the magnitude of the immediate loss.

Recoverability is a severity input: a fault that risks the irreplaceable tier
(learning/spec/process) outranks one that risks only regenerable output, even at equal
"size."

## Integrity gates functional (the precedence principle)

The user sharpened this past "higher severity" into a precedence rule:

> "convergence while important is lower in priority than integrity. it's a functional
> requirement, but if we cannot trust the underlying substrate, then there is no
> trustworthy functional feature even if it's written correctly."

Integrity is **foundational**; convergence / gate-correctness / feature behavior are
**functional**. Functional properties are *computed over* the substrate (ratchet
state, spec, hash, learning store). If the substrate can't be trusted, a functional
verdict — including a "converged / PASS" — is **unfalsifiable**: correct code on a
false substrate still yields a false result; a green check certifies nothing.

So a source-of-truth integrity defect does not merely *outrank* a convergence defect
on impact — it **gates the validity of every functional verdict, convergence
included.** This is why #313/#314 (integrity) sit ABOVE #314/#305/#309
(convergence-soundness) in the action plan: not because they hurt more, but because an
open integrity defect on the substrate makes the convergence verdict computed over
that substrate untrustworthy regardless of how correctly convergence itself is
implemented.

**Classify rule:** never rank a functional item (however important) above an open
integrity defect on the same substrate it depends on. The **Source-of-truth
integrity** group is always P0, above convergence-soundness.

## Fix applied (skill — classify engine)

`skills/beadle-triage/SKILL.md` step 3 (classify) gains:
- a **silent-integrity / source-of-truth corruption** escalation: detect when a fault
  makes a system of record disagree with reality without a signal, or threatens the
  irreplaceable (learning/spec/process) tier → escalate severity to top regardless of
  apparent size; never let a green check or small diff mask it.
- recoverability as a severity input (regenerable output < spec/process < learning).

## Charter delta (kos harvest — update the nodes, then the charter projection)

The classify model (currently severity/priority/leverage) should record a fourth
consideration: **blast-radius visibility × compounding**, with **silent integrity /
source-of-truth corruption** as its top class and **recoverability tier** as a
severity input. It should also record the **integrity-gates-functional precedence
rule**: integrity is foundational and gates the validity of every functional verdict
(convergence included), so an open integrity defect is always ranked above a
functional one on the same substrate — this is a precedence relationship, not just a
severity ordering. This is the first concrete severity-axis requirement and should be
reconciled with the issue/defect taxonomy research (workflow we6yrrcba) when it lands —
the axis is adjacent to ODC data/timing defect types and the severity-vs-priority
literature, but the user surfaced it ahead of the research, so it is baked in now.

## Re-scored verdict for the live board (apply on next refresh)

- #313 and #314 → promote to a top **P0 "Source-of-truth integrity"** group, ABOVE
  the convergence-soundness group. Both are silent-integrity corruption of the
  ratchet. #313 specifically threatens the irreplaceable tier (learning/spec/process
  produced-but-not-committed).
- This is propose-not-act (B2): beadle surfaces the re-scoring and the P0 group; the
  maintainer decides. The escalation is well inside beadle's read/analyze authority.
