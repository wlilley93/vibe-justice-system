//! [2026] VJS-CC-VJS 15 C6: every surviving `lawpack/v2` literal is DECLARED.
//!
//! The ruling's second limb: a site whose referent is the repository's own governed
//! records is correct-because-local and must NOT be re-pointed at the resolver - but from
//! the outside it is indistinguishable from a site that is merely unfixed. Nine literals
//! sat in the tree on 2026-08-01; three of them were the superseded canon-read law and the
//! rest were local, and nothing in the code said which was which. Reading them one by one
//! is exactly the audit that has to be redone every time somebody adds a tenth.
//!
//! So each surviving literal carries a marker of FIXED FORM naming its referent and citing
//! authority, and this test refuses any that does not. The marker is a DECLARATION, never
//! an approval: a `status=reserved` site is marked as reserved citing the ruling that
//! reserved it, so marking cannot quietly decide what the court left open.
//!
//! Modelled on `structural_ceiling.rs`, the live precedent for a machine-checked
//! structural rule that runs under `cargo test --workspace` (which the required CI re-runs,
//! K-27), rather than being trusted to review.

use std::path::{Path, PathBuf};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// The literal under discipline: a string literal that opens with the canon tree's path.
const LITERAL: &str = "\"lawpack/v2";

/// The marker's fixed form. All three parts are required, so a marker cannot be a shrug:
/// `LAWPACK-LITERAL: referent=<one of REFERENTS>; status=<local|reserved>; authority=[YYYY] VJS-...`
const MARKER: &str = "LAWPACK-LITERAL:";

/// The closed set of referents. Closed on purpose: a free-text referent would let the next
/// literal be waved through with a word, and the whole point is that the marker says WHICH
/// of the two limbs of the ratio the site falls under.
const REFERENTS: &[&str] = &[
    // the resolver's own vendored candidate - the one canon-read literal left standing
    "resolver",
    // this repository's own governed records / paths on its own disk
    "local-records",
    // this repository's own canon as a place to WRITE
    "write-target",
    // the read-only staged mirror family, reserved by CC-VJS 13/14 obiter (ii)
    "staged-mirror",
];

const STATUSES: &[&str] = &["local", "reserved"];

/// The resolver is the ONE site allowed to name the lawpack in order to READ the canon, so
/// literals inside it are exempt by the ratio itself and not by a marker. Matched on the
/// enclosing `fn` name, so `resolve_lawpack` and `resolve_lawpack_dir` are both covered.
const RESOLVER_FN_PREFIX: &str = "resolve_lawpack";

/// Is this a TEST file? Test fixtures build lawpacks on disk by construction and are not
/// the surface the ratio governs (they are how the ratio is proved).
fn is_test_file(p: &Path) -> bool {
    let s = p.to_string_lossy().replace('\\', "/");
    s.contains("/tests/") || s.ends_with("/tests.rs")
}

/// The line ranges of `#[cfg(test)] mod ... { ... }` blocks.
///
/// Detected on rustfmt's guarantee that an item's closing brace sits at the item's own
/// indentation - the workspace is rustfmt-clean and `cargo fmt --check` is a gate - rather
/// than by counting braces, which would need a Rust lexer to get string literals right.
/// Note this must NOT swallow a bare `#[cfg(test)] mod tests;` declaration (vjs-redact has
/// one on line 6): that form has no body, and treating it as a block would blank the file.
fn test_module_lines(src: &[&str]) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    for (i, line) in src.iter().enumerate() {
        if line.trim() != "#[cfg(test)]" {
            continue;
        }
        // The item this attribute decorates, skipping any further attributes.
        let mut j = i + 1;
        while j < src.len() && src[j].trim().starts_with('#') {
            j += 1;
        }
        let Some(item) = src.get(j) else { continue };
        if !item.trim_end().ends_with('{') {
            continue; // a `mod tests;` declaration, or a `#[cfg(test)] fn` one-liner
        }
        let indent = item.len() - item.trim_start().len();
        let closer = format!("{}}}", " ".repeat(indent));
        let end = src[j + 1..]
            .iter()
            .position(|l| l.trim_end() == closer)
            .map(|k| j + 1 + k)
            .unwrap_or(src.len() - 1);
        out.push((i, end));
    }
    out
}

