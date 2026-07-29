//! Pins for the routing layer: jurisdiction-scoped authorities reject inputs
//! with no jurisdiction (the None-bypass), a court-required route mints no
//! permit, the proof_exists predicate carries its own proof_kind field, and
//! validate_obligations reports real satisfaction instead of hardcoded false.

use std::path::{Path, PathBuf};
use vjs_core::spec::{Permit, Proof, SpecSet, validate_obligations};
use vjs_core::*;
use vjs_lawpack::*;

fn build_kernel_context(repo: &Path) -> Result<KernelContext, vjs_core::error::KernelError> {
    let lawpack = LawpackLoader::load(&repo.join("lawpack/v2"))?;
    let graph = lawpack.build_authority_graph()?;
    Ok(KernelContext {
        authority_graph: graph,
        limits: ContextLimits::default(),
        lawpack_digest: "sha256:test".into(),
    })
}

fn route_input(risk: RiskLevel, jurisdiction: Option<&str>) -> RouteInput {
    RouteInput {
        repo_root: Some(PathBuf::from(".")),
        jurisdiction: jurisdiction.map(|j| JurisdictionId(j.into())),
        actor: "test".into(),
        action_kind: ActionKind::ImplementationDecision,
        issue_tags: vec![IssueTag("test".into())],
        intent: "Test route".into(),
        affected_paths: Vec::new(),
        risk,
        public_target: false,
        external_target: false,
        irreversible: false,
        user_instruction: None,
    }
}

// --- court-required routes mint no permit -------------------------------------

#[test]
fn a_court_required_route_walks_away_without_a_permit() {
    let repo = repo_root();
    let ctx = build_kernel_context(&repo).unwrap();
    // Critical risk plus an issue tag no authority is on point for forces the
    // FirstImpression trigger and a court_required outcome.
    let mut input = route_input(RiskLevel::Critical, Some("default"));
    input.issue_tags = vec![IssueTag("zz-no-authority-mentions-this-zz".into())];
    let decision = route(input, &ctx).unwrap();
    assert!(
        decision.court_required,
        "precondition: this route must go to court"
    );
    assert!(
        decision.permit_id.is_none(),
        "a matter routed to court must not walk off with a permit id"
    );
}

#[test]
fn an_allowed_route_mints_a_permit() {
    let repo = repo_root();
    let ctx = build_kernel_context(&repo).unwrap();
    let decision = route(route_input(RiskLevel::Low, Some("default")), &ctx).unwrap();
    assert!(!decision.court_required);
    assert!(decision.permit_id.is_some());
}

#[test]
fn a_granted_permit_carries_its_law_source() {
    // K-17 (the grant half): "every grant carries its law_source". A self-issued permit
    // minted by open_permit records the binding authorities the route grounded it in - not
    // merely its route_id - so a grant is auditable to the law it rests on, mirroring a
    // DecisionLog's basis. The goal-completion audit (2026-06-26) found this half
    // effectively unimplemented; this proves it end to end through the real route.
    let repo = repo_root();
    let ctx = build_kernel_context(&repo).unwrap();
    let decision = route(route_input(RiskLevel::Low, Some("default")), &ctx).unwrap();
    assert!(
        !decision.binding.is_empty(),
        "precondition: an allowed route over the real canon resolves binding authorities"
    );
    let permit = vjs_core::spec::open_permit(&decision, "lexby").unwrap();
    assert!(
        !permit.law_source.is_empty(),
        "a granted permit must carry its law_source (the binding authorities), not an empty list"
    );
    let expected: Vec<String> = decision.binding.iter().map(|a| a.id.0.clone()).collect();
    assert_eq!(
        permit.law_source, expected,
        "law_source must be exactly the route's binding authorities"
    );
}

// --- jurisdiction scope filtering ----------------------------------------------

fn scoped_authority(jurisdictions: Vec<&str>) -> Authority {
    Authority {
        id: AuthorityId("TEST-AUTH-001".into()),
        kind: AuthorityKind::Rule,
        rank: AuthorityRank::Primary,
        status: AuthorityStatus::Binding,
        jurisdiction: None,
        title: "Test authority".into(),
        summary: "test".into(),
        source_path: None,
        issue_tags: Vec::new(),
        scope: Some(Scope {
            paths: None,
            jurisdictions: Some(
                jurisdictions
                    .into_iter()
                    .map(|j| JurisdictionId(j.into()))
                    .collect(),
            ),
            action_kinds: None,
            issue_tags: None,
            records: None,
        }),
        supersedes: Vec::new(),
    }
}

fn resolve_with(authority: Authority, input: &RouteInput) -> usize {
    let mut graph = AuthorityGraph::new();
    graph.authorities.insert(authority.id.clone(), authority);
    authority::resolve_authority(input, &graph)
        .unwrap()
        .authorities
        .len()
}

