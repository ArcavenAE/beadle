//! Where beadle may apply a label, and where it may only propose one.
//!
//! The owners that are propose-only are read from `<root>/.beadle/propose-only.yaml`
//! (`owners:` list), never named in code. The policy fails closed: a missing,
//! empty or unreadable file, or one that does not parse, makes every repo
//! propose-only. With a non-empty list, a repo may be applied to only when its
//! owner is absent from the list. A target that is not an `owner/name` pair
//! (a tracker, for one) is always propose-only.

use std::{collections::BTreeSet, fs, path::Path};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Apply,
    ProposeOnly,
}

impl Mode {
    pub fn as_str(self) -> &'static str {
        match self {
            Mode::Apply => "apply",
            Mode::ProposeOnly => "propose-only",
        }
    }
}

#[derive(serde::Deserialize)]
struct Shape {
    owners: Vec<String>,
}

#[derive(Debug, Default)]
pub struct Policy {
    /// `None` is the fail-closed state.
    owners: Option<BTreeSet<String>>,
}

impl Policy {
    pub fn load(root: &Path) -> Policy {
        let path = root.join(".beadle/propose-only.yaml");
        let owners = fs::read_to_string(&path)
            .ok()
            .and_then(|t| serde_saphyr::from_str::<Shape>(&t).ok())
            .map(|s| {
                s.owners
                    .into_iter()
                    .map(|o| o.trim().to_string())
                    .collect::<BTreeSet<_>>()
            })
            .filter(|o| !o.is_empty() && !o.contains(""));
        Policy { owners }
    }

    pub fn mode(&self, repo: &str) -> Mode {
        let Some(owners) = &self.owners else {
            return Mode::ProposeOnly;
        };
        match repo.split_once('/') {
            Some((owner, name))
                if !owner.is_empty()
                    && !name.is_empty()
                    && !name.contains('/')
                    && !owner.contains(':') =>
            {
                if owners.contains(owner) {
                    Mode::ProposeOnly
                } else {
                    Mode::Apply
                }
            }
            _ => Mode::ProposeOnly,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn with(owners: &[&str]) -> Policy {
        Policy {
            owners: Some(owners.iter().map(|s| s.to_string()).collect()),
        }
    }

    #[test]
    fn the_default_policy_applies_nowhere() {
        assert_eq!(Policy::default().mode("a/b"), Mode::ProposeOnly);
    }

    #[test]
    fn a_tracker_target_is_never_applied_to() {
        assert_eq!(with(&["x"]).mode("jira:PROJ"), Mode::ProposeOnly);
        assert_eq!(with(&["x"]).mode("PROJ"), Mode::ProposeOnly);
    }
}
