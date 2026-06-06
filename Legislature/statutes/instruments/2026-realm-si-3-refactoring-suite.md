# The Refactoring Suite Instrument 2026

**Citation:** [2026] REALM-SI 3 (under Bill 5 s.18, made for the purposes of the code-quality and refactoring-discipline function owned by the Ministry of Business, Engineering and Skills under section 5B of Bill 27, read with Bill 14; made by the Standing Committee per Bill 27 s.20)

**Made by:** the Standing Committee of the Legislature, in exercise of the statutory-instrument power conferred on any body named on the Bill 14 section 8 authorisation roll by section 18 of the Ministries and Offices Act 2026 (Bill 5) (as inserted by section 14(2) of the Statutory Instruments (Framework) Act 2026 (Bill 26)), the parent authority being the Ministry of Business, Engineering and Skills (MBES), which owns and superintends the Refactoring Suite under section 5B of the VJS (Constitution and Machinery) Act 2026 (Bill 27); read with the delegated authority of the Delegated Legislative Authority, Law Reform and Codification Act 2026 (Bill 14); and made by the Standing Committee under section 20 of the VJS (Constitution and Machinery) Act 2026 (Bill 27)

**Status:** made

**Procedure:** negative (Bill 14 s.14 objection window)

**Made:** 2026-06-06

**Coming into force:** on expiry of the Bill 14 s.14 objection window without valid objection.

---

## Recitals

In exercise of the powers conferred by section 18 of the Ministries and Offices Act 2026 (Bill 5) (as inserted by section 14(2) of the Statutory Instruments (Framework) Act 2026 (Bill 26)), being the power to make a statutory instrument for the operational detail and procedure of a function the parent authority owns, the parent authority being the Ministry of Business, Engineering and Skills (MBES), which owns and superintends the Refactoring Suite under section 5B of the VJS (Constitution and Machinery) Act 2026 (Bill 27); read with the delegated authority of the Delegated Legislative Authority, Law Reform and Codification Act 2026 (Bill 14); and as the Standing Committee makes statutory instruments in exercise of the enabling power conferred on the parent authority by section 20 of the VJS (Constitution and Machinery) Act 2026 (Bill 27), the Standing Committee of the Legislature makes the following Regulations:

---

## PART 1 - INTERPRETATION AND SCOPE

### 1. Definitions

In these Regulations:

**"the Suite"** means the Refactoring Suite owned and superintended by the Ministry of Business, Engineering and Skills (MBES) under section 5B of the VJS (Constitution and Machinery) Act 2026 (Bill 27): the binding standards of code quality, naming, clarity, single responsibility, dead-code and duplication control, test coverage, and atomicity that govern a supervised refactoring pass within the operational box.

**"a refactoring pass"** means a change to existing code that improves its internal quality without altering its external behaviour, undertaken in discharge of an obligation arising under section 2.

**"the Ministry"** means the Ministry of Business, Engineering and Skills (MBES), the parent authority of these Regulations and the owner of the Suite under Bill 27 s.5B.

**"a ruling obligation"** means a refactoring or remediation obligation that arises from a ruling of a court of the realm: a remedy order that includes a refactoring obligation, a breach finding identifying code quality as a contributing factor, a remediation order for work found below the standard of care, or an express direction of the Principal to conduct a supervised refactoring pass under the Suite.

**"the changed surface"** means the code that a refactoring pass actually adds, removes, or alters, as distinct from the wider codebase that the pass leaves untouched.

**"a Request for Ruling"** means the route by which a broader issue discovered in the course of a refactoring pass, but outside the scope of the ruling obligation in hand, is put to the single judiciary for a fresh ruling, rather than fixed silently within the pass.

**"machine-checkable control"** means a deterministic rule capable of enforcement by the fail-closed gate, admitting no model judgement and never punitive, with a deterministic verification algorithm; a principle in these Regulations that is not so reducible is a soft standard enforced through the duty of care and court review.

### 2. Scope and the invocation gate

(1) These Regulations specify the durable PRINCIPLES of the Suite. They hold PRINCIPLES only and no operative facts: they name no script, command string, file path, tool, or skill, and any such artefact that may give effect to the Suite is engineering owned by the Ministry and is not the source of the law (Bill 27 s.5B; [2026] REALM-SC 8). Where a tool or artefact is named in these Regulations it is named illustratively only and is not operative.

(2) The Suite is invoked ONLY on a ruling obligation within the meaning of section 1. It is NEVER invoked for routine work: new feature work, new additions, or cosmetic change carrying no ruling obligation falls outside the Suite, and nothing in these Regulations requires, authorises, or gates such work. The Suite is a supervised remedy, not a standing style guide imposed on ordinary engineering.

