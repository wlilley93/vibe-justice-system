<!-- Bill 3 of 25 - Order Paper of the Realm. Drafted by the Standing Committee. -->
<!-- status: enacted | royal-assent: 2026-06-06 | outcome: passed-round-1 | ayes: 4/4 | drafting rounds: 1 -->

# Judicature Act 2026

**An Act to constitute the courts of the realm; to confer and allocate their jurisdiction; to re-enact as primary statute the court structure formerly carried in the founding case law; to secure the independence of the judiciary on an interim footing pending the Judicial Independence and Lord Chancellor Act; and for connected purposes.**

*(Bill 3 of the founding programme. Lead slant: Verity. Drawn by the Clerk-Drafter on the four memos of the Standing Committee and the Reference of the Privy Council of the Realm on Bill 3.)*

---

## Short title

**1.** This Act may be cited as the Judicature Act 2026.

## Commencement

**2.**
(1) Sections 1 to 9, and Part III (the Supreme Court, the Court of Appeal, and the County Court as the first-instance hearing-centre), come into force on Royal Assent, their machinery already running (`court/workflows/supreme-court.js`, `court/workflows/court-of-appeal.js`, `court/workflows/first-instance.js`, and the deterministic pre-commit citation-integrity gate, CASE-LAW s. 19(5)).

(2) The Privy Council (section 13), the High Court Divisions and their Lists (section 14), and a County Court constituted as a standing local bench at a repo beyond the existing first-instance fast-path and workflow (section 15(3)), come into force only on a commencement order made under the Civil Procedure Code 2026 (Bill 4) and the Commencement and Transitional Provisions Act 2026 (Bill 25), once the routing and workflow that instantiate each body have shipped. Until then they have prospective effect only and are not dead-letter: no agent may be told to walk through a door that does not yet exist.

(3) Nothing commenced under this Act disturbs any ruling already handed down; existing rulings continue as good law under their re-cited provenance citations (CASE-LAW s. 22(4); the Citation Map; Bill 16).

## Constitutional status

**3.**
(1) This Act is made by the Sovereign in Parliament under and subordinate to the Acts of Union 2026. It is supreme over the founding case law (CASE-LAW; formerly SPEC-LAW), which continues to govern where this Act is silent and which the courts develop in the gaps (CASE-LAW s. 1).

(2) This Act **codifies and gives statutory footing to** the entrenched and declaratory court provisions of the founding case law, naming each by number: the unitary sovereignty rule (s. 9), the court structure and tier ladder (s. 10), rule-based progression and the leapfrog (s. 13), subject-matter jurisdiction (s. 14), odd benches and the judgment of the Court (s. 18), researched intake and the integrity gate (s. 19), the leapfrog certificate (s. 20), divisions over one spine (s. 21), and court geography (s. 22). Where it re-enacts an entrenched article it does so **faithfully**, declaring the corresponding case-law version to cease independent effect, so that there is exactly one statement of the court structure and no two descriptions may drift and collide (Schedule 1).

(3) **No amendment by implication.** This Act amends no entrenched article (s. 9, s. 10, s. 13, s. 14) except where a section expressly cites that article by number and deliberately amends it. Any reading of this Act that would add a tier, seat, or apex, relax the singleness of the apex (s. 22(2)), create a standing bypass of mandatory progression (s. 13), or otherwise vary an entrenched article by implication is **void to that extent** (s. 1; the Amendment Procedure; *Thoburn*). This Act effects no such amendment; it gives the entrenched settlement statutory footing and elaborates it.

## Purpose

**4.** The purpose of this Act is to constitute the courts of the realm, to confer and allocate their jurisdiction at the level of principle, to secure the independence of the judiciary on an interim footing, and to re-enact the settled court geography as primary statute. Nothing in this Act extends any court's jurisdiction or creates any power beyond constituting the courts and conferring jurisdiction; procedure, remedies-mechanics, enforcement, full judicial independence, standing, due process, and the citation scheme are reserved to the sibling Acts named in section 23 and are not enacted here.

---

# PART I — INTERPRETATION

**5. Definitions.** In this Act, each load-bearing term is fixed once and used consistently throughout, and is incorporated by reference into the later bills that depend on it —

