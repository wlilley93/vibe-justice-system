use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use vjs_core::spec::InvariantRaw;
use vjs_core::*;

pub mod overlay;
pub mod refs;

pub struct LawpackLoader;

impl LawpackLoader {
    pub fn load(lawpack_dir: &Path) -> Result<Lawpack, KernelError> {
        let mut statutes = Vec::new();
        let mut regulations = Vec::new();
        let mut rules = Vec::new();
        let mut orders = Vec::new();
        let mut specs = Vec::new();
        let mut invariants = Vec::new();
        let mut decisions = Vec::new();

        let statutes_dir = lawpack_dir.join("statutes");
        if statutes_dir.exists() {
            for entry in WalkDir::new(&statutes_dir).max_depth(1) {
                let entry = entry.map_err(|e| KernelError::Io(e.to_string()))?;
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                    let content = std::fs::read_to_string(path)
                        .map_err(|e| KernelError::Io(e.to_string()))?;
                    let statute: Statute = serde_yaml::from_str(&content)
                        .map_err(|e| KernelError::Serialization(e.to_string()))?;
                    statutes.push(statute);
                }
            }
        }

        let regulations_dir = lawpack_dir.join("regulations");
        if regulations_dir.exists() {
            for entry in WalkDir::new(&regulations_dir).max_depth(1) {
                let entry = entry.map_err(|e| KernelError::Io(e.to_string()))?;
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                    let content = std::fs::read_to_string(path)
                        .map_err(|e| KernelError::Io(e.to_string()))?;
                    let regulation: Regulation = serde_yaml::from_str(&content)
                        .map_err(|e| KernelError::Serialization(e.to_string()))?;
                    regulations.push(regulation);
                }
            }
        }

        let rules_dir = lawpack_dir.join("rules");
        if rules_dir.exists() {
            for entry in WalkDir::new(&rules_dir).max_depth(1) {
                let entry = entry.map_err(|e| KernelError::Io(e.to_string()))?;
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                    let content = std::fs::read_to_string(path)
                        .map_err(|e| KernelError::Io(e.to_string()))?;
                    let rule: RuleAtom = serde_yaml::from_str(&content)
                        .map_err(|e| KernelError::Serialization(e.to_string()))?;
                    rules.push(rule);
                }
            }
        }

        let orders_dir = lawpack_dir.join("orders");
        if orders_dir.exists() {
            for entry in WalkDir::new(&orders_dir).max_depth(2) {
                let entry = entry.map_err(|e| KernelError::Io(e.to_string()))?;
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                    let content = std::fs::read_to_string(path)
                        .map_err(|e| KernelError::Io(e.to_string()))?;
                    let order: Order = serde_yaml::from_str(&content)
                        .map_err(|e| KernelError::Serialization(e.to_string()))?;
                    orders.push(order);
                }
            }
        }

        let specs_dir = lawpack_dir.join("specs");
        if specs_dir.exists() {
            for entry in WalkDir::new(&specs_dir).max_depth(1) {
                let entry = entry.map_err(|e| KernelError::Io(e.to_string()))?;
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                    let content = std::fs::read_to_string(path)
                        .map_err(|e| KernelError::Io(e.to_string()))?;
                    let spec: Spec = serde_yaml::from_str(&content)
                        .map_err(|e| KernelError::Serialization(e.to_string()))?;
                    specs.push(spec);
                }
            }
        }

        let invariants_dir = lawpack_dir.join("invariants");
        if invariants_dir.exists() {
            for entry in WalkDir::new(&invariants_dir).max_depth(1) {
                let entry = entry.map_err(|e| KernelError::Io(e.to_string()))?;
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                    let content = std::fs::read_to_string(path)
                        .map_err(|e| KernelError::Io(e.to_string()))?;
                    let raw: InvariantRaw = serde_yaml::from_str(&content)
                        .map_err(|e| KernelError::Serialization(e.to_string()))?;
                    let invariant = raw.to_invariant().map_err(KernelError::Serialization)?;
                    invariants.push(invariant);
                }
            }
        }

        let mut obligations = Vec::new();
        let obligations_dir = lawpack_dir.join("obligations");
        if obligations_dir.exists() {
            for entry in WalkDir::new(&obligations_dir).max_depth(1) {
                let entry = entry.map_err(|e| KernelError::Io(e.to_string()))?;
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                    let content = std::fs::read_to_string(path)
                        .map_err(|e| KernelError::Io(e.to_string()))?;
                    let obligation: LawpackObligation = serde_yaml::from_str(&content)
                        .map_err(|e| KernelError::Serialization(e.to_string()))?;
                    obligations.push(obligation);
                }
            }
        }

        let decisions_dir = lawpack_dir.join("decisions");
        if decisions_dir.exists() {
            for entry in WalkDir::new(&decisions_dir).max_depth(1) {
                let entry = entry.map_err(|e| KernelError::Io(e.to_string()))?;
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                    let content = std::fs::read_to_string(path)
                        .map_err(|e| KernelError::Io(e.to_string()))?;
                    let decision: Decision = serde_yaml::from_str(&content)
                        .map_err(|e| KernelError::Serialization(e.to_string()))?;
                    decisions.push(decision);
                }
            }
        }

        Ok(Lawpack {
            statutes,
            regulations,
            rules,
            orders,
            specs,
            invariants,
            decisions,
            obligations,
        })
    }
}

