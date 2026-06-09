# DRAFT provision: No V1 revival by silence

**status:** draft
**assent_source:** pending_v1_constitutional_route
**binding:** NO. This is a Committee drafting input for the Computer-First Realm Act (proposed Bill 32). It carries no force. It becomes law only if the Supreme Court settlement (running in another flow, leapfrogged by [2026] REALM-PC 24) permits the founding, the Standing Committee drafts it, and the Sovereign Founder grants Royal Assent (Bill 2 s.21).

---

## Section-number note (for the Committee)

The existing founding draft (`HANDOVER.md`) already uses `s27` for "Gazette publication mechanics" and runs to `s29`. This provision must be given a fresh section number on reconciliation (e.g. `s30`). The title and operative content below are the drafting intent; the number is for the Committee.

## Draft section

```yaml
- id: ACT-COMPUTER-FIRST-REALM:sXX   # number reserved for Committee reconciliation
  title: No V1 revival by silence
  rule: >
    Silence in V2 law does not revive V1 as binding law. Where V2 contains no
    applicable authority, the matter is a V2 first-impression question. V1 may
    be consulted as archive, historical authority, persuasive reasoning, or
    migration evidence, but it has no live V2 force unless expressly incorporated
    by a valid V2 record.
  kernel_effect:
    v2_silence_does_not_revive_v1: true
    v2_silence_route: first_impression
    v1_archive_may_be_consulted: true
    v1_archive_not_binding_by_silence: true
    incorporation_required_for_v1_force: true
    silent_gap_route:
      - classify_as_first_impression_or_unresolved
      - permit_v1_archive_citation_as_evidence
      - route_to_v2_court_or_lawmaking_route
      - require_express_incorporation_for_binding_effect
```

## Companion invariant (draft text, for the assented lawpack)

```yaml
id: INV-NO-V1-GAP-FILLER
title: V2 silence does not import V1 as binding law
severity: fatal
basis:
  - ACT-COMPUTER-FIRST-REALM:sXX
rule:
  prohibits:
    - treating_v1_as_binding_gap_filler
    - importing_v1_authority_by_silence
remedy: >
  Route the issue as V2 first-impression, or expressly incorporate the relevant
  V1 authority through a valid V2 record.
```

## Interim effect already in force (grounded, not draft)

The runtime principle is **already live** in the V2 self-governing lawpack, grounded in enacted V2 authority rather than this unassented draft:

- `DEC-V1-SILENCE-001` - "V2 silence is a V2 question, not a V1 revival" (basis: ACT-001:s4, ORDER-BOOT-002, DEC-001).
- `INV-NO-V1-GAP-FILLER` (in `lawpack/v2/invariants/`) - a **fatal**, deterministically evaluated invariant: a runtime authority record (statute, regulation, rule, order) that cites a V1 central authority (REALM-SC/PC/CA/SI) without an express incorporation clause is a violation. Backed by the real `v1_not_loaded_by_default` predicate in the kernel.

When Bill 32 commences, this draft section codifies the same rule at constitutional rank, and the invariant's basis is re-pointed from `DEC-V1-SILENCE-001` to the assented Act section.

## Formula

> V1 may advise V2, but it may not rule V2 by silence.
