//! The orchestration kernel: the validate pipeline (and, later, record-creation),
//! lifted out of the CLI so the CLI, the MCP server, and CI all call ONE
//! implementation. REG-KERNEL-001's "only smart point", in one place.
//!
//! Every gate returns `vjs_core::report::Finding`; `validate` composes them and
//! applies the PC-14 D3 assent floor. Behaviour is identical to the former 592-line
//! `cmd_validate` - this is a move, not a redesign; the test suite is the net.

use std::collections::HashSet;
use std::path::Path;

use vjs_core::KernelError;
use vjs_core::report::{Finding, Report};
use vjs_core::types::Severity;
use vjs_git::GitIntegration;
use vjs_lawpack::{Lawpack, LawpackLoader, LawpackValidator};
use vjs_redact::RedactScanner;
use vjs_store::Store;

pub mod assent;
pub mod runtime;
mod staged;

use staged::staged_gates;

/// Options for a validate run.
#[derive(Clone, Debug, Default)]
pub struct ValidateOpts {
    /// Run the commit-gate (staged) checks: permit, bench/order, canon-write, apex,
    /// media, destructive-delete, install surface, and the assent floor.
    pub staged: bool,
    /// Warn if the repo has a public remote (release-warrant reminder).
    pub external: bool,
}

fn f(severity: Severity, code: &str, message: String) -> Finding {
    Finding {
        severity,
        code: code.into(),
        path: None,
        message,
        citation: None,
        suggested_fix: None,
    }
}

