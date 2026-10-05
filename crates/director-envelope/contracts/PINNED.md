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
  `github.com/ArcavenAE/marvel`, at `768788a` (marvel#464, which adds the
  global tier's `global://director` and `global://{cluster}/supervisor`
  recipient forms, marvel#457). It carries marvel#449 (authority optional,
  absent means strength `none`; director#197) and marvel#447
  (`sender.instance`). Fixtures from the same revision:
  `contracts/schema/testdata/*.json` at the top level, 25 files. The nested
  `testdata/event/` fixtures belong to the event schema and are not vendored.
- **`$id`:** `https://schema.arcaven.com/director/envelope/v1`
- **`director-envelope.schema.json` sha256:**
  `5b6332d3438ed38e1d42ef172a589254fe0d977dffb18ebec6bc56e2d7a20c2c`
- **`testdata/` (25 fixtures) sha256:** see `PINNED.sha256`, one line per file,
  checked by the same test.
- **Reading an absent authority:** the schema makes `authority` optional, and
  `Envelope::effective_authority()` (in `src/lib.rs`) reads an absent block as
  strength `none` with no seat. Use it instead of reading `authority` directly.

## Refreshing the pin

1. Copy the schema and `testdata/*.json` from marvel at the new revision.
2. Update the revision and hashes here and in `PINNED.sha256`
   (`shasum -a 256` over each file).
3. `just contracts-gen` to regenerate `src/envelope.gen.rs`.
4. `cargo test -p director-envelope`: the pin guard, the regeneration guard,
   and the fixture replay all have to pass.

No crate or git submodule until a third consumer appears (marvel-builder,
2026-09-12).