pub struct Lawpack {
    pub statutes: Vec<Statute>,
    pub regulations: Vec<Regulation>,
    pub rules: Vec<RuleAtom>,
    pub orders: Vec<Order>,
    pub specs: Vec<Spec>,
    pub invariants: Vec<Invariant>,
    pub decisions: Vec<Decision>,
    pub obligations: Vec<LawpackObligation>,
}

/// An obligation instrument: a standing duty enacted into the lawpack
/// (distinct from the runtime `Obligation` a route mints onto a permit).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LawpackObligation {
    pub id: String,
    pub title: String,
    pub status: String,
    pub kind: String,
    pub due: String,
    #[serde(default)]
    pub required: bool,
    pub text: String,
    #[serde(default)]
    pub basis: Vec<String>,
}

impl Lawpack {
    pub fn build_authority_graph(&self) -> Result<AuthorityGraph, KernelError> {
        let mut graph = AuthorityGraph::new();

        for statute in &self.statutes {
            for section in &statute.sections {
                let authority = Authority {
                    id: section.id.clone(),
                    kind: AuthorityKind::Statute,
                    rank: AuthorityRank::Constitutional,
                    status: statute.status.clone(),
                    jurisdiction: None,
                    title: section.title.clone(),
                    summary: section.text.clone(),
                    source_path: None,
                    issue_tags: Vec::new(),
                    scope: None,
                    supersedes: Vec::new(),
                };
                graph.authorities.insert(authority.id.clone(), authority);
            }
        }

        for regulation in &self.regulations {
            let authority = Authority {
                id: regulation.id.clone(),
                kind: AuthorityKind::Regulation,
                rank: AuthorityRank::Regulation,
                status: regulation.status.clone(),
                jurisdiction: None,
                title: regulation.title.clone(),
                summary: regulation.text.clone(),
                source_path: None,
                issue_tags: Vec::new(),
                scope: None,
                supersedes: Vec::new(),
            };
            graph.authorities.insert(authority.id.clone(), authority);
        }

        for rule in &self.rules {
            let authority = Authority::from(rule.clone());
            graph.authorities.insert(authority.id.clone(), authority);
        }

        for order in &self.orders {
            let authority = Authority {
                id: AuthorityId(order.id.clone()),
                kind: AuthorityKind::Order,
                rank: match order.court {
                    Court::SupremeCourt => AuthorityRank::SupremeCourt,
                    Court::CourtOfAppeal => AuthorityRank::CourtOfAppeal,
                    Court::PrivyCouncil => AuthorityRank::PrivyCouncil,
                    Court::County => AuthorityRank::CountyCourt,
                },
                status: order.status.clone(),
                jurisdiction: Some(order.jurisdiction.clone()),
                title: order.holding.clone(),
                summary: order.runtime_summary.clone(),
                source_path: order.source_opinion.clone(),
                issue_tags: vec![order.issue.clone()],
                scope: None,
                supersedes: Vec::new(),
            };
            graph.authorities.insert(authority.id.clone(), authority);
        }

        Ok(graph)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Statute {
    pub id: AuthorityId,
    pub citation: Option<String>,
    pub title: String,
    pub status: AuthorityStatus,
    pub enacted_by: Option<String>,
    pub purpose: Option<String>,
    pub sections: Vec<StatuteSection>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatuteSection {
    pub id: AuthorityId,
    pub title: String,
    pub text: String,
    pub commentary: Option<String>,
    pub kernel_effect: Option<KernelEffect>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Regulation {
    pub id: AuthorityId,
    pub citation: Option<String>,
    pub title: String,
    pub authority: String,
    pub status: AuthorityStatus,
    pub text: String,
    pub kernel_effect: Option<KernelEffect>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KernelEffect {
    pub when: Option<Condition>,
    pub must: Option<Vec<String>>,
    pub may: Option<Vec<String>>,
    pub must_not: Option<Vec<String>>,
    pub exceptions: Option<Vec<String>>,
    pub proof: Option<Vec<String>>,
    pub defines: Option<serde_json::Value>,
    pub prohibits: Option<Vec<String>>,
    pub status: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Condition {
    pub any: Option<Vec<String>>,
    pub all: Option<Vec<String>>,
}

/// ACT-COMPUTER-FIRST-REALM s.5 limb (a): a runtime-force record whose declared
/// kernel_effect binds to no evaluable operation is inert ceremony. A kernel_effect
/// is INERT when every modelled operation field is empty - so a block that carries
/// only unrecognized keys (e.g. a bare `force_source:` at the kernel_effect top
/// level, which serde drops) parses to an all-empty KernelEffect and is inert,
/// whereas the same `force_source` declared INSIDE a populated `defines` (as in
/// ACT-COMPUTER-FIRST-REALM s.2) is real kernel effect and is NOT inert. The guard
/// `when` is not itself an effect and does not count.
///
/// Authorized as machinery by [2026] VJS-CC 15 (Marrowby CCJ) under conditions
/// D2-D5: this is structural only (no model, no prose-reading, D4); the disposition
/// is ROUTE FOR CORRECTION, never silent exclusion or void (D2); the cure is to
/// declare a recognized operation or extend the recognized set, never to remove an
/// assented record (D3, D5). Limb (b), prose/effect divergence, stays at the agent
/// rung (s.11).
pub fn is_inert_kernel_effect(ke: &KernelEffect) -> bool {
    fn json_empty(v: &Option<serde_json::Value>) -> bool {
        match v {
            None | Some(serde_json::Value::Null) => true,
            Some(serde_json::Value::Object(m)) => m.is_empty(),
            Some(serde_json::Value::Array(a)) => a.is_empty(),
            Some(_) => false,
        }
    }
    fn vec_empty(v: &Option<Vec<String>>) -> bool {
        v.as_ref().is_none_or(|x| x.is_empty())
    }
    vec_empty(&ke.must)
        && vec_empty(&ke.may)
        && vec_empty(&ke.must_not)
        && vec_empty(&ke.exceptions)
        && vec_empty(&ke.proof)
        && vec_empty(&ke.prohibits)
        && json_empty(&ke.defines)
        && json_empty(&ke.status)
}

/// Every authority id the lawpack defines (statutes + their sections,
/// regulations, rules, orders, specs, invariants, decisions, obligations).
pub fn defined_ids(lawpack: &Lawpack) -> std::collections::HashSet<String> {
    let mut defined = std::collections::HashSet::new();
    for statute in &lawpack.statutes {
        defined.insert(statute.id.0.clone());
        for section in &statute.sections {
            defined.insert(section.id.0.clone());
        }
    }
    for regulation in &lawpack.regulations {
        defined.insert(regulation.id.0.clone());
    }
    for rule in &lawpack.rules {
        defined.insert(rule.id.0.clone());
    }
    for order in &lawpack.orders {
        defined.insert(order.id.clone());
    }
    for spec in &lawpack.specs {
        defined.insert(spec.id.0.clone());
    }
    for invariant in &lawpack.invariants {
        defined.insert(invariant.id.0.clone());
    }
    for decision in &lawpack.decisions {
        defined.insert(decision.id.0.clone());
    }
    for obligation in &lawpack.obligations {
        defined.insert(obligation.id.clone());
    }
    defined
}

/// Every case/instrument citation the lawpack defines (orders, statutes, and
/// regulations all carry a top-level citation), whitespace-normalised for stable keying
/// (PC-17 D1, the order citation-grounding gate). A holding citing "[2026] VJS-ACT 10"
/// (a statute) must resolve as surely as one citing "[2026] VJS-PC 16" (an order).
pub fn defined_citations(lawpack: &Lawpack) -> std::collections::HashSet<String> {
    let norm = |c: &str| c.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut set = std::collections::HashSet::new();
    for o in &lawpack.orders {
        if let Some(c) = &o.citation {
            set.insert(norm(c));
        }
    }
    for s in &lawpack.statutes {
        if let Some(c) = &s.citation {
            set.insert(norm(c));
        }
    }
    for r in &lawpack.regulations {
        if let Some(c) = &r.citation {
            set.insert(norm(c));
        }
    }
    set
}

/// The ids any order declares superseded - so a reference to one is defined-but-not-in-
/// force (an advisory under PC-17 D3), not unresolved.
pub fn superseded_ids(lawpack: &Lawpack) -> std::collections::HashSet<String> {
    lawpack
        .orders
        .iter()
        .flat_map(|o| o.supersedes.iter().map(|s| s.0.clone()))
        .collect()
}

/// Build the whole-lawpack facts the staged invariant evaluator needs. Reuses
/// the validator's findings (so a check change in one place changes both) and
/// reads the repo's `.vjs/config.toml` for the directory-roles and MCP checks.
pub fn lawpack_facts(repo_root: &Path, lawpack: &Lawpack) -> LawpackFacts {
    let report = LawpackValidator::validate(lawpack).unwrap_or(ValidationReport {
        ok: false,
        findings: Vec::new(),
    });
    let duplicate_ids = report.findings.iter().any(|f| f.code == "DUPLICATE_ID");
    let duplicate_citations = report
        .findings
        .iter()
        .any(|f| f.code == "CITATION_COLLISION");
    let config = std::fs::read_to_string(repo_root.join(".vjs/config.toml")).unwrap_or_default();
    // Every quoted path value under the roles config must stay inside the repo
    // (no `..` escape, no absolute path outside the tree).
    let directory_roles_resolve = config
        .lines()
        .filter(|l| l.contains('=') && l.contains('"'))
        .filter_map(|l| l.split_once('='))
        .map(|(_k, v)| v.trim().trim_matches('"').to_string())
        .filter(|v| !v.is_empty())
        .all(|v| !v.contains("..") && !Path::new(&v).is_absolute());
    // MCP is local-first unless the config declares a public/non-loopback bind.
    let mcp_local_first =
        !config.contains("0.0.0.0") && !config.to_lowercase().contains("bind_public");
    LawpackFacts {
        validates: report.ok,
        duplicate_ids,
        duplicate_citations,
        all_ids: defined_ids(lawpack),
        mcp_local_first,
        directory_roles_resolve,
    }
}


mod conformance;
mod report;
mod validator;

pub use conformance::*;
pub use report::*;
pub use validator::*;

#[cfg(test)]
mod citation_tests {
    use super::LawpackValidator as V;

    #[test]
    fn parses_canon_and_subscriber_citations() {
        assert_eq!(
            V::parse_citation("[2026] VJS-PC 13"),
            Some((2026, "PC".into(), None, 13))
        );
        assert_eq!(
            V::parse_citation("[2026] VJS-DEC 15"),
            Some((2026, "DEC".into(), None, 15))
        );
        assert_eq!(
            V::parse_citation("[2026] VJS-CC-OPBOX 79"),
            Some((2026, "CC".into(), Some("OPBOX".into()), 79))
        );
    }

    #[test]
    fn rejects_non_citations() {
        assert_eq!(V::parse_citation("DEC-OPBOX-UNITARY-001"), None);
        assert_eq!(V::parse_citation("not a citation"), None);
        assert_eq!(V::parse_citation("[2026] VJS-PC"), None);
    }
}
