# Rust Coding Rules

## Safety

```rust
// Every binary and library crate
#![forbid(unsafe_code)]
```

- No `unwrap()` in production code — use `?` or `expect()` with actionable message
- `unwrap()` is acceptable in tests

## Type Design

- **Newtypes for IDs:** `NodeId(String)`, `FindingId(String)` — prevents mixing ID types
- **Validated constructors at trust boundaries:** `new()` validates (CLI input, file parsing)
- **`new_unchecked()` for tests and trusted internal sources**
- **`#[non_exhaustive]` on enums that will grow** — forces callers to handle future variants
- **Private fields with getters** on types where invariants must hold

## Error Handling

```rust
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    // Semantic variants per domain
}

pub type Result<T> = std::result::Result<T, Error>;
```

- Use `thiserror` for error enums — structured variants, not string bags
- Define `pub type Result<T>` per crate for ergonomics
- `Display` impl is for user-facing output in a CLI — be clear and actionable

## Dependencies

- Edition 2024, MSRV tracked in `rust-toolchain.toml`
- Workspace-level dependency declarations in root `Cargo.toml` (if workspace)
- Use `cargo clippy -- -D warnings` — warnings are errors
- Use `cargo deny check` — advisories, licenses, bans, sources

## Testing

- Unit: `#[cfg(test)] mod tests {}` in same file
- Integration: `tests/` directory, named by feature
- Test names as documentation: `node_confidence_rejects_invalid()`, not `test_1()`
- Test boundaries: empty, missing fields, invalid values, broken edge cases
- Snapshot tests with `insta` for stable-output renderers

## Common mistakes to avoid

1. Don't `.unwrap()` outside tests — propagate errors with `?`
2. Don't `.clone()` reflexively — prefer borrows, then `Cow` when needed
3. Don't introduce `async` without a reason — sync is cheaper to reason about
4. Don't mix sync and async in the same layer — pick a discipline per crate
5. Don't write `impl From<&str> for ...` when a named constructor reads better
6. Don't use `String` where `&str` suffices in parameters
7. Don't define a trait for a single implementor — concrete types first

## Formatting & linting

- Formatter: `cargo +nightly fmt --all` (config in `rustfmt.toml`)
- Linter: `cargo clippy --all-targets --all-features -- -D warnings`
- Both enforced in CI and by lefthook (`lefthook.yml`)
- Never disable lints with `#[allow(...)]` without a justifying comment
