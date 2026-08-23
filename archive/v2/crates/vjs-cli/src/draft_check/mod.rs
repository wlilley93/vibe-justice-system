//! The Clerk Gate (Operation Watertight WS1): deterministic pre-flight over a draft
//! instrument BEFORE any counsel reads it, and the digest certificate as a command.
//!
//! WHY. Measured 2026-08-05: one Act cost five committee rounds because its defects
//! were found by expensive review instead of free checks - a misspelled kernel_effect
//! key silently dropping law, a wrong file:line cite, two totals for one measurement,
//! a denylisted term surviving to the enactment commit, and a ~38k-token bench sitting
//! convened to verify a sha256. Every check here is a refusal that used to be a
//! review round. Benches sit for judgment; the clerk does the arithmetic.
//!
//! The checks reuse the kernel's own readers wherever one exists - the struct-key
//! check round-trips through the REAL serde structs (so "what the kernel would drop"
//! is measured, not modelled), and citation grounding uses the SAME corpus as the
//! staged commit gate (vjs_engine::grounding) so the clerk and the gate cannot drift.

use super::*;
use std::collections::HashSet;
use vjs_core::types::Severity;

mod checks;
use checks::*;

#[derive(Subcommand)]
pub(crate) enum DraftCommands {
    /// Run the deterministic pre-flight checks over a draft statute.
    Check { file: PathBuf },
}

struct DFind {
    severity: Severity,
    code: &'static str,
    message: String,
}

fn d(severity: Severity, code: &'static str, message: String) -> DFind {
    DFind {
        severity,
        code,
        message,
    }
}

pub(crate) fn cmd_draft(repo: &Path, subcmd: DraftCommands, json: bool) -> Result<(), KernelError> {
    let DraftCommands::Check { file } = subcmd;
    let text = std::fs::read_to_string(&file)
        .map_err(|e| KernelError::Io(format!("{}: {e}", file.display())))?;
    let mut findings: Vec<DFind> = Vec::new();

    let draft_val: serde_yaml::Value = match serde_yaml::from_str(&text) {
        Ok(v) => v,
        Err(e) => {
            return report(
                &file,
                &text,
                vec![d(
                    Severity::Fatal,
                    "DRAFT-PARSE",
                    format!("not parseable as YAML: {e}"),
                )],
                None,
                json,
            );
        }
    };
    if draft_val.is_sequence() {
        // The root-sequence class: a `- id: ...` list parses as a SEQUENCE and every
        // top-level field silently becomes item structure - proven on a real canon file.
        return report(
            &file,
            &text,
            vec![d(
                Severity::Fatal,
                "DRAFT-ROOT-SEQUENCE",
                "the document root is a YAML SEQUENCE, not a mapping - a leading '- ' turns \
             the whole instrument into a list and the kernel parse fails or misreads it."
                    .into(),
            )],
            None,
            json,
        );
    }
    let statute: Statute = match serde_yaml::from_str(&text) {
        Ok(s) => s,
        Err(e) => {
            return report(
                &file,
                &text,
                vec![d(
                    Severity::Fatal,
                    "DRAFT-PARSE",
                    format!("not parseable as a Statute: {e}"),
                )],
                None,
                json,
            );
        }
    };

    struct_key_findings(&draft_val, &statute, &mut findings);
    let lawpack = vjs_engine::load_lawpack(repo)?;
    let (wired, unwired) = duty_findings(&statute, &lawpack, &mut findings);
    citation_findings(repo, &statute, &lawpack, &mut findings);
    address_findings(repo, &text, &mut findings);
    commencement_findings(&statute, &mut findings);
    count_findings(&text, &mut findings);
    denylist_findings(repo, &text, &mut findings);
    report(
        &file,
        &text,
        findings,
        Some((&statute, wired, unwired)),
        json,
    )
}

fn digest_of(text: &str) -> String {
    use sha2::Digest;
    format!(
        "sha256:{}",
        hex::encode(sha2::Sha256::digest(text.as_bytes()))
    )
}

