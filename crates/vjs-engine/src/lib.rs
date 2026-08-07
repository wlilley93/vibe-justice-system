//! The orchestration kernel: the validate pipeline (and, later, record-creation),
//! lifted out of the CLI so the CLI, the MCP server, and CI all call ONE
//! implementation. REG-KERNEL-001's "only smart point", in one place.
//!
//! Every gate returns `vjs_core::report::Finding`; `validate` composes them and
//! applies the PC-14 D3 assent floor. Behaviour is identical to the former 592-line
//! `cmd_validate` - this is a move, not a redesign; the test suite is the net.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use vjs_core::KernelError;
use vjs_core::report::{Finding, Report};
use vjs_core::types::Severity;
use vjs_git::GitIntegration;
use vjs_lawpack::{Lawpack, LawpackLoader, LawpackValidator};
use vjs_redact::RedactScanner;
use vjs_store::Store;

pub mod assent;
pub mod canon_licence;
pub mod context;
pub mod displacement;
mod freshness;
pub mod grounding;
pub mod order_checks;
mod ratchet;
pub(crate) mod record_removal;
mod resolver;
pub mod runtime;
mod staged;
pub mod store_register;
mod warrants;

pub use context::build_kernel_context;
pub use resolver::*;
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

// LAWPACK RESOLUTION LIVES IN `resolver`, AND NOWHERE ELSE ([2026] VJS-CC-VJS 12 / 15).
// Re-exported flat so every existing call site reads unchanged.

pub fn load_lawpack(repo: &Path) -> Result<Lawpack, KernelError> {
    match resolve_lawpack_dir(repo) {
        Some(dir) => LawpackLoader::load(&dir),
        None if is_invoked_jurisdiction(repo) => Err(unresolvable_lawpack_error(repo)),
        // NOT a jurisdiction, so there is no canon to be wrong about. This is the limb the
        // order preserved: `overlay_filed_orders` refuses to fail on a missing orders
        // directory because a fresh subscriber acquires orders BY OPERATING, and this court
        // affirmed that. A lawpack is never acquired by operating, which is why the same
        // absence is a failure once `.vjs/config.toml` exists.
        None => Ok(empty_lawpack()),
    }
}

/// The D1 refusal, in ONE place, so every door says the same thing. It lived inside
/// `load_lawpack` until [2026] VJS-CC-VJS 15, so the doors that resolved the lawpack
/// THEMSELVES (the Gazette, the MCP server) never reached it and published and recorded
/// against an empty canon instead. All three candidate sources are NAMED: a refusal that
/// does not say where it looked leaves the operator with the silent fallback's problem.
pub fn unresolvable_lawpack_error(repo: &Path) -> KernelError {
    KernelError::InvalidInput(format!(
        "no lawpack could be resolved for the jurisdiction at {}. Looked for: \
         <repo>/lawpack/v2, the `lawpack_path` in .vjs/config.toml, and $VJS_LAWPACK. \
         This repository is invoked as a jurisdiction, so an unresolvable lawpack is a \
         failure and not a stage: re-run `vjs invoke --lawpack <path>` against a real \
         lawpack. ([2026] VJS-CC-VJS 12 D1)",
        repo.display()
    ))
}

fn empty_lawpack() -> Lawpack {
    Lawpack {
        statutes: Vec::new(),
        regulations: Vec::new(),
        rules: Vec::new(),
        orders: Vec::new(),
        specs: Vec::new(),
        invariants: Vec::new(),
        decisions: Vec::new(),
        obligations: Vec::new(),
    }
}

pub fn compute_digest(repo: &Path) -> Result<String, KernelError> {
    use sha2::Digest;
    match resolve_lawpack_dir(repo) {
        Some(dir) => digest_of_lawpack_dir(&dir),
        // The empty digest. Reachable only where `load_lawpack` also declines to fail -
        // a repository that is not a jurisdiction - because every caller of this function
        // builds a kernel context first.
        None => Ok(format!(
            "sha256:{}",
            hex::encode(<sha2::Sha256 as Digest>::new().finalize())
        )),
    }
}