#[test]
fn a_jurisdiction_scoped_authority_rejects_an_input_with_no_jurisdiction() {
    let count = resolve_with(
        scoped_authority(vec!["realm-x"]),
        &route_input(RiskLevel::Low, None),
    );
    assert_eq!(
        count, 0,
        "a scope naming jurisdictions is a restriction; jurisdiction: None must not slip past it"
    );
}

#[test]
fn a_wildcard_jurisdiction_scope_accepts_any_input() {
    assert_eq!(
        resolve_with(
            scoped_authority(vec!["*"]),
            &route_input(RiskLevel::Low, None)
        ),
        1
    );
    assert_eq!(
        resolve_with(
            scoped_authority(vec!["*"]),
            &route_input(RiskLevel::Low, Some("anything"))
        ),
        1
    );
}

#[test]
fn a_jurisdiction_scoped_authority_matches_only_its_jurisdiction() {
    assert_eq!(
        resolve_with(
            scoped_authority(vec!["realm-x"]),
            &route_input(RiskLevel::Low, Some("realm-x"))
        ),
        1
    );
    assert_eq!(
        resolve_with(
            scoped_authority(vec!["realm-x"]),
            &route_input(RiskLevel::Low, Some("realm-y"))
        ),
        0
    );
}

// --- proof_exists predicate field mapping ---------------------------------------

// --- a merely Proposed authority is pre-enactment, not live binding law ---------

fn authority_with_status(status: AuthorityStatus) -> Authority {
    Authority {
        id: AuthorityId("TEST-PROPOSED-001".into()),
        kind: AuthorityKind::Rule,
        rank: AuthorityRank::Primary,
        status,
        jurisdiction: None,
        title: "Test proposed authority".into(),
        summary: "test".into(),
        source_path: None,
        issue_tags: Vec::new(),
        scope: None,
        supersedes: Vec::new(),
    }
}

#[test]
fn a_proposed_authority_does_not_resolve_as_live_binding_law() {
    // ACT-001:s7: a merely PROPOSED authority is pre-enactment (counsel's draft) and
    // confers no binding force. It must not resolve into the AuthoritySet, or it would
    // leak into RouteDecision.binding and silence a FirstImpression court trigger.
    assert!(
        !AuthorityStatus::Proposed.is_live(),
        "a pre-enactment Proposed authority must not be live"
    );
    let input = route_input(RiskLevel::Low, Some("default"));
    // Control: the identical record marked Binding resolves.
    assert_eq!(
        resolve_with(authority_with_status(AuthorityStatus::Binding), &input),
        1,
        "control: a Binding authority resolves as live"
    );
    // The same record marked Proposed must be filtered out entirely.
    assert_eq!(
        resolve_with(authority_with_status(AuthorityStatus::Proposed), &input),
        0,
        "a merely Proposed (pre-enactment) authority must not resolve as live binding law"
    );
}

#[test]
fn proof_exists_takes_its_kind_from_proof_kind_not_id() {
    let raw = RawPredicate {
        kind: "proof_exists".into(),
        items: None,
        item: None,
        condition: None,
        then: None,
        glob: None,
        pattern: None,
        value: None,
        name: None,
        issue: None,
        id: Some("SOME-UNRELATED-ID".into()),
        field: None,
        max: None,
        fields: None,
        allowed: None,
        proof_kind: Some("test_result".into()),
    };
    match raw.to_predicate().unwrap() {
        PredicateExpr::ProofExists { kind } => {
            assert_eq!(
                kind.as_deref(),
                Some("test_result"),
                "kind must come from proof_kind, not id"
            );
        }
        other => panic!("expected ProofExists, got {:?}", other),
    }
}

// --- validate_obligations reports real satisfaction ------------------------------

fn permit_with(obligations: Vec<Obligation>) -> Permit {
    Permit {
        id: PermitId("PERMIT-OBL-TEST".into()),
        route_id: RouteId("ROUTE-TEST".into()),
        actor: "lexby".into(),
        scope: None,
        obligations,
        expires_at: "2099-01-01T00:00:00+00:00".into(),
        status: PermitStatus::Active,
        self_issued: true,
        meaning: None,
        intent_digest: None,
        law_source: Vec::new(),
    }
}

fn obligation(kind: ObligationKind) -> Obligation {
    Obligation {
        id: ObligationId("OBL-TEST".into()),
        kind,
        required: true,
        due: ObligationDue::BeforeCommit,
        description: "test".into(),
    }
}

