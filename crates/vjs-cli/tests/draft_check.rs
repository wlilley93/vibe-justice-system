//! Red-seed tests for the Clerk Gate (`vjs draft check` / `vjs certify`), Operation
//! Watertight WS1. Every seed is a REAL error from the ACT 11/12 drafting record,
//! 2026-08-05 - each cost a committee round to find by review; each must now be a
//! free refusal. The tests drive the BINARY (a unit test over a helper would pass on
//! code the command never calls).

mod lawpack_common;
use lawpack_common::{VJS, copy_tree, real_lawpack, run, scratch};
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    // real_lawpack() is <root>/lawpack/v2, canonicalized.
    real_lawpack()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn write_draft(dir: &Path, name: &str, content: &str) -> PathBuf {
    let p = dir.join(name);
    std::fs::write(&p, content).unwrap();
    p
}

const WELL_FORMED_TAIL: &str =
    "- id: ACT-TEST-DRAFT:s9\n  title: Commencement\n  text: This Act commences on assent.\n";

#[test]
fn a_misspelled_kernel_effect_key_is_reported_as_dropped_law() {
    // THE SEED: the round-1 ACT 11 draft wrote `forbids:` where the struct has
    // `prohibits:`; serde dropped the list and the law under it did not exist at
    // runtime. Three committee rounds orbited that draft.
    let dir = scratch("draft-forbids");
    let draft = write_draft(
        &dir,
        "draft.yaml",
        "id: ACT-TEST-DRAFT\ntitle: Test\nstatus: draft\nsections:\n- id: ACT-TEST-DRAFT:s1\n  title: One\n  text: A duty.\n  kernel_effect:\n    forbids:\n    - do_bad_thing\n",
    );
    let (ok, out) = run(&repo_root(), &["draft", "check", draft.to_str().unwrap()]);
    assert!(!ok, "a dropped-law key must block: {out}");
    assert!(
        out.contains("DRAFT-UNKNOWN-KEY") && out.contains("'forbids'"),
        "names the dropped key: {out}"
    );
}

#[test]
fn a_root_sequence_document_is_refused() {
    // THE SEED: the G-A root-sequence hole, proven on a real canon file - a leading
    // `- ` turns the instrument into a YAML list and the kernel misreads everything.
    let dir = scratch("draft-rootseq");
    let draft = write_draft(
        &dir,
        "draft.yaml",
        "- id: ACT-TEST-DRAFT\n  title: Test\n  status: draft\n",
    );
    let (ok, out) = run(&repo_root(), &["draft", "check", draft.to_str().unwrap()]);
    assert!(!ok);
    assert!(out.contains("DRAFT-ROOT-SEQUENCE"), "{out}");
}

#[test]
fn a_line_cite_past_the_end_of_the_file_is_refused() {
    // THE SEED: the `:6`-for-`:7` wrong-line-cite class. The out-of-range form is the
    // hard-refusable half; the near-miss half is the token-window warning below.
    let dir = scratch("draft-linecite");
    let draft = write_draft(
        &dir,
        "draft.yaml",
        &format!(
            "id: ACT-TEST-DRAFT\ntitle: Test\nstatus: draft\nsections:\n- id: ACT-TEST-DRAFT:s1\n  title: One\n  text: >\n    READ crates/vjs-cli/src/main.rs:999999 before acting.\n{WELL_FORMED_TAIL}"
        ),
    );
    let (ok, out) = run(&repo_root(), &["draft", "check", draft.to_str().unwrap()]);
    assert!(!ok, "{out}");
    assert!(out.contains("DRAFT-ADDRESS-BAD"), "{out}");
}

