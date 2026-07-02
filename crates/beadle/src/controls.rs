//! Board-level maintenance-request controls (F5 Tier 2).
//!
//! Renderer emits a fixed vocabulary of unchecked checkboxes in the derived
//! zone. Push scans the LIVE body for checked boxes, records one
//! `control-request` note per verb, then writes the fresh render — which is
//! always unchecked. That reset IS the de-bounce: the checkbox lives in the
//! derived zone, so every regen clears it.
//!
//! Dispatch of the requested routines (`reprioritize`, `full-refresh`,
//! `revalidate`, `rescore-intent`) is deferred — the notes are the request
//! queue Phase-1 gh-aw will drain. See
//! `_kos/nodes/frontier/question-maintenance-request-controls.yaml`.

/// Canonical list of Tier-2 board-level verbs. Order is the render order.
///
/// Each entry is `(verb, one-line human description)`. The verb string is the
/// stable identifier: it appears both in the visible label and in the
/// `<!-- verb=... -->` marker the extractor scans for.
pub const BOARD_VERBS: &[(&str, &str)] = &[
    (
        "reprioritize",
        "re-rank the action plan without a full re-enumerate",
    ),
    (
        "full-refresh",
        "re-enumerate every open issue from scratch",
    ),
    (
        "revalidate",
        "re-run validate across the corpus (already-fixed / fix-not-fixed / hallucinated-citation)",
    ),
    (
        "rescore-intent",
        "re-run score-intent corpus-wide after an intent-manifest change",
    ),
];

/// Render the derived-zone board-controls block. Every regeneration emits
/// unchecked boxes — that's the reset that makes the control eventually-
/// consistent (finding-018 / `question-maintenance-request-controls`).
pub fn render_board_controls_block() -> String {
    let mut out = String::new();
    out.push_str("## Board controls\n\n");
    out.push_str(
        "_Tier-2 maintenance requests. Check a box, wait for the next `beadle push` — beadle records the request as a `control-request` note and resets the box. Dispatch of the routines lands with the Phase-1 gh-aw cron (deferred, per `question-maintenance-request-controls`)._\n\n",
    );
    for (verb, description) in BOARD_VERBS {
        out.push_str(&format!(
            "- [ ] `{}` — {} <!-- verb={};id=board -->\n",
            verb, description, verb
        ));
    }
    out.push('\n');
    out
}

/// Scan a live dashboard body for checked board-verb boxes. Returns the
/// verbs in canonical `BOARD_VERBS` order, deduped — a maintainer who
/// unfolds two identical-looking rows and checks both still only queues
/// the routine once (idempotency guard, F5 debounce concern 4).
///
/// Match shape: `- [x]` (or `- [X]`) followed by any characters on the same
/// line, ending with `<!-- verb=<verb>;...` where `<verb>` is one of
/// `BOARD_VERBS`. The `id=board` field is not required by the scanner (the
/// legend renders it; extraction only anchors on `verb=`), but the renderer
/// always emits it.
pub fn extract_checked_verbs(body: &str) -> Vec<String> {
    let mut found: Vec<String> = Vec::new();
    for line in body.lines() {
        let trimmed = line.trim_start();
        if !(trimmed.starts_with("- [x]") || trimmed.starts_with("- [X]")) {
            continue;
        }
        for (verb, _) in BOARD_VERBS {
            let marker = format!("verb={}", verb);
            if line.contains(&marker) && !found.iter().any(|v| v == verb) {
                found.push((*verb).to_string());
            }
        }
    }
    // Preserve canonical order regardless of body order.
    let mut ordered: Vec<String> = Vec::new();
    for (verb, _) in BOARD_VERBS {
        if found.iter().any(|v| v == verb) {
            ordered.push((*verb).to_string());
        }
    }
    ordered
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_emits_every_verb_unchecked() {
        let block = render_board_controls_block();
        assert!(block.contains("## Board controls"));
        for (verb, _) in BOARD_VERBS {
            let box_line = format!("- [ ] `{}`", verb);
            assert!(block.contains(&box_line), "verb {} missing", verb);
            let marker = format!("verb={};id=board", verb);
            assert!(block.contains(&marker), "marker {} missing", marker);
        }
        // Never emit a checked box from the renderer.
        assert!(!block.contains("- [x]"));
        assert!(!block.contains("- [X]"));
    }

    #[test]
    fn extract_none_from_unchecked_body() {
        let body = render_board_controls_block();
        let verbs = extract_checked_verbs(&body);
        assert!(verbs.is_empty(), "unchecked render must yield no verbs");
    }

    #[test]
    fn extract_one_checked_verb() {
        let body = "- [x] `full-refresh` — foo <!-- verb=full-refresh;id=board -->\n";
        let verbs = extract_checked_verbs(body);
        assert_eq!(verbs, vec!["full-refresh".to_string()]);
    }

    #[test]
    fn extract_capital_x_also_matches() {
        let body = "- [X] `revalidate` — foo <!-- verb=revalidate;id=board -->\n";
        let verbs = extract_checked_verbs(body);
        assert_eq!(verbs, vec!["revalidate".to_string()]);
    }

    #[test]
    fn extract_dedupes_and_orders_canonically() {
        // Body has verbs in reverse order; extractor returns canonical order.
        let body = "\
- [x] `rescore-intent` <!-- verb=rescore-intent;id=board -->
- [x] `full-refresh` <!-- verb=full-refresh;id=board -->
- [x] `full-refresh` <!-- verb=full-refresh;id=board -->
- [x] `reprioritize` <!-- verb=reprioritize;id=board -->
";
        let verbs = extract_checked_verbs(body);
        assert_eq!(
            verbs,
            vec![
                "reprioritize".to_string(),
                "full-refresh".to_string(),
                "rescore-intent".to_string(),
            ]
        );
    }

    #[test]
    fn extract_ignores_unknown_verbs() {
        let body = "- [x] `nuke-repo` <!-- verb=nuke-repo;id=board -->\n";
        let verbs = extract_checked_verbs(body);
        assert!(verbs.is_empty(), "unknown verbs must be ignored");
    }

    #[test]
    fn extract_ignores_tier1_per_issue_verbs() {
        // A per-issue verb (id=#42) is Tier 1, not this scanner's business —
        // the marker names one of the four canonical Tier-2 verbs so it
        // WOULD match verb=... today. That's OK: Tier 1 uses a distinct
        // verb vocabulary (fast-track / investigate / accept-deferral).
        let body = "- [x] `fast-track` <!-- verb=fast-track;id=#42 -->\n";
        let verbs = extract_checked_verbs(body);
        assert!(verbs.is_empty(), "Tier-1 verbs not in BOARD_VERBS");
    }
}
