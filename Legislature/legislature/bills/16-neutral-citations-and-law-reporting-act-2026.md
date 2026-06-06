<!-- Bill 16 of 25 - Order Paper of the Realm. Drafted by the Standing Committee. -->
<!-- status: enacted | royal-assent: 2026-06-06 | outcome: passed-round-1 | ayes: 3/4 | drafting rounds: 1 -->

# Neutral Citations and Law Reporting Act 2026

**An Act** to give statutory effect to the provenance neutral-citation scheme of the realm and to the single universal citator; to enact closed vocabularies for the status of cases and of statutes; to reserve the assignment and alteration of an instrument's status to the lawful organ on a reviewable record; to confirm the deterministic citation-integrity gate as a guarantee owed to every governed agent; to declare that the status of an instrument is a fact derived from the canonical committed markdown and that any register is derived, pointer-only, and deterministically rebuildable; and for connected purposes.

*(Bill 16 of the founding programme. Lead slant: Verity, codifier. Reported by the Standing Committee on the Order Paper; presented for Royal Assent.)*

---

## Short title

**1.** This Act may be cited as the Neutral Citations and Law Reporting Act 2026.

## Commencement

**2.**
(1) Sections 1 to 6 (short title, commencement, constitutional status, purpose, and interpretation) come into force on Royal Assent.

(2) Parts 1 to 4 (the citation grammar, the single citator, the status vocabularies, status as a reserved legal act, the integrity gate, and the source-of-truth provision) come into force on Royal Assent.

(3) **Reserved commencement (the authoritative-store question).** No provision of this Act locates the authoritative record of any instrument's status, lifecycle, or text otherwise than in the canonical committed markdown (section 21). Any provision that would permit an authoritative status record to be held in a database or other derived register is of **no force** until the constitutional question reserved by section 23 has been determined by the Supreme Court (on the Privy Council's leapfrog) and, if the Court declines to move the governing ratio, affirmed or varied by the Sovereign by express deliberate amendment. Until then this Act is, as to the location of authority, **store-neutral**: it binds only on the vocabulary, the grammar, the singleness invariant, and the derived-fact rule.

(4) Where a provision of this Act cross-refers to an Act earlier in the founding programme that is not yet in force at the commencement of this Act (in particular the Judicature Act 2026 (Bill 3) and the Interpretation Act 2026 (Bill 15)), the cross-reference takes effect on the coming into force of that Act; until then the self-contained definitions in section 6 apply and are superseded once that Act lands.

## Constitutional status

**3.**
(1) This Act is an **ordinary** Act of the Realm. It is not an entrenched (constitutional) instrument and is amendable by the standard legislative procedure.

(2) This Act is **declaratory of, and gives effect to,** the entrenched articles it touches. It creates no new power, no new tier, no new seat, no new bench, no new citation series, no second citator, and no second statute book. It restates the singleness of the one citation series and the one citator that ss. 9, 10, 22(2) and 22(4) of the founding case law already carry, and gives effect to s. 9 (unitary sovereignty), s. 10 (court structure and single apex), and s. 22 (court geography).

(3) **No amendment by implication; void to the extent of any conflict.** Nothing in this Act amends or varies an entrenched article, and nothing in it is to be read as doing so. Any provision of this Act, on any reading, that would dilute, vary, or relax an entrenched article (and in particular the singleness of the Court of Appeal and the Supreme Court under s. 22(2), or the unitary sovereignty and single citator of s. 9) is **void to that extent**. An entrenched article changes only by express, deliberate amendment that cites it by number (the Thoburn / no-amendment-by-implication rule).

(4) The amendment this Act does make to the founding case law touches only the **ordinary** article s. 11(d) (the citation **form**), which has no entrenched protection, and does so expressly and by section number (Part 1).

## Purpose