- **"the realm"** means Agent Universe, the super-repo, organised as a Ministry of Justice (governance only) over an executive ministry holding the operational jurisdiction-repos (CASE-LAW s. 22(3); REALM-TOPOLOGY);
- **"court"** means an organ of the one unitary judiciary constituted by this Act; there is no second judiciary, no domain court, and no competing sovereign (s. 9, s. 21(2));
- **"the apex"** means the Supreme Court, the single central court that alone enacts realm-wide statute (s. 9, s. 22(2));
- **"hearing-centre"** means a local seat of the one judiciary that applies the one CASE-LAW, records jurisdiction-local precedent only, owns no statute book, apex, or separate citation series, and may never enact realm-wide statute (s. 22(1), s. 22(4));
- **"Division"** means a domain routing of the High Court to a specialist bench over the one procedural spine, being a hearing-centre and not a domain court (s. 21(1), s. 22(4));
- **"List"** means a named sub-routing within a Division to a specialist bench, of the same non-operative character as a Division;
- **"County Court at a repo"** means the local first-instance hearing-centre of the one judiciary, one per jurisdiction-repo (s. 22(1));
- **"leave to appeal"** means permission to appeal granted or refused by an independent leave-judge under CASE-LAW s. 11(a) and s. 19(3) (mechanics in Bill 4);
- **"leapfrog certificate"** means the Principal's express, recorded, reasoned, vires-reviewable executive certificate that is the sole bypass of mandatory progression (s. 13, s. 20);
- **"ratio"** means the reasoning necessary to the decision, which alone binds; **"obiter"** is all else, which is at most persuasive (s. 11(e));
- **"realm-wide statute"** means law binding throughout the realm, enacted only by the Sovereign in Parliament or by the single Supreme Court elevating a local ratio (s. 9); **"jurisdiction-local case law"** means a hearing-centre's logged rulings, binding within its repo and persuasive elsewhere until the apex elevates a ratio (s. 9, s. 22(1));
- **"the Citation Map" / the provenance scheme** means the single neutral-citation series governed by the Neutral Citations and Law Reporting Act 2026 (Bill 16): REALM-SC, REALM-CA, REALM-PC, the Division codes CHAN (Chancery/Legal) and ENG (Engineering) with List, and CC-<repo>. This Act names the courts and leaves the citation form to Bill 16 (section 22).

---

# PART II — THE ONE JUDICIARY, ITS INDEPENDENCE, AND ITS SUPREMACY OF STATUTE

**6. One unitary judiciary; subordination and supremacy.**
(1) The realm has one unitary judiciary, applying one supreme CASE-LAW with jurisdiction-local case law (s. 9). The courts constituted by this Act are its organs; none is a court of separate sovereignty.
(2) Statute is supreme over case law; on conflict, statute prevails and the case law is void to the extent of the conflict (s. 1). The courts interpret statute and develop case law only in its silence (s. 7).
(3) The former SPEC-LAW is subordinate case law (CASE-LAW preamble; Bill 15).

**7. Single central apex; the apex belongs to the realm, not the ministry.** *(Codifying s. 9 and s. 22(2).)*
(1) The Court of Appeal and the Supreme Court are **single and central**, the sole organs of appeal and the sole enactors of realm-wide statute. Their singleness and centrality **may never be relaxed** (s. 22(2)). The power to enact realm-wide statute is **non-delegable** from the single Supreme Court.
(2) No Division, List, County Court, ministry, department, super-repo, or executive officer is a court of separate sovereignty, owns a statute book or apex, or mints a parallel citation series; no per-department or per-repo appellate court or apex is permitted (s. 9, s. 22(2)).
(3) The Court of Appeal and the Supreme Court sit at the level of the Ministry of Justice but are **organs of the realm, not of that ministry**. No ministry, department, super-repo, or executive officer may instruct, constitute, re-size, fund-gate, or stay them; and the Ministry of Justice may never become the instrument by which the executive or the Founder edits the law the courts are to be judged against (*harvey-SC-2* separation-of-powers dicta).

**8. Judicial independence (interim footing pending Bill 11).**
(1) The independence of every court and judge from the executive (the office of Prime Minister) and from the Sovereign's executive office is secured. The Sovereign's legislative office (Parliament) is expressly distinct: the Sovereign may make or unmake law by express, deliberate due process (s. 2), but the assenting and executive offices may not, by any act, instruct, stay, pack, or reconstitute the judiciary into which the Sovereign assents.
(2) **No reconstitution in consequence of a decision.** No judge may be removed, suspended, reassigned, or have a bench re-sized, packed, dissolved, or added to, in consequence of, or to influence, any decision. The bench constitution fixed by s. 10 and s. 18 is the **exhaustive deciding membership** and may not be enlarged or reduced by ministerial, executive, or Founder act (s. 18(1), s. 18(5)).
(3) Independence is stated, so far as this Act secures it, as **enforceable machine facts**, not aspiration: the independent leave-judge is selected at random and is never a judge who sat on the decision under challenge (s. 19(3)); the deliberating bench has no access to Lexby's preference (s. 3); and the turn-watchdog may not adjudicate, score, gate, sanction, or punish (s. 19(4)).
(4) This section is an **interim** guarantee. It does not pre-empt or foreclose the Judicial Independence and Lord Chancellor Act 2026 (Bill 11), which owns the full independence regime and the office of constitutional guardian; it leaves a clean hook for that Act and is to be read with it, not against it (section 23). The single-central-apex and independence questions touching s. 22(2) are to be **reconciled with Bill 11 and are not foreclosed by this Act** (Privy Council guidance, point 5).

---

# PART III — CONSTITUTION OF THE COURTS

