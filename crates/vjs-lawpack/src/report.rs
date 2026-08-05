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
    // ------------------------------------------------------------------
    // Operation Watertight WS4 tranche 1 (2026-08-05): duties ALREADY held by real
    // gates but unclaimed here. Every row names its gate; where an existing negative
    // control proves the gate, the label names that too. Rows marked "by
    // construction" follow the registry's standing precedent (keep_kernel_model_free
    // et al.): the capability is absent from the dependency closure or the code
    // path, which deny.toml or the loader's shape witnesses. NOT claimed, honestly:
    // delete_logs (.vjs/logs is not a governed_record_root, measured 2026-08-05),
    // enforce_order_word_limits / keep_decisions_short (no word-count gate exists),
    // name_the_instrument_that_caused_every_denial (citations are not universal on
    // findings), s2 machinery-claims filing checks (no filing gate yet).
    // ------------------------------------------------------------------
    // O5: unreadable orders refuse every binding command (ACT 11 s9).
    (
        "fail_closed_on_binding_commands_while_any_order_is_unreadable_naming_count_and_files",
        "O5 refusal (vjs_engine::door::refuse_if_orders_unreadable; control: mcp_door_parity)",
    ),
    (
        "treating_an_unloadable_instrument_as_repealed_or_not_in_force",
        "O5 refusal fails closed instead of skipping the unreadable record (same gate)",
    ),
    // The PC-14 D3 assent floor (ACT 11 s3/s7, REG-FRONT-DOOR-001).
    (
        "route_rather_than_block_where_the_record_declares_a_valid_assent_source",
        "PC-14 D3 assent floor (control: assent_resolution)",
    ),
    (
        "degrade_to_route_for_correction_for_any_record_declaring_a_valid_assent_source",
        "PC-14 D3 assent floor",
    ),
    (
        "void_or_block_a_record_declaring_a_valid_assent_source",
        "PC-14 D3 assent floor: the downgrade IS the must_not's teeth",
    ),
    // PC-17 grounding at the staged gate (ACT 11 s4).
    (
        "route_an_unresolved_operative_citation_for_correction",
        "ORDER_CITATION_UNRESOLVED (staged gate, PC-17; routed for correction)",
    ),
    (
        "the_clerk_declaring_an_order_void_for_a_dangling_citation",
        "ORDER_CITATION_UNRESOLVED states 'Routed for correction, not voided'",
    ),
    // The audit reports on itself (ACT 11 s14).
    (
        "report_this_acts_own_unwired_duties_in_the_audit",
        "conformance_audit enumerates ACT 11's own duties (control: conformance_ratchet)",
    ),
    // Lawpack lock (REG-KERNEL-001).
    (
        "load_only_law_that_hashes_to_the_pinned_digest_in_the_embedded_posture",
        "LAWPACK_LOCK_DRIFT (control: lawpack_lock falsified-digest test)",
    ),
    (
        "bump_a_pinned_digest_without_the_subscribers_deliberate_assent",
        "LAWPACK_LOCK_DRIFT forces the deliberate re-pin (refused twice, measured 2026-08-05)",
    ),
    // Order well-formedness + bench + apex (ACT-002:s10, REG-COURT-RECORD-001,
    // REG-FEDERATION-COORDINATION-001).
    (
        "validate_order_format",
        "ORDER_MALFORMED (staged gate, ACT-002:s10)",
    ),
    (
        "record_the_deciding_bench_on_every_new_order",
        "D10 bench-integrity constitutive codes (control: e2e_gate_harness, assent_resolution)",
    ),
    (
        "a_root_asserting_an_apex_or_final_court_function",
        "APEX_RECORD_IN_SUBSCRIBING_JURISDICTION + apex_routing_decision (control: assent_resolution)",
    ),
    // The permit gate holds the scope/proof/log duties (ACT-003:s3, ACT-004:s6/s7).
    (
        "act_within_permit_scope",
        "PermitGate scope_covers (control: e2e_gate_harness)",
    ),
    (
        "act_outside_permit_scope",
        "PermitGate scope_covers refuses the out-of-scope write",
    ),
    ("close_permit_without_proof", "PERMIT-PROOF-MISSING"),
    ("attach_required_proof", "PERMIT-PROOF-MISSING"),
    ("attach_required_proofs", "PERMIT-PROOF-MISSING"),
    (
        "write_log_for_material_decisions",
        "PERMIT-OBLIGATION-MISSING decision-log gate (control: permit_gate_messages)",
    ),
    // The route is not optional: an unrouted governed write has no permit and fails.
    (
        "call_vjs_route",
        "PermitGate: no routed permit, no governed write - the route is structurally forced",
    ),
    (
        "run_vjs_validate",
        "D4 install gate guarantees the hooks that run validate (INSTALL_HOOKS_MISSING)",
    ),
    // Boundary scanner limbs (ACT-005, ACT-006:s3).
    ("flag_secrets_in_scanner", "RedactScanner (deterministic)"),
    (
        "commit_secrets_to_public",
        "RedactScanner at validate; Token/Secret hard-block",
    ),
    (
        "remove_secrets_before_commit",
        "RedactScanner refusal stands until the secret is removed - the cure is forced",
    ),
    (
        "act_on_scanner_flags",
        "scanner Error/Fatal fails the gate closed; a flag cannot be waved through",
    ),
    (
        "publish_hostnames",
        "RedactScanner internal-hostname detection",
    ),
    (
        "load_gazette_as_runtime_authority",
        "by construction: the loader reads lawpack/v2 only; gazette-data.js is never a source",
    ),
    // ACT-001 structural duties.
    (
        "resolve_authority_in_hierarchy",
        "the deterministic resolver's rank order (control: golden_tests)",
    ),
    (
        "mark_proposed_law_as_draft",
        "is_live(): a status:draft instrument confers no force",
    ),
    (
        "load_v1_archive_as_runtime_by_default",
        "by construction: the loader reads lawpack/v2; V1 enters only by migrate-v1 + incorporation",
    ),
    // deny.toml closure witnesses.
    (
        "add_http_client_to_vjs_core",
        "deny.toml (reqwest/hyper banned from the kernel closure; ACT-003:s9)",
    ),
    (
        "kernel_auto_publish",
        "deny.toml network ban: a network-free kernel cannot publish anything",
    ),
    // Invariant evaluator shape (ACT-004:s3), same gate as use_llm_to_evaluate_invariant.
    (
        "use_cosine_for_invariant",
        "invariant evaluator is deterministic; no similarity scoring exists (ACT-004:s3)",
    ),
    (
        "use_free_form_script_for_invariant",
        "invariant evaluator runs typed predicates only (ACT-004:s3)",
    ),
    (
        "validate_record_schema",
        "LawpackValidator::validate on every load",
    ),
    // Human-approval and release outcomes block in gate mode (ACT-006).
    (
        "human_approval_required",
        "RouteOutcome::HumanApprovalRequired blocks in gate mode; DESTRUCTIVE_RECORD_DELETE surface",
    ),
    (
        "permit_with_human_approval",
        "RouteOutcome::HumanApprovalRequired blocks in gate mode",
    ),
    (
        "verify_release_warrant",
        "RouteOutcome::ReleaseWarrantRequired blocks in gate mode",
    ),
    (
        "verify_release_authority",
        "RouteOutcome::ReleaseWarrantRequired blocks in gate mode",
    ),
    (
        "load_canonical_lawpack_on_route",
        "the ONE door lawpack loader (CC-VJS 12) + LAWPACK_LOCK_DRIFT",
    ),
    // Hooks stay thin; the kernel stays the only smart point (REG-HOOKS-001,
    // REG-KERNEL-001, REG-FRONT-DOOR-001). The 40-word bound refused a long hook on
    // 2026-08-05 (INV-HOOKS-SHORT-001) - measured enforcement, not a promise.
    (
        "keep_hooks_thin_and_route_every_check_through_the_kernel",
        "REG-HOOKS-001 40-word bound (hook.rs Finding)",
    ),
    (
        "place_kernel_checking_logic_inside_a_hook",
        "REG-HOOKS-001 40-word bound",
    ),
    (
        "place_record_creation_or_any_kernel_checking_logic_inside_a_hook_or_the_mcp_adapter",
        "REG-HOOKS-001 40-word bound",
    ),
    (
        "keep_record_creation_logic_in_the_kernel_with_thin_hooks_and_adapters",
        "REG-HOOKS-001 40-word bound + thin adapters",
    ),
    (
        "let_the_kernel_deliberate_draft_or_judge_merits",
        "kernel is model-free by construction (deny.toml)",
    ),
    (
        "call_model_for_a_binding_decision",
        "kernel and hooks are model-free (deny.toml)",
    ),
    ("inject_long_context", "REG-HOOKS-001 40-word bound"),
    // The front door (REG-FRONT-DOOR-001, PC-14).
    (
        "bring_every_governed_record_into_being_only_through_the_kernel_record_creation_path",
        "K-1 staged coverage of every governed record (is_governed_record)",
    ),
    (
        "refuse_a_non_assented_off_front_door_record_at_the_write_pre_assent",
        "K-1 staged front-door gate: hard pre-assent, downgrade after",
    ),
    (
        "keep_the_absolute_path_commit_hook_the_bypass_proof_backstop_and_sole_guarantee",
        "D4 install gate verifies the absolute-path hooks (INSTALL_HOOKS_MISSING)",
    ),
    // Install manifest (REG-INSTALL-MANIFEST-001).
    (
        "reverify_the_manifest_at_validate_staged_and_pre_write",
        "D5 install manifest verify_surface at validate + pre_write",
    ),
    (
        "fail_closed_citing_the_instrument_behind_any_missing_or_stale_limb",
        "D5 install manifest fails closed",
    ),
    // Self-convening and court routing (REG-SELF-CONVENE-001, ACT-002:s6). The gate
    // mode's CourtRequired block prints 'convene on own motion; do not route the
    // fork to the Principal' and exits 2.
    (
        "route_the_fork_to_the_principal",
        "route gate mode: CourtRequired exits 2 naming the own-motion duty",
    ),
    (
        "convene_the_named_court_on_own_motion",
        "route gate mode CourtRequired block",
    ),
    (
        "proceed_without_a_ruling",
        "route gate mode CourtRequired block",
    ),
    (
        "check_existing_authority_before_court",
        "the router resolves binding authority first; WS2 prints the nearest known tags",
    ),
    (
        "convene_court_when_settled",
        "a binding authority on all fours disposes the route without a court",
    ),
    // WS4 tranche 2 (2026-08-05): the store-register gate (ACT 11 s13), landed with
    // its register and red seeds in the same commit that banks the ratchet fall.
    (
        "equal_governed_record_roots_to_the_register",
        "store-register gate (STORE-UNREGISTERED at validate; control: store_register tests)",
    ),
    (
        "report_an_unregistered_law_store_in_local_ci",
        "store-register stage in vjs local-ci (same gate as validate - one implementation)",
    ),
    (
        "register_justice_in_the_first_subscriber_jurisdiction_at_commencement",
        "store-register gate: the continuity citator is a registered store in the jurisdiction that carries it",
    ),
];
