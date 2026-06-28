# finding-009 — operational-impact axis: does the defect stop the factory?

Date: 2026-06-28 (session-051)
Probe: the user asked whether we track which reported GitHub issues *halt the dark
factory* — pause it pending manual intervention, or crash the Claude Code session
(panic, losing context). Answer after checking the label schema + beadle taxonomy:
**no — confirmed gap.** Nothing in `labels/schema.yaml` or beadle's nodes captured
operational/liveness impact. `status.blocked` / `status.needs-human` are
triage-workflow states of the *issue*, not the runtime liveness of the *factory*;
silent-integrity measures *substrate correctness*, orthogonal to whether the loop
is still running.

Nodes touched: new `elem-operational-impact-axis` (bedrock); relates
`elem-silent-integrity-severity` (the safety half vs this liveness half);
new label scope `impact.*` in `../labels/schema.yaml`.

## The decision: adopt the standard, don't invent

The user's instruction was to ground the axis in our standards research, adopt it
if we can, and decide whether to request a vsdd-factory change in support.

**What the research found** (IEEE 1044-2009 PDF, IBM ODC Wayback reference,
Lamport/Schneider primary PDFs — all primary-source-verified):

- **IEEE 1044-2009 has NO crash/halt value.** Its "Effect" attribute is only
  Functionality / Usability / Security / Performance / Serviceability / Other; a
  crash is `Effect=Functionality`. Work-stoppage lives in IEEE 1044 **Severity**:
  `Blocking` ("testing inhibited/suspended pending correction or workaround") and
  `Critical` ("essential operations unavoidably disrupted"). (Negative result.)
- **ODC's "Impact" attribute is the one standard with an explicit home.** Its
  `Reliability` value is verbatim-defined to cover "unplanned interruption ...
  ABEND and WAIT" — example "system crashed and had to be rebooted." **ABEND =
  crash, WAIT = hang.** ODC folds both into one value.
- **The failure-model literature splits them**, and the split is operationally
  load-bearing: a **crash** (crash/fail-stop, Schlichting & Schneider 1983) halts
  and loses context but is **detectable**; a **halt** (omission / liveness
  violation, Cristian 1991 / Alpern & Schneider 1985) stays alive with state
  preserved but **looks healthy while making no progress** — the "gray failure"
  trap (Huang 2017). No single standard term names "alive-but-stuck-needs-a-human";
  it is formally a liveness violation.

**The adopted shape:** take ODC's attribute NAME (`impact.*`) and decompose its
Reliability value along the failure-model spine into ordered values:

| value | what | recovery cost | detectability |
|---|---|---|---|
| `impact.panic` | session crashes, context lost | **worst** | self-announcing |
| `impact.halt`  | blocks awaiting manual intervention, state kept | major | **worst** (gray failure) |
| `impact.data-loss` | irreplaceable state destroyed/corrupted | — | varies |
| `impact.degraded` | runs but impaired (fail-slow/fail-stutter) | low | varies |

Ordering twist encoded in the node: panic > halt on *recovery cost*; halt > panic
on *detection difficulty*. Both major; degraded below both. The axis is
**orthogonal to defect-nature** — a trivial typo can be `impact.panic`, which is
the whole point of giving it its own tag.

## Adopted now (this session)

1. **Label schema** — `impact.*` exclusive scope added to `labels/schema.yaml`
   universal section (panic/halt/data-loss/degraded), each with the ODC +
   failure-model citation in its description. Fleet-wide; rolls out via
   `provision.sh <repo>` on the maintainer's cadence.
2. **beadle recognizes it** — `elem-operational-impact-axis` bedrock node; beadle
   scores this axis on every artifact, alongside (not inside) silent-integrity.
   The skill should set `impact.*` autonomously (it is a classification, like
   `type.*`), pairing it with priority but never collapsing into it.

## Requested of vsdd-factory (PROPOSAL — not posted; B2 propose-not-act)

The second half — the factory self-flagging at report time — is an upstream
behavior change in `drbothen/vsdd-factory`, so it is captured here as a contract
to propose, NOT posted. Recommended ask:

- When the factory files (or auto-files) an issue for a defect it encountered, and
  that defect **stopped or crashed a factory run**, it should stamp the issue with
  the operational impact: `impact.panic` if a session terminated abnormally /
  context was lost, `impact.halt` if the run paused awaiting manual intervention.
  The factory has the ground truth beadle can only infer — it *knows* whether the
  loop stopped and whether the session died.
- Mechanism options (factory's choice — state WHAT/WHY, not HOW, per beadle
  INC-002): a label at file time, a structured marker in the issue body that beadle
  reads, or a field in the dispatcher/run telemetry beadle joins against. The
  telemetry route also feeds critic's per-run health detector (the resolver-storm
  extractor already counts dispatcher errors; a panic/halt is the same class of
  per-run liveness signal).
- This is a beadle↔factory contract, parallel to the OTEL attribution ask
  (finding-007 / #324): factory emits ground-truth operational signal; beadle
  classifies + reports; critic measures across runs (a high `impact.panic` rate is
  a factory regression critic should flag — defect-class-weighted efficiency
  per finding-008 / boundary-beadle).

**Not posted upstream.** Whether/when to raise it with drbothen is the user's call.

## Consequence for the two siblings

- beadle: classify `impact.*` on every artifact; it is the **liveness** top-axis,
  with silent-integrity as the **safety** top-axis. Independent — score both.
- critic: a per-run `impact.panic`/`impact.halt` count is a liveness breakdown
  signal in the same family as dispatcher-health; it weights into class-weighted
  efficiency (a factory that panics often is spending tokens to lose context).
