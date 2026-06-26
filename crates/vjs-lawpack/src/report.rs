//! The validation report types (ValidationReport / ValidationFinding) and the duty-token tables
//! the validator emits.

use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValidationReport {
    pub ok: bool,
    pub findings: Vec<ValidationFinding>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValidationFinding {
    pub severity: Severity,
    pub code: String,
    pub path: Option<PathBuf>,
    pub message: String,
    pub suggested_fix: Option<String>,
}

// ---------------------------------------------------------------------------
// Full-spectrum conformance audit (PC-13 D11).
//
// Produced THROUGH the kernel (not by an agent reading context - the disease this
// line prosecutes): the kernel enumerates every substantive duty (must / must_not
// / prohibits) in every in-force instrument's kernel_effect, and records,
// deterministically, whether each is bound to a deterministic kernel gate. The
// gate binding is a CURATED registry: a duty is "wired" only if a named gate can be
// pointed at; everything else is honestly reported UNWIRED. The map of unwired
// duties is the factual predicate for the reserved D12 single-front-door instrument.
// ---------------------------------------------------------------------------

/// (duty token -> the gate that enforces it). A duty absent here is reported
/// UNWIRED. Conservative by construction: only a real, named, deterministic gate
/// earns a "wired" mark, so the map cannot overstate coverage.
pub(crate) const GATE_REGISTRY: &[(&str, &str)] = &[
    // D1 canon-write gate (ACT-005:s1/s5, ACT-007:s4)
    (
        "publish_private_repo_paths",
        "D1 canon-write gate (CANON_BOUNDARY_VIOLATION)",
    ),
    (
        "publish_client_facts",
        "D1 canon-write gate (CANON_BOUNDARY_VIOLATION)",
    ),
    (
        "publish_private_facts_from_contributor_repos",
        "D1 canon-write gate",
    ),
    (
        "publish_private_facts_from_contributors",
        "D1 canon-write gate",
    ),
    (
        "local_order_bind_other_repos",
        "D1 canon-write gate (ACT-007:s4)",
    ),
    // D3 cross-repo guard (ACT-007:s3)
    (
        "local_law_override_canonical_without_authority",
        "D3 cross-repo permit guard (CROSS_REPO_PERMIT)",
    ),
    // D2 citation uniqueness/allocation (ACT-004:s8)
    (
        "check_citation_uniqueness",
        "D2 citation gate (CITATION_COLLISION)",
    ),
    (
        "allow_duplicate_citations",
        "D2 citation gate (CITATION_COLLISION)",
    ),
    // Deterministic boundary scanner (ACT-005:s3/s7) - high-confidence kinds
    ("publish_secrets", "RedactScanner (deterministic)"),
    ("publish_tokens", "RedactScanner (deterministic)"),
    ("publish_credentials", "RedactScanner (deterministic)"),
    (
        "run_boundary_scan_on_public_changes",
        "RedactScanner at validate",
    ),
    ("run_boundary_scan", "RedactScanner at validate"),
    (
        "use_llm_for_boundary_check",
        "RedactScanner is deterministic (no LLM)",
    ),
    ("use_deterministic_scanner", "RedactScanner (deterministic)"),
    // D4/D5 install completeness + manifest (REG-INVOCATION-001, ACT-007:s1)
    (
        "install_enforcement_hooks",
        "D4/D5 install gate (INSTALL_HOOKS_MISSING)",
    ),
    (
        "subscribe_to_a_named_lawpack_and_lock_its_digest",
        "D4/D5 install gate",
    ),
    (
        "record_a_local_sovereign_invocation",
        "D4/D5 install gate (INSTALL_INVOCATION_MISSING)",
    ),
    (
        "create_config_toml_on_install",
        "D4/D5 install gate (INSTALL_CONFIG_MISSING)",
    ),
    ("install_hooks_on_init", "D4/D5 install gate"),
    ("install_validation_hooks", "D4/D5 install gate"),
    // D7/D10 bench + tier (ACT-002, [2026] VJS-SC 2, REG-COURT-RECORD-001)
    ("local_order_bind_other_repos_tier", "D7 tier-floor"),
    // Permit + log + lawpack-lock (existing gates)
    (
        "obtain_permit_before_governed_write",
        "PermitGate (PERMIT-MISSING)",
    ),
    ("close_permit_with_proof", "PermitGate obligations"),
    ("persist_and_close_permits", "PermitGate"),
    ("write_decision_log", "decision-log obligation gate"),
    (
        "evaluate_invariants_mechanically",
        "invariant evaluator at validate",
    ),
    (
        "wire_invariants_to_validate",
        "invariant evaluator at validate",
    ),
    (
        "check_lawpack_lock_consistency",
        "lawpack lock consistency (ACT-007:s7)",
    ),
    ("check_incorporation", "lawpack referential integrity"),
    // Hooks (REG-HOOKS-001) - closed five-event surface, thin adapters
    (
        "keep_hooks_short",
        "REG-HOOKS-001 40-word bound (hook.rs Finding)",
    ),
    (
        "keep_kernel_model_free",
        "kernel is model-free by construction",
    ),
    (
        "keep_kernel_network_free",
        "kernel is network-free by construction",
    ),
    (
        "keep_kernel_deterministic",
        "kernel is deterministic by construction",
    ),
    // PC-14 (the single front door) + the post-D11 improvement gates.
    (
        "agent_draft_becomes_binding_by_fact_of_being_written",
        "PC-14 front door (REG-FRONT-DOOR-001): law only through the commit gate",
    ),
    ("act_without_valid_permit", "PermitGate (PERMIT-MISSING)"),
    (
        "local_law_override_canonical_without_authority",
        "D3 cross-repo guard + canon-write gate",
    ),
    (
        "subordinate_validation_voiding_or_blocking_a_sovereign_assented_record",
        "PC-14 D3 assent floor (downgrades, never voids/blocks)",
    ),
    (
        "silently_excluding_a_sovereign_assented_record_for_a_defect_rather_than_routing_it_for_correction",
        "PC-14 D3 assent floor (surfaces + routes for correction)",
    ),
    (
        "check_lawpack_lock_consistency",
        "improvement #2 (LAWPACK_LOCK_DRIFT, ACT-007:s7)",
    ),
    (
        "lock_the_install_surface_atomically_at_invoke",
        "D5 install manifest (.vjs/install.lock)",
    ),
    (
        "accept_order_without_directives",
        "improvement #5 (ORDER_MALFORMED, ACT-002:s10)",
    ),
    (
        "accept_order_without_runtime_summary",
        "improvement #5 (ORDER_MALFORMED, ACT-002:s10)",
    ),
    (
        "delete_old_records",
        "improvement #6 (DESTRUCTIVE_RECORD_DELETE surface)",
    ),
    (
        "proceed_without_human_approval",
        "improvement #6 (DESTRUCTIVE_RECORD_DELETE; permit gate blocks un-permitted)",
    ),
    (
        "publish_logs",
        "improvement #7 (BOUNDARY_MEDIA_IN_CANON, ACT-005:s1)",
    ),
    (
        "publish_screenshots",
        "improvement #7 (BOUNDARY_MEDIA_IN_CANON, ACT-005:s1)",
    ),
    // PC-16 assent-RESOLUTION floor (improvement #5 burndown): a record carries binding
    // force only if its declared assent_source RESOLVES to a real Sovereign-assent event
    // (ACT-COMPUTER-FIRST-REALM s.23), the under-implementation [2026] VJS-PC 16 closed.
    (
        "binding_force_without_traceable_assent_source",
        "PC-16 assent-resolution floor (vjs-engine::assent; s.23 traceable-assent)",
    ),
    (
        "require_authorised_adoption_for_binding_force",
        "PC-16 assent-resolution floor (force only from a resolving adoption)",
    ),
    (
        "agent_self_authorise_law",
        "PC-16 assent-resolution floor (a self-declared assent resolving to nothing confers no force)",
    ),
    (
        "agent_self_authorised_law",
        "PC-16 assent-resolution floor (a self-declared assent resolving to nothing confers no force)",
    ),
    // D12 triage: the model-free / network-free kernel prohibitions are enforced by
    // deny.toml (cargo deny check bans), the authoritative dependency-closure witness
    // added at BREACH-2026-06-12 - a capability REMOVED, not merely prohibited.
    (
        "kernel_call_llm",
        "deny.toml (cargo deny bans model crates from the kernel closure)",
    ),
    (
        "kernel_use_vector_search",
        "deny.toml (no vector-search crate in the kernel closure)",
    ),
    (
        "add_model_call_to_vjs_core",
        "deny.toml + kernel model-free by construction (ACT-003:s8)",
    ),
    (
        "add_vector_search_to_vjs_core",
        "deny.toml (cargo deny bans; ACT-003:s8)",
    ),
    (
        "add_network_dependency_to_vjs_core",
        "deny.toml (reqwest/hyper banned from the kernel closure; ACT-003:s9)",
    ),
    (
        "use_llm_to_evaluate_invariant",
        "invariant evaluator is deterministic, no LLM (ACT-004:s3)",
    ),
];
