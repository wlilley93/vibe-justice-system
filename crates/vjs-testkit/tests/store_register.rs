//! Red seeds for the store-register gate (ACT-PROCEEDINGS-DISCIPLINE s13). Each is
//! paired with the case that must fail; a register whose red case was never
//! demonstrated is a YAML file.

use std::path::{Path, PathBuf};

fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("vjs-storereg-{}-{tag}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join(".vjs/orders")).unwrap();
    std::fs::create_dir_all(dir.join("lawpack/v2")).unwrap();
    dir
}

fn findings(repo: &Path) -> Vec<(String, String)> {
    let mut out = Vec::new();
    vjs_engine::store_register::store_register_findings(repo, &mut out);
    out.into_iter()
        .map(|f| (format!("{:?}", f.severity), f.code))
        .collect()
}

fn write_register(repo: &Path, stores: &[&str]) {
    let body: String = stores
        .iter()
        .map(|s| format!("- path: {s}\n  kind: test\n"))
        .collect();
    std::fs::write(
        repo.join(".vjs/store-register.yaml"),
        format!("stores:\n{body}"),
    )
    .unwrap();
}

#[test]
fn an_absent_register_is_disclosed_never_silently_passed() {
    let dir = scratch("absent");
    let f = findings(&dir);
    assert_eq!(f.len(), 1, "{f:?}");
    assert_eq!(f[0], ("Info".into(), "STORE-REGISTER-UNTRACKED".into()));
}

#[test]
fn a_root_missing_from_the_register_is_fatal() {
    // THE RED SEED: the register names the lawpack but omits .vjs/orders - a map
    // missing a continent. The audit must refuse to sweep by it.
    let dir = scratch("missing-root");
    write_register(&dir, &["lawpack/v2"]);
    let f = findings(&dir);
    assert!(
        f.contains(&("Fatal".into(), "STORE-UNREGISTERED".into())),
        "{f:?}"
    );
}

#[test]
fn a_ghost_entry_warns() {
    let dir = scratch("ghost");
    write_register(&dir, &["lawpack/v2", ".vjs/orders", "no/such/store"]);
    let f = findings(&dir);
    assert!(
        f.contains(&("Warning".into(), "STORE-REGISTER-GHOST".into())),
        "{f:?}"
    );
    assert!(!f.iter().any(|(_, c)| c == "STORE-UNREGISTERED"), "{f:?}");
}

#[test]
fn a_complete_register_is_silent() {
    let dir = scratch("complete");
    write_register(&dir, &["lawpack/v2", ".vjs/orders"]);
    let f = findings(&dir);
    assert!(f.is_empty(), "a complete register earns silence: {f:?}");
}

#[test]
fn a_garbled_or_empty_register_is_fatal_not_vacuous() {
    let dir = scratch("garbled");
    std::fs::write(dir.join(".vjs/store-register.yaml"), "stores: [unclosed").unwrap();
    let f = findings(&dir);
    assert!(
        f.contains(&("Fatal".into(), "STORE-REGISTER-GARBLED".into())),
        "{f:?}"
    );
    std::fs::write(dir.join(".vjs/store-register.yaml"), "stores: []\n").unwrap();
    let f = findings(&dir);
    assert!(
        f.contains(&("Fatal".into(), "STORE-REGISTER-GARBLED".into())),
        "a register with no entries would certify completeness over nothing: {f:?}"
    );
}

/// A scratch tree that is a REAL git repo with the register committed, which is the
/// only state in which the lost-entry witness has anything to compare against.
fn committed_scratch(tag: &str, stores: &[&str]) -> PathBuf {
    let dir = scratch(tag);
    let git = |args: &[&str]| {
        let ok = std::process::Command::new("git")
            .args(args)
            .current_dir(&dir)
            // a scratch repo must not inherit the caller's index or worktree, or the
            // "commit" lands in the repo running the suite
            .env_remove("GIT_DIR")
            .env_remove("GIT_WORK_TREE")
            .env_remove("GIT_INDEX_FILE")
            .output()
            .unwrap()
            .status
            .success();
        assert!(ok, "git {args:?} failed in {}", dir.display());
    };
    git(&["init", "-q"]);
    git(&["config", "user.email", "seed@vjs.test"]);
    git(&["config", "user.name", "seed"]);
    git(&["config", "commit.gpgsign", "false"]);
    write_register(&dir, stores);
    git(&["add", "-A"]);
    git(&["-c", "core.hooksPath=/dev/null", "commit", "-qm", "seed"]);
    dir
}

