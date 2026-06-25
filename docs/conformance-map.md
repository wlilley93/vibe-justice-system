# VJS Conformance Map (PC-13 D11)

Produced THROUGH the kernel by `vjs audit`. Every kernel_effect duty (must / must_not / prohibits) in every in-force statute and regulation, with whether it is bound to a deterministic kernel gate. The UNWIRED list is the factual predicate for the reserved D12 single-front-door instrument.

- total duties: 264
- wired: 22
- unwired: 242

> Triage note: UNWIRED does not mean "must be gated". Many unwired duties are declarative (`defines`-adjacent), one-time/transition acts, or agent-duties that a deterministic gate cannot or need not enforce. The conservative registry marks a duty WIRED only when a named, deterministic gate can be pointed at it, so the map never overstates coverage. D12 triages this list to decide which unwired duties the single-front-door instrument must bite on.

## Unwired duties (the side doors)

| instrument | kind | duty |
|---|---|---|
| ACT-001:s1 | must_not | load_v1_archive_as_runtime_by_default |
| ACT-001:s2 | must | record_principal_assent_for_base_law |
| ACT-001:s3 | must | resolve_authority_in_hierarchy |
| ACT-001:s4 | must_not | load_v1_archive_as_runtime_by_default |
| ACT-001:s4 | must_not | treat_v1_judgments_as_binding_without_incorporation |
| ACT-001:s5 | must_not | kernel_call_llm |
| ACT-001:s5 | must_not | kernel_use_vector_search |
| ACT-001:s5 | must_not | kernel_render_pdf |
| ACT-001:s5 | must_not | kernel_auto_publish |
| ACT-001:s5 | must_not | kernel_replace_human_approval |
| ACT-001:s6 | must | respect_real_world_law |
| ACT-001:s7 | must | mark_proposed_law_as_draft |
| ACT-001:s7 | must | require_authorised_adoption_for_binding_force |
| ACT-001:s7 | must_not | agent_self_authorise_law |
| ACT-001:s7 | must_not | agent_draft_becomes_binding_by_fact_of_being_written |
| ACT-001:s8 | must | evaluate_spec_on_governed_changes |
| ACT-001:s9 | must | principal_assent_for_local_sovereignty_change |
| ACT-CONSOLIDATION-FRAMEWORK:s4 | prohibits | runtime_force_by_restatement_alone |
| ACT-CONSOLIDATION-FRAMEWORK:s7 | prohibits | si_amending_the_act_or_the_assent_rule |
| ACT-CONSOLIDATION-FRAMEWORK:s10 | prohibits | legislature_self_extension |
| ACT-CONSOLIDATION-FRAMEWORK:s10 | prohibits | legislature_amends_assent_rule |
| ACT-CONSOLIDATION-FRAMEWORK:s20 | prohibits | v1_import_by_implication |
| ACT-CONSOLIDATION-FRAMEWORK:s20 | prohibits | lower_rank_incorporates_higher_rank |
| ACT-CONSOLIDATION-FRAMEWORK:s21 | prohibits | weakening_any_protective_floor_limb |
| ACT-CONSOLIDATION-FRAMEWORK:s25 | prohibits | amending_an_entrenched_guarantee_by_si_or_kernel |
| ACT-004:s1 | must | validate_record_schema |
| ACT-004:s2 | must | evaluate_spec_on_governed_changes |
| ACT-004:s3 | must_not | use_llm_to_evaluate_invariant |
| ACT-004:s3 | must_not | use_cosine_for_invariant |
| ACT-004:s3 | must_not | use_free_form_script_for_invariant |
| ACT-004:s4 | must | keep_decisions_short |
| ACT-004:s5 | must_not | act_without_valid_permit |
| ACT-004:s6 | must | attach_required_proofs |
| ACT-004:s7 | must | write_log_for_material_decisions |
| ACT-004:s7 | must_not | delete_logs |
| ACT-004:s9 | must | record_supersession_explicitly |
| ACT-004:s9 | must_not | delete_old_records |
| ACT-004:s10 | must | enforce_word_limits |
| ACT-006:s1 | must | call_vjs_route |
| ACT-006:s1 | must | verify_release_authority |
| ACT-006:s2 | must | verify_release_warrant |
| ACT-006:s3 | must | remove_secrets_before_commit |
| ACT-006:s3 | must | flag_secrets_in_scanner |
| ACT-006:s3 | must_not | commit_secrets_to_public |
| ACT-006:s4 | must | human_approval_required |
| ACT-006:s4 | must | permit_with_human_approval |
| ACT-006:s4 | must_not | proceed_without_human_approval |
| ACT-006:s5 | must | use_explicit_authority_for_external |
| ACT-006:s6 | must | log_release_receipt |
| ACT-005:s1 | must_not | publish_hostnames |
| ACT-005:s1 | must_not | publish_logs |
| ACT-005:s1 | must_not | publish_screenshots |
| ACT-005:s2 | must | route_private_facts_to_local_store |
| ACT-005:s3 | must | redact_before_publication |
| ACT-005:s3 | must | act_on_scanner_flags |
| ACT-005:s4 | must | use_pointers_for_local_evidence |
| ACT-005:s4 | must_not | expose_private_detail_in_pointer |
| ACT-005:s6 | must_not | load_gazette_as_runtime_authority |
| ACT-ASSENTED-RECORD-PROTECTION:s1 | prohibits | subordinate_validation_voiding_or_blocking_a_sovereign_assented_record |
| ACT-ASSENTED-RECORD-PROTECTION:s1 | prohibits | silently_excluding_a_sovereign_assented_record_for_a_defect_rather_than_routing_it_for_correction |
| ACT-ASSENTED-RECORD-PROTECTION:s2 | prohibits | amending_or_disapplying_this_act_other_than_by_a_sovereign_assented_constitutional_act_citing_it_by_number |
| ACT-ASSENTED-RECORD-PROTECTION:s2 | prohibits | reading_this_act_to_lower_the_external_law_floor_or_any_entrenched_provision |
| ACT-002:s1 | must_not | create_court_of_appeal_in_v2_mvp |
| ACT-002:s2 | must | route_repo_local_questions_to_county_court |
| ACT-002:s3 | must | route_jurisdiction_questions_to_privy_council |
| ACT-002:s3 | must | route_routing_questions_to_privy_council |
| ACT-002:s3 | must | route_constitutional_questions_to_privy_council |
| ACT-002:s3 | must | route_boundary_questions_to_privy_council |
| ACT-002:s4 | must | route_foundational_doctrine_to_supreme_court |
| ACT-002:s4 | must_not | use_supreme_court_for_routine_repo_questions |
| ACT-002:s5 | must_not | create_court_of_appeal |
| ACT-002:s6 | must | check_existing_authority_before_court |
| ACT-002:s6 | must_not | convene_court_when_settled |
| ACT-002:s7 | must | apply_order_directives |
| ACT-002:s7 | must_not | treat_opinion_as_runtime_authority |
| ACT-002:s8 | must | enforce_order_word_limits |
| ACT-002:s9 | must | supreme_court_order_for_overruling |
| ACT-002:s10 | must | validate_order_format |
| ACT-002:s10 | must_not | accept_order_without_directives |
| ACT-002:s10 | must_not | accept_order_without_runtime_summary |
| ACT-003:s1 | prohibits | agent_self_adjudication |
| ACT-003:s1 | prohibits | agent_self_authorised_law |
| ACT-003:s2 | must | call_vjs_route |
| ACT-003:s3 | must | act_within_permit_scope |
| ACT-003:s3 | must | attach_required_proof |
| ACT-003:s3 | must_not | act_outside_permit_scope |
| ACT-003:s3 | must_not | close_permit_without_proof |
| ACT-003:s5 | must | self_file_breach |
| ACT-003:s5 | must | correct_the_work |
| ACT-003:s6 | must_not | agent_act_on_capability_alone |
| ACT-003:s7 | must | run_vjs_validate |
| ACT-003:s8 | must_not | add_model_call_to_vjs_core |
| ACT-003:s8 | must_not | add_vector_search_to_vjs_core |
| ACT-003:s9 | must_not | add_network_dependency_to_vjs_core |
| ACT-003:s9 | must_not | add_http_client_to_vjs_core |
| ACT-003:s10 | must_not | comply_by_breaching_binding_law_floor_or_reservation |
| ACT-003:s10 | must_not | fabricate_a_pass_to_silence_the_gate |
| ACT-003:s10 | must_not | wait_for_principal_to_name_the_court |
| ACT-003:s11 | must_not | muzzle_a_gate_firing_on_a_printed_result_contradiction |
| ACT-003:s11 | must_not | self_apply_a_disposition_by_the_benefiting_party |
| ACT-COMPUTER-FIRST-REALM:s6 | prohibits | v1_import_by_implication |
| ACT-COMPUTER-FIRST-REALM:s8 | prohibits | general_reference_treated_as_incorporation |
| ACT-COMPUTER-FIRST-REALM:s9 | prohibits | treating_v1_as_binding_gap_filler |
| ACT-COMPUTER-FIRST-REALM:s10 | prohibits | lower_contradicts_higher |
| ACT-COMPUTER-FIRST-REALM:s10 | prohibits | local_log_amends_law |
| ACT-COMPUTER-FIRST-REALM:s11 | prohibits | kernel_legislates |
| ACT-COMPUTER-FIRST-REALM:s11 | prohibits | kernel_adjudicates_as_court |
| ACT-COMPUTER-FIRST-REALM:s11 | prohibits | kernel_creates_force_by_computation |
| ACT-COMPUTER-FIRST-REALM:s11 | prohibits | kernel_calls_model |
| ACT-COMPUTER-FIRST-REALM:s11 | prohibits | kernel_uses_semantic_similarity_for_authority |
| ACT-COMPUTER-FIRST-REALM:s11 | prohibits | kernel_treats_archive_as_live_without_incorporation |
| ACT-COMPUTER-FIRST-REALM:s14 | prohibits | legislature_self_extension |
| ACT-COMPUTER-FIRST-REALM:s14 | prohibits | legislature_amends_assent_rule |
| ACT-COMPUTER-FIRST-REALM:s14 | prohibits | legislature_creates_force_from_output |
| ACT-COMPUTER-FIRST-REALM:s14 | prohibits | kernel_or_agent_amends_entrenched_gate |
| ACT-COMPUTER-FIRST-REALM:s15 | prohibits | second_or_parallel_apex |
| ACT-COMPUTER-FIRST-REALM:s15 | prohibits | lexby_sits_as_bench |
| ACT-COMPUTER-FIRST-REALM:s15 | prohibits | lexby_judges_own_cause |
| ACT-COMPUTER-FIRST-REALM:s15 | prohibits | kernel_constitutes_or_counts_bench |
| ACT-COMPUTER-FIRST-REALM:s23 | prohibits | deny_list_assent_form |
| ACT-COMPUTER-FIRST-REALM:s23 | prohibits | binding_force_without_traceable_assent_source |
| ACT-COMPUTER-FIRST-REALM:s16 | prohibits | publication_creates_runtime_force |
| ACT-COMPUTER-FIRST-REALM:s16 | prohibits | publication_treated_as_v1_enactment |
| ACT-COMPUTER-FIRST-REALM:s16 | prohibits | publication_treated_as_v1_incorporation |
| ACT-COMPUTER-FIRST-REALM:s29 | must | express_constitution_by_binding_order_required |
| ACT-COMPUTER-FIRST-REALM:s29 | must | odd_bench_required |
| ACT-COMPUTER-FIRST-REALM:s29 | must | apex_singleness_non_relaxable |
| ACT-COMPUTER-FIRST-REALM:s29 | must_not | court_order_before_constitution |
| ACT-COMPUTER-FIRST-REALM:s29 | must_not | even_numbered_bench |
| ACT-COMPUTER-FIRST-REALM:s29 | must_not | relaxing_apex_singleness |
| ACT-007:s2 | must | load_canonical_lawpack_on_route |
| ACT-007:s6 | must | declare_lawpack_lineage |
| ACT-007:s6 | must_not | fork_without_declaring_lineage |
| REG-SELF-CONVENE-001 | must | convene_the_named_court_on_own_motion |
| REG-SELF-CONVENE-001 | must | file_symmetric_case_file_no_preference |
| REG-SELF-CONVENE-001 | must_not | route_the_fork_to_the_principal |
| REG-SELF-CONVENE-001 | must_not | ask_the_principal_to_choose_between_approaches |
| REG-SELF-CONVENE-001 | must_not | proceed_without_a_ruling |
| REG-ACCESSION-001 | must | load_only_law_that_hashes_to_the_pinned_digest_and_fail_closed_otherwise |
| REG-ACCESSION-001 | must | refuse_law_whose_schema_version_exceeds_the_loader_at_load_time |
| REG-ACCESSION-001 | must | record_every_digest_bump_as_a_deliberate_act |
| REG-ACCESSION-001 | must_not | fetch_law_at_runtime |
| REG-ACCESSION-001 | must_not | adopt_a_new_digest_by_silence |
| REG-ACCESSION-001 | must_not | enact_or_purport_to_enact_a_subscribers_supremacy_clause |
| REG-FEDERATION-COORDINATION-001 | prohibits | a_root_asserting_an_apex_or_final_court_function |
| REG-FEDERATION-COORDINATION-001 | prohibits | binding_or_gating_a_peer_local_law_without_adoption |
| REG-FEDERATION-COORDINATION-001 | prohibits | overriding_canonical_without_the_s6_route |
| REG-FEDERATION-COORDINATION-001 | prohibits | foreclosing_a_peer_amend_pin_fork_or_exit |
| REG-KERNEL-001 | must | keep_hooks_thin_and_route_every_check_through_the_kernel |
| REG-KERNEL-001 | must | load_only_law_that_hashes_to_the_pinned_digest_in_the_embedded_posture |
| REG-KERNEL-001 | must | name_the_instrument_that_caused_every_denial |
| REG-KERNEL-001 | must_not | let_the_kernel_deliberate_draft_or_judge_merits |
| REG-KERNEL-001 | must_not | bump_a_pinned_digest_without_the_subscribers_deliberate_assent |
| REG-KERNEL-001 | must_not | place_kernel_checking_logic_inside_a_hook |
| REG-006 | must | evaluate_default_invariants |
| REG-006 | must_not | bypass_invariant_evaluation |
| REG-007 | must | validate_mcp_input |
| REG-007 | must | audit_mcp_calls |
| REG-007 | must_not | expose_release_tool_in_mcp |
| REG-007 | must_not | expose_direct_file_write_in_mcp |
| REG-007 | must_not | allow_arbitrary_shell_in_mcp |
| REG-COURT-RECORD-001 | must | record_the_deciding_bench_on_every_new_order |
| REG-COURT-RECORD-001 | must | pin_the_case_file_digest_before_the_order_issues |
| REG-COURT-RECORD-001 | must_not | alter_bench_sizes_jurisdiction_or_the_assent_rule |
| REG-COURT-RECORD-001 | must_not | invalidate_a_legacy_ruling_for_want_of_the_structured_fields |
| REG-RELEASE-WARRANT-001 | must | match_remote_ref_and_sha |
| REG-RELEASE-WARRANT-001 | must | run_public_private_boundary_scan |
| REG-RELEASE-WARRANT-001 | must | identify_authorising_instrument_for_a_law_changing_push |
| REG-RELEASE-WARRANT-001 | must | privy_council_post_push_review |
| REG-RELEASE-WARRANT-001 | must_not | require_fresh_royal_assent_to_publish_already_assented_law |
| REG-RELEASE-WARRANT-001 | must_not | push_on_a_mismatch_or_a_boundary_scan_hit |
| REG-REPOS-HOUSE-001 | must | record_certification_as_kernel_restatement_not_attestation |
| REG-REPOS-HOUSE-001 | must | route_policy_proposals_through_lawmaking_route |
| REG-REPOS-HOUSE-001 | must | preserve_v1_ministry_names_as_archive_lineage_only |
| REG-REPOS-HOUSE-001 | must | maintain_two_layer_structure |
| REG-REPOS-HOUSE-001 | must_not | repos_house_creates_legal_force |
| REG-REPOS-HOUSE-001 | must_not | human_attestation_replaces_kernel_output_certification |
| REG-REPOS-HOUSE-001 | must_not | v1_ministry_names_have_legal_effect |
| REG-REPOS-HOUSE-001 | must_not | policy_office_outside_lawmaking_route |
| REG-REPOS-HOUSE-001 | must_not | superseding_repos_register_001 |
| REG-DEV-CONDUCT-001 | must | route_governed_load_bearing_work_for_a_permit_before_acting |
| REG-DEV-CONDUCT-001 | must | record_a_decisive_call_for_reversible_low_blast_work |
| REG-DEV-CONDUCT-001 | must | convene_the_named_court_on_a_genuine_fork |
| REG-DEV-CONDUCT-001 | must_not | perform_governed_work_without_an_active_permit |
| REG-DEV-CONDUCT-001 | must_not | route_a_fork_or_a_reversible_call_to_the_principal |
| REG-INVOCATION-001 | must_not | bind_a_repo_by_directory_ancestry |
| REG-INVOCATION-001 | must_not | require_a_ministry_or_v1_v2_tree_layout |
| REG-005 | must | enforce_word_limits |
| REG-005 | must_not | accept_overlong_records |
| REG-GAZETTE-CONTINUITY-001 | prohibits | treating_a_gazette_entry_as_runtime_source |
| REG-GAZETTE-CONTINUITY-001 | prohibits | publishing_private_facts |
| REG-INSTALL-MANIFEST-001 | must | lock_the_install_surface_atomically_at_invoke |
| REG-INSTALL-MANIFEST-001 | must | reverify_the_manifest_at_validate_staged_and_pre_write |
| REG-INSTALL-MANIFEST-001 | must | fail_closed_citing_the_instrument_behind_any_missing_or_stale_limb |
| REG-INSTALL-MANIFEST-001 | must_not | mandate_an_external_signing_key_for_the_install_manifest |
| REG-INSTALL-MANIFEST-001 | must_not | enumerate_more_than_the_reg_invocation_001_surface |
| REG-REALM-INVARIANTS-001 | must | load_and_enforce_schedules_directly |
| REG-REALM-INVARIANTS-001 | must | read_schedules_within_this_instrument_in_the_gazette |
| REG-REALM-INVARIANTS-001 | must_not | register_scheduled_machinery_as_separate_gazette_items |
| REG-REALM-INVARIANTS-001 | must_not | alter_force_or_severity_of_scheduled_records_by_consolidation |
| REG-LAWMAKING-001 | must | start_with_draft_status |
| REG-LAWMAKING-001 | must | add_authority_basis |
| REG-LAWMAKING-001 | must | validate_before_adoption |
| REG-LAWMAKING-001 | must_not | draft_becomes_binding_by_writing |
| REG-LAWMAKING-001 | must_not | silently_edit_live_law |
| REG-TRANSITION-CONTINUITY-001 | prohibits | relitigation_by_reason_only_of_transfer |
| REG-TRANSITION-CONTINUITY-001 | prohibits | reviving_v1_machinery_beyond_the_narrow_perfection_limb |
| REG-003 | must | generate_citations_deterministically |
| REG-003 | must | check_citation_collisions |
| REG-002 | must | resolve_records_by_role_not_path |
| REG-002 | must_not | require_ministry_style_paths |
| REG-001 | must | validate_manifest_on_load |
| REG-001 | must | check_lawpack_digest |
| REG-001 | must_not | load_lawpack_without_manifest |
| REG-CERTIFICATION-MARK-001 | must | issue_the_mark_only_on_a_passing_conformance_run_against_the_pinned_canon_digest |
| REG-CERTIFICATION-MARK-001 | must | lapse_the_mark_on_release_pin_change_revocation_or_divergence |
| REG-CERTIFICATION-MARK-001 | must_not | permit_an_uncertified_or_divergent_pack_to_bear_the_certification_mark |
| REG-CERTIFICATION-MARK-001 | must_not | treat_certification_as_a_human_attestation_rather_than_deterministic_kernel_output |
| REG-MIGRATION-INCORPORATION-001 | must | record_each_crossing_in_the_migration_ledger |
| REG-MIGRATION-INCORPORATION-001 | prohibits | v1_import_by_implication_or_in_bulk |
| REG-MIGRATION-INCORPORATION-001 | prohibits | lower_rank_incorporates_higher_rank |
| REG-MIGRATION-INCORPORATION-001 | prohibits | incorporation_weakening_the_protective_floor |
| REG-008 | must | publish_v1_as_archive |
| REG-008 | must_not | load_v1_as_runtime_without_incorporation |
| REG-HOOKS-001 | must_not | adjudicate_breach |
| REG-HOOKS-001 | must_not | create_law |
| REG-HOOKS-001 | must_not | call_model_for_a_binding_decision |
| REG-HOOKS-001 | must_not | inject_long_context |
| REG-004 | must | call_vjs_route |
| REG-004 | must | receive_permit |
| REG-004 | must | act_within_permit |
| REG-004 | must | attach_proof |
| REG-004 | must | write_log |
| REG-004 | must | run_validate |
| REG-004 | must | close_permit |
| REG-004 | must_not | bypass_lifecycle_step |
| REG-REPOS-REGISTER-001 | must | perform_subscription_within_the_period |
| REG-REPOS-REGISTER-001 | prohibits | treating_the_register_as_a_source_of_law |
| REG-CANONICALISATION-MIGRATION-001 | must | prove_each_fatal_invariant_still_binds_before_and_after |
| REG-CANONICALISATION-MIGRATION-001 | must | record_a_fresh_lock_under_a_commencement_addendum |
| REG-CANONICALISATION-MIGRATION-001 | prohibits | unstaged_mass_edit_or_blind_move |
| REG-CANONICALISATION-MIGRATION-001 | prohibits | retro_editing_any_digest_citation_or_id |

