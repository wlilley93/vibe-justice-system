//! The warrant register's reading rule, enforced.
//!
//! WHY. A superseded commission warrant keeps its `status: in_force` line by design - no
//! line of a signed record ever changes - so the moment a successor warrant is signed, the
//! register holds two in-force records for one jurisdiction and, until 2026-08-05, nothing
//! machine-readable said which governs: "the latest in-force signature governs" was prose
//! only, read by people and by no code path (measured in the residue adoption round -
//! zero references to `provenance/warrants` anywhere in the kernel). ACT-004:s9 requires
//! supersession to be EXPLICIT, the old authority visible but no longer binding. Explicit,
//! here, is a `supersedes_digest` field on the successor pinning the predecessor's
//! registered digest: visible stays true (nothing is edited), binding becomes computable.
//!
//! Four findings, all Fatal, all fail-closed:
//! - WARRANT-RECORD-UNREADABLE: a register entry that cannot be read as a commission
//!   warrant. An unseen record is not an absent record.
//! - WARRANT-SUPERSESSION-IMPLICIT: an in-force warrant declares supersession in prose
//!   with no pinned digest - the reading rule would be unenforceable exactly when needed.
//! - WARRANT-SUPERSESSION-DANGLING: a pinned digest that matches no registered warrant.
//! - WARRANT-REGISTER-AMBIGUOUS: two in-force warrants govern one jurisdiction with no
//!   supersession chain between them.
//!
//! Plus the ACT-RECTIFICATION-COMMISSION s2 cap: more than three GOVERNING warrants is
//! WARRANT-CONCURRENCY-EXCEEDED (a superseded record no longer counts; that is what
//! "replaces its predecessor in the count" means, made computable).

use std::path::Path;

use vjs_core::report::Finding;
use vjs_core::types::Severity;

struct WarrantRecord {
    path: std::path::PathBuf,
    jurisdiction_id: String,
    in_force: bool,
    /// The registered digest the signature pinned (normalised, no `sha256:` prefix).
    digest: Option<String>,
    supersedes_prose: bool,
    supersedes_digest: Option<String>,
}

fn norm_digest(d: &str) -> String {
    d.trim()
        .trim_start_matches("sha256:")
        .to_lowercase()
        .to_string()
}

fn fatal(code: &str, path: &Path, message: String, citation: &str, fix: &str) -> Finding {
    Finding {
        severity: Severity::Fatal,
        code: code.into(),
        path: Some(path.to_path_buf()),
        message,
        citation: Some(citation.into()),
        suggested_fix: Some(fix.into()),
    }
}

