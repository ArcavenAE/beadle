//! Per-target classification axes, declared in the target's intent file.
//!
//! No axis name or value lives in code. A target declares each axis under a
//! top-level `axes:` list; a classification record carries values for them under
//! `axes`, and the board counts rows by value. Declared shape:
//!
//! ```yaml
//! axes:
//!   - name: lane            # unique within the file
//!     scoped: true          # true: exactly one value per row; false: any number
//!     propose: label        # label | row_prefix | marker
//!     values: [a, b, c]
//!   - name: stage
//!     scoped: true
//!     propose: row_prefix   # derived from the prefix of a cited requirement id
//!     values: [early, late]
//!     prefixes: {EA: early, LA: late}
//!   - name: pace
//!     scoped: false
//!     propose: marker       # an unscoped marker: its value is present or absent
//!     values: [fast]
//! requirements:
//!   pattern: "{a}-{n}{l?}"  # one form, or a list of forms; see requirements.rs
//!   list: requirements.txt  # relative to --root; optional
//!   version: 3              # optional
//! ```

use std::collections::BTreeMap;

use serde::Deserialize;

use crate::requirements::PatternSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Propose {
    /// The classifier reads the value from the row's own text or labels.
    Label,
    /// Derived from the prefix of a cited requirement id; left unset otherwise.
    RowPrefix,
    /// An unscoped marker: present or absent.
    Marker,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AxisDef {
    pub name: String,
    pub scoped: bool,
    pub propose: Propose,
    pub values: Vec<String>,
    pub prefixes: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequirementsDecl {
    /// One or more id forms; an id fits when it fits any of them.
    pub patterns: Vec<String>,
    pub list: Option<String>,
    pub version: Option<String>,
}

/// What a target declares, as read from its intent file.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Declared {
    pub axes: Vec<AxisDef>,
    pub requirements: Option<RequirementsDecl>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Scalar {
    S(String),
    I(u64),
}

#[derive(Deserialize)]
#[serde(untagged)]
enum OneOrMany {
    One(String),
    Many(Vec<String>),
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AxisShape {
    name: String,
    #[serde(default)]
    scoped: bool,
    propose: String,
    values: Vec<String>,
    #[serde(default)]
    prefixes: BTreeMap<String, String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ReqShape {
    pattern: OneOrMany,
    list: Option<String>,
    version: Option<Scalar>,
}

#[derive(Deserialize)]
struct Shape {
    #[serde(default)]
    axes: Vec<AxisShape>,
    requirements: Option<ReqShape>,
}

/// Read the `axes:` and `requirements:` sections of an intent file. A file
/// that declares neither yields the empty `Declared`. Anything malformed is an
/// error naming what is wrong.
pub fn read(text: &str) -> Result<Declared, String> {
    let shape = serde_saphyr::from_str::<Shape>(text).map_err(|e| {
        let line = e.location().map_or(0, |l| l.line());
        format!(
            "line {line}: {}",
            e.to_string().lines().next().unwrap_or("")
        )
    })?;
    let mut axes: Vec<AxisDef> = Vec::new();
    for a in shape.axes {
        let propose = match a.propose.as_str() {
            "label" => Propose::Label,
            "row_prefix" => Propose::RowPrefix,
            "marker" => Propose::Marker,
            other => {
                return Err(format!(
                    "axis `{}`: propose `{other}` is not label, row_prefix or marker",
                    a.name
                ));
            }
        };
        if a.name.trim().is_empty() || a.name != a.name.trim() {
            return Err("an axis needs a name without leading or trailing space".to_string());
        }
        if axes.iter().any(|x| x.name == a.name) {
            return Err(format!("axis `{}` is declared twice", a.name));
        }
        if a.values.is_empty() {
            return Err(format!("axis `{}` declares no values", a.name));
        }
        let mut seen = std::collections::BTreeSet::new();
        for v in &a.values {
            if v.trim().is_empty() || !seen.insert(v) {
                return Err(format!(
                    "axis `{}`: value `{v}` is blank or repeated",
                    a.name
                ));
            }
        }
        if propose == Propose::Marker && a.scoped {
            return Err(format!("axis `{}`: a marker is unscoped", a.name));
        }
        if propose == Propose::RowPrefix && a.prefixes.is_empty() {
            return Err(format!(
                "axis `{}`: row_prefix needs a prefixes map",
                a.name
            ));
        }
        if propose != Propose::RowPrefix && !a.prefixes.is_empty() {
            return Err(format!(
                "axis `{}`: prefixes belong to row_prefix only",
                a.name
            ));
        }
        if let Some((p, v)) = a.prefixes.iter().find(|(_, v)| !a.values.contains(v)) {
            return Err(format!(
                "axis `{}`: prefix `{p}` maps to `{v}`, which is not a value",
                a.name
            ));
        }
        axes.push(AxisDef {
            name: a.name,
            scoped: a.scoped,
            propose,
            values: a.values,
            prefixes: a.prefixes,
        });
    }
    let requirements = match shape.requirements {
        None => None,
        Some(r) => {
            let patterns = match r.pattern {
                OneOrMany::One(p) => vec![p],
                OneOrMany::Many(v) => v,
            };
            PatternSet::new(&patterns).map_err(|e| format!("requirements.pattern: {e}"))?;
            Some(RequirementsDecl {
                patterns,
                list: r.list,
                version: r.version.map(|v| match v {
                    Scalar::S(s) => s,
                    Scalar::I(i) => i.to_string(),
                }),
            })
        }
    };
    Ok(Declared { axes, requirements })
}

/// Check a record's axis values against the declarations: a known axis, values
/// from its list, and one value only on a scoped axis.
pub fn check_assignment(
    axes: &[AxisDef],
    assigned: &BTreeMap<String, Vec<String>>,
) -> Result<(), String> {
    for (name, values) in assigned {
        let Some(def) = axes.iter().find(|a| &a.name == name) else {
            return Err(format!("axis `{name}` is not declared by this target"));
        };
        if def.scoped && values.len() > 1 {
            return Err(format!(
                "axis `{name}` is scoped: one value per row, got {}",
                values.len()
            ));
        }
        if let Some(v) = values.iter().find(|v| !def.values.contains(v)) {
            return Err(format!("`{v}` is not a value of axis `{name}`"));
        }
    }
    Ok(())
}

/// The value a `row_prefix` axis takes from a cited requirement id: the longest
/// declared prefix the id starts with, taken as a whole leading segment
/// (the prefix is followed by `-`, `.` or `_`, or ends the id). `None` when no
/// declared prefix fits, so the axis stays unset.
pub fn derive_from_row(def: &AxisDef, row_id: &str) -> Option<String> {
    def.prefixes
        .iter()
        .filter(|(p, _)| {
            row_id
                .strip_prefix(p.as_str())
                .is_some_and(|rest| rest.is_empty() || rest.starts_with(['-', '.', '_']))
        })
        .max_by_key(|(p, _)| p.len())
        .map(|(_, v)| v.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_file_with_no_axes_declares_nothing() {
        assert_eq!(read("repo: a/b\n").unwrap(), Declared::default());
    }

    #[test]
    fn malformed_declarations_are_named() {
        for (text, needle) in [
            (
                "axes:\n  - {name: a, propose: label, values: []}\n",
                "no values",
            ),
            ("axes:\n  - {name: a, propose: nope, values: [x]}\n", "nope"),
            (
                "axes:\n  - {name: a, propose: label, values: [x]}\n  - {name: a, propose: label, values: [y]}\n",
                "twice",
            ),
            (
                "axes:\n  - {name: a, propose: marker, scoped: true, values: [x]}\n",
                "unscoped",
            ),
            (
                "axes:\n  - {name: a, propose: row_prefix, values: [x], prefixes: {P: y}}\n",
                "not a value",
            ),
            (
                "axes:\n  - {name: a, propose: label, values: [x], typo: 1}\n",
                "typo",
            ),
            ("requirements:\n  pattern: \"{q}\"\n", "unknown token"),
        ] {
            let e = read(text).unwrap_err();
            assert!(e.contains(needle), "{text}: {e}");
        }
    }

    #[test]
    fn the_longest_whole_prefix_wins() {
        let def = AxisDef {
            name: "s".into(),
            scoped: true,
            propose: Propose::RowPrefix,
            values: vec!["a".into(), "b".into()],
            prefixes: [
                ("E".to_string(), "a".to_string()),
                ("EA".to_string(), "b".to_string()),
            ]
            .into(),
        };
        assert_eq!(derive_from_row(&def, "EA-1").as_deref(), Some("b"));
        assert_eq!(derive_from_row(&def, "E-1").as_deref(), Some("a"));
        assert_eq!(derive_from_row(&def, "EAX-1"), None);
    }
}
