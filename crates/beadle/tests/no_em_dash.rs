//! Guards against the em dash (U+2014) in beadle's sources, data and the text
//! that shapes a board body (aae-orc-1dkpg, `_kos/probes/brief-kill-em-dashes.md`).
//!
//! The source scan is a tripwire for the spellings people write by accident,
//! with no test-code boundary: it reads every `.rs` file under `crates/` and
//! refuses the character in code, comments and tests alike. A test that needs
//! the character builds it with `support::em_dash()`. Bytes assembled at run
//! time and passed to a UTF-8 decoder are a named residual; where they reach a
//! body or board data, the output checks catch them.
//!
//! This file must pass its own scan, so every forbidden spelling used by the
//! planted-case test is assembled from pieces at run time.

mod support;

use std::{
    fs,
    path::{Path, PathBuf},
};

use support::em_dash;

const DASH: u32 = 0x2013 + 1;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hit {
    form: &'static str,
    line: usize,
}

/// Files that may build the character from an integer.
fn is_helper(path: &str) -> bool {
    path.ends_with("tests/support/mod.rs") || path.ends_with("src/testsupport.rs")
}

fn line_of(text: &str, byte: usize) -> usize {
    text[..byte].bytes().filter(|b| *b == b'\n').count() + 1
}

fn is_ident(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// An integer literal found in `text`: (start byte, end byte, value).
fn int_literals(text: &str) -> Vec<(usize, usize, u64)> {
    let b = text.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        if b[i].is_ascii_digit() && (i == 0 || !is_ident(b[i - 1])) {
            let start = i;
            let (radix, mut j) = if b[i] == b'0' && i + 1 < b.len() {
                match b[i + 1] {
                    b'x' | b'X' => (16, i + 2),
                    b'o' | b'O' => (8, i + 2),
                    b'b' | b'B' => (2, i + 2),
                    _ => (10, i),
                }
            } else {
                (10, i)
            };
            let mut digits = String::new();
            while j < b.len() {
                let c = b[j] as char;
                if c == '_' {
                    j += 1;
                } else if c.is_digit(radix) {
                    digits.push(c);
                    j += 1;
                } else {
                    break;
                }
            }
            // A float such as 82.12 is not an integer literal.
            let float = j + 1 < b.len() && b[j] == b'.' && b[j + 1].is_ascii_digit();
            if !digits.is_empty() && !float {
                if let Ok(v) = u64::from_str_radix(&digits, radix) {
                    out.push((start, j, v));
                }
            }
            // Skip the rest of the token (suffix, trailing identifier chars).
            while j < b.len() && is_ident(b[j]) {
                j += 1;
            }
            i = j.max(i + 1);
        } else {
            i += 1;
        }
    }
    out
}

/// Spellings of the character inside any text, Rust or data.
fn scan_text(text: &str, hits: &mut Vec<Hit>) {
    for (at, c) in text.char_indices() {
        if u32::from(c) == DASH {
            hits.push(Hit {
                form: "raw character",
                line: line_of(text, at),
            });
        }
    }
    let lower = text.to_ascii_lowercase();
    let b = lower.as_bytes();

    // Unicode escapes: the Rust braced form and the JSON four-digit form.
    let mut i = 0;
    while let Some(rel) = lower[i..].find("\\u") {
        let at = i + rel;
        let rest = &lower[at + 2..];
        let (digits, form) = if let Some(r) = rest.strip_prefix('{') {
            let end = r.find('}').unwrap_or(0);
            (r[..end].replace('_', ""), "unicode escape")
        } else {
            let d: String = rest.chars().take(4).collect();
            (d, "json escape")
        };
        if !digits.is_empty()
            && digits.chars().all(|c| c.is_ascii_hexdigit())
            && u32::from_str_radix(&digits, 16) == Ok(DASH)
        {
            hits.push(Hit {
                form,
                line: line_of(text, at),
            });
        }
        i = at + 2;
    }

    // The UTF-8 bytes as a byte escape.
    let bytes_esc = ["\\xe2", "\\x80", "\\x94"].concat();
    if let Some(at) = lower.find(&bytes_esc) {
        hits.push(Hit {
            form: "utf-8 byte escape",
            line: line_of(text, at),
        });
    }

    // HTML entities, with leading zeros.
    let amp = ["&md", "ash;"].concat();
    if let Some(at) = lower.find(&amp) {
        hits.push(Hit {
            form: "html entity",
            line: line_of(text, at),
        });
    }
    let mut i = 0;
    while let Some(rel) = lower[i..].find("&#") {
        let at = i + rel;
        let rest = &lower[at + 2..];
        let (radix, body) = match rest.strip_prefix('x') {
            Some(r) => (16, r),
            None => (10, rest),
        };
        let digits: String = body.chars().take_while(|c| c.is_digit(radix)).collect();
        if !digits.is_empty()
            && body[digits.len()..].starts_with(';')
            && u32::from_str_radix(&digits, radix) == Ok(DASH)
        {
            hits.push(Hit {
                form: "html entity",
                line: line_of(text, at),
            });
        }
        i = at + 2;
    }
    let _ = b;
}

