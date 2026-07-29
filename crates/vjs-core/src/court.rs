use crate::types::*;

pub fn detect_court_trigger(
    input: &RouteInput,
    authorities: &AuthoritySet,
    _conflicts: &[AuthorityConflict],
    _boundary: &[BoundaryFinding],
) -> Option<CourtTrigger> {
    if input.risk == RiskLevel::Critical && authorities.authorities.is_empty() {
        return Some(CourtTrigger::FirstImpression);
    }

    // On-point silence: a significant matter is first-impression when no resolved
    // authority is actually ON POINT for its issue - not only when the authority
    // set is literally empty. A loosely-related authority must not suppress the
    // duty to convene. "Significant" excludes a low-risk, reversible routine step
    // (which is a decisive call, not a court sitting) unless it is irreversible or
    // reaches outside the repo.
    let significant = input.risk != RiskLevel::Low
        || input.irreversible
        || input.public_target
        || input.external_target;
    if significant && !any_on_point(input, authorities) {
        return Some(CourtTrigger::FirstImpression);
    }

    if input.user_instruction.is_some() {
        // Check for potential conflicts
    }

    None
}

/// Deterministic on-point test: is any resolved authority actually on point for
/// the matter's issue? An empty set is never on point. When the matter carries
/// issue tags, an authority is on point only if its OWN declared issue tags, or
/// its id, title or summary, mention one of them - a loosely-related authority
/// returned by resolution does not count. When no issue tag is supplied, a
/// non-empty resolved set is treated as on point (the agent already has
/// authority to follow). No model, no search.
///
/// The authority's own `issue_tags` are consulted FIRST and are the honest
/// answer: they are what the order says it is about. Before they were carried
/// onto `AuthorityPointer` this function could only see id/title/summary, so a
/// filed, binding, exactly-on-point order was reported as first-impression and
/// the matter was sent to a fresh court to re-decide settled law - a ruling then
/// given in ignorance of binding law, which is per incuriam and void. Measured
/// twice on 2026-07-29: routing on `operator_seat_host_boundary` returned
/// court_required while `OPERATOR-SEAT` returned allowed_with_conditions, for
/// the same order. The failure runs both ways: a loose tag suppresses a court
/// that was owed.
fn any_on_point(input: &RouteInput, authorities: &AuthoritySet) -> bool {
    if authorities.authorities.is_empty() {
        return false;
    }
    if input.issue_tags.is_empty() {
        return true;
    }
    input.issue_tags.iter().any(|tag| {
        let needle = fold_tag(&tag.0);
        authorities.authorities.iter().any(|a| {
            a.issue_tags.iter().any(|t| fold_tag(&t.0) == needle)
                || fold_tag(&a.id.0).contains(&needle)
                || fold_tag(&a.title).contains(&needle)
                || fold_tag(&a.summary).contains(&needle)
        })
    })
}

pub fn choose_court(input: &RouteInput, _trigger: &CourtTrigger) -> Court {
    if input
        .issue_tags
        .iter()
        .any(|t| t.0.starts_with("constitutional."))
    {
        return Court::SupremeCourt;
    }

    if input.issue_tags.iter().any(|t| {
        t.0.starts_with("jurisdiction.")
            || t.0.starts_with("routing.")
            || t.0.starts_with("public_private.")
            || t.0.starts_with("federation.")
            || t.0.starts_with("local_sovereignty.")
    }) {
        return Court::PrivyCouncil;
    }

    Court::County
}

#[derive(Clone, Debug)]
pub struct AuthorityConflict {
    pub id1: AuthorityId,
    pub id2: AuthorityId,
    pub reason: String,
}

#[derive(Clone, Debug)]
pub struct BoundaryFinding {
    pub severity: Severity,
    pub path: Option<std::path::PathBuf>,
    pub kind: BoundaryFindingKind,
    pub message: String,
    pub suggested_route: BoundaryRoute,
}

#[derive(Clone, Debug)]
pub enum BoundaryFindingKind {
    Secret,
    PrivateHostname,
    PrivateRepoPath,
    Token,
    Email,
    UnredactedEvidence,
}

#[derive(Clone, Debug)]
pub enum BoundaryRoute {
    Redact,
    MoveToPrivate,
    Block,
    Warn,
}