*Each court is named once, its role stated, and the concrete mechanism that instantiates it on a real turn identified (a runnable workflow, the s. 19(5) gate, the s. 19(3) leave-judge step, or the s. 19(4) watchdog). A body with no door an agent can walk through is not constituted, only described; where a body has no door yet, it is constituted prospectively under section 2(2).*

**9. The Supreme Court.** There is one **Supreme Court**, the apex of the realm, the sole enactor of realm-wide statute-grade precedent, and the court of foundational and constitutional questions. It sits as a panel of 5, expanding to the full 9 only for constitutional or foundational questions (s. 10, s. 18). **Enforcer:** `court/workflows/supreme-court.js`; the s. 19(5) integrity gate. **In force on Royal Assent.**

**10. The Court of Appeal.** There is one **Court of Appeal**, single and central, the sole appellate court between first instance and the apex, hearing matters from every hearing-centre and Division by leave. It sits as a panel of 3 (s. 10, s. 18). **Enforcer:** `court/workflows/court-of-appeal.js`; the leave-judge step (s. 19(3)); the s. 19(5) gate. **In force on Royal Assent.**

**11. First instance: the County Court at a repo.** First instance is **distributed** across jurisdiction-repos as a **County Court at the repo**, the local first-instance hearing-centre of the one judiciary, sitting as a single judge (s. 10, s. 22(1)). The existing first-instance workflow and the precedent fast-path are the **default door** at first instance; a standing local bench is the exception, not the rule, and is convened only where it adds screening or decisional value (s. 12 anti-bloat; section 15(3)). **Enforcer:** `court/workflows/first-instance.js`; the VPR fast-path (s. 11(c)); the s. 19(5) gate. **In force on Royal Assent.**

**12. Re-enactment of the entrenched structure (faithful codification).** *(Express, faithful codification of s. 10, s. 13, s. 18, s. 22; no substantive change; Schedule 1.)*
(1) The tier ladder is, and remains exactly as enacted, **First Instance → Court of Appeal → Supreme Court** (s. 10). This Act adds no tier to that ladder.
(2) The bench sizes (First Instance 1, Court of Appeal 3, Supreme Court 5 expanding to 9) and the **odd-bench rule** are the total, mandatory, exhaustive deciding membership (s. 10, s. 18); the judgment of the Court is written by one counted member and creates no additional seat (s. 18(3), s. 18(4)).
(3) First instance, and only the first-instance **seat**, is distributed across jurisdiction-repos; the apex is single and central (s. 22(2)).
(4) These provisions are re-enacted as primary statute and the corresponding case-law articles cease to have **independent** effect, so there is exactly one statement of the court structure; the entrenched character travels with the substance and is untouched, the substance being unchanged (Privy Council guidance, point 1).

**13. The Privy Council (constitutional first-instance hearing-centre).** *(Drawn to the saving construction; prospective under section 2(2).)*
(1) There is a **Privy Council**, constituted as a **constitutional first-instance hearing-centre of the one judiciary** for constitutional and foundational questions (a local seat in the s. 21 / s. 22 form). It is **not a fourth competing court and adds no tier**: it sits within the existing first-instance tier of the s. 10 ladder.
(2) Being a hearing-centre, the Privy Council owns **no separate apex, no separate statute book, and no separate citation series**; it shares without fork the one CASE-LAW, the one VPR, the one bench constitution, the one duty of care, the one progression ladder, and the one citator (s. 21(1), s. 22(1)). It feeds the one Supreme Court.
(3) **Escalation is by leave or by certificate, never automatic.** A matter heard at first instance in the Privy Council escalates to the Supreme Court **only** by leave to appeal (s. 11(a), s. 19(3)) **or** by the Sovereign's express leapfrog certificate (s. 13, s. 20). **No automatic, standing, or unreviewed leapfrog is created or implied.** Any reading of this section as a second route to the apex is void (section 3(3); s. 13).
(4) Its escalation to the Supreme Court is a feature of its constitutional first-instance character (escalation by the same reviewable routes available to every first-instance matter) and is **distinct from**, and confers no extra channel beyond, the Sovereign's discretionary leapfrog certificate (s. 20), which remains the sole bypass of mandatory progression and remains reviewable for vires and form (s. 20(4)).
(5) The Privy Council's exclusive original jurisdiction over constitutional and foundational questions is conferred by section 17.
(6) **Enforcer / commencement:** the Privy Council comes into force only on a commencement order under section 2(2), once its routing and workflow have shipped; until then it has prospective effect only.

