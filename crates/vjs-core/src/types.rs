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
#[serde(rename_all = "snake_case")]
pub struct RawPredicate {
    pub kind: String,
    pub items: Option<Vec<RawPredicate>>,
    pub item: Option<Box<RawPredicate>>,
    pub condition: Option<Box<RawPredicate>>,
    pub then: Option<Box<RawPredicate>>,
    pub glob: Option<String>,
    pub pattern: Option<String>,
    pub value: Option<String>,
    pub name: Option<String>,
    pub issue: Option<String>,
    pub id: Option<String>,
    pub field: Option<String>,
    pub max: Option<usize>,
    pub fields: Option<Vec<String>>,
    pub allowed: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PredicateExpr {
    All { items: Vec<PredicateExpr> },
    Any { items: Vec<PredicateExpr> },
    None { items: Vec<PredicateExpr> },
    Not { item: Box<PredicateExpr> },
    If { condition: Box<PredicateExpr>, then: Box<PredicateExpr> },
    PathChanged { glob: String },
    FileAdded { pattern: String },
    FileModified { pattern: String },
    FileDeleted { pattern: String },
    StringContains { value: String },
    ImportContains { value: String },
    DependencyAdded { name: String },
    DependencyRemoved { name: String },
    DecisionLogExists { issue: Option<String> },
    PermitExists { id: Option<String> },
    ProofExists { kind: Option<String> },
    OrderExists { issue: Option<String> },
    WordCountLte { field: String, max: usize },
    FileWordsLte { glob: String, max: usize },
    CitationUnique,
    RequiredFields { fields: Vec<String> },
    FieldEquals { field: String, value: String },
    IncludedInRuntimeAuthorityGraph,
    PublicNoPrivateFacts,
    CoreNoModelCalls,
    CoreNoNetwork,
    GovernedWritesRequirePermit,
    ProofsExistBeforeClose,
    LogsStayShort,
    LawpackValidates,
    NoDuplicateIds,
    NoDuplicateCitations,
    OrdersHaveDirectives,
    McpLocalFirst,
    DirectoryRolesResolve,
    V1NotLoadedByDefault,
    /// Affirmative, fail-closed allow-list enforcement of CASE-LAW s. 23(5)
    /// ([2026] REALM-SC 10): a record that claims runtime force carries it ONLY
    /// if it declares an `assent_source` resolving to one of `allowed` (e.g. a
    /// specific Sovereign-assent event, or a standing-bounded route tracing to
    /// specific assent). Absence, emptiness, an unrecognised form, or an
    /// unresolved trace each cause rejection. This is NOT a deny-list: a record
    /// that merely omits `assent_source` is rejected, never passed.
    AssentSourceValid { allowed: Vec<String> },
}

impl RawPredicate {
    pub fn to_predicate(&self) -> Result<PredicateExpr, String> {
        match self.kind.as_str() {
            "all" => {
                let items = self.items.as_ref()
                    .ok_or("all requires items")?
                    .iter()
                    .map(|i| i.to_predicate())
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(PredicateExpr::All { items })
            }
            "any" => {
                let items = self.items.as_ref()
                    .ok_or("any requires items")?
                    .iter()
                    .map(|i| i.to_predicate())
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(PredicateExpr::Any { items })
            }
            "none" => {
                let items = self.items.as_ref()
                    .ok_or("none requires items")?
                    .iter()
                    .map(|i| i.to_predicate())
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(PredicateExpr::None { items })
            }
            "not" => {
                let item = self.item.as_ref()
                    .ok_or("not requires item")?
                    .to_predicate()?;
                Ok(PredicateExpr::Not { item: Box::new(item) })
            }
            "if" => {
                let condition = self.condition.as_ref()
                    .ok_or("if requires condition")?
                    .to_predicate()?;
                let then = self.then.as_ref()
                    .ok_or("if requires then")?
                    .to_predicate()?;
                Ok(PredicateExpr::If { condition: Box::new(condition), then: Box::new(then) })
            }
            "path_changed" => {
                let glob = self.glob.as_ref().ok_or("path_changed requires glob")?.clone();
                Ok(PredicateExpr::PathChanged { glob })
            }
            "file_added" => {
                let pattern = self.pattern.as_ref().ok_or("file_added requires pattern")?.clone();
                Ok(PredicateExpr::FileAdded { pattern })
            }
            "file_modified" => {
                let pattern = self.pattern.as_ref().ok_or("file_modified requires pattern")?.clone();
                Ok(PredicateExpr::FileModified { pattern })
            }
            "file_deleted" => {
                let pattern = self.pattern.as_ref().ok_or("file_deleted requires pattern")?.clone();
                Ok(PredicateExpr::FileDeleted { pattern })
            }
            "string_contains" => {
                let value = self.value.as_ref().ok_or("string_contains requires value")?.clone();
                Ok(PredicateExpr::StringContains { value })
            }
            "import_contains" => {
                let value = self.value.as_ref().ok_or("import_contains requires value")?.clone();
                Ok(PredicateExpr::ImportContains { value })
            }
            "dependency_added" => {
                let name = self.name.as_ref().ok_or("dependency_added requires name")?.clone();
                Ok(PredicateExpr::DependencyAdded { name })
            }
            "dependency_removed" => {
                let name = self.name.as_ref().ok_or("dependency_removed requires name")?.clone();
                Ok(PredicateExpr::DependencyRemoved { name })
            }
            "decision_log_exists" => {
                Ok(PredicateExpr::DecisionLogExists { issue: self.issue.clone() })
            }
            "permit_exists" => {
                Ok(PredicateExpr::PermitExists { id: self.id.clone() })
            }
            "proof_exists" => {
                Ok(PredicateExpr::ProofExists { kind: self.id.clone() })
            }
            "order_exists" => {
                Ok(PredicateExpr::OrderExists { issue: self.issue.clone() })
            }
            "word_count_lte" => {
                let field = self.field.as_ref().ok_or("word_count_lte requires field")?.clone();
                let max = self.max.ok_or("word_count_lte requires max")?;
                Ok(PredicateExpr::WordCountLte { field, max })
            }
            "file_words_lte" => {
                let glob = self.glob.as_ref().ok_or("file_words_lte requires glob")?.clone();
                let max = self.max.ok_or("file_words_lte requires max")?;
                Ok(PredicateExpr::FileWordsLte { glob, max })
            }
            "citation_unique" => Ok(PredicateExpr::CitationUnique),
            "required_fields" => {
                let fields = self.fields.as_ref().ok_or("required_fields requires fields")?.clone();
                Ok(PredicateExpr::RequiredFields { fields })
            }
            "field_equals" => {
                let field = self.field.as_ref().ok_or("field_equals requires field")?.clone();
                let value = self.value.as_ref().ok_or("field_equals requires value")?.clone();
                Ok(PredicateExpr::FieldEquals { field, value })
            }
            "included_in_runtime_authority_graph" => Ok(PredicateExpr::IncludedInRuntimeAuthorityGraph),
            "public_no_private_facts" => Ok(PredicateExpr::PublicNoPrivateFacts),
            "core_no_model_calls" => Ok(PredicateExpr::CoreNoModelCalls),
            "core_no_network" => Ok(PredicateExpr::CoreNoNetwork),
            "governed_writes_require_permit" => Ok(PredicateExpr::GovernedWritesRequirePermit),
            "proofs_exist_before_close" => Ok(PredicateExpr::ProofsExistBeforeClose),
            "logs_stay_short" => Ok(PredicateExpr::LogsStayShort),
            "lawpack_validates" => Ok(PredicateExpr::LawpackValidates),
            "no_duplicate_ids" => Ok(PredicateExpr::NoDuplicateIds),
            "no_duplicate_citations" => Ok(PredicateExpr::NoDuplicateCitations),
            "orders_have_directives" => Ok(PredicateExpr::OrdersHaveDirectives),
            "mcp_local_first" => Ok(PredicateExpr::McpLocalFirst),
            "directory_roles_resolve" => Ok(PredicateExpr::DirectoryRolesResolve),
            "v1_not_loaded_by_default" => Ok(PredicateExpr::V1NotLoadedByDefault),
            "assent_source_valid" => {
                let allowed = self
                    .allowed
                    .as_ref()
                    .ok_or("assent_source_valid requires allowed")?
                    .clone();
                if allowed.is_empty() {
                    return Err("assent_source_valid requires a non-empty allowed list".to_string());
                }
                Ok(PredicateExpr::AssentSourceValid { allowed })
            }
            other => Err(format!("Unknown predicate kind: {}", other)),
        }
    }
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
