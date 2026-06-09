//! Functional hooks (REG-HOOKS-001).
//!
//! A VJS hook is a deterministic function over repo state and event context,
//! returning a typed decision. Any prompt text it emits is explanatory only and
//! bounded (<= 40 words). A hook never adjudicates breach, creates law, calls a
//! model for a binding decision, or injects long context - it is the switch that
//! asks the kernel whether the current state is valid.

use crate::KernelContext;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HookEvent {
    SessionStart,
    PreWrite,
    PostAction,
    PreCommit,
    PrePush,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HookInput {
    pub event: HookEvent,
    pub repo_root: PathBuf,
    pub actor: String,
    pub paths: Vec<PathBuf>,
    pub tool: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Finding {
    pub code: String,
    /// Bounded explanatory message (<= 40 words, REG-HOOKS-001). No sermon.
    pub message: String,
    pub next: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "decision", rename_all = "snake_case")]
pub enum HookDecision {
    Allow,
    Warn(Finding),
    Block(Finding),
    RequireRoute(Finding),
    RequireLog(Finding),
    RequireProof(Finding),
}

impl HookDecision {
    /// Fail-closed exit codes for an executable adapter: 0 allow/warn (non-
    /// blocking), 1 missing obligation, 2 must route / blocked, 3 warrant.
    pub fn exit_code(&self) -> i32 {
        match self {
            HookDecision::Allow | HookDecision::Warn(_) => 0,
            HookDecision::RequireLog(_) | HookDecision::RequireProof(_) => 1,
            HookDecision::RequireRoute(_) | HookDecision::Block(_) => 2,
        }
    }

    pub fn code(&self) -> &str {
        match self {
            HookDecision::Allow => "ALLOW",
            HookDecision::Warn(f)
            | HookDecision::Block(f)
            | HookDecision::RequireRoute(f)
            | HookDecision::RequireLog(f)
            | HookDecision::RequireProof(f) => &f.code,
        }
    }

    pub fn message(&self) -> &str {
        match self {
            HookDecision::Allow => "ok",
            HookDecision::Warn(f)
            | HookDecision::Block(f)
            | HookDecision::RequireRoute(f)
            | HookDecision::RequireLog(f)
            | HookDecision::RequireProof(f) => &f.message,
        }
    }
}

/// A governed surface: lawpack records, kernel crates, and the .vjs store.
/// Matched on whole path components (relative or absolute), so "lawpack-2/",
/// ".vjs_backup/" and "my-crates/" substrings never count as governed.
fn is_governed(path: &Path) -> bool {
    path.components().any(|c| {
        matches!(
            c.as_os_str().to_str(),
            Some("lawpack") | Some("crates") | Some(".vjs")
        )
    })
}

/// The deterministic hook function. Same logic for every runtime adapter.
/// Without permit state it is the conservative form: a governed write
/// requires a route (the permit-aware form is `evaluate_with_permits`).
pub fn evaluate(input: &HookInput, ctx: &KernelContext) -> HookDecision {
    evaluate_with_permits(input, ctx, &[])
}

/// The permit-aware hook: a governed write under an active, unexpired,
/// in-scope permit is lawful and passes silently; an unpermitted governed
/// write fails closed with the route as the remedy. Without a governance
/// config the component fallback defines the governed surface.
pub fn evaluate_with_permits(
    input: &HookInput,
    ctx: &KernelContext,
    permits: &[crate::spec::Permit],
) -> HookDecision {
    evaluate_governed(input, ctx, permits, &[], &[])
}

/// The fully configured form: ONE definition of the governed surface, the
/// jurisdiction's own permit_required/permit_exempt lists, classified by the
/// same PathClassifier the commit gate uses.
pub fn evaluate_governed(
    input: &HookInput,
    _ctx: &KernelContext,
    permits: &[crate::spec::Permit],
    permit_required: &[String],
    permit_exempt: &[String],
) -> HookDecision {
    let governed = |p: &PathBuf| -> bool {
        if permit_required.is_empty() {
            is_governed(p)
        } else {
            crate::governance::PathClassifier::classify(p, permit_required, permit_exempt)
                == crate::governance::PathClassification::Governed
        }
    };
    match input.event {
        HookEvent::SessionStart => HookDecision::Warn(Finding {
            code: "VJS_ACTIVE".into(),
            message: "VJS V2 active. For governed work call vjs route; validate before commit.".into(),
            next: Some("vjs route".into()),
        }),
        HookEvent::PreWrite => {
            let uncovered: Vec<&PathBuf> = input
                .paths
                .iter()
                .filter(|p| {
                    governed(p)
                        && !crate::governance::PermitGate::covers(&p.to_string_lossy(), permits)
                })
                .collect();
            if let Some(first) = uncovered.first() {
                HookDecision::RequireRoute(Finding {
                    code: "PERMIT_MISSING".into(),
                    message: format!(
                        "Governed write to {} has no active permit. Run vjs route for this action.",
                        first
                            .file_name()
                            .map(|f| f.to_string_lossy().to_string())
                            .unwrap_or_else(|| first.to_string_lossy().to_string())
                    ),
                    next: Some("vjs route".into()),
                })
            } else {
                HookDecision::Allow
            }
        }
        HookEvent::PostAction => HookDecision::RequireLog(Finding {
            code: "LOG_CHECK".into(),
            message: "Material decision must carry a decision log before commit.".into(),
            next: Some("vjs log decision".into()),
        }),
        HookEvent::PreCommit => HookDecision::RequireProof(Finding {
            code: "VALIDATE_STAGED".into(),
            message: "Run vjs validate --staged; fails closed on invariant or permit defects.".into(),
            next: Some("vjs validate --staged".into()),
        }),
        HookEvent::PrePush => HookDecision::Block(Finding {
            code: "RELEASE_AUTHORITY".into(),
            message: "Public push requires a recorded release warrant and post-push review.".into(),
            next: Some("verify release warrant".into()),
        }),
    }
}

pub fn parse_event(s: &str) -> Option<HookEvent> {
    match s.replace('-', "_").as_str() {
        "session_start" => Some(HookEvent::SessionStart),
        "pre_write" => Some(HookEvent::PreWrite),
        "post_action" => Some(HookEvent::PostAction),
        "pre_commit" => Some(HookEvent::PreCommit),
        "pre_push" => Some(HookEvent::PrePush),
        _ => None,
    }
}
