//! Red-seed tests for `vjs preserve-check` (ACT-RECTIFICATION-COMMISSION s5). The
//! seeds are the round-5 Guardrail conditions that killed three deny-list drafts of
//! s5: G-A (a root-sequence file whose items are the nodes) and G-B (the type flip
//! `appealable: "true"` -> `appealable: true` passing a name-based proof). Plus the
//! statute's own enumerated failure modes: lost key, gained key, sequence loss and
//! reorder, comment loss - and the one permitted difference, style.

mod lawpack_common;
use lawpack_common::VJS;
use std::path::{Path, PathBuf};
use std::process::Command;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("vjs-preserve-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn check(dir: &Path, before: &str, after: &str) -> (bool, String) {
    std::fs::write(dir.join("before.yaml"), before).unwrap();
    std::fs::write(dir.join("after.yaml"), after).unwrap();
    let out = Command::new(VJS)
        .arg("preserve-check")
        .arg(dir.join("before.yaml"))
        .arg(dir.join("after.yaml"))
        .output()
        .unwrap();
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    (out.status.success(), text)
}

#[test]
fn a_type_flip_fails_the_proof() {
    // THE G-B SEED: a name-based proof passed while appealable's parsed type flipped.
    let dir = scratch("typeflip");
    let (ok, out) = check(
        &dir,
        "appealable: \"true\"\ncourt: county\n",
        "appealable: true\ncourt: county\n",
    );
    assert!(!ok, "{out}");
    assert!(out.contains("PARSED TYPE CHANGED"), "{out}");
}

#[test]
fn a_key_lost_at_depth_fails_whatever_its_name() {
    let dir = scratch("keyloss");
    let (ok, out) = check(
        &dir,
        "order:\n  bench: [a, b, c]\n  case_file_digest: sha256:aa\n",
        "order:\n  bench: [a, b, c]\n",
    );
    assert!(!ok, "{out}");
    assert!(out.contains("PRESENT BEFORE, ABSENT AFTER"), "{out}");
}

#[test]
fn a_gained_key_fails_too() {
    let dir = scratch("keygain");
    let (ok, out) = check(&dir, "a: 1\n", "a: 1\nb: 2\n");
    assert!(!ok, "{out}");
    assert!(out.contains("ABSENT BEFORE, PRESENT AFTER"), "{out}");
}

#[test]
fn a_sequence_that_loses_or_reorders_items_fails() {
    let dir = scratch("seq");
    let (ok, out) = check(&dir, "items: [one, two, three]\n", "items: [one, three]\n");
    assert!(!ok, "{out}");
    assert!(out.contains("SEQUENCE LENGTH CHANGED"), "{out}");
    let (ok2, out2) = check(&dir, "items: [one, two]\n", "items: [two, one]\n");
    assert!(!ok2, "a reorder is a failure: {out2}");
}

#[test]
fn a_root_sequence_is_compared_as_a_node() {
    // THE G-A SEED: the file's root is a sequence; losing an item must fail even
    // though every surviving key name still exists somewhere.
    let dir = scratch("rootseq");
    let (ok, out) = check(&dir, "- id: one\n- id: two\n", "- id: one\n");
    assert!(!ok, "{out}");
    assert!(out.contains("SEQUENCE LENGTH CHANGED"), "{out}");
}

#[test]
fn a_dropped_comment_fails() {
    let dir = scratch("comment");
    let (ok, out) = check(&dir, "# the reason this order exists\nid: X\n", "id: X\n");
    assert!(!ok, "{out}");
    assert!(
        out.contains("comment PRESENT BEFORE, ABSENT AFTER"),
        "{out}"
    );
}

#[test]
fn style_quoting_indent_and_key_order_are_the_permitted_differences() {
    let dir = scratch("style");
    let (ok, out) = check(
        &dir,
        "# kept\nid: \"X\"\ntitle: hello world\nnested:\n  a: 1\n  b: 2\n",
        "# kept\ntitle: 'hello world'\nid: X\nnested:\n    b: 2\n    a: 1\n",
    );
    assert!(
        ok,
        "style, quoting, indent and key order are permitted: {out}"
    );
    assert!(out.contains("CONTENT PRESERVED"), "{out}");
}

#[test]
fn an_unparseable_after_file_refuses_the_proof() {
    let dir = scratch("unloadable");
    let (ok, out) = check(&dir, "a: 1\n", "a: [unclosed\n");
    assert!(!ok, "{out}");
    assert!(
        out.contains("not content-preserving") || out.contains("REFUSED"),
        "{out}"
    );
}

#[test]
fn a_hash_inside_a_block_scalar_is_content_not_a_comment() {
    let dir = scratch("blockscalar");
    let (ok, out) = check(
        &dir,
        "text: |\n  line with # not a comment\n  more\n",
        "text: |\n  line with # not a comment\n  more\n",
    );
    assert!(ok, "identical files with block-scalar # must pass: {out}");
}
