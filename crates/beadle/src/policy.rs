//! Where beadle may apply a label, and where it may only propose one.
//!
//! The policy is an allow-list. The owners beadle may apply labels to are read
//! from `<root>/.beadle/propose-only.yaml` (an `apply_owners:` list), never
//! named in code, and the file is not committed to the public repository. Every
//! other owner, including one the policy has never heard of, is propose-only.
//!
//! It fails closed. A missing, empty or unparsable file makes every repo
//! propose-only. So does a file with any malformed entry: an entry must be a
//! bare login (letters, digits, `-` and `_`), and one entry that is not (a `/`,
//! a `:`, a space or a blank) makes the whole file unparsable. Owners compare
//! without regard to case, as GitHub's do. A target that is not an `owner/name`
//! pair (a tracker, for one) is always propose-only.

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
    apply_owners: Vec<String>,
}

#[derive(Debug, Default)]
pub struct Policy {
    /// Lowercased owners beadle may apply to. `None` is the fail-closed state.
    apply: Option<BTreeSet<String>>,
}

fn is_bare_login(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

impl Policy {
    pub fn load(root: &Path) -> Policy {
        let path = root.join(".beadle/propose-only.yaml");
        let apply = fs::read_to_string(&path)
            .ok()
            .and_then(|t| serde_saphyr::from_str::<Shape>(&t).ok())
            .and_then(|s| {
                s.apply_owners.iter().all(|o| is_bare_login(o)).then(|| {
                    s.apply_owners
                        .iter()
                        .map(|o| o.to_ascii_lowercase())
                        .collect::<BTreeSet<_>>()
                })
            })
            .filter(|o| !o.is_empty());
        Policy { apply }
    }

    pub fn mode(&self, repo: &str) -> Mode {
        let Some(apply) = &self.apply else {
            return Mode::ProposeOnly;
        };
        match repo.split_once('/') {
            Some((owner, name))
                if is_bare_login(owner) && !name.is_empty() && !name.contains('/') =>
            {
                if apply.contains(&owner.to_ascii_lowercase()) {
                    Mode::Apply
                } else {
                    Mode::ProposeOnly
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
            apply: Some(owners.iter().map(|s| s.to_ascii_lowercase()).collect()),
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

    #[test]
    fn only_a_listed_owner_is_applied_to() {
        assert_eq!(with(&["x"]).mode("X/b"), Mode::Apply);
        assert_eq!(with(&["x"]).mode("y/b"), Mode::ProposeOnly);
    }
}
