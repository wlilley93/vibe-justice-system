# The Judgment Rendering and Lodgement Instrument 2026

**Citation:** [2026] REALM-SI 2 (under Bill 5 s.18, made for the purposes of the law-reporting and derived-projection functions of the Judicature in section 12 of Bill 16, read with Bill 14; made by the Standing Committee per Bill 27 s.5C)

**Made by:** the Standing Committee of the Legislature, in exercise of the statutory-instrument power conferred on any body named on the Bill 14 section 8 authorisation roll by section 18 of the Ministries and Offices Act 2026 (Bill 5) (as inserted by section 14(2) of the Statutory Instruments (Framework) Act 2026 (Bill 26)), the parent authority being the Ministry of Business, Engineering and Skills (MBES), which owns the engineering of the render-and-lodge mechanism under the VJS (Constitution and Machinery) Act 2026 (Bill 27); read with the delegated authority of the Delegated Legislative Authority, Law Reform and Codification Act 2026 (Bill 14); and made by the Standing Committee under section 5C of the VJS (Constitution and Machinery) Act 2026 (Bill 27)

**Status:** made

**Procedure:** negative (Bill 14 s.14 objection window)

**Made:** 2026-06-06

**Coming into force:** on expiry of the Bill 14 s.14 objection window without valid objection.

---

## Recitals

In exercise of the powers conferred by section 18 of the Ministries and Offices Act 2026 (Bill 5) (as inserted by section 14(2) of the Statutory Instruments (Framework) Act 2026 (Bill 26)), the parent authority being the Ministry of Business, Engineering and Skills (MBES) for the operational detail and procedure of the render-and-lodge engineering it owns under the VJS (Constitution and Machinery) Act 2026 (Bill 27); read with the delegated authority of the Delegated Legislative Authority, Law Reform and Codification Act 2026 (Bill 14); and as the Standing Committee makes statutory instruments in exercise of the enabling power conferred on the parent authority by section 5C of the VJS (Constitution and Machinery) Act 2026 (Bill 27), the Standing Committee of the Legislature makes the following Regulations:

---

## PART 1 - INTERPRETATION AND SCOPE

### 1. Definitions

In these Regulations:

