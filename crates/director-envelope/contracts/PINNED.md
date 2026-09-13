# Pinned director envelope contract

beadle is the Rust consumer of the director envelope; marvel is the Go one. The
single source of truth for both is the JSON Schema in the marvel repository.
This directory holds a byte-identical copy so beadle builds and tests without a
marvel checkout (component independence), and the pin below is what keeps the
copy honest: a test hashes the vendored files and fails if they differ from the
sha256 recorded here, so a hand edit cannot pass as the canonical schema and a
refresh is always an explicit commit that names the upstream revision.

## What is pinned

- **Canonical:** `contracts/schema/director-envelope.schema.json` in
  `github.com/ArcavenAE/marvel`, frozen at PR #248 head `415d65e` (branch
  `feat/b69n-contracts`); fixtures from PR #249 head `9a52282` (branch
  `feat/b69n-codegen`), which adds three invalid cases to the same schema.
- **`$id`:** `https://schema.arcaven.com/director/envelope/v1`
- **`director-envelope.schema.json` sha256:**
  `101ce10196f156243a20d8a5e22b07433869286b08a993a3726d9ed06ae127ad`
- **`testdata/` (11 fixtures) sha256:** see `PINNED.sha256`, one line per file,
  checked by the same test.

## Refreshing the pin

1. Copy the schema and `testdata/*.json` from marvel at the new revision.
2. Update the revision and hashes here and in `PINNED.sha256`
   (`shasum -a 256` over each file).
3. `just contracts-gen` to regenerate `src/envelope.gen.rs`.
4. `cargo test -p director-envelope`: the pin guard, the regeneration guard,
   and the fixture replay all have to pass.

No crate or git submodule until a third consumer appears (marvel-builder,
2026-09-12).
