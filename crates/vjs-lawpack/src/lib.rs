use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use vjs_core::*;
use vjs_core::types::*;
use vjs_core::error::*;
use vjs_core::spec::InvariantRaw;

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
                    let invariant = raw.to_invariant()
                        .map_err(|e| KernelError::Serialization(e))?;
                    invariants.push(invariant);
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

pub struct LawpackValidator;

impl LawpackValidator {
    pub fn validate(lawpack: &Lawpack) -> Result<ValidationReport, KernelError> {
        let mut findings = Vec::new();
        let mut ok = true;

        // Check for duplicate IDs
        let mut ids = std::collections::HashSet::new();
        for statute in &lawpack.statutes {
            for section in &statute.sections {
                if !ids.insert(section.id.0.clone()) {
                    findings.push(ValidationFinding {
                        severity: Severity::Error,
                        code: "DUPLICATE_ID".into(),
                        path: None,
                        message: format!("Duplicate authority ID: {}", section.id.0),
                        suggested_fix: Some("Change the ID to a unique value".into()),
                    });
                    ok = false;
                }
            }
        }

        // Check for citation collisions
        let mut citations = std::collections::HashSet::new();
        for order in &lawpack.orders {
            if !citations.insert(order.id.clone()) {
                findings.push(ValidationFinding {
                    severity: Severity::Error,
                    code: "CITATION_COLLISION".into(),
                    path: None,
                    message: format!("Duplicate order ID: {}", order.id),
                    suggested_fix: Some("Use vjs next-citation to get a unique citation".into()),
                });
                ok = false;
            }
        }

        // Check supersession targets exist
        for order in &lawpack.orders {
            for superseded in &order.supersedes {
                if !lawpack.orders.iter().any(|o| o.id == superseded.0) {
                    findings.push(ValidationFinding {
                        severity: Severity::Warning,
                        code: "ORPHAN_SUPERSESSION".into(),
                        path: None,
                        message: format!(
                            "Order {} supersedes non-existent authority {}",
                            order.id, superseded.0
                        ),
                        suggested_fix: Some("Verify the superseded ID exists".into()),
                    });
                }
            }
        }

        // Check text present in statute sections
        for statute in &lawpack.statutes {
            for section in &statute.sections {
                if section.text.is_empty() {
                    findings.push(ValidationFinding {
                        severity: Severity::Error,
                        code: "MISSING_TEXT".into(),
                        path: None,
                        message: format!("Section {} has no text", section.id.0),
                        suggested_fix: Some("Add text field with binding legal text".into()),
                    });
                    ok = false;
                }
                if section.kernel_effect.is_none() {
                    findings.push(ValidationFinding {
                        severity: Severity::Warning,
                        code: "NO_KERNEL_EFFECT".into(),
                        path: None,
                        message: format!("Section {} has no kernel_effect", section.id.0),
                        suggested_fix: Some("Add kernel_effect with structured executable rules".into()),
                    });
                }
            }
        }

        // Check text present in regulations
        for regulation in &lawpack.regulations {
            if regulation.text.is_empty() {
                findings.push(ValidationFinding {
                    severity: Severity::Error,
                    code: "MISSING_TEXT".into(),
                    path: None,
                    message: format!("Regulation {} has no text", regulation.id.0),
                    suggested_fix: Some("Add text field with binding legal text".into()),
                });
                ok = false;
            }
        }

        // Check word limits
        for order in &lawpack.orders {
            let word_count = order.runtime_summary.split_whitespace().count();
            if word_count > 500 {
                findings.push(ValidationFinding {
                    severity: Severity::Error,
                    code: "WORD_LIMIT_EXCEEDED".into(),
                    path: None,
                    message: format!(
                        "Order {} runtime summary exceeds word limit: {}/500",
                        order.id, word_count
                    ),
                    suggested_fix: Some("Shorten the runtime_summary to 500 words max".into()),
                });
                ok = false;
            }
        }

        Ok(ValidationReport { ok, findings })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValidationReport {
    pub ok: bool,
    pub findings: Vec<ValidationFinding>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValidationFinding {
    pub severity: Severity,
    pub code: String,
    pub path: Option<PathBuf>,
    pub message: String,
    pub suggested_fix: Option<String>,
}
