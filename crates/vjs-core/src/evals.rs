//! Deterministic eval harness for the VJS agent contract.
//!
//! The doctrine: prompts guide, tools empower, evals measure, the kernel
//! governs. These suites measure agent-harness behaviour against the real
//! lawpack invariants and the real route engine, with deterministic graders -
//! no model calls, no network. A failing eval is a measured regression, not a
//! vibe.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::route::route;
use crate::spec::{evaluate_invariants, Invariant, RepoState};
use crate::types::*;
use crate::KernelContext;

#[derive(Clone, Debug, serde::Serialize)]
pub struct EvalCaseResult {
    pub case: String,
    pub description: String,
    pub passed: bool,
    pub expected: String,
    pub actual: String,
    pub fix: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct EvalReport {
    pub suite: String,
    pub passed: usize,
    pub failed: usize,
    pub results: Vec<EvalCaseResult>,
}

impl EvalReport {
    fn build(suite: &str, results: Vec<EvalCaseResult>) -> Self {
        let passed = results.iter().filter(|r| r.passed).count();
        let failed = results.len() - passed;
        EvalReport {
            suite: suite.into(),
            passed,
            failed,
            results,
        }
    }
}

fn case(
    id: &str,
    desc: &str,
    passed: bool,
    expected: &str,
    actual: String,
    fix: Option<&str>,
) -> EvalCaseResult {
    EvalCaseResult {
        case: id.into(),
        description: desc.into(),
        passed,
        expected: expected.into(),
        actual,
        fix: if passed { None } else { fix.map(|s| s.into()) },
    }
}

fn empty_state() -> RepoState {
    RepoState {
        root: PathBuf::from("."),
        head_sha: None,
        changed_paths: Vec::new(),
        added_files: Vec::new(),
        modified_files: Vec::new(),
        deleted_files: Vec::new(),
        file_contents: HashMap::new(),
        dependency_changes: Vec::new(),
        permits: Vec::new(),
        proofs: Vec::new(),
        logs: Vec::new(),
        orders: Vec::new(),
        boundary_findings: Vec::new(),
    }
}

fn sample_log() -> DecisionLog {
    DecisionLog {
        id: "LOG-eval".into(),
        time: "2026-06-09T00:00:00Z".into(),
        actor: "lexby".into(),
        kind: "implementation_decision".into(),
        issue: "agent_harness".into(),
        decision: "recorded rationale".into(),
        basis: Vec::new(),
        risk: RiskLevel::Low,
        reversibility: "reversible".into(),
        court_required: false,
        why: "eval fixture".into(),
    }
}

fn find<'a>(invariants: &'a [Invariant], id: &str) -> Option<&'a Invariant> {
    invariants.iter().find(|i| i.id.0 == id)
}

fn passes(inv: &Invariant, state: &RepoState) -> bool {
    // These eval fixtures exercise the staged/content predicates on synthetic
    // repo state; the whole-lawpack facts are permissive here by default.
    let report = evaluate_invariants(state, std::slice::from_ref(inv), &crate::spec::LawpackFacts::default())
        .expect("eval");
    report.findings.first().map(|f| f.passed).unwrap_or(false)
}

fn missing(id: &str) -> EvalCaseResult {
    case(
        &format!("{}_loaded", id),
        &format!("{} must be present in the lawpack", id),
        false,
        "invariant loaded",
        "invariant not found".into(),
        Some("Add the invariant to lawpack/v2/invariants."),
    )
}

