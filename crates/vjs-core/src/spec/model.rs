//! The spec data model: the authority records (Invariant, Decision, Permit, Proof, Receipt,
//! Session, SpecSet) plus the staged-diff / lawpack fact carriers (RepoState, DependencyChange,
//! LawpackFacts) that the invariant evaluator reads. Plain data + the permit intent digest.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use crate::court::*;
use crate::types::*;

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
    /// [2026] VJS-PC 16 D3: a permit is an AGENT-ROUTED SELF-ISSUE - it records that the
    /// actor took the front door, NOT that an external authority approved. A check
    /// reserved to the Sovereign (assent, now resolution-gated) or to a constituted bench
    /// (an order) is never satisfiable by a self-issued permit. Defaults true for the
    /// local-sovereign invocation (ACT-007; the kernel is clerk, not court).
    #[serde(default = "default_self_issued")]
    pub self_issued: bool,
    /// The plain-terms meaning recorded on the permit so the audit trail never reads as
    /// more than it is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meaning: Option<String>,
    /// A content/identity digest binding the permit to the actor + route + concrete scope
    /// it claims, so it is non-repudiable and cannot be silently reused to cover a
    /// different write than the one it was minted for (PC-16 D3).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intent_digest: Option<String>,
}

fn default_self_issued() -> bool {
    true
}

/// The recorded meaning of a self-issued permit (PC-16 D3): an agent-routed self-issue,
/// never an authority approval.
pub const SELF_ISSUED_MEANING: &str = "agent-routed self-issue: records that the actor took the front door (ACT-007 \
     local-sovereign invocation); NOT an external authority's approval. It cannot satisfy \
     a check reserved to the Sovereign (assent) or to a constituted bench (an order).";

/// The non-repudiable binding digest over the permit's actor + route + declared scope.
pub fn permit_intent_digest(actor: &str, route_id: &str, scope: &Option<Scope>) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(actor.as_bytes());
    hasher.update(b"\n");
    hasher.update(route_id.as_bytes());
    hasher.update(b"\n");
    let scope_repr = scope
        .as_ref()
        .and_then(|s| s.paths.as_ref())
        .map(|p| p.join(","))
        .unwrap_or_default();
    hasher.update(scope_repr.as_bytes());
    format!("sha256:{}", hex::encode(hasher.finalize()))
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

impl Default for SpecSet {
    fn default() -> Self {
        Self::new()
    }
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

/// Facts about the WHOLE lawpack, computed once (by the lawpack crate) and
/// handed to the staged-only invariant evaluator. `RepoState` holds only the
/// staged diff; predicates that reason about the entire authority graph
/// (validation, duplicate ids/citations, graph membership) read these.
#[derive(Clone, Debug)]
pub struct LawpackFacts {
    pub validates: bool,
    pub duplicate_ids: bool,
    pub duplicate_citations: bool,
    pub all_ids: HashSet<String>,
    pub mcp_local_first: bool,
    pub directory_roles_resolve: bool,
}

impl Default for LawpackFacts {
    /// Permissive default for callers/tests that do not exercise the
    /// lawpack-wide predicates. Real call sites build facts from the lawpack.
    fn default() -> Self {
        LawpackFacts {
            validates: true,
            duplicate_ids: false,
            duplicate_citations: false,
            all_ids: HashSet::new(),
            mcp_local_first: true,
            directory_roles_resolve: true,
        }
    }
}
