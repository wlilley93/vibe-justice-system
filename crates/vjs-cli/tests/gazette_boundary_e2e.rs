//! [2026] VJS-CC-VJS 17 C4 and C5, proved the only way they can be: by GENERATING the
//! Gazette and reading the artefact. C4 is "vacuous if asserted by grepping the source", and
//! C5's blindness is invisible to any unit test of the render function, because the defect is
//! what the function never reads.
//!
//! `vjs-cli` is a binary crate, so these drive the built binary through CARGO_BIN_EXE_vjs.

use std::path::Path;
use std::process::Command;

use sha2::Digest;

/// A synthetic token. Never a real term: the register is hashes precisely so that a proof
/// does not have to carry a private term in cleartext in the source tree.
const SYNTHETIC: &str = "quibbleflange";

fn deny_hash() -> String {
    format!("{:x}", sha2::Sha256::digest(SYNTHETIC.as_bytes()))
}

/// A minimal jurisdiction on disk at `dir`: one canon order, and BOTH registers (C3 makes an
/// absent register a refusal, so a fixture without them tests the refusal).
fn fixture(dir: &Path, order_extra: &str) {
    std::fs::create_dir_all(dir.join("lawpack/v2/orders")).unwrap();
    std::fs::create_dir_all(dir.join("lawpack/v2/federation")).unwrap();
    std::fs::create_dir_all(dir.join(".vjs")).unwrap();
    std::fs::write(
        dir.join("lawpack/v2/manifest.toml"),
        "id = \"vjs-v2\"\nversion = \"0.1.0\"\nrepo_code = \"VJS\"\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("lawpack/v2/federation/subscriber-registry.yaml"),
        "id: FEDERATION-SUBSCRIBER-REGISTRY\ncodes:\n  - ACMECO\n",
    )
    .unwrap();
    std::fs::write(
        dir.join(".vjs/publication-denylist.txt"),
        format!(
            "# fixture register\n{}  # added=2026-08-01 class=synthetic\n",
            deny_hash()
        ),
    )
    .unwrap();
    std::fs::write(
        dir.join("lawpack/v2/orders/2026-VJS-PC-999.yaml"),
        format!(
            "id: 2026-VJS-PC-999\ntitle: A fixture order\ncitation: \"[2026] VJS-PC 999\"\n\
             court: privy_council\nissue: governance.fixture\n\
             created_at: \"2026-08-01T00:00:00Z\"\nruntime_summary: a fixture\n{order_extra}"
        ),
    )
    .unwrap();
}

fn gazette(dir: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_vjs"))
        .arg("--repo")
        .arg(dir)
        .arg("gazette")
        .env_remove("VJS_LAWPACK")
        .output()
        .expect("the vjs binary runs")
}

#[test]
fn the_gazette_publishes_no_absolute_checkout_path() {
    // C4's own vacuity guard: the checkout's ABSOLUTE path carries a synthetic denylisted
    // token, so the publication gate itself is the assertion. THE RED SEED: before C4 this
    // fixture REFUSES - render.rs published `resolution.dir.display()` and the gate caught its
    // own artefact. Revert C4 and this test goes red again.
    let base = std::env::temp_dir()
        .join(SYNTHETIC)
        .join(format!("vjs-c4-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    fixture(&base, "");

    let out = gazette(&base);
    assert!(
        out.status.success(),
        "the Gazette must publish from a checkout whose path carries a private term: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    let raw = std::fs::read_to_string(base.join("gazette-data.json")).unwrap();
    let d: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let published = &d["meta"]["lawpack"]["path"];
    assert!(
        published.is_null() || published.as_str().is_some_and(|p| !p.starts_with('/')),
        "meta.lawpack.path is repo-relative or omitted, never '/'-rooted: {published}"
    );
    assert!(
        !raw.contains(&base.display().to_string()),
        "no field of the artefact carries the checkout's absolute path"
    );
    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn the_gate_scans_the_source_opinion_body_it_links() {
    // C5's own vacuity guard: the fixture opinion IS linked from a published order - a fixture
    // whose opinion is not linked proves nothing, because the gate is blind precisely to what
    // it links.
    //
    // THE RED SEED: before C5 this publishes CLEANLY. The Gazette prints the opinion's path
    // and a public blob URL to its body and never opens the file. Revert C5 and the first
    // assertion goes red.
    let base = std::env::temp_dir().join(format!("vjs-c5-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    fixture(
        &base,
        "source_opinion: \".vjs/submissions/filed/2026-VJS-PC-999-opinion.md\"\n",
    );
    let op = base.join(".vjs/submissions/filed/2026-VJS-PC-999-opinion.md");
    std::fs::create_dir_all(op.parent().unwrap()).unwrap();

    // (i) the linked body carries the synthetic term: the Gazette must REFUSE.
    std::fs::write(&op, format!("# opinion\n\nthe {SYNTHETIC} matter\n")).unwrap();
    let out = gazette(&base);
    assert!(
        !out.status.success(),
        "an opinion body reachable from a published item must be scanned; it published"
    );
    let msg =
        String::from_utf8_lossy(&out.stderr).to_string() + &String::from_utf8_lossy(&out.stdout);
    assert!(
        msg.contains("denylisted private term"),
        "the refusal must be the denylist limb's, over the linked body: {msg}"
    );
    assert!(
        !msg.to_lowercase().contains(SYNTHETIC),
        "the refusal must not name the term"
    );

    // (ii) remove it and the SAME estate publishes. Without this the refusal above could be
    // any refusal at all.
    std::fs::write(&op, "# opinion\n\nthe generic matter\n").unwrap();
    let out = gazette(&base);
    assert!(
        out.status.success(),
        "with the term removed the estate publishes: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    // (iii) the third limb of C5: an unreadable linked body is a refusal, never a skip.
    std::fs::remove_file(&op).unwrap();
    let out = gazette(&base);
    assert!(
        !out.status.success(),
        "a source_opinion the gate cannot read must refuse, not be skipped"
    );
    let _ = std::fs::remove_dir_all(&base);
}
