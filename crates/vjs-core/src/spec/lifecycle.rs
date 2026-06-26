//! Permit lifecycle verbs: open a self-issued permit, attach a proof, close a permit, validate a
//! permit's obligations, and the (placeholder) spec-drift detector. The report/finding structs
//! these verbs return live alongside them.

use super::model::{
    Permit, Proof, Receipt, RepoState, SELF_ISSUED_MEANING, SpecSet, permit_intent_digest,
};
use crate::error::*;
use crate::types::*;

pub fn open_permit(route_decision: &RouteDecision, _actor: &str) -> Result<Permit, KernelError> {
    // Millisecond precision (#15): two permits routed in the same second no longer
    // share an id, which a seconds-precision stamp allowed.
    let id = PermitId(format!("PERMIT-{}", chrono::Utc::now().timestamp_millis()));
    let expires = chrono::Utc::now() + chrono::Duration::hours(2);

    let route_id = format!("ROUTE-{}", chrono::Utc::now().timestamp());
    Ok(Permit {
        id: id.clone(),
        route_id: RouteId(route_id.clone()),
        actor: "lexby".into(),
        scope: None,
        obligations: route_decision.obligations.clone(),
        expires_at: expires.to_rfc3339(),
        status: PermitStatus::Active,
        self_issued: true,
        meaning: Some(SELF_ISSUED_MEANING.into()),
        intent_digest: Some(permit_intent_digest("lexby", &route_id, &None)),
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

pub fn close_permit(permit_id: &PermitId, spec_set: &mut SpecSet) -> Result<Receipt, KernelError> {
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
    logs: &[DecisionLog],
) -> Result<ObligationReport, KernelError> {
    if let Some(permit) = spec_set.permits.get(permit_id) {
        let permit_proofs: Vec<&Proof> = spec_set
            .proofs
            .values()
            .filter(|p| p.permit_id == *permit_id && p.status == ProofStatus::Passed)
            .collect();

        let mut findings = Vec::new();
        for obligation in &permit.obligations {
            // The same satisfaction rules the permit gate enforces at commit:
            // a decision log must cite the permit; everything else is carried
            // by a passed proof of the corresponding kind.
            let satisfied = match obligation.kind {
                ObligationKind::DecisionLog => logs.iter().any(|log| {
                    log.id.contains(&permit_id.0)
                        || log.basis.iter().any(|b| b == &permit_id.0)
                        || log.issue.contains(&permit_id.0)
                }),
                ObligationKind::Proof => !permit_proofs.is_empty(),
                ObligationKind::Validation => permit_proofs
                    .iter()
                    .any(|p| p.kind == ProofKind::ValidationReport),
                ObligationKind::PublicPrivateScan => permit_proofs
                    .iter()
                    .any(|p| p.kind == ProofKind::PublicPrivateScan),
                ObligationKind::Command => permit_proofs
                    .iter()
                    .any(|p| p.kind == ProofKind::CommandResult),
            };
            findings.push(ObligationFinding {
                obligation_id: obligation.id.clone(),
                satisfied,
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
