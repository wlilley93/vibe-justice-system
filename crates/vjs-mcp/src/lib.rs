use serde::{Deserialize, Serialize};
use serde_json::Value;

use vjs_core::*;
use vjs_lawpack::*;

/// VJS MCP Server
/// Thin JSON-RPC adapter over the deterministic kernel (the server_of_law posture of
/// REG-KERNEL-001, adopted as a governed-record front door by [2026] VJS-PC 14).
/// Exposes 9 tools: route, lookup, validate, log, file, status (the lifecycle) plus
/// allocate, convene, record (the governed-record-creation verbs, PC-14 D5). The CLI
/// path stays co-equal; all logic lives in the kernel - these are thin transports.
pub struct McpServer {
    pub repo_root: std::path::PathBuf,
}

impl McpServer {
    pub fn new(repo_root: std::path::PathBuf) -> Self {
        Self { repo_root }
    }

    pub fn handle_request(&self, request: &str) -> Result<String, KernelError> {
        let req: JsonRpcRequest =
            serde_json::from_str(request).map_err(|e| KernelError::InvalidInput(e.to_string()))?;

        // #17 auth: when a token is configured (the cage, #16), every call must carry
        // a matching `_token`; otherwise (the dev default) no auth.
        if let Err(e) = self.check_auth(&req) {
            self.audit(&req.method, "auth_denied");
            return Err(e);
        }

        let result = match req.method.as_str() {
            "vjs.route" => self.handle_route(req.params),
            "vjs.lookup" => self.handle_lookup(req.params),
            "vjs.validate" => self.handle_validate(req.params),
            "vjs.log" => self.handle_log(req.params),
            "vjs.file" => self.handle_file(req.params),
            "vjs.status" => self.handle_status(req.params),
            // PC-14 D5: the governed-record-creation verbs, closing the front-door gap.
            "vjs.allocate" => self.handle_allocate(req.params),
            "vjs.convene" => self.handle_convene(req.params),
            "vjs.record" => self.handle_record(req.params),
            other => Err(KernelError::InvalidInput(format!(
                "Unknown method: {other}"
            ))),
        };

        // #17 audit: append every call and its outcome to the append-only trail.
        self.audit(&req.method, if result.is_ok() { "ok" } else { "error" });
        let result = result?;

        let response = JsonRpcResponse {
            jsonrpc: "2.0".into(),
            id: req.id,
            result: Some(result),
            error: None,
        };

        serde_json::to_string(&response).map_err(|e| KernelError::Serialization(e.to_string()))
    }

    /// #17 / #16 cage auth: if VJS_MCP_TOKEN is set in the server's environment,
    /// every request must carry a matching `_token` param (fail closed); if it is
    /// unset or empty, there is no auth (the trusting dev default).
    fn check_auth(&self, req: &JsonRpcRequest) -> Result<(), KernelError> {
        let expected = std::env::var("VJS_MCP_TOKEN").unwrap_or_default();
        if auth_satisfied(&expected, req.params.as_ref()) {
            Ok(())
        } else {
            Err(KernelError::InvalidInput(
                "unauthenticated: a matching _token is required (VJS_MCP_TOKEN set)".into(),
            ))
        }
    }

    /// #17 audit: append-only trail of every MCP call (.vjs/audit/mcp-audit.log,
    /// gitignored via *.log). Best-effort; an audit-write failure never blocks a call.
    fn audit(&self, method: &str, outcome: &str) {
        let dir = self.repo_root.join(".vjs/audit");
        let _ = std::fs::create_dir_all(&dir);
        let line = format!("{}\t{method}\t{outcome}\n", chrono::Utc::now().to_rfc3339());
        use std::io::Write;
        if let Ok(mut f) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(dir.join("mcp-audit.log"))
        {
            let _ = f.write_all(line.as_bytes());
        }
    }

    fn handle_route(&self, params: Option<Value>) -> Result<Value, KernelError> {
        let params = params.ok_or_else(|| KernelError::InvalidInput("params required".into()))?;
        let input: RouteInput =
            serde_json::from_value(params).map_err(|e| KernelError::InvalidInput(e.to_string()))?;

        let ctx = build_context(&self.repo_root)?;
        let decision = route(input, &ctx)?;

        serde_json::to_value(decision).map_err(|e| KernelError::Serialization(e.to_string()))
    }