/// Spellings that only mean something in Rust source.
fn scan_rust(path: &str, text: &str, hits: &mut Vec<Hit>) {
    scan_text(text, hits);

    // Integer literals equal to the dash's code point, in any base, and the byte triple.
    let lits = int_literals(text);
    for (s, _, v) in &lits {
        if *v == u64::from(DASH) {
            hits.push(Hit {
                form: "integer literal",
                line: line_of(text, *s),
            });
        }
    }
    for w in lits.windows(3) {
        let gap = |a: usize, b: usize| text[a..b].chars().all(|c| c == ',' || c.is_whitespace());
        if w[0].2 == 0xE2
            && w[1].2 == 0x80
            && w[2].2 == 0x94
            && gap(w[0].1, w[1].0)
            && gap(w[1].1, w[2].0)
        {
            hits.push(Hit {
                form: "utf-8 byte list",
                line: line_of(text, w[0].0),
            });
        }
    }

    // Building a char from an integer, or decoding UTF-16, outside the helper.
    if !is_helper(path) {
        let banned = [
            ["from", "_u32"].concat(),
            ["char::try", "_from"].concat(),
            ["char::", "from("].concat(),
            ["from_utf", "16"].concat(),
            ["decode_utf", "16"].concat(),
        ];
        for pat in &banned {
            let mut i = 0;
            while let Some(rel) = text[i..].find(pat.as_str()) {
                let at = i + rel;
                hits.push(Hit {
                    form: "char built from an integer",
                    line: line_of(text, at),
                });
                i = at + pat.len();
            }
        }
    }
}

