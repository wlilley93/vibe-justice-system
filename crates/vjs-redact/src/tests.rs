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
        &CanonRepoCode::inferred("VJS"),
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
        &CanonRepoCode::inferred("VJS"),
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

// ---------------------------------------------------------------------------
// The canon-code resolver, and the mirror-jurisdiction defect it cures. A repo whose
// lawpack/v2 is a lawful read-only mirror of VJS canon holds records that all carry
// `repo_code: VJS`; gated against the LOCAL config code every one of them read as a
// subscriber's law filed into canon. These proofs are the negative control: they fail
// again the moment the manifest read stops being load-bearing.
// ---------------------------------------------------------------------------

/// A mirror jurisdiction on disk. `manifest_code` is the lawpack's own declaration (None =
/// a pre-cure lawpack, silent on the field); `registry_code` is the accessioned subscriber
/// list, absent when None. Three records: one mirrored canon record coded VJS, one local
/// record coded OPBOX, one carrying no code at all.
fn mirror_repo(tag: &str, manifest_code: Option<&str>, registry_code: Option<&str>) -> PathBuf {
    let base = std::env::temp_dir().join(format!("vjs-redact-mirror-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let v2 = base.join("lawpack/v2");
    std::fs::create_dir_all(v2.join("decisions")).unwrap();

    let mut manifest = String::from("id = \"vjs-v2\"\nversion = \"0.1.0\"\n");
    if let Some(rc) = manifest_code {
        manifest.push_str(&format!("repo_code = \"{rc}\"\n"));
    }
    // A table follows the top-level keys, as in the real manifest: the scan must stop here.
    manifest.push_str("\n[limits]\nroute_max_words = 300\n");
    std::fs::write(v2.join("manifest.toml"), manifest).unwrap();

    if let Some(code) = registry_code {
        std::fs::create_dir_all(v2.join("federation")).unwrap();
        std::fs::write(
            v2.join("federation/subscriber-registry.yaml"),
            format!("id: FEDERATION-SUBSCRIBER-REGISTRY\ncodes:\n  - {code}\n"),
        )
        .unwrap();
    }

    std::fs::write(
        v2.join("decisions/mirrored.yaml"),
        "id: DEC-KERNEL-001\nrepo_code: VJS\ntitle: a mirrored canon record\n",
    )
    .unwrap();
    std::fs::write(
        v2.join("decisions/local.yaml"),
        "id: DEC-KERNEL-002\nrepo_code: OPBOX\ntitle: a local record\n",
    )
    .unwrap();
    std::fs::write(
        v2.join("decisions/neutral.yaml"),
        "id: DEC-KERNEL-003\ntitle: a record carrying no repo_code\n",
    )
    .unwrap();
    base
}

fn rel(name: &str) -> Vec<PathBuf> {
    vec![PathBuf::from(format!("lawpack/v2/decisions/{name}"))]
}

#[test]
fn a_declared_canon_code_beats_the_local_config_in_a_mirror_jurisdiction() {
    let base = mirror_repo("declared", Some("VJS"), None);
    let code = resolve_canon_repo_code(&base, Some("OPBOX"), Some("opbox"));
    assert_eq!(code, CanonRepoCode::declared("VJS"), "the lawpack declares");

    let mirrored = RedactScanner::scan_canon_writes(&base, &rel("mirrored.yaml"), &code);
    assert!(
        RedactScanner::check_public_safe(&mirrored),
        "a mirrored canon record coded VJS is enacted canon, not a subscriber's law: {mirrored:?}"
    );

    let local = RedactScanner::scan_canon_writes(&base, &rel("local.yaml"), &code);
    assert!(
        is_blocked(&local),
        "the subscriber's OWN code in the mirrored canon tree still blocks"
    );
    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn without_the_manifest_declaration_the_mirrored_record_blocks_again() {
    // The companion control: delete `repo_code` from the manifest and the same VJS-coded
    // record is blocked once more. If this passes, the manifest read is decoration.
    let base = mirror_repo("undeclared", None, None);
    let code = resolve_canon_repo_code(&base, Some("OPBOX"), Some("opbox"));
    assert_eq!(
        code,
        CanonRepoCode::inferred("OPBOX"),
        "silent lawpack: the config chain applies unchanged"
    );
    let mirrored = RedactScanner::scan_canon_writes(&base, &rel("mirrored.yaml"), &code);
    assert!(
        is_blocked(&mirrored),
        "with no declaration the gate falls back to OPBOX and blocks the VJS record"
    );
    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn a_declared_canon_code_naming_a_registered_subscriber_is_blocked() {
    // No code capture: declaring an accessioned subscriber's code as the canon's own would
    // make signal 4 skip that subscriber, switching the prose limb off for it silently.
    let base = mirror_repo("capture", Some("OPBOX"), Some("OPBOX"));
    let code = resolve_canon_repo_code(&base, Some("OPBOX"), Some("opbox"));
    assert!(code.declared);
    let f = RedactScanner::scan_canon_writes(&base, &rel("neutral.yaml"), &code);
    assert!(is_blocked(&f), "a captured canon code must block");
    assert!(
        f.iter().any(|x| x
            .path
            .as_ref()
            .is_some_and(|p| p.ends_with("manifest.toml"))),
        "the finding must name the manifest that declared it: {f:?}"
    );
    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn a_config_fallback_to_the_repos_own_subscriber_code_does_not_trip_code_capture() {
    // The other half of C4. OPBOX is in the registry and IS this repo's code, but nothing
    // declared it as canon's - that is the ordinary subscriber posture and must pass.
    let base = mirror_repo("fallback", None, Some("OPBOX"));
    let code = resolve_canon_repo_code(&base, Some("OPBOX"), Some("opbox"));
    assert!(!code.declared);
    let f = RedactScanner::scan_canon_writes(&base, &rel("neutral.yaml"), &code);
    assert!(
        RedactScanner::check_public_safe(&f),
        "an inferred code equal to a registered subscriber is not code capture: {f:?}"
    );
    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn canon_repo_code_source_order_runs_manifest_config_jurisdiction_then_vjs() {
    let declared = mirror_repo("order-declared", Some("VJS"), None);
    let silent = mirror_repo("order-silent", None, None);
    assert_eq!(
        resolve_canon_repo_code(&declared, Some("OPBOX"), Some("opbox")).code,
        "VJS"
    );
    assert_eq!(
        resolve_canon_repo_code(&silent, Some("OPBOX"), Some("opbox")).code,
        "OPBOX"
    );
    assert_eq!(
        resolve_canon_repo_code(&silent, None, Some("opbox")).code,
        "OPBOX",
        "the jurisdiction id upper-cased is the third source"
    );
    assert_eq!(
        resolve_canon_repo_code(&silent, Some("  "), Some("")).code,
        "VJS",
        "empty is not a declaration: the tail applies"
    );
    assert_eq!(resolve_canon_repo_code(&silent, None, None).code, "VJS");
    // A `repo_code` under a table is not the lawpack's own declaration.
    assert_eq!(
        manifest_repo_code_in("id = \"x\"\n\n[limits]\nrepo_code = \"OPBOX\"\n"),
        None
    );
    assert_eq!(
        manifest_repo_code_in("repo_code_extra = \"OPBOX\"\nrepo_code = \"VJS\"\n").as_deref(),
        Some("VJS")
    );
    let _ = std::fs::remove_dir_all(&declared);
    let _ = std::fs::remove_dir_all(&silent);
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
