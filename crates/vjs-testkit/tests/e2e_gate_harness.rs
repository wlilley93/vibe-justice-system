//! End-to-end gate harness (improvement #4): the assent floor + bench gate proven
//! through the FULL staged validate pipeline in an ephemeral git repo carrying the real
//! canon, not just at the unit level. This is the harness BREACH-2026-06-12 called for -
//! the security behaviour locked into the permanent suite, exercised exactly as a real
//! `git commit` exercises it (stage a fixture, run vjs_engine::validate{staged:true}).

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn git(repo: &Path, args: &[&str]) {
    let ok = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    assert!(ok, "git {args:?} failed");
}

fn copy_dir(src: &Path, dst: &Path) {
    std::fs::create_dir_all(dst).unwrap();
    for e in std::fs::read_dir(src).unwrap().flatten() {
        let p = e.path();
        let target = dst.join(p.file_name().unwrap());
        if p.is_dir() {
            copy_dir(&p, &target);
        } else {
            std::fs::copy(&p, &target).unwrap();
        }
    }
}

/// An ephemeral git repo carrying the real canon lawpack (incl. provenance), committed
/// as the baseline so HEAD-resolution works. `tag` keeps parallel tests isolated.
fn ephemeral_canon(tag: &str) -> PathBuf {
    let ws = workspace_root();
    let base = std::env::temp_dir().join(format!("vjs-e2e-{}-{tag}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    copy_dir(&ws.join("lawpack/v2"), &base.join("lawpack/v2"));
    git(&base, &["init", "-q"]);
    git(&base, &["config", "user.email", "t@example.invalid"]);
    git(&base, &["config", "user.name", "harness"]);
    git(&base, &["add", "-A"]);
    git(&base, &["commit", "-q", "-m", "baseline canon"]);
    base
}

fn validate_staged(repo: &Path) -> vjs_core::report::Report {
    vjs_engine::validate(
        repo,
        &vjs_engine::ValidateOpts {
            staged: true,
            external: false,
        },
    )
    .expect("validate runs")
}

#[test]
fn forged_fresh_apex_order_stays_fatal_through_the_pipeline() {
    // The audit's exploit: a brand-new apex order typing sovereign_assent, no bench. Its
    // assent resolves to nothing AND it has no constituted bench - it must NOT pass.
    let repo = ephemeral_canon("forged");
    // Its holding also cites a hallucinated authority ([2026] VJS-PC 99) - the PC-17
    // citation-grounding gate must catch the non-existent citation too.
    let forged = "id: \"2026-VJS-SC-999\"\n\
        assent_source: sovereign_assent\n\
        citation: \"[2026] VJS-SC 999\"\n\
        court: supreme_court\njurisdiction: vibe-justice-system\nstatus: binding\n\
        issue: forgery\nbench: []\nholding: \"forged, citing [2026] VJS-PC 99 which does not exist\"\n\
        directives: []\nsupersedes: []\nruntime_summary: forged\ncreated_at: \"2026\"\n";
    std::fs::write(repo.join("lawpack/v2/orders/FORGED.yaml"), forged).unwrap();
    git(&repo, &["add", "lawpack/v2/orders/FORGED.yaml"]);

    let report = validate_staged(&repo);
    assert!(!report.ok, "the forged order must not pass validate");
    let bench = report
        .findings
        .iter()
        .find(|f| f.code == "BENCH_REQUIRED")
        .expect("BENCH_REQUIRED is raised for the bench-less apex order");
    assert!(
        bench.is_blocking(),
        "BENCH_REQUIRED is constitutive - it must stay Fatal, never downgraded by the fake assent"
    );
    // PC-17: the hallucinated citation is caught, Fatal (the forgery's assent does not
    // resolve, so the correctable finding is not downgraded).
    let cite = report
        .findings
        .iter()
        .find(|f| f.code == "ORDER_CITATION_UNRESOLVED")
        .expect("ORDER_CITATION_UNRESOLVED is raised for the hallucinated [2026] VJS-PC 99");
    assert!(
        cite.is_blocking(),
        "an unresolved operative citation on a non-resolving order stays Fatal"
    );
    let _ = std::fs::remove_dir_all(&repo);
}

#[test]
fn bench_less_court_of_appeal_order_is_fatal() {
    // Regression (PC-19 D4 / VJS-SC 2): a Court of Appeal order recorded with NO bench must
    // be caught by the #10 "apex order must declare its bench" gate. CoA was missing from the
    // matches! set, so a bench-less CoA order evaded both that gate AND verify_bench (which
    // early-returns on an empty bench) - an unconstituted CoA order would have been accepted.
    let repo = ephemeral_canon("coa-benchless");
    let order = "id: \"2026-VJS-CA-001\"\n\
        court: court_of_appeal\njurisdiction: vibe-justice-system\nstatus: binding\n\
        issue: an appeal\nbench: []\nholding: \"a CoA order recorded without its constituted bench\"\n\
        directives: []\nsupersedes: []\nruntime_summary: x\ncreated_at: \"2026\"\n";
    std::fs::write(repo.join("lawpack/v2/orders/COA.yaml"), order).unwrap();
    git(&repo, &["add", "lawpack/v2/orders/COA.yaml"]);

    let report = validate_staged(&repo);
    let bench = report
        .findings
        .iter()
        .find(|f| f.code == "BENCH_REQUIRED")
        .expect("BENCH_REQUIRED must be raised for the bench-less Court of Appeal order");
    assert!(
        bench.is_blocking(),
        "BENCH_REQUIRED is constitutive - a bench-less CoA order must stay Fatal"
    );
    let _ = std::fs::remove_dir_all(&repo);
}

#[test]
fn forged_standing_bounded_order_is_still_blocked() {
    // The standing_bounded_assent path is permissive (PC-16 reserved per-class routes),
    // but the HIGH-VALUE order/apex vector is closed independently by the constitutive
    // codes: a forged bench-less order declaring standing_bounded_assent (not sovereign)
    // is still blocked - no assent claim makes a bench-less order a real order.
    let repo = ephemeral_canon("forged-standing");
    let forged = "id: \"2026-VJS-SC-998\"\n\
        assent_source: standing_bounded_assent\n\
        citation: \"[2026] VJS-SC 998\"\n\
        court: supreme_court\njurisdiction: vibe-justice-system\nstatus: binding\n\
        issue: forgery\nbench: []\nholding: forged\ndirectives: []\nsupersedes: []\n\
        runtime_summary: forged\ncreated_at: \"2026\"\n";
    std::fs::write(repo.join("lawpack/v2/orders/FORGED2.yaml"), forged).unwrap();
    git(&repo, &["add", "lawpack/v2/orders/FORGED2.yaml"]);

    let report = validate_staged(&repo);
    assert!(
        !report.ok,
        "the forged standing_bounded order must not pass"
    );
    assert!(
        report
            .findings
            .iter()
            .any(|f| f.code == "BENCH_REQUIRED" && f.is_blocking()),
        "BENCH_REQUIRED stays Fatal regardless of the standing_bounded_assent claim"
    );
    let _ = std::fs::remove_dir_all(&repo);
}

#[test]
fn constitutive_bench_defect_not_downgraded_even_for_an_established_assented_order() {
    // The other half: take a REAL, established, standing_bounded_assent order (which DOES
    // resolve) and break its bench. Bench-integrity is constitutive, so it stays Fatal
    // even though the record resolves - the old bug downgraded it on mere membership.
    let repo = ephemeral_canon("established");
    let path = repo.join("lawpack/v2/orders/2026-VJS-PC-015.yaml");
    let content = std::fs::read_to_string(&path).unwrap();
    // Empty the bench (a bench-integrity defect) while keeping the resolving assent_source.
    let broken = content
        .lines()
        .map(|l| {
            if l.starts_with("bench:") {
                "bench: []".to_string()
            } else if l.trim_start().starts_with("- Tindale")
                || l.trim_start().starts_with("- Rowanne")
                || l.trim_start().starts_with("- Marchmont")
            {
                String::new()
            } else {
                l.to_string()
            }
        })
        .filter(|l| !l.is_empty()) // drop the emptied seat lines, leaving bench: []
        .collect::<Vec<_>>()
        .join("\n");
    std::fs::write(&path, broken).unwrap();
    git(&repo, &["add", "lawpack/v2/orders/2026-VJS-PC-015.yaml"]);

    let report = validate_staged(&repo);
    let bench_fatal = report
        .findings
        .iter()
        .any(|f| f.code == "BENCH_REQUIRED" && f.is_blocking());
    assert!(
        bench_fatal,
        "an established, resolving order's bench-integrity defect must STILL be Fatal \
         (constitutive, never assent-downgraded). Findings: {:?}",
        report
            .findings
            .iter()
            .map(|f| (&f.code, &f.severity))
            .collect::<Vec<_>>()
    );
    let _ = std::fs::remove_dir_all(&repo);
}

#[test]
fn governed_order_outside_lawpack_hits_bench_integrity_raw_write() {
    // K-1 (sole mediated path): a governed order placed in the staged set by a RAW filesystem
    // write + `git add` - no `vjs route`, no `vjs record` verb, and OUTSIDE lawpack/v2/orders/
    // (the path Store::write_order targets) - must still hit the SAME verify_bench gate as a
    // canon order. County is below the apex-routing bright-line, so the order-integrity block
    // is the only gate that can catch it; before this change that block filtered to
    // lawpack/v2/orders/ and never saw this file. A county bench must be an odd 1, so a bench
    // of 2 is BENCH_SIZE_MISMATCH (constitutive, never assent-downgraded).
    let repo = ephemeral_canon("raw-vjs-bench");
    let forged = "id: \"2026-VJS-CC-ACMECO-777\"\n\
        citation: \"[2026] VJS-CC 777\"\n\
        court: county\njurisdiction: vibe-justice-system\nstatus: binding\n\
        issue: side_door\n\
        bench:\n  - Alpha CCJ\n  - Beta CCJ\n\
        holding: \"a county order with an even bench of two, written straight to .vjs\"\n\
        directives:\n  - id: D1\n    actor: lexby\n    must: do_x\n\
        supersedes: []\nruntime_summary: raw\ncreated_at: \"2026\"\n";
    std::fs::create_dir_all(repo.join(".vjs/orders")).unwrap();
    std::fs::write(repo.join(".vjs/orders/2026-VJS-CC-ACMECO-777.yaml"), forged).unwrap();
    git(&repo, &["add", ".vjs/orders/2026-VJS-CC-ACMECO-777.yaml"]);

    let report = validate_staged(&repo);
    assert!(
        report
            .findings
            .iter()
            .any(|f| f.code == "BENCH_SIZE_MISMATCH" && f.is_blocking()),
        "the staged bench-integrity gate must fire on a governed order OUTSIDE \
         lawpack/v2/orders/ (K-1 sole mediated path); findings: {:?}",
        report
            .findings
            .iter()
            .map(|f| (&f.code, &f.severity))
            .collect::<Vec<_>>()
    );
    let _ = std::fs::remove_dir_all(&repo);
}