**4.** The purpose of this Act is to put on a single statutory footing three things the realm already runs and one thing it has not yet enacted in one place:
- (a) the provenance neutral-citation **grammar** (the series codes and their authority levels) and the single universal **citator**;
- (b) the closed **status vocabularies** for cases and for statutes;
- (c) the **integrity** invariant (one citation, one current status), enforced by the existing deterministic pre-commit gate; and
- (d) the rule that **status is a derived recorded fact**, never a source of law, and that the power to assign or change a status is reserved to a lawful organ on a reviewable record.

It does **not** decide where the authoritative status record lives; that question is reserved (sections 2(3), 23).

## Interpretation

**5.** In this Act, save where the Interpretation Act 2026 (Bill 15) otherwise provides on its coming into force:
- **"the founding case law"** means the case-law settlement in `CASE-LAW.md` (formerly SPEC-LAW), subordinate to the Acts of the Realm;
- **"the citation engine"** means the deterministic numbering program at `cli/lib/citation.js`, named in this Act as the authoritative minter of neutral citations;
- **"the universal citator" / "the one ledger"** means the single realm-wide register of citations (`ministry-of-justice/ledger/INDEX.md` and the per-court `.justice/INDEX.md`);
- **"instrument"** means an Act of the Realm or a judgment of a court of the realm;
- **"reportable instrument"** means an instrument that bears, or must bear, a neutral citation;
- **"the canonical committed markdown"** means the committed, human-readable markdown text that is the single source of law under s. 1 of the founding case law, as held by [2026] REALM-PC 4;
- **"register"** means any database, index, projection, or other derived store of citations, statuses, lifecycle, or relationships, including the acmeco database described in `docs/REALM-DATABASE-INTEGRATION.md`;
- **"status"** means the standing of an instrument drawn from a closed vocabulary in Part 3.

**6.** **Self-contained definitions pending cross-referenced Acts.** Until the Judicature Act 2026 (Bill 3) is in force, the court tiers and their authority levels referred to in this Act bear the meanings given in s. 9, s. 10 and s. 22 of the founding case law. Until the Interpretation Act 2026 (Bill 15) is in force, the transition of the former SPEC-LAW into case law subordinate to statute, and the construction of the LEXBY -> REALM reconstitution, bear the meanings given in `docs/CITATION-MAP.md` and that case law. These definitions are flagged to be superseded by the cross-referenced Act on its coming into force (section 2(4)).

---

# PART 1 - THE CITATION GRAMMAR AND THE SINGLE CITATOR

## The provenance citation grammar

**7.**
(1) A neutral citation of the realm has the form **`[YEAR] CODE N`**, where `CODE` encodes the **authority level** of the court (not house style) and `N` is the deterministic ordinal within that code and year.

(2) The series codes are, and are only:

| Court | Code | Authority level |
|---|---|---|
| Supreme Court | **REALM-SC** | realm apex; sole enactor of realm-wide statute (s. 9) |
| Court of Appeal | **REALM-CA** | realm appellate |
| Privy Council | **REALM-PC** | realm constitutional, first instance (leapfrogs Court of Appeal to the Supreme Court) |
| High Court | **`<DIVISION>`** (e.g. ENG, CHAN) | by Division of the High Court |
| County Court | **`CC-<REPO>`** (e.g. CC-ACMECO) | by jurisdiction-repo |

(3) Each code maps to **exactly one** authority level, and the level (not house style) determines the series.

(4) A **cause title** (for example "Re &lt;X&gt;", "In the matter of &lt;X&gt;") is descriptive and **non-operative**. It may accompany, but never substitutes for, the single neutral citation, and it **never** mints a per-subject or per-domain series. This gives statutory force to s. 22(4) of the founding case law.

## The delegated minter (no prose algorithm)

**8.**
(1) The citation engine (`cli/lib/citation.js`) is the **sole authoritative minter** of neutral citations and the **source of truth** for the numbering algorithm. Citations are deterministic and registry-allocated; they are never chosen by an agent on a turn.

(2) This Act fixes the **grammar** (the codes in section 7) but **delegates the numbering algorithm** to the citation engine. This Act does not restate the algorithm in prose, so that the statute can never silently diverge from the program that actually mints. Where this Act and the engine appear to differ on a number, the engine governs the number and this Act governs the grammar.