/// The digest of a lawpack, over the WHOLE TREE.
///
/// Until [2026] VJS-CC-VJS 12 this hashed `manifest.toml` alone - nineteen lines carrying an
/// id, a version, a status, two timestamps and a `[limits]` table. Every statute, regulation,
/// rule, order, spec, invariant, decision and obligation sat outside the pin, so the canon
/// could be rewritten entirely and every subscriber's lock would still verify. Measured
/// 2026-07-31: appending a line to `statutes/01-authority.yaml` left the digest at
/// `14cdb3337039ffdb…`, byte-identical to the one this repository had pinned since 07-27.
/// D4: "a pin that cannot move when the law moves is not a pin".
///
/// Deterministic across machines: paths are sorted, recorded RELATIVE to the lawpack root so
/// the digest does not depend on where the tree is checked out, and each entry contributes
/// its path AND its length before its bytes - so moving a byte from the end of one file to
/// the start of the next cannot leave the digest unchanged.
pub fn digest_of_lawpack_dir(dir: &Path) -> Result<String, KernelError> {
    use sha2::Digest;
    let mut files: Vec<PathBuf> = Vec::new();
    collect_files(dir, &mut files)?;
    files.sort();

    let mut hasher = sha2::Sha256::new();
    for path in &files {
        let rel = path.strip_prefix(dir).unwrap_or(path);
        // Normalise the separator so a Windows checkout hashes to the same value.
        let rel = rel.to_string_lossy().replace('\\', "/");
        let content = std::fs::read(path).map_err(|e| KernelError::Io(e.to_string()))?;
        hasher.update(rel.as_bytes());
        hasher.update([0u8]);
        hasher.update((content.len() as u64).to_be_bytes());
        hasher.update(&content);
    }
    Ok(format!("sha256:{}", hex::encode(hasher.finalize())))
}

