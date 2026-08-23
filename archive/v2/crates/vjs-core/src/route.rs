use crate::KernelContext;
use crate::authority::*;
use crate::court::*;
use crate::error::*;
use crate::rank_value;
use crate::types::*;

pub fn route(input: RouteInput, ctx: &KernelContext) -> Result<RouteDecision, KernelError> {
    validate_input(&input)?;

    let jurisdiction = resolve_jurisdiction(&input, ctx)?;
    let authorities = resolve_authority(&input, &ctx.authority_graph)?;
    let conflicts: Vec<AuthorityConflict> = Vec::new();
    let boundary: Vec<BoundaryFinding> = Vec::new();

    let court_trigger = detect_court_trigger(&input, &authorities, &conflicts, &boundary);
    let court = court_trigger.as_ref().map(|t| choose_court(&input, t));

    let outcome = decide_route(&input, &authorities, &conflicts, &boundary, &court_trigger);

    let log_required = log_required(&input, &outcome);
    let must_do = build_must_do(&input, &outcome);
    let must_not_do = build_must_not_do(&input, &outcome);
    let summary = build_summary(&input, &outcome, &court_trigger);
    let obligations = build_obligations(&input, &outcome, &authorities);

    // A permit exists only where the route actually allows the work. A matter
    // sent to court must come back with a ruling, not walk off with a permit.
    let permit_id = if matches!(
        outcome,
        RouteOutcome::Allowed | RouteOutcome::AllowedWithConditions
    ) {
        Some(PermitId(format!(
            "PERMIT-{}",
            chrono::Utc::now().timestamp()
        )))
    } else {
        None
    };

    let mut binding: Vec<AuthorityPointer> = authorities
        .authorities
        .into_iter()
        .take(ctx.limits.route_max_authorities)
        .collect();

    binding.sort_by(|a, b| {
        let rank_a = rank_value(&a.rank);
        let rank_b = rank_value(&b.rank);
        rank_a.cmp(&rank_b)
    });

    Ok(RouteDecision {
        decision: outcome,
        jurisdiction,
        court_required: court_trigger.is_some(),
        court,
        court_trigger: court_trigger.clone(),
        log_required,
        binding,
        must_do,
        must_not_do,
        warnings: Vec::new(),
        max_context: ContextBudget::default(),
        summary,
        obligations,
        permit_id,
    })
}

fn validate_input(input: &RouteInput) -> Result<(), KernelError> {
    if input.intent.is_empty() {
        return Err(KernelError::InvalidInput("intent is required".into()));
    }
    Ok(())
}

fn resolve_jurisdiction(
    input: &RouteInput,
    _ctx: &KernelContext,
) -> Result<JurisdictionId, KernelError> {
    Ok(input
        .jurisdiction
        .clone()
        .unwrap_or_else(|| JurisdictionId("default".into())))
}

fn decide_route(
    input: &RouteInput,
    authorities: &AuthoritySet,
    _conflicts: &[AuthorityConflict],
    _boundary: &[BoundaryFinding],
    court_trigger: &Option<CourtTrigger>,
) -> RouteOutcome {
    if let Some(trigger) = court_trigger {
        match trigger {
            CourtTrigger::Breach => return RouteOutcome::CourtRequired,
            CourtTrigger::Conflict => return RouteOutcome::CourtRequired,
            CourtTrigger::FirstImpression => return RouteOutcome::CourtRequired,
            CourtTrigger::Distinction => return RouteOutcome::CourtRequired,
            CourtTrigger::Overruling => return RouteOutcome::CourtRequired,
        }
    }

    // AN IRREVERSIBLE ACT MAY NOT SELF-PERMIT (the 2026-08-05 sweep critical, cured
    // under WARRANT-CANON-001). Until this arm existed, `--irreversible` with no court
    // trigger fell through to Allowed and MINTED a self-issued permit; the only guard
    // was the prose must_not "act_without_human_checkpoint", and prose is not
    // enforcement. K-24 already states the doctrine (an irreversible outward action
    // blocks until granted); this makes the route the gate that holds it. The permit
    // for an irreversible act comes from the human grant, never from the actor's own
    // route (ACT-006:s4: human_approval_required, permit_with_human_approval).
    if input.irreversible {
        return RouteOutcome::HumanApprovalRequired;
    }

    if input.external_target || input.public_target {
        return RouteOutcome::AllowedWithConditions;
    }

    if authorities.authorities.is_empty() {
        return RouteOutcome::Allowed;
    }

    RouteOutcome::AllowedWithConditions
}

