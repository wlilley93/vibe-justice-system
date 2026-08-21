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
        // ONLY AN INVOKED JURISDICTION HAS AN AUDIT TRAIL (Defect 1 of the 2026-08-05
        // two-defects submission): tests construct servers whose repo_root is a crate
        // directory or the workspace, and the sink then scattered .vjs/audit/ files
        // into SOURCE TREES, where a test-run byproduct reads as a governed record.
        // The gate is the same one load_lawpack keys on: .vjs/config.toml. A scratch
        // fixture that wants an audit trail invokes itself; a bare directory never
        // acquires governance byproducts by being pointed at.
        if !self.repo_root.join(".vjs/config.toml").exists() {
            return;
        }
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

        // O5: no binding instruction from a partial corpus.
        vjs_engine::context::refuse_if_orders_unreadable(&self.repo_root)?;
        let ctx = vjs_engine::build_kernel_context(&self.repo_root)?;
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

        // O5: no binding instruction from a partial corpus.
        vjs_engine::context::refuse_if_orders_unreadable(&self.repo_root)?;
        let ctx = vjs_engine::build_kernel_context(&self.repo_root)?;
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
        // Same roots as the CLI front door, in the same change. D4 requires one
        // shared meaning across both doors, and two doors reading different
        // registers is precisely how they come apart.
        let roots = front_door::governed_record_roots(&self.repo_root);
        let max = LawpackValidator::live_citation_max(&roots, &series, repo_for_lookup, year)?;
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
        let lawpack = vjs_engine::load_lawpack(&self.repo_root)?;
        if let Some(constitution) = lawpack
            .orders
            .iter()
            .find(|o| o.id == vjs_core::bench::COURTS_CONSTITUTION_ID)
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
            KernelError::Io(format!(
                "reading filed submission {}: {e}",
                sub_path.display()
            ))
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
        // first-instance County line and refers anything higher up. This is a check on the
        // typed `order.court` and NOT on a path, so it does not move with the destination:
        // [2026] VJS-CC-VJS 16 C1 changed where this verb writes and left this refusal
        // exactly here, because what makes it necessary is that a record-creation verb can
        // mint an above-County order at all, not which directory it lands in. Mirrors
        // front.rs's APEX_SEAT and apex_routing_decision: same rule, same meaning (D4).
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
        // The ENGINE's loader, so this verb resolves the canon the same way every other
        // door does. When it was this crate's own, a jurisdiction that vendored no copy
        // got an EMPTY lawpack here, the `find` below returned None, the whole bench check
        // sat inside `if let Some(constitution)` and was therefore SKIPPED, and the order
        // was written. A constitutive gate that is skipped when the constitution cannot be
        // found is a gate that fails OPEN ([2026] VJS-CC-VJS 15).
        let lawpack = vjs_engine::load_lawpack(&self.repo_root)?;
        if let Some(constitution) = lawpack
            .orders
            .iter()
            .find(|o| o.id == vjs_core::bench::COURTS_CONSTITUTION_ID)
        {
            let opinion_text = order
                .source_opinion
                .as_ref()
                .and_then(|sp| vjs_engine::read_source_opinion(&self.repo_root, sp));
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
        // ONE KIND OF RECORD, ONE DESTINATION ([2026] VJS-CC-VJS 16 C1).
        //
        // This verb wrote into the CANON TREE's orders directory and CREATED that directory
        // if it was absent. Measured 2026-08-01 on a jurisdiction subscribing to the canon
        // out of tree: one valid `record` call manufactured the canon root, the resolver
        // prefers a vendored directory, and a 160-file constitutional canon was replaced by
        // a one-file one. `lookup` then returned the new order and nothing else; the
        // canon-boundary gate went silent because `resolve_canon_repo_code` found no
        // manifest and fell back to the subscriber's own repo_code; and zero invariants
        // evaluated while the reporter said "all passed". The order was not malformed -
        // that is what the verb did when it worked.
        //
        // A verb that records a jurisdiction's own governed record writes it to that
        // jurisdiction's LOCAL record store and never into the canon tree. This is the same
        // function `vjs order apply` calls (`Store::write_order` -> `.vjs/orders`), so the
        // two doors now have one destination and the marker that declared this site as a
        // reserved write-target is gone with the site.
        let path = vjs_store::Store::write_order(&self.repo_root, &order)?;
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

mod schemas;
pub use schemas::{McpTool, get_tool_schemas};

#[cfg(test)]
mod tests;
