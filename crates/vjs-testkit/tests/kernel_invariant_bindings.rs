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

/// [2026] VJS-CC-VJS 18 C3 (D5): THE DRIFT WITNESS, PROVED AT THE GOVERNED BOUNDARY AND NOT
/// AT A PROXY ([2026] VJS-CC-VJS 13).
///
/// WHY K-25 ABOVE IS NOT THIS. It calls `check_drift` DIRECTLY and asserts the ABSENCE of
/// drift on a clean tree. It therefore passes whether or not `validate` ever calls
/// `check_drift` at all: delete the sole production call site in
/// `crates/vjs-engine/src/lib.rs` and K-25 stays GREEN while the shipped binary stops
/// reporting gate drift entirely. Absence proves nothing where the finding was never
/// reachable. This test closes that seam by running the REAL `vjs_engine::validate` and
/// requiring the Fatal back out of the returned `Report`.
///
/// REACHABILITY FIRST ([2026] VJS-CC-VJS 14 obiter (i)). The SAME fixture is asserted clean
/// before it is dirtied, so the assertion of PRESENCE is made in a fixture where absence was
/// the measured baseline - not in one where nothing could ever have fired.
///
/// PROOF IT CAN FAIL. Delete `findings.extend(vjs_core::enforcement::check_drift(repo));`
/// from `validate` and THIS TEST GOES RED while `the_committed_enforcement_surface_matches_its_pin`
/// STAYS GREEN. That divergence is the whole content of the condition. Entrenching
/// `vjs-engine/src/lib.rs` makes such an excision loud; it cannot make the binary honest
/// about its own excision, and this out-of-band witness is what does (K-27).
#[test]
fn validate_reports_enforcement_surface_drift_through_the_real_pipeline() {
    let repo = std::env::temp_dir().join(format!("vjs-cc18-c3-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&repo);

    // A fixture repo carrying an enforcement lock and ONE pinned gate file. No lawpack and
    // no `.vjs/config.toml`, so it is not a jurisdiction and `validate` runs to completion.
    // `staged: false` keeps git out of it entirely.
    std::fs::create_dir_all(repo.join("crates/vjs-core/src")).unwrap();
    std::fs::create_dir_all(repo.join(".vjs")).unwrap();
    let gate = repo.join("crates/vjs-core/src/bench.rs");
    std::fs::write(&gate, "pub fn verify_bench() -> bool { true }\n").unwrap();
    vjs_core::enforcement::write_lock(&repo).expect("the fixture pins its own surface");

    let opts = vjs_engine::ValidateOpts {
        staged: false,
        external: false,
    };

    // BASELINE: freshly pinned, so the fixture is clean. A Fatal later cannot be an artefact
    // of the fixture's shape.
    let clean = vjs_engine::validate(&repo, &opts).expect("validate runs on the clean fixture");
    assert!(
        !clean
            .findings
            .iter()
            .any(|f| f.code == "ENFORCEMENT_SURFACE_DRIFT"),
        "a freshly-pinned fixture must report NO drift; got: {:?}",
        clean.findings.iter().map(|f| &f.code).collect::<Vec<_>>()
    );

    // One pinned file's bytes now differ from the pin - the weakening edit, in miniature.
    std::fs::write(&gate, "pub fn verify_bench() -> bool { false }\n").unwrap();

    let dirty = vjs_engine::validate(&repo, &opts).expect("validate runs on the edited fixture");
    let drift = dirty
        .findings
        .iter()
        .find(|f| f.code == "ENFORCEMENT_SURFACE_DRIFT")
        .unwrap_or_else(|| {
            panic!(
                "C3: `vjs_engine::validate` must report ENFORCEMENT_SURFACE_DRIFT for a pinned \
                 file whose bytes differ. If this is missing, the sole production call of \
                 `vjs_core::enforcement::check_drift` in crates/vjs-engine/src/lib.rs is gone \
                 and the binary no longer witnesses its own gates. Findings: {:?}",
                dirty.findings.iter().map(|f| &f.code).collect::<Vec<_>>()
            )
        });
    assert!(
        matches!(drift.severity, vjs_core::types::Severity::Fatal),
        "the drift finding must be Fatal, not advisory"
    );
    assert!(
        drift.message.contains("crates/vjs-core/src/bench.rs"),
        "the finding must NAME the gate that drifted: {}",
        drift.message
    );
    assert!(!dirty.ok, "a Fatal drift must make the report not-ok");

    let _ = std::fs::remove_dir_all(&repo);
}
