use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

pub mod audit;
pub mod authority;
pub mod bench;
pub mod capability;
pub mod citation;
pub mod court;
pub mod effects;
pub mod enforcement;
pub mod error;
pub mod evals;
pub mod front_door;
pub mod governance;
pub mod hook;
pub mod install;
pub mod repo;
pub mod report;
pub mod risk;
pub mod route;
pub mod scope;
pub mod spec;
pub mod types;

pub use authority::*;
pub use court::*;
pub use error::*;
pub use governance::*;
pub use repo::*;
pub use route::*;
pub use spec::*;
pub use types::*;

/// The canonical "is this a lawpack/canon YAML file" test, used by the law LOADER and
/// VALIDATOR so they recognise exactly the set the governed-record gate does. The front
/// door (`front_door::is_governed_record`) and the apex bright-line (`hook`) both accept
/// `.yaml` AND `.yml`; if the loader keyed on `.yaml` only, a `.yml` order would be
/// permit-gated and apex-routed on write yet never loaded, validated, dup-checked, or
/// citation-grounded - a record the gate protects but the validator never sees. Keeping
/// one shared extension set closes that seam (the loaded/validated set equals the
/// governed set). Runtime artifacts the kernel authors itself (.vjs/logs, permits,
/// proofs, invocations) are always written `.yaml` and are NOT this law-loading seam.
pub fn is_lawpack_yaml(path: &std::path::Path) -> bool {
    matches!(
        path.extension().and_then(|s| s.to_str()),
        Some("yaml") | Some("yml")
    )
}

pub struct KernelContext {
    pub authority_graph: AuthorityGraph,
    pub limits: ContextLimits,
    pub lawpack_digest: String,
}

pub struct ContextLimits {
    pub route_max_authorities: usize,
    pub route_max_words: usize,
    pub rule_summary_max_words: usize,
    pub decision_log_max_words: usize,
    pub county_submission_max_words: usize,
    pub county_order_max_words: usize,
    pub county_opinion_max_words: usize,
    pub privy_submission_max_words: usize,
    pub privy_order_max_words: usize,
    pub privy_opinion_max_words: usize,
    pub supreme_order_max_words: usize,
}

impl Default for ContextLimits {
    fn default() -> Self {
        Self {
            route_max_authorities: 5,
            route_max_words: 300,
            rule_summary_max_words: 120,
            decision_log_max_words: 150,
            county_submission_max_words: 500,
            county_order_max_words: 500,
            county_opinion_max_words: 2000,
            privy_submission_max_words: 1000,
            privy_order_max_words: 1000,
            privy_opinion_max_words: 5000,
            supreme_order_max_words: 1500,
        }
    }
}

impl ContextLimits {
    /// Read `[limits]` out of a lawpack's `manifest.toml`.
    ///
    /// Until 2026-08-07 nothing did. The manifest declared eleven limits, every one of
    /// them was in this struct, and every consumer got them from `Default::default()` -
    /// so the table was decorative and an estate that edited its manifest was ignored
    /// in silence. The two known consumers had additionally hardcoded their own numbers
    /// rather than reading even the defaults, which is how `vjs file` came to apply the
    /// COUNTY word ceiling to a Privy Council case file.
    ///
    /// PER-FIELD FALLBACK, and the reason is worth stating because "default on missing"
    /// is usually the silent-default hazard. Here the defaults above ARE the canonical
    /// values, written to match the manifest this repository ships; a manifest that
    /// omits a limit is not expressing a preference, it is declining to override one.
    /// What was dangerous was never the fallback. It was that the fallback was the ONLY
    /// path.
    ///
    /// A malformed manifest yields the defaults rather than an error: this is read on
    /// the way to building a context for commands that must keep working, and a
    /// jurisdiction should not lose the ability to run `vjs status` because a limit was
    /// typed wrong. The lawpack's own validator is where a malformed manifest is
    /// reported.
    pub fn from_manifest(lawpack_dir: &std::path::Path) -> Self {
        let mut out = Self::default();
        let Ok(text) = std::fs::read_to_string(lawpack_dir.join("manifest.toml")) else {
            return out;
        };
        let Ok(parsed) = text.parse::<toml::Value>() else {
            return out;
        };
        let Some(limits) = parsed.get("limits").and_then(|l| l.as_table()) else {
            return out;
        };
        let get = |k: &str| -> Option<usize> {
            limits
                .get(k)
                .and_then(|v| v.as_integer())
                .and_then(|i| usize::try_from(i).ok())
        };
        // Assigned one by one rather than through serde, so that a field the manifest
        // carries and this struct does not is simply ignored, and a field this struct
        // carries and the manifest does not keeps its canonical value. Neither is an
        // error. Verbose on purpose: a table-driven version needs either raw pointers or
        // a macro, and neither is worth it in a kernel whose whole claim is that you can
        // read what it does.
        macro_rules! take {
            ($($field:ident),* $(,)?) => {
                $( if let Some(v) = get(stringify!($field)) { out.$field = v; } )*
            };
        }
        take!(
            route_max_authorities,
            route_max_words,
            rule_summary_max_words,
            decision_log_max_words,
            county_submission_max_words,
            county_order_max_words,
            county_opinion_max_words,
            privy_submission_max_words,
            privy_order_max_words,
            privy_opinion_max_words,
            supreme_order_max_words,
        );
        out
    }
}