fn collect_files(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), KernelError> {
    let entries = std::fs::read_dir(dir).map_err(|e| KernelError::Io(e.to_string()))?;
    for entry in entries {
        let entry = entry.map_err(|e| KernelError::Io(e.to_string()))?;
        let path = entry.path();
        let file_type = entry
            .file_type()
            .map_err(|e| KernelError::Io(e.to_string()))?;
        if file_type.is_dir() {
            collect_files(&path, out)?;
        } else if file_type.is_file() {
            out.push(path);
        }
        // Symlinks are skipped: a digest that follows one is a digest over bytes outside
        // the tree it claims to describe.
    }
    Ok(())
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

    // The gate checks its own staleness first: a stale binary evaluating fresh law is the
    // vacuous-proof class (see freshness.rs). INV-HOOKS-SHORT-001 put this HERE, not in a hook.
    freshness::binary_freshness_findings(&mut findings);

    ratchet::conformance_ratchet_findings(repo, &lawpack, &mut findings);

    store_register::store_register_findings(repo, &mut findings);

    canon_licence::canon_licence_findings(repo, &mut findings);

    // [2026] VJS-CC-VJS 20 D13: the order gates run AT REST, not only over a staged set.
    // Reported at Warning under an AT_REST_ prefix - a record already in force is the
    // correction register's business, never a reason to refuse every future commit.
    findings.extend(order_checks::at_rest_order_findings(repo, &lawpack));

    // A GATE'S GUARD MUST BE KEYED TO THE SAME REFERENT AS THE GATE ([2026] VJS-CC-VJS 14).
    //
    // The three checks below were wrapped in ONE condition - the existence of a VENDORED
    // lawpack directory - and they do not share a referent. Since CC-VJS 12 a jurisdiction
    // may resolve its lawpack OUT OF TREE through `lawpack_path`, and such a repository
    // vendors no copy at all, so all three were skipped in silence. Measured 2026-08-01: a
    // scratch jurisdiction invoked with an out-of-tree `--lawpack` and a DELIBERATELY
    // FALSIFIED digest in `.vjs/lawpack.lock` reported `Validation: OK`, exit 0. The pin was
    // not weak, it was absent. Each check now carries its own condition, drawn from its own
    // referent, and `resolve_lawpack_dir` is the single source of both the directory scanned
    // and of whether the lawpack-referent check runs at all.

    // Referent: the lawpack TREE. The condition is that a lawpack resolved, and the directory
    // handed to the check is the SAME `PathBuf` the `lawpack` above was loaded from - so the
    // tree scanned for citations and the set of defined ids describe ONE tree. Passing
    // the vendored path while `lawpack` came from elsewhere would have compared two
    // different bodies of law and called the difference dangling.
    if let Some(lawpack_dir) = resolve_lawpack_dir(repo) {
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
        // Same referent (the resolved lawpack TREE), so the same condition: the warrant
        // register lives inside it, and the reading rule that ranks two in-force warrants
        // for one jurisdiction must be computable, not prose (ACT-004:s9; the residue
        // adoption round, 2026-08-05).
        warrants::warrant_register_findings(&lawpack_dir, &mut findings);
    }

    // ACT-004:s8 (D2): citation uniqueness, collisions fatal. Referent: the LOCAL governed
    // records, `front_door::governed_record_roots` - which has nothing to do with where the
    // lawpack resolved, so there is no lawful condition on it and it runs on every validate.
    // The register stays the local roots and is NOT re-pointed at the resolved lawpack:
    // [2026] VJS-CC-VJS 9 D1 holds that the allocator and this guard must read the same
    // register, and the allocator reads the local roots (a guard wider than the allocator
    // fails on records the allocator cannot see). Where none of the three roots exist the
    // scan is empty by construction, which is the right answer for a repo that is not a
    // jurisdiction.
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

    // ACT-007:s7 (#2): the loaded law must hash to the pinned lock digest. Referent: the
    // pinned lock against the resolved digest, so the condition is that a lock EXISTS.
    //
    // NO SILENT ARM. The old form was `if let Ok(Some(lock)) = ... && let Ok(computed) = ...`,
    // which discarded both failures: an unparseable lock, or a `compute_digest` error, deleted
    // this Fatal exactly as thoroughly as the guard did, and validate said OK. A check that did
    // not run is not a check that passed, so each half now reports itself by name.
    match Store::read_lawpack_lock(repo) {
        Ok(Some(lock)) => match compute_digest(repo) {
            Ok(computed) if lock.digest != computed => {
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
            Ok(_) => {}
            Err(e) => {
                findings.push(
                    f(
                        Severity::Fatal,
                        "LAWPACK_DIGEST_UNCOMPUTABLE",
                        format!(
                            "The pinned lock was read but the lawpack digest could not be \
                             computed, so ACT-007:s7 was NOT checked: {e}. The failing half is \
                             the DIGEST, not the lock."
                        ),
                    )
                    .fix(
                        "Make the resolved lawpack readable (check lawpack_path in \
                         .vjs/config.toml and the permissions on that tree), then re-run validate.",
                    ),
                );
            }
        },
        // No lock: nothing is pinned, so there is nothing to drift from. `vjs invoke` writes
        // one; a repository that has never been invoked is not in breach of a pin it never made.
        Ok(None) => {}
        Err(e) => {
            findings.push(
                f(
                    Severity::Fatal,
                    "LAWPACK_LOCK_UNREADABLE",
                    format!(
                        ".vjs/lawpack.lock exists but could not be read, so ACT-007:s7 was NOT \
                         checked: {e}. The failing half is the LOCK, not the digest."
                    ),
                )
                .fix(
                    "Repair .vjs/lawpack.lock, or re-pin it with vjs invoke after confirming \
                     which lawpack this jurisdiction subscribes to.",
                ),
            );
        }
    }

    // [2026] VJS-CC-VJS 16 C5: DISPLACEMENT, which is not a severity of drift.
    //
    // Keyed on the config and the resolution and NOT on the federation subscriber registry,
    // for the reason at obiter (iv): that registry names a fixture code and not the one live
    // subscriber, so a gate keyed to it would be born not reaching the repository it exists
    // for. Both findings may fire together - they are true about different things - and the
    // displacement finding carries its own cure, because drift's cure ratifies displacement.
    findings.extend(displacement::check(repo));

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