(3) These Regulations do not amend, suspend, or relax any Act of the Realm, any CASE-LAW article, the Vibe Procedure Rules, or the bench constitution; any provision so read is void to that extent (Bill 14 s.10, s.12, s.17; CASE-LAW s.1, s.11(f)).

(4) Enforcement is confined to three mechanisms only: (i) the deterministic fail-closed gate, for the machine-checkable controls the Suite contains; (ii) the soft watchdog reminder; and (iii) referral to the single judiciary. No punitive consequence is available, and no consequence on the merits issues from any of them. The Suite governs the administrative and procedural conduct of a refactoring pass; it does not prescribe or override how a court orders protective measures, remedies, or sanctions, which remain restorative only (Bill 27 s.5B(5); CASE-LAW s.6; Bill 6 s.14; Bill 13 s.5A). Any provision read to substitute a non-restorative remedy is ultra vires and void.

---

## PART 2 - WHEN THE SUITE IS INVOKED

### 3. The invocation principle

(1) **Principle: invoke only on a ruling obligation.** A refactoring pass under the Suite is undertaken only where a ruling obligation requires it. The four heads are exhaustive: (a) a court ruling's remedy order includes a refactoring obligation; (b) a breach finding identifies code quality as a contributing factor; (c) a remediation order is being executed for work found below the standard of care; or (d) the Principal expressly directs a supervised refactoring pass under the Suite.

(2) **Principle: routine work is excluded.** The Suite SHALL NOT be invoked for routine feature work, new additions, or cosmetic change that carries no ruling obligation. A refactoring pass is a discharge of an obligation, never a self-started reorganisation of code that no ruling has called for.

### 4. Respect for the ruling scope

(1) **Principle: scope is fixed by the ruling.** A refactoring pass applies its changes exactly to the scope identified in the ruling or remedy that called it. The pass does not extend beyond that scope.

(2) **Principle: broader issues go to a new Request for Ruling, never a silent fix.** Where a broader issue is discovered in the course of a refactoring pass, outside the scope of the ruling obligation in hand, it is surfaced as a new Request for Ruling to the single judiciary. It is NOT silently fixed within the pass. This preserves the rule that the scope of a remedy is set by the court that ordered it, and that a fresh issue is adjudicated before it is remedied (CASE-LAW s.6, s.13).

---

## PART 3 - THE PRINCIPLES OF THE SUITE

### 5. Naming and clarity

(1) **Principle: name for what, not how.** Public functions, classes, and variables are named for what they do, not for how they do it, so a reader understands the surface without reading the implementation.

(2) **Principle: no opaque names.** Single-letter names are not used outside a tight, local loop where the index has no other meaning.

(3) **Principle: predicates read as predicates.** A function whose purpose is to return a truth value is named so that its name reads as a question or a state (for illustration only, an `is`, `has`, or `can` prefix), so a caller can rely on the name to know the return is a boolean.

### 6. Single responsibility

(1) **Principle: one thing per unit.** Each function does one thing, and any side effect it carries is made explicit in its name or its signature rather than hidden in its body.

(2) **Principle: bounded size with reason.** A function does not grow past the point at which its single responsibility is no longer legible without a clear structural reason for the length; oversized units are decomposed.

(3) **Principle: cohesion of the module.** A module does not gather unrelated concerns; unrelated handling is separated so each module has one reason to change.

### 7. Dead code and duplication

(1) **Principle: remove the dead.** Unused exports, unreachable branches, and dead variables discovered within the changed surface are removed; the refactoring pass does not preserve code that nothing reaches.

(2) **Principle: extract on the rule of three.** Repeated logic is extracted into one shared site only when three or more sites share it (the rule of three); a single repetition is not prematurely abstracted.

(3) **Principle: no stubs for absent callers.** Backwards-compatibility stubs, shims, or dead seams are not retained for callers that do not exist; a compatibility layer is justified by a real caller or it is removed.

### 8. Test coverage at the changed surface

(1) **Principle: existing tests still pass.** A refactoring pass is complete only when the existing tests still pass after it; a pass that breaks a passing test has changed behaviour and is not a refactoring within the meaning of these Regulations until reconciled.

(2) **Principle: cover any behaviour change.** Where a change is not a pure rename or restructure but alters behaviour, that behaviour change is covered by at least one test at the changed surface before the pass is treated as complete.

### 9. Atomicity

(1) **Principle: one logical change per commit.** Each logical change is committed separately with a clear message describing what it does and why, so the history of the pass is legible and reviewable change by change.

(2) **Principle: separate refactoring from behaviour change.** A commit that only refactors (no behaviour change) is kept separate from a commit that changes behaviour, so that a reviewer or a later court can tell a pure restructure from a substantive change on the face of the history (CASE-LAW Amendment Procedure model of append-with-supersede, applied to the commit record).

