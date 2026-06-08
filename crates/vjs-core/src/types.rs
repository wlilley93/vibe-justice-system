use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AuthorityId(pub String);

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct JurisdictionId(pub String);

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct RepoCode(pub String);

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct IssueTag(pub String);

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SpecId(pub String);

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct DecisionId(pub String);

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct InvariantId(pub String);

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct PermitId(pub String);

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ProofId(pub String);

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ObligationId(pub String);

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct RouteId(pub String);

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SessionId(pub String);

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
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PredicateExpr {
    PathChanged { pattern: String },
    PathMatches { pattern: String },
    FileAdded { pattern: String },
    FileDeleted { pattern: String },
    StringContains { value: String },
    ImportContains { value: String },
    DependencyAdded { value: String },
    DependencyRemoved { value: String },
    NetworkCapabilityAdded,
    ModelCallAdded,
    PublicRecordChanged,
    PrivateMarkerDetected,
    DecisionLogExists,
    OrderExists,
    AuthorityBasisExists,
    TestExists,
    MigrationExists,
    SchemaChanged,
    ProofExists,
    WordCountLte { field: String, max: usize },
    CitationUnique,
    SupersessionTargetExists,
    NoneOf(Vec<PredicateExpr>),
    AllOf(Vec<PredicateExpr>),
    AnyOf(Vec<PredicateExpr>),
    Not(Box<PredicateExpr>),
    If { condition: Box<PredicateExpr>, then: Box<PredicateExpr> },
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