pub(crate) fn warrant_register_findings(lawpack_dir: &Path, findings: &mut Vec<Finding>) {
    let dir = lawpack_dir.join("provenance/warrants");
    if !dir.is_dir() {
        // A lawpack with no warrant register holds no warrants to disagree; the register
        // being absent is not the same class as a register entry being unreadable.
        return;
    }
    let entries = match std::fs::read_dir(&dir) {
        Ok(rd) => rd,
        Err(e) => {
            findings.push(fatal(
                "WARRANT-RECORD-UNREADABLE",
                &dir,
                format!("the warrant register exists but cannot be listed: {e}. An unseen record is not an absent record, so nothing below this was checked."),
                "ACT-004:s9",
                "make the register directory readable, then re-run validate",
            ));
            return;
        }
    };
    let mut recs: Vec<WarrantRecord> = Vec::new();
    for entry in entries.flatten() {
        let p = entry.path();
        let is_yaml = p.extension().is_some_and(|e| e == "yaml" || e == "yml");
        if !is_yaml {
            continue;
        }
        let unreadable = |findings: &mut Vec<Finding>, why: String| {
            findings.push(fatal(
                "WARRANT-RECORD-UNREADABLE",
                &p,
                format!("a warrant register entry could not be read as a commission warrant: {why}. A record the reading rule cannot read is a record it cannot rank."),
                "ACT-004:s9",
                "repair the record so it parses as a commission_warrant with id, status and jurisdiction.jurisdiction_id",
            ));
        };
        let text = match std::fs::read_to_string(&p) {
            Ok(t) => t,
            Err(e) => {
                unreadable(findings, e.to_string());
                continue;
            }
        };
        let v: serde_yaml::Value = match serde_yaml::from_str(&text) {
            Ok(v) => v,
            Err(e) => {
                unreadable(findings, e.to_string());
                continue;
            }
        };
        if v.get("kind").and_then(|k| k.as_str()) != Some("commission_warrant") {
            unreadable(findings, "kind is not commission_warrant".into());
            continue;
        }
        let Some(status) = v.get("status").and_then(|s| s.as_str()) else {
            unreadable(findings, "no status field".into());
            continue;
        };
        let Some(jid) = v
            .get("jurisdiction")
            .and_then(|j| j.get("jurisdiction_id"))
            .and_then(|j| j.as_str())
        else {
            unreadable(findings, "no jurisdiction.jurisdiction_id".into());
            continue;
        };
        let in_force = status == "in_force";
        let digest = v
            .get("signature")
            .and_then(|s| s.get("instrument_digest_at_signature"))
            .and_then(|d| d.as_str())
            .or_else(|| v.get("assent_instrument_digest").and_then(|d| d.as_str()))
            .map(norm_digest);
        if in_force && digest.is_none() {
            unreadable(
                findings,
                "in force but carries no registered digest (neither \
                 signature.instrument_digest_at_signature nor assent_instrument_digest)"
                    .into(),
            );
            continue;
        }
        recs.push(WarrantRecord {
            path: p,
            jurisdiction_id: jid.to_string(),
            in_force,
            digest,
            supersedes_prose: v.get("supersedes").is_some(),
            supersedes_digest: v
                .get("supersedes_digest")
                .and_then(|d| d.as_str())
                .map(norm_digest),
        });
    }

    for r in &recs {
        if r.in_force && r.supersedes_prose && r.supersedes_digest.is_none() {
            findings.push(fatal(
                "WARRANT-SUPERSESSION-IMPLICIT",
                &r.path,
                "this in-force warrant declares supersession in prose with no supersedes_digest, so 'the latest in-force signature governs' is unenforceable exactly when it is needed (ACT-004:s9: supersession must be explicit).".into(),
                "ACT-004:s9",
                "add supersedes_digest pinning the predecessor's registered digest (its signature.instrument_digest_at_signature)",
            ));
        }
        if let Some(sd) = &r.supersedes_digest
            && !recs
                .iter()
                .any(|o| !std::ptr::eq(o, r) && o.digest.as_deref() == Some(sd.as_str()))
        {
            findings.push(fatal(
                "WARRANT-SUPERSESSION-DANGLING",
                &r.path,
                "supersedes_digest matches no other registered warrant's digest, so this record claims to supersede nothing the register holds.".into(),
                "ACT-004:s9",
                "pin the predecessor's actual registered digest, or remove the claim",
            ));
        }
    }

    // GOVERNING = in force and not superseded by another in-force record's pinned digest.
    let governing: Vec<&WarrantRecord> = recs
        .iter()
        .filter(|r| {
            r.in_force
                && !recs.iter().any(|s| {
                    !std::ptr::eq(*r, s)
                        && s.in_force
                        && s.supersedes_digest.as_deref() == r.digest.as_deref()
                        && r.digest.is_some()
                })
        })
        .collect();

    let mut by_jid: std::collections::BTreeMap<&str, Vec<&WarrantRecord>> = Default::default();
    for g in &governing {
        by_jid
            .entry(g.jurisdiction_id.as_str())
            .or_default()
            .push(g);
    }
    for (jid, group) in &by_jid {
        if group.len() > 1 {
            findings.push(fatal(
                "WARRANT-REGISTER-AMBIGUOUS",
                &group[0].path,
                format!(
                    "{} in-force warrants govern jurisdiction '{jid}' with no supersession chain between them; the register cannot say which binds.",
                    group.len()
                ),
                "ACT-004:s9",
                "add supersedes_digest to the successor pinning the predecessor's registered digest",
            ));
        }
    }
    if governing.len() > 3 {
        findings.push(fatal(
            "WARRANT-CONCURRENCY-EXCEEDED",
            &dir,
            format!(
                "{} warrants govern concurrently; ACT-RECTIFICATION-COMMISSION s2 caps concurrency at three (a superseded record no longer counts, which is what makes the cap computable).",
                governing.len()
            ),
            "ACT-RECTIFICATION-COMMISSION:s2",
            "supersede or let expire until at most three govern",
        ));
    }
}