fn report(
    file: &Path,
    text: &str,
    findings: Vec<DFind>,
    parsed: Option<(&Statute, usize, usize)>,
    json: bool,
) -> Result<(), KernelError> {
    let blocking = findings
        .iter()
        .filter(|f| matches!(f.severity, Severity::Error | Severity::Fatal))
        .count();
    if json {
        let js: Vec<_> = findings
            .iter()
            .map(|f| {
                serde_json::json!({"severity": format!("{:?}", f.severity), "code": f.code, "message": f.message})
            })
            .collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "ok": blocking == 0,
                "file": file.display().to_string(),
                "digest": digest_of(text),
                "sections": parsed.map(|(s, _, _)| s.sections.len()),
                "duties": parsed.map(|(s, _, _)| draft_duty_tokens(s).len()),
                "duty_preview": parsed.map(|(_, w, u)| serde_json::json!({"wired": w, "unwired": u})),
                "findings": js,
            }))
            .unwrap()
        );
    } else {
        for f in &findings {
            println!("{:?} {}: {}", f.severity, f.code, f.message);
        }
        // The digest block, ready for a certificate: the counts are MEASURED here so a
        // draft never carries two totals for one measurement (the 84/85 class).
        println!("---");
        println!("file: {}", file.display());
        println!("digest: {}", digest_of(text));
        if let Some((s, w, u)) = parsed {
            println!("sections: {}", s.sections.len());
            let toks = draft_duty_tokens(s);
            println!(
                "duty tokens: {} (preview: {w} wired, {u} unwired)",
                toks.len()
            );
        }
        println!(
            "draft check: {} finding(s), {blocking} blocking",
            findings.len()
        );
    }
    if blocking > 0 {
        return Err(KernelError::InvalidInput(format!(
            "draft check: {blocking} blocking finding(s) - fix before any counsel reads it"
        )));
    }
    Ok(())
}

/// The digest check as a command: operative-text identity from `sections:` onward,
/// header delta enumerated, digests printed. Replaces the agent sitting that verified
/// a sha256 and a diff at ~38k tokens.
pub(crate) fn cmd_certify(draft: &Path, adopted: &Path, json: bool) -> Result<(), KernelError> {
    let read = |p: &Path| {
        std::fs::read_to_string(p).map_err(|e| KernelError::Io(format!("{}: {e}", p.display())))
    };
    let dt = read(draft)?;
    let at = read(adopted)?;
    let split = |t: &str| -> (String, String) {
        match t.find("\nsections:") {
            Some(i) => (t[..i + 1].to_string(), t[i + 1..].to_string()),
            None => (t.to_string(), String::new()),
        }
    };
    let (dh, dop) = split(&dt);
    let (ah, aop) = split(&at);
    let operative_identical = dop == aop && !dop.is_empty();
    let header_delta: Vec<String> = {
        let dl: HashSet<&str> = dh.lines().collect();
        let al: HashSet<&str> = ah.lines().collect();
        let mut delta = Vec::new();
        for l in dh.lines().filter(|l| !al.contains(l)) {
            delta.push(format!("- {l}"));
        }
        for l in ah.lines().filter(|l| !dl.contains(l)) {
            delta.push(format!("+ {l}"));
        }
        delta
    };
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "operative_identical": operative_identical,
                "draft": {"file": draft.display().to_string(), "digest": digest_of(&dt), "operative_digest": digest_of(&dop)},
                "adopted": {"file": adopted.display().to_string(), "digest": digest_of(&at), "operative_digest": digest_of(&aop)},
                "header_delta": header_delta,
            }))
            .unwrap()
        );
    } else {
        println!("draft:    {} {}", digest_of(&dt), draft.display());
        println!("adopted:  {} {}", digest_of(&at), adopted.display());
        println!(
            "operative (from `sections:`): {}",
            if operative_identical {
                "IDENTICAL"
            } else {
                "DIFFERENT"
            }
        );
        println!("  draft operative digest:   {}", digest_of(&dop));
        println!("  adopted operative digest: {}", digest_of(&aop));
        if header_delta.is_empty() {
            println!("header: identical");
        } else {
            println!("header delta ({} line(s)):", header_delta.len());
            for l in &header_delta {
                println!("  {l}");
            }
        }
    }
    if !operative_identical {
        // Show WHERE, so the refusal is actionable without a diff tool.
        let first = dop
            .lines()
            .zip(aop.lines())
            .position(|(a, b)| a != b)
            .map(|i| i + 1);
        return Err(KernelError::InvalidInput(format!(
            "certify: operative text DIFFERS from the adopted text{} - an engrossment must \
             carry the adopted words byte-identically from `sections:` onward",
            first
                .map(|n| format!(" (first divergence at operative line {n})"))
                .unwrap_or_default()
        )));
    }
    Ok(())
}
