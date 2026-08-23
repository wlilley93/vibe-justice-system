//! The permit gate: the live pre-write authorization for governed paths.
//!
//! `PermitGate::evaluate` is the full staged-paths check (governed vs ungoverned, permit status,
//! before-commit obligations); `PermitGate::covers` is the thin pre-write question, routed THROUGH
//! the unified capability primitive (`PathScope` is a permit glob as a capability `Resource`), so the
//! decision is identical to the legacy scope match by construction, with deny-dominance now available.
//! `cross_repo_reaches` is the D3 working-root jurisdiction guard (fail-closed on an escaping scope).

use std::path::{Path, PathBuf};

use super::path_class::{PathClassification, PathClassifier};
use crate::capability::{CapabilityStore, Decision, Effect, Resource};
use crate::spec::*;
use crate::types::*;

/// The subject under which permits are projected into capabilities. `PermitGate::covers` asks
/// "is this path covered by SOME active permit" - actor-agnostic, exactly as the legacy matcher -
/// so every projected permit shares this one subject and `covers` authorizes against it.
const PERMIT_SUBJECT: &str = "permit-holder";

/// A permit's path-glob scope, as a capability RESOURCE vocabulary (capability.rs is generic over
/// `Resource`; PC-19 K2). `covers` delegates to the kernel's ONE glob semantics
/// (PathClassifier::glob_matches), so a capability-backed permit decision is IDENTICAL to the legacy
/// scope match BY CONSTRUCTION - no glob translation, no edge-case (gazette*, trailing-slash, `**`)
/// risk. A pattern is a permit glob; a concrete request is an exact path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathScope(pub String);

