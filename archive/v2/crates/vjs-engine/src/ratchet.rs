//! The conformance ratchet, split from lib.rs 2026-08-05 under the structural
//! ceiling - the gate that ordered the split is the same continuous-closure
//! discipline this module enforces on the law. Behavior-preserving: the block
//! moved verbatim; only the wrapping changed.

use std::path::Path;

use vjs_core::report::Finding;
use vjs_core::types::Severity;
use vjs_lawpack::Lawpack;

// THE CONFORMANCE RATCHET: the unwired-duty count is MEASURED ON EVERY VALIDATE and may
// only fall.
//
// WHY. `vjs audit` (PC-13 D11's duty-conformance audit) had exactly one caller and nothing
// invoked it - no hook, no CI, no other command. Measured 2026-08-05: 281 enacted duties,
// 43 wired to a gate, 238 not, and that number appeared nowhere anyone looks. A duty whose
// gate does not exist is prose; a COUNT of such duties that nobody sees can only grow,
// because every new Act lands its tokens unwired and no gate goes red. This turns the
// count into a ratchet against a tracked baseline (`.vjs/conformance.lock`):
//
//   - unwired ABOVE the baseline is FATAL: new law landed with no gate and no recorded
//     decision to accept that. The cure is to wire the duty, or to raise the baseline in
//     the same commit as the Act that adds the duty - a visible, reviewable act, which is
//     the whole point.
//   - unwired BELOW the baseline is a WARNING naming the new number: lower the baseline so
//     the improvement is banked. A ratchet that does not tighten leaks.
//   - a PRESENT but unreadable baseline is FATAL (the gazette-denylist rule: a gate that
//     skips on a read failure looks exactly like a gate that passed).
//   - an ABSENT baseline is DISCLOSED as not-a-pass, never silently skipped: a fresh
//     subscriber has no baseline yet and hard-failing would brick genesis (the O5
//     bootstrap-trap reasoning), but "nothing to check" and "checked and clean" are
//     different facts and are reported differently.
pub(crate) fn conformance_ratchet_findings(
    repo: &Path,
    lawpack: &Lawpack,
    findings: &mut Vec<Finding>,
) {
    let conf = vjs_lawpack::conformance_audit(lawpack);
    let lock_path = repo.join(".vjs/conformance.lock");
    match std::fs::read_to_string(&lock_path) {
        Ok(text) => {
            let baseline: Option<usize> = text
                .lines()
                .find_map(|l| l.strip_prefix("unwired"))
                .and_then(|r| r.trim_start().strip_prefix('=').map(str::trim))
                .and_then(|v| v.parse().ok());
            match baseline {
                Some(base) if conf.unwired > base => findings.push(Finding {
                    severity: Severity::Fatal,
                    code: "CONFORMANCE-RATCHET".into(),
                    path: Some(lock_path.clone()),
                    message: format!(
                        "unwired duties ROSE: {} against a baseline of {base} (total {}, \
                         wired {}). New law has landed with no gate. Wire the duty into \
                         GATE_REGISTRY, or raise the baseline in the same commit as the \
                         instrument that adds it, so the acceptance is on the record.",
                        conf.unwired, conf.total, conf.wired
                    ),
                    citation: None,
                    suggested_fix: Some(
                        "wire the new duty tokens, or update .vjs/conformance.lock with the \
                         instrument that justifies the rise"
                            .into(),
                    ),
                }),
                Some(base) if conf.unwired < base => findings.push(Finding {
                    severity: Severity::Warning,
                    code: "CONFORMANCE-IMPROVED".into(),
                    path: Some(lock_path.clone()),
                    message: format!(
                        "unwired duties FELL: {} against a baseline of {base}. Lower the \
                         baseline to {} so the improvement is banked and cannot silently \
                         regress.",
                        conf.unwired, conf.unwired
                    ),
                    citation: None,
                    suggested_fix: Some("set unwired = the new lower number".into()),
                }),
                Some(_) => {}
                None => findings.push(Finding {
                    severity: Severity::Fatal,
                    code: "CONFORMANCE-LOCK-UNREADABLE".into(),
                    path: Some(lock_path.clone()),
                    message: "the conformance baseline exists but carries no parseable \
                              `unwired = N` line, so the ratchet cannot run. A gate that \
                              skips on a read failure looks exactly like a gate that passed."
                        .into(),
                    citation: None,
                    suggested_fix: Some("restore the `unwired = N` line".into()),
                }),
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => findings.push(Finding {
            severity: Severity::Info,
            code: "CONFORMANCE-UNTRACKED".into(),
            path: Some(lock_path.clone()),
            message: format!(
                "the conformance ratchet DID NOT RUN - no baseline at .vjs/conformance.lock. \
                 Current measurement: {} duties, {} wired, {} unwired. This is a disclosure, \
                 not a pass; write the baseline to arm the ratchet.",
                conf.total, conf.wired, conf.unwired
            ),
            citation: None,
            suggested_fix: Some(
                "write .vjs/conformance.lock with `unwired = <current>` to arm the ratchet".into(),
            ),
        }),
        Err(e) => findings.push(Finding {
            severity: Severity::Fatal,
            code: "CONFORMANCE-LOCK-UNREADABLE".into(),
            path: Some(lock_path.clone()),
            message: format!(
                "the conformance baseline exists but cannot be read ({e}); refusing to treat \
                 an unreadable ratchet as a passed one."
            ),
            citation: None,
            suggested_fix: Some("fix the file's permissions or restore it".into()),
        }),
    }
}
