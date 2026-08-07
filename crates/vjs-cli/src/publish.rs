//! THE PUBLICATION GATE ([2026] VJS-CC-VJS 21 D3).
//!
//! Publication is the one act in this corpus that cannot be undone. Everything else
//! here is a file that can be edited back; a public repository is mirrored, cached
//! and cloned within minutes of going up, and a licence the world has read at face
//! value is very hard to narrow afterwards.
//!
//! CC-VJS 21 held the Warning downgrade on the canon-licence finding lawful on three
//! conditions, and this file is the third. The reasoning is worth carrying here
//! because it is the reason this gate refuses things `validate` merely warns about:
//!
//! > "The answer is to ask what the downgrade actually buys. If the Warning lets the
//! > work of curing the breach proceed, it is harmless. If the Warning lets the
//! > breach be COMPLETED - if it lets the canon be published under the forbidden
//! > licence - then recording a breach has purchased the ability to commit it, and no
//! > amount of care in the filing format redeems that."
//!
//! So the refusals below are UNCONDITIONAL. No breach filing, no decision log and no
//! permit downgrades any of them, and that is deliberate: everywhere else in the
//! kernel a recorded exception is the lawful route, and here it is not one, because
//! here there is no going back.
//!
//! This gate is only worth anything if it is the route the act actually runs
//! through - "prose is not enforcement" - so `vjs publish` performs the visibility
//! change itself once it is satisfied. A gate beside the road stops nobody.

use std::path::Path;
use std::process::Command;

use vjs_core::error::KernelError;
use vjs_core::types::Severity;

pub struct PublishRefusal {
    pub code: &'static str,
    pub detail: String,
}

/// Every reason the canon may not be published, gathered rather than short-circuited:
/// a caller told about one blocker at a time makes one trip per blocker, and this is
/// not a fast check.
pub fn publication_refusals(repo: &Path) -> Vec<PublishRefusal> {
    let mut out = Vec::new();

    // 1. THE LICENCE. Re-asked here from the raw gate, WITHOUT the breach-filing
    //    downgrade that `validate` applies. This is the whole point of D3: the filing
    //    holds the finding open so the work of curing it can proceed, and it buys
    //    nothing at all at this door.
    let mut licence = Vec::new();
    vjs_engine::canon_licence::canon_licence_findings(repo, &mut licence);
    for f in licence {
        if f.code.starts_with("CANON-LICENCE") {
            out.push(PublishRefusal {
                code: "PUBLISH-REFUSED-LICENCE",
                detail: format!(
                    "{}: {} [{} is Warning at validate ONLY because the conflict is on the \
                     record; CC-VJS 21 D3 makes it unconditional here]",
                    f.code, f.message, f.code
                ),
            });
        }
    }

    // 2. THE BOUNDARY. A denylisted term reaching a public tree is the harm the whole
    //    pseudonymity programme exists to prevent, and publication is the moment it
    //    becomes irreversible. The scan NEVER prints the term; file:line is the whole
    //    disclosure, which is enough to fix it and not enough to leak it.
    let scan = repo.join("scripts/boundary-scan.sh");
    if scan.is_file() {
        match Command::new("bash").arg(&scan).current_dir(repo).output() {
            Ok(o) if o.status.success() => {}
            Ok(o) => out.push(PublishRefusal {
                code: "PUBLISH-REFUSED-BOUNDARY",
                detail: String::from_utf8_lossy(&o.stdout)
                    .lines()
                    .last()
                    .unwrap_or("the publication-boundary scan refused")
                    .to_string(),
            }),
            Err(e) => out.push(PublishRefusal {
                code: "PUBLISH-REFUSED-BOUNDARY",
                detail: format!(
                    "the publication-boundary scan DID NOT RUN ({e}). A scan that did not \
                     run is not a clean scan, and this door fails closed."
                ),
            }),
        }
    } else {
        out.push(PublishRefusal {
            code: "PUBLISH-REFUSED-BOUNDARY",
            detail: format!(
                "no publication-boundary scan at {}. This door fails closed: an absent \
                 scanner is not a finding of no findings.",
                scan.display()
            ),
        });
    }

    // 3. THE VALIDATE FLOOR. Publishing a tree the kernel calls Fatal publishes the
    //    Fatal to every subscriber who clones it.
    let opts = vjs_engine::ValidateOpts {
        staged: false,
        external: false,
    };
    if let Ok(report) = vjs_engine::validate(repo, &opts) {
        let fatals: Vec<_> = report
            .findings
            .iter()
            .filter(|f| f.severity == Severity::Fatal)
            .map(|f| f.code.clone())
            .collect();
        if !fatals.is_empty() {
            out.push(PublishRefusal {
                code: "PUBLISH-REFUSED-VALIDATE",
                detail: format!("validate is Fatal on: {}", fatals.join(", ")),
            });
        }
    } else {
        out.push(PublishRefusal {
            code: "PUBLISH-REFUSED-VALIDATE",
            detail: "validate could not run, and a validate that did not run is not a \
                     green validate."
                .into(),
        });
    }

    out
}

pub fn cmd_publish(repo: &Path, dry_run: bool, json: bool) -> Result<(), KernelError> {
    let refusals = publication_refusals(repo);

    if json {
        let payload = serde_json::json!({
            "publishable": refusals.is_empty(),
            "refusals": refusals.iter().map(|r| serde_json::json!({
                "code": r.code, "detail": r.detail
            })).collect::<Vec<_>>(),
            "authority": "[2026] VJS-CC-VJS 21 D3; [2026] VJS-PC 11 D2",
        });
        println!("{}", serde_json::to_string_pretty(&payload).unwrap());
        return if refusals.is_empty() {
            Ok(())
        } else {
            Err(KernelError::InvalidInput(format!(
                "{} publication blocker(s)",
                refusals.len()
            )))
        };
    }

    if !refusals.is_empty() {
        eprintln!(
            "PUBLICATION REFUSED ({} blocker{}). [2026] VJS-CC-VJS 21 D3: this refusal is \
             UNCONDITIONAL - no breach filing, decision log or permit downgrades it, \
             because publication cannot be undone.\n",
            refusals.len(),
            if refusals.len() == 1 { "" } else { "s" }
        );
        for r in &refusals {
            eprintln!("  [{}] {}\n", r.code, r.detail);
        }
        return Err(KernelError::InvalidInput(format!(
            "{} publication blocker(s)",
            refusals.len()
        )));
    }

    println!("Publication checks PASS: licence conforming, boundary clean, validate green.");
    if dry_run {
        println!("--dry-run: the visibility change was NOT performed.");
        return Ok(());
    }

    // D9: once the holder has stated the licence and the boundary is clean, PC 11 D2's
    // publicity limb makes this required rather than merely permitted.
    let out = Command::new("gh")
        .args(["repo", "edit", "--visibility", "public"])
        .current_dir(repo)
        .output();
    match out {
        Ok(o) if o.status.success() => {
            println!("The canon is PUBLIC. [2026] VJS-PC 11 D2 publicity limb discharged.");
            Ok(())
        }
        Ok(o) => Err(KernelError::InvalidInput(format!(
            "the checks passed but the visibility change failed: {}",
            String::from_utf8_lossy(&o.stderr).trim()
        ))),
        Err(e) => Err(KernelError::InvalidInput(format!(
            "the checks passed but `gh` could not be run: {e}"
        ))),
    }
}
