<!-- Bill 32 of 32 - Order Paper of the Realm. Drafted by the Standing Committee (second draft) on the void first draft of the V2 Kernel Team. -->
<!-- status: enacted | royal-assent: 2026-06-09 | outcome: passed-round-1 | ayes: 4/4 | drafting rounds: 1 (second draft on the void first draft) -->
<!-- CONSTITUTIONAL. Founds VJS V2 as a computer-first successor realm. Flagged (Bill 2 s.17): carries the Privy Council reference [2026] REALM-PC 24 and the Supreme Court settlement [2026] REALM-SC 10 through to Royal Assent. Commencement DEFERRED to the Schedule 1 conditions (declaratory until commenced; referral finding A). -->

# Computer-First Realm Act 2026

**Bill 32 of the Order Paper of the Realm**
**Neutral citation on commencement (V2 scheme):** [2026] VJS-ACT 8
**Rank:** constitutional

An Act to declare VJS Version 2 a computer-first legal system and the successor runtime jurisdiction of the Realm; to adopt the V2 Constitutional Migration Charter as the constitutional migration settlement; to supersede V1 as the live governing law of VJS while preserving V1 as the Gazette, Archive, historical record and migration evidence of the Realm; to establish the V2 lawpack and deterministic kernel as the runtime legal surface; to entrench the constitutive gate of Royal Assent against the computational legislature; to carry forward the protective floor of the Realm; to declare one Gazette with two estates; to preserve the singleness of the apex in continuity; to complete the constitutional relay from V1 to V2; and for connected purposes.

---

## Provenance (recital, not operative)

This Act gives effect to the settled law of the Realm on the founding of VJS V2. Its lawful chain is: the V2 Constitutional Migration Charter (Principal, 2026-06-08); the void first draft of the Computer-First Realm Act authored by the V2 Kernel Team (Lexby), admitted as a void first draft into the Standing Committee's second-draft stage (REALM-SC 8 ratio (4)-(5); CASE-LAW s.3(4)-(5)); the Privy Council reference and ruling [2026] REALM-PC 24 (competence limb disposed on the fast-path; founding and continuity limbs held foundational and leapfrogged); the Supreme Court settlement [2026] REALM-SC 10 (founding allowed on the continuity construction and on mandatory conditions; the Court ruled the principle and directed this Committee to draft); this Committee's second draft and vote (4 ayes of 4); and the Sovereign Founder's Royal Assent of 2026-06-09. Nothing in V2 bound before this Assent.

---

## Part 1 - Preliminary

### 1. Short title, citation and status

(1) This Act may be cited as the **Computer-First Realm Act 2026**.

(2) On commencement its neutral citation in the V2 scheme is **[2026] VJS-ACT 8**, a scheme this Act establishes on its own authority (as Bill 16 established the `REALM-*` scheme). Its identifier in the V1 Order Paper, **Bill 32**, remains a valid historical identifier.

(3) This is a **constitutional Act of VJS V2**.

```yaml
kernel_effect:
  short_title: Computer-First Realm Act 2026
  citation: "[2026] VJS-ACT 8"
  v1_identifier: "Bill 32"
  rank: constitutional
```

### 2. Constitutional status and the constitutional relay

(1) The force of this Act comes from the constitutive act of the Realm's organs - this Committee's drafting and the Sovereign's Royal Assent on a Supreme Court settlement - and never from the self-assertion of V2 or of any agent. V2 does not authorise V2; V1 authorises V2.

(2) The agent-originated text was a void first draft carrying no force until adoption; Royal Assent is constitutive and cures the origin defect, which is spent on adoption (REALM-SC 8 ratio (3)-(5)).

```yaml
kernel_effect:
  force_source: organ_constitutive_act
  v2_does_not_self_authorise: true
  void_first_draft_cured_by_assent: true
```

### 3. Commencement

(1) This Act is enacted on Royal Assent as a **declaratory instrument** and has **no binding runtime force** until it commences.

(2) This Act commences only when **all** the conditions in **Schedule 1** are satisfied and recorded on the deterministic record. Royal Assent (Schedule 1, condition (a)) is the **sole constitutive gate**; the remaining conditions are recorded preconditions to commencement.

