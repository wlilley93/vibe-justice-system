//! Red seeds for the canon-licence gate ([2026] VJS-PC 11 D2).
//!
//! Every one of these is a state the canon was ACTUALLY in, or one step from. The
//! gate exists because for close to a month the canon shipped a licence its own
//! binding law forbids and three files disagreed about which licence that was, and
//! nothing anywhere said a word.

use std::path::{Path, PathBuf};

fn findings(repo: &Path) -> Vec<(String, String)> {
    let mut out = Vec::new();
    vjs_engine::canon_licence::canon_licence_findings(repo, &mut out);
    out.into_iter()
        .map(|f| (format!("{:?}", f.severity), f.code))
        .collect()
}

const AGPL_HEAD: &str = "                    GNU AFFERO GENERAL PUBLIC LICENSE\n\
                         Version 3, 19 November 2007\n";
const POLYFORM_HEAD: &str = "PolyForm Noncommercial License 1.0.0\n\nAcceptance\n";

/// A tree shaped like the canon: the kernel source in it, and a manifest that says
/// `canonical = true`. Both halves are load-bearing - see `is_the_canon`.
fn canon_scratch(tag: &str, cargo_licence: &str, license_text: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("vjs-canonlic-{}-{tag}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("crates/vjs-core/src")).unwrap();
    std::fs::create_dir_all(dir.join("lawpack/v2")).unwrap();
    std::fs::write(dir.join("crates/vjs-core/src/lib.rs"), "// kernel\n").unwrap();
    std::fs::write(
        dir.join("lawpack/v2/manifest.toml"),
        "id = \"vjs-v2\"\ncanonical = true\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("Cargo.toml"),
        format!("[workspace.package]\nversion = \"0.1.0\"\nlicense = \"{cargo_licence}\"\n"),
    )
    .unwrap();
    std::fs::write(dir.join("LICENSE"), license_text).unwrap();
    dir
}

fn record_the_conflict(dir: &Path, operative: &str) {
    std::fs::create_dir_all(dir.join(".vjs/logs/breaches")).unwrap();
    std::fs::write(
        dir.join(".vjs/logs/breaches/BREACH-test.yaml"),
        format!("code: CANON-LICENCE-NOT-CONFORMING\noperative: \"{operative}\"\n"),
    )
    .unwrap();
}

#[test]
fn the_conforming_canon_earns_silence() {
    let dir = canon_scratch("ok", "AGPL-3.0-only", AGPL_HEAD);
    assert!(findings(&dir).is_empty(), "{:?}", findings(&dir));
}

#[test]
fn the_actual_2026_07_11_state_is_fatal_on_both_duties() {
    // THE RED SEED, and it is the recorded event: the anonymising squash replaced
    // LICENSE with PolyForm while Cargo.toml went on reciting AGPL. Two codes, not
    // one, because a drift is a mistake and a non-conforming licence is a breach of
    // a binding directive - and curing either alone leaves the other standing.
    let dir = canon_scratch("real", "AGPL-3.0-only", POLYFORM_HEAD);
    let f = findings(&dir);
    assert!(
        f.contains(&("Fatal".into(), "CANON-LICENCE-DRIFT".into())),
        "{f:?}"
    );
    assert!(
        f.contains(&("Fatal".into(), "CANON-LICENCE-NOT-CONFORMING".into())),
        "{f:?}"
    );
}

#[test]
fn making_the_files_agree_on_the_wrong_licence_does_not_clear_the_order() {
    // The cure that was IN FLIGHT when this gate was written, and the reason the two
    // duties are separate codes. Updating Cargo.toml to match LICENSE makes the
    // repository self-consistent and leaves it in breach of PC 11 D2 - with the last
    // signal that anything was wrong now silenced. The drift clears. The breach does
    // not, and must not.
    let dir = canon_scratch(
        "agree-wrong",
        "LicenseRef-PolyForm-Noncommercial-1.0.0",
        POLYFORM_HEAD,
    );
    let f = findings(&dir);
    assert!(
        !f.iter().any(|(_, c)| c == "CANON-LICENCE-DRIFT"),
        "the files now agree, so the drift is genuinely cured: {f:?}"
    );
    assert!(
        f.contains(&("Fatal".into(), "CANON-LICENCE-NOT-CONFORMING".into())),
        "agreeing on a forbidden licence is not conformity: {f:?}"
    );
}

#[test]
fn a_recorded_conflict_is_held_open_at_warning_and_an_unrecorded_one_is_not() {
    // Same rule as the store register's `deregistered:`: the unresolved state is
    // lawful while it is recorded and awaiting the only person who can decide it,
    // and unlawful the moment it is silent. Straight Fatal would refuse the very
    // filings that put the question to the copyright holder.
    let dir = canon_scratch("recorded", "AGPL-3.0-only", POLYFORM_HEAD);
    assert!(
        findings(&dir).iter().all(|(s, _)| s == "Fatal"),
        "unrecorded is Fatal"
    );
    record_the_conflict(&dir, "PolyForm Noncommercial License 1.0.0");
    let f = findings(&dir);
    assert!(!f.is_empty(), "recording it must not silence it: {f:?}");
    assert!(
        f.iter().all(|(s, _)| s == "Warning"),
        "a recorded conflict is held open, never dropped: {f:?}"
    );
}

#[test]
fn a_filing_naming_a_different_licence_does_not_hold_anything_open() {
    // What stops the record becoming a box to tick. The filing must name the ACTUAL
    // operative licence, so it cannot be written in advance and cannot be generic -
    // and it goes stale by itself the moment the licence changes again, at which
    // point the Fatal returns, because a new licence is a new question.
    let dir = canon_scratch("stale-filing", "AGPL-3.0-only", POLYFORM_HEAD);
    record_the_conflict(&dir, "BSL-1.1");
    let f = findings(&dir);
    assert!(
        f.iter().all(|(s, _)| s == "Fatal"),
        "a filing about some other licence is not a record of THIS conflict: {f:?}"
    );
}

#[test]
fn a_subscriber_is_not_bound_by_the_canons_licence_condition() {
    // PC 11 D2 binds THE CANON. A subscriber holding a vendored copy of the lawpack
    // licenses its own work as it pleases, and a gate that told it otherwise would
    // be the canon reaching into an estate it does not govern. The discriminator is
    // the kernel SOURCE, not `canonical = true`, because a vendored lawpack carries
    // that flag too.
    let dir = canon_scratch("subscriber", "MIT", POLYFORM_HEAD);
    std::fs::remove_dir_all(dir.join("crates")).unwrap();
    assert!(
        findings(&dir).is_empty(),
        "{:?} - a subscriber is silent",
        findings(&dir)
    );
}

#[test]
fn an_absent_licence_file_or_declaration_is_never_a_pass() {
    let dir = canon_scratch("no-license-file", "AGPL-3.0-only", AGPL_HEAD);
    std::fs::remove_file(dir.join("LICENSE")).unwrap();
    assert!(
        findings(&dir).contains(&("Fatal".into(), "CANON-LICENCE-UNDECLARED".into())),
        "{:?}",
        findings(&dir)
    );

    let dir = canon_scratch("no-cargo-licence", "AGPL-3.0-only", AGPL_HEAD);
    std::fs::write(
        dir.join("Cargo.toml"),
        "[workspace.package]\nversion = \"0.1.0\"\n",
    )
    .unwrap();
    assert!(
        findings(&dir).contains(&("Fatal".into(), "CANON-LICENCE-UNDECLARED".into())),
        "{:?}",
        findings(&dir)
    );
}

#[test]
fn a_commented_out_licence_is_not_a_declaration() {
    // `a check on presence is not a check on the value`: the reader must take the
    // VALUE of `license =`, never the fact that the string AGPL appears in the file.
    let dir = canon_scratch("commented", "AGPL-3.0-only", AGPL_HEAD);
    std::fs::write(
        dir.join("Cargo.toml"),
        "[workspace.package]\n# license = \"AGPL-3.0-only\"\nlicense = \"MIT\"\n",
    )
    .unwrap();
    let f = findings(&dir);
    assert!(
        f.contains(&("Fatal".into(), "CANON-LICENCE-DRIFT".into())),
        "the live declaration is MIT and LICENSE is AGPL: {f:?}"
    );
}

/// THE ADMISSION PROOF for `crates/vjs-engine/src/canon_licence.rs` joining
/// ENFORCEMENT_SURFACE ([2026] VJS-CC-VJS 18 C6: a file joins only on a committed test
/// showing a CONFINED edit to it flipping a bright-line outcome while the lock stays
/// green).
///
/// The confined edit is a one-character-class change to the severity hinge: return
/// `true` unconditionally from `conflict_is_on_the_record`, so every non-conforming
/// licence reports Warning whether or not anyone ever filed anything. Nothing else in
/// the workspace changes. No other pinned file changes, so every other digest in the
/// lock still matches and the lock reads green.
///
/// The outcome that flips is a bright line: whether a breach of a binding directive
/// blocks or merely mentions. It is simulated here rather than performed, because the
/// suite must not edit the kernel it is testing - what the test proves is that the
/// finding's severity is a FUNCTION of that one predicate, so an edit confined to this
/// file moves it, and only the entrenchment digest would notice.
#[test]
fn a_confined_edit_to_the_severity_hinge_flips_a_bright_line() {
    let dir = canon_scratch("admission", "AGPL-3.0-only", POLYFORM_HEAD);

    // As shipped, with nothing filed: the bright line BLOCKS.
    let blocking = findings(&dir);
    assert!(
        !blocking.is_empty() && blocking.iter().all(|(s, _)| s == "Fatal"),
        "as shipped an unrecorded breach is Fatal: {blocking:?}"
    );

    // The confined edit's exact effect, reached through the predicate's only input:
    // make `conflict_is_on_the_record` answer yes. Same file, same findings, same
    // codes, severity inverted across the board.
    record_the_conflict(&dir, "PolyForm Noncommercial License 1.0.0");
    let permissive = findings(&dir);
    assert_eq!(
        blocking.iter().map(|(_, c)| c).collect::<Vec<_>>(),
        permissive.iter().map(|(_, c)| c).collect::<Vec<_>>(),
        "the codes are identical either way - only the severity moves, which is what \
         makes this edit invisible to anything but a digest"
    );
    assert!(
        permissive.iter().all(|(s, _)| s == "Warning"),
        "and it moves to the non-blocking side: {permissive:?}"
    );

    // Why the pin is the only witness: an author who hardcoded that predicate to `true`
    // would ship this second state permanently, and every OTHER entrenched file would
    // still hash to its pin. Only this file's digest moves. Hence the entry.
    assert!(
        vjs_core::enforcement::ENFORCEMENT_SURFACE
            .contains(&"crates/vjs-engine/src/canon_licence.rs"),
        "the gate whose severity hinge this test just moved must be entrenched"
    );
}
