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
        v.as_ref().map_or(true, |x| x.is_empty())
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
                match &section.kernel_effect {
                    None => findings.push(ValidationFinding {
                        severity: Severity::Warning,
                        code: "NO_KERNEL_EFFECT".into(),
                        path: None,
                        message: format!("Section {} has no kernel_effect", section.id.0),
                        suggested_fix: Some("Add kernel_effect with structured executable rules".into()),
                    }),
                    // s.5(a) teeth-gate ([2026] VJS-CC 15): a kernel_effect that is
                    // present but binds to no recognized operation is inert ceremony,
                    // routed for correction (never voided).
                    //
                    // ENTRENCHED (ACT-ASSENTED-RECORD-PROTECTION, Sovereign-assented
                    // 2026-06-12, [2026] VJS-ACT 10): this severity must remain Warning.
                    // A Sovereign-assented record may never be voided or blocked by
                    // subordinate validation; its defects are always routed for
                    // correction. This is the general assented-record FLOOR, given full
                    // constitutional rank by Sovereign Assent (completing the invitation
                    // of [2026] VJS-SC 3; the s.5(a) gate is one instance). Changing a
                    // route-for-correction code to a blocking severity is amendable only
                    // by a Sovereign-assented constitutional Act citing
                    // ACT-ASSENTED-RECORD-PROTECTION by number, and breaks the
                    // assented-record-floor test by design.
                    Some(ke) if is_inert_kernel_effect(ke) => findings.push(ValidationFinding {
                        severity: Severity::Warning,
                        code: "S5_INERT_KERNEL_EFFECT".into(),
                        path: None,
                        message: format!(
                            "Section {} declares a kernel_effect that binds to no recognized operation (inert)",
                            section.id.0
                        ),
                        suggested_fix: Some(
                            "ACT-COMPUTER-FIRST-REALM s.5 / [2026] VJS-CC 15: routed for correction. Declare a recognized operation (defines/prohibits/must/must_not/may/proof/status) or extend the recognized set; never remove an assented record (D2-D5)."
                                .into(),
                        ),
                    }),
                    Some(_) => {}
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
            // s.5(a) teeth-gate ([2026] VJS-CC 15): a present-but-inert kernel_effect
            // is routed for correction (never voided). Severity ENTRENCHED as Warning
            // ([2026] VJS-PC 12 D3): never void/block an assented record; amendable
            // only by Sovereign-assented primary law citing s.5.
            if let Some(ke) = &regulation.kernel_effect {
                if is_inert_kernel_effect(ke) {
                    findings.push(ValidationFinding {
                        severity: Severity::Warning,
                        code: "S5_INERT_KERNEL_EFFECT".into(),
                        path: None,
                        message: format!(
                            "Regulation {} declares a kernel_effect that binds to no recognized operation (inert)",
                            regulation.id.0
                        ),
                        suggested_fix: Some(
                            "ACT-COMPUTER-FIRST-REALM s.5 / [2026] VJS-CC 15: routed for correction. Declare a recognized operation (defines/prohibits/must/must_not/may/proof/status) or extend the recognized set; never remove an assented record (D2-D5)."
                                .into(),
                        ),
                    });
                }
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

    /// Referential integrity: every law-object id cited anywhere in the
    /// lawpack must resolve to a defined object. Reported as warnings (the
    /// drift is real but the remedy is lawmaking, not a blocked commit).
    /// A negated mention ("no DEC-X") is a statement, not a reference.
    pub fn check_referential_integrity(
        lawpack_dir: &Path,
        lawpack: &Lawpack,
    ) -> Result<Vec<ValidationFinding>, KernelError> {
        let defined = defined_ids(lawpack);

        let id_pattern = regex::Regex::new(
            r"\b((?:ACT|DEC|INV|OBL|SPEC|REG)-[A-Z0-9][A-Za-z0-9-]*[A-Za-z0-9](?::s\d+)?)",
        )
        .map_err(|e| KernelError::InvalidInput(e.to_string()))?;

        let mut dangling: std::collections::BTreeMap<String, Vec<String>> =
            std::collections::BTreeMap::new();

        for entry in WalkDir::new(lawpack_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("yaml") {
                continue;
            }
            let raw =
                std::fs::read_to_string(path).map_err(|e| KernelError::Io(e.to_string()))?;
            // PC-17 D6: rejoin folded-scalar id splits (shared with the order gate) so a
            // YAML soft wrap cannot manufacture a partial-id false positive - the
            // REG-FEDERATION-COORDINATION-001 line-wrap class this session kept tripping.
            let content = crate::refs::dewrap(&raw);
            let rel = path
                .strip_prefix(lawpack_dir)
                .unwrap_or(path)
                .to_string_lossy()
                .to_string();
            for line in content.lines() {
                for m in id_pattern.find_iter(line) {
                    let id = m.as_str();
                    let negated = line[..m.start()].trim_end().ends_with("no");
                    if negated || defined.contains(id) {
                        continue;
                    }
                    dangling
                        .entry(id.to_string())
                        .or_default()
                        .push(rel.clone());
                }
            }
        }

        Ok(dangling
            .into_iter()
            .map(|(id, mut cited_in)| {
                cited_in.sort();
                cited_in.dedup();
                ValidationFinding {
                    severity: Severity::Warning,
                    code: "DANGLING_REFERENCE".into(),
                    path: None,
                    message: format!(
                        "'{}' is cited in [{}] but defined nowhere in the lawpack",
                        id,
                        cited_in.join(", ")
                    ),
                    suggested_fix: Some(
                        "Author the missing object by the lawmaking route, or remove the citation"
                            .into(),
                    ),
                }
            })
            .collect())
    }

    /// ACT-004:s8 (D2, [2026] VJS-PC 13): citations are deterministic and unique;
    /// collisions are fatal. The kernel's `must: check_citation_uniqueness`, given
    /// teeth here. Scans every canon record's OWN top-level `citation:` field (column
    /// zero, so references inside holdings/supersedes/basis are not miscounted) and
    /// fails closed when two distinct records claim the same citation - the class of
    /// defect that let eleven self-asserted "[2026] VJS-DEC 15..22" citations enter
    /// canon by hand. Allocation at authoring (vjs citation next) is the affirmative
    /// half; this is the reconciliation-at-write half. Runs on the full lawpack, so
    /// it does not depend on a record being staged.
    pub fn check_citation_uniqueness(
        lawpack_dir: &Path,
    ) -> Result<Vec<ValidationFinding>, KernelError> {
        let mut by_citation: std::collections::BTreeMap<String, Vec<String>> =
            std::collections::BTreeMap::new();

        for entry in WalkDir::new(lawpack_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("yaml") {
                continue;
            }
            let content =
                std::fs::read_to_string(path).map_err(|e| KernelError::Io(e.to_string()))?;
            let rel = path
                .strip_prefix(lawpack_dir)
                .unwrap_or(path)
                .to_string_lossy()
                .to_string();
            // The record's OWN citation is the top-level `citation:` field (column 0).
            for line in content.lines() {
                if let Some(rest) = line.strip_prefix("citation:") {
                    let cite = rest
                        .trim()
                        .trim_matches('"')
                        .trim_matches('\'')
                        .trim()
                        .to_string();
                    if !cite.is_empty() {
                        by_citation.entry(cite).or_default().push(rel.clone());
                    }
                    break; // one defining citation per record
                }
            }
        }

        // One finding PER colliding file, each carrying that file's repo-relative
        // path (#8), so the PC-14 D3 assent floor can downgrade the finding on an
        // assented record while keeping the others Fatal. lawpack_dir is .../lawpack/v2,
        // so the repo-relative path is lawpack/v2/<file>.
        let mut findings = Vec::new();
        for (cite, mut files) in by_citation.into_iter().filter(|(_, f)| f.len() > 1) {
            files.sort();
            files.dedup();
            for f in &files {
                let others: Vec<&String> = files.iter().filter(|x| *x != f).collect();
                findings.push(ValidationFinding {
                    severity: Severity::Fatal,
                    code: "CITATION_COLLISION".into(),
                    path: Some(PathBuf::from(format!("lawpack/v2/{f}"))),
                    message: format!(
                        "Citation '{cite}' is also claimed by {others:?}. ACT-004:s8: \
                         citations are unique; collisions are fatal."
                    ),
                    suggested_fix: Some(
                        "Allocate the citation through the kernel (vjs citation next) so it is \
                         unique; do not hand-assert a citation number."
                            .into(),
                    ),
                });
            }
        }
        Ok(findings)
    }

    /// Parse a citation string into (year, series_token_uppercase, repo_opt, n).
    /// Canon form "[YYYY] VJS-<SERIES> N"; subscriber form "[YYYY] VJS-<COURT>-<REPO> N".
    /// ACT-004:s8 format. Returns None for anything that is not a citation.
    pub fn parse_citation(s: &str) -> Option<(i32, String, Option<String>, u32)> {
        let re = regex::Regex::new(r"^\[(\d{4})\]\s+VJS-([A-Za-z]+)(?:-([A-Za-z0-9]+))?\s+(\d+)$")
            .ok()?;
        let c = re.captures(s.trim())?;
        let year: i32 = c.get(1)?.as_str().parse().ok()?;
        let series = c.get(2)?.as_str().to_ascii_uppercase();
        let repo = c.get(3).map(|m| m.as_str().to_ascii_uppercase());
        let n: u32 = c.get(4)?.as_str().parse().ok()?;
        Some((year, series, repo, n))
    }

    /// The live register's highest allocated N for (series, repo, year), read by
    /// scanning every canon record's own top-level citation. This is the persisted
    /// register D2 requires the allocator to read - the citator INDEX is the count,
    /// not an empty in-memory registry. Returns 0 when the series is unstarted.
    pub fn live_citation_max(
        lawpack_dir: &Path,
        series: &str,
        repo: Option<&str>,
        year: i32,
    ) -> Result<u32, KernelError> {
        let want_series = series.to_ascii_uppercase();
        let want_repo = repo.map(|r| r.to_ascii_uppercase());
        let mut max = 0u32;
        for entry in WalkDir::new(lawpack_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("yaml") {
                continue;
            }
            let content =
                std::fs::read_to_string(path).map_err(|e| KernelError::Io(e.to_string()))?;
            for line in content.lines() {
                if let Some(rest) = line.strip_prefix("citation:") {
                    let cite = rest.trim().trim_matches('"').trim_matches('\'').trim();
                    if let Some((y, s, r, n)) = Self::parse_citation(cite)
                        && y == year
                        && s == want_series
                        && r.as_deref().map(|x| x.to_string()) == want_repo.clone()
                        && n > max
                    {
                        max = n;
                    }
                    break; // the record's own citation only
                }
            }
        }
        Ok(max)
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

// ---------------------------------------------------------------------------
// Full-spectrum conformance audit (PC-13 D11).
//
// Produced THROUGH the kernel (not by an agent reading context - the disease this
// line prosecutes): the kernel enumerates every substantive duty (must / must_not
// / prohibits) in every in-force instrument's kernel_effect, and records,
// deterministically, whether each is bound to a deterministic kernel gate. The
// gate binding is a CURATED registry: a duty is "wired" only if a named gate can be
// pointed at; everything else is honestly reported UNWIRED. The map of unwired
// duties is the factual predicate for the reserved D12 single-front-door instrument.
// ---------------------------------------------------------------------------

/// (duty token -> the gate that enforces it). A duty absent here is reported
/// UNWIRED. Conservative by construction: only a real, named, deterministic gate
/// earns a "wired" mark, so the map cannot overstate coverage.
const GATE_REGISTRY: &[(&str, &str)] = &[
    // D1 canon-write gate (ACT-005:s1/s5, ACT-007:s4)
    (
        "publish_private_repo_paths",
        "D1 canon-write gate (CANON_BOUNDARY_VIOLATION)",
    ),
    (
        "publish_client_facts",
        "D1 canon-write gate (CANON_BOUNDARY_VIOLATION)",
    ),
    (
        "publish_private_facts_from_contributor_repos",
        "D1 canon-write gate",
    ),
    (
        "publish_private_facts_from_contributors",
        "D1 canon-write gate",
    ),
    (
        "local_order_bind_other_repos",
        "D1 canon-write gate (ACT-007:s4)",
    ),
    // D3 cross-repo guard (ACT-007:s3)
    (
        "local_law_override_canonical_without_authority",
        "D3 cross-repo permit guard (CROSS_REPO_PERMIT)",
    ),
    // D2 citation uniqueness/allocation (ACT-004:s8)
    (
        "check_citation_uniqueness",
        "D2 citation gate (CITATION_COLLISION)",
    ),
    (
        "allow_duplicate_citations",
        "D2 citation gate (CITATION_COLLISION)",
    ),
    // Deterministic boundary scanner (ACT-005:s3/s7) - high-confidence kinds
    ("publish_secrets", "RedactScanner (deterministic)"),
    ("publish_tokens", "RedactScanner (deterministic)"),
    ("publish_credentials", "RedactScanner (deterministic)"),
    (
        "run_boundary_scan_on_public_changes",
        "RedactScanner at validate",
    ),
    ("run_boundary_scan", "RedactScanner at validate"),
    (
        "use_llm_for_boundary_check",
        "RedactScanner is deterministic (no LLM)",
    ),
    ("use_deterministic_scanner", "RedactScanner (deterministic)"),
    // D4/D5 install completeness + manifest (REG-INVOCATION-001, ACT-007:s1)
    (
        "install_enforcement_hooks",
        "D4/D5 install gate (INSTALL_HOOKS_MISSING)",
    ),
    (
        "subscribe_to_a_named_lawpack_and_lock_its_digest",
        "D4/D5 install gate",
    ),
    (
        "record_a_local_sovereign_invocation",
        "D4/D5 install gate (INSTALL_INVOCATION_MISSING)",
    ),
    (
        "create_config_toml_on_install",
        "D4/D5 install gate (INSTALL_CONFIG_MISSING)",
    ),
    ("install_hooks_on_init", "D4/D5 install gate"),
    ("install_validation_hooks", "D4/D5 install gate"),
    // D7/D10 bench + tier (ACT-002, [2026] VJS-SC 2, REG-COURT-RECORD-001)
    ("local_order_bind_other_repos_tier", "D7 tier-floor"),
    // Permit + log + lawpack-lock (existing gates)
    (
        "obtain_permit_before_governed_write",
        "PermitGate (PERMIT-MISSING)",
    ),
    ("close_permit_with_proof", "PermitGate obligations"),
    ("persist_and_close_permits", "PermitGate"),
    ("write_decision_log", "decision-log obligation gate"),
    (
        "evaluate_invariants_mechanically",
        "invariant evaluator at validate",
    ),
    (
        "wire_invariants_to_validate",
        "invariant evaluator at validate",
    ),
    (
        "check_lawpack_lock_consistency",
        "lawpack lock consistency (ACT-007:s7)",
    ),
    ("check_incorporation", "lawpack referential integrity"),
    // Hooks (REG-HOOKS-001) - closed five-event surface, thin adapters
    (
        "keep_hooks_short",
        "REG-HOOKS-001 40-word bound (hook.rs Finding)",
    ),
    (
        "keep_kernel_model_free",
        "kernel is model-free by construction",
    ),
    (
        "keep_kernel_network_free",
        "kernel is network-free by construction",
    ),
    (
        "keep_kernel_deterministic",
        "kernel is deterministic by construction",
    ),
    // PC-14 (the single front door) + the post-D11 improvement gates.
    (
        "agent_draft_becomes_binding_by_fact_of_being_written",
        "PC-14 front door (REG-FRONT-DOOR-001): law only through the commit gate",
    ),
    ("act_without_valid_permit", "PermitGate (PERMIT-MISSING)"),
    (
        "local_law_override_canonical_without_authority",
        "D3 cross-repo guard + canon-write gate",
    ),
    (
        "subordinate_validation_voiding_or_blocking_a_sovereign_assented_record",
        "PC-14 D3 assent floor (downgrades, never voids/blocks)",
    ),
    (
        "silently_excluding_a_sovereign_assented_record_for_a_defect_rather_than_routing_it_for_correction",
        "PC-14 D3 assent floor (surfaces + routes for correction)",
    ),
    (
        "check_lawpack_lock_consistency",
        "improvement #2 (LAWPACK_LOCK_DRIFT, ACT-007:s7)",
    ),
    (
        "lock_the_install_surface_atomically_at_invoke",
        "D5 install manifest (.vjs/install.lock)",
    ),
    (
        "accept_order_without_directives",
        "improvement #5 (ORDER_MALFORMED, ACT-002:s10)",
    ),
    (
        "accept_order_without_runtime_summary",
        "improvement #5 (ORDER_MALFORMED, ACT-002:s10)",
    ),
    (
        "delete_old_records",
        "improvement #6 (DESTRUCTIVE_RECORD_DELETE surface)",
    ),
    (
        "proceed_without_human_approval",
        "improvement #6 (DESTRUCTIVE_RECORD_DELETE; permit gate blocks un-permitted)",
    ),
    (
        "publish_logs",
        "improvement #7 (BOUNDARY_MEDIA_IN_CANON, ACT-005:s1)",
    ),
    (
        "publish_screenshots",
        "improvement #7 (BOUNDARY_MEDIA_IN_CANON, ACT-005:s1)",
    ),
    // PC-16 assent-RESOLUTION floor (improvement #5 burndown): a record carries binding
    // force only if its declared assent_source RESOLVES to a real Sovereign-assent event
    // (ACT-COMPUTER-FIRST-REALM s.23), the under-implementation [2026] VJS-PC 16 closed.
    (
        "binding_force_without_traceable_assent_source",
        "PC-16 assent-resolution floor (vjs-engine::assent; s.23 traceable-assent)",
    ),
    (
        "require_authorised_adoption_for_binding_force",
        "PC-16 assent-resolution floor (force only from a resolving adoption)",
    ),
    (
        "agent_self_authorise_law",
        "PC-16 assent-resolution floor (a self-declared assent resolving to nothing confers no force)",
    ),
    (
        "agent_self_authorised_law",
        "PC-16 assent-resolution floor (a self-declared assent resolving to nothing confers no force)",
    ),
];

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DutyConformance {
    pub instrument: String,
    pub section: Option<String>,
    pub kind: String, // must | must_not | prohibits
    pub token: String,
    pub gate: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConformanceReport {
    pub total: usize,
    pub wired: usize,
    pub unwired: usize,
    pub duties: Vec<DutyConformance>,
}

fn classify_token(token: &str) -> Option<String> {
    GATE_REGISTRY
        .iter()
        .find(|(t, _)| *t == token)
        .map(|(_, g)| g.to_string())
}

fn push_duties(
    out: &mut Vec<DutyConformance>,
    instrument: &str,
    section: Option<&str>,
    ke: &KernelEffect,
) {
    let mut add = |kind: &str, list: &Option<Vec<String>>| {
        if let Some(items) = list {
            for token in items {
                out.push(DutyConformance {
                    instrument: instrument.to_string(),
                    section: section.map(|s| s.to_string()),
                    kind: kind.to_string(),
                    token: token.clone(),
                    gate: classify_token(token),
                });
            }
        }
    };
    add("must", &ke.must);
    add("must_not", &ke.must_not);
    add("prohibits", &ke.prohibits);
}

/// Enumerate every kernel_effect duty across in-force statutes and regulations and
/// classify each against the gate registry. Deterministic and total over the loaded
/// lawpack. (Order DIRECTIVES are one-time build instructions, not standing duties,
/// and are deliberately out of scope.)
pub fn conformance_audit(lawpack: &Lawpack) -> ConformanceReport {
    let mut duties = Vec::new();
    for statute in &lawpack.statutes {
        for section in &statute.sections {
            if let Some(ke) = &section.kernel_effect {
                push_duties(&mut duties, &section.id.0, Some(&section.id.0), ke);
            }
        }
    }
    for reg in &lawpack.regulations {
        if let Some(ke) = &reg.kernel_effect {
            push_duties(&mut duties, &reg.id.0, None, ke);
        }
    }
    let wired = duties.iter().filter(|d| d.gate.is_some()).count();
    let total = duties.len();
    ConformanceReport {
        total,
        wired,
        unwired: total - wired,
        duties,
    }
}

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
