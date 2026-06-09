use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use vjs_core::*;
use vjs_core::types::*;
use vjs_core::error::*;
use vjs_core::route::*;
use vjs_core::spec::*;
use vjs_lawpack::*;

/// VJS MCP Server
/// Thin JSON-RPC adapter over the deterministic kernel
/// Exposes only 6 tools: route, lookup, validate, log, file, status

pub struct McpServer {
    pub repo_root: std::path::PathBuf,
}

impl McpServer {
    pub fn new(repo_root: std::path::PathBuf) -> Self {
        Self { repo_root }
    }

    pub fn handle_request(&self, request: &str) -> Result<String, KernelError> {
        let req: JsonRpcRequest = serde_json::from_str(request)
            .map_err(|e| KernelError::InvalidInput(e.to_string()))?;

        let result = match req.method.as_str() {
            "vjs.route" => self.handle_route(req.params)?,
            "vjs.lookup" => self.handle_lookup(req.params)?,
            "vjs.validate" => self.handle_validate(req.params)?,
            "vjs.log" => self.handle_log(req.params)?,
            "vjs.file" => self.handle_file(req.params)?,
            "vjs.status" => self.handle_status(req.params)?,
            _ => {
                return Err(KernelError::InvalidInput(format!(
                    "Unknown method: {}",
                    req.method
                )))
            }
        };

        let response = JsonRpcResponse {
            jsonrpc: "2.0".into(),
            id: req.id,
            result: Some(result),
            error: None,
        };

        serde_json::to_string(&response)
            .map_err(|e| KernelError::Serialization(e.to_string()))
    }

    fn handle_route(&self, params: Option<Value>) -> Result<Value, KernelError> {
        let params = params.ok_or_else(|| KernelError::InvalidInput("params required".into()))?;
        let input: RouteInput = serde_json::from_value(params)
            .map_err(|e| KernelError::InvalidInput(e.to_string()))?;

        let ctx = build_context(&self.repo_root)?;
        let decision = route(input, &ctx)?;

        Ok(serde_json::to_value(decision)
            .map_err(|e| KernelError::Serialization(e.to_string()))?)
    }

    fn handle_lookup(&self, params: Option<Value>) -> Result<Value, KernelError> {
        let params = params.ok_or_else(|| KernelError::InvalidInput("params required".into()))?;
        let issue: String = serde_json::from_value(
            params.get("issue")
                .cloned()
                .ok_or_else(|| KernelError::InvalidInput("issue required".into()))?
        ).map_err(|e| KernelError::InvalidInput(e.to_string()))?;

        let ctx = build_context(&self.repo_root)?;
        let input = RouteInput {
            repo_root: Some(self.repo_root.clone()),
            jurisdiction: Some(JurisdictionId("default".into())),
            actor: "lexby".into(),
            action_kind: ActionKind::ImplementationDecision,
            issue_tags: vec![IssueTag(issue)],
            intent: "lookup".into(),
            affected_paths: Vec::new(),
            risk: RiskLevel::Low,
            public_target: false,
            external_target: false,
            irreversible: false,
            user_instruction: None,
        };

        let authorities = resolve_authority(&input, &ctx.authority_graph)?;
        Ok(serde_json::to_value(authorities)
            .map_err(|e| KernelError::Serialization(e.to_string()))?)
    }

    fn handle_validate(&self, params: Option<Value>) -> Result<Value, KernelError> {
        let _params = params;
        let lawpack = load_lawpack(&self.repo_root)?;
        let report = LawpackValidator::validate(&lawpack)?;

        Ok(serde_json::to_value(report)
            .map_err(|e| KernelError::Serialization(e.to_string()))?)
    }

    fn handle_log(&self, params: Option<Value>) -> Result<Value, KernelError> {
        let params = params.ok_or_else(|| KernelError::InvalidInput("params required".into()))?;
        let log: DecisionLog = serde_json::from_value(params)
            .map_err(|e| KernelError::InvalidInput(e.to_string()))?;

        vjs_store::Store::write_log(&self.repo_root, &log)?;

        Ok(serde_json::to_value(log)
            .map_err(|e| KernelError::Serialization(e.to_string()))?)
    }

    fn handle_file(&self, params: Option<Value>) -> Result<Value, KernelError> {
        let params = params.ok_or_else(|| KernelError::InvalidInput("params required".into()))?;
        let submission: vjs_store::Submission = serde_json::from_value(params)
            .map_err(|e| KernelError::InvalidInput(e.to_string()))?;

        vjs_store::Store::write_submission(&self.repo_root, &submission)?;

        Ok(serde_json::to_value(submission)
            .map_err(|e| KernelError::Serialization(e.to_string()))?)
    }

    fn handle_status(&self, _params: Option<Value>) -> Result<Value, KernelError> {
        let vjs_dir = self.repo_root.join(".vjs");
        let status = serde_json::json!({
            "repo": self.repo_root.display().to_string(),
            "vjs_installed": vjs_dir.exists(),
        });
        Ok(status)
    }
}

fn build_context(repo: &std::path::PathBuf) -> Result<KernelContext, KernelError> {
    let lawpack = load_lawpack(repo)?;
    let graph = lawpack.build_authority_graph()?;
    let digest = compute_digest(repo)?;

    Ok(KernelContext {
        authority_graph: graph,
        limits: ContextLimits::default(),
        lawpack_digest: digest,
    })
}

fn load_lawpack(repo: &std::path::PathBuf) -> Result<Lawpack, KernelError> {
    let lawpack_dir = repo.join("lawpack/v2");
    if lawpack_dir.exists() {
        LawpackLoader::load(&lawpack_dir)
    } else {
        Ok(Lawpack {
            statutes: Vec::new(),
            regulations: Vec::new(),
            rules: Vec::new(),
            orders: Vec::new(),
            specs: Vec::new(),
            invariants: Vec::new(),
            decisions: Vec::new(),
        })
    }
}

fn compute_digest(repo: &std::path::PathBuf) -> Result<String, KernelError> {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    let manifest = repo.join("lawpack/v2/manifest.toml");
    if manifest.exists() {
        let content = std::fs::read(&manifest)
            .map_err(|e| KernelError::Io(e.to_string()))?;
        hasher.update(&content);
    }
    Ok(format!("sha256:{}", hex::encode(hasher.finalize())))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Option<Value>,
    result: Option<Value>,
    error: Option<JsonRpcError>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct JsonRpcError {
    code: i32,
    message: String,
    data: Option<Value>,
}

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
    ]
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct McpTool {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
    pub output_schema: Option<Value>,
}