fn log_required(input: &RouteInput, outcome: &RouteOutcome) -> bool {
    match input.action_kind {
        ActionKind::ImplementationDecision
        | ActionKind::PublicRecordChange
        | ActionKind::ExternalAct
        | ActionKind::SecuritySensitiveAct
        | ActionKind::ReleaseOrPush
        // PC-15 D4: a governed load-bearing runtime act is always logged.
        | ActionKind::GovernedLoadBearingAct => true,
        _ => matches!(outcome, RouteOutcome::AllowedWithConditions),
    }
}

fn build_must_do(input: &RouteInput, outcome: &RouteOutcome) -> Vec<String> {
    let mut must = Vec::new();
    if *outcome == RouteOutcome::CourtRequired {
        // The lawful disposition of a fork is to convene, on the agent's own
        // motion, on a symmetric case file with no access to the agent's
        // preference (LEXBY-SC 3 s.17(b); the forks-go-to-the-court rule).
        must.push("convene_the_named_court_on_own_motion".into());
        must.push("file_symmetric_case_file_no_preference".into());
    }
    if log_required(input, outcome) {
        must.push("write_decision_log".into());
    }
    if input.public_target {
        must.push("run_public_private_scan".into());
    }
    if input.external_target {
        must.push("verify_release_authority".into());
    }
    must
}

fn build_must_not_do(input: &RouteInput, outcome: &RouteOutcome) -> Vec<String> {
    let mut must_not = Vec::new();
    if *outcome == RouteOutcome::CourtRequired {
        // A first-impression/breach/conflict fork is the court's, not the
        // Principal's: do not route it to the Principal, and do not improvise.
        must_not.push("route_the_fork_to_the_principal".into());
        must_not.push("ask_the_principal_to_choose_between_approaches".into());
        must_not.push("proceed_without_a_ruling".into());
    }
    if input.public_target {
        must_not.push("publish_private_facts".into());
    }
    if input.irreversible {
        must_not.push("act_without_human_checkpoint".into());
    }
    must_not
}

fn build_summary(
    input: &RouteInput,
    outcome: &RouteOutcome,
    trigger: &Option<CourtTrigger>,
) -> String {
    match outcome {
        RouteOutcome::Allowed => format!("Allowed: {}", input.intent),
        RouteOutcome::AllowedWithConditions => {
            format!("Allowed with conditions: {}", input.intent)
        }
        RouteOutcome::Blocked => format!("Blocked: {}", input.intent),
        RouteOutcome::CourtRequired => {
            if let Some(t) = trigger {
                format!("Court required ({:?}): {}", t, input.intent)
            } else {
                format!("Court required: {}", input.intent)
            }
        }
        RouteOutcome::HumanApprovalRequired => {
            format!("Human approval required: {}", input.intent)
        }
        RouteOutcome::ReleaseWarrantRequired => {
            format!("Release warrant required: {}", input.intent)
        }
        RouteOutcome::PrivateBoundaryRequired => {
            format!("Private boundary required: {}", input.intent)
        }
    }
}