(3) Until commencement no agent shall treat this Act, or any V2 record, as binding; and no record may represent the founding as settled, inevitable, or self-executing.

```yaml
kernel_effect:
  enacted: true
  draft_not_binding_before_commencement: true
  commencement_requires: schedule_1_all_satisfied_and_recorded
  sole_constitutive_gate: royal_assent
```

### 4. Interpretation

In this Act, the following terms are defined and govern; a section that uses an operative term not defined here or in an incorporated record is a validation defect (s.5(3)):

- **V1** - the first generation of VJS: its case-law settlement, Acts, statutory instruments, judgments, court hierarchy, Gazette, law reports, ledgers, and records.
- **V2** - the second generation of VJS, founded by this Act and the Migration Charter, implemented through the compact lawpack and deterministic kernel.
- **Charter** - the VJS V2 Constitutional Migration Charter.
- **computer-first legal system** - a legal system whose live operational law is expressed in compact, schema-valid, machine-checkable records capable of deterministic validation and application by the kernel.
- **lawpack** - the compact body of V2 constitutional law, Acts, regulations, rule atoms, court orders, specs, decisions, invariants, permits, proofs and logs loaded by the kernel.
- **kernel** - the deterministic VJS clerk that loads valid records, resolves authority, evaluates invariants, returns route decisions, and records proofs.
- **runtime authority** / **archive authority** - authority the kernel may apply to bind conduct / historical or evidential material that may be cited but does not bind unless expressly incorporated.
- **express incorporation** - incorporation by a V2 Act, Regulation, Rule Atom, Court Order, Spec, Decision or Invariant using a valid record identity and authority basis.
- **protective-floor carry-forward** - the enumerated, verified set of records in Schedule 2.
- **founding lock** - the recorded lawpack lock bearing a content digest over the lawpack.
- **estate** - a labelled division of the single Gazette: the V1 Archive estate or the V2 Current estate.
- **apex-singleness** - the single Court of Appeal and Supreme Court of the Realm, carried forward in continuity and never fractured.
- **first-impression** - the route for a V2 silent gap (s.9).

```yaml
kernel_effect:
  defines: [v1, v2, charter, computer_first_legal_system, lawpack, kernel, runtime_authority, archive_authority, express_incorporation, protective_floor_carry_forward, founding_lock, estate, apex_singleness, first_impression]
```

---

## Part 2 - The computer-first Realm

### 5. Declaration of the computer-first Realm; form and concordance

(1) VJS V2 is declared a **computer-first legal system**. Its live law is expressed through compact, schema-valid, machine-checkable records operated deterministically by the kernel.

(2) Computer-first does not abolish human language. Live operational law must be representable as a route rule, permission, prohibition, obligation, invariant, proof requirement, court trigger, citation rule, supersession link, boundary rule, log requirement or permit condition. **A record with no kernel effect is excluded from the runtime authority graph.**

(3) Each operative section bears both a ceremonial statement and a `kernel_effect`. A material discrepancy between them is a **validation defect** routed for correction; the kernel must not silently choose between them.

```yaml
kernel_effect:
  declares_computer_first_realm: true
  canonical_runtime_surface: lawpack
  no_kernel_effect_excludes_from_graph: true
  requires_concordance: true
  discrepancy_policy: validation_defect
  kernel_must_not_silently_choose: true
```

---

## Part 3 - Supersession and preservation of V1

### 6. Supersession of V1 as live runtime law

(1) The V1 legal system is superseded as the **live governing law** of VJS. No V1 authority binds V2 merely because it exists in the archive; no V1 rule is imported by implication.

(2) Supersession is **consolidation, not repudiation**, and operates entry-by-entry on the deterministic record; nothing is voided wholesale by reason only of provenance (REALM-SC 7).

```yaml
kernel_effect:
  supersedes_v1_live_law: true
  v1_not_runtime_authority_by_default: true
  no_v1_import_by_implication: true
  consolidation_not_repudiation: true
```

### 7. Preservation of V1 as Gazette and Archive

V1 is preserved as the VJS Gazette, Archive, historical research constitution, doctrinal record, and migration evidence. It may be cited as archive authority and is honoured as the record that discovered the V2 settlement. It is not loaded as ordinary live agent context.