/// `include_str!` and `include_bytes!` targets named in `text`, resolved
/// against the directory of `path`.
fn include_targets(path: &Path, text: &str) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for mac in ["include_str!(\"", "include_bytes!(\""] {
        let mut i = 0;
        while let Some(rel) = text[i..].find(mac) {
            let start = i + rel + mac.len();
            if let Some(end) = text[start..].find('"') {
                let rel_path = &text[start..start + end];
                out.push(path.parent().unwrap_or(Path::new(".")).join(rel_path));
            }
            i = start;
        }
    }
    out
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = fs::read_dir(dir) else { return };
    let mut entries: Vec<_> = rd.filter_map(Result::ok).collect();
    entries.sort_by_key(|e| e.path());
    for e in entries {
        let p = e.path();
        if p.is_dir() {
            if p.file_name().is_some_and(|n| n == "target") {
                continue;
            }
            rust_files(&p, out);
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
}

/// Scan every `.rs` file under `crates_dir`, following includes. Returns the
/// hits as (path, hit) and the number of files read under each crate dir.
fn scan_tree(crates_dir: &Path) -> (Vec<(String, Hit)>, Vec<(String, usize)>) {
    let mut files = Vec::new();
    rust_files(crates_dir, &mut files);
    let mut hits = Vec::new();
    let mut per_crate: Vec<(String, usize)> = Vec::new();
    for f in &files {
        let text = fs::read_to_string(f).unwrap_or_else(|e| panic!("read {}: {e}", f.display()));
        let rel = f.strip_prefix(crates_dir).unwrap_or(f);
        let name = rel
            .components()
            .next()
            .map(|c| c.as_os_str().to_string_lossy().into_owned())
            .unwrap_or_default();
        match per_crate.iter_mut().find(|(n, _)| *n == name) {
            Some((_, c)) => *c += 1,
            None => per_crate.push((name, 1)),
        }
        let shown = f.display().to_string();
        let mut h = Vec::new();
        scan_rust(&shown, &text, &mut h);
        hits.extend(h.into_iter().map(|x| (shown.clone(), x)));
        for t in include_targets(f, &text) {
            let data = fs::read_to_string(&t)
                .unwrap_or_else(|e| panic!("read include {}: {e}", t.display()));
            let mut h = Vec::new();
            scan_text(&data, &mut h);
            hits.extend(h.into_iter().map(|x| (t.display().to_string(), x)));
        }
    }
    (hits, per_crate)
}

#[test]
fn sources_have_no_em_dash_spelling() {
    let crates = repo_root().join("crates");
    let (hits, per_crate) = scan_tree(&crates);
    // An empty walk must fail, not pass.
    for krate in ["beadle", "beadle-store", "director-envelope"] {
        assert!(
            per_crate.iter().any(|(n, c)| n == krate && *c > 0),
            "the scan read no file under crates/{krate}: {per_crate:?}"
        );
    }
    assert!(
        hits.is_empty(),
        "{} em dash spelling(s) in crates/; first 20:\n{}",
        hits.len(),
        hits.iter()
            .take(20)
            .map(|(p, h)| format!("  {p}:{} ({})", h.line, h.form))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn refused(text: &str) -> Vec<Hit> {
    let mut h = Vec::new();
    scan_rust("crates/beadle/src/x.rs", text, &mut h);
    h
}

#[test]
fn scan_catches_planted_spelling() {
    let hex_lo = ["20", "14"].concat();
    let dec = (8211 + 1).to_string();
    let bin = format!("{:b}", 0x2013 + 1);
    let oct = format!("{:o}", 0x2013 + 1);
    let cases: Vec<(&str, String)> = vec![
        ("raw character", format!("a {} b", em_dash())),
        ("unicode escape", format!("\\u{{{hex_lo}}}")),
        ("unicode escape", format!("\\u{{0{hex_lo}}}")),
        ("unicode escape", format!("\\u{{20_{}}}", &hex_lo[2..])),
        ("unicode escape", format!("\\U{{{hex_lo}}}")),
        ("integer literal", format!("let x = 0x{hex_lo};")),
        ("integer literal", format!("let x = 0x0{hex_lo};")),
        ("integer literal", format!("let x = 0x20_{};", &hex_lo[2..])),
        ("integer literal", format!("let x = {dec};")),
        (
            "integer literal",
            format!("let x = {}_{};", &dec[..1], &dec[1..]),
        ),
        ("integer literal", format!("let x = 0o{oct};")),
        ("integer literal", format!("let x = 0b{bin};")),
        ("integer literal", format!("let x = {dec}u32;")),
        ("integer literal", format!("let x = 0x{hex_lo}_u32;")),
        ("utf-8 byte escape", ["\\xe2", "\\x80", "\\x94"].concat()),
        ("utf-8 byte escape", ["\\xE2", "\\x80", "\\x94"].concat()),
        ("utf-8 byte list", ["[0xE2, 0x80, ", "0x94]"].concat()),
        ("utf-8 byte list", ["[226, 128, ", "148]"].concat()),
        ("html entity", ["&md", "ash;"].concat()),
        ("html entity", ["&MD", "ASH;"].concat()),
        ("html entity", format!("&#{dec};")),
        ("html entity", format!("&#0{dec};")),
        ("html entity", format!("&#x{hex_lo};")),
        ("html entity", format!("&#X00{hex_lo};")),
        ("json escape", format!("\\u{hex_lo}")),
        (
            "char built from an integer",
            ["char::", "from_u", "32(x)"].concat(),
        ),
        (
            "char built from an integer",
            ["char::try", "_from(x)"].concat(),
        ),
        ("char built from an integer", ["char::", "from(x)"].concat()),
        (
            "char built from an integer",
            ["String::from_utf", "16(&v)"].concat(),
        ),
        (
            "char built from an integer",
            ["decode_utf", "16(it)"].concat(),
        ),
    ];
    for (form, text) in &cases {
        let hits = refused(&format!("fn f() {{}}\n{text}\n"));
        assert!(
            hits.iter().any(|h| h.form == *form && h.line == 2),
            "not refused as {form:?} on line 2: {text:?} -> {hits:?}"
        );
    }

    // Controls: the en dash, other entities and values, and the helper's own
    // expression pass.
    let controls = [
        ["&nd", "ash;"].concat(),
        "&#8211;".to_string(),
        "&#x2013;".to_string(),
        "let x = 0x2013;".to_string(),
        "let x = 8211;".to_string(),
        "let x = 0x2015;".to_string(),
        "let x = 82.12;".to_string(),
        "[0xe2, 0x80, 0x93]".to_string(),
        "[0xe2, 0x80, 0x95]".to_string(),
        "\\u{2013}".to_string(),
        "\\u2013".to_string(),
    ];
    for c in &controls {
        let hits = refused(c);
        assert!(
            hits.is_empty(),
            "control wrongly refused: {c:?} -> {hits:?}"
        );
    }
    let mut h = Vec::new();
    scan_rust(
        "crates/beadle/tests/support/mod.rs",
        &["char::", "from_u", "32(0x2013 + 1).unwrap()"].concat(),
        &mut h,
    );
    assert!(
        h.is_empty(),
        "the helper's own expression was refused: {h:?}"
    );
    let mut h = Vec::new();
    scan_rust(
        "crates/beadle/src/other.rs",
        &["char::", "from_u", "32(0x2013 + 1).unwrap()"].concat(),
        &mut h,
    );
    assert!(
        h.iter().any(|x| x.form == "char built from an integer"),
        "the same expression outside the helper must be refused: {h:?}"
    );
}

#[test]
fn scan_follows_includes() {
    let dir = tempfile::tempdir().unwrap();
    let data = dir.path().join("data.json");
    fs::write(&data, format!("{{\"k\": \"a {} b\"}}", em_dash())).unwrap();
    let rs = dir.path().join("x.rs");
    let src = "const D: &str = include_str!(\"data.json\");\n";
    let targets = include_targets(&rs, src);
    assert_eq!(targets, vec![dir.path().join("data.json")]);
    let mut h = Vec::new();
    scan_text(&fs::read_to_string(&targets[0]).unwrap(), &mut h);
    assert!(h.iter().any(|x| x.form == "raw character"), "{h:?}");
}

// ---- output-side checks over the data and text that shape a body ----------

fn dash_lines(text: &str) -> Vec<usize> {
    let d = em_dash();
    text.lines()
        .enumerate()
        .filter(|(_, l)| l.contains(d))
        .map(|(i, _)| i + 1)
        .collect()
}

fn assert_clean(rel: &str) {
    let p = repo_root().join(rel);
    let text = fs::read_to_string(&p).unwrap_or_else(|e| panic!("read {}: {e}", p.display()));
    let lines = dash_lines(&text);
    assert!(
        lines.is_empty(),
        "{rel} holds {} line(s) with an em dash; first 10: {:?}",
        lines.len(),
        &lines[..lines.len().min(10)]
    );
}

#[test]
fn body_data_has_no_em_dash() {
    assert_clean("skills/beadle-triage/vocabulary.json");
    let mut n = 0;
    for e in fs::read_dir(repo_root().join("targets")).unwrap().flatten() {
        let name = e.file_name().to_string_lossy().into_owned();
        if name.ends_with(".intent.yaml") {
            assert_clean(&format!("targets/{name}"));
            n += 1;
        }
    }
    assert!(n >= 3, "expected the intent files, read {n}");
}

#[test]
fn body_shaping_text_has_no_em_dash() {
    // Exempt by name: docs/fixtures/ (history), targets/*.verify.json and
    // tools/dashboard-gate.py (they quote historical headings, which must keep
    // the dash to match).
    for rel in [
        "skills/beadle-triage/SKILL.md",
        ".claude/commands/dashboard-refresh.md",
        "docs/dashboard-schema.md",
        "docs/example-dashboard.md",
        "prompts/create-dashboard.md",
    ] {
        assert_clean(rel);
    }
}
