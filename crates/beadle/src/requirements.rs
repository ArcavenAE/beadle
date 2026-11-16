//! The `Requirements:` line of an issue or PR body, and the id pattern a target
//! declares for it.
//!
//! The pattern is a small declared form, not a regular expression: the crate
//! carries no regex dependency and a target needs only a few shapes. Literal
//! text is letters, digits, `-`, `.` and `_`. Four tokens stand for runs:
//!
//! * `{n}`  one or more ASCII digits
//! * `{a}`  one or more ASCII capital letters
//! * `{l}`  one or more ASCII lowercase letters
//! * `{l?}` zero or one ASCII lowercase letter (a split row's trailing letter)
//!
//! An id matches only when the whole string fits, case as written. A target may
//! declare one form or a list of forms; an id fits when any form matches.

use std::{collections::BTreeSet, fs, path::Path};

use anyhow::{Context, Result, bail};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tok {
    Lit(char),
    Digits,
    Upper,
    Lower,
    OptLower,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pattern {
    toks: Vec<Tok>,
}

impl Pattern {
    pub fn new(text: &str) -> std::result::Result<Pattern, String> {
        if text.is_empty() {
            return Err("an id pattern may not be empty".to_string());
        }
        let mut toks = Vec::new();
        let mut chars = text.chars();
        while let Some(c) = chars.next() {
            match c {
                '{' => {
                    let mut name = String::new();
                    loop {
                        match chars.next() {
                            Some('}') => break,
                            Some(x) => name.push(x),
                            None => return Err(format!("unclosed `{{` in id pattern `{text}`")),
                        }
                    }
                    toks.push(match name.as_str() {
                        "n" => Tok::Digits,
                        "a" => Tok::Upper,
                        "l" => Tok::Lower,
                        "l?" => Tok::OptLower,
                        other => {
                            return Err(format!(
                                "unknown token `{{{other}}}` in id pattern `{text}` (use {{n}}, {{a}}, {{l}} or {{l?}})"
                            ));
                        }
                    });
                }
                c if c.is_ascii_alphanumeric() || matches!(c, '-' | '.' | '_') => {
                    toks.push(Tok::Lit(c));
                }
                other => {
                    return Err(format!(
                        "`{other}` is not allowed as literal text in id pattern `{text}`"
                    ));
                }
            }
        }
        Ok(Pattern { toks })
    }

    pub fn matches(&self, id: &str) -> bool {
        fn go(toks: &[Tok], s: &[u8]) -> bool {
            let Some((first, rest)) = toks.split_first() else {
                return s.is_empty();
            };
            let run = |ok: fn(u8) -> bool| -> bool {
                let n = s.iter().take_while(|b| ok(**b)).count();
                (1..=n).rev().any(|k| go(rest, &s[k..]))
            };
            match first {
                Tok::Lit(c) => s.first().is_some_and(|b| *b as char == *c) && go(rest, &s[1..]),
                Tok::Digits => run(|b| b.is_ascii_digit()),
                Tok::Upper => run(|b| b.is_ascii_uppercase()),
                Tok::Lower => run(|b| b.is_ascii_lowercase()),
                Tok::OptLower => {
                    go(rest, s)
                        || s.first().is_some_and(u8::is_ascii_lowercase) && go(rest, &s[1..])
                }
            }
        }
        id.is_ascii() && go(&self.toks, id.as_bytes())
    }
}

/// `beadle requirements <target> [--file F]`: print the ids on the body's
/// `Requirements:` line as a JSON array, for the classifier to carry into its
/// payload. Entries that do not fit the target's pattern go to stderr.
pub fn run(root: &Path, target: &str, file: Option<&Path>) -> Result<()> {
    use std::io::Read;
    let intent = crate::intent::load(root, target)?;
    let decl = intent.requirements.as_ref().with_context(|| {
        format!("target `{target}` declares no requirements pattern; there is nothing to match")
    })?;
    let pattern = PatternSet::new(&decl.patterns).map_err(|m| anyhow::anyhow!(m))?;
    let body = match file {
        Some(p) => fs::read_to_string(p).with_context(|| format!("read {}", p.display()))?,
        None => {
            let mut b = String::new();
            std::io::stdin()
                .read_to_string(&mut b)
                .context("read body from stdin")?;
            b
        }
    };
    let parsed = parse_body(&body, &pattern);
    for r in &parsed.rejected {
        eprintln!(
            "beadle requirements: `{r}` does not fit the id pattern `{}`",
            pattern.describe()
        );
    }
    println!("{}", serde_json::to_string(&parsed.ids)?);
    Ok(())
}

/// The id forms a target declares: an id fits when it fits any one of them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternSet {
    forms: Vec<(String, Pattern)>,
}

