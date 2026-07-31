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

// LAWPACK RESOLUTION LIVES HERE, AND NOWHERE ELSE.
//
// Until [2026] VJS-CC-VJS 12 there were TWO implementations of `load_lawpack` and
// `compute_digest`: one in this crate and one in `vjs-cli/src/context.rs`, byte-for-byte
// the same law expressed twice. The ruling was implemented against the CLI copy and the
// repository's own `vjs validate` gate went on reading THIS one, so the fix passed every
// test and the gate still enforced the old rule - it reported LAWPACK_LOCK_DRIFT against a
// digest computed the superseded way. Two copies of a rule are one copy and one silent
// disagreement. The CLI now delegates here.

pub fn load_lawpack(repo: &Path) -> Result<Lawpack, KernelError> {
    match resolve_lawpack_dir(repo) {
        Some(dir) => LawpackLoader::load(&dir),
        None if is_invoked_jurisdiction(repo) => Err(KernelError::InvalidInput(format!(
            "no lawpack could be resolved for the jurisdiction at {}. Looked for: \
             <repo>/lawpack/v2, the `lawpack_path` in .vjs/config.toml, and $VJS_LAWPACK. \
             This repository is invoked as a jurisdiction, so an unresolvable lawpack is a \
             failure and not a stage: re-run `vjs invoke --lawpack <path>` against a real \
             lawpack. ([2026] VJS-CC-VJS 12 D1)",
            repo.display()
        ))),
        // NOT a jurisdiction, so there is no canon to be wrong about. This is the limb the
        // order preserved: `overlay_filed_orders` refuses to fail on a missing orders
        // directory because a fresh subscriber acquires orders BY OPERATING, and this court
        // affirmed that. A lawpack is never acquired by operating, which is why the same
        // absence is a failure once `.vjs/config.toml` exists.
        None => Ok(empty_lawpack()),
    }
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

/// Whether this repository has been invoked as a VJS jurisdiction.
///
/// The gate is `.vjs/config.toml` and not `.vjs/`, deliberately. `vjs invoke` creates
/// `.vjs/invocation/` BEFORE it writes the config, so keying on the directory would make
/// invocation refuse itself half-way through its own first run.
pub fn is_invoked_jurisdiction(repo: &Path) -> bool {
    repo.join(".vjs/config.toml").exists()
}

/// Where this repository's lawpack actually is, or `None`.
///
/// Three sources, most specific first. Until [2026] VJS-CC-VJS 12 there was ONE source -
/// `<repo>/lawpack/v2` - and a repository that did not VENDOR a copy of the canon silently
/// resolved against nothing. Measured 2026-07-31: `vjs lookup --issue enforcement` returned
/// four constitutional sections in `vibe-justice-system` and no output at all in
/// `vibe-design-system`, same binary, same flags, neither answer marked.
pub fn resolve_lawpack_dir(repo: &Path) -> Option<PathBuf> {
    let vendored = repo.join("lawpack/v2");
    if vendored.is_dir() {
        return Some(vendored);
    }
    if let Some(p) = lawpack_path_from_config(repo) {
        // Relative paths resolve against the repo, so a config is portable between clones.
        let p = if p.is_absolute() { p } else { repo.join(p) };
        if p.is_dir() {
            return Some(p);
        }
    }
    if let Some(p) = std::env::var_os("VJS_LAWPACK") {
        let p = PathBuf::from(p);
        if p.is_dir() {
            return Some(p);
        }
    }
    None
}

/// What `vjs invoke --lawpack` names: the label to record, and the directory it resolved to.
///
/// D3 of [2026] VJS-CC-VJS 12: "a flag that labels without selecting must be made to select
/// or be removed". The value may be
///   - a path to a lawpack directory (absolute, or relative to the repo), or
///   - a registered id like `vjs-v2@0.1.0`, resolved against the vendored copy, and
///   - omitted, which means the vendored copy and nothing else.
///
/// A path that does not resolve is an ERROR, never a label. That is the whole ruling: the
/// old code accepted any string and wrote it into two artefacts as though a subscription had
/// happened.
pub fn resolve_invocation_lawpack(
    repo: &Path,
    lawpack: Option<String>,
) -> Result<(String, Option<PathBuf>), KernelError> {
    let Some(named) = lawpack else {
        // Nothing named: the vendored copy, or nothing. `invoke` is allowed to run in a
        // repository that has neither, because the refusal in `load_lawpack` is keyed on a
        // config that invocation has not written yet - see D1/D2, which do not collide.
        return Ok(("vjs-v2@0.1.0".to_string(), resolve_lawpack_dir(repo)));
    };

    // A registered id, not a path. Resolve it the only way this kernel currently can.
    if !named.contains('/') && !named.contains(std::path::MAIN_SEPARATOR) {
        return Ok((named, resolve_lawpack_dir(repo)));
    }

    let raw = PathBuf::from(&named);
    let candidate = if raw.is_absolute() { raw.clone() } else { repo.join(&raw) };
    if !candidate.is_dir() {
        return Err(KernelError::InvalidInput(format!(
            "--lawpack {} does not resolve to a directory (looked at {}). A named lawpack that \
             cannot be resolved is refused rather than recorded: writing it into config.toml \
             and lawpack.lock would assert a subscription that did not happen. \
             ([2026] VJS-CC-VJS 12 D3)",
            named,
            candidate.display()
        )));
    }
    // Record the resolved location, not the string the caller happened to type, so the
    // config is not a second opinion about where the law is.
    let dir = candidate.canonicalize().unwrap_or(candidate);
    let id = lawpack_id_of(&dir).unwrap_or_else(|| "vjs-v2@0.1.0".to_string());
    Ok((id, Some(dir)))
}

/// The lawpack's own declared id, read from its manifest.
fn lawpack_id_of(dir: &Path) -> Option<String> {
    let text = std::fs::read_to_string(dir.join("manifest.toml")).ok()?;
    let mut id = None;
    let mut version = None;
    for line in text.lines() {
        let line = line.trim();
        if let Some(v) = line.strip_prefix("id") {
            if let Some(v) = v.trim_start().strip_prefix('=') {
                id = Some(v.trim().trim_matches('"').to_string());
            }
        } else if let Some(v) = line.strip_prefix("version") {
            if let Some(v) = v.trim_start().strip_prefix('=') {
                version = Some(v.trim().trim_matches('"').to_string());
            }
        }
    }
    match (id, version) {
        (Some(i), Some(v)) => Some(format!("{i}@{v}")),
        (Some(i), None) => Some(i),
        _ => None,
    }
}

/// The `lawpack_path` recorded by `vjs invoke`, if the config carries one.
///
/// Read with a line scan rather than a TOML parse on purpose: this runs before the kernel
/// context exists, on a file that may be half-written by a concurrent invoke, and a parse
/// error here would turn a recoverable absence into a hard failure of every command.
fn lawpack_path_from_config(repo: &Path) -> Option<PathBuf> {
    let text = std::fs::read_to_string(repo.join(".vjs/config.toml")).ok()?;
    for line in text.lines() {
        let line = line.trim();
        let Some(rest) = line.strip_prefix("lawpack_path") else {
            continue;
        };
        let Some(rest) = rest.trim_start().strip_prefix('=') else {
            continue;
        };
        let v = rest.trim().trim_matches('"');
        if !v.is_empty() {
            return Some(PathBuf::from(v));
        }
    }
    None
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
        let file_type = entry.file_type().map_err(|e| KernelError::Io(e.to_string()))?;
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
