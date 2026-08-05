//! The gate checks its own staleness.
//!
//! WHY. On 2026-08-05, twice in one day, a fix was proven against the debug build while the
//! RELEASE binary the hooks execute predated the fix - so the gate that ran was not the gate
//! that was written, and a sabotage "proof" passed vacuously against the stale artefact. The
//! first cure was a shell check in the pre-commit hook; INV-HOOKS-SHORT-001 refused it (hooks
//! stay short state checks; reasoning lives in the kernel), and the refusal was right: HERE,
//! every door and every hook inherits the check from the one validate implementation.
//!
//! BOOTSTRAP CAVEAT, stated rather than hidden: a binary from before this module cannot warn
//! about itself. The gap is one generation wide and closed the first time this version ships.
//!
//! The workspace is derived from the RUNNING binary's own path (`<ws>/target/.../vjs` implies
//! `<ws>/crates`), so the same check is layout-correct in canon (crates at the repo root) and
//! in a vendored tree (crates under `governance/`), and a test binary under
//! `target/debug/deps` resolves the same workspace. Crate-local `.vjs/` dirs are excluded:
//! test runs append audit logs there and churn in a log must not read as "the kernel changed".

use std::path::Path;
use std::time::SystemTime;

use vjs_core::report::Finding;
use vjs_core::types::Severity;

pub(crate) fn binary_freshness_findings(findings: &mut Vec<Finding>) {
    let Ok(exe) = std::env::current_exe() else {
        return;
    };
    let Ok(exe_meta) = std::fs::metadata(&exe) else {
        return;
    };
    let Ok(exe_mtime) = exe_meta.modified() else {
        return;
    };
    // <ws>/target/<profile>/vjs or <ws>/target/<profile>/deps/<test> -> find the ancestor
    // whose child is `target`, then its sibling `crates`.
    let Some(ws) = exe
        .ancestors()
        .find(|a| a.file_name().is_some_and(|n| n == "target"))
        .and_then(Path::parent)
    else {
        return;
    };
    let crates = ws.join("crates");
    if !crates.is_dir() {
        return;
    }
    // THE SIBLING BIN CRATE IS NOT THIS GATE. The workspace ships two door binaries
    // (vjs from vjs-cli, vjs-mcp from vjs-mcp); neither contains the other's source,
    // and cargo will not rebuild one for an edit to the other - so scanning the
    // sibling made an mcp-only edit wedge every vjs-invoking test red with no cure
    // inside `cargo test` (measured 2026-08-05, 19201s of false staleness). Each
    // binary checks the sources it actually carries; the sibling checks its own.
    let exe_name = exe
        .file_stem()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let skip_sibling = if exe_name.starts_with("vjs-mcp") {
        Some("vjs-cli")
    } else {
        Some("vjs-mcp")
    };
    if let Some((newer, when)) = newest_source(&crates, exe_mtime, skip_sibling) {
        let secs = when
            .duration_since(exe_mtime)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        findings.push(Finding {
            severity: Severity::Fatal,
            code: "BINARY-STALE".into(),
            path: Some(newer.clone()),
            message: format!(
                "this vjs binary is OLDER than the kernel source it gates: {} is newer by \
                 {secs}s. A gate proven in source and stale in the artefact produced two \
                 vacuous proofs on 2026-08-05; the gate that runs must be the gate that was \
                 written.",
                newer.display()
            ),
            citation: None,
            suggested_fix: Some(format!(
                "(cd {} && cargo build --release), then rerun",
                ws.display()
            )),
        });
    }
}

/// The first source file strictly newer than `than`, or None. First hit suffices: the cure
/// (rebuild) is the same however many there are, and a short-circuit keeps validate fast.
fn newest_source(
    dir: &Path,
    than: SystemTime,
    skip: Option<&str>,
) -> Option<(std::path::PathBuf, SystemTime)> {
    let rd = std::fs::read_dir(dir).ok()?;
    for entry in rd.flatten() {
        let p = entry.path();
        let name = entry.file_name();
        // `tests/` directories are excluded: integration-test sources never ship in
        // the binary, so a tests-only edit cannot make the binary's GATES stale -
        // and because `cargo test` does not rebuild the bin for a tests-only edit,
        // including them WEDGES the suite red with no cure but a manual rebuild.
        // Measured 2026-08-05: a dead-code allow in tests/lawpack_common failed
        // preCI on BINARY-STALE twenty seconds after a fresh release build.
        if name == ".vjs"
            || name == "target"
            || (name == "tests" && p.is_dir())
            || skip.is_some_and(|sk| name == sk)
        {
            continue;
        }
        if p.is_dir() {
            if let Some(hit) = newest_source(&p, than, None) {
                return Some(hit);
            }
        } else if let Ok(m) = entry.metadata()
            && let Ok(t) = m.modified()
            && t > than
        {
            return Some((p, t));
        }
    }
    None
}