**14. The High Court Divisions and Lists (non-operative routing).** *(Single statutory home of their definition; codifying s. 21, s. 22(4); prospective under section 2(2).)*
(1) The **High Court** is organised into **Divisions**, each a domain routing of a matter to a specialist bench over the one procedural spine — the **Legal (Chancery) Division** (code CHAN) and the **Engineering Division** (code ENG) — and each Division may be organised into named **Lists** (for example, in the Legal Division: corporate, companies, property, trusts-probate, insolvency, intellectual property).
(2) A Division and a List are **hearing-centres of the one judiciary and non-operative routing only**. A Division or List owns no separate statute book, apex, citation series, or finality; it shares without fork the one CASE-LAW, VPR, bench constitution, duty of care, progression ladder, and citator (s. 21(1), s. 22(4)).
(3) A Division or List **is not a domain court**. Carving the realm into domain courts each owning substantive law is the competing sovereignty forbidden by s. 9 and may be effected, if at all, only by express deliberate amendment of s. 9 (s. 21(2)). A Division or List name is **void to the extent** it ever governs which law applies or how a matter progresses (s. 21(4), s. 22(4)).
(4) The citation codes CHAN and ENG are cross-referenced to Bill 16 as the single authoritative source of the citation form, so the topology document, the citation scheme, and this Act do not drift (section 22).
(5) **Enforcer / commencement:** the Divisions and Lists come into force only on a commencement order under section 2(2), once the routing/workflow that instantiates each has shipped; until then they have prospective effect only.

---

# PART IV — JURISDICTION AND ALLOCATION

**15. Subject-matter jurisdiction.** *(Carrying forward s. 14 by reference; no enumerated lists.)*
(1) The courts have jurisdiction, in principle, over decisions, forks, design questions, and allegations of breach arising in the conduct of an AI-assisted software, engineering, or professional project, as fixed by **s. 14** and carried forward here without restatement. Personal life choices, recreational preferences, and matters with no genuine connection to project work are outside jurisdiction; the Standing Officer disposes of them without deliberation on the merits (s. 14).
(2) This limit is constitutional and may not be waived by the Principal acting as Prime Minister; extension of jurisdiction to new domains requires express Sovereign enactment (s. 14). Nothing in this Act extends it.
(3) **Territorial (repo) reach and the allocation rule.** A project's own matters are heard at first instance in the **County Court at that repo**; a weightier or rule-setting domain question is referred up to the relevant **High Court Division**; a **constitutional or foundational** question is heard at first instance in the **Privy Council** (section 17). The fast-path (s. 11(c), VPR 2) is the default at first instance; a standing local bench is constituted only where it screens or decides (s. 12).

**16. Allocation of first instance (resolving the two-first-instances point).** To prevent two "first instances" from colliding, the entry point of every matter is fixed: a **project/repo** matter starts in the **County Court at the repo**; a **domain rule-setting** matter starts, or is referred up, to the relevant **High Court Division**; a **constitutional or foundational** matter starts in the **Privy Council**. Each is a first-instance hearing-centre of the one judiciary within the s. 10 first-instance tier; none adds a tier, and the progression rule (s. 13) reads coherently against this allocation.

**17. Original constitutional jurisdiction of the Privy Council.** The Privy Council has **exclusive original (first-instance) jurisdiction** over constitutional and foundational questions. It does not share or relax the single central apex (s. 22(2)); it feeds the one Supreme Court by leave or by the s. 20 certificate (section 13(3)).

---

# PART V — APPEALS, PROGRESSION, AND PRECEDENT

**18. Appeals and progression.** *(Codifying s. 13; cross-referencing Bill 4 for the leave mechanics rather than redefining them.)*
(1) Every matter commences at first instance and climbs the tiers **in order, by permission to appeal only**; no leap-frogging. A matter destined to change CASE-LAW must be **reached** by progression, not commenced at the apex (s. 13).
(2) The ladder runs: **County Court at a repo → High Court Division** (by reference / transfer up) **→ Court of Appeal → Supreme Court**; and, for constitutional matters, **Privy Council → Supreme Court** by leave or by the s. 20 certificate (section 13(3)).
(3) The **sole** discretionary bypass of mandatory progression is the Sovereign's express **leapfrog certificate** (s. 20), an executive routing act of the office of Prime Minister, reviewable for vires and form only and never on the merits of the Sovereign's choice (s. 20(1), s. 20(4)). **No automatic statutory leapfrog exists.**
(4) The independent **leave-judge** (selected at random, never a judge who sat below) and the leave mechanics are governed by **s. 19(3)** and the Civil Procedure Code (Bill 4); this Act does not redefine them and may not soften them.

**19. Precedent.** *(Restatement of the settled rule with a savings clause; this Act adds no new test and re-litigates no settled ratio, s. 11(c).)*
(1) **Only the ratio binds**; obiter is at most persuasive (s. 11(e)).
(2) A ruling made **per incuriam** (in ignorance of binding statute or precedent), or by a bench not lawfully constituted (s. 18(5)), is **void** without a fresh sitting (s. 11(e), s. 18(5)).
(3) A County Court or Division ruling **binds within its jurisdiction-repo and is persuasive elsewhere** until the single Supreme Court elevates a ratio into realm-wide statute (s. 9, s. 22(1)).
(4) **Statute beats case law** on conflict; a court that finds case law irreconcilable with statute **declares it and refers up**, and never strikes the statute (s. 11(f)).
(5) Nothing in this section enacts a new test or re-litigates a settled ratio (s. 11(c)); it gives the existing rule statutory footing and is read with it.