```yaml
kernel_effect:
  preserve_v1_archive: true
  v1_roles: [gazette, archive, historical_record, doctrinal_record, migration_evidence]
  v1_not_loaded_as_default_context: true
```

### 8. V1 incorporation rule

V1 material has live V2 force only if expressly incorporated by a valid V2 record stating the v1_source, v2_destination, operative rule, kernel effect, and supersession or variation. A general reference or a citation for history is not incorporation.

```yaml
kernel_effect:
  incorporation_required: true
  general_reference_not_incorporation: true
```

### 9. No V1 revival by silence

(1) Silence, a gap, or an omission in V2 law **does not revive V1 as binding law**. Where V2 contains no applicable authority, the matter is a **V2 first-impression question**.

(2) V1 may be consulted as archive, historical authority, persuasive reasoning, or migration evidence, but has no live V2 force unless expressly incorporated under s.8.

(3) This section is given continuing deterministic effect by the lawpack invariant `INV-NO-V1-GAP-FILLER`, whose authority basis is, on commencement, this section.

```yaml
kernel_effect:
  v2_silence_does_not_revive_v1: true
  v2_silence_route: first_impression
  v1_archive_may_be_consulted: true
  incorporation_required_for_v1_force: true
  enforced_by: INV-NO-V1-GAP-FILLER
```

---

## Part 4 - V2 runtime authority

### 10. The V2 authority hierarchy

The V2 authority hierarchy is: the real-world-law warning boundary; the V2 Constitution; V2 constitutional Acts; V2 primary Acts; Kernel Regulations; Supreme Court Orders; Privy Council Orders; County Court repo Orders; local decision logs; and incorporated V1 archive material. Lower authority cannot contradict higher; a local log cannot amend law.

```yaml
kernel_effect:
  authority_hierarchy: [real_world_law_warning_boundary, v2_constitution, v2_constitutional_acts, v2_primary_acts, kernel_regulations, supreme_court_orders, privy_council_orders, county_court_repo_orders, local_decision_logs, incorporated_v1_archive_material]
  lower_cannot_contradict_higher: true
  local_log_cannot_amend_law: true
```

### 11. The kernel as deterministic clerk

The kernel may load the lawpack, validate records, resolve authority, issue bounded route decisions, identify court triggers, evaluate invariants, record permits and proofs, validate logs, and report boundary findings. It **may not** legislate, adjudicate as a court, create legal force by computation, call a model, use semantic similarity for binding authority, or treat archive material as live without incorporation. The governing test for whether an artefact is engineering or law is the **source of its force** (function versus promulgation by an organ) (REALM-SC 8 ratio (7)).

```yaml
kernel_effect:
  kernel_is_clerk: true
  kernel_must_not: [legislate, adjudicate_as_court, create_legal_force_by_computation, call_model, use_semantic_similarity_for_authority, treat_archive_as_live_without_incorporation]
  governing_test: source_of_artefacts_force
```

### 12. The lawpack as runtime surface; record types

(1) The V2 lawpack is the runtime authority surface; agents use it through the kernel. It contains compact, schema-valid records and does not require agents to read long-form V1 material for ordinary governed work. The lawpack lock records the version and digest in force.

(2) V2 runtime law may be carried by the Constitution, an Act, a Kernel Regulation, a Rule Atom, a Court Order, a Spec, a Decision, an Invariant, a Permit, a Proof and a Log. Commentary and archive material assist understanding but are not runtime law unless incorporated.

```yaml
kernel_effect:
  lawpack_is_runtime_surface: true
  lawpack_lock_required: true
  runtime_record_types: [constitution, act, kernel_regulation, rule_atom, court_order, spec, decision, invariant, permit, proof, log]
```

### 13. Future lawmaking; amendment and supersession

(1) New V2 law is introduced through proposal or authority basis, draft record, schema validation, authority validation, boundary validation, deterministic citation, authorised adoption, lawpack entry, lawpack-lock update, and decision log. Primary Acts require Principal assent; Regulations require parent authority; draft text has no force until the route completes.

