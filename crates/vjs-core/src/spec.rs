use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::court::*;
use crate::error::*;
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

pub fn evaluate_invariants(
    repo_state: &RepoState,
    invariants: &[Invariant],
    facts: &LawpackFacts,
) -> Result<InvariantReport, KernelError> {
    let mut findings = Vec::new();

    for invariant in invariants {
        // Honor the declared scope: an invariant scoped to a set of paths only
        // applies when the staged change touches one of them. A scoped invariant
        // with no in-scope change is vacuously satisfied. (Without this gate the
        // rule would run against every staged file regardless of its declared
        // scope, which is both incorrect and the source of cross-record false
        // positives.) Unscoped invariants always evaluate.
        let in_scope = match &invariant.scope {
            Some(scope) => match &scope.paths {
                Some(paths) if !paths.is_empty() => paths.iter().any(|glob| {
                    repo_state
                        .changed_paths
                        .iter()
                        .any(|p| glob_matches(glob, p))
                }),
                _ => true,
            },
            None => true,
        };
        let result = if in_scope {
            evaluate_predicate(&invariant.rule, repo_state, invariant.scope.as_ref(), facts)
        } else {
            true
        };
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

/// A scoped invariant's content predicates examine only the files within its
/// declared scope. Without this, a content match (field_equals, string_contains)
/// would scan every staged file and false-positive on unrelated records that
/// merely quote the pattern in prose. An unscoped invariant scans everything.
fn scope_allows(scope: Option<&Scope>, path: &std::path::Path) -> bool {
    match scope.and_then(|s| s.paths.as_ref()) {
        Some(paths) if !paths.is_empty() => paths.iter().any(|g| glob_matches(g, path)),
        _ => true,
    }
}

/// A path that, by its location, claims binding runtime force (statute,
/// regulation, rule, or order). One definition so every witness agrees.
fn claims_runtime_force(path: &Path) -> bool {
    let p = path.to_string_lossy();
    p.contains("lawpack/v2/statutes/")
        || p.contains("lawpack/v2/regulations/")
        || p.contains("lawpack/v2/rules/")
        || p.contains("lawpack/v2/orders/")
}

/// Read a record's OPERATIVE top-level field by parsing its structure, never by
/// scanning raw lines. Returns the value only when `key` is a top-level mapping
/// entry whose value is a scalar string. A parse failure, a non-mapping document,
/// a missing key, or a non-string (e.g. an empty/null) value yields None - so a
/// runtime-force record that merely MENTIONS the field in prose (a section text
/// block, a quoted example) does not DECLARE it, and fails closed. Deterministic:
/// serde_yaml parse only - no model call, no network, no clock (DEC-KERNEL-001).
fn top_level_scalar(content: &str, key: &str) -> Option<String> {
    let value: serde_yaml::Value = serde_yaml::from_str(content).ok()?;
    value.get(key)?.as_str().map(|s| s.to_string())
}

fn evaluate_predicate(
    rule: &PredicateExpr,
    repo_state: &RepoState,
    scope: Option<&Scope>,
    facts: &LawpackFacts,
) -> bool {
    match rule {
        PredicateExpr::All { items } => items
            .iter()
            .all(|item| evaluate_predicate(item, repo_state, scope, facts)),
        PredicateExpr::Any { items } => items
            .iter()
            .any(|item| evaluate_predicate(item, repo_state, scope, facts)),
        PredicateExpr::None { items } => items
            .iter()
            .all(|item| !evaluate_predicate(item, repo_state, scope, facts)),
        PredicateExpr::Not { item } => !evaluate_predicate(item, repo_state, scope, facts),
        PredicateExpr::If { condition, then } => {
            if evaluate_predicate(condition, repo_state, scope, facts) {
                evaluate_predicate(then, repo_state, scope, facts)
            } else {
                true // if condition is false, the implication is vacuously true
            }
        }
        PredicateExpr::PathChanged { glob } => repo_state
            .changed_paths
            .iter()
            .any(|p| glob_matches(glob, p)),
        PredicateExpr::FileAdded { pattern } => repo_state
            .added_files
            .iter()
            .any(|p| glob_matches(pattern, p)),
        PredicateExpr::FileModified { pattern } => repo_state
            .modified_files
            .iter()
            .any(|p| glob_matches(pattern, p)),
        PredicateExpr::FileDeleted { pattern } => repo_state
            .deleted_files
            .iter()
            .any(|p| glob_matches(pattern, p)),
        PredicateExpr::StringContains { value } => repo_state
            .file_contents
            .iter()
            .filter(|(p, _)| scope_allows(scope, p))
            .any(|(_, content)| content.contains(value)),
        PredicateExpr::ImportContains { value } => repo_state
            .file_contents
            .iter()
            .filter(|(p, _)| scope_allows(scope, p))
            .any(|(_, content)| content.contains(value)),
        PredicateExpr::DependencyAdded { name } => repo_state
            .dependency_changes
            .iter()
            .any(|c| c.name == *name && c.added),
        PredicateExpr::DependencyRemoved { name } => repo_state
            .dependency_changes
            .iter()
            .any(|c| c.name == *name && c.removed),
        PredicateExpr::DecisionLogExists { issue: _ } => !repo_state.logs.is_empty(),
        PredicateExpr::PermitExists { id: _ } => !repo_state.permits.is_empty(),
        PredicateExpr::ProofExists { kind: _ } => !repo_state.proofs.is_empty(),
        PredicateExpr::OrderExists { issue: _ } => !repo_state.orders.is_empty(),
        PredicateExpr::WordCountLte { field: _, max: _ } => {
            // Simplified: always true for now
            true
        }
        PredicateExpr::FileWordsLte { glob, max } => {
            // Deterministic: every file in scope (matched by glob, among the
            // changed/loaded contents) must hold no more than `max` whitespace
            // separated words. This is the real enforcement behind the
            // "hooks stay short" law - a state check, not a prompt instruction.
            repo_state.file_contents.iter().all(|(path, content)| {
                if !glob_matches(glob, path) {
                    return true;
                }
                content.split_whitespace().count() <= *max
            })
        }
        PredicateExpr::CitationUnique => {
            // Simplified: always true for now
            true
        }
        PredicateExpr::RequiredFields { fields } => {
            // Every staged in-scope record must declare each required field
            // (e.g. a new law record must carry authority/status/kernel_effect).
            // A YAML key is `name:` at the start of a (possibly indented) line.
            repo_state
                .file_contents
                .iter()
                .filter(|(p, _)| scope_allows(scope, p))
                .all(|(_, content)| {
                    fields.iter().all(|f| {
                        let key = format!("{}:", f);
                        content.lines().any(|l| l.trim_start().starts_with(&key))
                    })
                })
        }
        PredicateExpr::FieldEquals { field, value } => {
            // Only the files within the invariant's scope: a field check on a
            // statute invariant must not trip on a draft spec or a narrative
            // doc that merely contains "status: draft" in prose.
            let pattern = format!("{}: {}", field, value);
            repo_state
                .file_contents
                .iter()
                .filter(|(p, _)| scope_allows(scope, p))
                .any(|(_, content)| content.contains(&pattern))
        }
        PredicateExpr::IncludedInRuntimeAuthorityGraph => {
            // A staged in-scope record whose declared id resolves in the whole
            // lawpack is part of the runtime authority graph. INV-LAWMAKING-002
            // wraps this as `if status==draft then not(included)`, so a draft
            // record sitting in a runtime-authority path (its id already in the
            // graph) fails - draft law is not binding.
            repo_state
                .file_contents
                .iter()
                .filter(|(p, _)| scope_allows(scope, p))
                .any(|(_, content)| {
                    top_level_scalar(content, "id")
                        .map(|id| facts.all_ids.contains(id.trim()))
                        .unwrap_or(false)
                })
        }
        PredicateExpr::PublicNoPrivateFacts => repo_state.boundary_findings.is_empty(),
        PredicateExpr::CoreNoModelCalls => {
            // SECONDARY defense-in-depth only. The AUTHORITATIVE model-free witness
            // is the dependency closure, fenced by deny.toml (`cargo deny check
            // bans`): a substring scan of source can be obfuscated, and cannot scan
            // the evaluator file (it holds these very patterns as string literals),
            // so it can never be the real guarantee. With no model/HTTP crate in the
            // closure, a model call here is impossible, not merely unspelt.
            //
            // Check if vjs-core source files contain actual model API imports or calls
            // Exclude the invariant evaluator itself (spec.rs) and test files
            repo_state.file_contents.iter().all(|(path, content)| {
                if !path.to_string_lossy().contains("vjs-core") {
                    return true;
                }
                // Skip the evaluator file and test files
                let path_str = path.to_string_lossy();
                if path_str.contains("spec.rs")
                    || path_str.contains("test")
                    || path_str.contains("golden")
                {
                    return true;
                }
                // Check for actual model API usage patterns
                let has_model_import = content.contains("use openai::")
                    || content.contains("use anthropic::")
                    || content.contains("openai::Client")
                    || content.contains("anthropic::Client")
                    || content.contains(".chat.completions")
                    || content.contains("/v1/messages");
                !has_model_import
            })
        }
        PredicateExpr::CoreNoNetwork => {
            // SECONDARY defense-in-depth, change-triggered. The AUTHORITATIVE
            // network-free witness is the dependency closure fenced by deny.toml
            // (`cargo deny check bans`), which checks the whole graph regardless of
            // what this staged diff happens to touch. This list flags an obvious
            // network crate the moment it is added to the kernel.
            const NET_CRATES: [&str; 8] = [
                "reqwest",
                "hyper",
                "hyper-util",
                "ureq",
                "curl",
                "isahc",
                "surf",
                "attohttpc",
            ];
            repo_state.dependency_changes.iter().all(|c| {
                if !repo_state
                    .changed_paths
                    .iter()
                    .any(|p| p.to_string_lossy().contains("vjs-core"))
                {
                    return true;
                }
                !NET_CRATES.contains(&c.name.as_str())
            })
        }
        PredicateExpr::GovernedWritesRequirePermit => {
            // Presence is not coverage: require at least one ACTIVE permit, not
            // merely a permit (a Closed/Revoked/Expired permit no longer excuses a
            // governed write). Per-path scope and time-of-day expiry are the clock's
            // business and are enforced at the pre-write hook (PermitGate::covers,
            // which holds the clock); the invariant evaluator stays deterministic
            // (no now()), so it witnesses status, the part it can check purely.
            repo_state
                .permits
                .iter()
                .any(|p| matches!(p.status, PermitStatus::Active))
        }
        PredicateExpr::ProofsExistBeforeClose => {
            // Presence is not proof: require a proof that actually PASSED. A Pending
            // or Failed proof does not discharge the close obligation.
            repo_state
                .proofs
                .iter()
                .any(|p| matches!(p.status, ProofStatus::Passed))
        }
        PredicateExpr::LogsStayShort => repo_state
            .logs
            .iter()
            .all(|log| log.why.split_whitespace().count() <= 150),
        PredicateExpr::LawpackValidates => facts.validates,
        PredicateExpr::NoDuplicateIds => !facts.duplicate_ids,
        PredicateExpr::NoDuplicateCitations => !facts.duplicate_citations,
        PredicateExpr::OrdersHaveDirectives => repo_state
            .orders
            .iter()
            .all(|order| !order.directives.is_empty()),
        PredicateExpr::McpLocalFirst => facts.mcp_local_first,
        PredicateExpr::DirectoryRolesResolve => facts.directory_roles_resolve,
        PredicateExpr::V1NotLoadedByDefault => {
            // Deterministic: a runtime authority record (statute, regulation,
            // rule, or order) must not drag V1 in as binding law on a V2
            // silence. A staged file under those dirs that cites a V1 central
            // authority (REALM-SC/PC/CA/SI) is a violation UNLESS it carries an
            // express incorporation clause. V1 stays persuasive archive; it
            // binds V2 only by incorporation. Other files (provenance, docs,
            // decisions citing V1 as evidence) are out of scope and pass.
            const V1_MARKERS: [&str; 4] = ["REALM-SC", "REALM-PC", "REALM-CA", "REALM-SI"];
            repo_state.file_contents.iter().all(|(path, content)| {
                if !claims_runtime_force(path) {
                    return true;
                }
                // A V1 authority binds V2 only by one of the two lawful routes
                // the Act recognises: express incorporation (s.8) or
                // constitutional carry-forward in continuity (s.19(4), e.g. the
                // court hierarchy). Either marker, present in the same runtime
                // record, satisfies the rule; a bare V1 citation with neither is
                // an unincorporated import and a violation.
                const CARRY_MARKERS: [&str; 5] = [
                    "incorporat",
                    "carried forward",
                    "carried into",
                    "carry-forward",
                    "carries forward",
                ];
                if CARRY_MARKERS.iter().any(|m| content.contains(m)) {
                    return true;
                }
                !V1_MARKERS.iter().any(|m| content.contains(m))
            })
        }
        PredicateExpr::AssentSourceValid { allowed } => {
            // CASE-LAW s. 23(5) ([2026] REALM-SC 10): a record that claims binding
            // runtime force carries it ONLY if it declares an `assent_source` whose
            // value resolves to one of the allowed forms (a specific Sovereign-assent
            // event, or a standing-bounded route tracing to specific assent).
            //
            // This is an AFFIRMATIVE ALLOW-LIST that FAILS CLOSED: absence of the
            // field, an empty value, an unrecognised form, or an unresolved trace each
            // cause rejection. It is NOT a deny-list: a record that merely omits the
            // field is rejected, never passed (the not-equal-to-self_authorised form is
            // void as fail-open, s. 23(5)). Deterministic: no model call, no similarity
            // search.
            //
            // The witness reads the OPERATIVE top-level `assent_source` field by
            // parsing structure. A raw-line scan read the document as a bag of lines,
            // so a value buried in a section's prose - or any line that happened to
            // trim to `assent_source: <allowed>` - pardoned the whole record, and an
            // explicitly-void operative `self_authorised` could be masked by a buried
            // valid line (confirmed admitted by the binary 2026-06-12). The operative
            // field, and only it, confers force.
            repo_state.file_contents.iter().all(|(path, content)| {
                if !claims_runtime_force(path) {
                    return true;
                }
                match top_level_scalar(content, "assent_source") {
                    Some(v) => allowed.iter().any(|a| *a == v),
                    None => false,
                }
            })
        }
    }
}

// One glob semantics for the whole kernel: the invariant evaluator must agree
// with the permit gate on what a glob covers (this copy had its own weaker,
// fail-open matching).
fn glob_matches(glob: &str, path: &Path) -> bool {
    crate::governance::PathClassifier::glob_matches(glob, &path.to_string_lossy())
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

pub fn open_permit(route_decision: &RouteDecision, _actor: &str) -> Result<Permit, KernelError> {
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