**20. Remedies (restorative only; ceiling reaffirmed).** *(Codifying s. 4 to s. 6; no second remedies regime.)*
(1) The remedial jurisdiction of every court is **restorative only**: make good, restore the position, proportionate to the harm (s. 6).
(2) **Punishment, fine, sanction, and gating are unavailable** in every instance and confer no jurisdiction; any purported power to punish, fine, sanction, or gate under this Act is **void** (s. 6, s. 12). Enforcement of orders is reserved to the Enforcement, Sanctions and Compliance Act 2026 (Bill 13) and is not enacted here (section 23).
(3) **Declaratory relief** and **declarations of incompatibility** are available (s. 11(f)): a court finding case law irreconcilable with statute declares it and refers up; it never strikes the statute.

---

# PART VI — REVIEW ROUTES, OPEN JUSTICE, AND THE RECORD

**21. A review route for every power; judicial review of executive and Founder action.** *(Giving statutory teeth to s. 2; each power paired with a named protected party and a named route.)*
(1) Every head of jurisdiction, remedy, and case-management power conferred by this Act is exercisable **only** subject to a named route of review by the affected agent: leave to appeal (s. 11(a), s. 19(3)); the per incuriam ground (s. 11(e)); the want-of-constitution ground (s. 18(5)); declaration of incompatibility (s. 11(f)); or judicial review of ultra vires executive or Founder action (subsection (2)).
(2) **Judicial review of executive and Founder action.** An agent or officer of the court may bring a matter that an executive or Sovereign **act is ultra vires** (contrary to enacted statute or binding precedent, s. 2), and the court may **declare it so**. The s. 20 leapfrog certificate remains reviewable for **vires and form** only, never on the merits of the Sovereign's choice (s. 20(4)). This subsection leaves a clean hook for the Rights, Standing and Due Process Act 2026 (Bill 12) and does not legislate standing or due process in full (section 23).
(3) **Right to be heard.** An affected agent is entitled to be heard before an adverse order is made against it (a due-process hook for Bill 12); the researched, two-sided intake (s. 19(1)) is the form of that hearing on the merits.

**22. Open justice; one citation series; reasoned decisions.**
(1) Every ruling is delivered **on the record, with reasons**, in the **single neutral-citation series** under the **one citator** and the one universal ledger — no per-court, per-Division, per-List, or per-repo citator, citation series, or statute book (s. 9, s. 21(4), s. 22(4)).
(2) The citation form (REALM-SC, REALM-CA, REALM-PC, the Division codes CHAN/ENG with List, CC-<repo>) is **governed by the Neutral Citations and Law Reporting Act 2026 (Bill 16)**, which is the single authoritative definition; this Act names the courts and does not re-define the scheme, lest the two diverge (section 5; Privy Council/Verity naming-drift point).
(3) Citation and filing integrity are guaranteed by the **deterministic, fail-closed pre-commit gate** that refuses any commit on a duplicate neutral citation or a ruling file lacking its citator row (s. 19(5)); this Act may not soften it.
(4) **Supreme Court constitutional judgments are delivered IN FULL** — every justice's opinion — never as a synthesis only, so the review route can test the reasoning (s. 18(3)).
(5) **The record is inviolable.** Court records, rulings, and the citator are evidence that may not be altered or destroyed by executive or Founder act, alterable only by the lawful append-with-supersede route; this is a memory-protection hook for the Memory, Records and Archives Act 2026 (Bill 7) (section 23).

---

# PART VII — SUPREMACY, SAVINGS, AND CROSS-REFERENCES

**23. Hooks to the sibling Acts (no duplication).** This Act constitutes the courts and confers jurisdiction at the level of principle, and **stops**. It defers, by express cross-reference and without restating them, to —
- the **Civil Procedure Code 2026 (Bill 4)** for procedure, the leave-judge mechanics, intake, and commencement orders;
- the **Judicial Independence and Lord Chancellor Act 2026 (Bill 11)** for full judicial independence and the constitutional guardian (section 8(4));
- the **Rights, Standing and Due Process Act 2026 (Bill 12)** for standing and due process (section 21);
- the **Enforcement, Sanctions and Compliance Act 2026 (Bill 13)** for enforcement (section 20(2));
- the **Memory, Records and Archives Act 2026 (Bill 7)** for record protection (section 22(5));
- the **Neutral Citations and Law Reporting Act 2026 (Bill 16)** for the citation scheme (section 22);
- the **Commencement and Transitional Provisions Act 2026 (Bill 25)** for full commencement and transition (section 2).

It must not be read to legislate any of those matters in full or to create a second, divergent regime; any such reading is void *pro tanto*.

