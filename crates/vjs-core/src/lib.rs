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
        std::cmp::Ordering::Equal
    });
    authorities
}

fn rank_value(rank: &AuthorityRank) -> u8 {
    match rank {
        AuthorityRank::Constitutional => 1,
        AuthorityRank::Primary => 2,
        AuthorityRank::Regulation => 3,
        AuthorityRank::SupremeCourt => 4,
        AuthorityRank::CourtOfAppeal => 5,
        AuthorityRank::PrivyCouncil => 6,
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
