//! The em dash (U+2014), named once.
//!
//! The gates have to recognise the character in a candidate body, so non-test
//! code needs it. It is built from a value that is one off the target, so the
//! spelling scan in `tests/no_em_dash.rs` passes this file and refuses every
//! other source that spells or assembles it. This file and
//! `tests/support/mod.rs` are the only two the scan allows to do so.

/// The em dash.
pub const EM_DASH: char = match char::from_u32(0x2013 + 1) {
    Some(c) => c,
    None => '-',
};
