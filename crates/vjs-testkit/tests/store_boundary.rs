//! The decision-log store enforces the public/private boundary at write time:
//! a log carrying a secret, token, or private-network fact never reaches the
//! public record (it fails closed with a remedy, not post-hoc in validate).

use vjs_core::types::*;
use vjs_store::Store;

fn log(id: &str, why: &str) -> DecisionLog {
    DecisionLog {
        id: id.into(),
        time: "2026-06-09T00:00:00+00:00".into(),
        actor: "lexby".into(),
        kind: "decision".into(),
        issue: "test".into(),
        decision: "d".into(),
        basis: Vec::new(),
        risk: RiskLevel::Low,
        reversibility: "reversible".into(),
        court_required: false,
        why: why.into(),
    }
}

#[test]
fn a_log_carrying_a_secret_never_reaches_the_record() {
    let dir = std::env::temp_dir().join(format!("vjs-store-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();

    let bad = log(
        "LOG-BOUNDARY-SECRET",
        "deployed with ghp_0123456789abcdefghijklmnopqrstuvwxyzAB on the box at 192.168.50.1",
    );
    let err = Store::write_log(&dir, &bad);
    assert!(
        err.is_err(),
        "a secret-bearing log must fail the boundary scan"
    );
    assert!(
        !dir.join(".vjs/logs/decisions/LOG-BOUNDARY-SECRET.yaml")
            .exists(),
        "nothing may hit disk on a failed scan"
    );

    let good = log(
        "LOG-BOUNDARY-CLEAN",
        "routine permit-gate hardening, no private facts",
    );
    assert!(Store::write_log(&dir, &good).is_ok());
    assert!(
        dir.join(".vjs/logs/decisions/LOG-BOUNDARY-CLEAN.yaml")
            .exists()
    );

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn a_log_carrying_raw_identity_never_reaches_the_record() {
    // K-20 (the IDENTITY half): "no raw sensitive content/IDENTITY stored by default".
    // The store boundary fails closed on identity, not only secrets - an email address or
    // an internal hostname in a log's `why` is an Error-severity boundary finding and the
    // log never hits disk. (The goal-completion audit, 2026-06-26, observed the canon-write
    // path downgrades identity to Warning; this proves the STORE path - the one K-20 is
    // about - blocks it.)
    let dir = std::env::temp_dir().join(format!("vjs-store-id-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();

    let email = log(
        "LOG-BOUNDARY-EMAIL",
        "decided after a call with alice.smith@example.com about the rollout",
    );
    assert!(
        Store::write_log(&dir, &email).is_err(),
        "a log carrying a raw email address must fail the boundary scan (identity)"
    );
    assert!(
        !dir.join(".vjs/logs/decisions/LOG-BOUNDARY-EMAIL.yaml").exists(),
        "an identity-bearing log must not hit disk"
    );

    let host = log(
        "LOG-BOUNDARY-HOST",
        "routed via the box at fileserver.internal during the migration",
    );
    assert!(
        Store::write_log(&dir, &host).is_err(),
        "a log naming an internal hostname must fail the boundary scan (identity)"
    );
    assert!(
        !dir.join(".vjs/logs/decisions/LOG-BOUNDARY-HOST.yaml").exists(),
        "an internal-hostname log must not hit disk"
    );

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn a_corrupt_proof_is_an_error_not_a_silent_absence() {
    let dir = std::env::temp_dir().join(format!("vjs-proof-test-{}", std::process::id()));
    std::fs::create_dir_all(dir.join(".vjs/proofs")).unwrap();
    std::fs::write(dir.join(".vjs/proofs/PROOF-CORRUPT.yaml"), "{{not yaml").unwrap();

    assert!(
        Store::read_proofs(&dir).is_err(),
        "a vanished proof would read as an unmet obligation or mask tampering"
    );

    std::fs::remove_dir_all(&dir).ok();
}
