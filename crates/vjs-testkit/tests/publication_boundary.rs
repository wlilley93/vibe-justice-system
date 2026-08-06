//! The published Gazette is public record. This is the CI net behind the
//! `vjs gazette` publication boundary gate (prevention for
//! BREACH-2026-06-10-client-data-published): the published artifacts must carry
//! no secrets/PII (RedactScanner, high-confidence kinds) and no denylisted
//! private identifier (the hashed denylist). A carried external-matter artifact
//! is private by default and must be cleared before publication.

use std::path::PathBuf;

use vjs_core::BoundaryFindingKind;
use vjs_redact::RedactScanner;

fn repo_root() -> PathBuf {
    // The PUBLISHING estate's root, FOUND by walking up rather than counting levels:
    // in a vendored tree the crates sit one level deeper than the law.
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    loop {
        if d.join("lawpack/v2/manifest.toml").is_file() {
            return d;
        }
        assert!(
            d.pop(),
            "no lawpack/v2 above CARGO_MANIFEST_DIR: these tests need one"
        );
    }
}

/// `false` means this estate has never published a Gazette and carries no publication
/// register (a subscriber is born unpublished and unregistered), disclosed on stderr -
/// a statement about this estate, never about the corpus. In the publishing estate
/// both exist and every assertion bites.
fn publishing_estate() -> bool {
    let root = repo_root();
    let ok = root.join("gazette-text.js").is_file()
        && root.join(".vjs/publication-denylist.txt").is_file();
    if !ok {
        eprintln!(
            "SKIP: {} carries no published Gazette artefacts or no publication register.",
            root.display()
        );
    }
    ok
}

fn published_text() -> String {
    let root = repo_root();
    let mut s = String::new();
    for f in ["gazette-data.json", "gazette-text.js"] {
        s.push_str(&std::fs::read_to_string(root.join(f)).unwrap_or_default());
        s.push('\n');
    }
    s
}

#[test]
fn the_published_gazette_carries_no_secrets_or_pii() {
    if !publishing_estate() {
        return;
    }
    let text = published_text();
    let findings: Vec<_> = RedactScanner::scan_file(&PathBuf::from("gazette"), &text)
        .into_iter()
        .filter(|f| !matches!(f.kind, BoundaryFindingKind::PrivateHostname))
        .collect();
    assert!(
        findings.is_empty(),
        "the published Gazette leaks private data: {:?}",
        findings.iter().map(|f| &f.message).collect::<Vec<_>>()
    );
}

#[test]
fn the_published_gazette_carries_no_denylisted_private_term() {
    if !publishing_estate() {
        return;
    }
    use sha2::Digest;
    let deny = std::fs::read_to_string(repo_root().join(".vjs/publication-denylist.txt"))
        .expect("the publication denylist exists");
    let hashes: std::collections::HashSet<String> = deny
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(|l| l.to_string())
        .collect();
    assert!(
        !hashes.is_empty(),
        "the denylist seeds at least one private term"
    );

    let text = published_text();
    let mut token = String::new();
    let mut hit: Option<String> = None;
    let check = |t: &str, hit: &mut Option<String>| {
        if t.len() >= 3 {
            let h = format!("{:x}", sha2::Sha256::digest(t.to_lowercase().as_bytes()));
            if hashes.contains(&h) {
                *hit = Some("<redacted>".into());
            }
        }
    };
    for ch in text.chars() {
        if ch.is_alphanumeric() || ch == '-' {
            token.push(ch);
        } else {
            check(&token, &mut hit);
            token.clear();
        }
    }
    check(&token, &mut hit);
    assert!(
        hit.is_none(),
        "the published Gazette contains a denylisted private term"
    );
}
