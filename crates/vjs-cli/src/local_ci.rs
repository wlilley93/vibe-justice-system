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

    // Step 4: canon order validation ([2026] VJS-CC-VJS 11 D2/D4).
    //
    // This stage used to check three emptiness conditions and print "Orders valid". It
    // never ran verify_bench, so it could not see BENCH_OPINION_MISSING, a CONSTITUTIVE
    // code. D2 adds that check. The scope is deliberately UNCHANGED: `lawpack.orders`
    // only. That is not an oversight and must not be "fixed" casually.
    //
    // No County order has ever been in this stage's scope, in any run, in any clone,
    // because County orders live under .vjs and this reads the lawpack. Thirteen binding
    // records carry a bench with no opinion. Widening to the .vjs roots would turn every
    // push red on them, so D3 STAYS that widening until the higher court disposes of
    // whether those records are void or routed for correction (SUBMISSION-2026-07-27-201553).
    // Widening early would decide by remedy a question this court referred up.
    //
    // The stage name and message therefore say CANON, so a pass here is never read as
    // "every order in the estate is valid", which is the exact overclaim D4 forbids.
    let mut order_findings: Vec<String> = Vec::new();
    let constitution = lawpack
        .orders
        .iter()
        .find(|o| o.id == vjs_core::bench::COURTS_CONSTITUTION_ID);
    for order in &lawpack.orders {
        if order.holding.is_empty()
            || order.directives.is_empty()
            || order.runtime_summary.is_empty()
        {
            order_findings.push(format!("{}: missing holding/directives/runtime_summary", order.id));
        }
        if order.bench.is_empty() {
            continue;
        }
        match constitution {
            Some(c) => {
                let opinion_text = order
                    .source_opinion
                    .as_ref()
                    .and_then(|p| std::fs::read_to_string(repo.join(p)).ok());
                for d in vjs_core::bench::verify_bench(order, c, opinion_text.as_deref()) {
                    order_findings.push(format!("{}: {}", order.id, d.code()));
                }
            }
            // Never silently pass a bench that could not be checked (D4).
            None => order_findings.push(format!("{}: BENCH_UNCHECKED", order.id)),
        }
    }
    // The two canon bench defects this check found on its FIRST EVER run, pinned exactly.
    //
    // Both are real and both are serious. On [2026] VJS-PC 8, Tindale is named on a
    // three-seat bench and owns 29 characters of the opinion while the other two own ~930
    // each. On [2026] VJS-PC 19, Wilberforce is mentioned once and owns 51 characters of a
    // 16,080-character opinion. A named seat that never spoke means a three-seat order was
    // decided by two, which is not the constituted size.
    //
    // They are NOT suppressed and they are NOT excused. Whether a pre-existing constitutive
    // bench defect voids its order is REFERRED UP (SUBMISSION-2026-07-27-201553), and
    // [2026] VJS-CC-VJS 11 holds that a repair must not "convert a referral into a
    // stoppage" nor achieve by remedy what First Instance cannot order directly. Failing
    // this gate closed on them would DECIDE that referral in favour of void, by default,
    // which is the one outcome this court was told it could not reach. So they are pinned
    // as a ratchet on the same discipline as the structural exemptions: compared EXACTLY,
    // so a NEW defect fails and a FIXED one also fails until it is removed from this list.
    // The list may only shrink. It is deleted entirely when the higher court rules.
    const KNOWN_CANON_BENCH_DEFECTS: &[&str] = &[
        "2026-VJS-PC-008: BENCH_SILENT_SEAT",
        "2026-VJS-PC-019: BENCH_SILENT_SEAT",
    ];
    let mut known: Vec<&str> = KNOWN_CANON_BENCH_DEFECTS.to_vec();
    known.sort_unstable();
    let mut found: Vec<&str> = order_findings.iter().map(|s| s.as_str()).collect();
    found.sort_unstable();
    let novel: Vec<&str> = found
        .iter()
        .filter(|f| !known.contains(f))
        .copied()
        .collect();
    let stale: Vec<&str> = known
        .iter()
        .filter(|k| !found.contains(k))
        .copied()
        .collect();
    let order_ok = novel.is_empty() && stale.is_empty();
    steps.push(CiStep {
        name: "canon_order_validate".into(),
        passed: order_ok,
        message: if order_ok {
            format!(
                "{} canon orders checked incl. bench-integrity; {} known defect(s) held sub judice \
                 (CC-VJS 11); .vjs orders OUT OF SCOPE pending the referral (CC-VJS 11 D3)",
                lawpack.orders.len(),
                known.len()
            )
        } else {
            let mut m = String::new();
            if !novel.is_empty() {
                m.push_str(&format!("NEW canon order defects: {}. ", novel.join("; ")));
            }
            if !stale.is_empty() {
                m.push_str(&format!(
                    "Pinned defects no longer present, remove them from \
                     KNOWN_CANON_BENCH_DEFECTS: {}. ",
                    stale.join("; ")
                ));
            }
            m
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
