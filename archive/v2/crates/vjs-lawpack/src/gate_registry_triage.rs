//! The s4(e) triage entries (tranches 2 and 3, 2026-08-05/06), split from the core
//! registry when the sittings grew it past the structural-cleanliness ceiling.
//! One logical registry: `classify_token` chains this table after the core, so a
//! duty is wired if EITHER table names its gate (behavior-preserving split).

pub(crate) const GATE_REGISTRY_TRIAGE: &[(&str, &str)] = &[
    // s4(e) triage tranche 2 (2026-08-05, four-seat sitting; report at
    // .vjs/submissions/COMMISSION-REPORT-2026-08-05-s4e-triage-tranche-2.md): every
    // row below re-verified seat-by-seat against its code path AND its negative
    // control before entry. Three tokens ride the one LAWPACK_LOCK_DRIFT gate -
    // three duty rows, ONE enforcement fact, said here so the audit never reads
    // them as three independent gates.
    (
        "record_supersession_explicitly",
        "warrant-register gate (WARRANT-SUPERSESSION-IMPLICIT/-DANGLING; control: prose_only_supersession_is_refused) for commission warrants; ORPHAN_SUPERSESSION existence check on orders",
    ),
    (
        "constituting_or_rectifying_an_apex_record_locally",
        "apex routing block on the write path (APEX_RECORD_IN_SUBSCRIBING_JURISDICTION; control: hook.rs subscribing-repo test)",
    ),
    (
        "re_rendering_a_record_so_it_ceases_to_load",
        "preserve-check refuses an unparseable after-file (standalone s5 proof tool, not in the commit pipeline; control: an_unparseable_after_file_refuses_the_proof)",
    ),
    (
        "publish_this_acts_own_unwired_duties_in_the_conformance_audit",
        "conformance_audit enumerates ACT 12's own duties (control: conformance_ratchet)",
    ),
    (
        "record_every_digest_bump_as_a_deliberate_act",
        "LAWPACK_LOCK_DRIFT forces the deliberate re-pin (control: lawpack_lock falsified-digest test)",
    ),
    (
        "publishing_private_facts",
        "Gazette publication boundary: RedactScanner + denylist whole-token AND segment measures, fail-closed before write (control: gazette_boundary_e2e)",
    ),
    (
        "register_scheduled_machinery_as_separate_gazette_items",
        "gazette MACHINERY kinds structurally diverted to schedules (control: gazette_data every-object test)",
    ),
    (
        "alter_force_or_severity_of_scheduled_records_by_consolidation",
        "schedule entries copy status verbatim and publication is constitutively inert (control: gazette_publication jsonld-mirror test)",
    ),
    (
        "publish_v1_as_archive",
        "Gazette V1-archive estate with migration edges (control: gazette_data v1-node test)",
    ),
    (
        "load_only_law_that_hashes_to_the_pinned_digest_and_fail_closed_otherwise",
        "LAWPACK_LOCK_DRIFT (same gate as the embedded-posture sibling; control: falsified-digest test)",
    ),
    (
        "validate_mcp_input",
        "MCP door refuses malformed input per handler (control: unknown_method_is_refused; the control seeds the method class, not every param path)",
    ),
    (
        "check_citation_collisions",
        "D2 citation gate (CITATION_COLLISION; control: lawpack_resolution uniqueness tests)",
    ),
    (
        "check_lawpack_digest",
        "LAWPACK_LOCK_DRIFT (third token on the one digest gate; control: falsified-digest test)",
    ),
    (
        "refuse_an_order_with_an_empty_issue",
        "staged ORDER_MALFORMED empty-issue limb, landed with this row (control: e2e_gate_harness empty-issue seed)",
    ),
    // s4(e) triage tranche 3 (2026-08-06, four-seat classification sitting over the
    // 182 awaiting-judgment rows; report at
    // .vjs/submissions/COMMISSION-REPORT-2026-08-06-s4e-triage-tranche-3.md): every
    // row re-verified against code path AND control before entry, exactly as
    // tranche 2. Where one enforcement fact carries several tokens (the apex gate,
    // the assent floor, the deny.toml closure ban, the nine-verb MCP surface, the
    // permit gate, preserve-check) the labels say so.
    (
        "legislature_amends_assent_rule",
        "the assent rule in code (front_door VALID_ASSENT_SOURCES) is a pinned enforcement surface; any edit is loud drift (control: check_drift_flags_an_edited_gate + the real-pipeline testkit twin)",
    ),
    (
        "redact_before_publication",
        "RedactScanner + the always-on boundary scan (BOUNDARY_VIOLATION; control: canon_secret_scan_blocks_credentials_but_only_warns_on_hostnames)",
    ),
    (
        "a_fourth_concurrent_warrant",
        "warrant-register gate (WARRANT-CONCURRENCY-EXCEEDED over GOVERNING warrants; control: a_fourth_governing_warrant_exceeds_the_cap)",
    ),
    (
        "prove_a_form_rectification_by_the_stated_test",
        "preserve-check node-tree comparison (standalone s5 proof tool; control: a_key_lost_at_depth_fails_whatever_its_name)",
    ),
    (
        "take_the_proof_over_the_file_as_filed",
        "preserve-check reads raw filed text, never a typed structure (control: the same depth seed catches non-schema keys)",
    ),
    (
        "compare_every_node_at_its_path_including_sequence_items",
        "preserve-check Sequence branch, index-wise (control: a_sequence_that_loses_or_reorders_items_fails + a_root_sequence_is_compared_as_a_node)",
    ),
    (
        "a_proof_taken_over_a_loaded_structure",
        "preserve-check constructs no Statute/Regulation/KernelEffect (control: the depth seed over arbitrary keys)",
    ),
    (
        "kernel_legislates",
        "resolver refuses every kernel write path into the canon tree (control: no_operator_supplied_output_path_can_manufacture_the_canon_tree)",
    ),
    (
        "kernel_creates_force_by_computation",
        "runtime force requires a resolving assent_source; a self-authorised claim is refused (control: rejects_self_authorised)",
    ),
    (
        "legislature_creates_force_from_output",
        "same assent floor: output carries no force without a resolving external assent_source (control: rejects_missing_assent_source)",
    ),
    (
        "kernel_calls_model",
        "deny.toml bans model-SDK crates from the whole workspace closure (control: the_kernel_closure_bans_network_and_model_crates)",
    ),
    (
        "fetch_law_at_runtime",
        "deny.toml bans HTTP-client crates and the loader reads only local paths (control: the_kernel_closure_bans_network_and_model_crates)",
    ),
    (
        "kernel_uses_semantic_similarity_for_authority",
        "authority resolution matches issue tags by exact fold equality only (control: a_tag_no_authority_declares_still_convenes_a_court)",
    ),
    (
        "second_or_parallel_apex",
        "apex routing block (APEX_RECORD_IN_SUBSCRIBING_JURISDICTION; control: subscribing_repo_recording_a_supreme_order_is_blocked)",
    ),
    (
        "apex_singleness_non_relaxable",
        "same apex gate; the sole apex jurisdiction is hard-coded at the call site, not record-variable",
    ),
    (
        "relaxing_apex_singleness",
        "same apex gate (must_not twin of the row above)",
    ),
    (
        "deny_list_assent_form",
        "assent validation is an affirmative allow-list, never a deny-list (control: valid_assent_value_is_the_allow_list_not_merely_non_empty)",
    ),
    (
        "express_constitution_by_binding_order_required",
        "bench convening refuses an unconstituted tier (TierNotConstituted; control: convening_for_an_unconstituted_tier_fails_closed)",
    ),
    (
        "adopt_a_new_digest_by_silence",
        "LAWPACK_LOCK_DRIFT fails closed on any unpinned digest (fourth token on the one digest gate; control: falsified-digest test)",
    ),
    (
        "evaluate_default_invariants",
        "staged_gates runs evaluate_invariants on every staged validate, the commit-gate path (control: predicate_teeth suite); the un-staged bypass is a queued build, not covered here",
    ),
    (
        "expose_release_tool_in_mcp",
        "the MCP surface is a closed nine-verb set with no release verb (control: mcp_surface_exposes_the_nine_verbs)",
    ),
    (
        "expose_direct_file_write_in_mcp",
        "same closed surface; write-capable verbs take typed fields through kernel verification, never path+content (control: the nine-verb test + fixed schemas)",
    ),
    (
        "allow_arbitrary_shell_in_mcp",
        "same closed surface, no shell verb (control: the nine-verb test + unknown_mcp_method_is_refused)",
    ),
    (
        "run_public_private_boundary_scan",
        "local-ci boundary_scan via the pre-push hook (control: canon_secret_scan_blocks_credentials_but_only_warns_on_hostnames)",
    ),
    (
        "reach_the_duty_surface_at_large_or_key_the_limb_on_the_conformance_map_counts",
        "is_governed_record keys on the fixed record roots, never the conformance map (control: front_door_governs_record_kinds_only)",
    ),
    (
        "route_governed_load_bearing_work_for_a_permit_before_acting",
        "PreWrite on a governed path requires route, fail-closed (control: pre_write_on_a_governed_path_requires_route_and_fails_closed)",
    ),
    (
        "perform_governed_work_without_an_active_permit",
        "PermitGate (PERMIT-MISSING; control: a_permitted_governed_write_passes_and_an_unpermitted_one_fails_closed)",
    ),
    (
        "receive_permit",
        "PermitGate refuses a governed write with no matching permit (control: the functional_hook fail-closed pair)",
    ),
    (
        "act_within_permit",
        "PermitGate scope matching (control: a_scoped_permit_covers_only_its_paths + a_no_scope_permit_covers_nothing)",
    ),
    (
        "attach_proof",
        "PermitGate proof obligation (PERMIT-PROOF-MISSING; control: a_proof_obligation_blocks_commit_until_a_proof_exists)",
    ),
    (
        "write_log",
        "PermitGate decision-log obligation (PERMIT-OBLIGATION-MISSING; control: a_decision_log_obligation_blocks_commit_until_a_log_exists)",
    ),
    (
        "load_and_enforce_schedules_directly",
        "the loader reads schedule kinds straight into the lawpack and invariants evaluate (control: no_duplicate_ids_has_teeth)",
    ),
    (
        "read_schedules_within_this_instrument_in_the_gazette",
        "Gazette groups machinery kinds into the instrument's schedules, never separate items (control: every_law_object_is_published_and_every_edge_resolves)",
    ),
    (
        "draft_becomes_binding_by_writing",
        "AuthorityStatus::is_live excludes Draft/Proposed from force (control: draft_law_is_not_binding_has_teeth)",
    ),
    (
        "silently_edit_live_law",
        "lawpack paths are permit_required governance; an un-permitted edit is refused (control: the functional_hook governed-path seed)",
    ),
    (
        "generate_citations_deterministically",
        "the allocator mints max-plus-one over the persisted register, no randomness (control: allocate_reads_the_county_register_outside_the_lawpack)",
    ),
    (
        "load_lawpack_without_manifest",
        "install surface verification (INSTALL_LAWPACK_LOCK_MISSING; control: incomplete_surface_is_caught)",
    ),
];
