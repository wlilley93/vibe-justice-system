use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

mod ids;
pub use ids::*;
mod predicate;
pub use predicate::*;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityStatus {
    Draft,
    Proposed,
    Binding,
    InForce,
    Stayed,
    Superseded,
    Overruled,
    Revoked,
    Spent,
    Void,
}

impl AuthorityStatus {
    pub fn is_live(&self) -> bool {
        matches!(
            self,
            AuthorityStatus::Binding | AuthorityStatus::InForce | AuthorityStatus::Proposed
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Court {
    County,
    /// The intermediate appellate tier. Persists in law (s.10; [2026] VJS-SC 2 D4) and is now
    /// representable at the canonical seat so vjs can convene and record a Court of Appeal order
    /// ([2026] VJS-PC 19). Serialises as `court_of_appeal`.
    CourtOfAppeal,
    PrivyCouncil,
    SupremeCourt,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionKind {
    ImplementationDecision,
    PublicRecordChange,
    PrivateRecordChange,
    ExternalAct,
    ReleaseOrPush,
    SecuritySensitiveAct,
    DataBoundaryDecision,
    CourtFiling,
    LegislativeDraft,
    Refactor,
    TrivialPreference,
    DependencyChange,
    SchemaChange,
    GovernedLoadBearingAct,
}

/// [2026] VJS-PC 15 D4: a generic subject-matter partition - the law that governs
/// CODE changes versus the law that governs RUNTIME operations. ADDITIVE machinery
/// under ACT-CONSOLIDATION-FRAMEWORK:s7: it sorts subject matter only and confers no
/// new court, no new apex, and no new assent path. Canon ships the CATEGORY frame
/// EMPTY; the concrete runtime acts a subscriber governs are supplied in its Tier-2,
/// never canon-enumerated. A runtime fork still routes to court (ACT-002:s6) and a
/// runtime act on an assented record stays under VJS-ACT 10.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LawDomain {
    CodeGovernance,
    RuntimeOperations,
}

impl LawDomain {
    /// The domain an action falls in. The one runtime-act category
    /// (`GovernedLoadBearingAct`) sorts to RuntimeOperations; every code-governance
    /// kind sorts to CodeGovernance. A partition, not a new authority.
    pub fn of(kind: &ActionKind) -> LawDomain {
        match kind {
            ActionKind::GovernedLoadBearingAct => LawDomain::RuntimeOperations,
            _ => LawDomain::CodeGovernance,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RouteInput {
    pub repo_root: Option<PathBuf>,
    pub jurisdiction: Option<JurisdictionId>,
    pub actor: String,
    pub action_kind: ActionKind,
    pub issue_tags: Vec<IssueTag>,
    pub intent: String,
    pub affected_paths: Vec<PathBuf>,
    pub risk: RiskLevel,
    pub public_target: bool,
    pub external_target: bool,
    pub irreversible: bool,
    pub user_instruction: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RouteDecision {
    pub decision: RouteOutcome,
    pub jurisdiction: JurisdictionId,
    pub court_required: bool,
    pub court: Option<Court>,
    pub court_trigger: Option<CourtTrigger>,
    pub log_required: bool,
    pub binding: Vec<AuthorityPointer>,
    pub must_do: Vec<String>,
    pub must_not_do: Vec<String>,
    pub warnings: Vec<String>,
    pub max_context: ContextBudget,
    pub summary: String,
    pub obligations: Vec<Obligation>,
    pub permit_id: Option<PermitId>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteOutcome {
    Allowed,
    AllowedWithConditions,
    Blocked,
    CourtRequired,
    HumanApprovalRequired,
    ReleaseWarrantRequired,
    PrivateBoundaryRequired,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CourtTrigger {
    FirstImpression,
    Distinction,
    Overruling,
    Conflict,
    Breach,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthorityPointer {
    pub id: AuthorityId,
    pub title: String,
    pub rank: AuthorityRank,
    pub status: AuthorityStatus,
    pub summary: String,
    pub source_path: Option<PathBuf>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthoritySet {
    pub authorities: Vec<AuthorityPointer>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContextBudget {
    pub summary_words: usize,
    pub records_returned: usize,
}

impl Default for ContextBudget {
    fn default() -> Self {
        Self {
            summary_words: 120,
            records_returned: 3,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Info,
    Warning,
    Error,
    Fatal,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermitStatus {
    Active,
    Expired,
    Closed,
    Revoked,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofStatus {
    Pending,
    Passed,
    Failed,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionStatus {
    Active,
    Superseded,
    Revoked,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordClass {
    PublicSystemData,
    PublicRedactedSummary,
    PrivateLocalEvidence,
    PrivateOperationalFact,
    Secret,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Scope {
    pub paths: Option<Vec<String>>,
    pub jurisdictions: Option<Vec<JurisdictionId>>,
    pub action_kinds: Option<Vec<ActionKind>>,
    pub issue_tags: Option<Vec<IssueTag>>,
    pub records: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Consequences {
    pub must: Vec<String>,
    pub must_not: Vec<String>,
    pub review_triggers: Option<Vec<Trigger>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Trigger {
    DependencyAdded,
    NetworkCapabilityAdded,
    ModelCallAdded,
    AuthorityRankingChanged,
    PublicRecordChanged,
    PrivateMarkerDetected,
    SchemaChanged,
    ProofRequired,
    PermitBypassed,
    ProofMissing,
    LogMissing,
    KernelExternalCapabilityRequired,
    KernelModelCapabilityRequired,
    V1ArchiveRequestedAsRuntime,
    OpinionClaimedAsBinding,
    AppealVolumeHigh,
    McpAuthorityClaimed,
    PathBrittlenessDetected,
    SemanticSearchProposedForAuthority,
    AuthorityBasisMissing,
    DraftTreatedAsBinding,
    MarkdownProposedAsCanonical,
    V1ConstitutionalRouteComplete,
    SupremeCourtSettlementReceived,
    SovereignAssentGranted,
    GazetteEntryPublished,
    HookExceededWordLimit,
    PromptPatchedWithoutEval,
    AgentHarnessChangedWithoutEval,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuleAtom {
    pub id: AuthorityId,
    pub title: String,
    pub status: AuthorityStatus,
    pub rank: AuthorityRank,
    pub scope: Scope,
    pub trigger: Option<PredicateExpr>,
    pub effect: Effect,
    pub exceptions: Option<Vec<String>>,
    pub summary: String,
    pub source: Option<HashMap<String, Vec<String>>>,
    pub supersedes: Vec<AuthorityId>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Effect {
    pub must: Option<Vec<String>>,
    pub must_not: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub court: Court,
    pub jurisdiction: JurisdictionId,
    pub repo_code: Option<RepoCode>,
    pub status: AuthorityStatus,
    pub issue: IssueTag,
    pub holding: String,
    pub directives: Vec<Directive>,
    pub forbidden: Option<Vec<String>>,
    pub exceptions: Option<Vec<String>>,
    pub supersedes: Vec<AuthorityId>,
    pub source_opinion: Option<PathBuf>,
    pub runtime_summary: String,
    pub created_at: String,
    // Promote the citation and assent source the orders already carry (they
    // were silently dropped on load). Optional so the existing orders are valid.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub citation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assent_source: Option<String>,
    // Auditable court record: the bench that decided, the sha256 of the
    // symmetric case file, when it convened, the vote, and the appeal links.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bench: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub case_file_digest: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub convened_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vote: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub appeal_of: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub appealable: Option<bool>,
    /// [2026] VJS-PC 17 D7: an OPTIONAL machine-resolvable list of the authorities this
    /// order's operative parts rely on (canonical ids / citations), mirroring `supersedes`.
    /// Directive bodies are presently lossy snake_case tokens no clerk can resolve
    /// (act_010_s2 does not mechanically resolve to ACT-ASSENTED-RECORD-PROTECTION:s2), so
    /// an author lists the directives' load-bearing authorities here and the
    /// citation-grounding teeth extend to them. Prose stays for humans.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cites_authorities: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Directive {
    pub id: String,
    pub actor: String,
    pub must: String,
    pub when: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecisionLog {
    pub id: String,
    pub time: String,
    pub actor: String,
    pub kind: String,
    pub issue: String,
    pub decision: String,
    pub basis: Vec<String>,
    pub risk: RiskLevel,
    pub reversibility: String,
    pub court_required: bool,
    pub why: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityRank {
    Constitutional,
    Primary,
    Regulation,
    SupremeCourt,
    PrivyCouncil,
    CourtOfAppeal,
    CountyCourt,
    Log,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Obligation {
    pub id: ObligationId,
    pub kind: ObligationKind,
    pub required: bool,
    pub due: ObligationDue,
    pub description: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObligationKind {
    DecisionLog,
    Command,
    PublicPrivateScan,
    Validation,
    Proof,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObligationDue {
    BeforeCommit,
    BeforeClose,
    AfterAction,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpecStatus {
    Active,
    Draft,
    Superseded,
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