#[test]
fn a_quoted_token_far_from_its_cited_line_draws_the_window_warning() {
    let dir = scratch("draft-tokenwin");
    let draft = write_draft(
        &dir,
        "draft.yaml",
        &format!(
            "id: ACT-TEST-DRAFT\ntitle: Test\nstatus: draft\nsections:\n- id: ACT-TEST-DRAFT:s1\n  title: One\n  text: >\n    the token `zzz_token_that_appears_nowhere` is defined at crates/vjs-cli/src/main.rs:3.\n{WELL_FORMED_TAIL}"
        ),
    );
    let (ok, out) = run(&repo_root(), &["draft", "check", draft.to_str().unwrap()]);
    assert!(ok, "a token-window miss warns, never blocks: {out}");
    assert!(out.contains("DRAFT-ADDRESS-TOKEN"), "{out}");
}

#[test]
fn two_totals_for_one_measurement_are_flagged() {
    // THE SEED: the 84/85 double total - a draft asserted 84 registered stores in one
    // section and 85 in another after a re-measure, and a round was spent on it.
    let dir = scratch("draft-count");
    let draft = write_draft(
        &dir,
        "draft.yaml",
        &format!(
            "id: ACT-TEST-DRAFT\ntitle: Test\nstatus: draft\nsections:\n- id: ACT-TEST-DRAFT:s1\n  title: One\n  text: The register holds 84 registered stores.\n- id: ACT-TEST-DRAFT:s2\n  title: Two\n  text: Across all 85 registered stores the duty applies.\n{WELL_FORMED_TAIL}"
        ),
    );
    let (ok, out) = run(&repo_root(), &["draft", "check", draft.to_str().unwrap()]);
    assert!(ok, "a count inconsistency warns: {out}");
    assert!(
        out.contains("DRAFT-INCONSISTENT-COUNT") && out.contains("84, 85"),
        "{out}"
    );
}

#[test]
fn a_duplicate_duty_token_and_the_wired_preview_both_surface() {
    let dir = scratch("draft-duty");
    let draft = write_draft(
        &dir,
        "draft.yaml",
        &format!(
            "id: ACT-TEST-DRAFT\ntitle: Test\nstatus: draft\nsections:\n- id: ACT-TEST-DRAFT:s1\n  title: One\n  text: A duty.\n  kernel_effect:\n    must_not:\n    - publish_secrets\n- id: ACT-TEST-DRAFT:s2\n  title: Two\n  text: The same duty again.\n  kernel_effect:\n    must_not:\n    - publish_secrets\n{WELL_FORMED_TAIL}"
        ),
    );
    let (ok, out) = run(&repo_root(), &["draft", "check", draft.to_str().unwrap()]);
    assert!(ok, "dup + collision are warnings: {out}");
    assert!(out.contains("DRAFT-DUTY-DUP"), "{out}");
    // publish_secrets is a GATE_REGISTRY row, so the preview must count it wired.
    assert!(out.contains("2 wired, 0 unwired"), "{out}");
}

#[test]
fn an_internal_reference_to_a_missing_section_is_refused() {
    let dir = scratch("draft-sref");
    let draft = write_draft(
        &dir,
        "draft.yaml",
        &format!(
            "id: ACT-TEST-DRAFT\ntitle: Test\nstatus: draft\nsections:\n- id: ACT-TEST-DRAFT:s1\n  title: One\n  text: as s99 provides, this binds.\n{WELL_FORMED_TAIL}"
        ),
    );
    let (ok, out) = run(&repo_root(), &["draft", "check", draft.to_str().unwrap()]);
    assert!(!ok, "{out}");
    assert!(
        out.contains("DRAFT-SECTION-REF-BAD") && out.contains("s99"),
        "{out}"
    );
}

#[test]
fn an_unresolvable_citation_in_section_text_is_refused() {
    let dir = scratch("draft-cite");
    let draft = write_draft(
        &dir,
        "draft.yaml",
        &format!(
            "id: ACT-TEST-DRAFT\ntitle: Test\nstatus: draft\nsections:\n- id: ACT-TEST-DRAFT:s1\n  title: One\n  text: ACT-NO-SUCH-INSTRUMENT-ZZ:s3 governs this.\n{WELL_FORMED_TAIL}"
        ),
    );
    let (ok, out) = run(&repo_root(), &["draft", "check", draft.to_str().unwrap()]);
    assert!(!ok, "{out}");
    assert!(out.contains("DRAFT-CITE-UNRESOLVED"), "{out}");
}

