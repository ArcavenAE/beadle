# beadle — task runner (Phase 0: docs + manifests; no compiled binary yet)

# List recipes
default:
    @just --list

# Lint markdown and YAML (best-effort; tools optional in Phase 0)
lint:
    @command -v markdownlint >/dev/null 2>&1 && markdownlint '**/*.md' --ignore node_modules || echo "markdownlint not installed — skipping"
    @command -v yamllint >/dev/null 2>&1 && yamllint . || echo "yamllint not installed — skipping"

# Validate intent manifests + pack.yaml parse as YAML and declare schema_version
check:
    @for f in pack.yaml targets/*.intent.yaml _kos/kos.yaml; do \
        python3 -c "import sys,yaml; d=yaml.safe_load(open('$f')); assert 'schema_version' in d, 'missing schema_version in $f'; print('ok: $f')" ; \
    done

# Render the philosophy doc with citations (requires pandoc + references.bib)
docs:
    @command -v pandoc >/dev/null 2>&1 && pandoc docs/PHILOSOPHY.md --citeproc --bibliography=references.bib -o /tmp/beadle-philosophy.html && echo "rendered -> /tmp/beadle-philosophy.html" || echo "pandoc not installed — skipping"

# Everything CI runs
ci: lint check