/// Load the canon lawpack (empty when there is none).
pub fn load_lawpack(repo: &Path) -> Result<Lawpack, KernelError> {
    let dir = repo.join("lawpack/v2");
    if dir.exists() {
        LawpackLoader::load(&dir)
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

/// The lawpack digest the lock pins: the sha256 of lawpack/v2/manifest.toml.
pub fn compute_digest(repo: &Path) -> Result<String, KernelError> {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    let manifest = repo.join("lawpack/v2/manifest.toml");
    if manifest.exists() {
        let content = std::fs::read(&manifest).map_err(|e| KernelError::Io(e.to_string()))?;
        hasher.update(&content);
    }
    Ok(format!("sha256:{}", hex::encode(hasher.finalize())))
}

/// Run the full validate pipeline and return the report. Pure of I/O effects beyond
/// reading the repo; the caller prints / exits.
pub fn validate(repo: &Path, opts: &ValidateOpts) -> Result<Report, KernelError> {
    let mut findings: Vec<Finding> = Vec::new();

    // --- Lawpack-wide checks (always) ---
    let lawpack = load_lawpack(repo)?;
    let report = LawpackValidator::validate(&lawpack)?;
    for ff in report.findings {
        findings.push(Finding {
            severity: ff.severity,
            code: ff.code,
            path: ff.path,
            message: ff.message,
            citation: None,
            suggested_fix: ff.suggested_fix,
        });
    }

    let lawpack_dir = repo.join("lawpack/v2");
    if lawpack_dir.exists() {
        for ff in LawpackValidator::check_referential_integrity(&lawpack_dir, &lawpack)? {
            findings.push(Finding {
                severity: ff.severity,
                code: ff.code,
                path: ff.path,
                message: ff.message,
                citation: None,
                suggested_fix: ff.suggested_fix,
            });
        }
        // ACT-004:s8 (D2): citation uniqueness, collisions fatal.
        for ff in LawpackValidator::check_citation_uniqueness(repo)? {
            findings.push(Finding {
                severity: ff.severity,
                code: ff.code,
                path: ff.path,
                message: ff.message,
                citation: None,
                suggested_fix: ff.suggested_fix,
            });
        }
        // ACT-007:s7 (#2): the loaded law must hash to the pinned lock digest.
        if let Ok(Some(lock)) = Store::read_lawpack_lock(repo)
            && let Ok(computed) = compute_digest(repo)
            && lock.digest != computed
        {
            findings.push(
                f(
                    Severity::Fatal,
                    "LAWPACK_LOCK_DRIFT",
                    format!(
                        "Loaded lawpack does not hash to the pinned lock digest (ACT-007:s7). \
                         lock={} computed={}.",
                        lock.digest, computed
                    ),
                )
                .fix(
                    "Re-pin the lock (vjs invoke regenerates .vjs/lawpack.lock) only after \
                     confirming the lawpack change is intended.",
                ),
            );
        }
    }

    // --- Commit-gate (staged) checks ---
    let mut assented_record_paths: HashSet<String> = HashSet::new();
    if opts.staged {
        let changed = GitIntegration::read_staged_files(repo)?;
        if changed.is_empty() {
            findings.push(f(
                Severity::Info,
                "NO_STAGED_FILES",
                "No staged files to validate".into(),
            ));
        } else {
            // ONE deterministic git read of the committed tree, not a per-record subprocess:
            // whether a governed record is established at HEAD (an edit vs a fresh insertion)
            // is now a pure fact handed to the resolver. A genuine git failure surfaces here
            // as a loud error, never a silent per-record floor-strip (ACT-010 / REG-KERNEL-001).
            let head_set = GitIntegration::tracked_at_head(repo)?;
            for rel in &changed {
                // PC-16 D1: the floor shelters a staged record only if its declared
                // assent_source RESOLVES to real Sovereign authority - not merely names
                // an allow-listed form. A forged record that types the words but resolves
                // to nothing is left at its native severity.
                if vjs_core::front_door::is_governed_record(rel)
                    && let Ok(content) = std::fs::read_to_string(repo.join(rel))
                    && crate::assent::assent_resolves(repo, rel, &content, head_set.contains(rel))
                {
                    assented_record_paths.insert(rel.clone());
                }
            }
            findings.push(f(
                Severity::Info,
                "STAGED_FILES",
                format!("{} staged files", changed.len()),
            ));

            staged_gates(repo, &lawpack, &changed, &mut findings)?;
        }
    }

    if opts.external && GitIntegration::is_public_remote(repo)? {
        findings.push(
            f(
                Severity::Warning,
                "PUBLIC_REMOTE",
                "Repository has a public remote. Release warrant may be required.".into(),
            )
            .fix("Run vjs release-warrant check"),
        );
    }

    // Boundary scan over .vjs (always).
    let vjs_dir = repo.join(".vjs");
    if vjs_dir.exists() {
        let boundary = RedactScanner::scan_directory(&vjs_dir)?;
        if !RedactScanner::check_public_safe(&boundary) {
            for bf in boundary {
                if matches!(bf.severity, Severity::Fatal | Severity::Error) {
                    findings.push(Finding {
                        severity: bf.severity,
                        code: "BOUNDARY_VIOLATION".into(),
                        path: bf.path,
                        message: bf.message,
                        citation: None,
                        suggested_fix: Some(format!("{:?}", bf.suggested_route)),
                    });
                }
            }
        }
    }

    // PC-16 D4: the entrenched-enforcement-surface pin. A drift of a pinned gate-source
    // file from its lock is a loud, blocking finding - a weakening edit is never silent.
    findings.extend(vjs_core::enforcement::check_drift(repo));

    // Install-completeness + atomic manifest, scoped to --staged (the commit gate).
    if opts.staged {
        let mut defects = vjs_core::install::verify_surface(repo);
        defects.extend(vjs_core::install::verify_manifest(repo));
        for d in defects {
            findings.push(
                f(Severity::Fatal, d.code(), d.message())
                    .fix("Run vjs invoke --install-hooks, then vjs install-lock"),
            );
        }
        for hook in vjs_core::install::hook_tamper(repo) {
            findings.push(
                f(
                    Severity::Warning,
                    "HOOK_TAMPERED",
                    format!(
                        "Hook '.vjs/hooks/{hook}' does not match its pinned digest \
                         (REG-INSTALL-MANIFEST-001) - possible tamper or post-template-change drift."
                    ),
                )
                .fix(
                    "If the change is intended, re-lock with vjs install-lock; otherwise restore \
                     the hook (vjs invoke --install-hooks).",
                ),
            );
        }
    }

    // PC-14 D3 assent floor: every block on a staged record whose assent RESOLVES
    // (PC-16 D1) degrades to route-for-correction (never void/block,
    // ACT-ASSENTED-RECORD-PROTECTION) - EXCEPT a constitutive-validity finding, which
    // goes to whether the record IS a valid record of its kind and is never softened by
    // any assent claim (PC-16: "void ab initio on both grounds").
    if !assented_record_paths.is_empty() {
        for fd in &mut findings {
            if fd.is_blocking()
                && !crate::assent::is_constitutive(&fd.code)
                && let Some(p) = &fd.path
                && assented_record_paths.contains(&p.to_string_lossy().to_string())
            {
                fd.severity = Severity::Warning;
                fd.message = format!(
                    "[{}: assented record routed for correction, never blocked - ACT-ASSENTED-RECORD-PROTECTION] {}",
                    vjs_core::front_door::ROUTE_FOR_CORRECTION_CODE,
                    fd.message
                );
            }
        }
    }

    Ok(Report::from_findings(findings))
}
