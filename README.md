# beadle

> The minor official who ushers, keeps order, makes the announcements, and tends
> the records. A *beadle* keeps the **beads** — the project's issues — and keeps
> the production aligned to what it's *for* without dictating the performance.

**beadle** is an intent-aligned issue-triage and dashboard system for software
projects. It is the next generation of the multiclaude **envoy** role, extended
with a single load-bearing idea nobody else ships:

> **It scores each issue / PR / contribution against the project's *declared,
> versioned intent*, weighted by what the *maintainers actually act on* — and
> renders the result as a living, bot-maintained dashboard whose state lives
> out-of-band.**

It answers the question a label-slapping bot can't: *is this contribution moving
the project toward what it is for — and are we being led into the weeds?*

## Why this exists

A prolific contributor (often an AI agent) can file sound, well-written issues
faster than maintainers can absorb them. Each issue is individually defensible;
in aggregate they can pull a project off its roadmap, drown high-leverage work in
minutiae, and quietly invert the cost of being helpful. Existing triage bots
(Triage Party, Dosu, Renovate's dashboard, stale-bot, Copilot) label, summarize,
dedup, or auto-close on a cheap signal. None of them measures **work against
intent**, and none weights it by **maintainer engagement** rather than raw volume
or reaction-popularity. That intersection is empty. beadle owns it.

See [`DESIGN.md`](./DESIGN.md) for the full architecture and
[`docs/prior-art.md`](./docs/prior-art.md) for the survey that located the gap.

## What it does (primary goals)

1. **Maintains a living triage dashboard** as a single pinned GitHub issue —
   regenerated each run from out-of-band state (the Renovate render-loop pattern,
   without Renovate's state-in-Markdown fragility). Statistical progress, a
   prioritized action plan, a human-readable classification index, and an
   **intent-alignment / drift** panel.
2. **Absorbs the envoy role**: greet reporters, screen issues through a
   three-layer firewall (deterministic gates → lightweight AI → escalate),
   apply scoped labels with mutual-exclusivity discipline, cross-check merged PRs
   against open issues, and keep reporters informed — relaying, never authorizing.
3. **Scores intent-alignment**: each issue/PR gets a graded verdict (advances /
   neutral / drifts) against the target project's declared intent, with cited
   rationale — never a bare label.
4. **Surfaces the meta-signal**: minutiae ratio, filed-vs-acted-upon, backlog
   net-flow, scope-drift candidates, and "led-by-the-backlog" warnings — so a
   maintainer can see at a glance whether the project is on-course.

## Guiding philosophy

beadle is bound by three layered sources (see [`docs/PHILOSOPHY.md`](./docs/PHILOSOPHY.md)):

- **aae-orc `SOUL.md`** — sovereignty, composability ("no component conscripts
  another"), observable-but-never-mandatory, **gradual elaboration**, parallel
  safety, spec-driven development.
- **The StrongDM Software Factory manifesto** — humans own *intent, held-out
  scenarios, and satisfaction scoring*; **information asymmetry** keeps the
  evaluation criteria out of the agent's view; **probabilistic satisfaction**
  beats binary pass/fail; deliberate naivety ("why am I doing this?").
- **The human-AI collaboration canon** — meaningful human control, propose-not-act
  for anything consequential or public, confidence-as-frequency, cheap-to-verify
  evidence over fluent prose, reversibility, and anti-Goodhart metric hygiene.

## Status & shape

beadle elaborates gradually. It is useful at every phase and over-built at none.

| Phase | Trigger | Agents | State | Surface | Packaging |
|---|---|---|---|---|---|
| **0 — now** | a Claude Code skill | one session | JSONL + the dashboard issue | one pinned dashboard issue | `skills/beadle-triage/` |
| **1** | scheduled `gh-aw` run *as arcavenai* | orchestrator + classifier | embedded store | + checkbox controls | a sideshow-pack |
| **2** | a marvel-orchestrated team | supervisor + classifiers + investigators + fresh-context verifier | **Dolt** (versioned SQL) | + optional Projects v2 rollup | pack runs on marvel ("multiclaude v2") |

**First targets:** [vsdd-factory](./targets/vsdd-factory.intent.yaml) and
DrBothen's [Prism](./targets/prism.intent.yaml). Generic for any repo — each
target declares its own intent anchor; nothing is hardcoded.

## Installation

beadle ships as a signed, notarized macOS binary (Apple Silicon) and Linux
binaries (`amd64`, `arm64`) via GitHub Releases. **The first stable release is
pending** — every release today is a prerelease from `main`. Everyday use is
still primarily via the Phase-0 Claude Code skill at `skills/beadle-triage/`;
the binary is here for automation and the Phase-1+ hooks.

### Option 1: Homebrew (macOS arm64)

```bash
brew install arcavenae/tap/beadle
```

Until the first stable release is cut, the tap tracks the latest alpha.

### Option 2: Install with mise

[mise](https://mise.jdx.dev/) is a polyglot version manager. It reads a
per-project `mise.toml`, pulls the exact signed binary from GitHub Releases,
and verifies GitHub Artifact Attestations natively — no Homebrew tap required.

**Stable** — no stable release exists yet; this block starts working once the
first `v*` tag lands:

```bash
mise use github:ArcavenAE/beadle@latest
beadle --version
```

**Alpha channel** (prereleases from `main`) — add `prerelease = true` to opt in
per-tool. Alpha binaries are not `-a`-suffixed for beadle, so stable and alpha
share the `beadle` shim (installing both concurrently is not supported until
we split the release-asset naming):

```toml
# mise.toml
[tools]
"github:ArcavenAE/beadle" = { version = "latest", prerelease = true }
```

```bash
mise install
beadle --version
```

**macOS troubleshooting** — if a quarantine-aware host propagates
`com.apple.quarantine` into the mise install and Gatekeeper prompts, clear it
once:

```bash
xattr -d com.apple.quarantine "$(mise which beadle)"
```

### Option 3: Download Pre-built Binary

Download the latest release from [GitHub Releases](https://github.com/ArcavenAE/beadle/releases). Binaries are available for macOS (arm64) and Linux (amd64, arm64).

## Running it (as arcavenai)

beadle rewrites **its own** dashboard issue each run, so the issue must be
created and maintained by the managing bot identity — **`arcavenai`**, not a human
account (an issue's author can't be reassigned). To create/refresh a target's
dashboard, run [`prompts/create-dashboard.md`](./prompts/create-dashboard.md) from
the beadle repo root **on a system whose active GitHub identity is `arcavenai`**.
It carries a hard identity guard (`gh api user` must be `arcavenai`/`arcavenai[bot]`,
abort otherwise) so the post can never be made under the wrong identity. Phase 1
replaces the manual launcher with a scheduled `gh-aw` workflow running as the
arcavenai GitHub App — same identity, same idempotent rewrite.

## Layout

```
beadle/
├── DESIGN.md            full architecture synthesis
├── charter.md           bedrock / frontier / graveyard (kos process)
├── docs/
│   ├── PHILOSOPHY.md    the three guiding-light sources (cited)
│   ├── prior-art.md     comparative survey + the white space
│   ├── research-notes.md triage, dashboarding, human-AI findings
│   ├── dashboard-schema.md  the issue-as-dashboard spec + out-of-band state
│   └── agent-team.md    team shape (envoy-absorbed) + agent-definition format
├── agents/              beadle's own agent definitions
├── decisions/           ADRs (MADR format)
├── _kos/                knowledge graph (bedrock/frontier/graveyard nodes)
├── targets/             per-project intent anchors (vsdd-factory, prism, ...)
├── skills/beadle-triage/  the Phase-0 MVP skill
├── prompts/             runnable launchers (create-dashboard.md — run as arcavenai)
└── pack.yaml            the sideshow-pack manifest (Phase-1+ target shape)
```

## License

MIT. See [`LICENSE`](./LICENSE).