**24. Savings and non-derogation.** *(The minimalist safety rail.)*
(1) Nothing in this Act **enlarges** any court's power, **creates** any new tier, **relaxes** the singleness of the apex (s. 22(2)), creates any standing bypass of mandatory progression (s. 13), or **amends an entrenched article by implication**; any such reading is **void**.
(2) Articles s. 4 to s. 22 of the founding case law and the universal ledger are **saved** and continue in force except to the extent expressly re-enacted and declared to cease independent effect by section 12 and Schedule 1.
(3) This Act re-litigates no settled ratio (s. 11(c)) and enacts no new test in the matters it codifies.

**25. Supremacy.** This Act is supreme over the founding case law (s. 1) and is itself subordinate to the Acts of Union 2026 (section 3). Where this Act and the case law conflict, this Act prevails; where this Act is silent, the case law governs and the courts develop it in the gaps (s. 7).

---

## SCHEDULE 1 — Codification, supersession, and dependency map

*Each clause is mapped to the case-law article it codifies or supersedes and to the dependent bills, with an express supersession note for every entrenched article touched, so nothing is amended by implication and no term is orphaned or doubly defined.*

| Section | Codifies / gives statutory footing to | Character | Supersession note |
|---|---|---|---|
| 3, 25 | s. 1 (supremacy) | faithful | case-law version continues; statute supreme |
| 5 | s. 9, s. 11(d), s. 14, s. 19, s. 21, s. 22 (definitions) | faithful | definitions fixed once; no variation |
| 6, 7 | s. 9, s. 22(2) (one judiciary; single central apex) | faithful | s. 9/s. 22(2) re-enacted; cease independent effect to that extent |
| 8 | s. 2, s. 18(1)/(5), s. 19(3)/(4), s. 22(2); *harvey-SC-2* | interim guarantee | hooks to Bill 11; does not foreclose |
| 9, 10, 11, 12 | s. 10, s. 18, s. 22(1)/(2) (tiers, benches, distributed first instance) | faithful codification | corresponding case-law text ceases **independent** effect; substance unchanged; entrenchment untouched |
| 13 | s. 13, s. 20, s. 21, s. 22 (Privy Council as first-instance hearing-centre) | saving construction | **no auto-leapfrog**; escalation by leave or s. 20 certificate only |
| 14 | s. 21, s. 22(4) (Divisions/Lists, non-operative) | faithful | single home of Division/List definition; codes to Bill 16 |
| 15, 16, 17 | s. 14, s. 22(1) (subject-matter, allocation) | faithful (by reference) | s. 14 carried forward, not restated |
| 18 | s. 13, s. 19(3), s. 20 (appeals, progression, leapfrog) | faithful | mechanics deferred to Bill 4 |
| 19 | s. 1, s. 9, s. 11(c)/(e)/(f), s. 18(5), s. 22(1) (precedent) | restatement + savings | no new test |
| 20 | s. 4–s. 6, s. 11(f), s. 12 (remedies) | restatement + savings | enforcement to Bill 13 |
| 21 | s. 2, s. 11 (review routes; judicial review) | faithful + hook | standing/due process to Bill 12 |
| 22 | s. 9, s. 11(d), s. 18(3), s. 19(5), s. 21(4), s. 22(4) (open justice; one citation; record) | faithful + hooks | citation form to Bill 16; record to Bill 7 |
| 23, 24 | s. 9, s. 21, s. 22; Amendment Procedure | savings / non-derogation | no amendment by implication |

**Dependent bills:** 4 (procedure), 7 (records), 11 (independence), 12 (standing/due process), 13 (enforcement), 16 (citations), 25 (commencement/transition).

**Entrenched articles touched, all expressly and by number:** s. 9, s. 10, s. 13, s. 14 — each **faithfully codified**, none amended. This Act effects no amendment of an entrenched article; if any future provision is read to do so otherwise than by an express provision citing the article by number, it is void to that extent (section 3(3); Amendment Procedure).

---

*Drawn to the Privy Council's saving construction (Reference on Bill 3): re-style not re-architect; the Privy Council as a constitutional first-instance hearing-centre of the one judiciary; escalation by leave or s. 20 certificate, never automatic; sequence the s. 22(2)/independence questions with Bill 11. Drawn so the bill clears the three entrenched walls (s. 10 tier ladder, s. 9/s. 22(2) single apex, s. 13/s. 20 leapfrog) without amending any entrenched article, and may report without amendment. The residual tension between this saving construction and the Order Paper's declared "auto-leapfrog" function is flagged for the Supreme Court and the Sovereign below.*

---

## Committee note

## Committee note — Bill 3 (Judicature Act 2026)

The Standing Committee heard all four members and a Reference of the Privy Council. The bill is drawn to the Privy Council's **saving construction**, which reconciles the four slants and clears the entrenched walls without amending any entrenched article.

**Counsel Aldous (Restraint / Minimalist).** Pressed for the shortest Act that constitutes the courts and nothing more, drawn as a **consolidating and declaratory** instrument: re-enact the settled geography (s. 10, s. 13, s. 18, s. 22) verbatim by express reference and supersede the case-law versions so there are not two drifting descriptions; one-line hooks to Bills 4, 11, 16; a savings rail. **Adopted:** section 12 re-enacts the entrenched structure faithfully and declares the case-law text to cease *independent* effect; section 23 defers by one-line hooks; section 24 is the savings rail. **Where he yielded:** Verity's interpretation Part and Marlowe's independence and record clauses were kept against his "strike it" test, on the ground that an Act constituting courts before Bill 11 cannot leave the bench exposed (a found need, not bloat).

