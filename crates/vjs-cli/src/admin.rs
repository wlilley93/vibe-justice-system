//! Administrative commands: next-citation, audit, install-lock, and migrate-v1.

use super::*;

pub(crate) fn cmd_next_citation(
    repo: &Path,
    series: String,
    year: Option<i32>,
    json: bool,
) -> Result<(), KernelError> {
    let y = year.unwrap_or_else(|| chrono::Utc::now().year());
    let repo_code = resolve_repo_code(repo);
    let s = series.to_ascii_uppercase();

    // PC-13 D2: allocate from the live persisted register, not an empty in-memory
    // registry. The register is the governed records themselves: each one carries its
    // own top-level `citation:`, and the highest allocated N is read off them. There is
    // no citator index in this path, and an earlier form of this comment said there
    // was, which certified a D2 compliance the code did not have.
    // The Cc series is bound to THIS repo's code; canon
    // series (PC/SC/REG/ACT/DEC/SPEC/INV/COA/...) carry no repo segment. The next
    // number is one past the current max, so a hand-asserted number cannot mint a
    // citation - validate --staged reconciles and fails closed on any collision.
    // ALL THREE governed-record roots, not lawpack/v2 alone. lawpack/v2 holds no
    // County citation at all, so the old single-root read returned 1 for every CC
    // request while the series stood at 8, and offered a held [2026] VJS-PC 20 on
    // the canon series. The `.exists()` short-circuit to 0 went with it: a missing
    // directory is not evidence that a series is unstarted, and silently allocating
    // 1 is exactly how a collision gets minted.
    let roots = front_door::governed_record_roots(repo);
    let (repo_for_lookup, repo_segment): (Option<&str>, String) = if s == "CC" {
        (Some(repo_code.as_str()), format!("-{}", repo_code))
    } else {
        (None, String::new())
    };
    let max = LawpackValidator::live_citation_max(&roots, &s, repo_for_lookup, y)?;
    let n = max + 1;
    let citation_str = format!("[{}] VJS-{}{} {}", y, s, repo_segment, n);

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "year": y,
                "series": s,
                "repoCode": repo_code,
                "n": n,
                "citation": citation_str
            }))
            .unwrap()
        );
    } else {
        println!("Next citation: {}", citation_str);
    }

    Ok(())
}

pub(crate) fn cmd_audit(repo: &Path, out: Option<PathBuf>, json: bool) -> Result<(), KernelError> {
    let lawpack = load_lawpack(repo)?;
    let report = vjs_lawpack::conformance_audit(&lawpack);

    if json {
        println!("{}", serde_json::to_string_pretty(&report).unwrap());
        return Ok(());
    }

    // Render the conformance map (the D12 factual predicate).
    let mut md = String::new();
    md.push_str("# VJS Conformance Map (PC-13 D11)\n\n");
    md.push_str(
        "Produced THROUGH the kernel by `vjs audit`. Every kernel_effect duty \
         (must / must_not / prohibits) in every in-force statute and regulation, with \
         whether it is bound to a deterministic kernel gate. The UNWIRED list is the \
         factual predicate for the reserved D12 single-front-door instrument.\n\n",
    );
    md.push_str(&format!(
        "- total duties: {}\n- wired: {}\n- unwired: {}\n\n",
        report.total, report.wired, report.unwired
    ));
    md.push_str(
        "> Triage note: UNWIRED does not mean \"must be gated\". Many unwired duties are \
         declarative (`defines`-adjacent), one-time/transition acts, or agent-duties that \
         a deterministic gate cannot or need not enforce. The conservative registry marks a \
         duty WIRED only when a named, deterministic gate can be pointed at it, so the map \
         never overstates coverage. D12 triages this list to decide which unwired duties the \
         single-front-door instrument must bite on.\n\n",
    );

    md.push_str("## Unwired duties (the side doors)\n\n");
    md.push_str("| instrument | kind | duty |\n|---|---|---|\n");
    for d in report.duties.iter().filter(|d| d.gate.is_none()) {
        md.push_str(&format!(
            "| {} | {} | {} |\n",
            d.instrument, d.kind, d.token
        ));
    }
    md.push_str("\n## Wired duties\n\n");
    md.push_str("| instrument | kind | duty | gate |\n|---|---|---|---|\n");
    for d in report.duties.iter().filter(|d| d.gate.is_some()) {
        md.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            d.instrument,
            d.kind,
            d.token,
            d.gate.as_deref().unwrap_or("")
        ));
    }

    let out_path = out.unwrap_or_else(|| PathBuf::from("docs/conformance-map.md"));
    // [2026] VJS-CC-VJS 16 D2, applied to the CLASS and not to one caller: an
    // operator-supplied --out must not manufacture the directory the resolver reads the
    // canon from. Measured 2026-08-01: `vjs audit --out <repo>/lawpack/v2/orders/probe.md`
    // created <repo>/lawpack/v2 and exited 0.
    vjs_engine::refuse_write_into_canon_tree(repo, &out_path)?;
    if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| KernelError::Io(e.to_string()))?;
    }
    std::fs::write(&out_path, &md).map_err(|e| KernelError::Io(e.to_string()))?;

    println!(
        "Conformance audit: {} duties, {} wired, {} unwired -> {}",
        report.total,
        report.wired,
        report.unwired,
        out_path.display()
    );
    Ok(())
}

