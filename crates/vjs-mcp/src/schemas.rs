//! The MCP tool surface: the nine-verb schema set the server advertises.
//! Split out of lib.rs (behavior-preserving) to keep each file under the
//! structural-cleanliness ceiling. The schemas are data, not logic.

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub fn get_tool_schemas() -> Vec<McpTool> {
    vec![
        McpTool {
            name: "vjs.route".into(),
            description: "Use before governed load-bearing work. Returns the permit, obligations, required proofs, court trigger, and bounded binding authority. Do not proceed with a governed write without this result.".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["action_kind", "issue_tags", "intent"],
                "properties": {
                    "repo_root": {"type": "string"},
                    "action_kind": {"type": "string"},
                    "issue_tags": {"type": "array", "items": {"type": "string"}},
                    "intent": {"type": "string"},
                    "risk": {"type": "string"},
                    "public_target": {"type": "boolean"},
                    "external_target": {"type": "boolean"},
                    "irreversible": {"type": "boolean"}
                }
            }),
            output_schema: Some(serde_json::json!({
                "type": "object",
                "required": ["decision", "court_required", "binding", "must_do", "must_not_do"]
            })),
        },
        McpTool {
            name: "vjs.lookup".into(),
            description: "Look up the binding V2 authorities for an issue. If none are returned the issue is V2-silent: treat it as first-impression and route it, do not import V1 by silence.".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["issue_tags"],
                "properties": {
                    "repo_root": {"type": "string"},
                    "issue_tags": {"type": "array", "items": {"type": "string"}},
                    "limit": {"type": "integer"}
                }
            }),
            output_schema: None,
        },
        McpTool {
            name: "vjs.validate".into(),
            description: "Use after changes and before commit. Returns deterministic findings (lawpack, invariants, permit gate, boundary). Repair every finding before continuing; do not bypass.".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "repo_root": {"type": "string"},
                    "scope": {"type": "string"}
                }
            }),
            output_schema: None,
        },
        McpTool {
            name: "vjs.log".into(),
            description: "Use when vjs.route returns log_required, or for any material implementation decision, public-record change, external act, or security-sensitive act. 50-150 words, with the basis.".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["kind", "issue", "decision", "basis", "risk", "why"],
                "properties": {
                    "id": {"type": "string"},
                    "kind": {"type": "string"},
                    "issue": {"type": "string"},
                    "decision": {"type": "string"},
                    "basis": {"type": "array", "items": {"type": "string"}},
                    "risk": {"type": "string"},
                    "why": {"type": "string"}
                }
            }),
            output_schema: None,
        },
        McpTool {
            name: "vjs.file".into(),
            description: "Use when vjs.route returns court_required (first-impression, distinction, overruling, conflict, or breach). Files a symmetric submission to the named V2 court; the bench decides, not the agent.".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["court", "question"],
                "properties": {
                    "court": {"type": "string"},
                    "question": {"type": "string"},
                    "facts": {"type": "string"},
                    "requested_order": {"type": "string"}
                }
            }),
            output_schema: None,
        },
        McpTool {
            name: "vjs.status".into(),
            description: "Report the current repo's VJS state: lawpack digest, active permits, the lifecycle stage, and open obligations. Read-only; call it to orient before acting.".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "repo_root": {"type": "string"}
                }
            }),
            output_schema: None,
        },
        McpTool {
            name: "vjs.allocate".into(),
            description: "Allocate the next citation in a series from the live register (allocate->citation). The kernel mints the number; never hand-pick a citation. The Cc series is repo-scoped and carries a repo segment (VJS-CC-<REPO> n) - pass `repo` to allocate a subscriber's Cc line, else it defaults to this server's repo_code. Returns the canonical citation string.".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["series"],
                "properties": {
                    "series": {"type": "string"},
                    "year": {"type": "integer"},
                    "repo": {"type": "string"}
                }
            }),
            output_schema: None,
        },
        McpTool {
            name: "vjs.convene".into(),
            description: "Convene a court over a filed submission (convene->court). The kernel verifies the bench size against the constitution ([2026] VJS-SC 2) and pins the case-file digest. Refuses an under-strength bench.".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["court", "submission", "bench"],
                "properties": {
                    "court": {"type": "string"},
                    "submission": {"type": "string"},
                    "bench": {"type": "array", "items": {"type": "string"}},
                    "issue": {"type": "string"}
                }
            }),
            output_schema: None,
        },
        McpTool {
            name: "vjs.record".into(),
            description: "Record a court order (record->order). The kernel verifies bench-integrity (size + an opinion per seat) against the constitution and writes the order; a non-assented bench defect is refused at the door. The commit hook remains the wall.".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["id", "court", "bench"],
                "properties": {
                    "id": {"type": "string"},
                    "court": {"type": "string"},
                    "citation": {"type": "string"},
                    "bench": {"type": "array", "items": {"type": "string"}},
                    "source_opinion": {"type": "string"},
                    "assent_source": {"type": "string"}
                }
            }),
            output_schema: None,
        },
    ]
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct McpTool {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
    pub output_schema: Option<Value>,
}