pub struct AuthorityGraph {
    pub authorities: HashMap<AuthorityId, Authority>,
    pub supersessions: Vec<Supersession>,
}

impl Default for AuthorityGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthorityGraph {
    pub fn new() -> Self {
        Self {
            authorities: HashMap::new(),
            supersessions: Vec::new(),
        }
    }

    pub fn apply(&mut self, delta: RuleDelta) -> Result<(), KernelError> {
        for added in delta.added {
            let id = added.id.clone();
            self.authorities.insert(id.clone(), Authority::from(added));
        }
        for varied in delta.varied {
            if let Some(existing) = self.authorities.get_mut(&varied.id) {
                existing.status = varied.status;
                existing.supersedes = varied.supersedes.clone();
            }
        }
        for superseded in &delta.superseded {
            if let Some(existing) = self.authorities.get_mut(superseded) {
                existing.status = AuthorityStatus::Superseded;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Authority {
    pub id: AuthorityId,
    pub kind: AuthorityKind,
    pub rank: AuthorityRank,
    pub status: AuthorityStatus,
    pub jurisdiction: Option<JurisdictionId>,
    pub title: String,
    pub summary: String,
    pub source_path: Option<PathBuf>,
    pub issue_tags: Vec<IssueTag>,
    pub scope: Option<Scope>,
    pub supersedes: Vec<AuthorityId>,
}

impl From<RuleAtom> for Authority {
    fn from(atom: RuleAtom) -> Self {
        Self {
            id: atom.id,
            kind: AuthorityKind::Rule,
            rank: AuthorityRank::Regulation,
            status: atom.status,
            jurisdiction: atom
                .scope
                .jurisdictions
                .as_ref()
                .and_then(|v| v.first().cloned()),
            title: atom.title,
            summary: atom.summary,
            source_path: None,
            issue_tags: atom.scope.issue_tags.clone().unwrap_or_default(),
            scope: Some(atom.scope),
            supersedes: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityKind {
    Statute,
    Regulation,
    Rule,
    Order,
    Log,
}

pub fn sort_by_rank_then_specificity_then_date(
    mut authorities: Vec<Authority>,
    _graph: &AuthorityGraph,
) -> Vec<Authority> {
    authorities.sort_by(|a, b| {
        let rank_cmp = rank_value(&a.rank).cmp(&rank_value(&b.rank));
        if rank_cmp != std::cmp::Ordering::Equal {
            return rank_cmp;
        }
        // Lex specialis: of two authorities at the same rank, the one that
        // constrains more scope dimensions is the more specific and wins.
        // Higher specificity must sort earlier, so compare in reverse.
        let spec_cmp = specificity_score(b).cmp(&specificity_score(a));
        if spec_cmp != std::cmp::Ordering::Equal {
            return spec_cmp;
        }
        // Final total, deterministic tiebreaker on the authority id so that
        // equal-rank, equal-specificity authorities have one fixed order
        // regardless of HashMap iteration (REG-KERNEL-001 determinism).
        a.id.0.cmp(&b.id.0)
    });
    authorities
}

/// A lex-specialis specificity score: the number of scope dimensions an
/// authority actually constrains. A higher score means a narrower, more
/// specific authority, which outranks a more general one of equal rank.
fn specificity_score(authority: &Authority) -> usize {
    match authority.scope {
        Some(ref scope) => {
            let mut score = 0;
            if scope.paths.as_ref().is_some_and(|v| !v.is_empty()) {
                score += 1;
            }
            if scope.jurisdictions.as_ref().is_some_and(|v| !v.is_empty()) {
                score += 1;
            }
            if scope.action_kinds.as_ref().is_some_and(|v| !v.is_empty()) {
                score += 1;
            }
            if scope.issue_tags.as_ref().is_some_and(|v| !v.is_empty()) {
                score += 1;
            }
            if scope.records.as_ref().is_some_and(|v| !v.is_empty()) {
                score += 1;
            }
            score
        }
        None => 0,
    }
}

fn rank_value(rank: &AuthorityRank) -> u8 {
    match rank {
        AuthorityRank::Constitutional => 1,
        AuthorityRank::Primary => 2,
        AuthorityRank::Regulation => 3,
        AuthorityRank::SupremeCourt => 4,
        // Privy Council outranks the Court of Appeal: the Privy Council grants leave to the
        // Supreme Court and holds interpretive power over entrenched terms (PC-10/PC-16), so it
        // sits second only to the apex; the CoA is the intermediate merits tier above County.
        AuthorityRank::PrivyCouncil => 5,
        AuthorityRank::CourtOfAppeal => 6,
        AuthorityRank::CountyCourt => 7,
        AuthorityRank::Log => 8,
    }
}

pub fn remove_superseded(authorities: Vec<Authority>, graph: &AuthorityGraph) -> Vec<Authority> {
    let superseded: std::collections::HashSet<AuthorityId> = graph
        .supersessions
        .iter()
        .map(|s| s.old_id.clone())
        .collect();
    authorities
        .into_iter()
        .filter(|a| !superseded.contains(&a.id))
        .collect()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Supersession {
    pub old_id: AuthorityId,
    pub new_id: AuthorityId,
    pub kind: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuleDelta {
    pub added: Vec<RuleAtom>,
    pub varied: Vec<RuleAtom>,
    pub superseded: Vec<AuthorityId>,
}

#[cfg(test)]
mod rank_tests {
    use super::rank_value;
    use crate::types::AuthorityRank;

    #[test]
    fn privy_council_outranks_the_court_of_appeal() {
        // PC-10/PC-16: the Privy Council sits second only to the apex; the Court of Appeal is
        // the intermediate merits tier above County. Lower rank_value = higher precedence.
        // (PR #21 had inverted this, ranking CoA above the Privy Council.)
        assert!(
            rank_value(&AuthorityRank::SupremeCourt) < rank_value(&AuthorityRank::PrivyCouncil)
        );
        assert!(
            rank_value(&AuthorityRank::PrivyCouncil) < rank_value(&AuthorityRank::CourtOfAppeal)
        );
        assert!(
            rank_value(&AuthorityRank::CourtOfAppeal) < rank_value(&AuthorityRank::CountyCourt)
        );
    }
}

#[cfg(test)]
mod deterministic_sort_tests {
    use super::{
        Authority, AuthorityGraph, AuthorityKind, sort_by_rank_then_specificity_then_date,
    };
    use crate::types::{
        AuthorityId, AuthorityRank, AuthorityStatus, IssueTag, JurisdictionId, Scope,
    };

    fn auth(id: &str, rank: AuthorityRank, scope: Option<Scope>) -> Authority {
        Authority {
            id: AuthorityId(id.into()),
            kind: AuthorityKind::Rule,
            rank,
            status: AuthorityStatus::Binding,
            jurisdiction: None,
            title: id.into(),
            summary: String::new(),
            source_path: None,
            issue_tags: Vec::new(),
            scope,
            supersedes: Vec::new(),
        }
    }

    fn specific_scope() -> Scope {
        Scope {
            paths: None,
            jurisdictions: Some(vec![JurisdictionId("uk".into())]),
            action_kinds: None,
            issue_tags: Some(vec![IssueTag("data".into())]),
            records: None,
        }
    }

    fn ids(authorities: &[Authority]) -> Vec<String> {
        authorities.iter().map(|a| a.id.0.clone()).collect()
    }

    fn build(perm: &[&str]) -> Vec<Authority> {
        perm.iter()
            .map(|id| match *id {
                // C is a Regulation that constrains two scope dimensions, so it
                // is more specific than the bare Regulations A and B.
                "C" => auth("C", AuthorityRank::Regulation, Some(specific_scope())),
                // D is the only Constitutional authority (highest rank).
                "D" => auth("D", AuthorityRank::Constitutional, None),
                other => auth(other, AuthorityRank::Regulation, None),
            })
            .collect()
    }

    #[test]
    fn equal_rank_order_is_total_and_deterministic_over_shuffles() {
        let graph = AuthorityGraph::new();
        // Expected order, independent of input order:
        //   D -> Constitutional (highest rank)
        //   C -> Regulation, more specific (lex specialis) than A/B
        //   A -> Regulation, general; id tiebreaker puts "A" before "B"
        //   B -> Regulation, general
        let expected = vec!["D", "C", "A", "B"];

        let permutations = [
            vec!["A", "B", "C", "D"],
            vec!["D", "C", "B", "A"],
            vec!["B", "D", "A", "C"],
            vec!["C", "A", "D", "B"],
        ];

        for perm in permutations {
            let sorted = sort_by_rank_then_specificity_then_date(build(&perm), &graph);
            assert_eq!(
                ids(&sorted),
                expected,
                "input permutation {perm:?} produced a non-deterministic order"
            );
        }
    }
}

#[cfg(test)]
mod yaml_ext_tests {
    use super::is_lawpack_yaml;
    use std::path::Path;

    /// The loader/validator extension set must match the governed-record gate, which
    /// accepts BOTH `.yaml` and `.yml`. A loader that took `.yaml` only would leave a
    /// `.yml` order gated-but-unvalidated.
    #[test]
    fn accepts_both_yaml_and_yml_and_nothing_else() {
        assert!(is_lawpack_yaml(Path::new(
            "lawpack/v2/orders/2026-VJS-PC-001.yaml"
        )));
        assert!(is_lawpack_yaml(Path::new(
            "lawpack/v2/orders/2026-VJS-PC-001.yml"
        )));
        assert!(!is_lawpack_yaml(Path::new("README.md")));
        assert!(!is_lawpack_yaml(Path::new("notes.yaml.txt")));
        assert!(!is_lawpack_yaml(Path::new("no-extension")));
    }
}
