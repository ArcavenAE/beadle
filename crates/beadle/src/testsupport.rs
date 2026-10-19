//! Unit-test copy of `tests/support/mod.rs::em_dash`. A binary crate's unit
//! tests cannot reach `tests/`, so the helper is repeated here; the spelling
//! scan allows these two files and no others to build the character.

#![allow(dead_code)]

/// The em dash (U+2014), built at run time so no source file spells it.
pub fn em_dash() -> char {
    char::from_u32(0x2013 + 1).unwrap()
}

/// `s` with each `{D}` replaced by the em dash.
pub fn dashed(s: &str) -> String {
    s.replace("{D}", &em_dash().to_string())
}