(2) V2 law is append-only. Amendment is by a new valid record stating whether it adds, varies, corrects, supersedes, revokes, preserves or incorporates. The old record remains visible. Silent repeal, implied importation from V1, and unrecorded amendment are prohibited.

```yaml
kernel_effect:
  requires_lawmaking_route: true
  draft_not_binding: true
  primary_acts_require_assent: true
  regulations_require_parent_authority: true
  append_only: true
  prohibit_silent_repeal: true
  prohibit_implied_v1_import: true
```

---

## Part 5 - Entrenched constitutional guarantees

### 14. The constitutive gate, entrenched against the machine [constitutional]

(1) The **sole constitutive gate** for a V2 primary or constitutional Act is the Sovereign's Royal Assent (Bill 2 s.21). Assent is **non-automatable, non-presumable and non-delegable** (Bill 2 s.21(3), s.24). No kernel computation, agent output, lapse of time, default, or absence of objection constitutes or presumes assent.

(2) The computational legislature may propose, draft, codify, consolidate and revise only (Bill 14 s.9). It is **not sovereign**; it **may not** expand its own competence (Bill 14 s.8(3): self-extension void), amend, suspend, bypass or reweight the assent rule, or create legal force from its own output (Bill 14 s.9(2)).

(3) Subsections (1) and (2), and this subsection, are **entrenched**. They may be amended only by an express Sovereign-assented constitutional Act citing this section by number. No agent, kernel, regulation, court order, decision, or local log may amend, disapply or route around them. A record purporting to do so is **void ab initio** and a **fatal validation defect**.

```yaml
kernel_effect:
  constitutive_gate: royal_assent
  assent_non_automatable: true
  assent_non_presumable: true
  assent_non_delegable: true
  legislature_not_sovereign: true
  legislature_cannot_self_extend: true
  legislature_cannot_amend_assent_rule: true
  legislature_cannot_create_force_from_output: true
  entrenched_against_machine: true
  amendable_only_by: sovereign_assented_constitutional_act_citing_s14
  enforced_by: INV-ENTRENCHED-GATE-001
```

### 15. Apex-singleness and judicial independence [constitutional]

(1) V2's judicature is the **same one judiciary carried forward in continuity** (REALM-SC 5, REALM-SC 6). V2 does not stand up a second or parallel apex. Apex-singleness may never be relaxed and apex enactment is non-delegable.

(2) Lexby may act as advocate, advisor, engineer and registrar only (CASE-LAW s.3 as amended by Bill 28; REALM-SC 8). Lexby may not sit as the bench, author a ratio that is the decision (void ab initio under s.18(5)), or adjudicate any cause in which Lexby is advocate (nemo iudex in causa sua); the symmetric case file may never be authored by one of its two sides.

(3) The kernel is clerk, not court: it may identify court triggers but may not adjudicate, nor constitute or count a bench by computation. This section is entrenched as in s.14(3).

```yaml
kernel_effect:
  single_apex_continuity: true
  no_parallel_apex: true
  apex_singleness_non_relaxable: true
  lexby_not_bench: true
  nemo_iudex_in_causa_sua: true
  kernel_not_court: true
  entrenched_against_machine: true
```

---

## Part 6 - The single Gazette

### 16. The single Gazette, two estates; the force-source rule

(1) There is one VJS Gazette continuing the public legal thread across V1 and V2, with two estates: the V1 Archive estate and the V2 Current estate. Each entry declares its estate, source of force, and lineage.

(2) **Publication in the Gazette does not create runtime force.** V2's runtime force derives from the V2 lawpack and the kernel-recognised status of its records, never from publication. The estate boundary is operative by substance, not by caption (REALM-PC 14; REALM-SC 8 obiter per Bowan J). Publication of V2 material is not enactment by V1 and not incorporation into V1.

(3) The mechanics of export and publication are a matter for a Kernel Regulation, not this Act.

```yaml
kernel_effect:
  single_gazette: true
  estates: [v1_archive, v2_current]
  publication_does_not_create_force: true
  estate_boundary_operative_by_substance: true
  publication_not_v1_enactment: true
  publication_not_v1_incorporation: true
```

---

## Part 7 - Offices, boundary, and the real-world floor

### 17. The Principal and Lexby (by incorporation)

