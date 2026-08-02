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
    vjs_core::enforcement::write_lock(&repo, "[2026] VJS-CC-VJS 18 C7 (test fixture)")
        .expect("the fixture pins its own surface");

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

/// [2026] VJS-CC-VJS 18 C7 (D5): THE UNREADABLE-LOCK FATAL, PROVED AT THE GOVERNED BOUNDARY.
///
/// WHY THIS TEST AND NOT THE UNIT TESTS. `enforcement.rs`'s own tests call `check_drift`
/// directly, so they prove the finding is CONSTRUCTED. They cannot prove it is REPORTED: the
/// production path is `vjs_engine::validate`, and a Fatal that never reaches the Report is a
/// Fatal nobody is stopped by. C7 introduced a stricter parse, which is to say it introduced
/// NEW WAYS FOR THE LOCK TO FAIL TO LOAD, and the code it replaced reported every load
/// failure as `None` - byte-identical to an un-pinned repository. So the day the authority
/// field landed was the day a corrupt lock could have become silence.
///
/// REACHABILITY FIRST. The same fixture is asserted clean under a well-formed lock before it
/// is corrupted, so PRESENCE is asserted where ABSENCE was the measured baseline.
///
/// THE PRE-C7 FLAT FORMAT IS THE CORRUPTION USED, deliberately: it is not a synthetic garbage
/// string but the exact bytes every lock on the estate carried until 2026-08-02. If a
/// subscriber's binary is upgraded before its lock is re-pinned, THIS is the state it lands
/// in, and it must be loud.
///
/// PROOF IT CAN FAIL, AND THE ISOLATION STEP IT NEEDS. Map every parse failure in
/// `read_lock` back to "no lock" (the pre-C7 shape) and this test goes red. Run it that way
/// and `the_committed_enforcement_surface_matches_its_pin` goes red TOO - but for a
/// mechanical reason with nothing to do with the seed: `enforcement.rs` is itself on the
/// entrenched surface, so ANY edit to it moves its digest off the pin. Re-pin the seeded
/// tree (`vjs enforcement-lock --authority '<anything>'`) and the divergence is exact:
/// 4 green, and only this test red. VERIFIED 2026-08-02, both ways.
///
/// That step is written down because without it a reader reproduces the seed, sees two
/// failures, and concludes the seed is merely noisy rather than that it disarmed a specific
/// gate. Two red tests where one is collateral looks identical to two red tests where both
/// are real.
#[test]
fn validate_reports_an_unreadable_enforcement_lock_through_the_real_pipeline() {
    let repo = std::env::temp_dir().join(format!("vjs-cc18-c7-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&repo);
    std::fs::create_dir_all(repo.join("crates/vjs-core/src")).unwrap();
    std::fs::create_dir_all(repo.join(".vjs")).unwrap();
    std::fs::write(
        repo.join("crates/vjs-core/src/bench.rs"),
        "pub fn verify_bench() -> bool { true }\n",
    )
    .unwrap();
    vjs_core::enforcement::write_lock(&repo, "[2026] VJS-CC-VJS 18 C7 (test fixture)")
        .expect("the fixture pins its own surface");

    let opts = vjs_engine::ValidateOpts {
        staged: false,
        external: false,
    };

    // BASELINE: a well-formed lock reports nothing. Without this the assertion below could
    // be satisfied by a fixture that was Fatal for some unrelated reason.
    let clean = vjs_engine::validate(&repo, &opts).expect("validate runs on the clean fixture");
    assert!(
        !clean
            .findings
            .iter()
            .any(|f| f.code == "ENFORCEMENT_LOCK_UNREADABLE"),
        "a lock this workspace just wrote must be readable; got: {:?}",
        clean.findings.iter().map(|f| &f.code).collect::<Vec<_>>()
    );

    // The pre-C7 flat format: a real lock, in the format the whole estate used yesterday.
    std::fs::write(
        repo.join(".vjs/enforcement-surface.lock"),
        "# VJS entrenched-enforcement-surface pin (PC-16 D4).\n\
         crates/vjs-core/src/bench.rs sha256:0000000000000000000000000000000000000000000000000000000000000000\n",
    )
    .unwrap();

    let corrupt = vjs_engine::validate(&repo, &opts).expect("validate runs on the corrupt fixture");
    let f = corrupt
        .findings
        .iter()
        .find(|f| f.code == "ENFORCEMENT_LOCK_UNREADABLE")
        .unwrap_or_else(|| {
            panic!(
                "C7: `vjs_engine::validate` must report ENFORCEMENT_LOCK_UNREADABLE for a lock \
                 that exists and cannot be parsed. If this is missing, an unparseable lock is \
                 being reported as an un-pinned repository and NOTHING is being witnessed. \
                 Findings: {:?}",
                corrupt.findings.iter().map(|f| &f.code).collect::<Vec<_>>()
            )
        });
    assert!(
        matches!(f.severity, vjs_core::types::Severity::Fatal),
        "an unwitnessed surface must be Fatal, not advisory: a warning is not a refusal"
    );
    assert!(
        f.message.contains("NOTHING WAS CHECKED"),
        "the finding must say that nothing was checked - 'unverified' and 'verified-good' are \
         the two readings an operator must never confuse. Got: {}",
        f.message
    );
    let _ = std::fs::remove_dir_all(&repo);
}
