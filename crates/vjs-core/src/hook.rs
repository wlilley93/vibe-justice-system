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
pub fn evaluate(input: &HookInput, _ctx: &KernelContext) -> HookDecision {
    match input.event {
        HookEvent::SessionStart => HookDecision::Warn(Finding {
            code: "VJS_ACTIVE".into(),
            message: "VJS V2 active. For governed work call vjs route; validate before commit.".into(),
            next: Some("vjs route".into()),
        }),
        HookEvent::PreWrite => {
            if input.paths.iter().any(|p| is_governed(p)) {
                HookDecision::RequireRoute(Finding {
                    code: "PERMIT_MISSING".into(),
                    message: "Governed write needs an active permit. Run vjs route for this action.".into(),
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
