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
            message: "VJS V2 active. For governed work call vjs route; validate before commit."
                .into(),
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
            message: "Run vjs validate --staged; fails closed on invariant or permit defects."
                .into(),
            next: Some("vjs validate --staged".into()),
        }),
        HookEvent::PrePush => HookDecision::Block(Finding {
            code: "RELEASE_AUTHORITY".into(),
            message: "Public push requires a recorded release warrant and post-push review.".into(),
            next: Some("verify release warrant".into()),
        }),
    }
}

/// The kernel-checkable bright-line of REG-FEDERATION-COORDINATION-001 (giving effect to [2026] VJS-SC 1):
/// a federated/subscribing jurisdiction lives "under one continued apex" and may NOT assert an apex or final
/// court function. Recording a supreme/privy (apex-tier) court ORDER is asserting that function. This returns
/// `Some(Block)` when a staged path is an apex-tier court order AND this repo's jurisdiction is NOT the apex
/// seat - the remedy is to file a referral UP. It returns `None` (no opinion) otherwise; the caller then falls
/// through to the ordinary event evaluation. The apex seat itself may record apex orders. This makes the
/// [2026] VJS-SC-ACMECO 1 class of mis-filing (a subscribing repo recording its own Supreme ruling) structurally
/// impossible rather than merely remembered.
pub fn apex_routing_decision(
    input: &HookInput,
    jurisdiction_id: &str,
    apex_seat: &str,
) -> Option<HookDecision> {
    // Only file-creating events gate a record into existence.
    if !matches!(input.event, HookEvent::PreWrite | HookEvent::PreCommit) {
        return None;
    }
    // The apex seat is the sole jurisdiction that may record an apex order; everyone else refers up.
    if jurisdiction_id == apex_seat {
        return None;
    }
    let offender = input
        .paths
        .iter()
        .find(|p| is_apex_court_order(&input.repo_root, p))?;
    let name = offender
        .file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_else(|| offender.to_string_lossy().to_string());
    Some(HookDecision::Block(Finding {
        code: "APEX_RECORD_IN_SUBSCRIBING_JURISDICTION".into(),
        message: format!(
            "{name} records an apex (supreme/privy) court ruling, but '{jurisdiction_id}' is a subscribing \
             jurisdiction. Refer up to the apex seat '{apex_seat}'."
        ),
        next: Some("vjs file (referral up)".into()),
    }))
}