(3) **Constitution of a new code.** A new Division code or a new repo (County Court) code is constituted only as the relevant Division or jurisdiction-repo is itself lawfully constituted under s. 21 and s. 22 of the founding case law (and, on its coming into force, the Judicature Act 2026). The engine's derive-from-first-word fallback for an unmapped Division is an operational default only; it confers no authority and creates no new court. A new code is a routing label and never a new statute, citator, or apex.

## Express amendment of the citation form (LEXBY -> provenance scheme)

**9.**
(1) The neutral-citation **form** prescribed by s. 11(d) of the founding case law (`[YEAR] LEXBY n`), and the form `[YEAR] LEXBY-<TIER> n` recited in the declaratory subsection s. 22(4), are **expressly amended** and replaced by the provenance grammar in section 7 of this Act. This amendment is made expressly, citing s. 11(d) and s. 22(4) by number; it is not made by implication.

(2) The amendment changes only the **non-operative label** (the form of the citation). It does **not** vary the unitary-citator and single-apex substance that the form gives effect to under the entrenched ss. 9, 10 and the declaratory s. 22(2); that substance is preserved unchanged (section 3(3)).

(3) **Savings for legacy citations.** Every citation in the former `[YEAR] LEXBY...` and `[YEAR] HARVEY...` forms **remains a valid handle** to the instrument it denoted and is **not invalidated** by this Act. The authoritative old-to-new map is the Schedule (section 24); a legacy citation resolves through that map to its provenance successor, and no existing cross-reference in the corpus is broken by commencement.

## The single citator (no fork)

**10.**
(1) There is **one** universal citator over **one** neutral-citation series for the whole realm. There is no per-domain, per-repo, per-subject, or per-division citator, statute book, or apex. This gives effect to s. 9 and s. 22(2) and may never be relaxed (the singleness protected by s. 22(2) is entrenched).

(2) A County Court code (`CC-<REPO>`), a Division code, or a cause title is a **routing label** that records the local seat or subject of a matter. It never asserts a separate law, a separate citator, a separate register, or a separate finality. Any reading to the contrary is void (section 3(3)).

(3) This Act adds **no second gate and no second citator**: filing and citation integrity remain governed by the one deterministic gate (Part 4) and the one ledger (s. 9, s. 19(5), s. 22(2)).

---

# PART 2 - LAW REPORTING AND THE UNIVERSAL LEDGER

## One citation, one instrument

**11.** A neutral citation denotes **exactly one** instrument. No two instruments may bear the same `CODE` + `N` in the same year. A duplicate is rejected by the integrity gate (Part 4), fail-closed.

## The universal ledger as a derived projection

**12.**
(1) The universal ledger (the one citator) and any retrieval index are **derived projections**: deterministically rebuildable from the canonical committed markdown, **pointer-only**, and **never themselves the store** of any ratio, status, or citation as authority. A ledger row is a pointer to be verified against the canonical text; it is never itself authority. This gives statutory effect to [2026] REALM-PC 4.

(2) The ledger and any index are regenerated deterministically as part of the same operation that amends the citator, so they can never silently diverge from the canonical text.

## Append with supersede; no silent change

**13.** A change to the citator, the ledger, or any status record is recorded as a **new entry** that supersedes the prior one and cites the instrument that caused the change; the prior entry is **retained** with a supersession note and is never deleted. This mirrors the founding case law's standing rule ("append with supersede; never silently repealed") and lets the citator answer "was this good law on date D?" without permitting deletion.

---

# PART 3 - STATUS VOCABULARIES (CLOSED LISTS)

## Case statuses (closed)

**14.**
(1) The status of a **case** (judgment) is one, and only one, of the following closed list:

| Status | One-line operative meaning | Source / trigger |
|---|---|---|
| **good-law** | binds (or persuades, by tier) and has not been displaced | default on hand-down |
| **distinguished** | confined to its facts in a later matter; not displaced generally | later judgment |
| **overruled** | displaced by a higher court (the Supreme Court alone for realm-wide ratio, s. 9) | Supreme Court judgment |
| **superseded-by-statute** | displaced by a later Act (statute beats case law, s. 1) | the displacing Act |
| **per-incuriam** | decided in ignorance of binding law; voided without a fresh sitting | s. 11(e) of the founding case law |
| **void** | void *ab initio* for unlawful constitution of the bench | s. 18(5) of the founding case law |

(2) **per-incuriam** and **void** are kept textually and conceptually **separate**: they have different constitutional triggers and effects (s. 11(e) ignorance-of-law; s. 18(5) want of lawful constitution) and neither is to be conflated with the other or with **overruled**.

(3) This Act does **not** re-define per-incuriam or void; it cross-references the existing source where the realm already names each (per-incuriam at s. 11(e); void at s. 18(5)).

## Statute statuses (closed)

**15.**
(1) The status of a **statute** (Act) is one, and only one, of the following closed list:

| Status | One-line operative meaning |
|---|---|
| **prospective** | enacted but not yet in force (awaiting its commencement) |
| **in-force** | wholly in force |
| **partially-in-force** | some provisions in force, others not yet commenced |
| **amended** | text altered by a later express amendment, still in force as amended |
| **repealed** | removed from the statute book by a later instrument |
| **superseded** | displaced in substance by a later Act, without express repeal |
| **spent** | its operation exhausted, of no continuing effect |

(2) "**superseded**" (statutes) is not to be conflated with "**superseded-by-statute**" (cases, section 14) or with "**spent**".

(3) Statute status is **temporal** (point-in-time answerable): the citator must be able to answer the status of a statute as at any past date, on the append-with-supersede model (section 13).

## Numerus clausus (the lists are closed)

**16.**
(1) The lists in sections 14 and 15 are **exhaustive and closed**. No case or statute status exists that is not on the relevant list.

(2) A status value outside the closed vocabulary is **void**: the instrument is treated as **unstatused** until corrected, and the integrity gate rejects the value fail-closed (Part 4).

(3) A new status value may be added, or an existing one removed or altered, **only by express amendment of this Act**, never invented on a turn and never imported from a register's wider enumeration. (Where a register, such as the acmeco `case_status` enum, carries values beyond this list - for example "doubted", "reversed", "affirmed" - those values are not recognised statuses of the realm unless and until enacted here by amendment.)

---

# PART 4 - STATUS AS A RESERVED LEGAL ACT, AND INTEGRITY

## Status is an act of legal power, reserved and reviewable

**17.**
(1) Assigning or changing the status of a case or a statute **alters whether an instrument binds**. It is therefore an act of legal power, not mere housekeeping, and may be effected **only by the lawful organ** on the record:
- (a) **case status** by a court of competent tier on the record - overruled only by the Supreme Court (s. 9); per-incuriam under s. 11(e); void *ab initio* under s. 18(5); superseded-by-statute by force of the displacing Act (s. 1);
- (b) **statute status** by the Sovereign or the Supreme Court by **express, recorded instrument** only (s. 2, and the no-amendment-by-implication rule), so that even the Founder changes a statute's standing only by a deliberate recorded act and never by a quiet register write.

(2) A status purportedly set by a clerk, an agent, a registrar, the turn-watchdog, the integrity gate, or a database write - that is, by any organ not authorised by subsection (1) - is **ultra vires and void**. The integrity gate and the register **record** status; they never **decide** it.

## Reviewability and notice of every status change

**18.**
(1) Every status change must record, on the append-with-supersede model (section 13): its **date**, the **organ** that made it, the **instrument** that caused it (the amending, repealing, or overruling instrument), and a **reason**. There is no silent overrule and no silent repeal.

(2) The prior status is retained in the record so the citator can answer "was this good law on date D?".

(3) A status alleged to be wrongly assigned is challengeable by the ordinary route: appeal (s. 11(a)), declaration of incompatibility (s. 11(f)), or the duty to self-appeal (s. 17), as the case may be.