---

## PART 4 - IMPLEMENTING BODY AND OPERATIONAL GOVERNANCE

### 10. Mandate to the Ministry

(1) The Ministry of Business, Engineering and Skills (MBES), the parent authority of these Regulations and the owner of the Suite under Bill 27 s.5B, is MANDATED to maintain, give effect to, and remedy the Suite, as part of the engineering and refactoring jurisdiction assigned to that Ministry under the VJS (Constitution and Machinery) Act 2026 (Bill 27).

(2) The mandate extends to:
  - (a) maintaining the canonical statement of the Suite's principles within the operational box;
  - (b) maintaining any engineering artefact (a checklist, a deterministic check, a command, or a guided procedure) that gives effect to those principles, such artefacts being engineering owned by the Ministry and never the source of the law (Bill 27 s.5B; [2026] REALM-SC 8);
  - (c) reducing to a machine-checkable control any principle of the Suite that is deterministically checkable, so that the deterministic pre-commit gate may enforce it without model judgement; and
  - (d) keeping the soft standards of the Suite enforceable through the duty of care and court review, never through a punitive consequence.

(3) The mandate is engineering, not law-making. It confers no power to author, amend, or enact the authoritative text of these Regulations; that authority rests with the Standing Committee with the Sovereign, per [2026] REALM-SC 8 and Bill 27 s.5C. The force of any artefact giving effect to the Suite comes from what it does (it checks, reminds, or guides), and it is engineering within the meaning of that judgment; the force of these Regulations comes from the Committee's making, not from the hand that drafted them.

### 11. Audit and reporting

(1) Every invocation of the Suite, every fail-closed rejection of a machine-checkable control, and every Request for Ruling raised for a broader issue under section 4(2) is recorded to the existing event chain (Bill 14 s.19; Bill 8), so the provenance of any refactoring pass is reconstructable on demand without bespoke logging.

(2) The Ministry SHALL report a persistent or unremedied defect in the Suite or in the artefacts that give it effect to the Ministry of Justice, and any matter of conformance MAY be referred to the single judiciary on the progression ladder (CASE-LAW s.13), never as a punitive matter but as a governance conformance check.

---

## PART 5 - AMENDMENT AND COMMENCEMENT

### 12. Amendment

(1) Amendment to this Instrument is made by the Standing Committee as a statutory instrument under section 18 of the Ministries and Offices Act 2026 (Bill 5), the parent authority being the Ministry of Business, Engineering and Skills (MBES), read with the delegated authority of the Delegated Legislative Authority, Law Reform and Codification Act 2026 (Bill 14), using the Bill 14 s.27 amendment procedure (as substituted by Bill 26 s.6), which re-runs the Bill 14 s.14 objection window, as the Suite evolves.

(2) **Principle: a court ruling that finds a refactoring-practice gap updates the Suite by amendment.** Where a court of the realm identifies, in a ruling, a gap in refactoring practice, that gap is closed by an amendment to this Instrument made under subsection (1), the amendment recording the ruling's citation. A ruling does not amend the Suite of its own force; it is given effect by the SI amendment process, so the Suite remains a single, published, version-controlled text and never a scattered accretion of unrecorded checks.

(3) Amendments are published, with the original text and the amending text clearly marked, on the append-with-supersede rule (CASE-LAW Amendment Procedure); silent repeal is never permitted. Each amendment undergoes the Bill 14 s.14 objection window.

(4) This Instrument is subordinate to the Acts of the Realm and to case law and is void to the extent of any conflict with an Act of the Realm or any entrenched article (Bill 14 s.17; CASE-LAW s.1, s.11(f)).

### 13. Commencement

(1) This Instrument comes into force on expiry of the Bill 14 s.14 objection window without valid objection.

(2) From commencement, the principles in Parts 2, 3, and 4 are operative, and the machine-checkable controls the Suite contains are enforced through the deterministic pre-commit gate and the soft watchdog (Bill 14 s.14; Bill 13 s.5A; CASE-LAW s.19(4), s.19(5)).

---

## Made by the Standing Committee

**Counsel Aldous (Restraint):** The Instrument does one load-bearing thing: it makes the durable principles of the Refactoring Suite law rather than convention, and names MBES as the owner of the remedy under a real enabling provision. It holds PRINCIPLES only (when to invoke, respect for the ruling scope, naming and clarity, single responsibility, dead code and duplication, test coverage at the changed surface, atomicity, and the rule that a broader issue goes to a new Request for Ruling) with no operative facts: no script names, no paths, no command strings, no tool or skill names save as illustration expressly marked non-operative. The invocation gate is drawn tightly: the Suite bites only on a ruling obligation and never on routine work, which is the whole point of a supervised remedy. No bloat; nothing here that is not load-bearing. It has my assent.

