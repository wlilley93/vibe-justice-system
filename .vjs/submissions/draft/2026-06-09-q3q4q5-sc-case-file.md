# Symmetric Case File: Federation/Apex (Q3), the Holes and their Mechanism (Q4), and General-Law Extraction (Q5)

**For:** the V2 Supreme Court (bench of 5)
**Filed by:** Lexby as s.19(1) registrar of the two-sided intake (NOT as advocate; both sides put with equal force; the bench has no access to Lexby's preference)
**Companion to:** the SC reference of 2026-06-09; [2026] VJS-PC 1 and order 2026-VJS-PC-001
**Status:** public; system-data only; intake material, not a judgment or law

> Two-sided by design. The bench decides. The entrenched single-apex rule is settled and is NOT re-opened.

## Part 1 - Agreed facts

1. V2 commenced 2026-06-09. The V2 PC ([2026] VJS-PC 1) leapfrogged the constructive apex/federation question here, recording as **settled**: the super-repo survives as an ACT-007 federation under one continued apex; no root may stand up a second apex or bind a peer beyond ACT-007 s.4.
2. **ACT-007**: install creates a local jurisdiction (anchor `.vjs/config.toml`); default subscription to the canonical lawpack (s.2); local Principal may vary local law but not override canonical without a PC order or assent (s.3); local law is local-scope-only (s.4); forks declare lineage (s.6); versions may be pinned (s.7).
3. **Bill 32**: supersession of V1 as live runtime law (s.6); express incorporation only (s.8); no V1 revival by silence (s.9); single-apex continuity entrenched (s.15); assent floor (CASE-LAW s.23).
4. **Migration Charter s.11**: a V1 doctrine is live in V2 only if it is in the **migration ledger** with a V2 destination; the ledger is the single source of truth.
5. Persuasive archive: REALM-SC 6 (super-repo over hearing-centres; single apex never relaxed), REALM-SC 7 (validity entry-by-entry; nullity non-infectious; nothing voided wholesale by provenance), REALM-PC 15 (super-repo is realm-as-state, adds no tier/seat/apex), REALM-PC 17 (install/fork creates a local jurisdiction; default subscription is not perpetual subjection; local sovereign may exit/federate).
6. The V2 courts are constituted (County 1, Privy 3, Supreme 5/9); the Court of Appeal persists but is administratively non-convened.

## Part 2 - Q3: the coordination/sovereignty line

**Case A (broad, consent-anchored coordination is lawful).** A coordinating root may do extensive infrastructural work - distribute the canonical lawpack, offer version pinning, run shared CI, cross-repo discovery, aggregated Gazette export - because none of that is *sovereignty*: it asserts no apex, makes no peer local law without the peer's adoption (s.4), overrides no canonical law without s.3, and leaves every local sovereign free to amend, pin/decline a version, fork (s.6), or exit (REALM-PC 17). Default subscription (s.2) is *consent-based and revocable*, not imposed rule; REALM-PC 15 already blesses a realm-as-state coordinating layer that adds no apex. The test: a root acts lawfully if its act is (a) infrastructural, (b) non-binding on peer local law absent adoption, (c) canonical-respecting (no s.3 override), and (d) exit-preserving.

**Case B (tight; coordination easily hardens into sovereignty).** Default subscription means the root's canonical lawpack *governs* peers by default - that is soft sovereignty, and infrastructural power (who ships the lawpack, sets versions, gates the Gazette) is exactly how a centre erodes local sovereignty by fait accompli. The line must be drawn tightly: a root that makes any peer's compliance practically unavoidable, or that conditions a peer's access/publication on accepting central terms, has crossed into sovereignty regardless of a formal "right to exit" few can use. Protect ACT-007 s.3-s.4 and REALM-PC 17 with a bright-line that bites on *practical* subjection, not just formal apex-claims.

## Part 3 - Q4: the holes and the mechanism for each

For each candidate hole, the contest is which mechanism applies. The principled allocation: **foundational/entrenched -> V2 Supreme Court; routing/constitutional first instance -> V2 Privy Council; repo-local -> County Court; new law/instrument -> V2 lawmaking route; reversible executive choice -> Principal policy; touches the V1 estate / already-heard V1 matter / V1's own historical law -> V1 Court; already settled -> dispose on citation.**

Candidate holes and the live tension on each:
- **Gazette publication governance** (unheard reference): PC matter (routing/public-boundary) + a directed V2 regulation. *Tension:* the interim push to the V1-hosted repo - V2 mechanism vs V1 SI 7 (a V1-estate matter).
- **The two directed regulations** (Transition+Continuity; Gazette): V2 lawmaking route (Committee + assent). Not adjudication.
- **"Perfect by old machinery"**: perfecting an *already-heard V1 matter* is V1's machinery acting on V1's estate -> arguably the **V1 Court** is the only body that can perfect it; but s.9 warns against reviving V1 machinery. *Tension:* V1-Court-necessary vs s.9-no-revival. The bench should draw the line (perfection of a genuinely already-heard matter, publication-only, is the narrow V1-Court case; anything wider is V2's).
- **The "V2 transition court"**: who constitutes/sits as the classifier of pending matters? Is it the PC, a County Court, or a named instrument? Gap to assign.
- **Non-convened Court of Appeal**: is there an appellate gap in V2? Mechanism: it persists; convened on need; abolition needs express amendment. Likely *settled/no-hole*, but the bench should confirm.
- **Citation series minting** (VJS-PC/REG/SPEC/DEC): deterministic-mint consistency. Likely V2 lawmaking/engineering, not adjudication.
- **Any live pending V1 matter at commencement**: carriage per VJS-PC 1; classification mechanism per the transition-court gap.

*Tension throughout:* over-referring to the V1 Court re-imports V1 machinery (s.9 hazard); under-referring strands matters only V1 can finish (already-heard V1 cases). The bench must hold the narrow line.

## Part 4 - Q5: extracting valid general V1 law into the V2 substrate

**Case A (the lawmaking-route incorporation pipeline).** General/substrate V1 law crosses by the central V2 lawmaking route: a void-first-draft incorporation record (s.8 fields: v1_source, v2_destination, operative_rule, kernel_effect, supersession), validated, adopted (Committee + Sovereign assent for primary; parent authority for subordinate), entered in the lawpack and the migration ledger (Charter s.11). Repo-specific law uses the *continuity election* (VJS-PC 1), local-scope. The general-vs-repo-specific test: does the doctrine state a realm-wide rule (general -> substrate) or only a repo-local convention (-> election)? An incorporation-validity invariant fails closed on any missing s.8 element. Clean, deterministic, ledger-audited.

**Case B (cautions).** (i) **Who certifies the original was valid?** "Extract where valid" requires a validity finding on the V1 source (not spent/overruled/void/per-incuriam); for contested sources only a **V1 Court** finding is authoritative - else V2 risks importing V1 law that V1 itself would not stand behind. (ii) **Bulk-import hazard:** a pipeline that makes incorporation easy invites wholesale re-importation of V1, defeating the compact-substrate purpose; the ledger and assent gate must keep it deliberate and entry-by-entry (REALM-SC 7). (iii) **Rank-floor:** a lower-rank V2 record must not incorporate higher-rank V1 material (echoing the Bill 32 rank-floor gloss); incorporation rank must match destination rank. (iv) **Who initiates:** is general-law extraction a Principal/Committee initiative, or may any agent propose? (Propose yes - void first draft; adopt only by the organ.)

## Part 4A - Q6: canonicalisation ("V2" is now canon)

**Case A (drop "V2" now).** A version ordinal that distinguished a *pending* successor from the incumbent is spent the moment the successor commences and becomes the sole live Realm. Continuing to call the canon "V2" is an anachronism that implies a live "V1" runtime peer, which no longer exists. The Realm is unqualified (VJS / the Realm / the lawpack); "V1" survives only to label the Archive/Gazette estate. De-naming is prospective and nominal: it touches no enacted record, citation, digest, or assented text. The mechanism is a short canonicalisation instrument + a governed engineering rename that preserves digests and citations.

**Case B (caution / timing).** While the V1 estate still exists and is actively referenced (the Gazette is V1-hosted; cross-estate matters are live), the "V1/V2" pairing is still doing real distinguishing work; dropping "V2" prematurely could create ambiguity in cross-estate records. A mass rename also risks breaking the very digests, citations, record IDs, and lockfiles the settlement depends on (the `8e1d3f51...` assented-text digest; the lawpack lock; `lawpack/v2/` paths embedded in invariants and orders). The safer course may be: settle the *principle* now, but stage the rename behind a governed instrument with a digest/citation-preservation guarantee, and keep "V1" as the estate label so the pairing still reads.

**Likely shape (for the bench):** settle the principle (canon is unqualified; "V2" dropped going forward; "V1" kept only as the estate label); make de-naming **prospective and non-disturbing** (no retro-edit of enacted records, citations, digests, assented text); and direct the rename as a **governed change** through the lawmaking/engineering route, not a mass edit.

## Part 5 - Relief options

- **(i)** Settle Q3 (the four-part consent-anchored test, with or without Case B's practical-subjection rider); map Q4's holes to mechanisms (drawing the narrow V1-Court line for already-heard matters only); settle Q5's pipeline and direct a Migration and Incorporation Regulation + a migration ledger + an incorporation-validity invariant; return to the PC / lawmaking route.
- **(ii)** As (i) but expand to 9 if the bench finds the entrenched rule is in fact being re-opened.
- **(iii)** Decide Q3 only; remit Q4/Q5 to the PC and lawmaking route.
- **(iv)** Identify a hole that genuinely needs the V1 Court and refer that limb to V1.

The bench is bound by none of these. Nothing is migrated, adopted, or published by this file.