#[test]
fn obligations_report_satisfied_once_their_evidence_exists() {
    let permit = permit_with(vec![
        obligation(ObligationKind::DecisionLog),
        obligation(ObligationKind::Proof),
    ]);
    let permit_id = permit.id.clone();

    let mut specs = SpecSet::new();
    specs.permits.insert(permit_id.clone(), permit);

    // No evidence yet: both unsatisfied.
    let report = validate_obligations(&permit_id, &specs, &[]).unwrap();
    assert!(report.findings.iter().all(|f| !f.satisfied));

    // A log citing the permit and a passed proof satisfy them.
    let log = DecisionLog {
        id: format!("LOG-{}", permit_id.0),
        time: "2026-06-09T00:00:00+00:00".into(),
        actor: "lexby".into(),
        kind: "decision".into(),
        issue: "test".into(),
        decision: "d".into(),
        basis: vec![permit_id.0.clone()],
        risk: RiskLevel::Low,
        reversibility: "reversible".into(),
        court_required: false,
        why: "w".into(),
    };
    let proof = Proof {
        id: ProofId("PROOF-OBL-TEST".into()),
        permit_id: permit_id.clone(),
        kind: ProofKind::TestResult,
        status: ProofStatus::Passed,
        digest: None,
        captured_at: "2026-06-09T00:00:00+00:00".into(),
    };
    specs.proofs.insert(proof.id.clone(), proof);

    let report = validate_obligations(&permit_id, &specs, &[log]).unwrap();
    assert!(
        report.findings.iter().all(|f| f.satisfied),
        "evidence present, yet findings: {:?}",
        report
            .findings
            .iter()
            .map(|f| (f.obligation_id.0.clone(), f.satisfied))
            .collect::<Vec<_>>()
    );
}

#[test]
fn a_failed_proof_does_not_satisfy_a_proof_obligation() {
    let permit = permit_with(vec![obligation(ObligationKind::Proof)]);
    let permit_id = permit.id.clone();
    let mut specs = SpecSet::new();
    specs.permits.insert(permit_id.clone(), permit);
    let proof = Proof {
        id: ProofId("PROOF-FAILED".into()),
        permit_id: permit_id.clone(),
        kind: ProofKind::TestResult,
        status: ProofStatus::Failed,
        digest: None,
        captured_at: "2026-06-09T00:00:00+00:00".into(),
    };
    specs.proofs.insert(proof.id.clone(), proof);

    let report = validate_obligations(&permit_id, &specs, &[]).unwrap();
    assert!(
        !report.findings[0].satisfied,
        "a failed proof is not performance"
    );
}

fn repo_root() -> PathBuf {
    // Anchor on the crate manifest: cargo runs integration tests from the
    // package dir, where "./lawpack/v2" resolves to nothing and every
    // lawpack-backed assertion would pass vacuously over an empty lawpack.
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

// --- the on-point matcher reads an order's OWN issue, and folds case/underscores ---

/// An authority is on point for the tag it declares about ITSELF.
///
/// `AuthorityPointer` dropped `issue_tags`, so `court::any_on_point` could only
/// see id/title/summary prose. Order ids are SCREAMING-HYPHEN while an order's
/// `issue:` field is lower_snake, so the tag an order states about itself was
/// exactly the tag that could not find it: a filed, binding, exactly-on-point
/// order was reported FirstImpression and the matter sent to a fresh court to
/// re-decide settled law - a ruling then given in ignorance of binding law,
/// which is per incuriam and void. Measured on boltrig 2026-07-29:
/// `operator_seat_host_boundary` returned court_required while `OPERATOR-SEAT`
/// returned allowed_with_conditions, for the same order.
#[test]
fn an_authority_is_on_point_for_its_own_declared_issue_whatever_its_case() {
    let repo = repo_root();
    let ctx = build_kernel_context(&repo).unwrap();

    // The tag as some authority in the real canon actually declares it.
    // Deterministic pick: sort by id so the chosen tag does not depend on hash order.
    let mut ids: Vec<_> = ctx.authority_graph.authorities.keys().cloned().collect();
    ids.sort_by(|a, b| a.0.cmp(&b.0));
    let declared: IssueTag = ids
        .iter()
        .find_map(|id| ctx.authority_graph.authorities[id].issue_tags.first().cloned())
        .expect("precondition: the canon has an authority declaring an issue tag");

    for variant in [
        declared.0.clone(),
        declared.0.to_uppercase(),
        declared.0.replace('_', "-"),
        declared.0.replace('-', "_"),
    ] {
        let mut input = route_input(RiskLevel::Medium, Some("default"));
        input.issue_tags = vec![IssueTag(variant.clone())];
        let decision = route(input, &ctx).unwrap();
        assert!(
            !decision.court_required,
            "'{variant}' is a case/separator variant of an authority's OWN declared \
             issue '{}', so the citator must find it rather than convene a court to \
             re-decide it",
            declared.0
        );
    }
}

/// The cure must not make the trigger unreachable: a tag no authority declares
/// or mentions still convenes. Without this the fix above could be satisfied by
/// a matcher that says yes to everything, which is the worse failure - it
/// suppresses a court that was owed.
#[test]
fn a_tag_no_authority_declares_still_convenes_a_court() {
    let repo = repo_root();
    let ctx = build_kernel_context(&repo).unwrap();
    let mut input = route_input(RiskLevel::Medium, Some("default"));
    input.issue_tags = vec![IssueTag("zz-no-authority-declares-or-mentions-this-zz".into())];
    let decision = route(input, &ctx).unwrap();
    assert!(
        decision.court_required,
        "a genuinely novel issue must still reach a court"
    );
}
