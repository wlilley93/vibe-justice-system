//! [2026] VJS-CC-VJS 21 D3. The publication gate, and specifically the one property
//! that makes the Warning downgrade elsewhere lawful: **a breach filing holds the
//! licence finding open at `validate` and buys NOTHING at this door.**
//!
//! Everywhere else in this kernel a recorded exception is the lawful route. Here it
//! is not one, and that asymmetry is the whole design, so it is tested by paired
//! cases rather than asserted in a comment.

use std::path::{Path, PathBuf};
use std::process::Command;

const AGPL_HEAD: &str = "                    GNU AFFERO GENERAL PUBLIC LICENSE\n\
                         Version 3, 19 November 2007\n";
const POLYFORM_HEAD: &str = "PolyForm Noncommercial License 1.0.0\n\nAcceptance\n";

/// A tree the publication gate recognises as the canon. Deliberately carries NO
/// `scripts/boundary-scan.sh`, so the boundary refusal is present in every case; the
/// assertions below are all about which OTHER refusals appear, which keeps each test
/// about one thing.
fn scratch(tag: &str, cargo_licence: &str, license_text: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("vjs-publish-{}-{tag}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("crates/vjs-core/src")).unwrap();
    std::fs::create_dir_all(dir.join("lawpack/v2")).unwrap();
    std::fs::write(dir.join("crates/vjs-core/src/lib.rs"), "// kernel\n").unwrap();
    std::fs::write(
        dir.join("lawpack/v2/manifest.toml"),
        "id = \"vjs-v2\"\ncanonical = true\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("Cargo.toml"),
        format!("[workspace.package]\nlicense = \"{cargo_licence}\"\n"),
    )
    .unwrap();
    std::fs::write(dir.join("LICENSE"), license_text).unwrap();
    dir
}

fn publish_json(dir: &Path) -> serde_json::Value {
    let out = Command::new(env!("CARGO_BIN_EXE_vjs"))
        .args(["publish", "--dry-run", "--json", "--repo"])
        .arg(dir)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .env_remove("GIT_INDEX_FILE")
        .output()
        .expect("vjs publish runs");
    // The CLI prints the report object and then, on a refusal, its own `{"error": ...}`
    // object. Take the FIRST, which is the report.
    let text = String::from_utf8_lossy(&out.stdout).to_string();
    let end = text.find("\n}\n").map(|i| i + 2).unwrap_or(text.len());
    serde_json::from_str(&text[..end]).unwrap_or_else(|e| {
        panic!(
            "publish --json emits a JSON report ({e}); stdout={text} stderr={}",
            String::from_utf8_lossy(&out.stderr)
        )
    })
}

fn codes(v: &serde_json::Value) -> Vec<String> {
    v["refusals"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["code"].as_str().unwrap().to_string())
        .collect()
}

fn file_the_breach(dir: &Path, operative: &str) {
    std::fs::create_dir_all(dir.join(".vjs/logs/breaches")).unwrap();
    std::fs::write(
        dir.join(".vjs/logs/breaches/BREACH-test.yaml"),
        format!("code: CANON-LICENCE-NOT-CONFORMING\noperative: \"{operative}\"\n"),
    )
    .unwrap();
}

#[test]
fn a_non_conforming_licence_refuses_publication() {
    let dir = scratch("refuse", "AGPL-3.0-only", POLYFORM_HEAD);
    let v = publish_json(&dir);
    assert_eq!(v["publishable"], false);
    assert!(
        codes(&v).contains(&"PUBLISH-REFUSED-LICENCE".to_string()),
        "{:?}",
        codes(&v)
    );
}

#[test]
fn a_filed_breach_does_not_open_this_door() {
    // THE TEST THIS FILE EXISTS FOR. CC-VJS 21 held the Warning lawful only if a
    // separate unconditional refusal stands across publication - "a filing that buys
    // the ability to complete the breach is not a record of it". So the pair below
    // must differ at `validate` and must NOT differ here.
    //
    // Note what the filing DOES legitimately move, because writing this test found it:
    // PUBLISH-REFUSED-VALIDATE disappears, since validate's copy of the licence
    // findings drops to Warning. That is right. Validate's job at this door is to
    // catch OTHER fatals; the licence is refused by its own unconditional check and
    // must never depend on borrowing a severity from somewhere else. So the assertion
    // is on the licence refusal itself and on publishability, which is the ratio, and
    // not on the whole refusal set, which would have been asserting a coincidence.
    let dir = scratch("filed", "AGPL-3.0-only", POLYFORM_HEAD);
    let before = publish_json(&dir);
    let before_licence = codes(&before)
        .iter()
        .filter(|c| *c == "PUBLISH-REFUSED-LICENCE")
        .count();
    assert!(before_licence > 0, "{:?}", codes(&before));

    file_the_breach(&dir, "PolyForm Noncommercial License 1.0.0");

    let after = publish_json(&dir);
    let after_licence = codes(&after)
        .iter()
        .filter(|c| *c == "PUBLISH-REFUSED-LICENCE")
        .count();
    assert_eq!(
        before_licence,
        after_licence,
        "the filing changed the licence refusal at the publication door: {:?} -> {:?}",
        codes(&before),
        codes(&after)
    );
    assert_eq!(
        after["publishable"], false,
        "filing a breach must never make the canon publishable"
    );
}

#[test]
fn an_absent_boundary_scanner_fails_closed() {
    // An absent scanner is not a finding of no findings. The one door where a missing
    // check must never read as a pass is the door that cannot be walked back through.
    let dir = scratch("noscan", "AGPL-3.0-only", AGPL_HEAD);
    let v = publish_json(&dir);
    assert!(
        codes(&v).contains(&"PUBLISH-REFUSED-BOUNDARY".to_string()),
        "{:?}",
        codes(&v)
    );
}

#[test]
fn the_gate_names_every_blocker_rather_than_stopping_at_the_first() {
    // One trip per blocker is how a release gate teaches people to run it repeatedly
    // and read only the last line. Both independent blockers are present here and
    // both must be reported.
    let dir = scratch("both", "AGPL-3.0-only", POLYFORM_HEAD);
    let c = codes(&publish_json(&dir));
    assert!(c.contains(&"PUBLISH-REFUSED-LICENCE".to_string()), "{c:?}");
    assert!(c.contains(&"PUBLISH-REFUSED-BOUNDARY".to_string()), "{c:?}");
}

#[test]
fn the_refusal_never_prints_the_denylisted_term() {
    // The gate's output lands in CI logs and release records. `file:line` is the whole
    // disclosure: enough to fix it, not enough to leak it.
    let dir = scratch("noleak", "AGPL-3.0-only", POLYFORM_HEAD);
    let raw = serde_json::to_string(&publish_json(&dir)).unwrap();
    assert!(
        !raw.to_lowercase().contains("denylisted private term: "),
        "the refusal must carry no term, only an address"
    );
}