    fn handle_lookup(&self, params: Option<Value>) -> Result<Value, KernelError> {
        let params = params.ok_or_else(|| KernelError::InvalidInput("params required".into()))?;
        let issue: String = serde_json::from_value(
            params
                .get("issue")
                .cloned()
                .ok_or_else(|| KernelError::InvalidInput("issue required".into()))?,
        )
        .map_err(|e| KernelError::InvalidInput(e.to_string()))?;

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
        serde_json::to_value(authorities).map_err(|e| KernelError::Serialization(e.to_string()))
    }

    fn handle_validate(&self, _params: Option<Value>) -> Result<Value, KernelError> {
        // The same engine the CLI and CI call - one validate implementation.
        let report = vjs_engine::validate(&self.repo_root, &vjs_engine::ValidateOpts::default())?;
        serde_json::to_value(report).map_err(|e| KernelError::Serialization(e.to_string()))
    }

    fn handle_log(&self, params: Option<Value>) -> Result<Value, KernelError> {
        let params = params.ok_or_else(|| KernelError::InvalidInput("params required".into()))?;
        let log: DecisionLog =
            serde_json::from_value(params).map_err(|e| KernelError::InvalidInput(e.to_string()))?;

        vjs_store::Store::write_log(&self.repo_root, &log)?;

        serde_json::to_value(log).map_err(|e| KernelError::Serialization(e.to_string()))
    }

    fn handle_file(&self, params: Option<Value>) -> Result<Value, KernelError> {
        let params = params.ok_or_else(|| KernelError::InvalidInput("params required".into()))?;
        let submission: vjs_store::Submission =
            serde_json::from_value(params).map_err(|e| KernelError::InvalidInput(e.to_string()))?;

        vjs_store::Store::write_submission(&self.repo_root, &submission)?;

        serde_json::to_value(submission).map_err(|e| KernelError::Serialization(e.to_string()))
    }

    fn handle_status(&self, _params: Option<Value>) -> Result<Value, KernelError> {
        let vjs_dir = self.repo_root.join(".vjs");
        let status = serde_json::json!({
            "repo": self.repo_root.display().to_string(),
            "vjs_installed": vjs_dir.exists(),
        });
        Ok(status)
    }

    // ---- PC-14 D5: the record-creation verbs (thin transports to the kernel) ----

    /// allocate -> citation: the next citation in a series from the live register.
    /// Thin transport to the kernel allocator; the agent cannot hand-pick a number.
    fn handle_allocate(&self, params: Option<Value>) -> Result<Value, KernelError> {
        let p = params.ok_or_else(|| KernelError::InvalidInput("params required".into()))?;
        let series = p
            .get("series")
            .and_then(|v| v.as_str())
            .ok_or_else(|| KernelError::InvalidInput("series required".into()))?
            .to_ascii_uppercase();
        let year = p.get("year").and_then(|v| v.as_i64()).unwrap_or_else(|| {
            chrono::Utc::now()
                .format("%Y")
                .to_string()
                .parse()
                .unwrap_or(2026)
        }) as i32;
        // The Cc series is bound to a SPECIFIC repo's code and so carries a repo segment
        // (`VJS-CC-ACMECO 7`); the canon series (PC/SC/REG/ACT/DEC/SPEC/INV/COA/...) are
        // seat-wide and carry none. The repo defaults to this server's own repo_code; a
        // caller allocating a subscriber's Cc line passes `repo` explicitly. The max is
        // looked up scoped to that segment, so two repos' Cc registers never collide and
        // the segment-less lookup (which mixed them) cannot under-count. Mirrors the CLI
        // cmd_next_citation exactly (D4: one shared meaning across both front doors).
        let default_repo_code = vjs_store::Store::read_repo_config(&self.repo_root)?
            .map(|c| {
                c.repo_code
                    .unwrap_or_else(|| c.jurisdiction_id.to_uppercase())
            })
            .unwrap_or_else(|| "VJS".to_string());
        let repo_code = p
            .get("repo")
            .and_then(|v| v.as_str())
            .map(|s| s.to_ascii_uppercase())
            .unwrap_or(default_repo_code);
        let (repo_for_lookup, repo_segment): (Option<&str>, String) = if series == "CC" {
            (Some(repo_code.as_str()), format!("-{repo_code}"))
        } else {
            (None, String::new())
        };
        let lawpack_dir = self.repo_root.join("lawpack/v2");
        let max = if lawpack_dir.exists() {
            LawpackValidator::live_citation_max(&lawpack_dir, &series, repo_for_lookup, year)?
        } else {
            0
        };
        let n = max + 1;
        let citation = format!("[{year}] VJS-{series}{repo_segment} {n}");
        Ok(serde_json::json!({ "series": series, "year": year, "n": n, "citation": citation }))
    }