/// A WARRANT THAT HAS BEEN SIGNED AND STILL READS `draft`
/// ([2026] VJS-CC-VJS 20 D7, as amended by D22).
///
/// A commission warrant is issued by the Principal's signature. The signature is
/// recorded in `provenance/assent/` and pins the ENGROSSED DRAFT by digest; the entry in
/// `provenance/warrants/` is a REGISTER PROJECTION that recites it. Two records, written
/// at two moments, and nothing compared them.
///
/// The failure that follows is quiet and total. The reading rule above counts only
/// GOVERNING warrants, and a warrant reading `draft` is not counted - so a warrant the
/// Principal has actually signed governs nothing, the concurrency cap does not see it,
/// and the supersession chain skips it. The register would be internally consistent and
/// simply wrong about who holds a commission.
///
/// D6 already makes it a duty to record a signature in the same act as the instrument it
/// issues. This is that duty made checkable: a duty nobody can fail to notice failing.
///
/// D22 GOVERNS HOW THE FINDING SPEAKS. The signature records were written before the
/// pseudonymity Acts and name their warrant by an older rendering; the register file is
/// named by the ACCESSIONED one. The join tolerates both - a signature is a signature
/// whatever it called its subject - but every message names the accessioned rendering,
/// taken from the register FILENAME and never from the id inside the signed record. A
/// gate that repeats an old rendering in order to report a defect has published it.
pub fn signed_but_draft_findings(lawpack_dir: &Path, findings: &mut Vec<Finding>) {
    let warrants_dir = lawpack_dir.join("provenance/warrants");
    let assent_dir = lawpack_dir.join("provenance/assent");
    if !warrants_dir.is_dir() || !assent_dir.is_dir() {
        return;
    }

    // Every warrant a signature record claims to have issued, by the id that record uses.
    let mut signed: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    if let Ok(rd) = std::fs::read_dir(&assent_dir) {
        for entry in rd.flatten() {
            let Ok(text) = std::fs::read_to_string(entry.path()) else {
                continue;
            };
            // A signature record names its subject in `instrument:`. Only a COMMISSION
            // WARRANT is in scope here; an Act's assent record is a different instrument
            // and a different gate's business.
            let Some(line) = text.lines().find(|l| l.starts_with("instrument:")) else {
                continue;
            };
            for tok in line.split(|c: char| !c.is_ascii_alphanumeric() && c != '-') {
                if tok.starts_with("WARRANT-") && tok.len() > "WARRANT-".len() {
                    let sig_id = text
                        .lines()
                        .find_map(|l| l.strip_prefix("id:"))
                        .map(|v| v.trim().to_string())
                        .unwrap_or_else(|| entry.file_name().to_string_lossy().to_string());
                    signed.insert(tok.to_ascii_uppercase(), sig_id);
                }
            }
        }
    }
    if signed.is_empty() {
        return;
    }

    let Ok(rd) = std::fs::read_dir(&warrants_dir) else {
        return;
    };
    let mut paths: Vec<_> = rd.flatten().map(|e| e.path()).collect();
    paths.sort();
    for path in paths {
        match path.extension().and_then(|e| e.to_str()) {
            Some("yaml") | Some("yml") => {}
            _ => continue,
        }
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let status = text
            .lines()
            .find_map(|l| l.strip_prefix("status:"))
            .map(|v| v.trim().to_string())
            .unwrap_or_default();
        if status != "draft" {
            continue;
        }
        // THE ACCESSIONED RENDERING, from the filename (D22). Never the `id:` inside the
        // record, which the older signed text may spell differently and which this gate
        // must not repeat.
        let accessioned = path
            .file_stem()
            .map(|s| s.to_string_lossy().to_ascii_uppercase())
            .unwrap_or_default();
        let inner_id = text
            .lines()
            .find_map(|l| l.strip_prefix("id:"))
            .map(|v| v.trim().to_ascii_uppercase())
            .unwrap_or_default();
        // Matched on EITHER rendering, because a signature is a signature whatever it
        // called its subject; reported on the accessioned one only.
        let Some(sig) = signed.get(&accessioned).or_else(|| signed.get(&inner_id)) else {
            continue;
        };
        findings.push(fatal(
            "WARRANT-SIGNED-BUT-DRAFT",
            &path,
            format!(
                "{accessioned} reads `status: draft`, but {sig} records the Principal's \
                 signature issuing it. A warrant reading draft is not counted as governing, \
                 so a warrant that has actually been signed would govern nothing, escape \
                 the concurrency cap, and be skipped by the supersession chain - a register \
                 internally consistent and simply wrong about who holds a commission."
            ),
            "[2026] VJS-CC-VJS 20 D7/D22; ACT-RECTIFICATION-COMMISSION s2",
            "Set the register entry in force in the SAME act that records the signature (D6), or withdraw the signature record if the warrant was not in fact issued.",
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!("vjs-warrants-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(d.join("provenance/warrants")).unwrap();
        d
    }

    fn write_warrant(dir: &Path, file: &str, body: &str) {
        std::fs::write(dir.join("provenance/warrants").join(file), body).unwrap();
    }

    fn run(dir: &Path) -> Vec<Finding> {
        let mut f = Vec::new();
        warrant_register_findings(dir, &mut f);
        f
    }

    fn codes(f: &[Finding]) -> Vec<&str> {
        f.iter().map(|x| x.code.as_str()).collect()
    }

    const PRED_A: &str = "id: WARRANT-A-001\nkind: commission_warrant\nstatus: in_force\njurisdiction:\n  jurisdiction_id: alpha\nsignature:\n  instrument_digest_at_signature: \"sha256:aaaa\"\n";
    const PRED_B: &str = "id: WARRANT-B-001\nkind: commission_warrant\nstatus: in_force\njurisdiction:\n  jurisdiction_id: beta\nsignature:\n  instrument_digest_at_signature: \"sha256:bbbb\"\n";

    #[test]
    fn the_live_register_shape_is_clean() {
        // Two in-force warrants, distinct jurisdictions, no supersession: the register
        // as it stands today. The check must accept it, or it refuses the present.
        let d = scratch("clean");
        write_warrant(&d, "a.yaml", PRED_A);
        write_warrant(&d, "b.yaml", PRED_B);
        assert!(run(&d).is_empty(), "{:?}", run(&d));
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn prose_only_supersession_is_refused() {
        // THE RED SEED for the reading rule: a successor that says so in prose and pins
        // nothing. Before this module, that record was the plan of record.
        let d = scratch("implicit");
        write_warrant(&d, "a.yaml", PRED_A);
        write_warrant(
            &d,
            "a2.yaml",
            "id: WARRANT-A-002\nkind: commission_warrant\nstatus: in_force\nsupersedes: >\n  WARRANT-A-001, by the reading rule.\njurisdiction:\n  jurisdiction_id: alpha\nsignature:\n  instrument_digest_at_signature: \"sha256:cccc\"\n",
        );
        let f = run(&d);
        assert!(
            codes(&f).contains(&"WARRANT-SUPERSESSION-IMPLICIT"),
            "{f:?}"
        );
        // And the same pair is ALSO ambiguous: nothing machine-readable ranks them.
        assert!(codes(&f).contains(&"WARRANT-REGISTER-AMBIGUOUS"), "{f:?}");
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn a_pinned_supersession_collapses_the_pair_and_both_stay_visible() {
        let d = scratch("pinned");
        write_warrant(&d, "a.yaml", PRED_A);
        write_warrant(
            &d,
            "a2.yaml",
            "id: WARRANT-A-002\nkind: commission_warrant\nstatus: in_force\nsupersedes: >\n  WARRANT-A-001.\nsupersedes_digest: \"sha256:AAAA\"\njurisdiction:\n  jurisdiction_id: alpha\nsignature:\n  instrument_digest_at_signature: \"sha256:cccc\"\n",
        );
        // Digest matching is normalised (prefix stripped, lowercased), the predecessor's
        // untouched in_force line stops counting, and no finding fires: old authority
        // visible, no longer binding, exactly ACT-004:s9.
        assert!(run(&d).is_empty(), "{:?}", run(&d));
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn a_dangling_pin_is_refused() {
        let d = scratch("dangling");
        write_warrant(&d, "a.yaml", PRED_A);
        write_warrant(
            &d,
            "a2.yaml",
            "id: WARRANT-A-002\nkind: commission_warrant\nstatus: in_force\nsupersedes: >\n  a predecessor this register does not hold.\nsupersedes_digest: \"sha256:dddd\"\njurisdiction:\n  jurisdiction_id: alpha\nsignature:\n  instrument_digest_at_signature: \"sha256:cccc\"\n",
        );
        let f = run(&d);
        assert!(
            codes(&f).contains(&"WARRANT-SUPERSESSION-DANGLING"),
            "{f:?}"
        );
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn a_fourth_governing_warrant_exceeds_the_cap() {
        let d = scratch("cap");
        for (i, jid) in ["alpha", "beta", "gamma", "delta"].iter().enumerate() {
            write_warrant(
                &d,
                &format!("w{i}.yaml"),
                &format!(
                    "id: WARRANT-{i}\nkind: commission_warrant\nstatus: in_force\njurisdiction:\n  jurisdiction_id: {jid}\nsignature:\n  instrument_digest_at_signature: \"sha256:d{i}\"\n"
                ),
            );
        }
        let f = run(&d);
        assert!(codes(&f).contains(&"WARRANT-CONCURRENCY-EXCEEDED"), "{f:?}");
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn an_unreadable_register_entry_is_fatal_never_skipped() {
        let d = scratch("unreadable");
        write_warrant(&d, "a.yaml", PRED_A);
        write_warrant(&d, "broken.yaml", "kind: [unclosed\n");
        let f = run(&d);
        assert!(codes(&f).contains(&"WARRANT-RECORD-UNREADABLE"), "{f:?}");
        let _ = std::fs::remove_dir_all(&d);
    }
}
