//! Board sections for a target that declares classification axes: effort by
//! each axis, effort by evidence, and, where the target names a requirements
//! list, scope coverage (rows cited against rows listed).
//!
//! "Effort" here is the count of classified rows, not hours: the store holds
//! no time data. A target that declares no axes gets no section, so its board
//! is the board it had before axes existed.

use std::{
    collections::{BTreeSet, HashMap},
    path::Path,
};

use anyhow::{Context, Result, anyhow};
use beadle_store::ClassificationRecord;

use crate::{
    axes::Propose,
    evidence::Evidence,
    intent::Intent,
    requirements::{self, PatternSet},
};

fn esc(s: &str) -> String {
    s.replace('|', "\\|").replace('\n', " ")
}

fn list_or_none(items: &[String]) -> String {
    if items.is_empty() {
        "none".to_string()
    } else {
        esc(&items.join(", "))
    }
}

pub fn render(
    root: &Path,
    intent: &Intent,
    latest: &HashMap<u32, ClassificationRecord>,
) -> Result<String> {
    if intent.axes.is_empty() {
        return Ok(String::new());
    }
    let mut rows: Vec<&ClassificationRecord> = latest.values().collect();
    rows.sort_by_key(|c| c.number);
    let mut out = String::new();

    for def in &intent.axes {
        let kind = if def.scoped {
            "one value per row"
        } else {
            "any number of values"
        };
        out.push_str(&format!(
            "## Effort by {} ({kind})\n\n| Value | Rows |\n|---|---|\n",
            esc(&def.name)
        ));
        for v in &def.values {
            let n = rows
                .iter()
                .filter(|c| c.axes.get(&def.name).is_some_and(|vs| vs.contains(v)))
                .count();
            out.push_str(&format!("| {} | {n} |\n", esc(v)));
        }
        if def.propose != Propose::Marker {
            let none = rows
                .iter()
                .filter(|c| c.axes.get(&def.name).is_none_or(|vs| vs.is_empty()))
                .count();
            out.push_str(&format!("| unassigned | {none} |\n"));
        }
        out.push('\n');
    }

    out.push_str("## Effort by evidence\n\n| Evidence | Rows |\n|---|---|\n");
    for e in [
        Evidence::None,
        Evidence::Test,
        Evidence::Live,
        Evidence::SocConfirmed,
    ] {
        let n = rows
            .iter()
            .filter(|c| c.evidence.as_deref() == Some(e.as_str()))
            .count();
        out.push_str(&format!("| {} | {n} |\n", e.as_str()));
    }
    let none = rows.iter().filter(|c| c.evidence.is_none()).count();
    out.push_str(&format!("| unassigned | {none} |\n\n"));
    out.push_str(
        "_Beadle proposes `none` or `test` only. `live` and `soc-confirmed` are set by a person._\n\n",
    );

    if let Some(decl) = &intent.requirements {
        let cited: BTreeSet<String> = rows
            .iter()
            .flat_map(|c| c.requirements.iter().cloned())
            .collect();
        out.push_str("## Scope coverage\n\n");
        match &decl.list {
            Some(list) => {
                let pattern = PatternSet::new(&decl.patterns).map_err(|m| anyhow!(m))?;
                let path = root.join(list);
                let listed = requirements::load_list(&path, &pattern)
                    .with_context(|| "scope coverage needs the declared requirements list")?;
                let cov = requirements::coverage(&listed, &cited);
                let version = decl
                    .version
                    .as_deref()
                    .map_or(String::new(), |v| format!(" version {}", esc(v)));
                out.push_str(&format!(
                    "_Rows cited against the requirements list{version}._\n\n"
                ));
                out.push_str(&format!(
                    "| Measure | Value |\n|---|---|\n| listed | {} |\n| cited and listed | {} |\n\
                     | listed, not cited | {} |\n| cited, not listed | {} |\n\n",
                    cov.listed,
                    cov.cited_and_listed,
                    list_or_none(&cov.listed_not_cited),
                    list_or_none(&cov.cited_not_listed),
                ));
            }
            None => {
                let ids: Vec<String> = cited.into_iter().collect();
                out.push_str(&format!(
                    "_No requirements list is declared, so coverage cannot be computed._\n\n\
                     | Measure | Value |\n|---|---|\n| cited | {} |\n\n",
                    list_or_none(&ids)
                ));
            }
        }
    }
    Ok(out)
}
