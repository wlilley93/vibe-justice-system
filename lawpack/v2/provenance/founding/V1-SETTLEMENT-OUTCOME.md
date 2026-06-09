# V2 Founding Provenance: the settled outcome of the V1 constitutional route

**id:** PROVENANCE-V2-FOUNDING-OUTCOME
**status:** v1_route_settled; royal_assent_granted; commencement_pending (Gate B engineering)
**recorded:** 2026-06-09
**supersedes the status of:** the "pending" markers in `HANDOVER.md` (this record states the settled outcome; the HANDOVER remains a draft implementation brief, not authority)

> The V1 constitutional route is now SETTLED. V2 receives that settlement as its founding authority. V2 does not yet have live runtime force: two gates remain, and one of them is a constitutive act of the Sovereign that no agent may perform.

## The settled V1 route

| Step | Outcome | Citation |
|------|---------|----------|
| Privy Council reference judgment | Defined the eight governing questions and the seventeen limits; referred up to the Supreme Court | **[2026] REALM-PC 24** |
| Supreme Court settlement (full court of 9, DECLARATORY, unanimous 9-0) | Settled the migration; **enacted CASE-LAW s. 23(1)-(6)** [constitutional] (the Sovereign-assent floor and AI non-sovereignty); issued the express handover order | **[2026] REALM-SC 10** |
| Standing Committee adoption | Adopted Bill 32 (the Computer-First Realm Act 2026) **4-0** at its second drafting round (after curing the Bill 27 s. 14(2) -> s. 15(2) entrenchment-citation defect); adoption is constitutive | Bill 32 (adopted) |
| Royal Assent | GRANTED 2026-06-09: positive, specific, digest-pinned assent (V1 record: Legislature/legislature/2026-06-09-royal-assent-bill-32.md); CASE-LAW s. 23 in force; Bill 32 enacted | done |

**Adopted Bill 32 final-text digest (sha256):** `8e1d3f516cb2aca8e044d8c73bdc6ededa91a47ca86b729eece06f7eee6b9a0c`

## What V2 inherits as binding founding law

1. **The Sovereign-assent floor (CASE-LAW s. 23(1)-(6)), now enacted in V1 and carried into V2 by the migration.** All V2 law, including AI-drafted or delegated law, derives binding force only from Sovereign assent (specific, or standing-bounded under a no-Henry-VIII enabling regime). AI may run the lawmaking machinery and the kernel as clerk but may never be sovereign, expand its competence, sub-delegate, amend the assent rule, or create force from its own output.
2. **The express handover order ([2026] REALM-SC 10).** On satisfaction of the conditions precedent, V2 becomes the self-governing computer-first successor jurisdiction for runtime purposes; V1 remains the Gazette and Archive estate; the V1 courts relinquish continuing runtime control, save the entrenched real-world-law floor. Until each V2 court is expressly constituted, jurisdiction over its matters remains with V1 (no phantom forum).
3. **The deterministic enforcement rule (CASE-LAW s. 23(5)).** The V2 kernel must reject, by an always-on, fail-closed, model-free, affirmative allow-list invariant, any record claiming runtime force without a valid traceable assent source. The existing DRAFT records `SPEC-ASSENT-DRAFT-001`, `INV-ASSENT-DRAFT-001`, and `INV-ASSENT-DRAFT-002` are now backed by settled V1 law and the handover order; they are to be promoted to settled lawpack records as part of commencement, in the affirmative allow-list form the Court mandated.

## The two remaining gates to a live V2 (commencement)

Per [2026] REALM-SC 10, commencement requires, and no commencement condition may be satisfiable by the V2 machinery alone:

- **Gate A - the Sovereign's positive, specific, digest-pinned assent.** SATISFIED 2026-06-09: the Sovereign granted Royal Assent against the digest 8e1d3f51...6b9a0c; CASE-LAW s. 23 is in force and Bill 32 is enacted. The sealed adopted bytes (`bill-32-adopted-final-text.md`, this directory) reproduce the digest.
- **Gate B - the V2 commencement engineering (authorised by the handover order).**
  1. Implement and prove the fail-closed Assent-Source Invariant (affirmative allow-list; rejects a missing-assent-field record and an unresolved-trace record).
  2. Expressly constitute each V2 court by a named constituting instrument fixing its odd bench (FI 1, CA 3, SC 5/9) and jurisdiction, before any V2 order issues.
  3. Validate, enter, and lock the V2 lawpack as the compiled carrier of the assented settlement, producing a single commencement record naming the assent event, the pinned digest, the lawpack version, and the lock.

When Gate A and Gate B are both satisfied, V2 commences and the constitutional relay is complete: **V1 discovered the law; V2 compiles it.**
