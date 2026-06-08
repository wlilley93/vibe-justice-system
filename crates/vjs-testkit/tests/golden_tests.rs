use std::path::PathBuf;

use vjs_core::*;
use vjs_core::types::*;
use vjs_lawpack::*;

#[test]
fn test_local_ci_on_v2_repo() {
    let repo = PathBuf::from(".");
    let lawpack = LawpackLoader::load(&repo.join("lawpack/v2")).unwrap();
    let report = LawpackValidator::validate(&lawpack).unwrap();
    assert!(report.ok, "Lawpack validation should pass");
}

#[test]
fn test_route_basic() {
    let repo = PathBuf::from(".");
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
    let repo = PathBuf::from(".");
    let lawpack = LawpackLoader::load(&repo.join("lawpack/v2")).unwrap();
    let repo_state = RepoScanner::build_repo_state(&repo).unwrap();
    let report = evaluate_invariants(&repo_state, &lawpack.invariants).unwrap();
    
    let failures: Vec<_> = report.findings.iter().filter(|f| !f.passed).collect();
    assert!(failures.is_empty(), "All invariants should pass: failures: {:?}", 
        failures.iter().map(|f| &f.invariant_id.0).collect::<Vec<_>>());
}

#[test]
fn test_validate_command() {
    let repo = PathBuf::from(".");
    let lawpack = LawpackLoader::load(&repo.join("lawpack/v2")).unwrap();
    let report = LawpackValidator::validate(&lawpack).unwrap();
    assert!(report.ok);
    assert!(!report.findings.iter().any(|f| matches!(f.severity, Severity::Fatal | Severity::Error)));
}

#[test]
fn test_citation_uniqueness() {
    let repo = PathBuf::from(".");
    let lawpack = LawpackLoader::load(&repo.join("lawpack/v2")).unwrap();
    let mut seen = std::collections::HashSet::new();
    for order in &lawpack.orders {
        assert!(seen.insert(order.id.clone()), "Duplicate order citation: {}", order.id);
    }
}

fn build_kernel_context(repo: &PathBuf) -> Result<KernelContext, vjs_core::error::KernelError> {
    let lawpack = LawpackLoader::load(&repo.join("lawpack/v2"))?;
    let graph = lawpack.build_authority_graph()?;
    let digest = compute_digest(repo)?;
    Ok(KernelContext {
        authority_graph: graph,
        limits: ContextLimits::default(),
        lawpack_digest: digest,
    })
}

fn compute_digest(repo: &PathBuf) -> Result<String, vjs_core::error::KernelError> {
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
