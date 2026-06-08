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

    if authorities.authorities.is_empty() && input.risk != RiskLevel::Low {
        return Some(CourtTrigger::FirstImpression);
    }

    if input.user_instruction.is_some() {
        // Check for potential conflicts
    }

    None
}

pub fn choose_court(input: &RouteInput, _trigger: &CourtTrigger) -> Court {
    if input.issue_tags.iter().any(|t| t.0.starts_with("constitutional.")) {
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
