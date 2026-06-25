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
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
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
