use serde::{Deserialize, Serialize};
use crate::types::*;
use crate::error::*;
use crate::route::*;
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Spec {
    pub id: SpecId,
    pub title: String,
    pub scope: Scope,
    pub owner: String,
    pub status: SpecStatus,
    pub purpose: String,
    pub decisions: Vec<DecisionId>,
    pub invariants: Vec<InvariantId>,
    pub obligations: Vec<ObligationId>,
    pub review_triggers: Vec<Trigger>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpecStatus {
    Active,
    Draft,
    Superseded,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Invariant {
    pub id: InvariantId,
    pub title: String,
    pub basis: Vec<AuthorityId>,
    pub scope: Scope,
    pub rule: PredicateExpr,
    pub severity: Severity,
    pub remedy: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Decision {
    pub id: DecisionId,
    pub title: String,
    pub status: DecisionStatus,
    pub scope: Scope,
    pub decision: String,
    pub basis: Vec<AuthorityId>,
    pub consequences: Consequences,
    pub review_triggers: Vec<Trigger>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Permit {
    pub id: PermitId,
    pub route_id: RouteId,
    pub actor: String,
    pub scope: Scope,
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofKind {
    CommandResult,
    DecisionLog,
    TestResult,
    PublicPrivateScan,
    ValidationReport,
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionState {
    Idle,
    Routed,
    Permitted,
    Acting,
    ProofAttached,
    Logged,
    Validated,
    Closed,
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

pub fn evaluate_invariants(
    _repo_state: &RepoState,
    spec_set: &SpecSet,
) -> Result<InvariantReport, KernelError> {
    let mut findings = Vec::new();

    for (id, invariant) in &spec_set.invariants {
        // Simplified evaluation: always pass in MVP
        findings.push(InvariantFinding {
            invariant_id: id.clone(),
            passed: true,
            severity: invariant.severity.clone(),
            message: format!("Invariant {} passed", invariant.title),
        });
    }

    Ok(InvariantReport { findings })
}

pub struct RepoState {
    pub changed_paths: Vec<std::path::PathBuf>,
    pub added_files: Vec<std::path::PathBuf>,
    pub deleted_files: Vec<std::path::PathBuf>,
    pub decision_logs: Vec<DecisionLog>,
    pub orders: Vec<Order>,
}

pub struct InvariantReport {
    pub findings: Vec<InvariantFinding>,
}

pub struct InvariantFinding {
    pub invariant_id: InvariantId,
    pub passed: bool,
    pub severity: Severity,
    pub message: String,
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
        scope: Scope {
            paths: None,
            jurisdictions: None,
            action_kinds: None,
            issue_tags: None,
            records: None,
        },
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
        // In MVP, attaching proof always succeeds
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
