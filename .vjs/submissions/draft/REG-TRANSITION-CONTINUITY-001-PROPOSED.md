# PROPOSED DRAFT: Transitional and Continuity Instrument (V1 -> V2)

**status:** proposal / void first draft (REALM-SC 8; CASE-LAW s.23(6)). NOT a loaded runtime record; no force until adopted through the V2 lawmaking route. Kept in `.vjs/submissions/draft/` so it is not (correctly) rejected by INV-ASSENT-SOURCE-001 / INV-LAWMAKING-002.

When adopted it would be lodged as `lawpack/v2/regulations/REG-TRANSITION-CONTINUITY-001.yaml`, `authority: ACT-COMPUTER-FIRST-REALM` (and ACT-007 for the onboarding limb), `assent_source: standing_bounded_assent`, VJS-REG ordinal minted at adoption.

Modelled on the Judicature Acts 1873 continuity doctrine (persuasive comparative authority): transfer does not erase pending cases; it carries them across at their current procedural state, preserves accrued rights, and lets the successor court finish them under old or new procedure as justice requires.

---

## Proposed record (for the bench / Committee to vary or reject)

```yaml
id: REG-TRANSITION-CONTINUITY-001
title: Transitional and Continuity Instrument (V1 to V2)
authority: ACT-COMPUTER-FIRST-REALM:s9   # plus ACT-007 for onboarding
status: draft
text: >
  Governs the carriage of pending V1 matters and project local precedent into V2
  on and after commencement, on the continuity model: vest, preserve, perfect,
  continue, bridge, do not relitigate.
kernel_effect:
  defines:
    # 1. Vesting (declaratory; already effected by Bill 32 + courts-constitution)
    vesting: v1_runtime_jurisdiction_vested_in_v2_at_commencement
    # 2. Preservation of perfected orders and accrued rights
    preserved:
      - v1_orders_and_judgments_perfected_before_commencement_stand
      - accrued_appeal_or_review_rights_in_pending_v1_matters_preserved
    # 3. Perfection by the old machinery (bounded)
    perfection_by_old_machinery:
      scope: v1_matters_already_heard_but_not_yet_perfected
      effect: may_be_perfected_in_v1_form_and_published_to_the_gazette
      bound: no_new_v1_doctrine_created; publication_only; guards_against_s9_revival
    # 4. Continuation in V2 at the current procedural point
    continuation:
      classifier: a_named_v2_transition_court_classifies_each_pending_matter
      assignment_by_issue:
        county_court: repo_local_operational_matters
        privy_council: constitutional_jurisdiction_routing_public_private
        supreme_court: foundational_questions
      rule: continue_at_current_state; not_void; not_restarted
    # 5. Procedural bridge
    procedure:
      going_forward: v2_procedure_governs
      bridge: v2_court_may_direct_old_style_procedure_as_nearly_as_may_be
    # 6. Anti-relitigation
    anti_relitigation: no_matter_void_restarted_or_relitigated_by_reason_only_of_transfer
    # 7. Project onboarding (with ACT-007)
    onboarding:
      mechanism: local_installation_by_local_principal (ACT-007:s1-s2)
      v1_local_precedent_default: carried_only_by_express_incorporation (Bill 32 s.8; s.9)
      continuity_election:
        available_to: local_principal
        effect: wholesale_carry_forward_of_local_precedent_at_current_state
        recorded_in: [".vjs/config.toml", "local_lawpack"]
        scope: local_only (ACT-007:s4); revocable
    # 8. Apex (subject to Q3 leapfrog)
    apex: single_apex_continuity_preserved (ACT-COMPUTER-FIRST-REALM:s15); no_second_apex
  must:
    - classify_each_pending_v1_matter_before_continuation
    - record_continuity_election_where_made
    - preserve_perfected_orders_and_accrued_rights
  must_not:
    - treat_a_pending_matter_as_void_or_restarted_by_reason_only_of_transfer
    - create_new_v1_doctrine_under_the_perfection_limb
    - let_a_super_repo_root_stand_up_a_second_apex
  proof:
    - each_pending_matter_has_a_recorded_classification_and_forum
    - each_continuity_election_recorded_in_config_and_local_lawpack
```

## The completed transitional clause (prose, for the ceremonial form)

> All V1 matters pending at V2 commencement shall be classified by the V2 transition court and continued in the V2 court competent for the issue, at their current procedural state. Matters fully heard in V1 but not yet perfected may be perfected by the V1 machinery in V1 form and published to the Gazette, creating no new doctrine. V1 orders and judgments perfected before commencement, and accrued appeal or review rights in pending matters, are preserved. V2 procedure governs continued matters going forward, save that the V2 court may direct procedure in the old manner, as nearly as may be, as a transitional bridge where justice requires. No pending matter is void, restarted, or relitigated by reason only of the transfer. A project is onboarded to V2 by local installation under the local Principal's authority; its V1 local precedent is carried into V2 only by express incorporation, save that the local Principal may make a recorded, revocable continuity election carrying its local precedent forward at its current state, of local effect only.

## Open for the bench

The apex / super-repo reconciliation (Q3) is flagged for leapfrog to the V2 Supreme Court; this draft preserves single-apex continuity pending that ruling and does not itself decide it.
