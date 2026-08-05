//! THE CONFORMANCE RATCHET: the unwired-duty count is measured on every validate and may
//! only fall.
//!
//! WHY. `vjs audit` had exactly one caller and nothing invoked it - no hook, no CI. Measured
//! 2026-08-05: 281 duties, 43 wired, 238 unwired, and the number appeared nowhere anyone
//! looks. Every new Act lands its tokens unwired and no gate goes red, so the unbound surface
//! can only grow. The ratchet turns the count into a gate against a tracked baseline
//! (`.vjs/conformance.lock`).
//!
//! Every behaviour below is paired with the case that MUST fail. A ratchet whose red case was
//! never demonstrated is a number in a file.

use std::path::{Path, PathBuf};

fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("vjs-ratchet-{}-{tag}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join(".vjs")).unwrap();
    std::fs::create_dir_all(dir.join("lawpack/v2/statutes")).unwrap();
    std::fs::write(
        dir.join("lawpack/v2/manifest.toml"),
        "id = \"t\"\nversion = \"0\"\n",
    )
    .unwrap();
    dir
}

/// A minimal statute carrying `n` duty tokens, none of which is in GATE_REGISTRY - so the
/// fixture's unwired count IS `n`, and the test controls it exactly.
fn statute_with_unwired(dir: &Path, n: usize) {
    let mut musts = String::new();
    for i in 0..n {
        musts.push_str(&format!("      - ratchet_fixture_duty_{i}\n"));
    }
    std::fs::write(
        dir.join("lawpack/v2/statutes/01-fixture.yaml"),
        format!(
            "id: ACT-FIXTURE\ntitle: fixture\nstatus: in_force\nsections:\n\
             - id: ACT-FIXTURE:s1\n  title: t\n  text: fixture\n  kernel_effect:\n    must:\n{musts}"
        ),
    )
    .unwrap();
}

fn validate_codes(repo: &Path) -> Vec<(String, String)> {
    let report =
        vjs_engine::validate(repo, &vjs_engine::ValidateOpts::default()).expect("validates");
    report
        .findings
        .into_iter()
        .map(|f| (format!("{:?}", f.severity), f.code))
        .collect()
}

/// AT the baseline: no ratchet finding of any severity. The gate is silent when the law and
/// the ledger agree - a gate that cries wolf gets skipped.
#[test]
fn at_baseline_the_ratchet_is_silent() {
    let repo = scratch("at");
    statute_with_unwired(&repo, 3);
    std::fs::write(repo.join(".vjs/conformance.lock"), "unwired = 3\n").unwrap();
    let codes = validate_codes(&repo);
    assert!(
        !codes.iter().any(|(_, c)| c.starts_with("CONFORMANCE")),
        "at baseline there must be no conformance finding, got: {codes:?}"
    );
}

/// ABOVE the baseline is FATAL. This is the ratchet's whole job: new law landed with no gate
/// and no recorded acceptance.
#[test]
fn above_baseline_is_fatal() {
    let repo = scratch("above");
    statute_with_unwired(&repo, 4);
    std::fs::write(repo.join(".vjs/conformance.lock"), "unwired = 3\n").unwrap();
    let codes = validate_codes(&repo);
    assert!(
        codes
            .iter()
            .any(|(s, c)| c == "CONFORMANCE-RATCHET" && s == "Fatal"),
        "a rise must be Fatal, got: {codes:?}"
    );
}

/// BELOW the baseline warns to bank the improvement. A ratchet that does not tighten leaks:
/// the next regression would hide inside the slack.
#[test]
fn below_baseline_warns_to_bank_the_improvement() {
    let repo = scratch("below");
    statute_with_unwired(&repo, 2);
    std::fs::write(repo.join(".vjs/conformance.lock"), "unwired = 3\n").unwrap();
    let codes = validate_codes(&repo);
    assert!(
        codes
            .iter()
            .any(|(s, c)| c == "CONFORMANCE-IMPROVED" && s == "Warning"),
        "a fall must warn to lower the baseline, got: {codes:?}"
    );
}

/// An ABSENT baseline is disclosed, never silently skipped - and never Fatal, because a fresh
/// subscriber has no baseline yet and hard-failing would brick genesis (the O5 bootstrap-trap
/// reasoning). "Nothing to check" and "checked and clean" are different facts.
#[test]
fn an_absent_baseline_is_disclosed_not_skipped_and_not_fatal() {
    let repo = scratch("absent");
    statute_with_unwired(&repo, 3);
    let codes = validate_codes(&repo);
    assert!(
        codes
            .iter()
            .any(|(s, c)| c == "CONFORMANCE-UNTRACKED" && s == "Info"),
        "an absent baseline must be disclosed as Info, got: {codes:?}"
    );
    assert!(
        !codes
            .iter()
            .any(|(s, c)| c.starts_with("CONFORMANCE") && s == "Fatal"),
        "an absent baseline must not be Fatal (bootstrap trap), got: {codes:?}"
    );
}

/// A PRESENT baseline with no parseable `unwired = N` line is FATAL: a gate that skips on a
/// read failure looks exactly like a gate that passed (the gazette-denylist rule).
#[test]
fn a_garbled_baseline_is_fatal_not_skipped() {
    let repo = scratch("garbled");
    statute_with_unwired(&repo, 3);
    std::fs::write(repo.join(".vjs/conformance.lock"), "# no number here\n").unwrap();
    let codes = validate_codes(&repo);
    assert!(
        codes
            .iter()
            .any(|(s, c)| c == "CONFORMANCE-LOCK-UNREADABLE" && s == "Fatal"),
        "a present-but-unparseable baseline must be Fatal, got: {codes:?}"
    );
}

/// NEGATIVE CONTROL FOR THE FIXTURE ITSELF. The fixture's tokens must genuinely be unwired -
/// if one ever lands in GATE_REGISTRY the counts above stop meaning what the tests say.
#[test]
fn the_fixture_tokens_are_really_unwired() {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../vjs-lawpack/src/report.rs");
    let registry = std::fs::read_to_string(&src).unwrap();
    assert!(
        !registry.contains("ratchet_fixture_duty_"),
        "the fixture's tokens must never enter GATE_REGISTRY"
    );
}