    /// convene -> court: verify the bench size against the constitution ([2026]
    /// VJS-SC 2, read by reference - the PC-13 D10 gate) and write the convening with
    /// a pinned case-file digest. Refuses an under-strength bench.
    fn handle_convene(&self, params: Option<Value>) -> Result<Value, KernelError> {
        let p = params.ok_or_else(|| KernelError::InvalidInput("params required".into()))?;
        let court = p
            .get("court")
            .and_then(|v| v.as_str())
            .ok_or_else(|| KernelError::InvalidInput("court required".into()))?
            .to_string();
        let submission = p
            .get("submission")
            .and_then(|v| v.as_str())
            .ok_or_else(|| KernelError::InvalidInput("submission required".into()))?
            .to_string();
        let bench: Vec<String> = p
            .get("bench")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|x| x.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        let issue = p.get("issue").and_then(|v| v.as_str()).map(String::from);
        if bench.is_empty() {
            return Err(KernelError::InvalidInput(
                "a convening records at least one seat".into(),
            ));
        }
        // D10 convening half: bench size must be the constituted odd size for the
        // tier. Same shared kernel check the CLI convene path uses (#12), so they
        // cannot drift (D4: the kernel is the only smart point).
        let lawpack = load_lawpack(&self.repo_root)?;
        if let Some(constitution) = lawpack
            .orders
            .iter()
            .find(|o| o.id == "2026-VJS-COURTS-CONSTITUTION-001")
            && let Err(msg) =
                vjs_core::bench::convening_bench_check(constitution, &court, bench.len())
        {
            return Err(KernelError::InvalidInput(msg));
        }
        let subs = vjs_store::Store::read_submissions(&self.repo_root)?;
        let _sub = subs.iter().find(|s| s.id == submission).ok_or_else(|| {
            KernelError::InvalidInput(format!("no filed submission {submission}"))
        })?;
        // Pin the case file by the digest of the RAW bytes on disk, exactly as the CLI
        // convene path does (lifecycle.rs). Re-serialising the parsed struct would hash
        // a kernel-normalised form, not the document the parties actually filed - field
        // reordering, dropped unknown keys, or comment loss would all change the bytes
        // and break the pin. The whole point of a case-file digest is byte-faithfulness.
        let sub_path = self
            .repo_root
            .join(".vjs/submissions/filed")
            .join(format!("{submission}.yaml"));
        let bytes = std::fs::read(&sub_path).map_err(|e| {
            KernelError::Io(format!("reading filed submission {}: {e}", sub_path.display()))
        })?;
        use sha2::Digest;
        let digest = format!("sha256:{}", hex::encode(sha2::Sha256::digest(&bytes)));
        let rec = vjs_store::ConveningRecord {
            id: format!(
                "CONVENING-{}-{}",
                court,
                chrono::Utc::now().format("%Y-%m-%d-%H%M%S")
            ),
            court,
            submission_id: submission,
            issue,
            case_file_digest: digest.clone(),
            bench,
            convened_at: chrono::Utc::now().to_rfc3339(),
        };
        vjs_store::Store::write_convening(&self.repo_root, &rec)?;
        Ok(serde_json::json!({ "convening": rec.id, "case_file_digest": digest }))
    }

