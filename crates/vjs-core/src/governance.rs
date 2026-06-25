use std::path::{Path, PathBuf};

use crate::spec::*;
use crate::types::*;

/// Classify a path relative to repo root against governance rules
pub struct PathClassifier;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PathClassification {
    Governed,
    Exempt,
    Ungoverned,
}

impl PathClassifier {
    pub fn classify(
        path: &Path,
        permit_required: &[String],
        permit_exempt: &[String],
    ) -> PathClassification {
        let path_str = path.to_string_lossy();

        // Check exempt first
        if Self::matches_glob_any(&path_str, permit_exempt) {
            return PathClassification::Exempt;
        }

        // Check required
        if Self::matches_glob_any(&path_str, permit_required) {
            return PathClassification::Governed;
        }

        PathClassification::Ungoverned
    }

    fn matches_glob_any(path: &str, globs: &[String]) -> bool {
        globs.iter().any(|g| Self::glob_matches(g, path))
    }

    pub fn glob_matches(glob: &str, path: &str) -> bool {
        if let Some(prefix) = glob.strip_suffix("/**") {
            // Boundary-aware: "crates/**" covers crates and crates/..., never
            // crates-evil/... (a bare starts_with let sibling dirs through).
            path == prefix || path.starts_with(&format!("{}/", prefix))
        } else if glob.contains("/**/") {
            let parts: Vec<&str> = glob.split("/**/").collect();
            if parts.len() == 2 {
                let (prefix, suffix) = (parts[0], parts[1]);
                // "a/**/b" matches a/b and a/x/y/b, with both edges on a
                // path-separator boundary so "a2/b" and "a/xb" stay out.
                (path == format!("{}/{}", prefix, suffix))
                    || (path.starts_with(&format!("{}/", prefix))
                        && path.ends_with(&format!("/{}", suffix)))
            } else {
                false
            }
        } else if glob.contains('*') {
            regex::Regex::new(&Self::glob_to_regex(glob))
                .map(|re| re.is_match(path))
                .unwrap_or(false)
        } else {
            // A literal glob names exactly one path. starts_with here let
            // "Cargo.toml.bak" ride on a permit scoped to "Cargo.toml"; a
            // directory scope must be written as "dir/**".
            path == glob
        }
    }

    fn glob_to_regex(glob: &str) -> String {
        let mut re = String::from("^");
        let mut chars = glob.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '*' => {
                    if chars.peek() == Some(&'*') {
                        chars.next();
                        re.push_str(".*");
                    } else {
                        re.push_str("[^/]*");
                    }
                }
                '?' => re.push_str("[^/]"),
                c if r"\.+()[]{}^$|".contains(c) => {
                    re.push('\\');
                    re.push(c);
                }
                c => re.push(c),
            }
        }
        re.push('$');
        re
    }
}

/// Evaluate whether a staged path is covered by a valid permit
pub struct PermitGate;

#[derive(Clone, Debug)]
pub struct PermitGateResult {
    pub ok: bool,
    pub findings: Vec<PermitGateFinding>,
}

#[derive(Clone, Debug)]
pub struct PermitGateFinding {
    pub severity: Severity,
    pub code: String,
    pub path: Option<PathBuf>,
    pub message: String,
    pub remedy: String,
}

/// A permit scope glob "escapes the working root" when it is absolute or climbs out
/// via `..`/`~` - i.e. it reaches into another repo. Used by the D3 cross-repo guard.
pub fn path_escapes_root(g: &str) -> bool {
    let s = g.trim();
    s.starts_with('/')
        || s.starts_with('~')
        || s.starts_with("../")
        || s.contains("/../")
        || s == ".."
        || s.ends_with("/..")
}

