use std::path::PathBuf;

use crate::error::*;
use crate::types::*;
use crate::spec::*;

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
        path: &PathBuf,
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
        if glob.ends_with("/**") {
            let prefix = &glob[..glob.len() - 3];
            path.starts_with(prefix) || path == prefix
        } else if glob.contains("/**/") {
            let parts: Vec<&str> = glob.split("/**/").collect();
            if parts.len() == 2 {
                let (prefix, suffix) = (parts[0], parts[1]);
                path.starts_with(prefix) && path.ends_with(suffix)
            } else {
                false
            }
        } else if glob.contains("*") {
            let regex = glob
                .replace("**", ".*")
                .replace("*", "[^/]*");
            regex::Regex::new(&format!("^{}$", regex))
                .map(|re| re.is_match(path))
                .unwrap_or(false)
        } else {
            path == glob || path.starts_with(glob)
        }
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
                        remedy: "Run vjs route for this action and stage the resulting permit.".into(),
                    });
                }
                Some(permit) => {
                    // Check permit status
                    if matches!(permit.status, PermitStatus::Expired) {
                        ok = false;
                        findings.push(PermitGateFinding {
                            severity: Severity::Fatal,
                            code: "PERMIT-EXPIRED".into(),
                            path: Some(path.clone()),
                            message: format!(
                                "Matching permit '{}' has expired.",
                                permit.id.0
                            ),
                            remedy: "Run vjs route again or renew the permit.".into(),
                        });
                    } else if matches!(permit.status, PermitStatus::Revoked) {
                        ok = false;
                        findings.push(PermitGateFinding {
                            severity: Severity::Fatal,
                            code: "PERMIT-REVOKED".into(),
                            path: Some(path.clone()),
                            message: format!(
                                "Matching permit '{}' has been revoked.",
                                permit.id.0
                            ),
                            remedy: "Run vjs route again to obtain a new permit.".into(),
                        });
                    }

                    // Check obligations due before commit (only for active permits)
                    if matches!(permit.status, PermitStatus::Active) {
                        for obligation in &permit.obligations {
                            if matches!(obligation.due, ObligationDue::BeforeCommit) && obligation.required {
                                match obligation.kind {
                                    ObligationKind::DecisionLog => {
                                        let log_exists = logs.iter().any(|log| {
                                        log.id.contains(&permit.id.0) ||
                                        log.basis.iter().any(|b| b == &permit.id.0) ||
                                        log.issue.contains(&permit.id.0)
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
                                        let proof_exists = proofs.iter().any(|p| p.permit_id.0 == permit.id.0);
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
                message: format!("{} governed staged paths have valid permit coverage.", governed_paths.len()),
                remedy: "".into(),
            });
        }

        PermitGateResult { ok, findings }
    }

    fn find_matching_permit(
        path: &PathBuf,
        permits: &[Permit],
    ) -> Option<Permit> {
        let now = chrono::Utc::now();
        let path_str = path.to_string_lossy();

        permits.iter().find(|permit| {
            // Status must be Active or Closed
            let status_ok = matches!(permit.status, PermitStatus::Active | PermitStatus::Closed);
            if !status_ok {
                return false;
            }

            // Not expired
            let not_expired = if let Ok(expiry) = chrono::DateTime::parse_from_rfc3339(&permit.expires_at) {
                now < expiry.with_timezone(&chrono::Utc)
            } else {
                true // if we can't parse, assume not expired
            };
            if !not_expired {
                return false;
            }

            // Scope covers path
            let scope_covers = if let Some(ref scope) = permit.scope {
                if let Some(ref paths) = scope.paths {
                    paths.iter().any(|glob| PathClassifier::glob_matches(glob, &path_str))
                } else {
                    true // no path scope means all paths
                }
            } else {
                true // no scope means all paths
            };

            scope_covers
        }).cloned()
    }
}
