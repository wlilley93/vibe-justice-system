//! Real bindings for global invariants (docs/global-invariants.yaml) that the VJS kernel
//! satisfies by mechanism but had no test for - levelling the kernel up by closing the
//! K-29 binding debt with genuine tests, not paper claims.

use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// K-12: deterministic, model-free, network-free decisions. The sound witness is the
/// dependency fence (deny.toml): no HTTP client or hosted-model SDK may enter the kernel
/// closure. This test binds the invariant to that fence.
#[test]
fn the_kernel_closure_bans_network_and_model_crates() {
    let deny = std::fs::read_to_string(workspace_root().join("deny.toml")).expect("deny.toml");
    for banned in [
        "reqwest",
        "hyper",
        "ureq",
        "curl",
        "async-openai",
        "anthropic",
    ] {
        assert!(
            deny.contains(&format!("crate = \"{banned}\"")),
            "K-12: the dependency fence must ban '{banned}' (network/model egress) from the kernel closure"
        );
    }
}

/// K-25: the enforcement surface is digest-pinned outside the witnessed code, so any gate
/// edit is non-silent. This exercises `check_drift` against the REAL committed surface: the
/// pin must match the gate sources (it would catch a forgotten re-lock after a gate edit).
#[test]
fn the_committed_enforcement_surface_matches_its_pin() {
    let root = workspace_root();
    let drift = vjs_core::enforcement::check_drift(&root);
    assert!(
        drift.is_empty(),
        "K-25: the committed enforcement surface does not match its pin (re-lock with `vjs enforcement-lock`): {:?}",
        drift.iter().map(|f| &f.code).collect::<Vec<_>>()
    );
    // and the pinned surface actually covers the gate sources
    assert!(vjs_core::enforcement::ENFORCEMENT_SURFACE.contains(&"crates/vjs-engine/src/assent.rs"));
    assert!(vjs_core::enforcement::ENFORCEMENT_SURFACE.contains(&"crates/vjs-core/src/front_door.rs"));
}

/// K-27: the required-CI trust root re-runs the SAME deterministic gate on the canonical
/// remote, so a local `--no-verify` bypass cannot reach it. Binds the invariant to the CI
/// config: canon-enforce must re-run `validate --staged` and the workspace test suite.
#[test]
fn required_ci_reruns_the_same_deterministic_gate() {
    let ci = std::fs::read_to_string(
        workspace_root().join(".github/workflows/canon-enforce.yml"),
    )
    .expect(".github/workflows/canon-enforce.yml");
    assert!(
        ci.contains("validate --staged"),
        "K-27: the CI trust root must re-run the staged validate gate"
    );
    assert!(
        ci.contains("cargo test"),
        "K-27: the CI trust root must re-run the workspace test suite (which carries the K-29 gate)"
    );
}