/// The name of the `fn` a line sits in, by the nearest preceding `fn` declaration at or
/// above column zero of the enclosing item. Coarse, and deliberately so: it exists only to
/// exempt the resolver, and every other site has to carry a marker anyway.
fn enclosing_fn(src: &[&str], line: usize) -> String {
    for l in src[..=line].iter().rev() {
        let t = l.trim_start();
        for prefix in [
            "pub fn ",
            "pub(crate) fn ",
            "fn ",
            "pub async fn ",
            "async fn ",
        ] {
            if let Some(rest) = t.strip_prefix(prefix) {
                return rest
                    .split(['(', '<', ' '])
                    .next()
                    .unwrap_or_default()
                    .to_string();
            }
        }
    }
    String::new()
}

/// The whole comment block a marker opens, joined into one line.
///
/// Markers wrap: the fixed form plus the prose explaining WHY runs past the line width, and
/// rustfmt has no opinion about where a `//` comment breaks. Reading only the first line
/// read `authority=[2026]` and rejected every marker whose citation had wrapped, which is a
/// gate that fails on correct work - so the block is joined before it is parsed.
fn marker_block(src: &[&str], start: usize) -> String {
    let mut text = String::new();
    for line in &src[start..] {
        let t = line.trim_start();
        let Some(body) = t
            .strip_prefix("///")
            .or_else(|| t.strip_prefix("//!"))
            .or_else(|| t.strip_prefix("//"))
        else {
            break;
        };
        text.push_str(body.trim_end());
        text.push(' ');
    }
    text
}

/// A well-formed marker: all three fields present, referent and status from the closed
/// sets, authority carrying a `[YYYY] VJS-` neutral citation.
fn is_well_formed_marker(line: &str) -> bool {
    let Some(rest) = line.split_once(MARKER).map(|(_, r)| r) else {
        return false;
    };
    let field = |key: &str| -> Option<String> {
        rest.split_once(key)
            .map(|(_, v)| v.split(';').next().unwrap_or_default().trim().to_string())
    };
    let Some(referent) = field("referent=") else {
        return false;
    };
    let Some(status) = field("status=") else {
        return false;
    };
    let Some(authority) = field("authority=") else {
        return false;
    };
    REFERENTS.contains(&referent.as_str())
        && STATUSES.contains(&status.as_str())
        && cites_a_neutral_citation(&authority)
}

/// `[YYYY] VJS-` somewhere in the authority field. The marker may cite more than one
/// ruling (the staged family is reserved by three), so this asks for at least one.
fn cites_a_neutral_citation(text: &str) -> bool {
    let b = text.as_bytes();
    for i in 0..b.len().saturating_sub(7) {
        if b[i] == b'['
            && b[i + 1..i + 5].iter().all(|c| c.is_ascii_digit())
            && b[i + 5] == b']'
            && text[i + 6..].trim_start().starts_with("VJS-")
        {
            return true;
        }
    }
    false
}

/// A marker heads the STATEMENT the literal sits in, so one declaration covers a wrapped
/// expression (`let rel = format!(` and its arguments) and an adjacent run of fields
/// (`default_paths`'s two) without a marker per line.
///
/// The walk upward stops at the first line that ENDS a statement - a `;`, a closing brace,
/// or a blank line - so a marker cannot drift into covering a literal added later in the
/// same function. Lines that leave a statement open (`(`, `[`, `{`, `,`), comments, and
/// other literal lines are walked through.
fn is_marked(src: &[&str], line: usize) -> bool {
    let mut i = line;
    while i > 0 {
        i -= 1;
        let l = src[i];
        if l.contains(MARKER) {
            return is_well_formed_marker(&marker_block(src, i));
        }
        let t = l.trim();
        if t.starts_with("//") || l.contains(LITERAL) {
            continue;
        }
        if t.ends_with('(') || t.ends_with('[') || t.ends_with('{') || t.ends_with(',') {
            continue; // the statement is still open above us
        }
        return false;
    }
    false
}

fn rust_sources(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            // target/ holds generated artifacts, never source.
            if p.file_name().and_then(|s| s.to_str()) != Some("target") {
                rust_sources(&p, out);
            }
        } else if p.extension().and_then(|s| s.to_str()) == Some("rs") {
            out.push(p);
        }
    }
}

