# Targets — per-project intent anchors

beadle is generic; nothing about a target is hardcoded. Each target repo declares
its intent in `targets/<project>.intent.yaml`. beadle scores issues/PRs against
that manifest and weights them by what the named **maintainers** actually act on.

## Files

| File | Target | Status |
|---|---|---|
| `vsdd-factory.intent.yaml` | `BOHICA-LABS/vsdd-factory` | populated (v0) |
| `prism.intent.yaml` | DrBothen's Prism | **draft — unpopulated** |
| `marvel.intent.yaml` | `ArcavenAE/marvel` | populated (draft 0.3, not yet ratified by a maintainer) |
| `director.intent.yaml` | `ArcavenAE/director` | populated (draft 0.3, not yet ratified by a maintainer) |

## Which file is read

`beadle` reads `<root>/targets/<target>.intent.yaml`. When that file is absent it reads
the repo-local `<root>/.beadle/intent.yaml`. When both exist, the `targets/` file wins and
the local file is ignored.

## Axes, requirements and evidence (schema 0.7, all optional)

A target may declare its own classification axes and a requirement id pattern. Nothing
about them is in the code: the names, the values and the pattern come from this file, so a
product's own terms stay in that product's intent file.

```yaml
axes:
  - name: lane              # unique within the file
    scoped: true            # true: exactly one value per row; false: any number
    propose: label          # label | row_prefix | marker
    values: [a, b, c]
  - name: stage
    scoped: true
    propose: row_prefix     # taken from the prefix of a cited requirement id
    values: [early, late]
    prefixes: {EA: early, LA: late}
  - name: pace
    scoped: false
    propose: marker         # an unscoped marker, present or absent
    values: [fast]
requirements:
  pattern: "{a}-{n}{l?}"    # the id pattern, see below
  list: requirements.txt    # optional; one id per line, relative to --root
  version: 3                # optional; shown on the board
```

- `propose: label` means the classifier reads the value from the row's own text or labels.
  `row_prefix` derives it from the longest declared prefix of a cited requirement id, and
  leaves the axis unset when no prefix fits (the board counts that row as unassigned).
  `marker` is an unscoped present-or-absent flag.
- A scoped axis takes one value per row. An axis or a value the file does not declare is
  refused at `beadle classify ingest`.
- A file may declare `axes:` without `requirements:`. The board then renders the effort
  sections and no scope coverage.
- The id pattern is a small declared form, not a regular expression. Literal text is
  letters, digits, `-`, `.` and `_`. Tokens: `{n}` one or more digits, `{a}` one or more
  capital letters, `{l}` one or more lowercase letters, `{l?}` zero or one lowercase letter
  (a split row's trailing letter). An id must match whole, case as written. The list file
  never renumbers or repeats an id.
- `beadle requirements <target> --file BODY` prints the ids on a body's `Requirements:` line
  as a JSON array. Ids elsewhere in the text are not read. `classify ingest` records them
  under `requirements` and appends them to `cited_evidence`.
- The evidence axis is the same for every target: `none`, `test`, `live`, `soc-confirmed`.
  Beadle proposes only `none` or `test`, as the label `evidence.<value>`; a payload with
  `live` or `soc-confirmed` is refused.
- Where a label may be applied, and where it is only proposed, is read from
  `<root>/.beadle/propose-only.yaml` (`owners:` list). A missing, empty or unparsable file
  makes every repo propose-only. With a non-empty list, a repo is applied to only when its
  owner is absent from the list. A target that is not an `owner/name` pair is always
  propose-only. `classify ingest` prints one `label-plan:` line per proposed evidence label.
- A target that declares no `axes:` renders the board it always did.

## Adding a target

1. Copy `vsdd-factory.intent.yaml` as a template.
2. Set `target` (repo, default branch, local checkout path).
3. List `maintainers` (the compass) and `measured_contributors` (who's being
   measured).
4. Point `intent_sources` at the repo's real canonical docs (composite, weighted).
   Prefer reading what exists over inventing a new doc — minimize the
   maintenance-rot risk (the GORE lesson).
5. Fill `goals` / `non_goals` / `scope_boundaries` from those sources.
6. Write the `alignment_rubric` (advances / neutral / drifts) with concrete,
   project-specific criteria.

The schema is frontier (charter F1) — expect it to firm up as more targets are
onboarded. Never score against a manifest marked `status: draft-unpopulated`.
