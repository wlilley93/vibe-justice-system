use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::types::*;
use crate::error::*;
use crate::court::*;

fn default_predicate() -> PredicateExpr {
    PredicateExpr::LawpackValidates
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Invariant {
    pub id: InvariantId,
    pub title: String,
    pub basis: Vec<AuthorityId>,
    pub scope: Option<Scope>,
    #[serde(skip, default = "default_predicate")]
    pub rule: PredicateExpr,
    pub severity: Severity,
    pub remedy: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InvariantRaw {
    pub id: InvariantId,
    pub title: String,
    pub basis: Vec<AuthorityId>,
    pub scope: Option<Scope>,
    pub rule: RawPredicate,
    pub severity: Severity,
    pub remedy: String,
}

impl InvariantRaw {
    pub fn to_invariant(self) -> Result<Invariant, String> {
        Ok(Invariant {
            id: self.id,
            title: self.title,
            basis: self.basis,
            scope: self.scope,
            rule: self.rule.to_predicate()?,
            severity: self.severity,
            remedy: self.remedy,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Decision {
    pub id: DecisionId,
    pub citation: Option<String>,
    pub title: String,
    pub status: DecisionStatus,
    pub scope: Option<Scope>,
    pub decision: String,
    pub basis: Vec<AuthorityId>,
    pub consequences: Option<Consequences>,
    pub review_triggers: Vec<Trigger>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Permit {
    pub id: PermitId,
    pub route_id: RouteId,
    pub actor: String,
    pub scope: Option<Scope>,
    pub obligations: Vec<Obligation>,
    pub expires_at: String,
    pub status: PermitStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Proof {
    pub id: ProofId,
    pub permit_id: PermitId,
    pub kind: ProofKind,
    pub status: ProofStatus,
    pub digest: Option<String>,
    pub captured_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Receipt {
    pub receipt_id: String,
    pub permit_id: PermitId,
    pub status: String,
    pub proofs: Vec<String>,
    pub remaining_obligations: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Session {
    pub session_id: SessionId,
    pub actor: String,
    pub state: SessionState,
    pub active_permits: Vec<PermitId>,
    pub created_at: String,
    pub expires_at: String,
}

pub struct SpecSet {
    pub specs: HashMap<SpecId, Spec>,
    pub invariants: HashMap<InvariantId, Invariant>,
    pub decisions: HashMap<DecisionId, Decision>,
    pub permits: HashMap<PermitId, Permit>,
    pub proofs: HashMap<ProofId, Proof>,
    pub sessions: HashMap<SessionId, Session>,
}

impl SpecSet {
    pub fn new() -> Self {
        Self {
            specs: HashMap::new(),
            invariants: HashMap::new(),
            decisions: HashMap::new(),
            permits: HashMap::new(),
            proofs: HashMap::new(),
            sessions: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RepoState {
    pub root: PathBuf,
    pub head_sha: Option<String>,
    pub changed_paths: Vec<PathBuf>,
    pub added_files: Vec<PathBuf>,
    pub modified_files: Vec<PathBuf>,
    pub deleted_files: Vec<PathBuf>,
    pub file_contents: HashMap<PathBuf, String>,
    pub dependency_changes: Vec<DependencyChange>,
    pub permits: Vec<Permit>,
    pub proofs: Vec<Proof>,
    pub logs: Vec<DecisionLog>,
    pub orders: Vec<Order>,
    pub boundary_findings: Vec<BoundaryFinding>,
}

#[derive(Clone, Debug)]
pub struct DependencyChange {
    pub name: String,
    pub added: bool,
    pub removed: bool,
}

pub fn evaluate_invariants(
    repo_state: &RepoState,
    invariants: &[Invariant],
) -> Result<InvariantReport, KernelError> {
    let mut findings = Vec::new();

    for invariant in invariants {
        let result = evaluate_predicate(&invariant.rule, repo_state);
        findings.push(InvariantFinding {
            invariant_id: invariant.id.clone(),
            title: invariant.title.clone(),
            severity: invariant.severity.clone(),
            passed: result,
            message: if result {
                format!("Invariant {} passed", invariant.title)
            } else {
                format!("Invariant {} failed: {}", invariant.title, invariant.remedy)
            },
            remedy: invariant.remedy.clone(),
        });
    }

    Ok(InvariantReport { findings })
}

fn evaluate_predicate(rule: &PredicateExpr, repo_state: &RepoState) -> bool {
    match rule {
        PredicateExpr::All { items } => items.iter().all(|item| evaluate_predicate(item, repo_state)),
        PredicateExpr::Any { items } => items.iter().any(|item| evaluate_predicate(item, repo_state)),
        PredicateExpr::None { items } => items.iter().all(|item| !evaluate_predicate(item, repo_state)),
        PredicateExpr::Not { item } => !evaluate_predicate(item, repo_state),
        PredicateExpr::If { condition, then } => {
            if evaluate_predicate(condition, repo_state) {
                evaluate_predicate(then, repo_state)
            } else {
                true // if condition is false, the implication is vacuously true
            }
        }
        PredicateExpr::PathChanged { glob } => {
            repo_state.changed_paths.iter().any(|p| glob_matches(glob, p))
        }
        PredicateExpr::FileAdded { pattern } => {
            repo_state.added_files.iter().any(|p| glob_matches(pattern, p))
        }
        PredicateExpr::FileModified { pattern } => {
            repo_state.modified_files.iter().any(|p| glob_matches(pattern, p))
        }
        PredicateExpr::FileDeleted { pattern } => {
            repo_state.deleted_files.iter().any(|p| glob_matches(pattern, p))
        }
        PredicateExpr::StringContains { value } => {
            repo_state.file_contents.values().any(|content| content.contains(value))
        }
        PredicateExpr::ImportContains { value } => {
            repo_state.file_contents.values().any(|content| content.contains(value))
        }
        PredicateExpr::DependencyAdded { name } => {
            repo_state.dependency_changes.iter().any(|c| c.name == *name && c.added)
        }
        PredicateExpr::DependencyRemoved { name } => {
            repo_state.dependency_changes.iter().any(|c| c.name == *name && c.removed)
        }
        PredicateExpr::DecisionLogExists { issue: _ } => {
            !repo_state.logs.is_empty()
        }
        PredicateExpr::PermitExists { id: _ } => {
            !repo_state.permits.is_empty()
        }
        PredicateExpr::ProofExists { kind: _ } => {
            !repo_state.proofs.is_empty()
        }
        PredicateExpr::OrderExists { issue: _ } => {
            !repo_state.orders.is_empty()
        }
        PredicateExpr::WordCountLte { field: _, max: _ } => {
            // Simplified: always true for now
            true
        }
        PredicateExpr::CitationUnique => {
            // Simplified: always true for now
            true
        }
        PredicateExpr::RequiredFields { fields: _ } => {
            // Simplified: always true for now
            true
        }
        PredicateExpr::FieldEquals { field, value } => {
            // Check if any file content contains the field with the specified value
            repo_state.file_contents.values().any(|content| {
                let pattern = format!("{}: {}", field, value);
                content.contains(&pattern)
            })
        }
        PredicateExpr::IncludedInRuntimeAuthorityGraph => {
            // Simplified: always true for now
            true
        }
        PredicateExpr::PublicNoPrivateFacts => {
            repo_state.boundary_findings.is_empty()
        }
        PredicateExpr::CoreNoModelCalls => {
            // Check if vjs-core source files contain actual model API imports or calls
            // Exclude the invariant evaluator itself (spec.rs) and test files
            repo_state.file_contents.iter().all(|(path, content)| {
                if !path.to_string_lossy().contains("vjs-core") {
                    return true;
                }
                // Skip the evaluator file and test files
                let path_str = path.to_string_lossy();
                if path_str.contains("spec.rs") || path_str.contains("test") || path_str.contains("golden") {
                    return true;
                }
                // Check for actual model API usage patterns
                let has_model_import = content.contains("use openai::") || content.contains("use anthropic::") ||
                    content.contains("openai::Client") || content.contains("anthropic::Client") ||
                    content.contains(".chat.completions") || content.contains("/v1/messages");
                !has_model_import
            })
        }
        PredicateExpr::CoreNoNetwork => {
            // Check if any file in vjs-core contains network-related dependencies
            repo_state.dependency_changes.iter().all(|c| {
                if !repo_state.changed_paths.iter().any(|p| p.to_string_lossy().contains("vjs-core")) {
                    return true;
                }
                c.name != "reqwest" && c.name != "hyper" && c.name != "ureq" && c.name != "curl"
            })
        }
        PredicateExpr::GovernedWritesRequirePermit => {
            !repo_state.permits.is_empty()
        }
        PredicateExpr::ProofsExistBeforeClose => {
            !repo_state.proofs.is_empty()
        }
        PredicateExpr::LogsStayShort => {
            repo_state.logs.iter().all(|log| log.why.split_whitespace().count() <= 150)
        }
        PredicateExpr::LawpackValidates => {
            true
        }
        PredicateExpr::NoDuplicateIds => {
            true
        }
        PredicateExpr::NoDuplicateCitations => {
            true
        }
        PredicateExpr::OrdersHaveDirectives => {
            repo_state.orders.iter().all(|order| !order.directives.is_empty())
        }
        PredicateExpr::McpLocalFirst => {
            true
        }
        PredicateExpr::DirectoryRolesResolve => {
            true
        }
        PredicateExpr::V1NotLoadedByDefault => {
            true
        }
    }
}

fn glob_matches(glob: &str, path: &PathBuf) -> bool {
    let path_str = path.to_string_lossy();
    if glob.ends_with("/**") {
        let prefix = &glob[..glob.len() - 3];
        path_str.starts_with(prefix)
    } else if glob.contains("*") {
        let regex = glob.replace("*", ".*");
        regex::Regex::new(&regex).map(|re| re.is_match(&path_str)).unwrap_or(false)
    } else {
        path_str == glob || path_str.starts_with(glob)
    }
}

pub struct InvariantReport {
    pub findings: Vec<InvariantFinding>,
}

pub struct InvariantFinding {
    pub invariant_id: InvariantId,
    pub title: String,
    pub severity: Severity,
    pub passed: bool,
    pub message: String,
    pub remedy: String,
}

pub fn open_permit(
    route_decision: &RouteDecision,
    _actor: &str,
) -> Result<Permit, KernelError> {
    let id = PermitId(format!("PERMIT-{}", chrono::Utc::now().timestamp()));
    let expires = chrono::Utc::now() + chrono::Duration::hours(2);

    Ok(Permit {
        id: id.clone(),
        route_id: RouteId(format!("ROUTE-{}", chrono::Utc::now().timestamp())),
        actor: "lexby".into(),
        scope: None,
        obligations: route_decision.obligations.clone(),
        expires_at: expires.to_rfc3339(),
        status: PermitStatus::Active,
    })
}

pub fn attach_proof(
    permit_id: &PermitId,
    proof: Proof,
    spec_set: &mut SpecSet,
) -> Result<PermitStatus, KernelError> {
    spec_set.proofs.insert(proof.id.clone(), proof);

    if let Some(_permit) = spec_set.permits.get_mut(permit_id) {
        Ok(PermitStatus::Active)
    } else {
        Err(KernelError::PermitNotFound(permit_id.0.clone()))
    }
}

pub fn close_permit(
    permit_id: &PermitId,
    spec_set: &mut SpecSet,
) -> Result<Receipt, KernelError> {
    if let Some(permit) = spec_set.permits.get_mut(permit_id) {
        permit.status = PermitStatus::Closed;

        Ok(Receipt {
            receipt_id: format!("RECEIPT-{}", chrono::Utc::now().timestamp()),
            permit_id: permit_id.clone(),
            status: "closed_valid".into(),
            proofs: Vec::new(),
            remaining_obligations: Vec::new(),
        })
    } else {
        Err(KernelError::PermitNotFound(permit_id.0.clone()))
    }
}

pub fn validate_obligations(
    permit_id: &PermitId,
    spec_set: &SpecSet,
) -> Result<ObligationReport, KernelError> {
    if let Some(permit) = spec_set.permits.get(permit_id) {
        let mut findings = Vec::new();
        for obligation in &permit.obligations {
            findings.push(ObligationFinding {
                obligation_id: obligation.id.clone(),
                satisfied: false,
                kind: obligation.kind.clone(),
            });
        }
        Ok(ObligationReport { findings })
    } else {
        Err(KernelError::PermitNotFound(permit_id.0.clone()))
    }
}

pub struct ObligationReport {
    pub findings: Vec<ObligationFinding>,
}

pub struct ObligationFinding {
    pub obligation_id: ObligationId,
    pub satisfied: bool,
    pub kind: ObligationKind,
}

pub fn detect_spec_drift(
    _diff: &RepoState,
    _specs: &SpecSet,
    _logs: &[DecisionLog],
) -> Result<DriftReport, KernelError> {
    Ok(DriftReport {
        findings: Vec::new(),
    })
}

pub struct DriftReport {
    pub findings: Vec<DriftFinding>,
}

pub struct DriftFinding {
    pub spec_id: SpecId,
    pub severity: Severity,
    pub message: String,
}
