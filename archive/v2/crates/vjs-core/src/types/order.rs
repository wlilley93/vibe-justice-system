//! The Order record and its Directive: split from types.rs 2026-08-06 under the
//! structural ceiling (behavior-preserving), matching the subscriber's own earlier
//! split so the two trees carry one module shape.

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::{
    ACTOR_UNSTATED, AuthorityId, AuthorityStatus, Court, IssueTag, JurisdictionId, RepoCode,
    seq_of_strings_lossy, string_or_seq, string_or_seq_opt,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Order {
    /// Defaulted ([2026] VJS-CC-OPBOX 160 O1/O2, adopted upstream 2026-08-06): two of a
    /// subscriber's filed orders carry no `id` key. The overlay fills an empty id from
    /// the FILE STEM, which is the id by the store's own naming convention. That is
    /// reading the record as written, not inventing a value: the id is already stated,
    /// on the file.
    #[serde(default)]
    pub id: String,
    pub court: Court,
    pub jurisdiction: JurisdictionId,
    pub repo_code: Option<RepoCode>,
    pub status: AuthorityStatus,
    pub issue: IssueTag,
    pub holding: String,
    pub directives: Vec<Directive>,
    #[serde(default, deserialize_with = "seq_of_strings_lossy")]
    pub forbidden: Option<Vec<String>>,
    #[serde(default, deserialize_with = "string_or_seq_opt")]
    pub exceptions: Option<Vec<String>>,
    /// Defaulted: an order that supersedes nothing should not have to say so, and requiring it made
    /// SIX filed orders unparseable - validated, committed, and then invisible to the kernel, which
    /// is the worst of both worlds. Never rewrite a filed record to satisfy a struct; widen the
    /// struct to read the record.
    #[serde(default)]
    pub supersedes: Vec<AuthorityId>,
    /// `opinion` is accepted as an alias ([2026] VJS-CC-OPBOX 160 O1, adopted upstream
    /// 2026-08-06): dozens of a subscriber's filed orders write `opinion:` rather than
    /// `source_opinion:`, and until this alias existed a bench-declaring order among
    /// them failed the bench-opinion check - the order declared its bench, the opinion
    /// file was sitting right there on disk, and the record could not verify one against
    /// the other because of a key name. Read the record as written.
    #[serde(alias = "opinion")]
    pub source_opinion: Option<PathBuf>,
    /// Defaulted for the same reason as `supersedes`. An order without an agent-facing summary is
    /// thinner, but it is still BINDING; refusing to parse it makes it invisible to the resolver,
    /// which is strictly worse. The lawpack validator still enforces the word limit where one is
    /// present, so nothing is weakened for orders that carry it.
    #[serde(default)]
    pub runtime_summary: String,
    /// Defaulted ([2026] VJS-CC-OPBOX 160 O2, adopted upstream 2026-08-06): one filed
    /// order carries no `created_at`. The date is recoverable from git if it is ever
    /// needed; its absence is not a reason for the order to bind nothing.
    #[serde(default)]
    pub created_at: String,
    // Promote the citation and assent source the orders already carry (they
    // were silently dropped on load). Optional so the existing orders are valid.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub citation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assent_source: Option<String>,
    // Auditable court record: the bench that decided, the sha256 of the
    // symmetric case file, when it convened, the vote, and the appeal links.
    #[serde(
        default,
        deserialize_with = "string_or_seq",
        skip_serializing_if = "Vec::is_empty"
    )]
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
    /// left open. CC-SUB1 4 recorded ten fact corrections and called one of them the most important
    /// correction in the case; an apply over that order would have deleted it, leaving a ruling that
    /// cites facts the same ruling found false with no record that it had. Deleting `reserved` is the
    /// same harm in the other direction: it turns "expressly not decided" into "silent".
    ///
    /// FLATTEN RATHER THAN MORE NAMED FIELDS, deliberately. The comment above `citation` records that
    /// this exact class was hit before and cured by adding two fields - which leaves the next author
    /// of the next field to remember. A catch-all is structural: an unknown key round-trips because
    /// it is unknown, not because somebody listed it. Same reasoning as the credential envelope in
    /// SUB1: where loss must be impossible, the mechanism cannot be a list of names.
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, serde_yaml::Value>,
}

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