    /// record -> order: verify bench-integrity (the PC-13 D10 gate) against the
    /// constitution and write the order. A non-assented bench defect is refused; an
    /// assented order's defect is left to route-for-correction at validate (the floor).
    /// The absolute-path commit hook remains the wall; this verb is the door.
    fn handle_record(&self, params: Option<Value>) -> Result<Value, KernelError> {
        let p = params.ok_or_else(|| KernelError::InvalidInput("params required".into()))?;
        let order: Order = serde_json::from_value(p)
            .map_err(|e| KernelError::InvalidInput(format!("order: {e}")))?;
        // PC-19 apex routing, in the typed record verb. Only the apex seat ("vjs") may
        // RECORD an above-County order; a subscribing jurisdiction records only its
        // first-instance County line and refers anything higher up. The commit hook's
        // path scan (hook.rs) DELIBERATELY excludes lawpack/ paths, so this verb - which
        // writes straight into lawpack/v2/orders - is the one chokepoint where that gap
        // must be closed on the typed `order.court`, not by path. Mirrors front.rs's
        // APEX_SEAT and apex_routing_decision: same rule, same shared meaning (D4).
        const APEX_SEAT: &str = "vjs";
        let jurisdiction_id = vjs_store::Store::read_repo_config(&self.repo_root)?
            .map(|c| c.jurisdiction_id)
            .unwrap_or_else(|| APEX_SEAT.to_string());
        let above_county = matches!(
            order.court,
            Court::CourtOfAppeal | Court::PrivyCouncil | Court::SupremeCourt
        );
        if above_county && jurisdiction_id != APEX_SEAT {
            return Err(KernelError::InvalidInput(format!(
                "jurisdiction '{jurisdiction_id}' is a subscribing seat and may record only a \
                 first-instance County order; an above-County ruling ({:?}) refers up to the \
                 apex seat '{APEX_SEAT}' ([2026] VJS-PC 19)",
                order.court
            )));
        }
        let lawpack = load_lawpack(&self.repo_root)?;
        if let Some(constitution) = lawpack
            .orders
            .iter()
            .find(|o| o.id == "2026-VJS-COURTS-CONSTITUTION-001")
        {
            let opinion_text = order
                .source_opinion
                .as_ref()
                .and_then(|sp| std::fs::read_to_string(self.repo_root.join(sp)).ok());
            let defects =
                vjs_core::bench::verify_bench(&order, constitution, opinion_text.as_deref());
            // Bench-integrity (a constituted odd size; no silent seat) is a CONSTITUTIVE
            // code, not an assent-protected one: it is what makes the order a real court
            // ruling at all. The assented-record floor routes-for-correction the codes it
            // protects, but constitutive codes (bench, citation collision, apex, canon
            // boundary) are never assent-downgraded. So a bench defect is refused at the
            // door regardless of assent - assent cannot manufacture a quorum that the
            // constitution did not seat ([2026] VJS-SC 2; the assent-floor carve-out).
            if !defects.is_empty() {
                return Err(KernelError::InvalidInput(format!(
                    "bench-integrity defects, refused at the door (constitutive - not assent-softenable): {:?}",
                    defects.iter().map(|d| d.code()).collect::<Vec<_>>()
                )));
            }
        }
        let dir = self.repo_root.join("lawpack/v2/orders");
        std::fs::create_dir_all(&dir).map_err(|e| KernelError::Io(e.to_string()))?;
        let path = dir.join(format!("{}.yaml", order.id));
        let yaml =
            serde_yaml::to_string(&order).map_err(|e| KernelError::Serialization(e.to_string()))?;
        std::fs::write(&path, yaml).map_err(|e| KernelError::Io(e.to_string()))?;
        Ok(serde_json::json!({ "recorded": order.id, "path": path.display().to_string() }))
    }
}

/// The pure cage-auth decision (#17/#16): when `expected` is empty there is no auth
/// (the dev default); otherwise the request must carry a matching `_token`. Pure, so
/// it is testable without touching the process environment.
pub fn auth_satisfied(expected: &str, params: Option<&Value>) -> bool {
    if expected.is_empty() {
        return true;
    }
    params
        .and_then(|p| p.get("_token"))
        .and_then(|t| t.as_str())
        == Some(expected)
}

fn build_context(repo: &std::path::Path) -> Result<KernelContext, KernelError> {
    let lawpack = load_lawpack(repo)?;
    let graph = lawpack.build_authority_graph()?;
    let digest = compute_digest(repo)?;

    Ok(KernelContext {
        authority_graph: graph,
        limits: ContextLimits::default(),
        lawpack_digest: digest,
    })
}

fn load_lawpack(repo: &std::path::Path) -> Result<Lawpack, KernelError> {
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
            obligations: Vec::new(),
        })
    }
}