impl PatternSet {
    pub fn new(forms: &[String]) -> std::result::Result<PatternSet, String> {
        if forms.is_empty() {
            return Err("requirements.pattern lists no forms".to_string());
        }
        let mut out = Vec::new();
        for f in forms {
            out.push((f.clone(), Pattern::new(f)?));
        }
        Ok(PatternSet { forms: out })
    }

    pub fn matches(&self, id: &str) -> bool {
        self.forms.iter().any(|(_, p)| p.matches(id))
    }

    /// The forms as written, for messages.
    pub fn describe(&self) -> String {
        self.forms
            .iter()
            .map(|(f, _)| f.as_str())
            .collect::<Vec<_>>()
            .join(" | ")
    }
}

/// What a body's `Requirements:` lines held.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Parsed {
    /// Ids that fit the pattern, in order of first appearance, without repeats.
    pub ids: Vec<String>,
    /// Entries on a `Requirements:` line that did not fit the pattern.
    pub rejected: Vec<String>,
}

/// Read the ids from every line of `body` that starts with `Requirements:`.
/// Ids elsewhere in the text are not read.
pub fn parse_body(body: &str, pattern: &PatternSet) -> Parsed {
    let mut out = Parsed::default();
    for line in body.lines() {
        let Some(rest) = line.trim_start().strip_prefix("Requirements:") else {
            continue;
        };
        for entry in rest.split(',').map(str::trim).filter(|e| !e.is_empty()) {
            if pattern.matches(entry) {
                if !out.ids.iter().any(|i| i == entry) {
                    out.ids.push(entry.to_string());
                }
            } else {
                out.rejected.push(entry.to_string());
            }
        }
    }
    out
}

/// The versioned list of requirement ids a target names: one id per line, `#`
/// starts a comment. An id repeated in the list is an error, since ids are
/// never reused or renumbered.
pub fn load_list(path: &Path, pattern: &PatternSet) -> Result<Vec<String>> {
    let text = fs::read_to_string(path)
        .with_context(|| format!("read requirements list {}", path.display()))?;
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for (n, line) in text.lines().enumerate() {
        let id = line.split('#').next().unwrap_or("").trim();
        if id.is_empty() {
            continue;
        }
        if !pattern.matches(id) {
            bail!(
                "{}:{}: `{id}` does not fit the declared id pattern",
                path.display(),
                n + 1
            );
        }
        if !seen.insert(id.to_string()) {
            bail!("{}:{}: `{id}` is listed twice", path.display(), n + 1);
        }
        out.push(id.to_string());
    }
    Ok(out)
}

/// Rows cited against rows listed.
#[derive(Debug, PartialEq, Eq)]
pub struct Coverage {
    pub listed: usize,
    pub cited_and_listed: usize,
    pub listed_not_cited: Vec<String>,
    pub cited_not_listed: Vec<String>,
}

pub fn coverage(listed: &[String], cited: &BTreeSet<String>) -> Coverage {
    let list: BTreeSet<&String> = listed.iter().collect();
    Coverage {
        listed: listed.len(),
        cited_and_listed: listed.iter().filter(|i| cited.contains(*i)).count(),
        listed_not_cited: listed
            .iter()
            .filter(|i| !cited.contains(*i))
            .cloned()
            .collect(),
        cited_not_listed: cited
            .iter()
            .filter(|i| !list.contains(i))
            .cloned()
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_pattern_with_an_unknown_token_or_a_stray_character_is_refused() {
        assert!(Pattern::new("{x}-{n}").is_err());
        assert!(Pattern::new("A B").is_err());
        assert!(Pattern::new("{n").is_err());
        assert!(Pattern::new("").is_err());
    }

    #[test]
    fn the_optional_letter_is_zero_or_one() {
        let p = Pattern::new("{a}-{n}{l?}").unwrap();
        assert!(p.matches("AB-1") && p.matches("AB-1c"));
        assert!(!p.matches("AB-1cd"));
    }

    #[test]
    fn coverage_counts_both_directions() {
        let listed: Vec<String> = ["A-1", "A-2"].iter().map(|s| s.to_string()).collect();
        let cited: BTreeSet<String> = ["A-1", "B-9"].iter().map(|s| s.to_string()).collect();
        let c = coverage(&listed, &cited);
        assert_eq!(c.cited_and_listed, 1);
        assert_eq!(c.listed_not_cited, vec!["A-2".to_string()]);
        assert_eq!(c.cited_not_listed, vec!["B-9".to_string()]);
    }
}
