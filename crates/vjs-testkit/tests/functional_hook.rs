//! Eval coverage for the functional-hook model (REG-HOOKS-001) and the
//! self-convening disposition (REG-SELF-CONVENE-001). This is the "evals
//! measure" half of the doctrine: the hook is a deterministic function whose
//! decision and bounded message are asserted, not reviewed by vibes.

use std::path::PathBuf;
use vjs_core::hook::{evaluate, parse_event, HookDecision, HookEvent, HookInput};
use vjs_core::{ContextLimits, KernelContext};
use vjs_lawpack::LawpackLoader;

fn build_ctx() -> KernelContext {
    let repo = PathBuf::from(".");
    let lawpack = LawpackLoader::load(&repo.join("lawpack/v2")).unwrap();
    let graph = lawpack.build_authority_graph().unwrap();
    KernelContext {
        authority_graph: graph,
        limits: ContextLimits::default(),
        lawpack_digest: "test".into(),
    }
}

fn hook(event: HookEvent, paths: Vec<&str>) -> HookDecision {
    let ctx = build_ctx();
    let input = HookInput {
        event,
        repo_root: PathBuf::from("."),
        actor: "lexby".into(),
        paths: paths.into_iter().map(PathBuf::from).collect(),
        tool: None,
    };
    evaluate(&input, &ctx)
}

#[test]
fn pre_write_on_a_governed_path_requires_route_and_fails_closed() {
    let d = hook(HookEvent::PreWrite, vec!["lawpack/v2/orders/x.yaml"]);
    assert!(
        matches!(d, HookDecision::RequireRoute(_)),
        "a governed write must require a route/permit"
    );
    assert_eq!(d.exit_code(), 2, "the function hook must fail closed");
}

#[test]
fn pre_write_on_an_ungoverned_path_is_allowed() {
    let d = hook(HookEvent::PreWrite, vec!["notes/scratch.txt"]);
    assert!(matches!(d, HookDecision::Allow));
    assert_eq!(d.exit_code(), 0);
}

#[test]
fn every_hook_message_is_bounded_and_carries_no_sermon() {
    for ev in [
        HookEvent::SessionStart,
        HookEvent::PreWrite,
        HookEvent::PostAction,
        HookEvent::PreCommit,
        HookEvent::PrePush,
    ] {
        let d = hook(ev, vec!["lawpack/v2/statutes/x.yaml"]);
        let words = d.message().split_whitespace().count();
        assert!(words <= 40, "REG-HOOKS-001: hook message must be <= 40 words, got {}", words);
    }
}

#[test]
fn session_start_is_a_non_blocking_reminder() {
    let d = hook(HookEvent::SessionStart, vec![]);
    assert!(
        matches!(d, HookDecision::Warn(_)),
        "the session reminder is a Warn, not a silent Allow"
    );
    assert_eq!(d.exit_code(), 0, "the session reminder must not block");
}

#[test]
fn post_action_requires_a_log_and_pre_commit_requires_proof() {
    let d = hook(HookEvent::PostAction, vec![]);
    assert!(matches!(d, HookDecision::RequireLog(_)));
    assert_eq!(d.exit_code(), 1);

    let d = hook(HookEvent::PreCommit, vec![]);
    assert!(matches!(d, HookDecision::RequireProof(_)));
    assert_eq!(d.exit_code(), 1);
}

#[test]
fn governed_surface_is_matched_on_path_components_not_substrings() {
    // Look-alike directories outside the governed surface stay ungoverned.
    for p in ["lawpack-2/x.yaml", "my-crates/lib.rs", ".vjs_backup/state.json", "a/lawpack_archive/f.txt"] {
        let d = hook(HookEvent::PreWrite, vec![p]);
        assert!(matches!(d, HookDecision::Allow), "{} must not count as governed", p);
    }
    // Governed components match wherever they sit, including absolute paths.
    for p in ["crates/vjs-core/src/lib.rs", "/abs/repo/crates/vjs-core/src/lib.rs", ".vjs/config.toml"] {
        let d = hook(HookEvent::PreWrite, vec![p]);
        assert!(matches!(d, HookDecision::RequireRoute(_)), "{} must count as governed", p);
    }
}

#[test]
fn pre_push_blocks_pending_release_authority() {
    let d = hook(HookEvent::PrePush, vec![]);
    assert!(matches!(d, HookDecision::Block(_)));
    assert_eq!(d.exit_code(), 2);
}

#[test]
fn parse_event_accepts_canonical_and_hyphenated_forms() {
    assert!(matches!(parse_event("pre_write"), Some(HookEvent::PreWrite)));
    assert!(matches!(parse_event("pre-commit"), Some(HookEvent::PreCommit)));
    assert!(matches!(parse_event("session_start"), Some(HookEvent::SessionStart)));
    assert!(parse_event("nonsense").is_none());
}