impl PermitGate {
    pub fn evaluate(
        staged_paths: &[PathBuf],
        permits: &[Permit],
        logs: &[DecisionLog],
        proofs: &[Proof],
        permit_required: &[String],
        permit_exempt: &[String],
    ) -> PermitGateResult {
        let mut findings = Vec::new();
        let mut ok = true;

        let mut governed_paths: Vec<PathBuf> = Vec::new();
        let mut ungoverned_paths: Vec<PathBuf> = Vec::new();

        for path in staged_paths {
            let classification = PathClassifier::classify(path, permit_required, permit_exempt);
            match classification {
                PathClassification::Governed => {
                    governed_paths.push(path.clone());
                }
                PathClassification::Exempt => {
                    // Exempt paths do not require permits
                }
                PathClassification::Ungoverned => {
                    ungoverned_paths.push(path.clone());
                }
            }
        }

        // Warn on ungoverned paths (not fatal in alpha)
        for path in &ungoverned_paths {
            findings.push(PermitGateFinding {
                severity: Severity::Warning,
                code: "PATH-UNGOVERNED".into(),
                path: Some(path.clone()),
                message: "Staged path is not governed by permit rules. No permit required.".into(),
                remedy: "Add to permit_required or permit_exempt in .vjs/config.toml to make governance explicit.".into(),
            });
        }

        // For each governed path, check permit coverage
        for path in &governed_paths {
            let matching_permit = Self::find_matching_permit(path, permits);

            match matching_permit {
                None => {
                    ok = false;
                    findings.push(PermitGateFinding {
                        severity: Severity::Fatal,
                        code: "PERMIT-MISSING".into(),
                        path: Some(path.clone()),
                        message: format!(
                            "Governed staged path '{}' is not covered by an active permit.",
                            path.display()
                        ),
                        remedy: "Run vjs route for this action and stage the resulting permit."
                            .into(),
                    });
                }
                Some(permit) => {
                    // Check permit status
                    if matches!(permit.status, PermitStatus::Expired)
                        || (matches!(permit.status, PermitStatus::Active)
                            && Self::permit_is_expired(&permit, chrono::Utc::now()))
                    {
                        ok = false;
                        findings.push(PermitGateFinding {
                            severity: Severity::Fatal,
                            code: "PERMIT-EXPIRED".into(),
                            path: Some(path.clone()),
                            message: format!("Matching permit '{}' has expired.", permit.id.0),
                            remedy: "Run vjs route again or renew the permit.".into(),
                        });
                    } else if matches!(permit.status, PermitStatus::Revoked) {
                        ok = false;
                        findings.push(PermitGateFinding {
                            severity: Severity::Fatal,
                            code: "PERMIT-REVOKED".into(),
                            path: Some(path.clone()),
                            message: format!("Matching permit '{}' has been revoked.", permit.id.0),
                            remedy: "Run vjs route again to obtain a new permit.".into(),
                        });
                    } else if matches!(permit.status, PermitStatus::Closed) {
                        // A closed permit's work is done. Letting it cover NEW
                        // staged changes would also skip its obligations, which
                        // are only checked while Active - a bypass, not a grace.
                        ok = false;
                        findings.push(PermitGateFinding {
                            severity: Severity::Fatal,
                            code: "PERMIT-CLOSED".into(),
                            path: Some(path.clone()),
                            message: format!(
                                "Matching permit '{}' is closed and does not excuse new staged changes.",
                                permit.id.0
                            ),
                            remedy: "Run vjs route again to obtain a new permit for this work.".into(),
                        });
                    }

                    // Check obligations due before commit (only for usable permits)
                    if matches!(permit.status, PermitStatus::Active)
                        && !Self::permit_is_expired(&permit, chrono::Utc::now())
                    {
                        for obligation in &permit.obligations {
                            if matches!(obligation.due, ObligationDue::BeforeCommit)
                                && obligation.required
                            {
                                match obligation.kind {
                                    ObligationKind::DecisionLog => {
                                        let log_exists = logs.iter().any(|log| {
                                            log.id.contains(&permit.id.0)
                                                || log.basis.iter().any(|b| b == &permit.id.0)
                                                || log.issue.contains(&permit.id.0)
                                        });
                                        if !log_exists {
                                            ok = false;
                                            findings.push(PermitGateFinding {
                                                severity: Severity::Fatal,
                                                code: "PERMIT-OBLIGATION-MISSING".into(),
                                                path: Some(path.clone()),
                                                message: format!(
                                                    "Permit '{}' requires a decision log before commit.",
                                                    permit.id.0
                                                ),
                                                remedy: format!("Run vjs log from-permit {} --decision <decision> --why <why>.", permit.id.0),
                                            });
                                        }
                                    }
                                    ObligationKind::Proof => {
                                        let proof_exists =
                                            proofs.iter().any(|p| p.permit_id.0 == permit.id.0);
                                        if !proof_exists {
                                            ok = false;
                                            findings.push(PermitGateFinding {
                                                severity: Severity::Fatal,
                                                code: "PERMIT-PROOF-MISSING".into(),
                                                path: Some(path.clone()),
                                                message: format!(
                                                    "Permit '{}' requires a proof before commit.",
                                                    permit.id.0
                                                ),
                                                remedy: format!("Run vjs proof add --permit {} --status passed.", permit.id.0),
                                            });
                                        }
                                    }
                                    _ => {
                                        // Other obligation kinds not yet enforced
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if governed_paths.is_empty() && ungoverned_paths.is_empty() {
            findings.push(PermitGateFinding {
                severity: Severity::Info,
                code: "PERMIT-GATE".into(),
                path: None,
                message: "No staged paths require permit validation.".into(),
                remedy: "".into(),
            });
        } else if !governed_paths.is_empty() && ok {
            findings.push(PermitGateFinding {
                severity: Severity::Info,
                code: "PERMIT-GATE".into(),
                path: None,
                message: format!(
                    "{} governed staged paths have valid permit coverage.",
                    governed_paths.len()
                ),
                remedy: "".into(),
            });
        }

        PermitGateResult { ok, findings }
    }

    /// Fail closed: an unparseable expiry never excuses a write.
    fn permit_is_expired(permit: &Permit, now: chrono::DateTime<chrono::Utc>) -> bool {
        match chrono::DateTime::parse_from_rfc3339(&permit.expires_at) {
            Ok(expiry) => now >= expiry.with_timezone(&chrono::Utc),
            Err(_) => true,
        }
    }

    fn scope_covers(permit: &Permit, path_str: &str) -> bool {
        if let Some(ref scope) = permit.scope {
            if let Some(ref paths) = scope.paths {
                paths
                    .iter()
                    .any(|glob| PathClassifier::glob_matches(glob, path_str))
            } else {
                false // a permit with a scope but no paths covers nothing
            }
        } else {
            false // a permit with no scope covers nothing - it must name the
            // paths it excuses, or it would blanket-cover every governed
            // write (the permit-scoping rule). A route now scopes its permit.
        }
    }

    /// A usable permit covers the path: Active, unexpired, in scope. This is
    /// the question a pre-write hook asks before the work happens.
    pub fn covers(path: &str, permits: &[Permit]) -> bool {
        let now = chrono::Utc::now();
        permits.iter().any(|p| {
            matches!(p.status, PermitStatus::Active)
                && !Self::permit_is_expired(p, now)
                && Self::scope_covers(p, path)
        })
    }

    /// D3 ([2026] VJS-PC 13 "Teeth For The Front Door"): the thin working-root
    /// jurisdiction check. A "true cross-repo permit" is one whose scope reaches
    /// OUTSIDE the working root - an absolute path, or one escaping via `..`/`~`.
    /// Under ACT-007:s3 a reach into another repo's law is lawful ONLY by a Privy
    /// Council order or Principal assent; the repo-local permit model carries no
    /// such authority field, so the kernel FAILS CLOSED. This is a narrow exception
    /// path folded onto the canon-write gate, not a second gate. It is
    /// false-positive-free: every lawful permit scopes in-root globs.
    /// Returns (permit id, offending glob) for each escaping scope path.
    pub fn cross_repo_reaches(permits: &[Permit]) -> Vec<(String, String)> {
        let mut out = Vec::new();
        for p in permits {
            if let Some(ref scope) = p.scope
                && let Some(ref paths) = scope.paths
            {
                for g in paths {
                    if path_escapes_root(g) {
                        out.push((p.id.0.clone(), g.clone()));
                    }
                }
            }
        }
        out
    }

    /// Prefer a usable (Active, unexpired) permit; otherwise return the first
    /// scope-covering permit so evaluate can report WHY it fails (expired,
    /// revoked, closed) instead of a bare PERMIT-MISSING.
    fn find_matching_permit(path: &Path, permits: &[Permit]) -> Option<Permit> {
        let now = chrono::Utc::now();
        let path_str = path.to_string_lossy();

        let covering: Vec<&Permit> = permits
            .iter()
            .filter(|p| Self::scope_covers(p, &path_str))
            .collect();

        covering
            .iter()
            .find(|p| matches!(p.status, PermitStatus::Active) && !Self::permit_is_expired(p, now))
            .or_else(|| covering.first())
            .map(|p| (*p).clone())
    }
}

#[cfg(test)]
mod cross_repo_tests {
    use super::path_escapes_root;

    #[test]
    fn in_root_globs_never_escape() {
        for g in [
            "crates/**",
            "lawpack/v2/**",
            "Cargo.toml",
            "gazette*",
            ".vjs/orders/**",
            "src/main.rs",
        ] {
            assert!(
                !path_escapes_root(g),
                "{g} is in-root and must not be flagged"
            );
        }
    }

    #[test]
    fn absolute_or_climbing_globs_escape() {
        for g in [
            "/etc/passwd",
            "/home/other/repo/**",
            "~/secrets",
            "../canon/lawpack/**",
            "crates/../../other/**",
            "..",
            "a/b/..",
        ] {
            assert!(path_escapes_root(g), "{g} reaches outside the working root");
        }
    }
}
