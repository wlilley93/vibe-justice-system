# The s4(e) triage of the unwired residue - tranche 1 (dated burndown, ACT 11 s12)

Commission work product under WARRANT-OPBOX-001 and WARRANT-CANON-001, 2026-08-05.
A reclassification occurs IN THE AUDIT AND THE REPORT, never in the instrument
(ACT 12 s4(e)). This tranche states the framework, the mechanical first pass, and
the three reclassifications already MEASURED; every 'proposal' below is a proposal
for the Commission's judged tranches, not a finding.

Measured: 399 duties, 104 wired, 295 unwired.

## The three measured reclassifications (from the WS4 registry commit)

- `delete_logs` (ACT-004:s7 must_not): no gate can hold it as specified -
  `.vjs/logs` is not a governed record root (measured 2026-08-05), so the
  destructive-delete surface never sees a log. EITHER widen governed_record_roots
  (then it wires to the existing delete gate) OR reclassify court-enforced.
- word-limit duties (`keep_decisions_short`, `enforce_word_limits`, ACT-004:s4/s10,
  `enforce_order_word_limits` ACT-002:s8): no counting gate exists; gateable in
  principle (a word-count check is deterministic); proposal: WIRE LATER, class A.
- `name_the_instrument_that_caused_every_denial` (REG-KERNEL-001): citations are
  not universal on findings; gateable only after a findings-schema change;
  proposal: class A with a prerequisite noted.

## Mechanical first pass (morphology buckets; judgment pending per row)

- D-awaiting-judgment: 183
- A-gateable-audit-machinery: 54
- B-court-enforced-candidate: 34
- A-gateable-refusal: 21
- C-one-time-transition: 3

## Residue by instrument

| instrument | unwired |
|---|---|
| ACT-RECTIFICATION-COMMISSION | 65 |
| ACT-PROCEEDINGS-DISCIPLINE | 46 |
| ACT-COMPUTER-FIRST-REALM | 29 |
| ACT-002 | 13 |
| ACT-003 | 9 |
| REG-REPOS-HOUSE-001 | 9 |
| ACT-CONSOLIDATION-FRAMEWORK | 8 |
| ACT-001 | 7 |
| REG-004 | 7 |
| REG-FRONT-DOOR-DELIVERY-001 | 7 |
| REG-ACCESSION-001 | 6 |
| REG-RELEASE-WARRANT-001 | 6 |
| ACT-004 | 5 |
| REG-007 | 5 |
| REG-DEV-CONDUCT-001 | 5 |
| REG-LAWMAKING-001 | 5 |
| ACT-005 | 4 |
| REG-REALM-INVARIANTS-001 | 4 |
| REG-CERTIFICATION-MARK-001 | 4 |
| REG-MIGRATION-INCORPORATION-001 | 4 |
| REG-CANONICALISATION-MIGRATION-001 | 4 |
| REG-FEDERATION-COORDINATION-001 | 3 |
| REG-COURT-RECORD-001 | 3 |
| REG-FRONT-DOOR-001 | 3 |
| REG-001 | 3 |
| ACT-006 | 2 |
| ACT-ASSENTED-RECORD-PROTECTION | 2 |
| ACT-007 | 2 |
| REG-SELF-CONVENE-001 | 2 |
| REG-006 | 2 |
| REG-INVOCATION-001 | 2 |
| REG-005 | 2 |
| REG-GAZETTE-CONTINUITY-001 | 2 |
| REG-INSTALL-MANIFEST-001 | 2 |
| REG-TRANSITION-CONTINUITY-001 | 2 |
| REG-003 | 2 |
| REG-002 | 2 |
| REG-008 | 2 |
| REG-HOOKS-001 | 2 |
| REG-REPOS-REGISTER-001 | 2 |
| REG-KERNEL-001 | 1 |

## Appendix: every unwired duty with its mechanical proposal