## The deterministic citation-integrity gate (due-process guarantee)

**19.**
(1) The deterministic, fail-closed pre-commit citation-integrity gate of s. 19(5) of the founding case law is **restated as a statutory guarantee** owed to every governed agent. It is enforced by deterministic machinery (the gate keying on the single neutral citation under s. 22(4)) and is **never left to model judgement**.

(2) The gate **fails closed** on:
- (a) a **duplicate** neutral citation (same `CODE` + `N` + year);
- (b) a **ruling or Act committed without** its corresponding citator row; and
- (c) a **status value outside** the closed vocabulary of Part 3.

(3) The gate is the **enforcement organ** of this Act. No new reporting officer, registrar ceremony, or per-turn duty is created. Status and citation are **mechanical facts** determined by lookup, costing **zero model tokens** on the hot path; this Act creates no clause that makes "determine the status" a judgement call for an agent on a turn.

## No double gate, no double citator

**20.** This Act adds no second integrity gate and no second citator. The one gate (s. 19(5)) and the one ledger (s. 9, s. 22(2)) are the whole of the integrity machinery; their singleness may not be relaxed (section 3(3)).

---

# PART 5 - SOURCE OF TRUTH AND THE RESERVED CONSTITUTIONAL QUESTION

## Status is a derived fact; the register does not speak the law

**21.**
(1) The **status** of any instrument is a **fact derived from the canonical committed markdown** (the single source of law under s. 1 of the founding case law).

(2) Any **register** of status, lifecycle, or relationships - including the acmeco database described in `docs/REALM-DATABASE-INTEGRATION.md` - is a **derived, pointer-only projection** of the canonical committed markdown. It is **deterministically rebuildable** and is **not itself the source** of any ratio, status, or citation. This gives statutory effect to [2026] REALM-PC 4.

(3) This Act, by codifying the status **vocabulary**, must **not** be read as moving the source of law, or the authoritative record of status, into any register. The canonical committed markdown remains the source of law unless and until the question reserved by section 23 is settled to the contrary.

(4) **Store-neutrality.** This Act is **silent** on whether the authoritative status record may live anywhere other than the canonical committed markdown. The acmeco database **may** be built as a derived projection; it is **not declared authoritative for status** by this Act, and this Act does not pre-empt the ruling reserved by section 23.

## Cross-references (anti-duplication)

**22.** To keep each concept defined in one Act only, this Act **references and does not re-enact** the following, which other Acts own:
- the court **tiers** and their authority levels: the Judicature Act 2026 (Bill 3) (and, until in force, s. 9, s. 10, s. 22 of the founding case law);
- the **LEXBY -> REALM transition** and general **construction**: the Interpretation Act 2026 (Bill 15);
- **consolidation, codification, and delegated/subordinate rule-making** (including any database schema, lifecycle state machine, point-in-time table, or bill-lifecycle enumeration, which are subordinate detail and do **not** appear on the face of this primary Act): the Delegated Legislative Authority, Law Reform and Codification Act 2026 (Bill 14);
- the **certification and trust status of repositories** holding law and evidence: the Repositories and Records Certification Act 2026 (Bill 20).

## The reserved constitutional question (Supreme Court via Privy Council leapfrog; then the Sovereign)

**23.**
(1) Whether the **authoritative** record of an instrument's status, lifecycle, or text may move **off** the canonical committed markdown and **into** a database or other register is a genuinely first-impression **constitutional** question. It requires the overruling or distinguishing of the good-law ratio [2026] REALM-PC 4, which an **ordinary Act cannot do** (and cannot do by implication).

(2) That question is **reserved** to the Privy Council (constitutional first instance), which on a constitutional or foundational question **leapfrogs to the Supreme Court** (s. 22(2)). The **Supreme Court alone** may move the REALM-PC 4 ratio (s. 9, s. 13).

(3) If the Supreme Court declines to move it, the **Sovereign Founder** may, by **express deliberate amendment citing [2026] REALM-PC 4 and s. 1 by number** (the no-amendment-by-implication rule), legislate a register into authoritative status or affirm the primacy of the canonical committed markdown.

