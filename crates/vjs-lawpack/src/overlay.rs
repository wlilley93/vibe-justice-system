//! The two-tier overlay loader ([2026] VJS-PC 15 D1/D2): load the canon Tier-1 floors
//! (read-only) together with a subscriber's Tier-2 local rules, applying anti-relaxation
//! AT LOAD. Extends the single-tier `LawpackLoader::load`. Subordinate machinery under
//! ACT-CONSOLIDATION-FRAMEWORK:s7 - it gives load-time teeth to ACT-007:s3/s4 +
//! REG-FEDERATION-COORDINATION-001 and amends no statute, bench, tier, jurisdiction, or
//! assent rule.
//!
//! Canon ships an EMPTY floors frame; the subscriber supplies its dimensions, floors'
//! force, rules, and runtime-act verbs in its own Tier-2. The cascade algebra and the
//! relaxation rule live in `vjs_core::scope`; this is the disk-loading + finding seam.

use std::path::Path;
use walkdir::WalkDir;

use serde::de::DeserializeOwned;
use vjs_core::KernelError;
use vjs_core::front_door::is_valid_assent_value;
use vjs_core::report::Finding;
use vjs_core::scope::{Floor, LocalRule, RuleFate, rule_fate};
use vjs_core::types::Severity;

/// The loaded, anti-relaxation-checked two-tier law: canon floors (Tier-1) plus the
/// KEPT subscriber rules (Tier-2). A void (relaxing, un-authorised) rule is absent from
/// `local`; its defect is in the loader's findings. A rule routed for correction (it
/// relaxes a floor but bears valid assent) is KEPT and also carries a Warning.
#[derive(Clone, Debug, Default)]
pub struct Overlay {
    pub floors: Vec<Floor>,
    pub local: Vec<LocalRule>,
}

pub struct OverlayLoader;

impl OverlayLoader {
    /// Load canon Tier-1 floors from `canon_floors_dir` and subscriber Tier-2 rules from
    /// `local_dir`, run anti-relaxation at load, and return the kept Overlay plus every
    /// defect. Absent dirs load to empty (canon ships an empty floors frame). Pure of
    /// effects beyond reading the two dirs.
    pub fn load(
        canon_floors_dir: &Path,
        local_dir: &Path,
    ) -> Result<(Overlay, Vec<Finding>), KernelError> {
        let floors = load_yaml_dir::<Floor>(canon_floors_dir)?;
        let candidates = load_yaml_dir::<LocalRule>(local_dir)?;

        let mut findings = Vec::new();
        let mut local = Vec::new();
        for rule in candidates {
            match rule_fate(&rule, &floors, is_valid_assent_value) {
                // Only adds restriction, or an authorised ACT-007:s3 override: kept.
                RuleFate::Kept => local.push(rule),
                // VJS-ACT 10: bears valid assent - never void/block. Keep it with force
                // AND surface the defect as a route-for-correction Warning.
                RuleFate::RouteForCorrection { floor, verb } => {
                    findings.push(
                        Finding::new(
                            Severity::Warning,
                            "OVERLAY_RELAXATION_ASSENTED",
                            format!(
                                "Tier-2 rule '{}' grants '{verb}' which canon floor '{floor}' \
                                 forbids; it declares a valid assent_source, so it is routed for \
                                 correction, not voided (ACT-007:s3; VJS-ACT 10).",
                                rule.id
                            ),
                        )
                        .citing("ACT-007:s3"),
                    );
                    local.push(rule);
                }
                // Relaxes a covering floor with no s3 authority and no assent: only THIS
                // rule is denied force; the floor stands, canon is untouched. Named, not
                // silently swallowed.
                RuleFate::Void { floor, verb } => {
                    findings.push(
                        Finding::new(
                            Severity::Error,
                            "OVERLAY_RELAXATION_VOID",
                            format!(
                                "Tier-2 rule '{}' grants '{verb}' which canon floor '{floor}' \
                                 forbids and carries no ACT-007:s3 authority; it is VOID at load \
                                 (denied force). Local law may only ADD restriction.",
                                rule.id
                            ),
                        )
                        .citing("ACT-007:s3")
                        .fix(
                            "Remove the relaxing grant, or declare an ACT-007:s3 authority \
                             (a Privy Council order, or a valid assent_source) on the rule.",
                        ),
                    );
                    // not pushed: void at load
                }
            }
        }
        Ok((Overlay { floors, local }, findings))
    }
}

/// Load every `*.yaml` in a directory (depth 1) into `T`. A missing directory loads to
/// an empty Vec - the empty-frame default.
fn load_yaml_dir<T: DeserializeOwned>(dir: &Path) -> Result<Vec<T>, KernelError> {
    let mut out = Vec::new();
    if !dir.exists() {
        return Ok(out);
    }
    for entry in WalkDir::new(dir).max_depth(1) {
        let entry = entry.map_err(|e| KernelError::Io(e.to_string()))?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
            let content =
                std::fs::read_to_string(path).map_err(|e| KernelError::Io(e.to_string()))?;
            let item: T = serde_yaml::from_str(&content)
                .map_err(|e| KernelError::Serialization(e.to_string()))?;
            out.push(item);
        }
    }
    Ok(out)
}