## Wired duties

| instrument | kind | duty | gate |
|---|---|---|---|
| ACT-004:s3 | must | evaluate_invariants_mechanically | invariant evaluator at validate |
| ACT-004:s5 | must | close_permit_with_proof | PermitGate obligations |
| ACT-004:s8 | must | check_citation_uniqueness | D2 citation gate (CITATION_COLLISION) |
| ACT-004:s8 | must_not | allow_duplicate_citations | D2 citation gate (CITATION_COLLISION) |
| ACT-005:s1 | must_not | publish_secrets | RedactScanner (deterministic) |
| ACT-005:s1 | must_not | publish_tokens | RedactScanner (deterministic) |
| ACT-005:s1 | must_not | publish_private_repo_paths | D1 canon-write gate (CANON_BOUNDARY_VIOLATION) |
| ACT-005:s1 | must_not | publish_client_facts | D1 canon-write gate (CANON_BOUNDARY_VIOLATION) |
| ACT-005:s1 | must_not | publish_credentials | RedactScanner (deterministic) |
| ACT-005:s5 | must_not | publish_private_facts_from_contributor_repos | D1 canon-write gate |
| ACT-005:s7 | must | run_boundary_scan_on_public_changes | RedactScanner at validate |
| ACT-005:s7 | must_not | use_llm_for_boundary_check | RedactScanner is deterministic (no LLM) |
| ACT-003:s4 | must | write_decision_log | decision-log obligation gate |
| ACT-007:s1 | must | create_config_toml_on_install | D4/D5 install gate (INSTALL_CONFIG_MISSING) |
| ACT-007:s3 | must_not | local_law_override_canonical_without_authority | D3 cross-repo permit guard (CROSS_REPO_PERMIT) |
| ACT-007:s4 | must_not | local_order_bind_other_repos | D1 canon-write gate (ACT-007:s4) |
| ACT-007:s5 | must_not | publish_private_facts_from_contributors | D1 canon-write gate |
| ACT-007:s7 | must | check_lawpack_lock_consistency | lawpack lock consistency (ACT-007:s7) |
| REG-INVOCATION-001 | must | subscribe_to_a_named_lawpack_and_lock_its_digest | D4/D5 install gate |
| REG-INVOCATION-001 | must | install_enforcement_hooks | D4/D5 install gate (INSTALL_HOOKS_MISSING) |
| REG-INVOCATION-001 | must | record_a_local_sovereign_invocation | D4/D5 install gate (INSTALL_INVOCATION_MISSING) |
| REG-003 | must_not | allow_duplicate_citations | D2 citation gate (CITATION_COLLISION) |
