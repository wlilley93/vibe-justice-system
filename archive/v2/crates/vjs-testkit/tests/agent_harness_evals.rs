//! Golden eval coverage for the agent-harness invariants.
//!
//! These build the same predicates the lawpack ships (INV-HOOKS-SHORT-001,
//! INV-PROMPT-PATCH-001, INV-AGENT-EVALS-001) and run the deterministic
//! agent-harness eval suite against them. This is the "evals measure" half of
//! the doctrine: the suite must pass green, and each case proves the invariant
//! both accepts compliant state and catches the violation.

use vjs_core::evals;
use vjs_core::spec::Invariant;
use vjs_core::types::*;

fn scope(paths: &[&str]) -> Scope {
    Scope {
        paths: Some(paths.iter().map(|p| (*p).into()).collect()),
        jurisdictions: None,
        action_kinds: None,
        issue_tags: None,
        records: None,
    }
}

fn inv(id: &str, scope_paths: &[&str], rule: PredicateExpr, severity: Severity) -> Invariant {
    Invariant {
        id: InvariantId(id.into()),
        title: id.into(),
        basis: Vec::new(),
        scope: Some(scope(scope_paths)),
        rule,
        severity,
        remedy: String::new(),
    }
}

fn any(globs: &[&str]) -> PredicateExpr {
    PredicateExpr::Any {
        items: globs
            .iter()
            .map(|g| PredicateExpr::PathChanged { glob: (*g).into() })
            .collect(),
    }
}

fn harness_invariants() -> Vec<Invariant> {
    vec![
        inv(
            "INV-HOOKS-SHORT-001",
            &[".vjs/hooks/**"],
            PredicateExpr::FileWordsLte {
                glob: ".vjs/hooks/**".into(),
                max: 40,
            },
            Severity::Fatal,
        ),
        inv(
            "INV-PROMPT-PATCH-001",
            &["AGENTS.md", "VJS.md", "crates/vjs-mcp/**"],
            PredicateExpr::If {
                condition: Box::new(any(&["AGENTS.md", "VJS.md", "crates/vjs-mcp/**"])),
                then: Box::new(PredicateExpr::DecisionLogExists { issue: None }),
            },
            Severity::Error,
        ),
        inv(
            "INV-AGENT-EVALS-001",
            &["crates/vjs-mcp/**", ".vjs/hooks/**", "AGENTS.md", "VJS.md"],
            PredicateExpr::If {
                condition: Box::new(any(&[
                    "crates/vjs-mcp/**",
                    ".vjs/hooks/**",
                    "AGENTS.md",
                    "VJS.md",
                ])),
                then: Box::new(PredicateExpr::PathChanged {
                    glob: "crates/vjs-testkit/**".into(),
                }),
            },
            Severity::Warning,
        ),
    ]
}

#[test]
fn agent_harness_suite_is_green() {
    let invariants = harness_invariants();
    let report = evals::run_agent_harness_suite(&invariants);
    let failures: Vec<_> = report.results.iter().filter(|r| !r.passed).collect();
    assert_eq!(
        report.failed, 0,
        "agent-harness evals must pass; failures: {:?}",
        failures
    );
    // Six cases: each invariant proven to both accept and catch.
    assert_eq!(report.results.len(), 6);
}

#[test]
fn missing_invariant_is_reported() {
    // An empty lawpack should surface the missing invariants rather than
    // silently passing.
    let report = evals::run_agent_harness_suite(&[]);
    assert!(report.failed >= 3, "missing invariants must be flagged");
}