#[test]
fn a_missing_commencement_is_disclosed() {
    let dir = scratch("draft-commence");
    let draft = write_draft(
        &dir,
        "draft.yaml",
        "id: ACT-TEST-DRAFT\ntitle: Test\nstatus: draft\nsections:\n- id: ACT-TEST-DRAFT:s1\n  title: One\n  text: A duty with no beginning.\n",
    );
    let (ok, out) = run(&repo_root(), &["draft", "check", draft.to_str().unwrap()]);
    assert!(ok, "missing commencement warns: {out}");
    assert!(out.contains("DRAFT-COMMENCEMENT-MISSING"), "{out}");
}

#[test]
fn a_denylisted_term_is_caught_at_drafting_not_at_the_enactment_commit() {
    // THE SEED: the assent-provenance file carried a subscriber name to the enactment
    // commit, where CANON_DENYLISTED_TERM refused it - one full cycle too late.
    let dir = scratch("draft-denylist");
    copy_tree(&real_lawpack(), &dir.join("lawpack/v2"));
    std::fs::create_dir_all(dir.join(".vjs")).unwrap();
    std::fs::write(
        dir.join(".vjs/publication-denylist.txt"),
        "zzyzx-subscriber\n",
    )
    .unwrap();
    let draft = write_draft(
        &dir,
        "draft.yaml",
        &format!(
            "id: ACT-TEST-DRAFT\ntitle: Test\nstatus: draft\nsections:\n- id: ACT-TEST-DRAFT:s1\n  title: One\n  text: The Zzyzx-Subscriber jurisdiction carries this duty.\n{WELL_FORMED_TAIL}"
        ),
    );
    let (ok, out) = run(&dir, &["draft", "check", draft.to_str().unwrap()]);
    assert!(!ok, "{out}");
    assert!(out.contains("DRAFT-DENYLISTED-TERM"), "{out}");
}

#[test]
fn certify_passes_a_header_only_delta_and_prints_it() {
    let dir = scratch("certify-header");
    let draft = write_draft(
        &dir,
        "engrossed.yaml",
        "id: ACT-X\nstatus: draft\nsections:\n- id: ACT-X:s1\n  title: One\n  text: the operative words\n",
    );
    let adopted = write_draft(
        &dir,
        "enacted.yaml",
        "id: ACT-X\nstatus: in_force\nassent_source: sovereign_assent\nsections:\n- id: ACT-X:s1\n  title: One\n  text: the operative words\n",
    );
    let out = std::process::Command::new(VJS)
        .args([
            "certify",
            draft.to_str().unwrap(),
            adopted.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.status.success(), "{text}");
    assert!(
        text.contains("IDENTICAL") && text.contains("header delta"),
        "{text}"
    );
    assert!(text.contains("+ status: in_force"), "{text}");
}

#[test]
fn certify_refuses_an_operative_divergence_and_names_the_line() {
    let dir = scratch("certify-diff");
    let draft = write_draft(
        &dir,
        "engrossed.yaml",
        "id: ACT-X\nstatus: draft\nsections:\n- id: ACT-X:s1\n  title: One\n  text: the adopted words\n",
    );
    let adopted = write_draft(
        &dir,
        "enacted.yaml",
        "id: ACT-X\nstatus: draft\nsections:\n- id: ACT-X:s1\n  title: One\n  text: different words entirely\n",
    );
    let out = std::process::Command::new(VJS)
        .args([
            "certify",
            draft.to_str().unwrap(),
            adopted.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(!out.status.success(), "{text}");
    assert!(
        text.contains("DIFFERENT") && text.contains("first divergence at operative line"),
        "{text}"
    );
}