/// A staged path is an apex-tier court ORDER when it is a YAML record that DECLARES itself a supreme or privy
/// court ruling (a `court:` field of that tier). Content-based, so it is robust to the filename; a record that
/// merely references an apex citation (e.g. a referral with `court: county`) is not caught.
///
/// The lawpack is the SUBSCRIBED-LAW MIRROR (read-only canon): a subscribing jurisdiction MIRRORING a canon
/// apex order into its `lawpack/` is lawful and is NOT an assertion of an apex court function - so lawpack
/// paths are excluded here. The bright-line fires only on a jurisdiction's OWN court-record store (its
/// `.vjs/orders` / court trees). Lawpack integrity (a mirror that diverges from canon) is a separate concern,
/// guarded by the lawpack pin/validation, not by this rule.
fn is_apex_court_order(repo_root: &Path, rel: &Path) -> bool {
    let name = rel.to_string_lossy().to_ascii_lowercase();
    if !(name.ends_with(".yaml") || name.ends_with(".yml")) {
        return false;
    }
    if rel.components().any(|comp| comp.as_os_str() == "lawpack") {
        return false; // the read-only subscribed-law mirror, not the jurisdiction's own court function
    }
    let content = std::fs::read_to_string(repo_root.join(rel)).unwrap_or_default();
    // Parse the typed YAML and read the `court` field, rather than substring-matching
    // (which a reformat - two spaces, a flow mapping, a quoted value - could dodge).
    // Must be an order-shaped record (carries a citation) declaring an apex-tier court.
    let Ok(val) = serde_yaml::from_str::<serde_yaml::Value>(&content) else {
        return false;
    };
    let Some(map) = val.as_mapping() else {
        return false;
    };
    let has_citation = map.contains_key(serde_yaml::Value::String("citation".into()));
    let court = map
        .get(serde_yaml::Value::String("court".into()))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    has_citation && (court.contains("supreme") || court.contains("privy"))
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

#[cfg(test)]
mod closed_surface_tests {
    use super::*;

    /// D9 ([2026] VJS-PC 13): REG-HOOKS-001 fixes a CLOSED five-event surface. The
    /// five VJS events parse; the git names "prepare-commit-msg" and "post-checkout"
    /// are outside the surface and must NOT be accepted or legislated.
    #[test]
    fn the_five_event_surface_is_closed() {
        for ev in [
            "session_start",
            "pre_write",
            "post_action",
            "pre_commit",
            "pre_push",
        ] {
            assert!(parse_event(ev).is_some(), "{ev} is a VJS event");
        }
        for non in [
            "prepare-commit-msg",
            "post-checkout",
            "pre-rebase",
            "update",
        ] {
            assert!(
                parse_event(non).is_none(),
                "{non} is a git name outside the closed five-event surface"
            );
        }
    }
}

#[cfg(test)]
mod apex_routing_tests {
    use super::*;
    use std::fs;

    fn write(dir: &Path, rel: &str, body: &str) -> PathBuf {
        let p = dir.join(rel);
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&p, body).unwrap();
        PathBuf::from(rel)
    }

    fn input(root: &Path, event: HookEvent, rels: Vec<PathBuf>) -> HookInput {
        HookInput {
            event,
            repo_root: root.to_path_buf(),
            actor: "lexby".into(),
            paths: rels,
            tool: None,
        }
    }

    #[test]
    fn subscribing_repo_recording_a_supreme_order_is_blocked() {
        let dir = std::env::temp_dir().join(format!("vjs_apex_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let rel = write(
            &dir,
            ".vjs/orders/2026-VJS-SC-ACMECO-001.yaml",
            "id: x\ncitation: \"[2026] VJS-SC-ACMECO 1\"\ncourt: supreme_court\nstatus: binding\n",
        );
        let inp = input(&dir, HookEvent::PreCommit, vec![rel]);
        let d = apex_routing_decision(&inp, "acmeco", "vjs");
        assert!(
            matches!(d, Some(HookDecision::Block(_))),
            "a subscribing jurisdiction must not record an apex order"
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn apex_seat_recording_a_supreme_order_is_allowed() {
        let dir = std::env::temp_dir().join(format!("vjs_apex_seat_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let rel = write(
            &dir,
            "lawpack/v2/orders/2026-VJS-SC-004.yaml",
            "id: x\ncitation: \"[2026] VJS-SC 4\"\ncourt: supreme_court\nstatus: binding\n",
        );
        let inp = input(&dir, HookEvent::PreCommit, vec![rel]);
        assert!(
            apex_routing_decision(&inp, "vjs", "vjs").is_none(),
            "the apex seat may record apex orders"
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_referral_with_court_county_is_not_caught() {
        let dir = std::env::temp_dir().join(format!("vjs_apex_ref_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let rel = write(
            &dir,
            ".vjs/orders/referral.yaml",
            "id: x\ncitation: \"REFERRAL -> [2026] VJS-SC 4\"\ncourt: county\nstatus: corrected_to_referral\n",
        );
        let inp = input(&dir, HookEvent::PreCommit, vec![rel]);
        assert!(
            apex_routing_decision(&inp, "acmeco", "vjs").is_none(),
            "a referral (court: county) is lawful, not an apex assertion"
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn county_order_in_a_subscribing_repo_is_allowed() {
        let dir = std::env::temp_dir().join(format!("vjs_apex_cc_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let rel = write(
            &dir,
            ".vjs/orders/2026-VJS-CC-ACMECO-079.yaml",
            "id: x\ncitation: \"[2026] VJS-CC-ACMECO 79\"\ncourt: county\nstatus: binding\n",
        );
        let inp = input(&dir, HookEvent::PreCommit, vec![rel]);
        assert!(
            apex_routing_decision(&inp, "acmeco", "vjs").is_none(),
            "a subscribing jurisdiction may run its own county court"
        );
        let _ = fs::remove_dir_all(&dir);
    }
    #[test]
    fn mirroring_a_canon_apex_order_into_the_lawpack_is_allowed() {
        let dir = std::env::temp_dir().join(format!("vjs_apex_mirror_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        // a subscribing repo mirroring the canon's VJS-SC 4 into its READ-ONLY lawpack mirror
        let rel = write(
            &dir,
            "lawpack/v2/orders/2026-VJS-SC-004.yaml",
            "id: x\ncitation: \"[2026] VJS-SC 4\"\ncourt: supreme_court\nstatus: binding\n",
        );
        let inp = input(&dir, HookEvent::PreCommit, vec![rel]);
        assert!(
            apex_routing_decision(&inp, "acmeco", "vjs").is_none(),
            "mirroring canon law into lawpack/ is lawful, not an apex assertion"
        );
        let _ = fs::remove_dir_all(&dir);
    }
}