impl Resource for PathScope {
    fn covers(&self, req: &PathScope) -> bool {
        PathClassifier::glob_matches(&self.0, &req.0)
    }
    fn within(&self, parent: &PathScope) -> bool {
        // Permits do not delegate today; a sound-conservative attenuation check (never widens):
        // equal globs, or a parent whose glob covers the child glob's own form.
        self.0 == parent.0 || parent.covers(self)
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
                                                message: Self::obligation_missing_message(
                                                    &permit.id.0,
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

    /// The PERMIT-OBLIGATION-MISSING message, as a named artefact so a test can pin
    /// the discharge contract to the words the operator actually reads. The contract
    /// itself lives at the containment check; this states it out loud.
    pub fn obligation_missing_message(permit_id: &str) -> String {
        format!(
            "Permit '{permit_id}' requires a decision log before commit. THE DISCHARGE CONTRACT, so \
             nobody has to read this gate's source to learn it: a log discharges this \
             obligation only if its id, basis or issue CONTAINS the permit id above - \
             `vjs log decision ... --basis <permit-id>` is the canonical form. Two logs \
             written in the subscribing jurisdiction on 2026-08-05 failed here for carrying \
             citations in basis instead of the permit id."
        )
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

    /// Project the USABLE permits (Active + unexpired) into a capability store. The permit gate
    /// owns the clock here (capability.rs is deterministic + clock-free); each in-scope glob of a
    /// usable permit becomes an Allow `PathScope` capability under the actor-agnostic subject. The
    /// unified capability primitive (K-4..K-11) thus becomes the live pre-write authorization
    /// engine - a permit IS a profile of a capability - and deny-dominance is available to it,
    /// which path-permits alone could never express.
    fn permit_capability_store(
        permits: &[Permit],
        now: chrono::DateTime<chrono::Utc>,
    ) -> CapabilityStore<PathScope> {
        let mut store = CapabilityStore::new();
        for p in permits {
            let usable =
                matches!(p.status, PermitStatus::Active) && !Self::permit_is_expired(p, now);
            if !usable {
                continue;
            }
            if let Some(scope) = &p.scope
                && let Some(paths) = &scope.paths
            {
                for glob in paths {
                    // rights are fixed (&["write"]) so issue_root never errors here.
                    let _ = store.issue_root(
                        PERMIT_SUBJECT,
                        PathScope(glob.clone()),
                        &["write"],
                        Effect::Allow,
                        None,
                    );
                }
            }
        }
        store
    }

    /// A usable permit covers the path: Active, unexpired, in scope. This is the question a
    /// pre-write hook asks before the work happens. Now routed THROUGH the unified capability
    /// primitive: identical to the legacy scope match by construction (PathScope::covers delegates
    /// to the same glob semantics + the same usable filter), with deny-dominance now available.
    pub fn covers(path: &str, permits: &[Permit]) -> bool {
        let mut store = Self::permit_capability_store(permits, chrono::Utc::now());
        matches!(
            store.authorize(PERMIT_SUBJECT, PathScope(path.to_string()), "write"),
            Decision::Allowed { .. }
        )
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

#[cfg(test)]
mod permit_capability_tests {
    use super::*;
    use crate::capability::DenyReason;

    fn permit(globs: &[&str], status: PermitStatus, expires: &str) -> Permit {
        Permit {
            id: PermitId("PERMIT-test".into()),
            route_id: RouteId("ROUTE-test".into()),
            actor: "lexby".into(),
            scope: Some(Scope {
                paths: Some(globs.iter().map(|g| g.to_string()).collect()),
                jurisdictions: None,
                action_kinds: None,
                issue_tags: None,
                records: None,
            }),
            obligations: vec![],
            expires_at: expires.into(),
            status,
            self_issued: true,
            meaning: None,
            intent_digest: None,
            law_source: Vec::new(),
        }
    }

    /// The legacy covers logic, reconstructed from PUBLIC primitives, kept inline so the
    /// capability-backed `covers` is held to it exactly (fail-closed expiry, same glob semantics).
    fn legacy_covers(path: &str, permits: &[Permit]) -> bool {
        let now = chrono::Utc::now();
        permits.iter().any(|p| {
            let active = matches!(p.status, PermitStatus::Active);
            let unexpired = chrono::DateTime::parse_from_rfc3339(&p.expires_at)
                .map(|e| now < e.with_timezone(&chrono::Utc))
                .unwrap_or(false); // unparseable expiry never excuses a write
            let in_scope = p
                .scope
                .as_ref()
                .and_then(|s| s.paths.as_ref())
                .map(|paths| paths.iter().any(|g| PathClassifier::glob_matches(g, path)))
                .unwrap_or(false);
            active && unexpired && in_scope
        })
    }

    /// K-1 integration: routing `covers` through the unified capability primitive is IDENTICAL to
    /// the legacy glob match for every path - across the real permit glob forms, including the edge
    /// cases (mid-segment `gazette*`, prefix-collision `crates-evil`, the `Cargo.toml.bak` literal
    /// trap, and an expired permit's exclusion). The primitive is now the live decision engine.
    #[test]
    fn the_capability_backed_covers_is_identical_to_the_legacy_glob_match() {
        let future = "2099-01-01T00:00:00Z";
        let past = "2000-01-01T00:00:00Z";
        let permits = vec![
            permit(&["crates/**"], PermitStatus::Active, future),
            permit(&["Cargo.toml"], PermitStatus::Active, future),
            permit(&["gazette*"], PermitStatus::Active, future),
            permit(
                &["lawpack/v2/decisions/DEC-X.yaml"],
                PermitStatus::Active,
                future,
            ),
            permit(&["scripts/**"], PermitStatus::Expired, past), // excluded (expired)
            permit(&["README.md"], PermitStatus::Revoked, future), // excluded (revoked)
        ];
        for path in [
            "crates/vjs-core/src/lib.rs", // covered by crates/**
            "crates-evil/x",              // prefix collision - NOT covered
            "Cargo.toml",                 // literal hit
            "Cargo.toml.bak",             // literal must not prefix-match
            "gazette-2026.html",          // mid-segment glob hit
            "gazettes/x",                 // gazette* must not cross a slash
            "lawpack/v2/decisions/DEC-X.yaml",
            "scripts/run.sh", // expired permit -> not covered
            "README.md",      // revoked permit -> not covered
            "unknown/path",
        ] {
            assert_eq!(
                PermitGate::covers(path, &permits),
                legacy_covers(path, &permits),
                "capability-backed covers diverged from the legacy match on {path}"
            );
        }
    }

    /// K-2 (Visibility != Authority): a GOVERNED, visible path with no covering permit confers no
    /// authority - the primitive denies with NoCapability. Seeing the governed surface grants
    /// nothing; only a permit-projected capability does.
    #[test]
    fn a_governed_path_with_no_permit_capability_is_denied_at_the_primitive() {
        let permits = vec![permit(
            &["crates/**"],
            PermitStatus::Active,
            "2099-01-01T00:00:00Z",
        )];
        // a governed order path is visible to the gate but uncovered:
        assert!(!PermitGate::covers(
            "lawpack/v2/orders/2026-VJS-PC-099.yaml",
            &permits
        ));
        let mut store = PermitGate::permit_capability_store(&permits, chrono::Utc::now());
        assert!(matches!(
            store.authorize(
                PERMIT_SUBJECT,
                PathScope("lawpack/v2/orders/x.yaml".into()),
                "write"
            ),
            Decision::Denied {
                reason: DenyReason::NoCapability
            }
        ));
    }

    /// Deny-dominance is now reachable for permits via the primitive - a capability path-permits
    /// alone could never express. A Deny on a sensitive subtree beats a broad allow.
    #[test]
    fn a_deny_capability_dominates_an_allowing_permit_in_the_permit_context() {
        let permits = vec![permit(
            &["lawpack/v2/**"],
            PermitStatus::Active,
            "2099-01-01T00:00:00Z",
        )];
        let mut store = PermitGate::permit_capability_store(&permits, chrono::Utc::now());
        store
            .issue_root(
                PERMIT_SUBJECT,
                PathScope("lawpack/v2/orders/**".into()),
                &["write"],
                Effect::Deny,
                None,
            )
            .unwrap();
        // an ordinary path under the broad allow still passes
        assert!(matches!(
            store.authorize(
                PERMIT_SUBJECT,
                PathScope("lawpack/v2/specs/s.yaml".into()),
                "write"
            ),
            Decision::Allowed { .. }
        ));
        // the denied subtree is refused even though the broad allow covers it (deny dominates)
        assert!(matches!(
            store.authorize(
                PERMIT_SUBJECT,
                PathScope("lawpack/v2/orders/o.yaml".into()),
                "write"
            ),
            Decision::Denied {
                reason: DenyReason::ExplicitDeny
            }
        ));
    }
}
