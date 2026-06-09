# PROPOSED DRAFT: Gazette-Continuity / Publication Regulation

**status:** proposal / void first draft (REALM-SC 8; CASE-LAW s.23(6))
**no force:** This is a proposal for the V2 Privy Council and, on direction, the V2 lawmaking route. It is NOT a loaded runtime record and carries no force until adopted (Committee/standing-bounded route + lawpack entry + lock). It is deliberately placed in `.vjs/submissions/draft/`, not in `lawpack/v2/regulations/`, because a draft regulation in the runtime tree would (correctly) be rejected fail-closed by INV-ASSENT-SOURCE-001 (no assent yet) and flagged by INV-LAWMAKING-002.

When adopted, it would be lodged as `lawpack/v2/regulations/REG-GAZETTE-CONTINUITY-001.yaml`, its `assent_source` resolving to `standing_bounded_assent` (tracing to Bill 32 / ACT-COMPUTER-FIRST-REALM s.16), and its VJS-REG ordinal minted deterministically at adoption.

---

## Proposed record (for the bench / Committee to consider, vary, or reject)

```yaml
id: REG-GAZETTE-CONTINUITY-001
# assent_source: standing_bounded_assent   # set only on adoption
# citation: "[2026] VJS-REG N"             # minted at adoption
title: Gazette-Continuity and Publication Regulation
authority: ACT-COMPUTER-FIRST-REALM:s16
status: draft
text: >
  Defines how V2 records are exported from the V2 lawpack and published into the
  single VJS Gazette, without the Gazette becoming the runtime source of V2 law.
  One Gazette, two estates; publication is constitutively inert; the estate
  boundary is operative by substance; a Gazette copy is never the runtime source.
kernel_effect:
  defines:
    estates: [v1_archive, v2_current]
    entry_must_declare: [estate, source_of_force, lineage]
    publication_packet:
      - manifest
      - validated_records
      - rendered_views
      - redaction_report
      - validation_report
      - lawpack_digest
    allowed_lineage_edges:
      - supersedes_runtime
      - preserves_archive
      - incorporates
      - derived_from
      - published_in
      - explains
    publish_by_default:
      - constitutional_acts
      - kernel_regulations
      - public_rule_atoms
      - public_specs
      - public_invariants
      - public_decisions
      - public_court_orders
      - migration_ledger_summaries
      - lawpack_manifest
      - lawpack_digest
  must:
    - generate_publication_packet_from_validated_lawpack
    - declare_estate_source_of_force_and_lineage_per_entry
    - trace_v2_runtime_force_to_lawpack_not_publication
  must_not:
    - treat_gazette_entry_as_runtime_source
    - treat_publication_as_v1_enactment_or_incorporation
    - publish_private_decision_logs_permits_proofs_or_unredacted_facts
  proof:
    - export_must_fail_unless:
        - lawpack_validates
        - public_private_scan_passes
        - statuses_valid
        - citations_unique
        - lineage_resolves
        - manifest_includes_digest
```

## Transitional interface with the V1 estate (for the bench to settle)

Until this Regulation commences and a V2-controlled publication route exists, the *act* of pushing a generated packet to the V1-hosted Gazette repository proceeds under the V1 estate's preserved law (REALM-SI 7 release warrant + Privy Council post-push review; REALM-PC 19), not as V1 binding V2 runtime (s.9), but as V1 governing its own repo. On commencement of this Regulation with a V2-controlled route, V2's mechanic governs V2 publication; the V1 SI 7 review remains available to the V1 estate for its own repo integrity.

## What a future `vjs gazette` surface would do (engineering note, not law)

`vjs gazette export` (build the packet from the validated lawpack, with redaction), `vjs gazette validate-packet` (the export-fail-unless checks), `vjs gazette publish` (emit to the target). Deterministic; no model call. This is the V2-native replacement for leaning on V1's SI 7 by analogy.