#[test]
fn a_store_dropped_from_the_register_without_a_word_is_fatal() {
    // THE RED SEED, and it is a RECORDED event, not a hypothetical: on 2026-08-06 a
    // YAML-aware repair reserialised the register and dropped two entries. Note the
    // shape deliberately - the store that disappears here is NOT a governed record
    // root, so the completeness duty cannot see it. Before D4 this passed in total
    // silence while the register went on reporting itself checked.
    let dir = committed_scratch("lost", &["lawpack/v2", ".vjs/orders", "archive/v1"]);
    write_register(&dir, &["lawpack/v2", ".vjs/orders"]);
    let f = findings(&dir);
    assert!(
        f.contains(&("Fatal".into(), "STORE-REGISTER-ENTRY-LOST".into())),
        "a store may leave the register, but never quietly: {f:?}"
    );
}

#[test]
fn a_declared_deregistration_is_lawful_and_earns_silence() {
    // Deregistration is lawful; silent deregistration is not. One line of YAML with
    // a reason and an authority is the whole difference, which is the point: the
    // witness must not make the lawful act expensive, only the silent one impossible.
    let dir = committed_scratch("dereg", &["lawpack/v2", ".vjs/orders", "archive/v1"]);
    std::fs::write(
        dir.join(".vjs/store-register.yaml"),
        "stores:\n- path: lawpack/v2\n  kind: test\n- path: .vjs/orders\n  kind: test\n\
         deregistered:\n- path: archive/v1\n  reason: folded into lawpack/v2\n  \
         authority: \"[2026] VJS-CC-VJS 20 D4\"\n",
    )
    .unwrap();
    let f = findings(&dir);
    assert!(
        !f.iter().any(|(_, c)| c == "STORE-REGISTER-ENTRY-LOST"),
        "a declared deregistration is the lawful route and must pass: {f:?}"
    );
}

#[test]
fn an_unchanged_register_raises_no_ghost_of_a_loss() {
    // The witness must not cry wolf on the ordinary case, which is every commit that
    // does not touch the register at all.
    let dir = committed_scratch("unchanged", &["lawpack/v2", ".vjs/orders"]);
    let f = findings(&dir);
    assert!(f.is_empty(), "an unchanged register earns silence: {f:?}");
}

#[test]
fn a_register_with_no_committed_self_is_silent_not_witnessed() {
    // A fresh subscriber has no HEAD copy of the register, so nothing CAN have been
    // lost. That is a proof, not a bounded search, so it is silence and not a
    // disclosure - and a new jurisdiction is not greeted by a finding about a
    // history it does not have.
    let dir = scratch("uncommitted");
    write_register(&dir, &["lawpack/v2", ".vjs/orders"]);
    let f = findings(&dir);
    assert!(f.is_empty(), "{f:?}");
}

#[test]
fn an_unregistered_continuity_citator_is_fatal() {
    // THE RED SEED from the 2026-08-05 live probe: the first gate version enforced
    // only the governed roots, so deleting the .justice entry passed silently while
    // the registry row claimed the duty wired. The citator is citation-bearing and
    // nameable, so its omission is Fatal wherever the tree carries one.
    let dir = scratch("citator");
    std::fs::create_dir_all(dir.join(".justice")).unwrap();
    write_register(&dir, &["lawpack/v2", ".vjs/orders"]);
    let f = findings(&dir);
    assert!(
        f.contains(&("Fatal".into(), "STORE-UNREGISTERED".into())),
        "{f:?}"
    );
    write_register(&dir, &["lawpack/v2", ".vjs/orders", ".justice"]);
    let f = findings(&dir);
    assert!(f.is_empty(), "registered citator earns silence: {f:?}");
}
