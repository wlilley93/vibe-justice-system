# Drafting Brief: the Consolidation & Reconciliation Framework Act (and its raft)

**Status:** drafting brief / referral to the Standing Committee. No force. Consolidates and supersedes the earlier programme note (2026-06-09-legislative-programme-sc-1.md). Every instrument is a **void first draft** until adopted and assented (CASE-LAW s.23).
**Sources of direction:** [2026] VJS-SC 1 (Q3-Q6 + the five-instrument programme), [2026] VJS-PC 1 (transition/onboarding), [2026] VJS-PC 2 (repository topology), and the Principal's directions of 2026-06-09.
**Drafting standard (a forward duty, see Part DUTIES):** UK-statute form - Parts, numbered sections, subsections - **as lean and legible as possible**. One idea per subsection. No ceremony without kernel effect. Restate, do not re-argue.

---

## A. The Framework Act (primary, Sovereign-assented, constitutional rank)

**Working title:** Realm Consolidation and Reconciliation Framework Act. **It is the law that restates the settled V1 law as a whole into the canon, and frames all future lawmaking.** It must contain, in lean sectioned form:

**Part 1 - Preliminary.** Short title; commencement; interpretation/definitions (only operative terms); the rule that a section without kernel effect is not runtime law; concordance (prose vs kernel_effect; discrepancy is a validation defect).

**Part 2 - The restatement of the settled law.** A concise restatement, **one principle per subsection, each tracing to its V1 source** (the source citation kept so the graph resolves; the source is archival, not live - see Part ARCHIVE). The restatement is fed by the **researcher's principles digest** (one sentence per valid V1 principle, across CASE-LAW, the 32 Bills, the SIs, and the good-law caselaw). Void / per-incuriam / superseded V1 material is not restated. Grouped by subject: the offices (Principal; Lexby's closed office); the duty of reasonable skill and care and the graded endeavours; the courts and the odd-bench rule; the public/private boundary; the real-world-law floor; the citation and record discipline; the agent-loop duties; etc. **Lean: the principle, not the V1 prose.**

**Part 3 - Lawmaking and future adaptation (routes out).** The bounded lawmaking route (proposal -> draft -> validate -> adopt -> enter -> log); **the power to make Statutory Instruments / subordinate regulations** under this Act (the bounded delegated route, anti-Henry-VIII: an SI may not amend this Act or the assent rule); the rule that primary law needs Sovereign assent and subordinate law needs parent authority; amendment is append-only and express (no silent repeal, no implied V1 import); **registers may be mandated by this Act but maintained at the lowest competent level** (subsidiarity - the centre mandates that a register exist and its schema; the local/relevant level maintains its contents).

**Part 4 - The assent floor (carry CASE-LAW s.23).** Royal Assent is the sole constitutive gate, non-automatable, non-presumable, non-delegable; the computational legislature may draft only, is not sovereign, may not expand competence, amend the assent rule, or create force from its output; the fail-closed Assent-Source Invariant.

**Part 5 - The courts and transition.** The courts continue in continuity (County 1 / Privy 3 / Supreme 5-9; Court of Appeal persists, convened on need); the Privy Council is the transition court classifying pending matters (continue / reframe / moot); the narrow V1-Court limb (perfect already-heard matters, publication-only); no relitigation by reason only of transfer.

**Part 6 - Federation, subscription, and the mandatory transition.** Install creates a local jurisdiction anchored at `.vjs/config.toml`; default subscription to the canonical lawpack (ACT-007); local sovereignty preserved (amend local law, pin/decline a version, fork with declared lineage, exit - REALM-PC 17); the federation coordination test (the SC-1 bright-line + the practical-subjection standard reserved to a court). **The mandatory transition:** the law binds every repo of the realm **as of commencement** - it applies to them today - and **mandates that each repo perform the internal subscription work** (create `.vjs/config.toml`, declare lawpack lineage, default-subscribe) within a stated period; subscription is the default and is owed now, with the lawful exit/variation routes (s.3/s.4, REALM-PC 17) preserved. A **repos register** is mandated (each subscribed repo recorded), maintained at the federation level but populated by each repo's own subscription act.

