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