**Counsel Verity (Codifier):** The Instrument completely codifies the substance of the Suite at Judicature/.justice/suites/refactoring.md: the four exhaustive heads of invocation (s.3); respect for the ruling scope with broader issues routed to a new Request for Ruling, never silently fixed (s.4); naming and clarity, single responsibility, dead code and duplication on the rule of three, test coverage at the changed surface, and atomicity (ss.5 to 9); the engineering mandate to MBES (s.10); and the amendment limb by which a court-identified refactoring-practice gap updates the Suite via SI amendment carrying the ruling's citation (s.12(2)). The enabling chain is recited correctly and on all fours with [2026] REALM-SI 2: the operational-detail-and-procedure SI power of Bill 5 s.18 (inserted by Bill 26 s.14(2)), the parent authority being MBES, which owns the Suite under Bill 27 s.5B; read with the Bill 14 delegated authority; made by the Standing Committee under Bill 27 s.20. No fork; the authority chain is transparent and grounded in a real enabling provision. It has my assent.

**Counsel Marlowe (Guardrail):** The Instrument enforces no punitive consequence. Enforcement is confined to the deterministic fail-closed gate (for the machine-checkable controls only), the soft watchdog reminder, and referral to the single judiciary; none may adjudicate, score, sanction, or raise the standard of care (Bill 27 s.5B(5); CASE-LAW s.6; Bill 13 s.5A). The scope rule is the rights-protecting heart of the draft: a refactoring pass is bounded by the ruling that ordered it, and a broader issue is adjudicated by a fresh Request for Ruling before it is remedied, so no agent uses a narrow remedy as a licence to reach across the codebase. The Suite governs administrative and procedural conduct only and does not touch how a court orders protective measures or remedies, which remain restorative (Bill 6 s.14; CASE-LAW s.6). The MBES mandate is engineering only and confers no authority to author the authoritative text of these Regulations, faithfully observing [2026] REALM-SC 8. Rights and the separation of powers are protected. It has my assent.

**Counsel Drummond (Pragmatist):** The Instrument is operationally sound. The principles map cleanly onto the engineering that already exists in the Suite: the deterministically checkable ones (existing tests still pass, dead code within the changed surface removed, separate refactoring and behaviour-change commits) reduce to machine-checkable controls the pre-commit gate can run without model judgement, and the soft ones (name for what not how, single responsibility, the rule of three) ride the duty of care and court review. Invocation only on a ruling obligation keeps the Suite from becoming a standing tax on ordinary work, which is exactly how the suite reads today. The amendment-by-ruling limb keeps the Suite a single versioned text rather than a drift of ad hoc checks. This will work. It has my assent.

**Clerk's Note:** The Standing Committee makes this statutory instrument in exercise of the statutory-instrument power conferred on any body named on the Bill 14 section 8 authorisation roll by section 18 of the Ministries and Offices Act 2026 (Bill 5) (as inserted by section 14(2) of the Statutory Instruments (Framework) Act 2026 (Bill 26)), the parent authority being the Ministry of Business, Engineering and Skills (MBES), which owns and superintends the Refactoring Suite under section 5B of the VJS (Constitution and Machinery) Act 2026 (Bill 27); read with the delegated authority of the Delegated Legislative Authority, Law Reform and Codification Act 2026 (Bill 14); and made by the Standing Committee under section 20 of the VJS (Constitution and Machinery) Act 2026 (Bill 27), under negative procedure per Bill 14 s.14. The Instrument gives legal force to the Refactoring Suite contemplated by Bill 27 s.5B(1): it enshrines the Suite's substance as durable PRINCIPLES holding no operative facts (invocation only on a ruling obligation and never on routine work; respect for the ruling scope; naming and clarity; single responsibility; dead code and duplication; test coverage at the changed surface; atomicity; and the rule that a broader issue found in passing goes to a new Request for Ruling, not a silent fix), mandates the Ministry of Business, Engineering and Skills to maintain, give effect to, and remedy the Suite as engineering it owns, and provides that a court ruling identifying a refactoring-practice gap updates the Suite by amendment under Bill 14 s.27 carrying the ruling's citation. This Instrument is a void first draft prepared by Lexby as the s.9 agent and admitted to the Committee's second-draft stage; it derives its force from the Committee's making and not from the hand that first moved the pen ([2026] REALM-SC 8; Bill 28 s.4; CASE-LAW s.3(2) to (7)). Made 2026-06-06. Commencement on lapse of the Bill 14 s.14 objection window without valid objection.

---

**END OF INSTRUMENT**