**Part 7 - Canonicalisation and the Gazette.** The canon is unqualified; "V1" is the Archive/Gazette estate label; de-naming is prospective and strictly non-disturbing (digests, lock, citations, IDs immutable); the governed rename (no mass edit; preserve paths or redirect; fresh lock under a commencement addendum; halt-and-re-refer if invariant-scoping cannot be proven). One Gazette, two estates, on the canonical line; the V1 estate is an immutable-tag-anchored, protected, read-only estate rendered distinctly.

**Part 8 - Offices and ministries by role, not directory.** There is **no mandatory four-branch ministry directory** (Bill 27's mandate is not carried as runtime structure); the functions formerly held by the Ministry of Justice (adjudication/records) and the Ministry of Business, Engineering and Skills (engineering/build) **persist as configured roles**, discharged by whichever record/agent holds the role, in any directory. A path name creates no legal force; records bind by role, schema, id, status, authority, and kernel effect. **Projects may live in any directory or repo** provided they are lawfully subscribed and recorded in the repos register.

**Part 9 - Licence.** Record the Principal's adoption of **VJS's own AGPL licence** for the canonical codebase (the prerogative of the copyright holder; the public repo is presently unlicensed). State the network-copyleft consequence and the separate-service boundary for AGPL third-party tools (e.g. a clean-roomed Picard).

**Part 10 - Transitional, savings, severance.** V1 citations remain historical identifiers; migrate V1 only by incorporation; the entrenched guarantees (assent floor; apex-singleness) are non-severable; the rest severable.

## B. DUTIES (carried forward + new forward duties)

Carry the V1 agent duties (reasonable skill and care; all/best endeavours by engagement; proactive disclosure; offer the lawful route; no self-authorised law; nemo iudex) **and add the forward duty**: all legislation and records must be drafted **lean and legible** (UK-statute form; one idea per subsection; no kernel-effect-free ceremony); breach is a falling-below to be made good, not punished.

## C. The raft (subordinate regulations, standing-bounded assent, parented by the Act)

1. **Migration & Incorporation Regulation** - the s.8 incorporation-record form + `validity_of_original`; the **migration ledger** (provenance-scoped, not runtime-loaded; single source of truth); the fail-closed `INV-INCORPORATION-VALIDITY-001` (missing element / rank breach / missing ledger row / floor dilution -> reject); the general-vs-repo-specific split (declared `scope`); the **archival-status / citation-graph-continuity rule** (REALM records are archival-source-only with estate markers; V1 citations are source edges, not authority edges); contested-original validity -> V1 Court.
2. **Federation Coordination Regulation** - the SC-1 bright-line as kernel `must_not` predicates; the practical-subjection standard reserved to the bench; the s.2 revocable-default gloss.
3. **Transition-Court & Continuity Regulation** - completes REG-TRANSITION-CONTINUITY-001: PC as transition court; classify -> {continue|reframe|moot}; the narrow V1-Court perfection limb; CoA persists.
4. **Canonicalisation & Migration Regulation** - the governed rename + lock-preservation procedure + redirect (PC-2 staged migration).
5. **Gazette-Continuity Regulation** - the two-estate single-Gazette build; estate markers; distinct Archive rendering; the immutable-tag pin; the force-source rule; the V1-estate SI 7 interface.
6. **Repos Register & Subscription Regulation** - the register schema (mandated centrally, maintained per-repo); the mandatory-subscription procedure and period; lineage declaration.

## D. Sequencing & leanness

Adopt the **Framework Act first** (it parents the raft and supplies the SI power); then the regulations as SIs under it. Keep the Act constitutional-commitment-only; push machine detail down to the regulations (Aldous's restraint floor). The whole system should read as a **small, tight statute book**: one Framework Act + a short raft + the compact kernel records - not a re-bloat of V1.

## E. Inputs required before drafting

1. The **researcher's principles digest** (one sentence per valid V1 principle, sourced) - the raw material for Part 2.
2. The PC-2 / SC-1 / PC-1 rulings (settled; cited above).

Nothing here is binding, migrated, renamed, subscribed, or re-licensed. The Committee drafts void first drafts; the Sovereign assents; the registers and subscriptions are then performed by each repo.
