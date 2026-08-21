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

    // The store register (ACT-PROCEEDINGS-DISCIPLINE s13): an unregistered law
    // store is REPORTED here, in local-ci, which is where the Act says the report
    // lives - the same gate validate runs, so the two doors cannot disagree.
    {
        let mut sr = Vec::new();
        vjs_engine::store_register::store_register_findings(repo, &mut sr);
        let bad: Vec<String> = sr
            .iter()
            .filter(|f| !matches!(f.severity, vjs_core::types::Severity::Info))
            .map(|f| format!("{}: {}", f.code, f.message))
            .collect();
        steps.push(CiStep {
            name: "store_register".into(),
            passed: bad.is_empty(),
            message: if bad.is_empty() {
                format!("store register complete ({} disclosure(s))", sr.len())
            } else {
                bad.join("; ")
            },
        });
        if !bad.is_empty() {
            ok = false;
        }
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
    // No County order had ever been in this stage's scope, in any run, in any clone,
    // because County orders live under .vjs and this reads the lawpack. CC-VJS 11 D3
    // STAYED the widening while it was undecided whether the thirteen flagged records were
    // void; widening then would have decided that by remedy. [2026] VJS-PC 21 has since
    // answered it, so the stay is SPENT and the .vjs roots are handled by their own stage
    // below, on their own footing: flagged, not fatal.
    //
    // This stage still says CANON, and keeps its own scope, so a pass here is never read as
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
            order_findings.push(format!(
                "{}: missing holding/directives/runtime_summary",
                order.id
            ));
        }
        if order.bench.is_empty() {
            continue;
        }
        match constitution {
            Some(c) => {
                let opinion_text = order
                    .source_opinion
                    .as_ref()
                    .and_then(|p| vjs_engine::read_source_opinion(repo, p));
                for d in vjs_core::bench::verify_bench(order, c, opinion_text.as_deref()) {
                    order_findings.push(format!("{}: {}", order.id, d.code()));
                }
            }
            // Never silently pass a bench that could not be checked (D4).
            None => order_findings.push(format!("{}: BENCH_UNCHECKED", order.id)),
        }
    }
    // Held sub judice: EMPTY, and that is the whole story.
    //
    // This check found two canon defects on its first ever run, PC-8 and PC-19, both
    // BENCH_SILENT_SEAT. They were pinned here rather than failed closed, because failing
    // closed would have decided the then-pending referral in favour of VOID by default.
    //
    // [2026] VJS-PC 21 then held the check could not ground a void, and correcting it per
    // D4 showed both findings were FALSE POSITIVES. Tindale and Wilberforce each wrote a
    // full concurring opinion. They were flagged because a seat's owned block ends at the
    // next colleague NAMED, so "I join Marchmont" and "I concur in full with Reid J."
    // handed each judge's own opinion to the colleague they credited. The heuristic
    // punished the courteous style that good judgments use.
    //
    // The list stays as a mechanism with nothing in it. Compared EXACTLY, so a new defect
    // fails AND a pin that is no longer real also fails until removed. That second limb is
    // what forced this entry to be deleted rather than left to rot, which is the point.
    const KNOWN_CANON_BENCH_DEFECTS: &[&str] = &[];
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

    // Step 4b: the correction register ([2026] VJS-PC 21 D2/D3).
    //
    // The thirteen flagged records are BINDING AND FLAGGED: in force, relied upon, and
    // carrying a visible obligation to supply the missing opinion. They are NOT void, NOT
    // stayed and NOT deleted, so this stage must never fail merely because they exist.
    // What it fails on is the REGISTER drifting from reality, in either direction:
    //
    //   * a newly flagged record that is not on the register (an obligation nobody recorded)
    //   * a register row whose record is no longer flagged (an obligation nobody discharged
    //     from the list, which is how a register becomes a place obligations go to be
    //     forgotten)
    //
    // Deliberately a separate stage from canon_order_validate, and named for what it does.
    // Folding .vjs findings into a stage called "canon" would be the same overclaim PC 21
    // and CC-VJS 11 both condemn.
    let register_path = repo.join(".vjs/court/correction-register.tsv");
    let register: std::collections::BTreeSet<String> = std::fs::read_to_string(&register_path)
        .unwrap_or_default()
        .lines()
        .filter(|l| !l.starts_with('#') && !l.starts_with("order_id"))
        .filter_map(|l| l.split('\t').next())
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_string())
        .collect();

    // FLAGGED IS DERIVED FROM THE AT-REST SWEEP, not from a second walk of its own.
    //
    // This stage used to re-implement the whole thing: walk the governed roots, parse
    // each order, read its opinion, verify the bench, and call a record flagged if the
    // opinion was missing. That was a duplicate of `order_checks`, and it had the vice
    // duplicates always have - it could only ever see ONE defect class. A record with a
    // perfectly good opinion and an empty `runtime_summary` carried a real, visible,
    // unrecorded obligation that this register was structurally unable to hold
    // (2026-VJS-CC-CODEX-ORCHESTRATION-LOCUS-008 was exactly that, and it took the D13
    // sweep to see it at all).
    //
    // One sweep, one ledger. Every at-rest finding is an obligation, and the register is
    // the exact set of records carrying one - so a NEW defect fails until it is recorded,
    // and a row whose defect is gone fails until it is removed. The second limb is the
    // one that matters: it is what stops a register becoming a place obligations go to
    // be forgotten.
    let lawpack_for_sweep = vjs_engine::load_lawpack(repo)?;
    let mut flagged: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for fnd in vjs_engine::order_checks::at_rest_order_findings_raw(repo, &lawpack_for_sweep) {
        // AN OBLIGATION IS A FINDING THAT WOULD REFUSE THE RECORD IF YOU WROTE IT TODAY.
        // Advisories are not obligations: TIER_ADVISORY says "possibly under-tiered" and
        // PC-17 D3 holds ORDER_CITATION_NOT_IN_FORCE "advisory only, never blocks". A
        // register carrying those is a list of opinions, not a ledger of what is owed.
        // Keyed on severity rather than on a list of codes, so a future check joins the
        // right side of the line by being written correctly.
        if fnd.severity != vjs_core::types::Severity::Fatal {
            continue;
        }
        let Some(path) = fnd.path.as_ref() else {
            continue;
        };
        // ORDERS ONLY, on the same test the walk used: orders live under an `orders/`
        // dir in every root. A convening carries a `bench:` too and owes no opinion.
        if !path.parent().is_some_and(|d| d.ends_with("orders")) {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(repo.join(path)) else {
            continue;
        };
        if let Some(id) = text.lines().find_map(|l| {
            let v = l
                .strip_prefix("id:")?
                .trim()
                .trim_matches('"')
                .trim_matches('\'');
            (!v.is_empty()).then(|| v.to_string())
        }) {
            flagged.insert(id);
        }
    }

    let unrecorded: Vec<&String> = flagged.difference(&register).collect();
    let discharged: Vec<&String> = register.difference(&flagged).collect();
    let register_ok = unrecorded.is_empty() && discharged.is_empty();
    steps.push(CiStep {
        name: "correction_register".into(),
        passed: register_ok,
        message: if register_ok {
            // ZERO FINDINGS AND ZERO CHECKERS ARE NOT ONE NUMBER. Once the court store
            // left version tracking under [2026] VJS-CC-VJS 20 D2, a fresh clone carries
            // no `.vjs/court` at all - and this stage went on reporting "0 record(s)
            // binding-and-flagged, all on the register", which is a claim about a corpus
            // it never opened. Both sides being empty is genuine AGREEMENT here and not a
            // false pass, because an estate with no court records really does owe no
            // corrections; but the sentence has to say which of the two it is, or every
            // subscriber's CI recites a clean audit of nothing.
            if !repo.join(".vjs/court").exists() {
                "the correction register DID NOT RUN - this tree carries no .vjs/court \
                 store, so there are no orders to flag and no register to compare. A \
                 statement about this estate, never a finding about the corpus \
                 ([2026] VJS-CC-VJS 20 D2 untracked the store; PC 21 D2/D3)."
                    .to_string()
            } else {
                format!(
                    "{} record(s) binding-and-flagged, all on the register (PC 21 D2/D3); \
                     flagged is not fatal, register drift is",
                    flagged.len()
                )
            }
        } else {
            let mut m = String::new();
            if !unrecorded.is_empty() {
                m.push_str(&format!(
                    "flagged but NOT on the register, add them with today's date: {:?}. ",
                    unrecorded
                ));
            }
            if !discharged.is_empty() {
                m.push_str(&format!(
                    "on the register but no longer flagged, remove the row: {:?}. ",
                    discharged
                ));
            }
            m
        },
    });
    if !register_ok {
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
