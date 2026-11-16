//! The evidence axis, the same for every target: `none`, `test`, `live`,
//! `soc-confirmed`. Beadle may propose only the first two. The last two record
//! something a person confirmed, so beadle never proposes or applies them.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Evidence {
    None,
    Test,
    Live,
    SocConfirmed,
}

impl Evidence {
    pub fn as_str(self) -> &'static str {
        match self {
            Evidence::None => "none",
            Evidence::Test => "test",
            Evidence::Live => "live",
            Evidence::SocConfirmed => "soc-confirmed",
        }
    }

    pub fn parse(s: &str) -> Option<Evidence> {
        [
            Evidence::None,
            Evidence::Test,
            Evidence::Live,
            Evidence::SocConfirmed,
        ]
        .into_iter()
        .find(|e| e.as_str() == s)
    }

    /// Whether beadle may propose this value.
    pub fn beadle_may_propose(self) -> bool {
        matches!(self, Evidence::None | Evidence::Test)
    }
}

/// The evidence value beadle proposes for a row: `test` when a test is cited,
/// else `none`. Never anything above `test`.
#[allow(dead_code)] // the classifier skill decides; kept so the bound is one tested function
pub fn propose(cites_a_test: bool) -> Evidence {
    if cites_a_test {
        Evidence::Test
    } else {
        Evidence::None
    }
}

/// Check an evidence value on a payload from the classifier.
pub fn check_proposal(s: &str) -> Result<Evidence, String> {
    match Evidence::parse(s) {
        Some(e) if e.beadle_may_propose() => Ok(e),
        Some(e) => Err(format!(
            "evidence `{}` is set by a person; beadle proposes only `none` or `test`",
            e.as_str()
        )),
        None => Err(format!("unknown evidence `{s}` (use `none` or `test`)")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn beadle_never_proposes_a_confirmed_value() {
        for cites in [false, true] {
            assert!(propose(cites).beadle_may_propose());
        }
        assert!(check_proposal("live").is_err());
        assert!(check_proposal("soc-confirmed").is_err());
        assert_eq!(check_proposal("test"), Ok(Evidence::Test));
    }
}