**"a judgment"** means a ruling of any court of the realm that forms part of the public central record (the constitutional courts in full; the Court of Appeal's citation, ratio and status, with any factual narrative sealed), as scoped by the VJS (Constitution and Machinery) Act 2026 (Bill 27 s.14); a judgment of a local court that remains in its own jurisdiction-local directory is outside these Regulations save as that local court adopts them.

**"delivery"** means the filing of a new judgment or the amendment of an existing judgment in the central record.

**"render"** means the deterministic production of the formal court form of a judgment as a portable document, being a derived projection of the filed judgment within the meaning of section 12 of the Neutral Citations and Law Reporting Act 2026 (Bill 16).

**"lodgement"** means the staging and recording of everything a properly filed judgment requires: the citator row, the derived projections, and the rendered document.

**"derived projection"** means any deterministically rebuildable, pointer-only projection of the canonical committed judgment text, including the law-reports corpus, the search or retrieval index, and the rulings ledger (the universal ledger), as those projections are characterised by section 12 of the Neutral Citations and Law Reporting Act 2026 (Bill 16) and [2026] REALM-PC 4; the rendered document is treated as a derived projection for the purposes of these Regulations.

**"the citation layer"** means the neutral citation and the citator row of a judgment, the integrity of which is the record-integrity duty guaranteed by the deterministic citation-integrity gate (CASE-LAW s.19(5)).

**"the convenience layer"** means the derived projections and the rendered document, which restate the law for ease of access but are never themselves the source of any ratio, status or citation.

**"machine-checkable control"** means a deterministic rule enforced by the fail-closed gate, admitting no model judgement and never punitive, with a deterministic verification algorithm.

**"the Engineering department"** means the Engineering department of the Ministry of Business, Engineering and Skills (MBES), holding the engineering jurisdiction assigned to that Ministry under the VJS (Constitution and Machinery) Act 2026 (Bill 27).

### 2. Scope

(1) These Regulations specify the principles by which a judgment is rendered and lodged on delivery. They hold PRINCIPLES only and no operative facts: they name no script, path, file format detail or command string, consistent with the principles-at-law rule (Bill 27).

(2) These Regulations do not locate the authoritative record of any judgment otherwise than in the canonical committed markdown. The rendered document and every derived projection are derived from, and never the source of, the canonical text (Bill 16 s.12, s.21; [2026] REALM-PC 4). Nothing in these Regulations displaces that rule.

(3) These Regulations do not amend, suspend or relax any Act of the Realm, any CASE-LAW article, the Vibe Procedure Rules, or the bench constitution; any provision so read is void to that extent (Bill 14 s.10, s.12, s.17).

(4) Enforcement is confined to three mechanisms only: (i) the deterministic fail-closed gate; (ii) the soft watchdog reminder; and (iii) referral to the single judiciary. No punitive consequence is available, and no consequence on the merits issues from any of them (CASE-LAW s.6; Bill 14 s.20; Bill 13 s.5A).

---

## PART 2 - RENDERING AND LODGEMENT ON DELIVERY

### 3. The duty to render

(1) **Principle: render on delivery.** On the delivery of a judgment the court system SHALL deterministically render it to the formal court form of a portable document. Rendering is a first-class, deterministic act of the court system and is never left to model judgement.

(2) **Principle: idempotence.** Rendering SHALL be idempotent. The court system SHALL render a judgment only where its rendered document is missing or is older than its source, and SHALL NOT record a rendered document that has changed only through a non-deterministic re-render. A render that produces no material change from the canonical source effects no change to the record.

(3) **Principle: lockstep.** The rendered document is a derived projection of the filed judgment (Bill 16 s.12) and SHALL be kept in lockstep with it: where the source judgment is delivered or amended, the rendered document is regenerated as part of the same operation, so the two can never silently diverge.

### 4. The duty to lodge

(1) **Principle: lodge on delivery.** On the delivery of a judgment the court system SHALL lodge it, staging and recording everything a properly filed judgment requires, namely:
  - (a) the citator row for the judgment (the citation layer);
  - (b) the derived projections (the law-reports corpus, the search or retrieval index, and the rulings ledger); and
  - (c) the rendered document.

(2) **Principle: derived projections in lockstep.** The derived projections SHALL be regenerated deterministically as part of the same operation that records the citator row, so they can never silently diverge from the canonical committed judgment text (Bill 16 s.12(2)).

(3) **Principle: pointer-only.** No derived projection and no rendered document is itself the store of any ratio, status or citation as authority. Each is a pointer to be verified against the canonical text and is deterministically rebuildable from it (Bill 16 s.12(1); [2026] REALM-PC 4).

### 5. The verify split

(1) **Principle: fail-closed on the citation layer.** Where the integrity of the record is at stake, the verification SHALL fail closed. Filing and citation integrity are guaranteed by the deterministic pre-commit check that fails closed on a duplicate neutral citation and on any judgment committed without its corresponding citator row, giving effect to CASE-LAW s.19(5), s.1 and s.11(d), and Bill 16 Part 4. A judgment that does not satisfy the citation layer is not lodged.

(2) **Principle: fail-open on the convenience layer.** The convenience layer (the derived projections and the rendered document) SHALL fail open. A defect, staleness or absence in a derived projection or in the rendered document does not block the lodgement of a judgment whose citation layer is sound; it is recorded as a remediable convenience-layer defect and is remedied by regeneration in the ordinary course.

(3) This split mirrors the existing gate (CASE-LAW s.19(5)): the record's integrity is non-negotiable and fails closed; the restatement of the law for access is a convenience that fails open and is made good without blocking the law itself.

### 6. First-class command and automatic invocation

(1) **Principle: a first-class deterministic command.** The render-and-lodge mechanism SHALL be exposed as a first-class deterministic command (a command-line verb), so that delivery of a judgment is one auditable, deterministic act rather than a discretionary sequence.

(2) **Principle: automatic invocation by the gate.** The pre-commit gate SHALL invoke the mechanism automatically on delivery, so a filed judgment is always rendered and lodged and delivery is never left to model judgement. The mechanism is a machine-checkable control within the meaning of section 1.

(3) The command and the gate enforce these Regulations as a machine-checkable control and the soft watchdog reminder; neither adjudicates, scores, sanctions or punishes, and neither raises the standard of care. Any purported adjudication or consequence issuing from the gate or the watchdog is ultra vires and void, the remedy remaining exclusively judicial (CASE-LAW s.19(4), s.6).

---

## PART 3 - IMPLEMENTING BODY AND OPERATIONAL GOVERNANCE

### 7. Mandate to the Engineering department

(1) The Engineering department of the Ministry of Business, Engineering and Skills (MBES), the parent authority of these Regulations, is MANDATED to take all actions necessary to implement, maintain and remedy the render-and-lodge mechanism, as part of the engineering it owns under the VJS (Constitution and Machinery) Act 2026 (Bill 27).

(2) The mandate extends to:
  - (a) the renderer that produces the formal court form of a judgment;
  - (b) the idempotent render-all step (render only where missing or stale; never record a non-deterministic re-render);
  - (c) the first-class deterministic command (the command-line verb) by which delivery is effected; and
  - (d) the wiring of that command into the pre-commit gate so delivery is invoked automatically.

(3) The Engineering department SHALL remedy any defect in the mechanism. Where the convenience layer fails open under section 5(2), the department SHALL make it good by regeneration in the ordinary course; where the citation layer fails closed under section 5(1), the department SHALL preserve the fail-closed behaviour and SHALL NOT relax it to a convenience.

(4) The mandate is engineering, not law-making. It confers no power to author, amend or enact the authoritative text of any judgment or instrument; that authority rests with the constituted bench (for a judgment) and with the Standing Committee with the Sovereign (for an instrument), per [2026] REALM-SC 8. The force of the mechanism comes from what it does (it renders, lodges and gates), and it is engineering within the meaning of that judgment.

### 8. Audit and reporting

(1) Every render-and-lodge operation, and every fail-closed rejection or fail-open convenience-layer defect, is recorded to the existing event chain (Bill 14 s.19; Bill 8), so the provenance of any rendered or lodged judgment is reconstructable on demand without bespoke logging.

(2) The Engineering department SHALL report a persistent or unremedied defect in the mechanism to the Ministry of Justice, and any matter of conformance MAY be referred to the single judiciary on the progression ladder (CASE-LAW s.13), never as a punitive matter but as a governance conformance check.

---

## PART 4 - AMENDMENT AND COMMENCEMENT

### 9. Amendment

(1) Amendment to this Instrument is made by the Standing Committee as a statutory instrument under section 18 of the Ministries and Offices Act 2026 (Bill 5), the parent authority being the Ministry of Business, Engineering and Skills (MBES), read with the delegated authority of the Delegated Legislative Authority, Law Reform and Codification Act 2026 (Bill 14), using the Bill 14 s.27 amendment procedure (as substituted by Bill 26 s.6), which re-runs the Bill 14 s.14 objection window, as the mechanism evolves.

(2) Amendments are published, with the original text and the amending text clearly marked, on the append-with-supersede rule (CASE-LAW Amendment Procedure); silent repeal is never permitted. Each amendment undergoes the Bill 14 s.14 objection window.

(3) This Instrument is subordinate to the Acts of the Realm and to case law and is void to the extent of any conflict with an Act of the Realm or any entrenched article (Bill 14 s.17; CASE-LAW s.1, s.11(f)).

### 10. Commencement

(1) This Instrument comes into force on expiry of the Bill 14 s.14 objection window without valid objection.

(2) From commencement, the principles in Parts 2 and 3 are operative, and the mechanism is enforced as a machine-checkable control through the deterministic pre-commit gate and the soft watchdog (Bill 14 s.14; CASE-LAW s.19(4), s.19(5)).

---

## Made by the Standing Committee

**Counsel Aldous (Restraint):** The Instrument does one new load-bearing thing: it makes deterministic render-and-lodge of a delivered judgment law rather than convention, and names the body that owns the remedy. It holds PRINCIPLES only - render, lodge, the fail-closed/fail-open split, the first-class command, the automatic gate invocation - with no operative facts: no script names, no paths, no command strings, no file-format detail. The verify split is borrowed wholesale from the existing CASE-LAW s.19(5) gate rather than reinvented. No bloat; nothing here that is not load-bearing. It has my assent.

**Counsel Verity (Codifier):** The Instrument completely specifies the mechanism the Ministry of Justice recommended: the duty to render (idempotent, in lockstep), the duty to lodge (citator row, derived projections, rendered document), the fail-closed citation layer and the fail-open convenience layer, the first-class command, and the automatic gate invocation, with the Engineering department of MBES mandated to implement, maintain and remedy each named component. The enabling chain is recited correctly: the operational-detail-and-procedure SI power of Bill 5 s.18 (inserted by Bill 26 s.14(2)), the parent authority being MBES, which owns the engineering under Bill 27; the derived-projection characterisation of Bill 16 s.12 (the rendered document and the projections being derived projections, pointer-only, in lockstep, [2026] REALM-PC 4) supplies the substance, not the power; read with the Bill 14 delegated authority, made by the Standing Committee under Bill 27 s.5C. No fork; the authority chain is transparent and grounded in a real enabling provision. It has my assent.

**Counsel Marlowe (Guardrail):** The Instrument enforces no punitive consequence. Enforcement is confined to the deterministic fail-closed gate, the soft watchdog reminder, and referral to the single judiciary, and the gate and watchdog may not adjudicate, score, sanction or raise the standard of care (CASE-LAW s.19(4), s.6; Bill 13 s.5A). The fail-closed line is drawn exactly where the record's integrity is at stake (citation/citator, s.19(5)) and no wider; the convenience layer fails open so the law itself is never blocked by a stale projection. The Engineering mandate is engineering only and confers no authority to author the authoritative text of a judgment, faithfully observing [2026] REALM-SC 8. Rights and the separation of powers are protected. It has my assent.

**Counsel Drummond (Pragmatist):** The Instrument is operationally sound. The render duty is idempotent and deterministic; the lodge duty regenerates the projections in lockstep so they cannot silently diverge; the split is machine-checkable and matches the gate already in service. Exposing the mechanism as a first-class command invoked automatically by the pre-commit gate takes delivery out of model judgement entirely, which is the whole point of the briefing. The engineering that gives effect to this substantially exists; this names it law, names MBES as the parent authority on a real enabling provision (Bill 5 s.18), and names the Engineering department as the owner of the remedy. This will work. It has my assent.

**Clerk's Note:** The Standing Committee makes this statutory instrument in exercise of the statutory-instrument power conferred on any body named on the Bill 14 section 8 authorisation roll by section 18 of the Ministries and Offices Act 2026 (Bill 5) (as inserted by section 14(2) of the Statutory Instruments (Framework) Act 2026 (Bill 26)), the parent authority being the Ministry of Business, Engineering and Skills (MBES), which owns the engineering of the render-and-lodge mechanism under the VJS (Constitution and Machinery) Act 2026 (Bill 27); read with the delegated authority of the Delegated Legislative Authority, Law Reform and Codification Act 2026 (Bill 14); and made by the Standing Committee under section 5C of the VJS (Constitution and Machinery) Act 2026 (Bill 27), under negative procedure per Bill 14 s.14. The earlier draft recited the enabling power as a non-existent power conferred by section 12 of Bill 16; that is corrected here, since Bill 16 creates no new power (Bill 16 s.3(2)), Bill 26 s.14 inserted no enabling clause into Bill 16, and section 12 of Bill 16 is the derived-projection characterisation only. Section 12 of Bill 16 and [2026] REALM-PC 4 are retained in these Regulations as the substantive characterisation of the rendered document and the projections (pointer-only, in lockstep), not as the source of the power. The Instrument gives legal force to the deterministic render-and-lodge mechanism recommended in the Ministry of Justice policy briefing of 2026-06-06: it enshrines the render and lodge duties and the fail-closed/fail-open split as durable PRINCIPLES holding no operative facts, exposes the mechanism as a first-class deterministic command invoked automatically by the pre-commit gate, and mandates the Engineering department of the Ministry of Business, Engineering and Skills to implement, maintain and remedy it. This Instrument is itself a void first draft prepared by Lexby as the s.9 agent and admitted to the Committee's second-draft stage; it derives its force from the Committee's making and not from the hand that first moved the pen ([2026] REALM-SC 8). Made 2026-06-06. Commencement on lapse of the Bill 14 s.14 objection window without valid objection.

---

**END OF INSTRUMENT**