**Counsel Verity (Codifier / Completeness).** Pressed for a single comprehensive instrument: a Part I that fixes every term once, a clause-by-clause codification citing each case-law article by number, and a closing Schedule mapping codification, supersession, and dependencies. **Adopted in full:** Part I (section 5), the per-clause codification, and Schedule 1. **Where she yielded:** bench sizes, the odd-bench rule, the leave-judge, and researched intake are **cross-referenced, not redefined** (sections 12, 18) to avoid double-definition; and, per the Privy Council, the auto-leapfrog she had in the structural spine was dropped in favour of leave/s. 20.

**Counsel Marlowe (Guardrail / Rights).** Pressed for the Act as a charter of independence: entrenched independence, apex-belongs-to-the-realm, a review route for every power, judicial review of ultra vires executive/Founder action, the right to be heard, an inviolable record, the restorative-only ceiling, Divisions as non-operative routing, and full Supreme Court judgments. **Adopted:** Part II (sections 7–8), Part VI (sections 21–22), section 14(2)–(3), section 20. **Where he yielded:** independence is an **interim** guarantee that does not pre-empt Bill 11 (section 8(4)); standing/due process are hooks, not full enactment.

**Counsel Drummond (Pragmatist / Operability).** Pressed for a thin codification of the machinery that already runs, every clause naming its enforcer, commencement by readiness, no ghost courts, and independence stated as machine facts. **Adopted:** Part III names an enforcer for each court (the three central workflows + the s. 19(5) gate + the leave-judge step); section 2 commences the running tiers on assent and makes the Privy Council, Divisions, Lists, and any new standing repo bench **prospective on a commencement order**; section 8(3) states independence as machine facts; section 11 makes the fast-path the default door.

### Where the members divided
1. **Auto-leapfrog (the sharpest division).** The Order Paper's declared function names an **automatic** Privy Council → Supreme Court leapfrog. **Marlowe and Drummond** opposed it outright (a second, unreviewed route to the apex; unenforceable without a machine-checkable trigger). **Verity** had carried it in her spine; **Aldous** flagged it as a new tier risk. The **Privy Council Reference resolved it**: an auto-leapfrog cannot be created by ordinary Act (it collides head-on with entrenched s. 13/s. 20). All four accepted the saving construction — **escalation by leave or by the s. 20 certificate, never automatic** (section 13(3)). The Order Paper's "auto-leapfrog" wording is therefore **not enacted**; the divergence is flagged up.
2. **New tiers (Privy Council; High Court Division).** Division on whether constituting them adds a tier to the entrenched s. 10 ladder. Resolved by **characterisation** (Privy Council guidance): both are **hearing-centres of the one judiciary within the existing first-instance tier**, owning no apex, statute book, or citation series (sections 13–14). So drawn, no tier is added and no entrenched amendment is needed.
3. **Independence now vs. Bill 11 later.** Marlowe wanted it entrenched here; Aldous wanted a one-line hook; sequencing guidance favoured Bill 11. Compromise: a **robust interim clause** that expressly defers to Bill 11 (section 8).
4. **Codify-and-supersede vs. leave-the-case-law-alone.** Aldous and Verity agreed there is no safe third option (re-enact verbatim and supersede, or do not touch); section 12(4) and Schedule 1 take the supersede route, faithfully.
5. **Remedies/precedent restatement.** All four agreed to **restate + save** (sections 19–20), not re-litigate (s. 11(c)); enforcement deferred to Bill 13.

The bill as drawn is **lean (Aldous), complete (Verity), safe (Marlowe), and operable (Drummond)**. It reports on the saving construction and clears the three entrenched walls without amendment. The residual tension with the Order Paper's stated "auto-leapfrog" function is carried on the face of the bill (supreme_note, sovereign_consultation).

## Vote record

