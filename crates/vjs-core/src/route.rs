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
