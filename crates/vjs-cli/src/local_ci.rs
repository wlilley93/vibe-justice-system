//! The `vjs local-ci` command: run the full local conformance gate set over the repo.

use super::*;

pub(crate) fn cmd_local_ci(repo: &Path, json: bool) -> Result<(), KernelError> {
    let mut ok = true;
    let mut steps = Vec::new();

    // Step 1: Lawpack validation
    let lawpack = load_lawpack(repo)?;
    let report = LawpackValidator::validate(&lawpack)?;
    steps.push(CiStep {
        name: "lawpack_validate".into(),
        passed: report.ok,
        message: if report.ok {
            "Lawpack valid".into()
        } else {
            "Lawpack invalid".into()
        },
    });
    if !report.ok {
        ok = false;
    }

    // Step 2: Citation check
    let mut citation_ok = true;
    let mut seen = std::collections::HashSet::new();
    for order in &lawpack.orders {
        if !seen.insert(order.id.clone()) {
            citation_ok = false;
        }
    }
    steps.push(CiStep {
        name: "citation_check".into(),
        passed: citation_ok,
        message: if citation_ok {
            "No citation collisions".into()
        } else {
            "Citation collision detected".into()
        },
    });
    if !citation_ok {
        ok = false;
    }

    // Step 3: Boundary scan
    let vjs_dir = repo.join(".vjs");
    let boundary_ok = if vjs_dir.exists() {
        let findings = RedactScanner::scan_directory(&vjs_dir)?;
        RedactScanner::check_public_safe(&findings)
    } else {
        true
    };
    steps.push(CiStep {
        name: "boundary_scan".into(),
        passed: boundary_ok,
        message: if boundary_ok {
            "Boundary scan passed".into()
        } else {
            "Boundary scan failed".into()
        },
    });
    if !boundary_ok {
        ok = false;
    }

    // Step 4: Order validation
    let mut order_ok = true;
    for order in &lawpack.orders {
        if order.holding.is_empty()
            || order.directives.is_empty()
            || order.runtime_summary.is_empty()
        {
            order_ok = false;
        }
    }
    steps.push(CiStep {
        name: "order_validate".into(),
        passed: order_ok,
        message: if order_ok {
            "Orders valid".into()
        } else {
            "Invalid orders found".into()
        },
    });
    if !order_ok {
        ok = false;
    }

    // Step 5: Invariant evaluation
    let repo_state = RepoScanner::build_repo_state(repo)?;
    let facts = vjs_lawpack::lawpack_facts(repo, &lawpack);
    let invariant_report = evaluate_invariants(&repo_state, &lawpack.invariants, &facts)?;
    let invariant_ok = invariant_report.findings.iter().all(|f| f.passed);
    steps.push(CiStep {
        name: "invariant_eval".into(),
        passed: invariant_ok,
        message: if invariant_ok {
            format!("{} invariants passed", invariant_report.findings.len())
        } else {
            let failures: Vec<_> = invariant_report
                .findings
                .iter()
                .filter(|f| !f.passed)
                .map(|f| f.invariant_id.0.clone())
                .collect();
            format!("Invariant failures: {}", failures.join(", "))
        },
    });
    if !invariant_ok {
        ok = false;
    }

    // Step 6: Permit gate
    let config = Store::read_repo_config(repo)?;
    let (permit_required, permit_exempt) = if let Some(ref cfg) = config {
        let req = cfg
            .governance
            .as_ref()
            .map(|g| g.permit_required.clone())
            .unwrap_or_default();
        let ex = cfg
            .governance
            .as_ref()
            .map(|g| g.permit_exempt.clone())
            .unwrap_or_default();
        (req, ex)
    } else {
        (Vec::new(), Vec::new())
    };

    let staged_files = GitIntegration::read_staged_files(repo)?;
    let staged_paths: Vec<PathBuf> = staged_files.iter().map(PathBuf::from).collect();
    let permits = Store::read_permits(repo)?;
    let logs = Store::read_logs(repo)?;
    let proofs = Store::read_proofs(repo)?;

    let gate_result = PermitGate::evaluate(
        &staged_paths,
        &permits,
        &logs,
        &proofs,
        &permit_required,
        &permit_exempt,
    );

    let permit_gate_ok = gate_result.ok;
    steps.push(CiStep {
        name: "permit_gate".into(),
        passed: permit_gate_ok,
        message: if permit_gate_ok {
            format!("{} staged paths, permit gate passed", staged_paths.len())
        } else {
            let failures: Vec<_> = gate_result
                .findings
                .iter()
                .filter(|f| matches!(f.severity, Severity::Fatal | Severity::Error))
                .map(|f| f.code.clone())
                .collect();
            format!("Permit gate failures: {}", failures.join(", "))
        },
    });
    if !permit_gate_ok {
        ok = false;
    }

    let result = CiResult { ok, steps };

    if json {
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
    } else {
        println!("Local CI: {}", if ok { "PASS" } else { "FAIL" });
        for step in &result.steps {
            println!(
                "  {}: {}",
                step.name,
                if step.passed { "PASS" } else { "FAIL" }
            );
        }
    }

    if !ok {
        std::process::exit(1);
    }

    Ok(())
}
