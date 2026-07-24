# finding-021 — `just check` assumes PyYAML in system python; fails wholesale on clean machines

**Date:** 2026-07-24 · **Run context:** run-17 wrap (manifest v0.5 edit) · **Class:** tooling friction (capture-before-workaround per aae-orc `.claude/rules/tooling-friction.md`)

## Symptom

`just check` exits 1 on mokuzai with `ModuleNotFoundError: No module named 'yaml'`
for **all five** YAML files (pack.yaml, three intent manifests, _kos/kos.yaml) —
including files untouched by the session. The recipe (justfile line 14–15) shells
out to bare `python3 -c "import yaml; ..."` per file; nothing declares or
bootstraps the PyYAML dependency. Any machine whose system python lacks PyYAML
fails validation regardless of manifest correctness. `just lint` degrades
gracefully on the same machine (markdownlint/yamllint probe with `command -v`
and skip); `check` does not.

## Workaround applied (this session)

Ran the same per-file validation through `uvx --with pyyaml python3 -c ...`
(uv-managed ephemeral env, per the global uv/uvx preference) — all five files
pass, including the new v0.5 `maintainer_capacity.pr_channel` block.

## Why this is happening

The recipe was authored on a machine where system python had PyYAML (brew
python site-packages, most likely). The dependency was invisible at authoring
time — classic works-on-my-machine env coupling in a repo whose CI (if/when it
runs `just check`) would install deps explicitly.

## Fix shape (not applied here — one-line recipe change)

Point the recipe at uv: `uvx --with pyyaml python3 -c ...` (or probe like the
lint recipe does and emit an actionable skip). Cheap, bounded; belongs in a
hygiene commit touching only the justfile.
