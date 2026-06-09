# DRAFT: REG-REPOS-HOUSE-001 - Repos House Registry Regulation

**status:** VOID FIRST DRAFT per [2026] VJS-PC 5. Subordinate regulation under Framework Act s.7; **extends** (does not supersede) REG-REPOS-REGISTER-001. Drafted now; **build and lodgement sequenced after permit enforcement and the lawmaking route** (Repos House is not the critical path). Not runtime law until lodged + commenced.

When lodged it becomes `lawpack/v2/regulations/REG-REPOS-HOUSE-001.yaml` (`assent_source: standing_bounded_assent`, `authority: ACT-CONSOLIDATION-FRAMEWORK:s7`).

```yaml
id: REG-REPOS-HOUSE-001
assent_source: standing_bounded_assent
title: Repos House Registry Regulation
authority: ACT-CONSOLIDATION-FRAMEWORK:s7
status: in_force            # on lodgement
text: >
  Repos House records, certifies, discovers, and publishes VJS jurisdictions; it
  does NOT create their legal force, which derives from local sovereign
  invocation and the locked lawpack (REG-INVOCATION-001). It decides no merits
  (boundness, validity, whether evidence is public, whether V1 binds V2). It
  extends, and does not supersede, REG-REPOS-REGISTER-001 (the mandatory-
  subscription floor): Repos House is the certification and discovery layer above
  that floor. Two layers: a private in-repo register (.vjs/registry/) may hold
  operational facts; a public register (gazette/v2/repos/) carries system-data
  only - no secrets, paths, hostnames, logs, permits, proofs, or evidence.
  Certification is a restatement of deterministic kernel output, never a human
  attestation: local_ci, hooks_installed, boundary_scan, permit_gate are gate
  results or they are absent. Registration is optional for a private/local repo
  and owed only when it federates, publishes to the Gazette, or claims canonical
  status. The order of operations is fixed: invoke -> govern -> (optional)
  register -> certify-as-kernel-output -> public-safe system-data to the Gazette.
  The V1 Repositories House ([2026] REALM-SI 6) is carried forward as archival
  source, incorporated only by express incorporation.
kernel_effect:
  defines:
    registry_role: jurisdiction_registry
    declarative_not_sovereign: true
    registration_creates_no_force: true
    extends: REG-REPOS-REGISTER-001
    public_register_system_data_only: true
    private_register_may_hold_operational_facts: true
    registration_optional_unless: [federate, publish_to_gazette, claim_canonical]
    certification_fields_are_kernel_outputs: [local_ci, hooks_installed, boundary_scan, permit_gate]
  must:
    - derive_the_public_register_as_a_pointer_only_rebuildable_projection
    - keep_certification_a_restatement_of_deterministic_kernel_output
  must_not:
    - confer_or_decide_legal_force_boundness_validity_or_canonicity
    - publish_private_repo_facts
    - accept_a_human_attestation_in_place_of_a_kernel_output
```

## Public register entry shape (system-data only)
```yaml
id: repo:agent-universe-v2
jurisdiction_id: agent-universe-v2
estate: v2
visibility: private-source-public-metadata
status: active
lawpack: vjs-v2@0.1.0
lawpack_digest: sha256:...
registered_at: "2026-..."
certification: { local_ci: pass, hooks: installed, boundary_scan: pass, permit_gate: pass }
gazette_policy: redacted_public_packets_only
lineage:
  - { type: founded_under, target: ACT-COMPUTER-FIRST-REALM }
```

## Commands (sequenced after enforcement)
`vjs repos register | status | certify | export-public-packet | publish-to-gazette | list | lineage`

## Note
The companion **Court Registry + Gazette Clerk** and **Toolchain + Skills/Agent Registry** roles (PC 5 Q2) are record roles, not offices; a Skills/Agent registry is modelled on the V1 agent roll (derived, pointer-only). Policy is a **Policy Proposal record** routed by the existing kernel lawmaking route, not a department. None of this is lodged until the critical path (permit enforcement + lawmaking route) is complete.
