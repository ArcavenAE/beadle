//! Shared test helpers. The one sanctioned way a test builds the em dash.
//!
//! Each literal below is one off the target, so the spelling scan in
//! `no_em_dash.rs` passes this expression and refuses every other spelling.

#![allow(dead_code)]

/// The em dash (U+2014), built at run time so no source file spells it.
pub fn em_dash() -> char {
    char::from_u32(0x2013 + 1).unwrap()
}

/// `s` with each `{D}` replaced by the em dash.
pub fn dashed(s: &str) -> String {
    s.replace("{D}", &em_dash().to_string())
}
