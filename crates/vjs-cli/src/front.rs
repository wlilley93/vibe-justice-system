//! Front-of-house commands: init, route, hook, and lookup.

use super::*;

pub(crate) fn cmd_init(repo: &Path, lawpack: Option<String>) -> Result<(), KernelError> {
    let git_root = GitIntegration::find_repo_root(repo)?;
    let target = git_root.as_deref().unwrap_or(repo);

    Store::init_repo(target)?;
    GitIntegration::install_hooks(target)?;

    // Write AGENTS.md block
    let agents_md = target.join("AGENTS.md");
    let content = "# VJS V2 Agent Contract\n\nThis repo is governed by VJS V2.\n\nBefore governed load-bearing work, call `vjs.route`.\nIf the route is settled, follow the returned orders/rules.\nIf `court_required=true`, file a short submission.\nAfter material implementation decisions, write a decision log.\nDo not place private repo facts in public records.\nThe kernel answer is the runtime authority surface.\n";
    if !agents_md.exists() {
        std::fs::write(&agents_md, content).map_err(|e| KernelError::Io(e.to_string()))?;
    }

    let lawpack_id = lawpack.as_deref().unwrap_or("vjs-v2");
    Store::write_lawpack_lock(target, lawpack_id, "0.1.0", "sha256:placeholder")?;

    println!("VJS V2 initialized in {}", target.display());
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn cmd_route(
    repo: &Path,
    kind: String,
    issue: Option<String>,
    risk: Option<String>,
    intent: String,
    public: bool,
    external: bool,
    irreversible: bool,
    paths: Vec<PathBuf>,
    gate: bool,
    json: bool,
) -> Result<(), KernelError> {
    let action_kind = parse_action_kind(&kind);
    let risk_level = parse_risk_level(risk.as_deref().unwrap_or("low"));
    let issue_tags = issue.map(|i| vec![IssueTag(i)]).unwrap_or_default();
    let path_globs: Vec<String> = paths
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();

    let input = RouteInput {
        repo_root: Some(repo.to_path_buf()),
        jurisdiction: Some(JurisdictionId("default".into())),
        actor: "lexby".into(),
        action_kind,
        issue_tags,
        intent,
        affected_paths: paths.clone(),
        risk: risk_level,
        public_target: public,
        external_target: external,
        irreversible,
        user_instruction: None,
    };

    let ctx = build_kernel_context(repo)?;
    let decision = route(input, &ctx)?;

    // Save permit if one was created
    if let Some(ref permit_id) = decision.permit_id {
        let route_id = format!("ROUTE-{}", chrono::Utc::now().format("%Y%m%d-%H%M%S"));
        let scope = if path_globs.is_empty() {
            None
        } else {
            Some(Scope {
                paths: Some(path_globs.clone()),
                jurisdictions: None,
                action_kinds: None,
                issue_tags: None,
                records: None,
            })
        };
        // PC-16 D3: bind the self-issued permit to its actor + route + concrete scope and
        // record, in plain terms, that it is an agent-routed self-issue (not an authority
        // approval) so the audit trail never reads as more than it is.
        let intent_digest = vjs_core::spec::permit_intent_digest("lexby", &route_id, &scope);
        let permit = Permit {
            id: permit_id.clone(),
            route_id: RouteId(route_id),
            actor: "lexby".into(),
            scope,
            obligations: decision.obligations.clone(),
            expires_at: (chrono::Utc::now() + chrono::Duration::hours(24)).to_rfc3339(),
            status: PermitStatus::Active,
            self_issued: true,
            meaning: Some(vjs_core::spec::SELF_ISSUED_MEANING.into()),
            intent_digest: Some(intent_digest),
            // K-17: the grant carries its law_source (the binding authorities the route
            // grounded it in), not merely its route_id.
            law_source: decision.binding.iter().map(|a| a.id.0.clone()).collect(),
        };
        Store::write_permit(repo, &permit)?;
    }

    if json {
        println!("{}", serde_json::to_string_pretty(&decision).unwrap());
    } else {
        println!("Decision: {:?}", decision.decision);
        println!("Court required: {}", decision.court_required);
        println!("Log required: {}", decision.log_required);
        if !decision.binding.is_empty() {
            println!("Binding authorities:");
            for auth in &decision.binding {
                println!("  - {} ({})", auth.id.0, auth.summary);
            }
        }
        if !decision.must_do.is_empty() {
            println!("Must do:");
            for item in &decision.must_do {
                println!("  - {}", item);
            }
        }
        if !decision.must_not_do.is_empty() {
            println!("Must not do:");
            for item in &decision.must_not_do {
                println!("  - {}", item);
            }
        }
        if let Some(ref permit_id) = decision.permit_id {
            println!("Permit: {}", permit_id.0);
        }
    }

    // Function-hook mode: fail closed so an executable hook can gate the action.
    if gate {
        match decision.decision {
            RouteOutcome::CourtRequired => {
                eprintln!(
                    "BLOCK: court required - convene on own motion; do not route the fork to the Principal."
                );
                std::process::exit(2);
            }
            RouteOutcome::Blocked => {
                eprintln!("BLOCK: action blocked by the kernel.");
                std::process::exit(2);
            }
            RouteOutcome::HumanApprovalRequired
            | RouteOutcome::ReleaseWarrantRequired
            | RouteOutcome::PrivateBoundaryRequired => {
                eprintln!("BLOCK: a warrant/approval/boundary condition must be satisfied first.");
                std::process::exit(3);
            }
            _ => {}
        }
    }

    Ok(())
}

pub(crate) fn cmd_hook(
    repo: &Path,
    event: String,
    mut paths: Vec<PathBuf>,
    tool: Option<String>,
    stdin_json: bool,
    json: bool,
) -> Result<(), KernelError> {
    let hook_event = vjs_core::hook::parse_event(&event)
        .ok_or_else(|| KernelError::InvalidInput(format!("unknown hook event: {}", event)))?;
    if stdin_json {
        // The agent-harness payload: extract the written path, relativise it
        // to this jurisdiction; payloads without one (or outside it) pass.
        let mut buf = String::new();
        use std::io::Read;
        let _ = std::io::stdin().read_to_string(&mut buf);
        let payload: serde_json::Value = serde_json::from_str(&buf).unwrap_or_default();
        let file = payload["tool_input"]["file_path"]
            .as_str()
            .or_else(|| payload["tool_input"]["notebook_path"].as_str())
            .map(PathBuf::from);
        match file {
            Some(f) => {
                let abs = if f.is_absolute() { f } else { repo.join(f) };
                match abs.strip_prefix(repo) {
                    Ok(rel) => paths.push(rel.to_path_buf()),
                    Err(_) => return Ok(()), // outside this jurisdiction
                }
            }
            None => return Ok(()),
        }
    }
    let input = vjs_core::hook::HookInput {
        event: hook_event,
        repo_root: repo.to_path_buf(),
        actor: "lexby".into(),
        paths,
        tool,
    };
    let ctx = build_kernel_context(repo)?;
    // Permit-aware: a governed write under an active in-scope permit passes;
    // an unpermitted one fails closed with the route as the remedy.
    let permits = Store::read_permits(repo).unwrap_or_default();
    let cfg = Store::read_repo_config(repo).ok().flatten();
    let jurisdiction_id = cfg
        .as_ref()
        .map(|c| c.jurisdiction_id.clone())
        .unwrap_or_default();
    let (required, exempt) = cfg
        .as_ref()
        .and_then(|c| c.governance.clone())
        .map(|g| (g.permit_required, g.permit_exempt))
        .unwrap_or_default();
    // REG-FEDERATION-COORDINATION-001 kernel-checkable bright-line (giving effect to [2026] VJS-SC 1): a
    // subscribing jurisdiction may not assert an apex/final-court function, i.e. may not record a supreme/privy
    // court order; it must refer up. The apex seat is the canonical VJS jurisdiction.
    const APEX_SEAT: &str = "vjs";
    // The ONE resolver, shared with the staged commit gate: the lawpack's own declaration
    // first, this repo's config only where the lawpack is silent. The two call sites used to
    // carry two different chains (only the staged one had the "VJS" tail), so the same tree
    // could be gated against two different codes depending on which door the write came in by.
    let canon_repo_code = vjs_redact::resolve_canon_repo_code(
        repo,
        cfg.as_ref().and_then(|c| c.repo_code.as_deref()),
        Some(jurisdiction_id.as_str()),
    );
    // Canon-write gate ([2026] VJS-PC 13 D1), pre_write half: best-effort on content
    // (the file may not be on disk yet; the authoritative bite is validate --staged).
    let canon_block = |paths: &[PathBuf]| -> Option<vjs_core::hook::HookDecision> {
        let cf = RedactScanner::scan_canon_writes(repo, paths, &canon_repo_code);
        let first = cf
            .into_iter()
            .find(|x| matches!(x.severity, Severity::Error | Severity::Fatal))?;
        let where_ = first
            .path
            .as_ref()
            .and_then(|p| p.file_name())
            .map(|f| f.to_string_lossy().to_string())
            .unwrap_or_default();
        Some(vjs_core::hook::HookDecision::Block(
            vjs_core::hook::Finding {
                code: "CANON_BOUNDARY_VIOLATION".into(),
                message: format!(
                    "{where_} carries subscriber-scoped content (private repo path or repo_code). \
                 Canon holds system data only; file it in the subscriber's own .justice/."
                ),
                next: Some("move to subscriber .justice/".into()),
            },
        ))
    };
    // PC-13 D4, pre_write half: fail closed on an incomplete install surface before
    // any governed write proceeds (REG-INVOCATION-001).
    let install_block = || -> Option<vjs_core::hook::HookDecision> {
        let d = vjs_core::install::verify_surface(repo).into_iter().next()?;
        Some(vjs_core::hook::HookDecision::Block(
            vjs_core::hook::Finding {
                code: d.code().into(),
                message: "Install incomplete: governance is only partly active. Run vjs invoke \
                      --install-hooks then vjs install-lock before governed writes."
                    .into(),
                next: Some("vjs invoke --install-hooks".into()),
            },
        ))
    };
    let decision = vjs_core::hook::apex_routing_decision(&input, &jurisdiction_id, APEX_SEAT)
        .or_else(install_block)
        .or_else(|| canon_block(&input.paths))
        .unwrap_or_else(|| {
            vjs_core::hook::evaluate_governed(&input, &ctx, &permits, &required, &exempt)
        });

    if json {
        println!("{}", serde_json::to_string_pretty(&decision).unwrap());
    } else {
        println!("[{}] {}", decision.code(), decision.message());
    }

    let code = decision.exit_code();
    if code != 0 {
        std::process::exit(code);
    }
    Ok(())
}

pub(crate) fn cmd_lookup(
    repo: &Path,
    issue: Option<String>,
    limit: Option<usize>,
    json: bool,
) -> Result<(), KernelError> {
    let ctx = build_kernel_context(repo)?;
    let issue_tags = issue.map(|i| vec![IssueTag(i)]).unwrap_or_default();

    let input = RouteInput {
        repo_root: Some(repo.to_path_buf()),
        jurisdiction: Some(JurisdictionId("default".into())),
        actor: "lexby".into(),
        action_kind: ActionKind::ImplementationDecision,
        issue_tags,
        intent: "lookup".into(),
        affected_paths: Vec::new(),
        risk: RiskLevel::Low,
        public_target: false,
        external_target: false,
        irreversible: false,
        user_instruction: None,
    };

    let authorities = resolve_authority(&input, &ctx.authority_graph)?;
    let mut authorities = authorities;
    let lim = limit.unwrap_or(5);
    authorities.authorities.truncate(lim);

    if json {
        println!("{}", serde_json::to_string_pretty(&authorities).unwrap());
    } else {
        for auth in &authorities.authorities {
            println!("{} ({:?}): {}", auth.id.0, auth.rank, auth.summary);
        }
    }

    Ok(())
}
