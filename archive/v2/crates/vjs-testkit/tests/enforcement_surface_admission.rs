//! [2026] VJS-CC-VJS 18 C6 (D3): THE BOUND ON THE ENTRENCHED SURFACE.
//!
//! The ratio admits a file only on PROOF - a committed test showing that an edit confined to
//! that file flips a bright-line outcome while the lock stays green. The anti-noise bound is
//! therefore not a category rule but a DOCKET: an addition costs a counterexample, so the
//! surface cannot grow by argument, and a list known to be short becomes a visible debt
//! rather than a silent one.
//!
//! WHERE IT IS RECORDED: in the `ENFORCEMENT_SURFACE` const itself, as an
//! `admitted-by: <test file>::<test fn>` note in the entry's own comment. Recorded at the
//! point of addition, in the file an author must already edit to make the addition, rather
//! than in a register they can forget.
//!
//! HOW IT IS CHECKED PER ADDITION: this test parses the const out of
//! `crates/vjs-core/src/enforcement.rs`, takes every entry that is not one of the founding
//! twelve, and requires an `admitted-by:` note whose named file EXISTS and whose named
//! function is DECLARED in it. Citing a test that does not exist is refused, which is the
//! difference between this and a comment.

use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// The surface as it stood when CC-VJS 18 imposed the bound. These twelve predate the proof
/// requirement and are grandfathered BY THE ORDER, not by omission: the ruling entrenches
/// the twelfth (the dispatcher) on its own measured counterexample and imposes the
/// requirement prospectively. Every path added after this list must pay.
const FOUNDING_TWELVE: &[&str] = &[
    "crates/vjs-engine/src/assent.rs",
    "crates/vjs-core/src/front_door.rs",
    "crates/vjs-core/src/bench.rs",
    "crates/vjs-core/src/hook.rs",
    "crates/vjs-core/src/governance/permit_gate.rs",
    "crates/vjs-redact/src/lib.rs",
    "crates/vjs-redact/src/tests.rs",
    "crates/vjs-engine/src/staged.rs",
    "crates/vjs-lawpack/src/validator.rs",
    "crates/vjs-lawpack/src/refs.rs",
    "crates/vjs-core/src/enforcement.rs",
    "crates/vjs-engine/src/lib.rs",
];

const MARKER: &str = "admitted-by:";

/// Every (path, admitting-comment) pair in the const, in source order. The comment block is
/// the run of `//` lines immediately above the entry PLUS any trailing comment on the entry
/// line itself, joined - so a note may wrap, which it will, because a test path plus a
/// function name does not fit beside a path literal.
fn entries_with_comments(src: &str) -> Vec<(String, String)> {
    let lines: Vec<&str> = src.lines().collect();
    let start = lines
        .iter()
        .position(|l| l.contains("pub const ENFORCEMENT_SURFACE"))
        .expect("ENFORCEMENT_SURFACE is declared in enforcement.rs");
    let end = lines[start..]
        .iter()
        .position(|l| l.trim() == "];")
        .map(|k| start + k)
        .expect("the ENFORCEMENT_SURFACE literal is closed");

    let mut out = Vec::new();
    for i in start + 1..end {
        let line = lines[i];
        // A COMMENT LINE IS NOT AN ENTRY, even when it quotes something. Caught by this
        // file's own parse-agreement test: the dispatcher's comment quotes the phrase
        // "this witness itself", and taking the first quoted run on every line invented a
        // thirteenth entry out of prose.
        if line.trim_start().starts_with("//") {
            continue;
        }
        let Some(open) = line.find('"') else { continue };
        let Some(close) = line[open + 1..].find('"') else {
            continue;
        };
        let path = line[open + 1..open + 1 + close].to_string();

        let mut comment = line[open + 1 + close..].to_string();
        let mut j = i;
        while j > start + 1 {
            j -= 1;
            let t = lines[j].trim_start();
            if let Some(body) = t.strip_prefix("//") {
                comment = format!("{body} {comment}");
            } else {
                break;
            }
        }
        out.push((path, comment));
    }
    out
}

#[test]
fn the_parse_agrees_with_the_const_the_kernel_actually_compiles() {
    // A source parser that has drifted from the const would let an addition through while
    // reporting green. Bind the two before relying on either.
    let src = std::fs::read_to_string(workspace_root().join("crates/vjs-core/src/enforcement.rs"))
        .expect("crates/vjs-core/src/enforcement.rs");
    let parsed: Vec<String> = entries_with_comments(&src)
        .into_iter()
        .map(|(p, _)| p)
        .collect();
    let actual: Vec<String> = vjs_core::enforcement::ENFORCEMENT_SURFACE
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(
        parsed, actual,
        "the C6 source parse must see exactly the paths the compiled const holds"
    );
}

