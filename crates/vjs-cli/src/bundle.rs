//! The `vjs bundle verify` command: the fail-closed deployment-bundle check (REG-BUNDLE-001)
//! and its manifest types + licence allow/deny tables.

use super::*;

// ---- vjs bundle verify: the fail-closed deployment-bundle check (REG-BUNDLE-001) ----

#[derive(Deserialize)]
struct BundleManifest {
    schema_version: Option<u64>,
    bundle: Option<String>,
    distribution_licence: Option<String>,
    #[serde(default)]
    component: Vec<BundleComponent>,
}

#[derive(Deserialize)]
struct BundleComponent {
    id: Option<String>,
    repo: Option<String>,
    digest: Option<String>,
    source_commit: Option<String>,
    licence: Option<String>,
    adoption_mode: Option<String>,
}

const BUNDLE_COPYLEFT: &[&str] = &[
    "AGPL-3.0-only",
    "AGPL-3.0",
    "GPL-3.0-only",
    "GPL-3.0",
    "LGPL-3.0",
];
const BUNDLE_PERMISSIVE: &[&str] = &["MIT", "Apache-2.0", "BSD-3-Clause", "BSD-2-Clause", "ISC"];

fn is_sha256_digest(s: &str) -> bool {
    match s.strip_prefix("sha256:") {
        Some(h) => {
            h.len() == 64
                && h.bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        }
        None => false,
    }
}

/// Validate a parsed bundle manifest per REG-BUNDLE-001: every component carries
/// every prescribed field, every digest is a well-formed sha256, and the AGPL/MIT
/// licence firewall holds (an AGPL component into a permissive distribution
/// boundary only as a vendored, re-stamped, pinned copy). Pure (no I/O), so it is
/// unit-tested directly. Returns the pass summary, or the first violation.
fn verify_bundle_manifest(m: &BundleManifest) -> Result<String, String> {
    let present = |o: &Option<String>| o.as_deref().map(|s| !s.trim().is_empty()).unwrap_or(false);
    if m.schema_version.is_none() {
        return Err("manifest is missing the prescribed field 'schema_version'".into());
    }
    if !present(&m.bundle) {
        return Err("manifest is missing the prescribed field 'bundle'".into());
    }
    if !present(&m.distribution_licence) {
        return Err("manifest is missing the prescribed field 'distribution_licence'".into());
    }
    if m.component.is_empty() {
        return Err("manifest declares no components".into());
    }
    let dist = m.distribution_licence.as_deref().unwrap();
    let mut seen = std::collections::HashSet::new();
    for c in &m.component {
        let cid = c.id.as_deref().unwrap_or("<unnamed>");
        for (name, val) in [
            ("id", &c.id),
            ("repo", &c.repo),
            ("digest", &c.digest),
            ("source_commit", &c.source_commit),
            ("licence", &c.licence),
            ("adoption_mode", &c.adoption_mode),
        ] {
            if !present(val) {
                return Err(format!(
                    "component '{cid}' is missing the prescribed field '{name}'"
                ));
            }
        }
        if !seen.insert(cid.to_string()) {
            return Err(format!("duplicate component id '{cid}'"));
        }
        let digest = c.digest.as_deref().unwrap();
        if !is_sha256_digest(digest) {
            return Err(format!(
                "component '{cid}' digest is not a well-formed sha256: {digest}"
            ));
        }
        let licence = c.licence.as_deref().unwrap();
        let adoption = c.adoption_mode.as_deref().unwrap();
        if BUNDLE_PERMISSIVE.contains(&dist)
            && BUNDLE_COPYLEFT.contains(&licence)
            && adoption != "vendored-restamped-readonly"
        {
            return Err(format!(
                "licence firewall: copyleft component '{cid}' ({licence}) is consumed into a \
                 {dist} distribution boundary with adoption_mode '{adoption}'; AGPL is permitted \
                 only as vendored-restamped-readonly"
            ));
        }
    }
    Ok(format!(
        "bundle '{}' verified - {} components, distribution {}, licence firewall holds.",
        m.bundle.as_deref().unwrap(),
        m.component.len(),
        dist
    ))
}

pub(crate) fn cmd_bundle(
    repo: &Path,
    subcmd: BundleCommands,
    json: bool,
) -> Result<(), KernelError> {
    match subcmd {
        BundleCommands::Verify { path } => {
            let p = if path.is_absolute() {
                path.clone()
            } else {
                repo.join(&path)
            };
            let content = std::fs::read_to_string(&p).map_err(|e| {
                KernelError::InvalidInput(format!("cannot read {}: {e}", p.display()))
            })?;
            let manifest: BundleManifest = toml::from_str(&content)
                .map_err(|e| KernelError::InvalidInput(format!("bundle.lock parse error: {e}")))?;
            match verify_bundle_manifest(&manifest) {
                Ok(summary) => {
                    if json {
                        println!("{}", serde_json::json!({ "ok": true, "summary": summary }));
                    } else {
                        println!("OK: {summary}");
                        for c in &manifest.component {
                            println!(
                                "   {:8} {:16} {:30} {}",
                                c.id.as_deref().unwrap_or(""),
                                c.licence.as_deref().unwrap_or(""),
                                c.adoption_mode.as_deref().unwrap_or(""),
                                c.digest.as_deref().unwrap_or("")
                            );
                        }
                    }
                    Ok(())
                }
                Err(msg) => Err(KernelError::InvalidInput(format!("FAIL: {msg}"))),
            }
        }
    }
}

#[cfg(test)]
mod bundle_tests {
    use super::*;
    fn comp(id: &str, licence: &str, mode: &str) -> BundleComponent {
        BundleComponent {
            id: Some(id.into()),
            repo: Some("wlilley93/x".into()),
            digest: Some(format!("sha256:{}", "a".repeat(64))),
            source_commit: Some("abc1234".into()),
            licence: Some(licence.into()),
            adoption_mode: Some(mode.into()),
        }
    }
    fn manifest(dist: &str, comps: Vec<BundleComponent>) -> BundleManifest {
        BundleManifest {
            schema_version: Some(1),
            bundle: Some("house".into()),
            distribution_licence: Some(dist.into()),
            component: comps,
        }
    }
    #[test]
    fn passes_a_well_formed_bundle() {
        let m = manifest(
            "MIT",
            vec![
                comp("canon", "AGPL-3.0-only", "vendored-restamped-readonly"),
                comp("engine", "MIT", "vendored-readonly"),
            ],
        );
        assert!(verify_bundle_manifest(&m).is_ok());
    }
    #[test]
    fn fails_closed_on_licence_firewall_breach() {
        let m = manifest(
            "MIT",
            vec![comp("canon", "AGPL-3.0-only", "monorepo-package")],
        );
        let e = verify_bundle_manifest(&m).unwrap_err();
        assert!(e.contains("licence firewall"), "{e}");
    }
    #[test]
    fn fails_closed_on_missing_field() {
        let mut c = comp("canon", "MIT", "vendored-readonly");
        c.source_commit = None;
        let m = manifest("MIT", vec![c]);
        assert!(
            verify_bundle_manifest(&m)
                .unwrap_err()
                .contains("source_commit")
        );
    }
    #[test]
    fn fails_closed_on_bad_digest() {
        let mut c = comp("canon", "MIT", "vendored-readonly");
        c.digest = Some("sha256:notahex".into());
        let m = manifest("MIT", vec![c]);
        assert!(
            verify_bundle_manifest(&m)
                .unwrap_err()
                .contains("well-formed sha256")
        );
    }
}
