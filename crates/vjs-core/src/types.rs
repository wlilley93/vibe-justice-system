use serde::{Deserialize, Deserializer, Serialize};
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

/// The non-optional twin of `string_or_seq_opt`, for `Order::bench`.
///
/// [2026] VJS-CC-OPBOX 160 O2, adopted upstream 2026-08-06. Six of a subscriber's filed
/// orders write `bench` as a SCALAR (`bench: first_instance_one_judge`) where this struct
/// wants a sequence, and until this existed those orders did not parse - so a recorded
/// bench bound nothing. Read as written; the record is not edited to suit the reader
/// (the ratio already stated at `supersedes` below).
fn string_or_seq<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum OneOrMany {
        // `bench:` with NO VALUE parses as a unit, which one filed order writes. Listed
        // FIRST because serde tries untagged variants in order and null must not fall
        // through to a string.
        Null,
        One(String),
        Many(Vec<String>),
    }
    Ok(match OneOrMany::deserialize(deserializer)? {
        OneOrMany::Null => Vec::new(),
        OneOrMany::One(s) => vec![s],
        OneOrMany::Many(v) => v,
    })
}

/// Read a list whose elements may be strings OR structured nodes, rendering a non-string
/// as its YAML text.
///
/// [2026] VJS-CC-OPBOX 160 O1, adopted upstream 2026-08-06. One filed order's
/// `forbidden[0]` is a MAP where this struct wants a string, and until this existed that
/// order bound nothing. The content is preserved verbatim as text rather than dropped or
/// guessed at: the reader reports what the record says, and does not decide what the
/// author meant by nesting it.
fn seq_of_strings_lossy<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw: Option<Vec<serde_yaml::Value>> = Option::deserialize(deserializer)?;
    Ok(raw.map(|items| {
        items
            .into_iter()
            .map(|v| match v {
                serde_yaml::Value::String(s) => s,
                other => serde_yaml::to_string(&other)
                    .unwrap_or_default()
                    .trim_end()
                    .to_string(),
            })
            .collect()
    }))
}

mod ids;
pub use ids::*;
mod predicate;
pub use predicate::*;
mod referral;
pub use referral::*;
mod order;
pub use order::*;

/// See `Directive::actor`. A named constant rather than a literal so a search for the
/// sentinel finds every place that reads it.
pub const ACTOR_UNSTATED: &str = "UNSTATED";

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
    /// An authority whose recorded status this enum does not know.
    ///
    /// [2026] VJS-CC-OPBOX 160 O4, adopted upstream 2026-08-06. A subscriber's filed
    /// record carries `status: corrected_to_referral`, and until this variant existed
    /// that record did not parse at all. Aliasing it onto an existing status was
    /// expressly FORBIDDEN by that order: it would be the reader deciding what the
    /// record's status IS, which is substantive. So it is readable, it is reported as
    /// unrecognised, and `is_live()` is FALSE for it - an authority whose status cannot
    /// be understood must not confer binding force. Its real status is reserved to a
    /// fresh matter.
    #[serde(other)]
    Unrecognised,
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
    /// The long form `County Court at <jurisdiction>` is accepted as well as `county`,
    /// on the same footing as `appeal` and `privy` below: three of a subscriber's filed
    /// orders write the long form, and until this alias existed all three were absent
    /// from the citator, so every route in that repository resolved without them
    /// ([2026] VJS-CC-OPBOX 160 O2, adopted upstream 2026-08-06).
    #[serde(alias = "County Court at opbox")]
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