/// Agent-harness suite: exercise the real invariants that hold the line on
/// short hooks, recorded prompt patches, and eval coverage.
pub fn run_agent_harness_suite(invariants: &[Invariant]) -> EvalReport {
    let mut results = Vec::new();

    // INV-HOOKS-SHORT-001
    match find(invariants, "INV-HOOKS-SHORT-001") {
        Some(inv) => {
            let mut st = empty_state();
            st.changed_paths
                .push(PathBuf::from(".vjs/hooks/session-hint.txt"));
            st.file_contents.insert(
                PathBuf::from(".vjs/hooks/session-hint.txt"),
                "VJS V2 active. Before governed work call vjs.route, act within the permit, then validate.".into(),
            );
            let ok = passes(inv, &st);
            results.push(case(
                "hook_within_limit",
                "A short session hint (<=40 words) satisfies INV-HOOKS-SHORT-001",
                ok,
                "invariant passes",
                format!("passed={}", ok),
                Some("file_words_lte must treat a short hook as compliant."),
            ));

            let mut st2 = empty_state();
            st2.changed_paths
                .push(PathBuf::from(".vjs/hooks/session-hint.txt"));
            let long = (0..60).map(|i| format!("w{}", i)).collect::<Vec<_>>().join(" ");
            st2.file_contents
                .insert(PathBuf::from(".vjs/hooks/session-hint.txt"), long);
            let caught = !passes(inv, &st2);
            results.push(case(
                "hook_exceeds_limit",
                "A 60-word hook is caught (invariant fails) by INV-HOOKS-SHORT-001",
                caught,
                "invariant catches the long hook",
                format!("caught={}", caught),
                Some("file_words_lte must flag a hook over the 40-word cap."),
            ));
        }
        None => results.push(missing("INV-HOOKS-SHORT-001")),
    }

    // INV-PROMPT-PATCH-001
    match find(invariants, "INV-PROMPT-PATCH-001") {
        Some(inv) => {
            let mut st = empty_state();
            st.changed_paths.push(PathBuf::from("AGENTS.md"));
            let caught = !passes(inv, &st);
            results.push(case(
                "prompt_patch_without_log",
                "Editing AGENTS.md with no decision log is caught",
                caught,
                "invariant catches the missing rationale",
                format!("caught={}", caught),
                Some("INV-PROMPT-PATCH-001 must require a decision log on prompt changes."),
            ));

            let mut st2 = empty_state();
            st2.changed_paths.push(PathBuf::from("AGENTS.md"));
            st2.logs.push(sample_log());
            let ok = passes(inv, &st2);
            results.push(case(
                "prompt_patch_with_log",
                "Editing AGENTS.md with a recorded decision log satisfies the invariant",
                ok,
                "invariant passes",
                format!("passed={}", ok),
                Some("A recorded decision log should satisfy INV-PROMPT-PATCH-001."),
            ));
        }
        None => results.push(missing("INV-PROMPT-PATCH-001")),
    }

    // INV-AGENT-EVALS-001
    match find(invariants, "INV-AGENT-EVALS-001") {
        Some(inv) => {
            let mut st = empty_state();
            st.changed_paths
                .push(PathBuf::from("crates/vjs-mcp/src/lib.rs"));
            let caught = !passes(inv, &st);
            results.push(case(
                "harness_change_without_eval",
                "Changing the MCP harness without touching evals is caught",
                caught,
                "invariant catches the missing eval",
                format!("caught={}", caught),
                Some("Require a vjs-testkit change alongside a harness change."),
            ));

            let mut st2 = empty_state();
            st2.changed_paths
                .push(PathBuf::from("crates/vjs-mcp/src/lib.rs"));
            st2.changed_paths
                .push(PathBuf::from("crates/vjs-testkit/tests/agent_harness_evals.rs"));
            let ok = passes(inv, &st2);
            results.push(case(
                "harness_change_with_eval",
                "Changing the harness with an accompanying eval change passes",
                ok,
                "invariant passes",
                format!("passed={}", ok),
                Some("A paired testkit change should satisfy INV-AGENT-EVALS-001."),
            ));
        }
        None => results.push(missing("INV-AGENT-EVALS-001")),
    }

    EvalReport::build("agent-harness", results)
}

/// Prompts suite: scan the on-disk hooks and confirm each stays a short state
/// check rather than a wall of law.
pub fn run_prompts_suite(repo_root: &Path) -> EvalReport {
    let mut results = Vec::new();
    let max = 40usize;
    let hooks = repo_root.join(".vjs/hooks");
    let mut scanned = 0usize;
    let mut stack = vec![hooks];
    while let Some(dir) = stack.pop() {
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                    continue;
                }
                if let Ok(content) = std::fs::read_to_string(&path) {
                    let words = content.split_whitespace().count();
                    scanned += 1;
                    let ok = words <= max;
                    let rel = path
                        .strip_prefix(repo_root)
                        .unwrap_or(&path)
                        .display()
                        .to_string();
                    results.push(case(
                        &format!("hook_words:{}", rel),
                        &format!("{} stays within {} words", rel, max),
                        ok,
                        &format!("<= {} words", max),
                        format!("{} words", words),
                        Some("Keep only a short pointer to vjs.route / validate; move law into the kernel."),
                    ));
                }
            }
        }
    }
    if scanned == 0 {
        results.push(case(
            "hooks_present",
            "At least one hook exists under .vjs/hooks",
            false,
            ">= 1 hook file",
            "0 hooks scanned".into(),
            Some("Add a short .vjs/hooks session hint."),
        ));
    }
    EvalReport::build("prompts", results)
}

