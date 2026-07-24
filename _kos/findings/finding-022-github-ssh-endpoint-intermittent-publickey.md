# finding-022 — GitHub SSH auth fails intermittently per backend endpoint; identical key accepted seconds later

**Date:** 2026-07-24 · **Run context:** run-17 wrap (pushing feat/pr-channel-cap) · **Class:** tooling friction (capture per aae-orc `.claude/rules/tooling-friction.md`)

## Symptom

`git push` / `git ls-remote` / bare `ssh -T git@github.com` all returned
`git@github.com: Permission denied (publickey)` for ~5 minutes, offering the
correct, long-registered key (`~/.ssh/id_ed25519_github`,
SHA256:HkD3evhf2qSlI9OauLLizAtqkOfeumHpTS70g2mBWos, authenticates as
`arcavenai`). The SAME command with the SAME key succeeded immediately before
and after the window. Verbose transcripts isolated the variable: success
authenticated against `140.82.112.3`, rejections came from `140.82.114.4`.
Server host key verified as GitHub's genuine ed25519 both times — no MITM, no
local config change, agent state irrelevant (key is `IdentitiesOnly` from
disk).

## Diagnosis

Endpoint-dependent auth flakiness on GitHub's SSH front-end pool (backend
`18bbdcb`). Nothing on this machine was wrong.

## Workaround / correct response

**Retry.** Each attempt may land on a different backend; a 4×/3s retry loop
succeeded on attempt 1. Fallbacks if a bad backend persists: HTTPS push with
`-c credential.helper='!gh auth git-credential'`, or SSH over
`ssh.github.com:443` (different pool).

## Anti-pattern this finding exists to prevent

`Permission denied (publickey)` reads as a local key/agent/config problem, and
the obvious moves — re-adding agent identities, editing `~/.ssh/config`,
regenerating keys, touching `core.sshCommand` — are all wrong here and leave
config debris. Before rewiring anything, check whether the failure is
endpoint-intermittent: run `ssh -Tv git@github.com` twice and compare the
connected IP and result.