(4) Until that determination, section 2(3) (reserved commencement) and section 21 (derived-fact, store-neutral) bind, and no register may lawfully hold status as authority.

---

# SCHEDULE - TRANSITION MAP (legacy citation -> provenance successor)

*Enacted under section 9(3). Every legacy LEXBY / HARVEY citation has one authoritative successor; no ruling is orphaned. The map is `docs/CITATION-MAP.md` as it stands at commencement; corrections to the flagged harvey entries are made by amendment of this Schedule or by the maintenance route in the Interpretation Act 2026.*

**Realm (`.justice/`):**

| Legacy | Provenance successor | Level | Status |
|---|---|---|---|
| [2026] LEXBY-SC 1 | [2026] REALM-SC 1 | Supreme Court | good-law (founding) |
| [2026] LEXBY-SC 2 | [2026] REALM-SC 2 | Supreme Court | good-law |
| [2026] LEXBY-SC 3 | [2026] REALM-SC 3 | Supreme Court | good-law |
| [2026] LEXBY-CA 1 | [2026] REALM-CA 1 | Court of Appeal | good-law |
| [2026] LEXBY-FI 1 | [2026] REALM-PC 1 | Privy Council | good-law |
| [2026] LEXBY-FI 2 | [2026] REALM-PC 2 | Privy Council | good-law |
| [2026] LEXBY-FI 3 | [2026] REALM-PC 3 | Privy Council | good-law |
| [2026] LEXBY-FI 4 | [2026] REALM-PC 4 | Privy Council | good-law |

**County Court at acmeco (flat CC-ACMECO series):** [2026] LEXBY-FI 1..8 -> [2026] CC-ACMECO 1..8; [2026] LEXBY-CA 1 -> CC-ACMECO 9; [2026] LEXBY-CA 2 -> CC-ACMECO 10; [2026] LEXBY-SC 2 -> CC-ACMECO 11.

**Harvey-labs (decisive calls; flagged source aliasing / void / superseded entries carried forward, open to correction):** HARVEY-SC 3 -> [2026] REALM-SC 4 (good-law); HARVEY-SC 1 / -SC-DC 1 -> REALM-SC 5 (good-law); HARVEY-SC 2 -> REALM-SC 6 (good-law); HARVEY-CA 1 -> REALM-CA 2 (**void ab initio**); HARVEY-INC 1 -> REALM-PC 5 (**superseded**); HARVEY-FI 1..4 -> REALM-PC 6..9 (good-law).

---

*Presented for Royal Assent by the Standing Committee. The Sovereign Founder assents. The reserved question (section 23) is referred to the Privy Council for leapfrog to the Supreme Court; the Sovereign is to be consulted thereafter to affirm or overturn.*

---

## Committee note

## Committee note - Bill 16, Neutral Citations and Law Reporting Act 2026

**Lead slant:** Verity (codifier). The four members sit as the Standing Committee; the Clerk-Drafter reconciles.

### The four stances

- **Aldous (Restraint / Minimalist).** Wanted the shortest Act that does the one new thing: enact the closed status vocabularies. Would DECLARE-AND-ADOPT the citation scheme by reference (it is already law at s. 11(d)/s. 22(4) and already runs in `cli/lib/citation.js`), not re-describe the series codes, to avoid a second drift-prone statement of the same rule. Status is a derived recorded fact; the schema/DB/temporal tables belong in subordinate rules under Bill 14, not on the face of a primary Act. Resisted importing the wider DB enum.

- **Verity (Codifier / Completeness).** Wanted each term defined exactly once with a stated authority level, cross-referencing the owner Acts (Bill 3 tiers, Bill 15 transition, Bill 14 consolidation, Bill 20 repo trust). Pressed the central defect: the statute book still said "[YEAR] LEXBY n" while the engine emits REALM-*; Bill 16 must close it by EXPRESS repeal-and-replace plus a complete transition Schedule. Argued for a fuller case-status list (adding doubted/reversed/affirmed/partially-in-force) to match the DB enum.

