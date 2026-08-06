use serde::{Deserialize, Deserializer, Serialize};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::path::PathBuf;

/// Read `exceptions:` written either as a YAML sequence or as a single prose block.
///
/// Same rule as the `appeal` / `privy` court aliases and the defaulted `supersedes` below:
/// **never rewrite a filed record to satisfy a struct; widen the struct.** The filed order
/// `2026-VJS-CC-BOLTRIG-CODEX-APPROVAL-ROUTING-001` writes its exception as a `|` block
/// (a contingency paragraph, which is a reasonable way to express one), and until this
/// existed the whole order failed to deserialise - so it was absent from the citator and
/// bound nothing at all. The failure mode is the dangerous one: the order validated, was
/// committed, and then silently had no effect.
///
/// A single string is read as a one-element list, which is what it means.
fn string_or_seq_opt<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum OneOrMany {
        One(String),
        Many(Vec<String>),
    }
    Ok(
        Option::<OneOrMany>::deserialize(deserializer)?.map(|v| match v {
            OneOrMany::One(s) => vec![s],
            OneOrMany::Many(v) => v,
        }),
    )
}

mod ids;
pub use ids::*;
mod predicate;
pub use predicate::*;
mod referral;
pub use referral::*;

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
            // Proposed is PRE-ENACTMENT (statute ACT-001:s7: "Proposed law must be
            // marked draft. Binding law requires authorised adoption."), parallel to
            // Draft and Stayed which are also excluded - it confers no binding force and
            // must not resolve as live law (RouteDecision.binding) or suppress a
            // FirstImpression court trigger (court.rs::any_on_point). Only an adopted
            // authority is live - matching the in-force test in vjs-engine staged.rs.
            AuthorityStatus::Binding | AuthorityStatus::InForce
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
    ///
    /// `appeal` is accepted as well, on the same footing as `privy` below: the filed order
    /// `2026-VJS-CA-BOLTRIG-CODEX-APPROVAL-ROUTING-001` writes `court: appeal`, and until this
    /// alias existed that order did not parse - so it was silently absent from the citator and
    /// bound nothing. An order that does not parse is not a lenient reader's problem, it is an
    /// order that has no effect, and the cure is to read the record as written rather than edit
    /// a filed record to suit the reader.
    #[serde(alias = "appeal")]
    CourtOfAppeal,
    /// `privy` is accepted as well as `privy_council`: a filed order already uses it, and a filed
    /// record is read as written rather than edited to fit the reader (the never-rewrite-history
    /// rule). Serialises as `privy_council`.
    #[serde(alias = "privy")]
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

/// Normalise an issue tag for matching: lowercase, `_` folded to `-`.
///
/// Order ids are SCREAMING-HYPHEN (`2026-VJS-CC-BOLTRIG-OPERATOR-SEAT-001`) while
/// an order's own `issue:` field is lower_snake (`operator_seat_host_boundary`),
/// and both name the same matter. ONE definition, used by both
/// `court::any_on_point` and the hoisting partition in `authority::resolve`: if
/// they disagreed, an order could be judged on-point yet not hoisted, fall below
/// the truncation limit, and so never reach the test that just admitted it.
pub fn fold_tag(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c == '_' {
                '-'
            } else {
                c.to_ascii_lowercase()
            }
        })
        .collect()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthorityPointer {
    pub id: AuthorityId,
    pub title: String,
    pub rank: AuthorityRank,
    pub status: AuthorityStatus,
    pub summary: String,
    pub source_path: Option<PathBuf>,
    /// What the authority says IT is about. Carried through from `Authority` so
    /// `court::any_on_point` can read an order's own declared issue instead of
    /// inferring it from the id/title/summary prose. Dropping it here is what
    /// let a filed, binding, exactly-on-point order be reported as
    /// first-impression. Defaults empty so an older serialised set still loads.
    #[serde(default)]
    pub issue_tags: Vec<IssueTag>,
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
    #[serde(default, deserialize_with = "string_or_seq_opt")]
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
    #[serde(default, deserialize_with = "string_or_seq_opt")]
    pub exceptions: Option<Vec<String>>,
    /// Defaulted: an order that supersedes nothing should not have to say so, and requiring it made
    /// SIX filed orders unparseable - validated, committed, and then invisible to the kernel, which
    /// is the worst of both worlds. Never rewrite a filed record to satisfy a struct; widen the
    /// struct to read the record.
    #[serde(default)]
    pub supersedes: Vec<AuthorityId>,
    pub source_opinion: Option<PathBuf>,
    /// Defaulted for the same reason as `supersedes`. An order without an agent-facing summary is
    /// thinner, but it is still BINDING; refusing to parse it makes it invisible to the resolver,
    /// which is strictly worse. The lawpack validator still enforces the word limit where one is
    /// present, so nothing is weakened for orders that carry it.
    #[serde(default)]
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

    /// EVERY OTHER FIELD THE FILE CARRIES, preserved verbatim.
    ///
    /// Without this, `vjs order apply` was DESTRUCTIVE: it parsed a filed order into this struct and
    /// wrote it back, so any key not named above was silently deleted from the record. On 2026-07-27
    /// applying one order removed 69 lines - `title`, `question`, `fact_corrections`,
    /// `execution_findings`, `reserved`, `rows_already_written`, `full_case_file` and its digest,
    /// `filed_submission`, `convening`, `permission_to_appeal` - and reported only "Order applied".
    /// `vjs validate` passed either side of the deletion, so nothing noticed.
    ///
    /// The losses were the parts that make a holding CHECKABLE: the question it answers, the case
    /// file it was decided on, the corrections to the filing's facts, and the questions expressly
    /// left open. CC-OPBOX 4 recorded ten fact corrections and called one of them the most important
    /// correction in the case; an apply over that order would have deleted it, leaving a ruling that
    /// cites facts the same ruling found false with no record that it had. Deleting `reserved` is the
    /// same harm in the other direction: it turns "expressly not decided" into "silent".
    ///
    /// FLATTEN RATHER THAN MORE NAMED FIELDS, deliberately. The comment above `citation` records that
    /// this exact class was hit before and cured by adding two fields - which leaves the next author
    /// of the next field to remember. A catch-all is structural: an unknown key round-trips because
    /// it is unknown, not because somebody listed it. Same reasoning as the credential envelope in
    /// opbox: where loss must be impossible, the mechanism cannot be a list of names.
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, serde_yaml::Value>,
}

/// See `Directive::actor`. A named constant rather than a literal so a search for the
/// sentinel finds every place that reads it.
pub const ACTOR_UNSTATED: &str = "UNSTATED";

fn actor_unstated() -> String {
    ACTOR_UNSTATED.to_string()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Directive {
    pub id: String,
    /// Defaults to `UNSTATED`, and that spelling is the point ([2026] VJS-CC-OPBOX 160 O3,
    /// adopted upstream 2026-08-06 after the strict parse cost a subscriber fifty-four
    /// binding precedents at the reconciliation re-pull - the O5 gate caught the loss).
    ///
    /// Dozens of a subscriber's filed orders omit `actor` on their directives. A directive
    /// is a DUTY, so defaulting it to `engineer` - or to any other bearer - would be the
    /// reader deciding who is bound, which that order expressly forbids. `UNSTATED` reads
    /// back as what the record actually says: that nobody was named. Whether such a
    /// directive binds anyone, and if so whom, is reserved; ACT-PROCEEDINGS-DISCIPLINE s10
    /// expects exactly this state to be READ, reported and counted, never refused and
    /// never papered over.
    #[serde(default = "actor_unstated")]
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
