# VOID FIRST DRAFT - Subscribing Jurisdiction and External Supremacy

**DISPOSED (2026-06-11). This draft is spent.** Its open tier question was settled by [2026] VJS-PC 10 (the supremacy clause is the subscriber's own charter act engaging no Sovereign-assent floor; the mechanics are machinery under the [2026] VJS-PC 8 test). Its mechanical content is adopted as **REG-ACCESSION-001** (the Accession and Subscription Form Regulation, made under the s.7 power). Its supremacy limb is expressly NOT enacted Realm-side: per PC-10 it belongs to each subscriber's own charter. Nothing below carries any force; it remains as drafting history.

**Original status: a void first draft, awaiting committee adoption and the Sovereign's digest-pinned assent. It enacts nothing. Drafting is not assent (the assent floor; the cured breach of 2026-06-09). The Principal stated they would lodge this law; this draft is offered as the strongest case to assent to or replace, not to commence.**

**Persuasive authority:** [2026] REALM-SC 11 (the server-of-law judgment, V1-lineage, persuasive) and the federation statute ACT-007. The concept is **accession with supremacy (primacy)** - the legal shape by which a jurisdiction joins a higher legal order and accepts the supremacy of its law, given domestic force by the joining sovereign's own act (the dualist model; the United Kingdom's European Communities Act 1972, ss.2(1) and 2(4); the primacy doctrine of Costa v ENEL).

## 1. Open question of tier (for the committee / a court)

Establishing the **supremacy** of canonical VJS over a subscriber's local law is constitutional in character and may require **primary law (Sovereign assent)**, not a subordinate instrument; the **mechanical** subscription framework (pinning, currency, projection) is plausibly subordinate under the federation power (ACT-007; ACT-CONSOLIDATION-FRAMEWORK s.7). This draft does not presume its tier; the committee or the Privy Council should settle it before commencement.

## 2. Accession (the joining act is the subscriber's own)

A jurisdiction becomes a **subscribing jurisdiction** only by an express act of **its own local Sovereign** giving canonical VJS law supremacy within it. The higher law binds the subscriber because the subscriber's Sovereign so enacted, never because VJS imposed it; this preserves the assent floor on both sides and leaves the local Sovereign the reserved power to **repeal accession** (the withdrawal reservation).

## 3. Supremacy (primacy, with reservations)

On a genuine conflict between canonical VJS law and the subscriber's local law, **canonical VJS prevails**, save: (a) the subscriber's **entrenched local constitution**; (b) the **accession act itself**; and (c) the **external-law floor** of [2026] REALM-SC 9 - real-world law (regulation, confidentiality owed to real persons) is a floor VJS permission cannot lift. A subscriber may add local law that does not contradict the canon.

## 4. Lockstep and the currency of source (the safe mechanism)

Per [2026] REALM-SC 11, condition (1): the subscriber must render the canonical text **live as the apex enacts it** - never a vendored, pinned, or stale local copy whose currency the subscriber's release cycle, rather than the canonical apex, controls. The safe mechanism is **content-addressed subscription**:

1. The accession record **pins the sha256 of the canonical lawpack** the local Sovereign assented to. The local kernel **loads only law that hashes to the pinned digest** and fails closed otherwise; transport (a git pull, a read-only data endpoint, a signed release) is irrelevant because nothing loads unless it matches the assented hash.
2. **Lockstep = digest bumps by assent.** A new canonical digest is adopted locally only on the local Sovereign's assent to that digest, or under a pre-authorised bounded auto-track; no law flows downstream by silence.
3. **Caution (REALM-SC 11):** the moment a subscriber serves its **own vendored copy** of the canon, it has minted a subscribed local jurisdiction under ACT-007 / s.9, with the duties that entails - not a thin client.

## 5. The two lawful shapes

A subscriber may take either shape, and must meet the six conditions of [2026] REALM-SC 11 where they apply:

- A re-skinned **client** that renders the one central canon **live** (no local copy), under all six conditions: unity and currency of law; unity of citator and apex; the synthesiser bar (no bench of its own); delivery not law-making; candour of provenance; and the external-law non-impersonation floor.
- A **subscribed jurisdiction** with its own digest-pinned copy under ACT-007, accepting canonical supremacy, kept in lockstep.

## 6. Presentation is inert (the DB-backed subscriber)

A subscriber whose presentation surface is a **database** rather than the Gazette is lawful: publication is constitutively inert (REG-GAZETTE-CONTINUITY-001), so the database is a **projection** of the digest-verified canonical lawpack, rebuilt whenever the pinned digest changes - downstream of the law, never the source of it.

## 7. On commencement, the kernel will

record the accession (the local Sovereign's assent + the pinned digest); load only digest-matching law, fail-closed; resolve conflicts by the supremacy rule with the three reservations; and project the verified law into the subscriber's surface. The subscriber local form will be built to this prescribed shape once the law commences.