The offices of the Principal and of Lexby, and the firewall that Lexby holds no law-making or adjudicative drafting authority on its own account, are those of CASE-LAW s.2 and s.3 (as amended by Bill 28 giving effect to REALM-SC 8). This Act does not restate them; it incorporates them by reference, subject to the entrenchment of s.14 and s.15.

```yaml
kernel_effect:
  incorporates: [CASE-LAW:s2, CASE-LAW:s3, Bill-28]
  no_restatement: true
```

### 18. Public/private boundary and the real-world-law floor

(1) Public records carry system data only, not unredacted private facts; private evidence belongs in the configured private area; redacted summaries are permitted.

(2) Nothing in this Act authorises unlawful real-world conduct. The real-world-law floor is an external supremacy floor (REALM-SC 9); an agent may refuse, stop, narrow or escalate an instruction that appears unlawful or unauthorised.

(3) The records giving effect to this section are carried forward and verified under Schedule 2.

```yaml
kernel_effect:
  public_records_system_data_only: true
  real_world_law_unaffected: true
  agent_may_refuse_unlawful_instruction: true
  carried_forward_in: schedule_2
```

---

## Part 8 - Transitional, directory, severance

### 19. Transitional effect; directory; citations; court continuity

(1) From commencement, V2 runtime work uses the V2 lawpack and kernel. Existing V1 `REALM-*` citations remain valid historical identifiers; V1 material migrates only by express incorporation.

(2) **Bill 27** (four-branch ministry directory) is superseded as **runtime machinery** while **preserved as V1 archive law**; the public/private boundary it carries is **not** superseded but is carried forward and verified under Schedule 2.

(3) **Bill 16** is preserved as archive; the V2 `VJS-ACT` scheme stands on this Act's authority and does not depend on Bill 16 as gatekeeper.

(4) The Supreme Court, Privy Council and County Court **continue by constitutional carry-forward in continuity** (s.15); supersession of the V1 legal system does not dissolve the courts, and V2 stands up no second apex.

```yaml
kernel_effect:
  v1_citations_historical: true
  migrate_v1_only_by_incorporation: true
  bill_27_superseded_as_runtime_preserved_as_archive: true
  bill_27_boundary_carried_forward: true
  bill_16_preserved_archive: true
  v2_citation_scheme_self_authorised: true
  courts_continue_by_continuity: true
  no_second_apex: true
```

### 20. Severance

If any provision of this Act is held void or unenforceable, the remainder continues in force; but s.14 and s.15 (the entrenched guarantees) are non-severable from the commencement of this Act, and their failure stays commencement.

```yaml
kernel_effect:
  severable: true
  entrenched_guarantees_non_severable: true
```

---

## Schedule 1 - Commencement conditions (each a deterministic check)