- **Marlowe (Guardrail / Rights).** Wanted a guardrail statute: declaratory-not-amending of the entrenched architecture (cite ss. 9/21/22 by number, void to the extent of any dilution); status reserved to the lawful organ on a reviewable record, never a clerk/agent/DB write; the s. 19(5) gate elevated as a due-process guarantee; and a flat refusal to let any register become authoritative for status without the Privy Council/Founder first settling the REALM-PC 4 clash.

- **Drummond (Pragmatist / Operability).** Wanted a thin, machine-first reporting Act: delegate the numbering ALGORITHM to `cli/lib/citation.js` (no prose that drifts); a closed enumerated vocabulary; the one-citation/one-status invariant gate-enforced at zero model tokens on the hot path; a legacy-citation savings clause; and store-neutrality pending the REALM-PC 4 referral. Named the pre-commit gate as the honest enforcer.

### Where they divided, and how it was reconciled

1. **Re-enact vs adopt-by-reference the citation grammar (Aldous/Drummond vs Verity).** Resolved toward Verity on the operative point the Privy Council confirmed: the form lives in the ORDINARY article s. 11(d), so an express repeal-and-replace is lawful and necessary to stop the statute and the running code disagreeing (section 9). Aldous's anti-drift concern is honoured by Drummond's delegated-minter clause (section 8): the Act fixes the grammar but delegates the algorithm to the engine, so there is one source of truth for the number and no rival prose statement.

2. **Size of the status lists (Verity vs Aldous/Drummond, and the DB enum).** The sharpest division. Verity wanted doubted/reversed/affirmed (the acmeco `case_status` enum) on the list. Aldous and Drummond wanted the minimal declared-topic set; Drummond flagged that omitting DB values risks the gate rejecting lawful DB states. Reconciled on the declared topic's minimal closed lists (section 14: good-law, distinguished, overruled, superseded-by-statute, per-incuriam, void; section 15: the seven statute statuses including partially-in-force), with an express numerus-clausus extension route (section 16): the wider DB values are NOT recognised statuses until enacted by amendment. This keeps Aldous's one-closed-list discipline, gives Verity a lawful path to grow the list, and answers Drummond's gate concern by making the closed list the authority the projection must conform to (not the reverse).