fn compute_digest(repo: &std::path::Path) -> Result<String, KernelError> {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    let manifest = repo.join("lawpack/v2/manifest.toml");
    if manifest.exists() {
        let content = std::fs::read(&manifest).map_err(|e| KernelError::Io(e.to_string()))?;
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

#[cfg(test)]
mod front_door_verb_tests {
    use super::*;

    /// PC-14 D5: the front-door gap is closed - the surface now exposes the
    /// governed-record-creation verbs (allocate, convene, record) alongside the
    /// lifecycle six, nine in all.
    #[test]
    fn surface_exposes_the_record_creation_verbs() {
        let names: Vec<String> = get_tool_schemas().into_iter().map(|t| t.name).collect();
        assert_eq!(
            names.len(),
            9,
            "six lifecycle + three record-creation verbs"
        );
        for v in ["vjs.allocate", "vjs.convene", "vjs.record"] {
            assert!(names.contains(&v.to_string()), "{v} must be exposed");
        }
    }

    /// An unknown method is refused - the surface is a closed set, not an open shell.
    #[test]
    fn unknown_method_is_refused() {
        let srv = McpServer::new(std::path::PathBuf::from("."));
        let req = r#"{"jsonrpc":"2.0","id":1,"method":"vjs.exec","params":{}}"#;
        assert!(srv.handle_request(req).is_err());
    }

    fn scratch_dir(tag: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!("vjs_mcp_{tag}_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    /// allocate: the Cc series is repo-scoped and must carry the `-<REPO>` segment, the
    /// canon series must NOT. A caller-supplied `repo` selects a subscriber's Cc line;
    /// absent it, the segment defaults to this server's own repo_code. (Audit #10: the
    /// verb was minting segment-less Cc citations that collide across repos.)
    #[test]
    fn allocate_scopes_cc_to_a_repo_segment_and_leaves_canon_unsegmented() {
        let dir = scratch_dir("alloc"); // no config.toml -> repo_code defaults to VJS
        let srv = McpServer::new(dir.clone());

        let cc_named = srv
            .handle_allocate(Some(serde_json::json!({"series":"CC","year":2026,"repo":"acmeco"})))
            .unwrap();
        assert_eq!(cc_named["citation"], "[2026] VJS-CC-ACMECO 1");

        let cc_default = srv
            .handle_allocate(Some(serde_json::json!({"series":"cc","year":2026})))
            .unwrap();
        assert_eq!(cc_default["citation"], "[2026] VJS-CC-VJS 1");

        let canon = srv
            .handle_allocate(Some(serde_json::json!({"series":"PC","year":2026})))
            .unwrap();
        assert_eq!(canon["citation"], "[2026] VJS-PC 1");

        let _ = std::fs::remove_dir_all(&dir);
    }

    /// record: PC-19 apex routing in the typed verb. A subscribing jurisdiction (here
    /// "acmeco") may record only its first-instance County order; an above-County ruling
    /// refers up to the apex seat. The commit hook's path scan excludes lawpack/, so this
    /// verb is the chokepoint. The refusal fires BEFORE the lawpack is loaded, so the
    /// fixture needs only the config. (Audit #10: the verb had no apex check at all.)
    #[test]
    fn record_refuses_an_above_county_order_from_a_subscriber() {
        let dir = scratch_dir("apex");
        std::fs::create_dir_all(dir.join(".vjs")).unwrap();
        std::fs::write(
            dir.join(".vjs/config.toml"),
            "version = \"2\"\njurisdiction_id = \"acmeco\"\nrepo_code = \"ACMECO\"\nlawpack = \"vjs-v2@0.1.0\"\n\n[paths]\norders = \".vjs/orders\"\nlogs = \".vjs/logs\"\nsubmissions = \".vjs/submissions\"\nspecs = \"lawpack/v2/specs\"\ndecisions = \"lawpack/v2/decisions\"\nproofs = \".vjs/proofs\"\npermits = \".vjs/permits\"\nprivate = \".vjs/private\"\ncache = \".vjs/cache\"\n\n[paths.public]\nenabled = false\n",
        )
        .unwrap();
        let srv = McpServer::new(dir.clone());

        let order = serde_json::json!({
            "id": "TEST-SUPREME-1",
            "court": "supreme_court",
            "jurisdiction": "acmeco",
            "repo_code": "ACMECO",
            "status": "binding",
            "issue": "test/apex",
            "holding": "a subscriber tries to record an apex order",
            "directives": [],
            "forbidden": null,
            "exceptions": null,
            "supersedes": [],
            "source_opinion": null,
            "runtime_summary": "test",
            "created_at": "2026-06-26"
        });
        let err = srv.handle_record(Some(order)).unwrap_err();
        let msg = format!("{err:?}");
        assert!(
            msg.contains("refers up") && msg.contains("VJS-PC 19"),
            "subscriber apex record must refer up, got: {msg}"
        );

        // And a County order from the same subscriber is NOT refused by the apex gate
        // (it falls through to the ordinary bench/lawpack path - a different failure, or
        // success, but never the apex refusal).
        let county = serde_json::json!({
            "id": "TEST-COUNTY-1", "court": "county", "jurisdiction": "acmeco",
            "repo_code": "ACMECO", "status": "binding", "issue": "test/county",
            "holding": "first-instance", "directives": [], "forbidden": null,
            "exceptions": null, "supersedes": [], "source_opinion": null,
            "runtime_summary": "test", "created_at": "2026-06-26"
        });
        let county_res = srv.handle_record(Some(county));
        if let Err(e) = county_res {
            assert!(
                !format!("{e:?}").contains("refers up"),
                "a County order must never trip the apex refusal"
            );
        }

        let _ = std::fs::remove_dir_all(&dir);
    }
}