| instrument | kind | token | proposal |
|---|---|---|---|
| ACT-001:s2 | must | record_principal_assent_for_base_law | B-court-enforced-candidate |
| ACT-001:s4 | must_not | treat_v1_judgments_as_binding_without_incorporation | D-awaiting-judgment |
| ACT-001:s5 | must_not | kernel_render_pdf | D-awaiting-judgment |
| ACT-001:s5 | must_not | kernel_replace_human_approval | D-awaiting-judgment |
| ACT-001:s6 | must | respect_real_world_law | D-awaiting-judgment |
| ACT-001:s8 | must | evaluate_spec_on_governed_changes | D-awaiting-judgment |
| ACT-001:s9 | must | principal_assent_for_local_sovereignty_change | B-court-enforced-candidate |
| ACT-CONSOLIDATION-FRAMEWORK:s4 | prohibits | runtime_force_by_restatement_alone | D-awaiting-judgment |
| ACT-CONSOLIDATION-FRAMEWORK:s7 | prohibits | si_amending_the_act_or_the_assent_rule | D-awaiting-judgment |
| ACT-CONSOLIDATION-FRAMEWORK:s10 | prohibits | legislature_self_extension | D-awaiting-judgment |
| ACT-CONSOLIDATION-FRAMEWORK:s10 | prohibits | legislature_amends_assent_rule | D-awaiting-judgment |
| ACT-CONSOLIDATION-FRAMEWORK:s20 | prohibits | v1_import_by_implication | C-one-time-transition |
| ACT-CONSOLIDATION-FRAMEWORK:s20 | prohibits | lower_rank_incorporates_higher_rank | D-awaiting-judgment |
| ACT-CONSOLIDATION-FRAMEWORK:s21 | prohibits | weakening_any_protective_floor_limb | D-awaiting-judgment |
| ACT-CONSOLIDATION-FRAMEWORK:s25 | prohibits | amending_an_entrenched_guarantee_by_si_or_kernel | D-awaiting-judgment |
| ACT-PROCEEDINGS-DISCIPLINE:s1 | must_not | reading_the_definition_of_reversible_in_this_act_as_altering_act_003_s2_or_act_004_s7 | D-awaiting-judgment |
| ACT-PROCEEDINGS-DISCIPLINE:s1 | must_not | treating_an_enumeration_that_omits_a_store_as_a_measurement_of_the_jurisdiction | A-gateable-audit-machinery |
| ACT-PROCEEDINGS-DISCIPLINE:s2 | must | refuse_a_filing_whose_machinery_claims_entry_is_absent_or_malformed | A-gateable-refusal |
| ACT-PROCEEDINGS-DISCIPLINE:s2 | must | name_each_defective_machinery_claims_entry | D-awaiting-judgment |
| ACT-PROCEEDINGS-DISCIPLINE:s2 | must | verify_a_quote_against_its_pinned_blob_where_given | A-gateable-refusal |
| ACT-PROCEEDINGS-DISCIPLINE:s3 | must | refuse_a_second_live_order_on_one_normalised_issue_tag_without_a_declared_relation | A-gateable-refusal |
| ACT-PROCEEDINGS-DISCIPLINE:s3 | must | refuse_an_order_with_an_empty_issue | A-gateable-refusal |
| ACT-PROCEEDINGS-DISCIPLINE:s4 | must | cite_this_section_in_addition_to_the_authorities_pc17_d1_named | D-awaiting-judgment |
| ACT-PROCEEDINGS-DISCIPLINE:s4 | must | count_unrecognised_citation_forms_as_ungrounded_in_the_self_test | D-awaiting-judgment |
| ACT-PROCEEDINGS-DISCIPLINE:s5 | must | permit_a_low_risk_reversible_non_boundary_matter_to_resolve_by_decision_and_log | D-awaiting-judgment |
| ACT-PROCEEDINGS-DISCIPLINE:s5 | must | record_the_declarant_and_the_facts_relied_on | A-gateable-audit-machinery |
| ACT-PROCEEDINGS-DISCIPLINE:s6 | must | record_an_amendment_on_the_face_with_prior_text_authority_and_date | A-gateable-audit-machinery |
| ACT-PROCEEDINGS-DISCIPLINE:s6 | must | hold_a_slip_amendment_to_the_fixed_content_preservation_test | D-awaiting-judgment |
| ACT-PROCEEDINGS-DISCIPLINE:s6 | must | preserve_routability_of_prior_text_and_superseded_instruments | D-awaiting-judgment |
| ACT-PROCEEDINGS-DISCIPLINE:s6 | must | require_a_concordance_table_on_a_consolidation | D-awaiting-judgment |
| ACT-PROCEEDINGS-DISCIPLINE:s6 | must_not | amending_the_substance_of_an_assented_order_without_fresh_assent | D-awaiting-judgment |
| ACT-PROCEEDINGS-DISCIPLINE:s6 | must_not | a_locally_constituted_apex_amending_or_consolidating_an_apex_record | A-gateable-audit-machinery |
| ACT-PROCEEDINGS-DISCIPLINE:s7 | must | mint_ordinals_over_the_store_register_only | A-gateable-audit-machinery |
| ACT-PROCEEDINGS-DISCIPLINE:s7 | must | refuse_a_self_asserted_ordinal | A-gateable-refusal |
| ACT-PROCEEDINGS-DISCIPLINE:s7 | must | refuse_reissue_of_a_tombstoned_citation | A-gateable-refusal |
| ACT-PROCEEDINGS-DISCIPLINE:s7 | must | require_a_forwarding_record_in_the_same_act_as_a_citation_amendment | A-gateable-audit-machinery |
| ACT-PROCEEDINGS-DISCIPLINE:s7 | must | enumerate_citing_documents_on_a_citation_amendment | A-gateable-audit-machinery |
| ACT-PROCEEDINGS-DISCIPLINE:s7 | must_not | a_forwarding_record_targeting_an_unallocated_or_forwarded_citation | A-gateable-audit-machinery |
| ACT-PROCEEDINGS-DISCIPLINE:s8 | must | enumerate_stores_query_form_and_per_store_record_counts_against_the_register | A-gateable-audit-machinery |
| ACT-PROCEEDINGS-DISCIPLINE:s8 | must | report_an_omitted_registered_store_as_incomplete_search | A-gateable-audit-machinery |
| ACT-PROCEEDINGS-DISCIPLINE:s9 | must | carry_the_unreadable_count_in_payload_and_local_ci | D-awaiting-judgment |
| ACT-PROCEEDINGS-DISCIPLINE:s9 | must | treat_a_count_increase_as_a_regression | D-awaiting-judgment |
| ACT-PROCEEDINGS-DISCIPLINE:s10 | must | refuse_a_new_directive_with_an_empty_or_denylisted_or_unregistered_actor | A-gateable-audit-machinery |
| ACT-PROCEEDINGS-DISCIPLINE:s10 | must | report_and_count_unstated_actors_on_legacy_directives | A-gateable-audit-machinery |
| ACT-PROCEEDINGS-DISCIPLINE:s10 | must_not | the_reader_supplying_an_actor_for_a_directive_that_names_none | D-awaiting-judgment |
| ACT-PROCEEDINGS-DISCIPLINE:s11 | must | require_a_review_date_on_a_new_reservation | D-awaiting-judgment |
| ACT-PROCEEDINGS-DISCIPLINE:s11 | must | report_an_expired_reservation_as_owed_in_local_ci_without_blocking | A-gateable-audit-machinery |
| ACT-PROCEEDINGS-DISCIPLINE:s12 | must | ship_a_negative_control_with_every_claimed_gate_binding | A-gateable-refusal |
| ACT-PROCEEDINGS-DISCIPLINE:s12 | must | assert_the_specific_finding_and_ship_the_positive_twin | D-awaiting-judgment |
| ACT-PROCEEDINGS-DISCIPLINE:s12 | must | publish_the_three_state_audit_as_a_dated_burndown | A-gateable-audit-machinery |
| ACT-PROCEEDINGS-DISCIPLINE:s12 | must_not | reporting_a_duty_as_enforced_on_a_gate_with_no_negative_control | A-gateable-audit-machinery |
| ACT-PROCEEDINGS-DISCIPLINE:s13 | must | equal_governed_record_roots_to_the_register | A-gateable-audit-machinery |
| ACT-PROCEEDINGS-DISCIPLINE:s13 | must | report_an_unregistered_law_store_in_local_ci | A-gateable-audit-machinery |
| ACT-PROCEEDINGS-DISCIPLINE:s13 | must | register_justice_in_the_opbox_jurisdiction_at_commencement | A-gateable-audit-machinery |
| ACT-PROCEEDINGS-DISCIPLINE:s14 | must | commence_a_part_2_section_only_when_its_gate_and_control_are_in_force | A-gateable-refusal |
| ACT-PROCEEDINGS-DISCIPLINE:s15 | must | measure_mirror_identity_store_by_store_naming_the_stores_compared | A-gateable-audit-machinery |
| ACT-PROCEEDINGS-DISCIPLINE:s15 | must_not | local_amendment_of_this_act_outside_the_preserved_routes | D-awaiting-judgment |
| ACT-PROCEEDINGS-DISCIPLINE:s15 | must_not | reading_this_act_as_disturbing_any_floor_or_pc17_d1 | D-awaiting-judgment |
| ACT-PROCEEDINGS-DISCIPLINE:s16 | must | ground_over_every_registered_form | A-gateable-audit-machinery |
| ACT-PROCEEDINGS-DISCIPLINE:s16 | must | report_tombstoned_with_its_ground | A-gateable-audit-machinery |
| ACT-PROCEEDINGS-DISCIPLINE:s16 | must | reserve_dec_15_to_22_pending_the_footing_matter | D-awaiting-judgment |
| ACT-004:s2 | must | evaluate_spec_on_governed_changes | D-awaiting-judgment |
| ACT-004:s4 | must | keep_decisions_short | D-awaiting-judgment |
| ACT-004:s7 | must_not | delete_logs | D-awaiting-judgment |
| ACT-004:s9 | must | record_supersession_explicitly | A-gateable-audit-machinery |
| ACT-004:s10 | must | enforce_word_limits | D-awaiting-judgment |
| ACT-006:s5 | must | use_explicit_authority_for_external | D-awaiting-judgment |
| ACT-006:s6 | must | log_release_receipt | D-awaiting-judgment |
| ACT-005:s2 | must | route_private_facts_to_local_store | D-awaiting-judgment |
| ACT-005:s3 | must | redact_before_publication | D-awaiting-judgment |
| ACT-005:s4 | must | use_pointers_for_local_evidence | D-awaiting-judgment |
| ACT-005:s4 | must_not | expose_private_detail_in_pointer | D-awaiting-judgment |
| ACT-ASSENTED-RECORD-PROTECTION:s2 | prohibits | amending_or_disapplying_this_act_other_than_by_a_sovereign_assented_constitutional_act_citing_it_by_number | B-court-enforced-candidate |
| ACT-ASSENTED-RECORD-PROTECTION:s2 | prohibits | reading_this_act_to_lower_the_external_law_floor_or_any_entrenched_provision | D-awaiting-judgment |
| ACT-002:s1 | must_not | create_court_of_appeal_in_v2_mvp | B-court-enforced-candidate |
| ACT-002:s2 | must | route_repo_local_questions_to_county_court | B-court-enforced-candidate |
| ACT-002:s3 | must | route_jurisdiction_questions_to_privy_council | D-awaiting-judgment |
| ACT-002:s3 | must | route_routing_questions_to_privy_council | D-awaiting-judgment |
| ACT-002:s3 | must | route_constitutional_questions_to_privy_council | D-awaiting-judgment |
| ACT-002:s3 | must | route_boundary_questions_to_privy_council | D-awaiting-judgment |
| ACT-002:s4 | must | route_foundational_doctrine_to_supreme_court | B-court-enforced-candidate |
| ACT-002:s4 | must_not | use_supreme_court_for_routine_repo_questions | B-court-enforced-candidate |
| ACT-002:s5 | must_not | create_court_of_appeal | B-court-enforced-candidate |
| ACT-002:s7 | must | apply_order_directives | D-awaiting-judgment |
| ACT-002:s7 | must_not | treat_opinion_as_runtime_authority | D-awaiting-judgment |
| ACT-002:s8 | must | enforce_order_word_limits | D-awaiting-judgment |
| ACT-002:s9 | must | supreme_court_order_for_overruling | B-court-enforced-candidate |
| ACT-RECTIFICATION-COMMISSION:s1 | must | void_a_commission_act_that_reaches_a_floor | A-gateable-refusal |
| ACT-RECTIFICATION-COMMISSION:s1 | must | lapse_the_warrant_on_a_floor_breach | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s1 | must_not | crossing_the_sovereign_assent_floor | B-court-enforced-candidate |
| ACT-RECTIFICATION-COMMISSION:s1 | must_not | derogating_from_any_limb_of_the_protective_floor | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s1 | must_not | constituting_or_rectifying_an_apex_record_locally | A-gateable-audit-machinery |
| ACT-RECTIFICATION-COMMISSION:s2 | must | name_exactly_one_jurisdiction_in_a_warrant | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s2 | must | state_an_expiry_date_in_a_warrant | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s2 | must | record_the_warrant_in_the_named_jurisdiction | A-gateable-audit-machinery |
| ACT-RECTIFICATION-COMMISSION:s2 | must | record_a_warrant_in_canon_as_well_as_in_the_named_jurisdiction | A-gateable-audit-machinery |
| ACT-RECTIFICATION-COMMISSION:s2 | must_not | a_commission_acting_outside_the_jurisdiction_named_in_its_warrant | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s2 | must_not | citing_a_commission_act_as_authority | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s2 | must_not | a_fourth_concurrent_warrant | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s3 | must | enumerate_every_store_in_the_named_jurisdiction | A-gateable-audit-machinery |
| ACT-RECTIFICATION-COMMISSION:s3 | must | record_an_address_and_a_mode_on_every_finding | A-gateable-audit-machinery |
| ACT-RECTIFICATION-COMMISSION:s4 | must | confine_a_commission_to_the_exhaustive_power_list | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s4 | must | record_a_tombstoned_ordinal_with_its_reason | A-gateable-audit-machinery |
| ACT-RECTIFICATION-COMMISSION:s4 | must | widen_the_reader_rather_than_edit_the_record | A-gateable-audit-machinery |
| ACT-RECTIFICATION-COMMISSION:s4 | must_not | a_commission_altering_the_operative_terms_of_an_order | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s4 | must_not | a_commission_deciding_a_reserved_question | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s4 | must_not | a_commission_touching_a_verb_capability_permission_model_or_auth_tier | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s4 | must_not | a_commission_enacting_amending_or_reading_down_an_instrument | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s4 | must_not | a_commission_overruling_anything | B-court-enforced-candidate |
| ACT-RECTIFICATION-COMMISSION:s4 | must_not | reissuing_a_tombstoned_ordinal | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s5 | must | prove_a_form_rectification_by_the_stated_test | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s5 | must | record_the_test_and_its_result | A-gateable-audit-machinery |
| ACT-RECTIFICATION-COMMISSION:s5 | must | take_the_proof_over_the_file_as_filed | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s5 | must | compare_every_node_at_its_path_including_sequence_items | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s5 | must_not | substituting_another_content_preservation_test | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s5 | must_not | a_proof_taken_over_a_loaded_structure | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s5 | must_not | re_rendering_a_record_so_it_ceases_to_load | A-gateable-audit-machinery |
| ACT-RECTIFICATION-COMMISSION:s6 | must | convene_a_deferred_matter_on_expiry_without_fresh_application | B-court-enforced-candidate |
| ACT-RECTIFICATION-COMMISSION:s6 | must | toll_every_reservation_and_review_date_for_the_stay | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s6 | must | determine_an_application_to_lift_or_narrow_a_stay_before_the_matter_it_concerns_is_entered_on_a_register | A-gateable-audit-machinery |
| ACT-RECTIFICATION-COMMISSION:s6 | must | enter_and_certify_the_rectification_schedule_before_a_stay_attaches | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s6 | must | confine_a_stay_to_the_certified_rectification_schedule | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s6 | must_not | staying_a_discovered_breach | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s6 | must_not | staying_an_unsatisfiable_enforcement_gate | A-gateable-refusal |
| ACT-RECTIFICATION-COMMISSION:s6 | must_not | staying_a_trust_boundary_fork | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s6 | must_not | staying_a_matter_engaging_a_floor | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s6 | must_not | a_warrant_purporting_to_narrow_a_carve_out | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s6 | must_not | staying_an_application_to_lift_or_narrow_a_stay_or_a_challenge_to_a_warrant | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s6 | must_not | staying_a_matter_outside_the_certified_rectification_schedule | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s6 | must_not | certifying_a_rectification_schedule_naming_a_corpus_as_a_whole | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s7 | must | lift_the_stay_automatically_on_expiry | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s7 | must | void_the_acts_of_a_dissolved_commission | A-gateable-refusal |
| ACT-RECTIFICATION-COMMISSION:s7 | must | require_a_fresh_warrant_to_extend | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s7 | must_not | extending_a_stay_by_inaction_or_an_unfinished_sweep | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s7 | must_not | a_warrant_exceeding_ninety_days | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s7 | must_not | a_second_extension_without_a_delivered_report | A-gateable-audit-machinery |
| ACT-RECTIFICATION-COMMISSION:s8 | must | deliver_a_closing_report_before_expiry | A-gateable-audit-machinery |
| ACT-RECTIFICATION-COMMISSION:s8 | must | report_every_finding_not_rectified_with_its_reason | A-gateable-audit-machinery |
| ACT-RECTIFICATION-COMMISSION:s8 | must | name_every_store_not_reached | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s8 | must | name_the_build_and_commit_behind_every_count | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s8 | must | report_the_audited_duty_count_before_and_after_every_reclassification | A-gateable-audit-machinery |
| ACT-RECTIFICATION-COMMISSION:s9 | must | keep_the_prior_text_routable_until_the_appeal_period_closes | B-court-enforced-candidate |
| ACT-RECTIFICATION-COMMISSION:s9 | must | refer_a_substantive_finding_rather_than_act_on_it | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s9 | must_not | staying_an_appeal_from_a_commission_act | B-court-enforced-candidate |
| ACT-RECTIFICATION-COMMISSION:s9 | must_not | treating_a_commission_act_as_precedent | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s9 | must_not | refusing_a_warrant_challenge_on_the_stated_grounds | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s10 | must | commence_a_constraint_before_the_power_it_bounds | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s10 | must | publish_this_acts_own_unwired_duties_in_the_conformance_audit | A-gateable-audit-machinery |
| ACT-RECTIFICATION-COMMISSION:s10 | must_not | reporting_a_section_of_this_act_as_enforced_before_its_gate_commences | A-gateable-audit-machinery |
| ACT-RECTIFICATION-COMMISSION:s11 | must | declare_every_variation_this_act_makes_with_its_section_and_extent | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s12 | must | state_the_method_and_the_bound_of_every_enumeration_in_this_schedule | D-awaiting-judgment |
| ACT-RECTIFICATION-COMMISSION:s12 | must | name_every_store_this_schedule_does_not_reach | D-awaiting-judgment |
| ACT-003:s1 | prohibits | agent_self_adjudication | B-court-enforced-candidate |
| ACT-003:s5 | must | self_file_breach | B-court-enforced-candidate |
| ACT-003:s5 | must | correct_the_work | D-awaiting-judgment |
| ACT-003:s6 | must_not | agent_act_on_capability_alone | B-court-enforced-candidate |
| ACT-003:s10 | must_not | comply_by_breaching_binding_law_floor_or_reservation | D-awaiting-judgment |
| ACT-003:s10 | must_not | fabricate_a_pass_to_silence_the_gate | A-gateable-refusal |
| ACT-003:s10 | must_not | wait_for_principal_to_name_the_court | B-court-enforced-candidate |
| ACT-003:s11 | must_not | muzzle_a_gate_firing_on_a_printed_result_contradiction | A-gateable-refusal |
| ACT-003:s11 | must_not | self_apply_a_disposition_by_the_benefiting_party | D-awaiting-judgment |
| ACT-COMPUTER-FIRST-REALM:s6 | prohibits | v1_import_by_implication | C-one-time-transition |
| ACT-COMPUTER-FIRST-REALM:s8 | prohibits | general_reference_treated_as_incorporation | D-awaiting-judgment |
| ACT-COMPUTER-FIRST-REALM:s9 | prohibits | treating_v1_as_binding_gap_filler | D-awaiting-judgment |
| ACT-COMPUTER-FIRST-REALM:s10 | prohibits | lower_contradicts_higher | D-awaiting-judgment |
| ACT-COMPUTER-FIRST-REALM:s10 | prohibits | local_log_amends_law | D-awaiting-judgment |
| ACT-COMPUTER-FIRST-REALM:s11 | prohibits | kernel_legislates | D-awaiting-judgment |
| ACT-COMPUTER-FIRST-REALM:s11 | prohibits | kernel_adjudicates_as_court | B-court-enforced-candidate |
| ACT-COMPUTER-FIRST-REALM:s11 | prohibits | kernel_creates_force_by_computation | D-awaiting-judgment |
| ACT-COMPUTER-FIRST-REALM:s11 | prohibits | kernel_calls_model | D-awaiting-judgment |
| ACT-COMPUTER-FIRST-REALM:s11 | prohibits | kernel_uses_semantic_similarity_for_authority | D-awaiting-judgment |
| ACT-COMPUTER-FIRST-REALM:s11 | prohibits | kernel_treats_archive_as_live_without_incorporation | D-awaiting-judgment |
| ACT-COMPUTER-FIRST-REALM:s14 | prohibits | legislature_self_extension | D-awaiting-judgment |
| ACT-COMPUTER-FIRST-REALM:s14 | prohibits | legislature_amends_assent_rule | D-awaiting-judgment |
| ACT-COMPUTER-FIRST-REALM:s14 | prohibits | legislature_creates_force_from_output | D-awaiting-judgment |
| ACT-COMPUTER-FIRST-REALM:s14 | prohibits | kernel_or_agent_amends_entrenched_gate | B-court-enforced-candidate |
| ACT-COMPUTER-FIRST-REALM:s15 | prohibits | second_or_parallel_apex | D-awaiting-judgment |
| ACT-COMPUTER-FIRST-REALM:s15 | prohibits | lexby_sits_as_bench | B-court-enforced-candidate |
| ACT-COMPUTER-FIRST-REALM:s15 | prohibits | lexby_judges_own_cause | B-court-enforced-candidate |
| ACT-COMPUTER-FIRST-REALM:s15 | prohibits | kernel_constitutes_or_counts_bench | B-court-enforced-candidate |
| ACT-COMPUTER-FIRST-REALM:s23 | prohibits | deny_list_assent_form | D-awaiting-judgment |
| ACT-COMPUTER-FIRST-REALM:s16 | prohibits | publication_creates_runtime_force | D-awaiting-judgment |
| ACT-COMPUTER-FIRST-REALM:s16 | prohibits | publication_treated_as_v1_enactment | D-awaiting-judgment |
| ACT-COMPUTER-FIRST-REALM:s16 | prohibits | publication_treated_as_v1_incorporation | D-awaiting-judgment |
| ACT-COMPUTER-FIRST-REALM:s29 | must | express_constitution_by_binding_order_required | D-awaiting-judgment |
| ACT-COMPUTER-FIRST-REALM:s29 | must | odd_bench_required | B-court-enforced-candidate |
| ACT-COMPUTER-FIRST-REALM:s29 | must | apex_singleness_non_relaxable | D-awaiting-judgment |
| ACT-COMPUTER-FIRST-REALM:s29 | must_not | court_order_before_constitution | B-court-enforced-candidate |
| ACT-COMPUTER-FIRST-REALM:s29 | must_not | even_numbered_bench | B-court-enforced-candidate |
| ACT-COMPUTER-FIRST-REALM:s29 | must_not | relaxing_apex_singleness | D-awaiting-judgment |
| ACT-007:s6 | must | declare_lawpack_lineage | D-awaiting-judgment |
| ACT-007:s6 | must_not | fork_without_declaring_lineage | D-awaiting-judgment |
| REG-SELF-CONVENE-001 | must | file_symmetric_case_file_no_preference | D-awaiting-judgment |
| REG-SELF-CONVENE-001 | must_not | ask_the_principal_to_choose_between_approaches | B-court-enforced-candidate |
| REG-ACCESSION-001 | must | load_only_law_that_hashes_to_the_pinned_digest_and_fail_closed_otherwise | A-gateable-refusal |
| REG-ACCESSION-001 | must | refuse_law_whose_schema_version_exceeds_the_loader_at_load_time | A-gateable-refusal |
| REG-ACCESSION-001 | must | record_every_digest_bump_as_a_deliberate_act | A-gateable-audit-machinery |
| REG-ACCESSION-001 | must_not | fetch_law_at_runtime | D-awaiting-judgment |
| REG-ACCESSION-001 | must_not | adopt_a_new_digest_by_silence | D-awaiting-judgment |
| REG-ACCESSION-001 | must_not | enact_or_purport_to_enact_a_subscribers_supremacy_clause | D-awaiting-judgment |
| REG-FEDERATION-COORDINATION-001 | prohibits | binding_or_gating_a_peer_local_law_without_adoption | D-awaiting-judgment |
| REG-FEDERATION-COORDINATION-001 | prohibits | overriding_canonical_without_the_s6_route | D-awaiting-judgment |
| REG-FEDERATION-COORDINATION-001 | prohibits | foreclosing_a_peer_amend_pin_fork_or_exit | D-awaiting-judgment |
| REG-KERNEL-001 | must | name_the_instrument_that_caused_every_denial | D-awaiting-judgment |
| REG-006 | must | evaluate_default_invariants | D-awaiting-judgment |
| REG-006 | must_not | bypass_invariant_evaluation | D-awaiting-judgment |
| REG-007 | must | validate_mcp_input | A-gateable-refusal |
| REG-007 | must | audit_mcp_calls | A-gateable-audit-machinery |
| REG-007 | must_not | expose_release_tool_in_mcp | D-awaiting-judgment |
| REG-007 | must_not | expose_direct_file_write_in_mcp | D-awaiting-judgment |
| REG-007 | must_not | allow_arbitrary_shell_in_mcp | D-awaiting-judgment |
| REG-COURT-RECORD-001 | must | pin_the_case_file_digest_before_the_order_issues | D-awaiting-judgment |
| REG-COURT-RECORD-001 | must_not | alter_bench_sizes_jurisdiction_or_the_assent_rule | B-court-enforced-candidate |
| REG-COURT-RECORD-001 | must_not | invalidate_a_legacy_ruling_for_want_of_the_structured_fields | B-court-enforced-candidate |
| REG-RELEASE-WARRANT-001 | must | match_remote_ref_and_sha | D-awaiting-judgment |
| REG-RELEASE-WARRANT-001 | must | run_public_private_boundary_scan | D-awaiting-judgment |
| REG-RELEASE-WARRANT-001 | must | identify_authorising_instrument_for_a_law_changing_push | D-awaiting-judgment |
| REG-RELEASE-WARRANT-001 | must | privy_council_post_push_review | D-awaiting-judgment |
| REG-RELEASE-WARRANT-001 | must_not | require_fresh_royal_assent_to_publish_already_assented_law | A-gateable-audit-machinery |
| REG-RELEASE-WARRANT-001 | must_not | push_on_a_mismatch_or_a_boundary_scan_hit | D-awaiting-judgment |
| REG-FRONT-DOOR-001 | must_not | reach_the_duty_surface_at_large_or_key_the_limb_on_the_conformance_map_counts | D-awaiting-judgment |
| REG-FRONT-DOOR-001 | must_not | rest_any_enforcement_guarantee_on_mcp_exclusivity_or_treat_mcp_use_as_proof_of_conformance | D-awaiting-judgment |
| REG-FRONT-DOOR-001 | must_not | enact_any_new_substantive_duty_court_tier_jurisdiction_bench_size_or_assent_form | B-court-enforced-candidate |
| REG-REPOS-HOUSE-001 | must | record_certification_as_kernel_restatement_not_attestation | A-gateable-audit-machinery |
| REG-REPOS-HOUSE-001 | must | route_policy_proposals_through_lawmaking_route | D-awaiting-judgment |
| REG-REPOS-HOUSE-001 | must | preserve_v1_ministry_names_as_archive_lineage_only | D-awaiting-judgment |
| REG-REPOS-HOUSE-001 | must | maintain_two_layer_structure | D-awaiting-judgment |
| REG-REPOS-HOUSE-001 | must_not | repos_house_creates_legal_force | D-awaiting-judgment |
| REG-REPOS-HOUSE-001 | must_not | human_attestation_replaces_kernel_output_certification | D-awaiting-judgment |
| REG-REPOS-HOUSE-001 | must_not | v1_ministry_names_have_legal_effect | D-awaiting-judgment |
| REG-REPOS-HOUSE-001 | must_not | policy_office_outside_lawmaking_route | D-awaiting-judgment |
| REG-REPOS-HOUSE-001 | must_not | superseding_repos_register_001 | A-gateable-audit-machinery |
| REG-DEV-CONDUCT-001 | must | route_governed_load_bearing_work_for_a_permit_before_acting | D-awaiting-judgment |
| REG-DEV-CONDUCT-001 | must | record_a_decisive_call_for_reversible_low_blast_work | A-gateable-audit-machinery |
| REG-DEV-CONDUCT-001 | must | convene_the_named_court_on_a_genuine_fork | B-court-enforced-candidate |
| REG-DEV-CONDUCT-001 | must_not | perform_governed_work_without_an_active_permit | D-awaiting-judgment |
| REG-DEV-CONDUCT-001 | must_not | route_a_fork_or_a_reversible_call_to_the_principal | B-court-enforced-candidate |
| REG-INVOCATION-001 | must_not | bind_a_repo_by_directory_ancestry | D-awaiting-judgment |
| REG-INVOCATION-001 | must_not | require_a_ministry_or_v1_v2_tree_layout | D-awaiting-judgment |
| REG-005 | must | enforce_word_limits | D-awaiting-judgment |
| REG-005 | must_not | accept_overlong_records | A-gateable-audit-machinery |
| REG-GAZETTE-CONTINUITY-001 | prohibits | treating_a_gazette_entry_as_runtime_source | D-awaiting-judgment |
| REG-GAZETTE-CONTINUITY-001 | prohibits | publishing_private_facts | A-gateable-audit-machinery |
| REG-INSTALL-MANIFEST-001 | must_not | mandate_an_external_signing_key_for_the_install_manifest | D-awaiting-judgment |
| REG-INSTALL-MANIFEST-001 | must_not | enumerate_more_than_the_reg_invocation_001_surface | A-gateable-audit-machinery |
| REG-REALM-INVARIANTS-001 | must | load_and_enforce_schedules_directly | D-awaiting-judgment |
| REG-REALM-INVARIANTS-001 | must | read_schedules_within_this_instrument_in_the_gazette | D-awaiting-judgment |
| REG-REALM-INVARIANTS-001 | must_not | register_scheduled_machinery_as_separate_gazette_items | A-gateable-audit-machinery |
| REG-REALM-INVARIANTS-001 | must_not | alter_force_or_severity_of_scheduled_records_by_consolidation | A-gateable-audit-machinery |
| REG-LAWMAKING-001 | must | start_with_draft_status | D-awaiting-judgment |
| REG-LAWMAKING-001 | must | add_authority_basis | D-awaiting-judgment |
| REG-LAWMAKING-001 | must | validate_before_adoption | A-gateable-refusal |
| REG-LAWMAKING-001 | must_not | draft_becomes_binding_by_writing | D-awaiting-judgment |
| REG-LAWMAKING-001 | must_not | silently_edit_live_law | D-awaiting-judgment |
| REG-TRANSITION-CONTINUITY-001 | prohibits | relitigation_by_reason_only_of_transfer | D-awaiting-judgment |
| REG-TRANSITION-CONTINUITY-001 | prohibits | reviving_v1_machinery_beyond_the_narrow_perfection_limb | D-awaiting-judgment |
| REG-003 | must | generate_citations_deterministically | D-awaiting-judgment |
| REG-003 | must | check_citation_collisions | A-gateable-refusal |
| REG-002 | must | resolve_records_by_role_not_path | A-gateable-audit-machinery |
| REG-002 | must_not | require_ministry_style_paths | D-awaiting-judgment |
| REG-001 | must | validate_manifest_on_load | A-gateable-refusal |
| REG-001 | must | check_lawpack_digest | A-gateable-refusal |
| REG-001 | must_not | load_lawpack_without_manifest | D-awaiting-judgment |
| REG-CERTIFICATION-MARK-001 | must | issue_the_mark_only_on_a_passing_conformance_run_against_the_pinned_canon_digest | D-awaiting-judgment |
| REG-CERTIFICATION-MARK-001 | must | lapse_the_mark_on_release_pin_change_revocation_or_divergence | D-awaiting-judgment |
| REG-CERTIFICATION-MARK-001 | must_not | permit_an_uncertified_or_divergent_pack_to_bear_the_certification_mark | D-awaiting-judgment |
| REG-CERTIFICATION-MARK-001 | must_not | treat_certification_as_a_human_attestation_rather_than_deterministic_kernel_output | D-awaiting-judgment |
| REG-MIGRATION-INCORPORATION-001 | must | record_each_crossing_in_the_migration_ledger | A-gateable-audit-machinery |
| REG-MIGRATION-INCORPORATION-001 | prohibits | v1_import_by_implication_or_in_bulk | C-one-time-transition |
| REG-MIGRATION-INCORPORATION-001 | prohibits | lower_rank_incorporates_higher_rank | D-awaiting-judgment |
| REG-MIGRATION-INCORPORATION-001 | prohibits | incorporation_weakening_the_protective_floor | D-awaiting-judgment |
| REG-008 | must | publish_v1_as_archive | A-gateable-audit-machinery |
| REG-008 | must_not | load_v1_as_runtime_without_incorporation | D-awaiting-judgment |
| REG-HOOKS-001 | must_not | adjudicate_breach | D-awaiting-judgment |
| REG-HOOKS-001 | must_not | create_law | D-awaiting-judgment |
| REG-004 | must | receive_permit | D-awaiting-judgment |
| REG-004 | must | act_within_permit | D-awaiting-judgment |
| REG-004 | must | attach_proof | D-awaiting-judgment |
| REG-004 | must | write_log | D-awaiting-judgment |
| REG-004 | must | run_validate | A-gateable-refusal |
| REG-004 | must | close_permit | D-awaiting-judgment |
| REG-004 | must_not | bypass_lifecycle_step | D-awaiting-judgment |
| REG-REPOS-REGISTER-001 | must | perform_subscription_within_the_period | D-awaiting-judgment |
| REG-REPOS-REGISTER-001 | prohibits | treating_the_register_as_a_source_of_law | A-gateable-audit-machinery |
| REG-CANONICALISATION-MIGRATION-001 | must | prove_each_fatal_invariant_still_binds_before_and_after | D-awaiting-judgment |
| REG-CANONICALISATION-MIGRATION-001 | must | record_a_fresh_lock_under_a_commencement_addendum | A-gateable-audit-machinery |
| REG-CANONICALISATION-MIGRATION-001 | prohibits | unstaged_mass_edit_or_blind_move | D-awaiting-judgment |
| REG-CANONICALISATION-MIGRATION-001 | prohibits | retro_editing_any_digest_citation_or_id | D-awaiting-judgment |
| REG-FRONT-DOOR-DELIVERY-001 | must | deliver_the_server_of_law_as_a_container_and_the_wall_as_a_host_hook | D-awaiting-judgment |
| REG-FRONT-DOOR-DELIVERY-001 | must | keep_the_host_commit_hook_the_sole_enforcement_guarantee_never_contingent_on_the_container | D-awaiting-judgment |
| REG-FRONT-DOOR-DELIVERY-001 | must | resolve_the_host_kernel_binary_from_the_repo_root_under_either_delivery | D-awaiting-judgment |
| REG-FRONT-DOOR-DELIVERY-001 | must | offer_the_host_setup_to_the_principal_never_perform_a_host_mutation_silently | B-court-enforced-candidate |
| REG-FRONT-DOOR-DELIVERY-001 | must_not | run_the_enforcement_commit_hook_inside_the_container_or_make_it_depend_on_the_container_being_up | D-awaiting-judgment |
| REG-FRONT-DOOR-DELIVERY-001 | must_not | treat_the_agent_having_used_the_dockerized_server_as_proof_of_conformance | B-court-enforced-candidate |
| REG-FRONT-DOOR-DELIVERY-001 | must_not | require_a_rust_toolchain_on_the_host_to_obtain_the_wall_binary | D-awaiting-judgment |