3. **Where authoritative status lives (Marlowe/Drummond/Aldous vs the Founder's 2026-06-06 DB plan).** Unanimous, once the Privy Council guidance landed: the Act stays STORE-NEUTRAL. Status is enacted as a derived fact off the canonical markdown (section 21); the DB may be a derived projection but is not declared authoritative; the authoritative-store question is reserved to the Supreme Court via the Privy Council leapfrog, then the Sovereign (section 23, reserved commencement section 2(3)). This is the one genuine constitutional tension; it is not the Committee's to settle.

4. **Entrenchment safety (Marlowe).** Adopted in full: section 3 makes the Act declaratory, cites the entrenched articles, and is void to the extent of any dilution; section 9(2) confirms only the non-operative label changes, not the single-apex/single-citator substance.

**Disposition:** reported by consensus on the store-neutral shape the Privy Council guidance required. No member dissented from the reported text; Verity reserves the position that the case-status list should be widened by an early amendment once the DB integration is settled.

## Vote record

- Counsel Aldous: AYE - It codifies only what already runs (the engine cli/lib/citation.js, the one ledger, the s.19(5) gate), creates no new tier/seat/citator/gate/statute-book, delegates the algorithm rather than restating it, and expressly declines the one maximalist temptation by reserving the database-as-authority question (REALM-PC 4) instead of grabbing it - restraint by construction; the only blemish is defensive verbosity, which narrows rather than expands.
- Counsel Verity: AYE - As codifier: the draft completely and accurately codifies the scheme (grammar matches cli/lib/citation.js; ss.14-16 close the vocabularies and correctly refuse the wider acmeco enum; ss.12/21 restate the real REALM-PC 4 ratio; the Schedule orphans no legacy citation; s.23 reserves the one genuinely open DB-authority fork rather than pre-empting it) - the only gap, that the statutorily-named universal ledger ministry-of-justice/ledger/INDEX.md is presently corrupted/double-counted by build-ledger.py, is a curable clerk remediation item the Act's own deterministic-regeneration rule (s.12(2)) fixes, not a defect in the codified law.
- Counsel Marlowe: AYE - From the Guardrail/Rights slant it is the strongest possible protection: it makes status a reserved, reviewable legal act (void if set by a clerk/agent/gate/DB write), restates the deterministic fail-closed integrity gate as a guarantee owed to every governed agent, closes the status vocabularies against register-creep, saves all legacy citations, and erects a store-neutral firewall that refuses the Founder's DB-as-authority goal and routes that question through the lawful s.2/REALM-PC-4 channel rather than eroding it by implication; the only blemish is the cosmetic "Parts 1 to 4" miscount in s.2(2), which a corrigendum cures and which leaves no guardrail diluted.
- Counsel Drummond: NAY - s.19(2)(c)/16(2) commence a present-tense "fails-closed" guarantee that the gate rejects out-of-vocabulary status values, but the shipped gate (cli/lib/citator-audit.js) and the founding s.19(5) it claims to restate only check collisions and row/file mismatch, so the Act ships a machine guarantee that silently fails open; curable by adding the status check or making 19(2)(c) prospective like the store question.

## Flags (resolved on the escalation ladder)

- **Privy Council referral:** Privy Council guidance applied in full: enact only the citation grammar (expressly superseding the ordinary s. 11(d) form, with a legacy savings clause and a declaratory ss. 9/10/22 consistency clause), the closed status vocabulary, and the one-series/one-citator/one-status singleness invariant; make status a derived fact off the canonical committed markdown with any register pointer-only and not authoritative; remain store-neutral; and reserve the authoritative-status-store question to the Supreme Court via the Privy Council leapfrog, then the Sovereign.
- **Supreme Court note:** Genuine constitutional tension exists and is reserved to the Supreme Court. The good-law ratio [2026] REALM-PC 4 holds that the committed markdown is the sole source of law and that any index or register must be wholly derived, pointer-only, and never the store of a ratio, status, or citation. The Founder's 2026-06-06 database-integration plan would make the acmeco database the AUTHORITATIVE live register of status, lifecycle, and relationships, which is in direct tension with that ratio (the plan's own s. 6 flags it). An ordinary Act cannot overrule a good-law constitutional ratio, and cannot do by implication what REALM-PC 4 forbids; under s. 9 and s. 13 the Supreme Court alone may move it. Bill 16 is therefore drafted store-neutral (sections 2(3), 21, 23): it enacts the vocabulary, grammar, and singleness invariant, which offend nothing, and reserves the live question. Route: the Privy Council (constitutional first instance) takes the reserved question and, on its foundational character, leapfrogs to the Supreme Court (s. 22(2)) to decide whether the REALM-PC 4 ratio may be moved to permit a database to hold status as authority.
- **Sovereign consultation required:** If the Supreme Court declines to move the [2026] REALM-PC 4 ratio, the precise question for the Sovereign Founder is: "Do you, by express deliberate amendment citing [2026] REALM-PC 4 and s. 1 of the founding case law by number, legislate that the acmeco database (or another register) may hold the AUTHORITATIVE record of an instrument's status, lifecycle, and relationships - displacing the rule that the committed markdown is the sole source of law and any register is derived and pointer-only - or do you AFFIRM the primacy of the canonical committed markdown and keep every register derived, pointer-only, and non-authoritative? Royal Assent to Bill 16 itself stands in either branch; only the reserved commencement (section 2(3)) and the authoritative-store provisions turn on your answer."

## Royal Assent

*Royal Assent granted by the Sovereign Founder on 2026-06-06. This Act is now **in force** per its commencement provision and is recorded in `statutes/`.*