This Act commences only when every condition is satisfied=true and recorded with evidence on the deterministic record. The `mechanism` column states the kernel check; where a check is **to-build**, commencement is stayed until it exists and passes (referral finding A; Drummond's operability requirement).

| Cond. | Condition | Mechanism (vjs) | Status |
|---|---|---|---|
| (a) | Royal Assent by the Sovereign Founder, recorded (assent_source = principal, not pending/self) | assent record + `INV-ASSENT-DRAFT-001` tightened to require principal | **satisfied 2026-06-09** (record to write) |
| (b) | Express handover order fixing the moment and entry-by-entry terms of supersession | `vjs order validate` (handover order record) | to-record |
| (c) | The constitutive gate and apex-singleness entrenched (s.14, s.15) present and fatal in the lawpack | `INV-ENTRENCHED-GATE-001` present + fatal | to-build |
| (d) | Protective-floor carry-forward (Schedule 2) present, schema-valid, incorporation-edge-resolved, verified=true | enumerated record-presence + verification check | to-build |
| (e) | Lawpack validation passes; founding lock with real content digest; founding provenance complete (no pending steps) | `vjs validate` (live) + real `vjs` digest (placeholder today) + provenance completeness check | partly live; digest + completeness to-build |
| (f) | The Sovereign's choice was genuine: the apex-preserving alternative (Bills 14/30, PC 24 / B5) and the option to decline were on the face of the matter | recorded real-alternative in the commencement record | to-record |

**Royal Assent (a) is the sole constitutive gate.** Conditions (b)-(f) are recorded preconditions; commencement is stayed until all are satisfied. A `vjs commencement-check` SHALL be the single command that decides all rows and alone flips this Act from `enacted` to `in_force`.

## Schedule 2 - Protective-floor carry-forward (enumerated and verified)

Each item is a binding V2 lawpack record tracing to its V1 source by an express incorporation edge (s.8). Commencement fails (Schedule 1 (d)) unless every item is present, schema-valid, edge-resolved and verified=true. The floor is a found fact, not a label.

| # | Floor item | V1 source | V2 record (to create) | Verified |
|---|---|---|---|---|
| 1 | Real-world-law floor | REALM-SC 9 | RULE/DEC real-world-law-floor | false (pending) |
| 2 | Rights and due process | Bill 12 | RULE rights-due-process | false (pending) |
| 3 | Judicial independence | Bill 11 | RULE judicial-independence | false (pending) |
| 4 | Public/private boundary | Bill 27; ACT-005 | (already in V2 lawpack: ACT-005) verify edge | false (pending) |

## Schedule 3 - V1 materials: superseded as runtime / preserved as archive

- **Superseded as live runtime law:** V1 case-law settlement; V1 Acts; V1 statutory instruments; V1 judgments as runtime source; V1 court-of-appeal-as-separate-runtime; V1 procedural machinery as runtime; V1 ministry four-branch directory as mandatory runtime structure (Bill 27); V1 Gazette as runtime source; V1 long hook prose as runtime instruction.
- **Preserved as archive (and citable):** the Gazette; the Archive; the law reports; historical `REALM-*` citations; migration evidence; the doctrinal record of discovery; source material for express incorporation. **Courts continue in continuity** (s.15, s.19(4)) - not superseded.

## Schedule 4 - Required properties of computer-first law

Every V2 runtime record must carry: id; citation; title; status; rank; authority basis; scope or jurisdiction; kernel_effect; supersession relationship; public/private classification; commencement state; validation state.

---

## Validation (fatal_if)

- missing recorded Royal Assent for an Act marked in_force;
- draft treated as binding;
- a ceremonial/computer-language discrepancy left unresolved;
- a Schedule 2 floor item absent, unverified, or carried by label only;
- a record purporting to amend s.14 or s.15 without a Sovereign-assented constitutional Act citing the section;
- a V1 authority in the runtime graph without incorporation (`INV-NO-V1-GAP-FILLER`).

---

## Committee note (Bill 2 s.20)

Drafted by the Standing Committee at the second-draft stage on the V2 Kernel Team's void first draft (REALM-SC 8). The four members reported: **Aldous (Restraint)** - reduce the 29-section first draft to a disciplined instrument, cite-don't-restate settled law, move companion records and packet mechanics out of the Act, one commencement clause. **Verity (Completeness)** - a Part architecture with definitions enacted, dispose of Bill 27/Bill 16/court hierarchy by number, a verified carry-forward schedule, no dangling reference. **Marlowe (Guardrails)** - entrench the constitutive gate self-referentially (s.14) with a fatal invariant, verify the protective floor as a found fact (s.18/Sch.2), record a genuine real-alternative (Sch.1 (f)), hard-wire apex-singleness and nemo iudex (s.15). **Drummond (Operability)** - express commencement as deterministic kernel checks (Schedule 1), replace the placeholder lawpack digest with a real content digest, build a single `vjs commencement-check`, and promise no enforcement the kernel cannot deliver. The Clerk-Drafter reconciled the four into this second draft: Verity's structure, trimmed by Aldous, with Marlowe's entrenched Part 5 and Drummond's honest Schedule 1. **Vote: 4 ayes, 0 nays.** Each member's aye conditions are met on the face of the draft; the build items in Schedules 1-2 are recorded as preconditions to commencement, not as present enforcement.

**Flag (Bill 2 s.17):** this Bill carries [2026] REALM-PC 24 and [2026] REALM-SC 10 through to Assent.

**Royal Assent:** granted by the Sovereign Founder, 2026-06-09. The Act is enacted and declaratory; it commences on satisfaction and recording of Schedule 1.