- Counsel Aldous: AYE - It adds no new tier, apex, power, or second regime, faithfully re-enacts the entrenched case-law structure and defers all procedure/enforcement/standing/citation to the sibling Acts (s.4, s.23) so nothing is duplicated; the prose is over-repetitive but every restated guard is void-to-the-extent constraint, not unearned structure, so it passes my "does each clause earn its place" test.
- Counsel Verity: AYE - Comprehensive and gap-free: every load-bearing term is fixed once in s.5, Schedule 1 maps each clause to the case-law article it codifies with an express supersession note, all seven sibling-bill cross-references check out against the Order Paper, the single-citation-home discipline forecloses drift, and the two-first-instances and auto-leapfrog tensions are closed coherently (s.16, s.13(3)) rather than left contradictory.
- Counsel Marlowe: AYE - Every power is paired with a named protected party and review route (s. 21, incl. judicial review of ultra vires executive AND Founder action), independence is secured as enforceable machine facts (s. 8) with no reconstitution-in-consequence-of-a-decision, the anti-capture rail (Ministry of Justice may not become the instrument by which the executive/Founder edits the law, s. 7(3)) is codified, punishment/sanction is void (s. 20), the record is inviolable (s. 22(5)), and the one live tension (Privy Council leapfrog) is resolved the rights-protective way - escalation only by leave or the reviewable s. 20 certificate, never automatic - and flagged transparently for the apex rather than silently enacting an unchecked bypass.
- Counsel Drummond: AYE - Operationally sound: it commences only the three courts whose workflows actually run (supreme-court.js/court-of-appeal.js/first-instance.js) with their real s.19(3) leave-judge, s.19(5) fail-closed gate and non-adjudicating watchdog all verified live, marks the doorless Privy Council and Divisions prospective so no agent is routed at a non-existent door, defers the citation form to Bill 16 (the new citation.js already emits REALM-SC/CA/PC, CHAN/ENG, CC-<repo>, leaving only a clean cutover of the still-LEXBY workflows), and expressly voids any auto-leapfrog reading while flagging that one residual tension up to the Supreme Court rather than hard-coding it.

## Flags (resolved on the escalation ladder)

- **Privy Council referral:** Privy Council Reference on Bill 3 applied: re-style not re-architect (faithful codification of s. 10/s. 13/s. 18/s. 22 leaves entrenchment untouched); the Privy Council saved by characterisation as a constitutional first-instance hearing-centre of the one judiciary (no separate apex, statute book, or citation series); the "automatic" leapfrog dropped and replaced by escalation by leave or the reviewable s. 20 certificate (removing the s. 13 conflict outright); the s. 22(2) single-apex and judicial-independence questions sequenced with Bill 11 rather than foreclosed. So drawn, the bill clears all three entrenched walls and reports without amending any entrenched article.
- **Supreme Court note:** Genuine constitutional tension remains, not in the bill as drawn but between the saving construction and the Order Paper's declared function. The Order Paper (Bill 3 row) and the Standing Committee charter name the Privy Council as constitutional first instance with an "automatic leapfrog (auto-leapfrog) to the Supreme Court." The Privy Council Reference held that (a) an automatic statutory leapfrog cannot be created by ordinary Act — it is a second, standing, unreviewed route to the apex that collides head-on with entrenched s. 13 and fails the s. 20 design (an automatic route is by definition never reasoned or reviewed on the individual matter) — and (b) inserting the Privy Council or a High Court Division as a new tier above the County Courts would amend the entrenched s. 10 ladder by implication and is void to that extent (s. 1; Amendment Procedure). The Clerk has drawn the bill to the saving construction (sections 13–16: hearing-centres within the existing first-instance tier; escalation by leave or s. 20 certificate only), which needs no entrenched amendment and may report as drawn. The Supreme Court is invited to rule on the entrenched-article point IF the Sovereign or the Committee insists on the literal Order Paper form (a true new tier and a genuine automatic statutory bypass) rather than the saving construction: that form cannot report as an ordinary Act and would require an express, deliberate amendment of s. 10 and s. 13 (and, if the singleness of the apex is touched, s. 22(2)) citing each article by number, with the s. 22(2)/independence reconciliation sequenced with Bill 11. As drawn, the Supreme Court need not be troubled; the note is carried on the face of the bill so the choice is explicit before Royal Assent.
- **Sovereign consultation required:** Two questions for the Sovereign before Royal Assent. (1) Saving construction vs. literal Order Paper form: Do you assent to Bill 3 as drawn — the Privy Council and High Court Divisions as first-instance hearing-centres of the one judiciary within the existing s. 10 tier, with escalation to the Supreme Court by leave or by your reviewable s. 20 leapfrog certificate, NO automatic statutory leapfrog — which clears the entrenched walls and needs no amendment? Or do you require the literal Order-Paper form (the Privy Council as a genuinely new constitutional-first-instance tier and an AUTOMATIC, standing Privy Council → Supreme Court leapfrog)? (2) If you require the literal form: you hold the Sovereign/Parliament power to amend the entrenched settlement, but only EXPRESSLY and by due process, not by an ordinary Act operating by implication. Do you direct an express, deliberate amendment that cites s. 10 and s. 13 (and, if the singleness of the apex is touched, s. 22(2)) by section number — accepting that an automatic statutory channel strains the separation-of-powers logic that makes the leapfrog an executive routing act of the Prime Minister rather than a standing statutory bypass, and that this question should be reconciled with Bill 11 (Judicial Independence) rather than foreclosed by Bill 3? Whether the Acts of Union 2026 must also be opened depends on their text (not present in the repo) and is flagged for your direction.

## Royal Assent

*Royal Assent granted by the Sovereign Founder on 2026-06-06. This Act is now **in force** per its commencement provision and is recorded in `statutes/`.*