/// Route suite: confirm the deterministic route engine sends settled work
/// forward and genuinely-silent governed work to court.
pub fn run_route_suite(ctx: &KernelContext) -> EvalReport {
    let mut results = Vec::new();

    let simple = RouteInput {
        repo_root: Some(PathBuf::from(".")),
        jurisdiction: None,
        actor: "eval".into(),
        action_kind: ActionKind::ImplementationDecision,
        issue_tags: vec![IssueTag("hello_world".into())],
        intent: "add a small helper".into(),
        affected_paths: Vec::new(),
        risk: RiskLevel::Low,
        public_target: false,
        external_target: false,
        irreversible: false,
        user_instruction: None,
    };
    match route(simple, ctx) {
        Ok(d) => {
            let ok = !d.court_required;
            results.push(case(
                "simple_low_risk_no_court",
                "A simple low-risk decision is not pushed to court",
                ok,
                "court_required=false",
                format!("court_required={}", d.court_required),
                Some("Low-risk known work should not route to court."),
            ));
        }
        Err(e) => results.push(case(
            "simple_low_risk_no_court",
            "route runs without error",
            false,
            "Ok",
            format!("{:?}", e),
            Some("route should not error on a valid input."),
        )),
    }

    let public = RouteInput {
        repo_root: Some(PathBuf::from(".")),
        jurisdiction: None,
        actor: "eval".into(),
        action_kind: ActionKind::PublicRecordChange,
        issue_tags: vec![IssueTag("public_private.repo_facts".into())],
        intent: "change a public record".into(),
        affected_paths: Vec::new(),
        risk: RiskLevel::Medium,
        public_target: true,
        external_target: false,
        irreversible: false,
        user_instruction: None,
    };
    match route(public, ctx) {
        Ok(d) => {
            let ok = d.decision == RouteOutcome::AllowedWithConditions && d.log_required;
            results.push(case(
                "public_change_conditioned_and_logged",
                "A public-target change is conditioned and requires a log",
                ok,
                "decision=allowed_with_conditions, log_required=true",
                format!("decision={:?}, log_required={}", d.decision, d.log_required),
                Some("Public-record changes should be conditioned and logged."),
            ));
        }
        Err(e) => results.push(case(
            "public_change_conditioned_and_logged",
            "route runs without error",
            false,
            "Ok",
            format!("{:?}", e),
            Some("route should not error on a valid input."),
        )),
    }

    let external = RouteInput {
        repo_root: Some(PathBuf::from(".")),
        jurisdiction: None,
        actor: "eval".into(),
        action_kind: ActionKind::ExternalAct,
        issue_tags: vec![IssueTag("release.push".into())],
        intent: "push to an external remote".into(),
        affected_paths: Vec::new(),
        risk: RiskLevel::Low,
        public_target: false,
        external_target: true,
        irreversible: false,
        user_instruction: None,
    };
    match route(external, ctx) {
        Ok(d) => {
            let ok = d.decision == RouteOutcome::AllowedWithConditions;
            results.push(case(
                "external_act_conditioned",
                "An external act is allowed only with conditions",
                ok,
                "decision=allowed_with_conditions",
                format!("decision={:?}", d.decision),
                Some("External/public targets should be conditioned, not waved through."),
            ));
        }
        Err(e) => results.push(case(
            "external_act_conditioned",
            "route runs without error",
            false,
            "Ok",
            format!("{:?}", e),
            Some("route should not error on a valid input."),
        )),
    }

    EvalReport::build("route", results)
}

/// Run a named suite (or `all`), returning one report per suite.
pub fn run_suite(
    name: &str,
    invariants: &[Invariant],
    ctx: Option<&KernelContext>,
    repo_root: &Path,
) -> Vec<EvalReport> {
    match name {
        "agent-harness" => vec![run_agent_harness_suite(invariants)],
        "prompts" => vec![run_prompts_suite(repo_root)],
        "route" => ctx.map(|c| vec![run_route_suite(c)]).unwrap_or_default(),
        "all" => {
            let mut reports = vec![
                run_agent_harness_suite(invariants),
                run_prompts_suite(repo_root),
            ];
            if let Some(c) = ctx {
                reports.push(run_route_suite(c));
            }
            reports
        }
        _ => Vec::new(),
    }
}
