//! The deterministic invariant evaluator: `evaluate_invariants` walks the declared invariants over
//! the staged `RepoState` + `LawpackFacts`, and `evaluate_predicate` is the pure `PredicateExpr`
//! interpreter (no model call, no network, no clock; DEC-KERNEL-001). One glob semantics, shared
//! with the permit gate.

use std::path::Path;

use super::model::{Invariant, LawpackFacts, RepoState};
use crate::error::*;
use crate::types::*;

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

/// The canon subtrees whose location alone asserts binding runtime force. ONE named
/// declaration, so the four arms cannot drift apart.
///
/// LAWPACK-LITERAL: referent=local-records; status=local; authority=[2026] VJS-CC-VJS 15.
/// Matched against paths in THIS working tree, never used to open a canon.
const RUNTIME_FORCE_SUBTREES: [&str; 4] = [
    "lawpack/v2/statutes/",
    "lawpack/v2/regulations/",
    "lawpack/v2/rules/",
    "lawpack/v2/orders/",
];

/// A path that, by its location, claims binding runtime force (statute,
/// regulation, rule, or order). One definition so every witness agrees.
fn claims_runtime_force(path: &Path) -> bool {
    let p = path.to_string_lossy();
    RUNTIME_FORCE_SUBTREES.iter().any(|sub| p.contains(sub))
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

/// The canonical snake_case token for a proof kind, matching the lawpack wire form
/// (`proof_kind:` in a `proof_exists` predicate). Explicit and total so the match is
/// deterministic, and so a new `ProofKind` variant fails to COMPILE until it is given a
/// token here, rather than silently never matching.
fn proof_kind_token(kind: &ProofKind) -> &'static str {
    match kind {
        ProofKind::CommandResult => "command_result",
        ProofKind::DecisionLog => "decision_log",
        ProofKind::TestResult => "test_result",
        ProofKind::PublicPrivateScan => "public_private_scan",
        ProofKind::ValidationReport => "validation_report",
    }
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
        // Existence predicates honour their OPTIONAL argument: a `None` arg keeps the
        // bare existence check (the form the live invariants use), but a `Some` arg MUST
        // match a specific record. Before this the arg was discarded - so a lawpack author
        // who wired `decision_log_exists{issue: X}`, a permit by id, a proof of a specific
        // kind, or an order on an issue got a silent always-pass satisfied by ANY record.
        // Match the arg so a check that READS as targeted IS targeted. Deterministic: a
        // pure scan of the staged records, no model call/network/clock (DEC-KERNEL-001).
        PredicateExpr::DecisionLogExists { issue } => match issue {
            Some(want) => repo_state.logs.iter().any(|l| l.issue == *want),
            None => !repo_state.logs.is_empty(),
        },
        PredicateExpr::PermitExists { id } => match id {
            Some(want) => repo_state.permits.iter().any(|p| p.id.0 == *want),
            None => !repo_state.permits.is_empty(),
        },
        PredicateExpr::ProofExists { kind } => match kind {
            Some(want) => repo_state
                .proofs
                .iter()
                .any(|p| proof_kind_token(&p.kind) == want.as_str()),
            None => !repo_state.proofs.is_empty(),
        },
        PredicateExpr::OrderExists { issue } => match issue {
            Some(want) => repo_state.orders.iter().any(|o| o.issue.0 == *want),
            None => !repo_state.orders.is_empty(),
        },
        PredicateExpr::WordCountLte { .. } => {
            // FAIL-CLOSED. RepoState carries no structured per-field record to count
            // words on, so this cannot be implemented faithfully here. The parser
            // (RawPredicate::to_predicate) rejects `word_count_lte` so a lawpack can
            // never LOAD one; if one is ever constructed directly it must not give false
            // assurance, so it fails rather than silently passing. Use `file_words_lte`
            // for file-level word limits or `logs_stay_short` for decision-log brevity.
            false
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
            // The whole-lawpack uniqueness fact (computed by the validator's
            // check_citation_uniqueness, handed in via LawpackFacts). "Citation unique"
            // == "no duplicate citations", so this shares the real witness with
            // NoDuplicateCitations instead of always passing.
            !facts.duplicate_citations
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
            // Exclude the invariant evaluator itself (spec/evaluate.rs holds these patterns as string literals; was spec.rs pre-split) and test files
            repo_state.file_contents.iter().all(|(path, content)| {
                if !path.to_string_lossy().contains("vjs-core") {
                    return true;
                }
                // Skip the evaluator file and test files
                let path_str = path.to_string_lossy();
                if path_str.contains("spec/evaluate.rs")
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
            // runtime force carries it ONLY if it declares a valid `assent_source`.
            // s.23 has TWO limbs - well-formedness (the value is an allowed FORM) and
            // resolution (the form resolves to a specific Sovereign-assent event / a
            // standing-bounded route tracing to specific assent; "an unresolved trace
            // causes rejection").
            //
            // THIS evaluator is the always-on WELL-FORMEDNESS limb: an AFFIRMATIVE
            // ALLOW-LIST that FAILS CLOSED over every runtime-force record. Absence of
            // the field, an empty value, or an unrecognised form cause rejection here.
            // It is NOT a deny-list (the not-equal-to-self_authorised form is void as
            // fail-open, s. 23(5)). Deterministic: no model call, no similarity search.
            //
            // The RESOLUTION limb ("an unresolved trace causes rejection") is enforced
            // deterministically at the floor-attachment site, where a STAGED record
            // claims the assent floor's protection: vjs_engine::assent::assent_resolves
            // ([2026] VJS-PC 16 D1). A record that types an allowed form but resolves to
            // no real Sovereign-assent event is well-formed here yet does not earn the
            // floor, so its findings keep their native severity. The two limbs are kept
            // separate by design: this one binds all canon every run; resolution binds a
            // record at the moment it would be sheltered.
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
                    Some(v) => allowed.contains(&v),
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