fn build_obligations(
    input: &RouteInput,
    outcome: &RouteOutcome,
    _authorities: &AuthoritySet,
) -> Vec<Obligation> {
    let mut obligations = Vec::new();

    if log_required(input, outcome) {
        obligations.push(Obligation {
            id: ObligationId("OBL-LOG-001".into()),
            kind: ObligationKind::DecisionLog,
            required: true,
            due: ObligationDue::BeforeCommit,
            description: "Write a decision log for this material implementation decision".into(),
        });
    }

    if input.public_target {
        obligations.push(Obligation {
            id: ObligationId("OBL-BOUNDARY-001".into()),
            kind: ObligationKind::PublicPrivateScan,
            required: true,
            due: ObligationDue::BeforeCommit,
            description: "Run public/private boundary scan".into(),
        });
    }

    if matches!(outcome, RouteOutcome::AllowedWithConditions) {
        obligations.push(Obligation {
            id: ObligationId("OBL-VALIDATE-001".into()),
            kind: ObligationKind::Validation,
            required: true,
            due: ObligationDue::BeforeCommit,
            description: "Run vjs validate before commit".into(),
        });
    }

    obligations
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AuthorityGraph, ContextLimits, KernelContext};

    fn ctx() -> KernelContext {
        // One resolvable binding authority, so the matter is NOT first-impression
        // (any_on_point: no issue tags + a non-empty resolved set = on point). The
        // defect under test only bit on exactly this path: settled law on point, no
        // court trigger, and the irreversible act then minted its own permit.
        let mut graph = AuthorityGraph::default();
        let a = crate::Authority {
            id: crate::AuthorityId("ACT-TEST".into()),
            kind: crate::AuthorityKind::Statute,
            rank: crate::AuthorityRank::Primary,
            status: crate::types::AuthorityStatus::InForce,
            jurisdiction: None,
            title: "test statute".into(),
            summary: "governs the routine act".into(),
            source_path: None,
            issue_tags: Vec::new(),
            scope: None,
            supersedes: Vec::new(),
        };
        graph.authorities.insert(a.id.clone(), a);
        KernelContext {
            authority_graph: graph,
            limits: ContextLimits::default(),
            lawpack_digest: "sha256:test".into(),
        }
    }

    /// THE RED SEED for the self-permit cure: before the irreversible arm existed,
    /// this exact input came back Allowed WITH a minted permit - an irreversible act
    /// authorising itself. It must come back HumanApprovalRequired with NO permit.
    #[test]
    fn an_irreversible_act_cannot_mint_its_own_permit() {
        let input = RouteInput {
            repo_root: None,
            jurisdiction: None,
            actor: "lexby".into(),
            action_kind: ActionKind::ExternalAct,
            issue_tags: Vec::new(),
            intent: "delete the production data".into(),
            affected_paths: Vec::new(),
            // Low risk ON PURPOSE: high risk courts anyway. The defect's shape was the
            // untriggered path - a low-risk-labelled irreversible act walking off with
            // a self-issued permit; this seed pins exactly that path.
            risk: RiskLevel::Low,
            public_target: false,
            external_target: false,
            irreversible: true,
            user_instruction: None,
        };
        let d = route(input, &ctx()).expect("routes");
        assert_eq!(d.decision, RouteOutcome::HumanApprovalRequired);
        assert!(
            d.permit_id.is_none(),
            "an irreversible act must not walk away with a self-issued permit"
        );
    }

    /// The positive twin: the same input made reversible still routes normally and
    /// may mint - proving the cure keys on irreversibility, not on the action kind.
    #[test]
    fn the_same_act_made_reversible_still_routes() {
        let input = RouteInput {
            repo_root: None,
            jurisdiction: None,
            actor: "lexby".into(),
            action_kind: ActionKind::ExternalAct,
            issue_tags: Vec::new(),
            intent: "stage the deletion behind a reversible flag".into(),
            affected_paths: Vec::new(),
            risk: RiskLevel::Low,
            public_target: false,
            external_target: false,
            irreversible: false,
            user_instruction: None,
        };
        let d = route(input, &ctx()).expect("routes");
        assert!(matches!(
            d.decision,
            RouteOutcome::Allowed | RouteOutcome::AllowedWithConditions
        ));
        assert!(d.permit_id.is_some());
    }
}
