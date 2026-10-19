//! Unit-test helpers. A binary crate's unit tests cannot reach `tests/`, so
//! the integration-test helper in `tests/support/mod.rs` is mirrored here,
//! backed by the one production constant in `dash.rs`.

#![allow(dead_code)]

/// The em dash (U+2014).
pub fn em_dash() -> char {
    crate::dash::EM_DASH
}

/// `s` with each `{D}` replaced by the em dash.
pub fn dashed(s: &str) -> String {
    s.replace("{D}", &em_dash().to_string())
}
