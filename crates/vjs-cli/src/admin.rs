//! Administrative commands: next-citation, audit, install-lock, and migrate-v1.

use super::*;

/// The canon seat's repo code. Only canon may mint a canon series ([2026] VJS-PC 19).
const CANON_REPO_CODE: &str = "VJS";

pub(crate) fn cmd_next_citation(
    repo: &Path,
    series: String,
    year: Option<i32>,
    json: bool,
) -> Result<(), KernelError> {
    let y = year.unwrap_or_else(|| chrono::Utc::now().year());
    let repo_code = resolve_repo_code(repo);
    let s = series.to_ascii_uppercase();

    // ACT-004:s8 - "Citations are deterministic and unique. Collisions are fatal."
    // [2026] VJS-PC 13 D2 - allocate from the live persisted register, collisions fatal.
    //
    // CORRECTION 2026-07-17: the comment previously here claimed this allocated from
    // "the LIVE persisted register (the citator index)". That was FALSE WHEN WRITTEN.
    // The code read `lawpack/v2` and nothing else; it never opened the citator. The
    // kernel was certifying a D2 compliance it did not have. That false record, not any
    // later patch, is the gravest ceremonial/effect discrepancy on this path
    // (ACT-COMPUTER-FIRST-REALM:s5). It is corrected here rather than restated.
    //
    // The CC series is bound to THIS repo's code; canon series (PC/SC/REG/ACT/DEC/
    // SPEC/INV/COA/...) carry no repo segment and are NOT this seat's to mint unless
    // this seat IS canon - see the refusal below.
    let lawpack_dir = repo.join("lawpack/v2");
    let (repo_for_lookup, repo_segment): (Option<&str>, String) = if s == "CC" {
        (Some(repo_code.as_str()), format!("-{}", repo_code))
    } else {
        (None, String::new())
    };
    // REFUSE a canon series at a subscriber seat ([2026] VJS-PC 19: capability is not
    // authority; an under-inclusive gate is a gap to close, not a jurisdiction to
    // occupy). A subscriber's reach over a canon series is DEFINITIONALLY partial: it
    // holds only a mirror. Allocating from what it can see is how opbox came to offer
    // `[2026] VJS-SC 6`, which canon already holds as good law, in a string
    // indistinguishable from canon's own. Where reach is partial, refuse; do not guess.
    if s != "CC" && repo_code != CANON_REPO_CODE {
        return Err(KernelError::InvalidInput(format!(
            "refusing to mint the canon series {s} at the subscriber seat {repo_code}: this seat holds \
             only a mirror of that series, so any number it offers may collide with canon good-law \
             ([2026] VJS-PC 19: capability is not authority). Mint {s} at canon, or refer the matter up."
        )));
    }

    // The register is the UNION of every store within the allocator's reach that
    // evidences an allocation. This is compelled by ACT-004:s8, not chosen: a clerk
    // must not issue a number it can see is taken, and the law does not ask where the
    // clerk's knowledge came from.
    //
    // Reading only lawpack/v2 made the clerk fail OPEN at a SUBSCRIBING seat. A
    // subscriber records its local series in `.vjs/orders/` and its citator and does
    // NOT mirror them into lawpack/v2 (which carries the canon it subscribes to), so
    // the scan legitimately found nothing, max stayed 0, and the clerk returned n=1 -
    // offering a citation that collides with existing good-law, silently, every time.
    // That is the worst failure available to this component: ACT-004:s8 makes the
    // allocator's number authoritative, so a wrong number is trusted OVER a right guess.
    //
    // Verified figures only (contested counts deliberately not restated): at opbox the
    // order-store max is 84 and the citator row-max is 121, both good law. So lawpack
    // plus orders ALONE would still allocate 85, into occupied numbers. The citator
    // read is what makes the allocation safe, not a nicety.
    let mut max = 0u32;
    for dir in [lawpack_dir, repo.join(".vjs/orders")] {
        if dir.exists() {
            let m = LawpackValidator::live_citation_max(&dir, &s, repo_for_lookup, y)?;
            if m > max {
                max = m;
            }
        }
    }
    // The citator is prose: hand-maintained markdown, outside the lawpack, schema-checked
    // by nothing. It is read as a ONE-DIRECTIONAL FLOOR ONLY - able to raise the next
    // value, never to lower it, and never to establish that a value is free. The kernel
    // is not taking prose as LAW; it is taking prose as EVIDENCE OF OCCUPANCY, in the one
    // direction where misreading is harmless. The scan is deliberately UNANCHORED and
    // over-matching: over-matching can only raise max, and skipping a number is not
    // fatal while colliding is. A rigorous structured parse would be the unsafe choice -
    // the citator carries at least three incompatible row formats, and a strict parse
    // misses row 121 entirely and allocates 104, straight into occupied good-law.
    max = max.max(citator_citation_max(repo, &s, repo_for_lookup, y)?);
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
    _v1_path: &Path,
    out: Option<PathBuf>,
    json: bool,
) -> Result<(), KernelError> {
    let output = out.unwrap_or_else(|| PathBuf::from("migration/draft-ledger.yaml"));
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

/// The highest N already recorded in this repo's citator for (series, repo, year).
///
/// The citator is a markdown index of rows, not lawpack YAML, so its citations appear
/// inline (`[2026] VJS-CC-OPBOX 121`) rather than on a `citation:` key.
///
/// It is NOT "the only store that sees every ruling" - an earlier draft said so and that
/// was false. The divergence is BIDIRECTIONAL: citator numbers exist with no order file,
/// AND order files exist that the current-grammar citator does not list. Neither store
/// dominates, which is exactly why the union is the only safe read available today.
/// Verified figures only: order-store max 84, citator row-max 121, both good law.
///
/// BOTH grammars are counted: the current `[2026] VJS-CC-OPBOX 121` and the legacy
/// pre-Bill-16 `[2026] CC-OPBOX 23` (no `VJS-` segment).
///
/// An earlier draft skipped legacy rows, reasoning that "the two run as separate
/// sequences". THAT PREMISE IS FALSE. They are ONE sequence, re-spelled: ruling 23
/// exists exactly once (order file `2026-VJS-CC-OPBOX-023.yaml`), recorded in the
/// citator under the legacy spelling. Bill 16 s.7 replaced a FORM, it did not open a
/// second sequence.
///
/// The safety direction is the opposite of what that draft assumed. COUNTING legacy can
/// only RAISE max, which is safe (skipping a number is not fatal). SKIPPING legacy is
/// what can UNDER-allocate into occupied ground, which is fatal (ACT-004:s8). This is
/// canon code running at every subscriber, so wherever a seat's legacy max exceeds its
/// current max, the skip would mint a collision. Harmless at opbox today (legacy row-max
/// is below the current max, so the union is unchanged), but the trap is realm-wide.
fn citator_citation_max(
    repo: &Path,
    series: &str,
    repo_code: Option<&str>,
    year: i32,
) -> Result<u32, KernelError> {
    let index = repo.join(".justice/INDEX.md");
    if !index.exists() {
        return Ok(0);
    }
    let content = std::fs::read_to_string(&index).map_err(|e| KernelError::Io(e.to_string()))?;
    // `VJS-` optional: match both grammars. Deliberately unanchored (see the floor note
    // at the call site) - over-matching raises max, which is the safe direction.
    let re = regex::Regex::new(r"\[(\d{4})\]\s+(?:VJS-)?([A-Za-z]+)(?:-([A-Za-z0-9]+))?\s+(\d+)")
        .map_err(|e| KernelError::Io(e.to_string()))?;
    let want_series = series.to_ascii_uppercase();
    let want_repo = repo_code.map(|r| r.to_ascii_uppercase());
    let mut max = 0u32;
    for c in re.captures_iter(&content) {
        let y: i32 = match c.get(1).and_then(|m| m.as_str().parse().ok()) {
            Some(v) => v,
            None => continue,
        };
        let s = c.get(2).map(|m| m.as_str().to_ascii_uppercase());
        let r = c.get(3).map(|m| m.as_str().to_ascii_uppercase());
        let n: u32 = match c.get(4).and_then(|m| m.as_str().parse().ok()) {
            Some(v) => v,
            None => continue,
        };
        if y == year && s.as_deref() == Some(want_series.as_str()) && r == want_repo && n > max {
            max = n;
        }
    }
    Ok(max)
}