#[test]
fn the_founding_twelve_are_all_still_entrenched() {
    // A floor, never a ceiling: the surface may grow (on proof), but it may not shrink by
    // silence. `check_drift` iterates the CONST, so a path deleted from it leaves an orphan
    // lock line that nothing reports (CC-VJS 18 obiter (iv)) - a removal is the quietest
    // possible de-entrenchment, and this is the only thing standing in its way.
    for path in FOUNDING_TWELVE {
        assert!(
            vjs_core::enforcement::ENFORCEMENT_SURFACE.contains(path),
            "C6: '{path}' was entrenched by order and cannot leave the surface silently"
        );
    }
}

#[test]
fn every_entry_beyond_the_twelve_cites_the_test_that_admitted_it() {
    let root = workspace_root();
    let src = std::fs::read_to_string(root.join("crates/vjs-core/src/enforcement.rs"))
        .expect("crates/vjs-core/src/enforcement.rs");

    let mut offences: Vec<String> = Vec::new();
    for (path, comment) in entries_with_comments(&src) {
        if FOUNDING_TWELVE.contains(&path.as_str()) {
            continue;
        }
        let Some(rest) = comment.split_once(MARKER).map(|(_, r)| r) else {
            offences.push(format!(
                "'{path}' joined ENFORCEMENT_SURFACE with no `{MARKER} <file>::<fn>` note. \
                 [2026] VJS-CC-VJS 18 C6: a file joins only on a committed test showing a \
                 confined edit to it flipping a bright-line outcome while the lock stays \
                 green. Name that test, or take the path out."
            ));
            continue;
        };
        // `<repo-relative path>.rs::<fn name>`, first whitespace-delimited token.
        let cite = rest.split_whitespace().next().unwrap_or_default();
        let Some((file, func)) = cite.split_once("::") else {
            offences.push(format!(
                "'{path}' cites '{cite}', which is not of the form <file>.rs::<test fn>"
            ));
            continue;
        };
        let Ok(body) = std::fs::read_to_string(root.join(file)) else {
            offences.push(format!(
                "'{path}' cites '{file}', which does not exist. A citation to a test that is \
                 not there is exactly the prose CC-VJS 15 held is not enforcement."
            ));
            continue;
        };
        if !body.contains(&format!("fn {func}")) {
            offences.push(format!(
                "'{path}' cites '{file}::{func}', but '{file}' declares no `fn {func}`"
            ));
        }
    }
    assert!(
        offences.is_empty(),
        "C6: the entrenched surface is bounded BY PROOF, not by category. Offences:\n{}",
        offences.join("\n")
    );
}

/// The gate's own negative control. Without it the check above could be satisfied by a
/// parser that finds nothing, which is a check that cannot fail.
#[test]
fn the_admission_check_can_actually_refuse() {
    let synthetic = r#"
pub const ENFORCEMENT_SURFACE: &[&str] = &[
    "crates/vjs-core/src/bench.rs", // founding, needs nothing
    // a newcomer with no proof at all
    "crates/some/newcomer.rs",
    // admitted-by: crates/vjs-testkit/tests/enforcement_surface_admission.rs::the_admission_check_can_actually_refuse
    "crates/some/proved.rs",
];
"#;
    let parsed = entries_with_comments(synthetic);
    assert_eq!(parsed.len(), 3, "the parser must see all three entries");

    let has_marker = |p: &str| {
        parsed
            .iter()
            .find(|(path, _)| path == p)
            .map(|(_, c)| c.contains(MARKER))
            .unwrap_or(false)
    };
    assert!(
        !has_marker("crates/some/newcomer.rs"),
        "an entry with no admitted-by note must be SEEN as having none"
    );
    assert!(
        has_marker("crates/some/proved.rs"),
        "an entry whose note sits on the line above must be seen as having one"
    );
    // and the note must resolve to a real function, which this one does - it is this test.
    let (_, c) = parsed
        .iter()
        .find(|(p, _)| p == "crates/some/proved.rs")
        .unwrap();
    let cite = c
        .split_once(MARKER)
        .unwrap()
        .1
        .split_whitespace()
        .next()
        .unwrap();
    let (file, func) = cite.split_once("::").unwrap();
    let body = std::fs::read_to_string(workspace_root().join(file)).expect("the cited file");
    assert!(
        body.contains(&format!("fn {func}")),
        "the cited function must be declared in the cited file"
    );
}