#[test]
fn every_surviving_lawpack_literal_carries_a_marker() {
    let mut files = Vec::new();
    rust_sources(&workspace_root().join("crates"), &mut files);
    files.sort();

    let mut unmarked: Vec<String> = Vec::new();
    for path in files {
        if is_test_file(&path) {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let src: Vec<&str> = text.lines().collect();
        let test_blocks = test_module_lines(&src);
        for (i, line) in src.iter().enumerate() {
            if !line.contains(LITERAL) || line.contains(MARKER) {
                continue;
            }
            if test_blocks.iter().any(|(a, b)| i >= *a && i <= *b) {
                continue;
            }
            if enclosing_fn(&src, i).starts_with(RESOLVER_FN_PREFIX) {
                continue;
            }
            if is_marked(&src, i) {
                continue;
            }
            let rel = path
                .strip_prefix(workspace_root())
                .map(|r| r.display().to_string())
                .unwrap_or_else(|_| path.display().to_string());
            unmarked.push(format!("{rel}:{}", i + 1));
        }
    }

    assert!(
        unmarked.is_empty(),
        "unmarked lawpack literal(s): {unmarked:?}.\n\
         Every site that names the lawpack outside the resolver must DECLARE its referent, \
         so a site that is correct-because-local is distinguishable from one that is merely \
         unfixed ([2026] VJS-CC-VJS 15). Either take the directory from \
         `vjs_engine::resolve_lawpack`, or head the literal with:\n\
         // LAWPACK-LITERAL: referent=<{}>; status=<{}>; authority=[YYYY] VJS-<court> <n>.\n\
         The marker is a declaration, not an approval: mark a reserved site \
         `status=reserved` and cite the ruling that reserved it.",
        REFERENTS.join("|"),
        STATUSES.join("|"),
    );
}

/// The gate's own negative control: a marker that is missing a field, names a referent
/// outside the closed set, or cites nothing must NOT pass. Without this the check could be
/// satisfied by any comment containing the token, which is a check that cannot fail.
#[test]
fn a_malformed_marker_is_not_a_marker() {
    assert!(is_well_formed_marker(
        "// LAWPACK-LITERAL: referent=local-records; status=local; authority=[2026] VJS-CC-VJS 15."
    ));
    // no referent
    assert!(!is_well_formed_marker(
        "// LAWPACK-LITERAL: status=local; authority=[2026] VJS-CC-VJS 15."
    ));
    // referent outside the closed set
    assert!(!is_well_formed_marker(
        "// LAWPACK-LITERAL: referent=whatever; status=local; authority=[2026] VJS-CC-VJS 15."
    ));
    // no status
    assert!(!is_well_formed_marker(
        "// LAWPACK-LITERAL: referent=local-records; authority=[2026] VJS-CC-VJS 15."
    ));
    // an authority that cites nothing
    assert!(!is_well_formed_marker(
        "// LAWPACK-LITERAL: referent=local-records; status=local; authority=because I said so"
    ));
    // a bare mention is not a marker
    assert!(!is_well_formed_marker("// this is fine, honestly"));

    // A WRAPPED marker is still a marker: the block is joined before it is parsed. The
    // first cut of this gate read only the marker's first line, so every marker whose
    // citation had wrapped past the line width failed - a check that fails on correct work
    // teaches people to weaken it.
    let wrapped = vec![
        "        // LAWPACK-LITERAL: referent=write-target; status=reserved; authority=[2026]",
        "        // VJS-CC-VJS 15. Prose explaining why, over several lines.",
        "        let dir = repo.join(\"lawpack/v2/orders\");",
    ];
    assert!(is_well_formed_marker(&marker_block(&wrapped, 0)));
    assert!(is_marked(&wrapped, 2));
}

/// The test-module detector must not swallow a bodyless `#[cfg(test)] mod tests;`
/// declaration. vjs-redact/src/lib.rs carries one on line 6, and treating it as a block
/// would exempt that whole file - including two RESERVED literals - from the gate.
#[test]
fn a_bodyless_test_module_declaration_exempts_nothing() {
    let src = vec![
        "use std::path::Path;",
        "",
        "#[cfg(test)]",
        "mod tests;",
        "",
        "fn f(p: &Path) -> bool {",
        "    p.ends_with(\"lawpack/v2\")",
        "}",
        "",
        "#[cfg(test)]",
        "mod inline {",
        "    const X: &str = \"lawpack/v2\";",
        "}",
    ];
    let blocks = test_module_lines(&src);
    assert_eq!(
        blocks.len(),
        1,
        "only the block form is a block: {blocks:?}"
    );
    let (a, b) = blocks[0];
    assert!(
        a <= 9 && b == 12,
        "the inline module spans 9..=12, got {a}..={b}"
    );
    assert!(
        !(a..=b).contains(&6),
        "the bodyless declaration must exempt nothing"
    );
}