pub(crate) fn cmd_install_lock(repo: &Path, json: bool) -> Result<(), KernelError> {
    // The surface must be complete before it can be atomically locked.
    let surface = vjs_core::install::verify_surface(repo);
    if !surface.is_empty() {
        let msgs: Vec<String> = surface.iter().map(|d| d.message()).collect();
        return Err(KernelError::InvalidInput(format!(
            "cannot lock an incomplete install: {}",
            msgs.join("; ")
        )));
    }
    // Materialise the thin agent-runtime adapters (D8) before locking, so the
    // manifest binds them (create-if-absent; never clobbers a customised adapter).
    let _ = vjs_core::install::generate_adapters(repo);
    let now = chrono::Utc::now().to_rfc3339();
    let manifest = vjs_core::install::build_manifest(repo, now).ok_or_else(|| {
        KernelError::InvalidInput("surface complete but manifest could not be built".into())
    })?;
    let body = toml::to_string(&manifest).map_err(|e| KernelError::Serialization(e.to_string()))?;
    let header = "# VJS install manifest (REG-INSTALL-MANIFEST-001). Atomic sha256 lock of the\n\
                  # REG-INVOCATION-001 surface. Re-lock with `vjs install-lock` after a surface change.\n";
    let path = repo.join(vjs_core::install::MANIFEST_FILE);
    std::fs::write(&path, format!("{header}{body}")).map_err(|e| KernelError::Io(e.to_string()))?;
    if json {
        println!(
            "{}",
            serde_json::json!({ "manifest": vjs_core::install::MANIFEST_FILE, "locked": true })
        );
    } else {
        println!(
            "Install manifest locked: {}",
            vjs_core::install::MANIFEST_FILE
        );
    }
    Ok(())
}

pub(crate) fn cmd_migrate_v1(
    repo: &Path,
    _v1_path: &Path,
    out: Option<PathBuf>,
    json: bool,
) -> Result<(), KernelError> {
    let output = out.unwrap_or_else(|| PathBuf::from("migration/draft-ledger.yaml"));
    // Same class, same guard ([2026] VJS-CC-VJS 16 D2).
    vjs_engine::refuse_write_into_canon_tree(repo, &output)?;
    std::fs::create_dir_all(output.parent().unwrap_or(&PathBuf::from(".")))
        .map_err(|e| KernelError::Io(e.to_string()))?;

    let ledger = MigrationLedger {
        version: "v2-draft".into(),
        entries: vec![
            MigrationEntry {
                id: "L-001".into(),
                title: "Memory is not authority".into(),
                status: MigrationStatus::Migrated,
                v1_sources: vec![
                    V1Source {
                        file: "Constitution/AGENTS.md".into(),
                        reference: "retrieval-first".into(),
                    },
                    V1Source {
                        file: "AGENTS.md".into(),
                        reference: "cdd-cli-spine".into(),
                    },
                ],
                v2_destination: V2Destination {
                    statute: "ACT-AGENT-DUTIES".into(),
                    rule: "AGENT-LOOKUP-001".into(),
                    invariant: "INV-PERMIT-REQUIRED".into(),
                },
                runtime_effect: vec![
                    "route lookup required before governed work".into(),
                    "missing permit blocks governed write".into(),
                ],
            },
            MigrationEntry {
                id: "L-002".into(),
                title: "Five court triggers".into(),
                status: MigrationStatus::Migrated,
                v1_sources: vec![V1Source {
                    file: "README.md".into(),
                    reference: "five-triggers".into(),
                }],
                v2_destination: V2Destination {
                    statute: "ACT-COURTS-ORDERS".into(),
                    rule: "RULE-COURT-TRIGGER".into(),
                    invariant: "INV-COURT-REQUIRED".into(),
                },
                runtime_effect: vec![
                    "court only convenes for defined triggers".into(),
                    "everything else is citation fast-path".into(),
                ],
            },
            MigrationEntry {
                id: "L-003".into(),
                title: "Public/private boundary".into(),
                status: MigrationStatus::Migrated,
                v1_sources: vec![
                    V1Source {
                        file: "README.md".into(),
                        reference: "public-private".into(),
                    },
                    V1Source {
                        file: ".gitignore".into(),
                        reference: "private-exclusions".into(),
                    },
                ],
                v2_destination: V2Destination {
                    statute: "ACT-PUBLIC-PRIVATE".into(),
                    rule: "DATA-PRIVATE-001".into(),
                    invariant: "INV-PUBLIC-NO-PRIVATE-FACTS".into(),
                },
                runtime_effect: vec![
                    "block unredacted private evidence from public record".into(),
                    "route local facts to .vjs/private".into(),
                ],
            },
        ],
    };

    let content =
        serde_yaml::to_string(&ledger).map_err(|e| KernelError::Serialization(e.to_string()))?;
    std::fs::write(&output, content).map_err(|e| KernelError::Io(e.to_string()))?;

    if json {
        println!("{}", serde_json::to_string_pretty(&ledger).unwrap());
    } else {
        println!("Migration ledger written to {}", output.display());
        println!("Entries: {}", ledger.entries.len());
    }

    Ok(())
}
