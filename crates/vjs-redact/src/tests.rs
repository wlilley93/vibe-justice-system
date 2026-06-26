//! Canon-gate scanner tests. Split out of lib.rs (behavior-preserving) to keep
//! each file under the structural-cleanliness ceiling. Included as a
//! `#[cfg(test)] mod tests;`, so `super` here is the crate root (the scanner).
//! Still a pinned enforcement surface: weakening these proofs is non-silent.

use super::*;
use std::path::PathBuf;

fn is_blocked(findings: &[BoundaryFinding]) -> bool {
    !RedactScanner::check_public_safe(findings)
}

// The real DEC-ACMECO-UNITARY-STACK-001 shape: canon-format citation, no explicit
// repo_code, subscriber org scope path, subscriber repo_code in the id.
const ACMECO_DECISION: &str = r#"
id: DEC-ACMECO-UNITARY-STACK-001
citation: "[2026] VJS-DEC 15"
title: The Unitary Stack - one source of truth for Acmeco kernel-owned data
scope:
  paths:
    - Executive/ministry-of-business-engineering-and-skills/engineering-department/projects/acmeco/**
"#;

#[test]
fn blocks_the_acmeco_decision_that_self_asserted_into_canon() {
    let (f, code) = RedactScanner::scan_canon_record(
        &PathBuf::from("lawpack/v2/decisions/DEC-ACMECO-UNITARY-STACK-001.yaml"),
        ACMECO_DECISION,
        "VJS",
        &[], // empty registry: scope-path corroboration alone must catch it
    );
    assert!(
        is_blocked(&f),
        "subscriber-scoped canon record must be blocked"
    );
    assert!(
        f.iter()
            .any(|x| matches!(x.kind, BoundaryFindingKind::PrivateRepoPath)),
        "the foreign scope path must surface as PrivateRepoPath"
    );
    assert_eq!(
        code.as_deref(),
        Some("ACMECO"),
        "the foreign code is corroborated"
    );
}

#[test]
fn blocks_explicit_foreign_repo_code() {
    let rec = "id: DEC-X-001\nrepo_code: ACMECO\ncitation: \"[2026] VJS-DEC 99\"\n";
    let (f, _) = RedactScanner::scan_canon_record(
        &PathBuf::from("lawpack/v2/decisions/x.yaml"),
        rec,
        "VJS",
        &[],
    );
    assert!(is_blocked(&f));
    assert!(
        f.iter()
            .any(|x| matches!(x.kind, BoundaryFindingKind::UnredactedEvidence))
    );
}

#[test]
fn canon_secret_scan_blocks_credentials_but_only_warns_on_hostnames() {
    // Audit 2026-06-26: scan_file never ran over the canon tree, so a credential
    // committed into a lawpack/v2 record reached the public repo undetected. Now it
    // runs in scan_canon_writes - credentials HARD-BLOCK, boundary-example hostnames warn.
    let base = std::env::temp_dir().join(format!("vjs-redact-canonscan-{}", std::process::id()));
    let dir = base.join("lawpack/v2/decisions");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("cred.yaml"),
        "id: DEC-X\ntoken: ghp_0123456789abcdefghijABCDEFGHIJ0123456\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("host.yaml"),
        "id: DEC-Y\nexample: private_store.local\n",
    )
    .unwrap();

    let creds = RedactScanner::scan_canon_writes(
        &base,
        &[PathBuf::from("lawpack/v2/decisions/cred.yaml")],
        "VJS",
    );
    assert!(
        !RedactScanner::check_public_safe(&creds),
        "a GitHub token in a canon record must hard-block"
    );
    assert!(
        creds
            .iter()
            .any(|f| matches!(f.kind, BoundaryFindingKind::Token))
    );

    let hosts = RedactScanner::scan_canon_writes(
        &base,
        &[PathBuf::from("lawpack/v2/decisions/host.yaml")],
        "VJS",
    );
    assert!(
        RedactScanner::check_public_safe(&hosts),
        "a .local boundary example must surface as a non-blocking Warning, not block canon"
    );
    assert!(
        hosts
            .iter()
            .any(|f| matches!(f.kind, BoundaryFindingKind::PrivateHostname)),
    );
    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn registry_catches_a_foreign_id_code_without_scope_corroboration() {
    // No scope path, no repo_code field - only the id carries ACMECO. Corroboration
    // alone would miss it; the federation registry (#11) catches it.
    let rec = "id: DEC-ACMECO-SNEAKY-001\ncitation: \"[2026] VJS-DEC 98\"\ntitle: x\n";
    let codes = vec!["ACMECO".to_string()];
    let (f, code) = RedactScanner::scan_canon_record(
        &PathBuf::from("lawpack/v2/decisions/y.yaml"),
        rec,
        "VJS",
        &codes,
    );
    assert!(
        is_blocked(&f),
        "a registered subscriber code in the id must block"
    );
    assert_eq!(code.as_deref(), Some("ACMECO"));
    // Without the registry, the same record (no scope, no repo_code) passes.
    let (f2, _) = RedactScanner::scan_canon_record(
        &PathBuf::from("lawpack/v2/decisions/y.yaml"),
        rec,
        "VJS",
        &[],
    );
    assert!(
        !is_blocked(&f2),
        "uncorroborated, unregistered id-code does not trip"
    );
}

#[test]
fn does_not_flag_a_clean_canon_record() {
    let clean = r#"
id: REG-KERNEL-001
citation: "[2026] VJS-REG 1"
title: The kernel is the single smart enforcement point
scope:
  paths:
    - crates/vjs-core/**
    - lawpack/v2/**
    - .vjs/**
"#;
    let (f, code) = RedactScanner::scan_canon_record(
        &PathBuf::from("lawpack/v2/regulations/REG-KERNEL-001.yaml"),
        clean,
        "VJS",
        &["ACMECO".to_string()],
    );
    assert!(
        !is_blocked(&f),
        "a clean canon record (KERNEL is not a foreign code) must pass: {f:?}"
    );
    assert_eq!(code, None);
}

#[test]
fn prose_body_naming_a_subscriber_is_blocked_but_the_registry_is_exempt() {
    // PC-15 boundary cure: a canon record whose BODY/prose names a registered
    // subscriber - even with a clean id, citation, and no scope path - is blocked.
    let codes = vec!["ACMECO".to_string()];
    let leak = "id: 2026-VJS-PC-099\ncitation: \"[2026] VJS-PC 99\"\nholding: A subscriber (Acmeco) asked canon to build a keystone.\n";
    let (f, _) = RedactScanner::scan_canon_record(
        &PathBuf::from("lawpack/v2/orders/2026-VJS-PC-099.yaml"),
        leak,
        "VJS",
        &codes,
    );
    assert!(is_blocked(&f), "a subscriber named in prose must block");
    // The registry file itself lists the codes by design - it is exempt.
    let registry = "id: FEDERATION-SUBSCRIBER-REGISTRY\ncodes:\n  - ACMECO\n";
    let (rf, _) = RedactScanner::scan_canon_record(
        &PathBuf::from("lawpack/v2/federation/subscriber-registry.yaml"),
        registry,
        "VJS",
        &codes,
    );
    assert!(!is_blocked(&rf), "the registry is exempt: {rf:?}");
}

#[test]
fn wildcard_and_root_file_scopes_are_canon() {
    for p in ["*", "**", "Cargo.toml", "AGENTS.md", "public/**", "src/**"] {
        assert!(
            !RedactScanner::is_foreign_canon_path(p),
            "{p} must be treated as canon surface, not a foreign path"
        );
    }
    assert!(RedactScanner::is_foreign_canon_path(
        "Executive/ministry-of-business-engineering-and-skills/engineering-department/projects/acmeco/**"
    ));
    assert!(RedactScanner::is_foreign_canon_path(
        "frontend-v2/prisma/**"
    ));
}
