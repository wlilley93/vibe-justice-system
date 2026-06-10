use std::path::{Path, PathBuf};

use vjs_core::*;
use vjs_lawpack::*;

#[test]
fn test_local_ci_on_v2_repo() {
    let repo = repo_root();
    let lawpack = LawpackLoader::load(&repo.join("lawpack/v2")).unwrap();
    let report = LawpackValidator::validate(&lawpack).unwrap();
    assert!(report.ok, "Lawpack validation should pass");
}

#[test]
fn test_route_basic() {
    let repo = repo_root();
    let ctx = build_kernel_context(&repo).unwrap();
    let input = RouteInput {
        repo_root: Some(repo.clone()),
        jurisdiction: Some(JurisdictionId("default".into())),
        actor: "test".into(),
        action_kind: ActionKind::ImplementationDecision,
        issue_tags: vec![IssueTag("test".into())],
        intent: "Test route".into(),
        affected_paths: Vec::new(),
        risk: RiskLevel::Low,
        public_target: false,
        external_target: false,
        irreversible: false,
        user_instruction: None,
    };
    let decision = route(input, &ctx).unwrap();
    assert!(!decision.court_required);
}

#[test]
fn test_invariant_evaluation() {
    let repo = repo_root();
    let lawpack = LawpackLoader::load(&repo.join("lawpack/v2")).unwrap();
    let repo_state = RepoScanner::build_repo_state(&repo).unwrap();
    let facts = vjs_lawpack::lawpack_facts(&repo, &lawpack);
    let report = evaluate_invariants(&repo_state, &lawpack.invariants, &facts).unwrap();

    let failures: Vec<_> = report.findings.iter().filter(|f| !f.passed).collect();
    assert!(failures.is_empty(), "All invariants should pass: failures: {:?}", 
        failures.iter().map(|f| &f.invariant_id.0).collect::<Vec<_>>());
}

#[test]
fn test_validate_command() {
    let repo = repo_root();
    let lawpack = LawpackLoader::load(&repo.join("lawpack/v2")).unwrap();
    let report = LawpackValidator::validate(&lawpack).unwrap();
    assert!(report.ok);
    assert!(!report.findings.iter().any(|f| matches!(f.severity, Severity::Fatal | Severity::Error)));
}

#[test]
fn test_citation_uniqueness() {
    let repo = repo_root();
    let lawpack = LawpackLoader::load(&repo.join("lawpack/v2")).unwrap();
    let mut seen = std::collections::HashSet::new();
    for order in &lawpack.orders {
        assert!(seen.insert(order.id.clone()), "Duplicate order citation: {}", order.id);
    }
}

fn build_kernel_context(repo: &Path) -> Result<KernelContext, vjs_core::error::KernelError> {
    let lawpack = LawpackLoader::load(&repo.join("lawpack/v2"))?;
    let graph = lawpack.build_authority_graph()?;
    let digest = compute_digest(repo)?;
    Ok(KernelContext {
        authority_graph: graph,
        limits: ContextLimits::default(),
        lawpack_digest: digest,
    })
}

fn compute_digest(repo: &Path) -> Result<String, vjs_core::error::KernelError> {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    let manifest = repo.join("lawpack/v2/manifest.toml");
    if manifest.exists() {
        let content = std::fs::read(&manifest)
            .map_err(|e| vjs_core::error::KernelError::Io(e.to_string()))?;
        hasher.update(&content);
    }
    Ok(format!("sha256:{}", hex::encode(hasher.finalize())))
}

fn repo_root() -> PathBuf {
    // Anchor on the crate manifest: cargo runs integration tests from the
    // package dir, where "./lawpack/v2" resolves to nothing and every
    // lawpack-backed assertion would pass vacuously over an empty lawpack.
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn orders_load_with_the_new_optional_fields() {
    let repo = repo_root();
    let lawpack = LawpackLoader::load(&repo.join("lawpack/v2")).unwrap();
    // The existing orders still load (the new fields are optional)...
    assert!(lawpack.orders.len() >= 22, "all orders load: {}", lawpack.orders.len());
    // ...and the citation the orders already carried is now a real field.
    let pc7 = lawpack.orders.iter().find(|o| o.id == "2026-VJS-PC-007").unwrap();
    assert_eq!(pc7.citation.as_deref(), Some("[2026] VJS-PC 7"));
    assert_eq!(pc7.assent_source.as_deref(), Some("standing_bounded_assent"));
}
