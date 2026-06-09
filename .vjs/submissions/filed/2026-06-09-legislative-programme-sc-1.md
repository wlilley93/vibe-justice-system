# Legislative Programme directed by [2026] VJS-SC 1 (priming the Standing Committee)

**Status:** drafting brief / referral to the Standing Committee. No force. Each instrument is a **void first draft** until adopted (Committee) and assented (Sovereign, where primary rank) or made under parent authority (subordinate rank), per CASE-LAW s.14 / s.23.
**Source:** [2026] VJS-SC 1 (order 2026-VJS-SC-001); built on [2026] VJS-PC 1 (order 2026-VJS-PC-001).
**Purpose:** capture the settled SC principles as enacted legislation so the settlement does not rest as bare apex precedent (the REALM-SC 8 method: the Court ruled the principles; the legislature enacts them).

Lexby drafts each as a void first draft and routes it to the Committee; the Committee reconciles and reports; the Sovereign assents where required. Lexby does not enact.

---

## Instrument 1 - Federation Coordination Regulation (Q3)

**Authority:** ACT-007 (federation); ACT-COMPUTER-FIRST-REALM s.15 (apex). **Rank:** regulation (standing-bounded assent).
**Must codify:**
- the **bright-line** as kernel-checkable, fail-closed `must_not` predicates over a root's act: (i) no asserting/claiming an apex or final-court function; (ii) no binding/writing/gating a peer's local law or `.vjs/config.toml` without that peer's recorded adoption (s.4); (iii) no overriding the canonical lawpack without s.3; (iv) no foreclosing a peer's amend / pin-or-decline (s.7) / fork-with-lineage (s.6) / exit (REALM-PC 17);
- the **practical-subjection standard** as an interpretive, court-applied section expressly **reserved to the bench, not the kernel** (s.11): leveraging an infrastructural chokepoint to make compliance practically unavoidable, or conditioning essential access/Gazette publication on central terms beyond the canonical lawpack, is sovereignty;
- a gloss on ACT-007 s.2 that **default subscription is lawful because genuinely revocable** (decline/pin/fork/exit must remain real, not sham);
- the rule that coordination may standardise *up to* the protective floor but may never bargain a peer *below* it.

## Instrument 2 - Migration and Incorporation Regulation (Q5)

**Authority:** ACT-COMPUTER-FIRST-REALM s.8/s.9; Migration Charter s.11. **Rank:** regulation.
**Must codify:**
- the **incorporation-record form**: the five s.8 fields (`v1_source`, `v2_destination`, `operative_rule`, `kernel_effect`, `supersession_or_variation`) **plus a mandatory `validity_of_original` finding**;
- the **migration ledger** as a deterministic, append-only, **provenance-scoped** record (one row per crossing), the single source of truth (Charter s.11), **not loaded into runtime agent context**;
- **`INV-INCORPORATION-VALIDITY-001`** - a fatal, fail-closed, allow-list invariant rejecting any incorporation missing an s.8 element or the validity finding, breaching the **rank-floor** (destination rank must be >= source rank; a lower-rank record may not incorporate higher-rank V1 material), missing a ledger row, or **weakening the protective floor** (real-world-law boundary, rights/due process, public/private boundary);
- the **general-vs-repo-specific** split by a declared `scope` field (general -> central lawmaking route into the canonical lawpack; repo-specific -> the [2026] VJS-PC 1 continuity election, local-scope); doubt resolves to general;
- **propose-by-any-agent, adopt-by-organ-only** (REALM-SC 8); entry-by-entry, no bulk import (REALM-SC 7);
- the **contested-validity referral**: a genuinely contested V1-source validity question is found by the V1 Court (narrow), and the incorporation records that finding by reference.

## Instrument 3 - Transition-Court and Reconciliation Instrument (Q4)

**Authority:** ACT-COMPUTER-FIRST-REALM s.9/s.15; 2026-VJS-COURTS-CONSTITUTION-001; builds on REG-TRANSITION-CONTINUITY-001 (VJS-PC 1). **Rank:** regulation/order.
**Must codify:**
- naming the **Privy Council as the transition court** that classifies each pending V1 matter (continue / reframe / moot per VJS-PC 1) - no new tier (s.15; REALM-PC 15);
- the **narrow V1-Court referral gate** as a fail-closed flag: the V1 Court acts iff the matter was fully heard in V1 before commencement, only perfection-in-V1-form + Gazette publication remains, no new doctrine is made, and nothing is imported into V2 (s.9; s.16);
- a recital that the **Court of Appeal persists** (convened on need; abolition needs express s.10 amendment) - no appellate hole;
- the s.9 guardrail and the classify->{continue|reframe|moot} triage from VJS-PC 1 (this instrument completes REG-TRANSITION-CONTINUITY-001 with the SC's federation clause resolved).

## Instrument 4 - Canonicalisation Instrument (Q6)

**Authority:** ACT-COMPUTER-FIRST-REALM s.16; commencement. **Rank:** constitutional/primary (Sovereign assent).
**Must codify:**
- the **principle**: the commenced successor is the unqualified canon (VJS / the Realm / the lawpack / the kernel); the "V2" ordinal is spent on commencement and dropped from names going forward; **"V1" survives only as the Archive/Gazette estate label**;
- the **non-disturbance rule** as a hard `must_not`: de-naming is prospective and nominal; **no retro-edit** of any enacted record, citation, digest, record ID, assented text, or protective/void-status record; the assented digest `8e1d3f51...`, the lawpack lock `4d2639cc...`, and the `[YEAR] VJS-ACT N` series are immutable;
- the **governed rename mechanism**: prospective naming; physical `lawpack/v2/` paths preserved behind a compatibility/redirect, or moved only by a single staged migration that records a **fresh lock under a commencement addendum** (the old lock stays pinned); an **unstaged mass edit is prohibited** (it would break the lock and un-scope path-scoped invariants fail-open).

## Instrument 5 - Gazette-Continuity / Publication Regulation (Q4 / the unheard reference)

**Authority:** ACT-COMPUTER-FIRST-REALM s.16; builds on REG-GAZETTE-CONTINUITY-001 (proposed). **Rank:** regulation.
**Must codify:** the content/force/estate-label split (V2 governs what/whether it publishes and the force-source rule; the act on the V1-hosted repo follows the V1 estate's preserved SI 7 / PC 19, not barred by s.9); the publication packet, export-fail-unless checks, estate labelling operative-by-substance, and lineage edges; and it **cites the estate-label rule** captured once in Instrument 4. The Gazette publication-governance reference is remitted to the **Privy Council** for hearing; this regulation is its cure.

---

## Sequencing note (for the Committee and the Principal)

The instruments are independent void first drafts and may be adopted in any order, save that **Instrument 4 (canonicalisation)** should fix the estate-label rule that Instrument 5 cites, and **Instrument 2's** rank-floor and ledger should be in place before any general-law extraction is adopted. Royal Assent attaches to each only on its valid adoption against its final text (s.23(6)); the Principal's standing assent "where validly capable" does not ripen on any instrument until it is Committee-adopted. Nothing here is binding, migrated, renamed, or published.
