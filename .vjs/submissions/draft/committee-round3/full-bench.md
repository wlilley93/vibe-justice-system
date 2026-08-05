I have completed the review. Every finding below carries the address READ or the command EXECUTED.

---

# FOUR SEATS, EIGHT VERDICTS

| Seat | `ACT-PROCEEDINGS-DISCIPLINE` (omnibus) | `ACT-RECTIFICATION-COMMISSION` (standalone) |
|---|---|---|
| **RESTRAINT** | **AYE WITH CONDITIONS** | **NAY** |
| **CODIFICATION** | **AYE WITH CONDITIONS** (ground 3 released) | **NAY** |
| **GUARDRAIL** | **AYE WITH CONDITIONS** (NAY ground cured) | **AYE WITH CONDITIONS** (NAY ground cured) |
| **OPERABILITY** | **AYE WITH CONDITIONS** | **NAY** |

## IS IT ADOPTED?

**THE OMNIBUS IS ADOPTED, 4–0, subject to the constitutive conditions below.** All four seats are AYE-WITH-CONDITIONS. This is the first text in three rounds to carry all four seats.

**THE COMMISSION ACT IS NOT ADOPTED. Three seats against.** What blocks:
- **RESTRAINT**: s6 stays *every new matter* in a jurisdiction for up to ninety days on an executive warrant, with no court order and no express route to lift; the warrant is the one act in the instrument the Act makes unreviewable.
- **CODIFICATION**: the instrument has **no schedule of express amendments at all** while varying REG-SELF-CONVENE-001, ACT-002:s6 and the entrenched due-process limb on its face. The vehicle ground was **relocated, not cured** — the omnibus's own Schedule 3 says so in terms.
- **OPERABILITY**: s10 does not commence itself; two `exceptions` limbs are silently discarded by the loader in the Act whose drafting_note names that defect; the `schedule:` its operative text relies on is not a loadable field; and the schedule's enumeration is measurably wrong in both directions.

---

# 1. THE SPLIT ITSELF — ALL SEATS

**The split is the right disposal for the omnibus and the wrong one for the Commission, because it was carried out on the omnibus and not on the Commission.**

Measured (EXECUTED, `yaml.safe_load` + token count over both files):

| | round 2 (one instrument) | round 3 |
|---|---|---|
| omnibus | 24 sections, 71 duty tokens | **18 sections, 53 duty tokens** |
| Commission | (Part 3: 6 sections, 18 tokens) | **10 sections, 52 duty tokens** |
| combined | 71 | **105** |

The omnibus fell 25% on both measures. The Commission's token load rose from 18 to 52 — that is the safeguards the compression dropped being paid for, and it is money well spent. But the combined corpus burden rose 48%.

**Did the standalone always have the safeguards the compression dropped?** Substantially yes, and I verify each against my round-2 Part 3 findings (all READ in `ACT-RECTIFICATION-COMMISSION.yaml`):

| Round-2 Part 3 defect | Standalone |
|---|---|
| trust-boundary fork STAYED | **CURED** — s6(c): a fork "goes to court before implementation regardless of reversibility"; `must_not: staying_a_trust_boundary_fork`; `must_not: a_warrant_purporting_to_narrow_a_carve_out` |
| unsatisfiable gate silently paused | **CURED** — s6(b), citing ACT-003:s10 and [2026] VJS-SC 4 |
| self-classified pause | **MOOT BY DESIGN** — nothing is classified because everything is stayed. This cures the evasion and creates a larger problem (R-1 below) |
| unbounded warrants | **CURED** — s7: 90-day cap; Act spent at 90 if no warrant; 180-day absolute; `must_not: a_warrant_exceeding_ninety_days` |
| no report before renewal | **CURED** — s8 report *before expiry*; s7 `must_not: a_second_extension_without_a_delivered_report` |
| review deferred by the reviewee | **CURED** — s9: "APPEALABLE AS OF RIGHT… an appeal is not stayed by section 6"; `must_not: staying_an_appeal_from_a_commission_act`; five set-aside grounds; prior text routable |
| power list not exhaustive; s16(h) recital-promotion | **CURED** — s4 "and nothing else. The list is EXHAUSTIVE"; the transcription power is gone entirely |
| floors as `defines` only | **CURED** — s1 `must_not: crossing_the_sovereign_assent_floor`, `derogating_from_any_limb_of_the_protective_floor`, `constituting_or_rectifying_an_apex_record_locally` |
| good-faith immunity unbounded | **CURED** — removed entirely |
| no tolling | **CURED** — s6 tolls every reservation and review date; deferred matters convene on expiry without fresh application |
| R-4 (executive taking the court's power) | **CURED verbatim** — s5: "PREPARES AND EXECUTES… never AUTHORS or MAKES… the maker is the competent court, whose order the Commission obtains BEFORE the act" |
| R-6 (retiring a store excludes a record) | **CURED verbatim** — s4, final paragraph |
| C-5 (jurisdiction register does not exist) | **CURED** — s2 cites REG-REPOS-REGISTER-001, verified at `lawpack/v2/regulations/REG-REPOS-REGISTER-001.yaml:1-6` (READ), `[2026] VJS-REG 18` |
| C-6 (reach without accession) | **CURED** — s2: "NO WARRANT ISSUES for a jurisdiction that has not acceded" |
| G-3 concurrency cap | **NOT CURED** — s2 caps a warrant to one jurisdiction but nothing caps concurrent warrants |
| G-6 reclassification shrinks the audit | **PARTIAL** — s8 reports the reclassification but not the count before and after |

So the standalone did carry the safeguards. What it never had, and still does not have, is a **vehicle**: a schedule of the amendments its stay makes. That is the defect the relocation moved rather than cured.

---

# 2. GUARDRAIL — MY NAY GROUND

**CURED, in both instruments, and I record it plainly.** My central round-2 finding was that Part 3 stayed a trust-boundary fork that binding law requires to reach court before implementation.

- The binding rule, READ at `/home/jellytot/Projects/opbox-prod/opbox-kernel/.justice/judgments/county-court/2026-cc-opbox-16.md:44`: "when a fork involves a trust boundary (any verb rated SENSITIVE or above, any EXTERNAL auth tier, any token/capability/permission model)… **The fork goes to the court before implementation.**" Carried forward at `2026-cc-opbox-17.md:54-62` (READ): item 3 "QUASHED and replaced", the finding "AFFIRMED".
- **Standalone s6(c) (READ)** places the fork in the carve-out list — "THE STAY DOES NOT REACH, AND NO WARRANT MAY EXTEND IT TO REACH" — in the drafter's own original words, with the token.
- **Omnibus s15 (READ, `:639-645`)** states the rule on its own terms, cites its holder with the repository named, and **Schedule 3 (READ, `:739`)** now says "no provision of this Act defers, stays or pauses the convening of any matter". EXECUTED grep over the omnibus for `pause|stay|warrant|commission`: every hit is in `purpose`, in the s7 commentary, or in Schedule 3's *disclaimer*. There is no pause provision.

**My NAY is withdrawn on both instruments.**

## Guardrail, fresh, on the standalone

**GF-3 — s5's hardened test is a deny-list, and I drew the list, and my list was not exhaustive.** s5 enumerates `status`, `assent_source`, `citation`, `issue`, `supersedes`, `disposes`, `exceptions`, `cites_authorities`, each directive's `id` and `when`, and catch-all keys. READ `crates/vjs-core/src/types.rs:369-447`: `Order` also carries **`court`, `jurisdiction`, `repo_code`, `created_at`, `bench`, `case_file_digest`, `convened_at`, `vote`, `appeal_of`, `appealable`, `source_opinion`, `runtime_summary`** — every one a *named* struct field, so none is reached by the catch-all limb, and none is in the test.

Consequences, each a lawful "formal rectification" with a passing proof:
- flip `appealable: true` → `false`, removing the appeal route from a filed order — a due-process act, framework s21 limb 2, entrenched at s25;
- rewrite `bench` or `case_file_digest`, destroying the auditability REG-COURT-RECORD-001 exists to create (READ `REG-COURT-RECORD-001.yaml:29-31`: `record_the_deciding_bench_on_every_new_order`, `pin_the_case_file_digest_before_the_order_issues`);
- rewrite `court` or `jurisdiction`, re-attributing an order to a different tier and thereby to a different amending court under omnibus s6 and a different answer under s1(c) apex singleness.

The cure is not a longer list. The corpus already argues this against me, READ at `types.rs` above `extra`: *"FLATTEN RATHER THAN MORE NAMED FIELDS, deliberately… A catch-all is structural: an unknown key round-trips because it is unknown, not because somebody listed it… where loss must be impossible, the mechanism cannot be a list of names."* Invert the test to an allow-list.

**GF-1 — no express route to lift or narrow a stay.** s9 makes "every act of a Commission" appealable. The **warrant** is an act of the Principal under s2, not of the Commission, so s9 does not reach the single act that closes a jurisdiction's courts. s6(d) carves out "any matter engaging a floor under section 1", and s1(b) includes "rights, standing and due process" — so a route exists, but only by the applicant classifying their own application, which is the vice this bench struck twice.

## Guardrail, on the omnibus — two round-2 conditions survive uncured

- **G-1 UNCURED.** EXECUTED over the parsed s6 text: `status` → absent; `cites_authorities` → absent; `when` → absent. s6 ¶2 still reads "Content-preserving means, and means only: the normalised token sequence of every OPERATIVE PART, and every directive's actor". The Clerk applied my G-1 wording to the **Commission Act s5** and not to the omnibus s6 it was addressed to. The narrow test now sits in a permanent Act while the wide one sits in a 180-day one.
- **G-5 UNCURED.** EXECUTED over the parsed s2 text: `assent_source` → absent, `never blocked` → absent, `ASSENTED-RECORD` → absent. `lawpack/v2/statutes/10-assented-record-protection.yaml:22-30` (READ) forbids an assented record being "voided, excluded, or **blocked**… including an inert or unevaluable kernel_effect, **an operation not yet implemented in the kernel registry**" — the exact class s2 refuses on. Every other refusing section of the Act (s3, s7, s10, Sch 1 ¶3) carries the bifurcation. s2 alone does not.

**G-12 CURED** — Sch 1 ¶4 now reads "RESERVED TO THE SOVEREIGN, assent being non-delegable (ACT-CONSOLIDATION-FRAMEWORK:s10); no other organ acts on the face of a constitutional Act." Verified against `09-consolidation-framework.yaml:53-58` (READ): s10 is "The assent floor", "non-automatable, non-presumable, non-delegable".

---

# 3. CODIFICATION — GROUND 3, THE VEHICLE

## The vehicle ground is RELEASED as against the omnibus.

I do **not** require physically separate amending schedules. The two limbs of my ground were (i) Schedule 3 declares amendments it does not draft and (ii) it mis-states the amendment set. Limb (ii) is cured and limb (i) I now abandon as a preference, not a ground — an amendment whose extent is stated in the amending section, enumerated in a schedule, with a correction route for a variation found outside the set, is this corpus's ordinary vehicle.

The cure, READ at `sch3:748-755`:
> "IT VARIES [2026] VJS-PC 17 D1 NOT AT ALL. No other instrument is varied, and **this sentence is stated as a bounded claim rather than an absolute**: the enumeration above is the amendment set the Clerk-Drafter measured, and a variation found outside it is a defect in this Schedule correctable on the s6 slip track, never an implied amendment. In particular this Act does NOT vary REG-COURT-RECORD-001…, REG-SELF-CONVENE-001, ACT-003:s5 or ACT-003:s10: the draft pause… is no longer in this Act, **and any instrument that proposes such a pause must declare those variations for itself.**"

That is the right form and the last clause is exactly right. It is also the sentence that convicts the Commission Act.

## The four fixes to my other conditions

| Condition | Verdict | Verification |
|---|---|---|
| **C-2(c) — s10 vs REG-COURT-RECORD-001** | **CURED, verbatim** | s10 now: "reported UNSTATED and the reader never supplies a bearer; WHETHER SUCH A DIRECTIVE BINDS, AND WHOM, IS RESERVED TO THE COURT, and nothing in this section invalidates a legacy ruling for want of a structured field (REG-COURT-RECORD-001, which binds prospective rulings only)." Verified against `lawpack/v2/regulations/REG-COURT-RECORD-001.yaml:27,:34` (READ): `binds: prospective_rulings_only`; `must_not: invalidate_a_legacy_ruling_for_want_of_the_structured_fields`. "binds NOBODY" is gone. |
| **C-5 — repos register citation** | **CURED** | Sch 1 ¶1: "Repo is a value on the repos register kept under REG-REPOS-REGISTER-001". Verified `REG-REPOS-REGISTER-001.yaml:1-6` (READ), `[2026] VJS-REG 18`, authority framework s7. The phantom "jurisdiction register" is gone from both instruments. |
| **C-1 — floors named by their true sections** | **CURED IN SUBSTANCE, ONE MIS-ATTACHMENT** | s15 now cites "ACT-ASSENTED-RECORD-PROTECTION:s1 and s2, and **ACT-CONSOLIDATION-FRAMEWORK:s10 and s25**". Verified: `09-consolidation-framework.yaml:53` (READ) s10 **is** "The assent floor"; `:137-154` s25 entrenches s10, s11, s21. The wrong "s7" is gone. **But** "and s11 as entrenched by s25" is placed inside the *protective-floor* parenthetical, while `09-consolidation-framework.yaml:69-83` (READ) shows s11 is "Courts in continuity", `defines: single_apex: true` — apex singleness, which the same sentence cites only by [2026] VJS-SC 4. Right sections, one in the wrong bracket. |
| **C-4 — the continuity-citator reclassification** | **THE FIX IS DEFENSIBLE. THE ACT CONTRADICTS IT TWO HUNDRED LINES EARLIER.** | See below — this is my constitutive ground. |

### C-4: is the new classification actually right?

Sch 1 ¶2 (READ, `:672-679`) now classifies the bare `[YEAR] SERIES-REPO N` form as "**A REGISTERED LEGACY RENDER FORM OF ESTATE V2 AND NOT AN ESTATE V1 FORM**", and adds the saving I asked for: "Registering a render form is an act on IDENTITY and never on force: it neither incorporates an estate v1 record (ACT-001:s4; ACT-CONSOLIDATION-FRAMEWORK:s20, which require an express incorporation record) nor alters the force of any estate v2 record."

Checked against the two instruments I named:
- `lawpack/v2/statutes/01-authority.yaml:68-83` (READ) — ACT-001:s4, `must_not: treat_v1_judgments_as_binding_without_incorporation`.
- `lawpack/v2/statutes/09-consolidation-framework.yaml:99-115` (READ) — s20, "V1 law not in Schedule 1 has live force only by an express incorporation record… nothing crosses by silence… A fail-closed incorporation-validity invariant rejects a missing element".

The saving disclaims force in **both** directions, so neither instrument is tripped. On that basis the fix works, and it is the first branch of the disjunctive cure I offered. **But I must correct my own round-2 reasoning against myself.** I asserted CC-OPBOX 16 was "a live 2026 subscriber ruling" and not archive law. READ `2026-cc-opbox-16.md:6` and `2026-cc-opbox-17.md:6`: **both are dated `2026-06-07`**, and both `decided_via: "…CASE-LAW s.10/s.18"`. READ `lawpack/v2/provenance/founding/COMMENCEMENT-V2-0001.yaml:4`: V2 `commenced: "2026-06-09"`. **Both judgments predate V2 commencement by two days and were decided under V1 case-law.** EXECUTED `grep -rn "CC-OPBOX 16\|cc-opbox-16" lawpack/` → **zero**: there is no incorporation record and no migration-ledger row anywhere. So "estate v2" is a bare assertion the dates do not support, and my round-2 premise for it was wrong. It does no harm only because of the force-disclaiming saving — and because both instruments restate the trust-boundary rule **on their own terms** rather than resting its force on the judgment.

### C-4′ — MY CONSTITUTIVE GROUND: the Act contradicts its own fix on its own face

- READ `.vjs/submissions/draft/ACT-PROCEEDINGS-DISCIPLINE.yaml:647-649` (s15): "The Schedule 1 migration registers the continuity series as **estate v1** so that rule becomes visible to the grounding gate for the first time."
- READ `.vjs/submissions/draft/ACT-PROCEEDINGS-DISCIPLINE.yaml:674` (sch1 ¶2): "…**WHICH IS A REGISTERED LEGACY RENDER FORM OF ESTATE V2 AND NOT AN ESTATE V1 FORM**".

Two operative provisions of one Act give opposite classifications of the same series, in the section carrying the Act's showpiece citation. The Clerk cured C-4 in Schedule 1 and left the sentence C-4 was aimed at standing in s15. Under the Act's own s6 substantive track this is an "error on the face… where the order's own text contradicts itself" — the Act would be, on assent, its own first candidate.

---

# 4. RESTRAINT — MY SEVEN PART 3 CONDITIONS

**R-1, R-2, R-3, R-4, R-5, R-6, R-7 were all on Part 3. Against the omnibus, all seven are MOOT.** Against the standalone: **R-1 CURED** (s7's ninety-day self-expiry if no warrant issues — my wording adopted in substance), **R-4 CURED verbatim** (s5), **R-5 CURED** (s2/s4's one-jurisdiction confinement with the token), **R-6 CURED verbatim** (s4's store-retirement rule), **R-3 MOOT** ("editorial" no longer does classificatory work), **R-7 transformed** into the s10 commencement incoherence.

**R-2 is not cured — it is enlarged, and it is my NAY.** My round-1 condition 5 was that a statute may not command a court not to sit. Round 2 it reappeared at Part 3 s17 for a class of matters, 30 days, unclassified. Round 3 it appears at standalone s6 for **every** new matter, **90 days**, extendable once. That is the third appearance and each is wider than the last. The four carve-outs and the tolling rule are real improvements and I record them. They do not answer the point: no court order convenes the stay, no application opens it, no cap limits concurrent warrants (READ s2 — one jurisdiction per warrant, nothing on how many warrants), and on the schedule's own enumeration eight or nine warrants would close every court in the realm simultaneously.

## Is the omnibus now within a defensible size?

**Yes.** 24 entries → **18**; 71 duty tokens → **53** (EXECUTED count over the parsed YAML). My nine-section minimum is **not met but near**: s1, s2, s3, s6, s7+Sch 1, s9, s10, s13, s15 remain my nine; the surplus is now **five** (s4, s5, s8, s11, s12) where in round 2 it was fifteen. Each of the five is a subordinate-legislation candidate, not a defect, and I record my preference without pressing it. **The size ground is discharged and Restraint does not vote on it.**

## Restraint on the omnibus — one round-1 condition still partial

**Condition 9** remains PARTIAL. s15 states the store-by-store mirror duty, but the known divergence is still on no instrument's face. Re-EXECUTED 2026-08-05: `diff -rq` canon ↔ `opbox-kernel/.worktrees/tablelist-ext/lawpack/v2/statutes/` — `03-agent-duties.yaml` differs and `10-assented-record-protection.yaml` is **absent**. A divergent unique copy of an entrenched Act sits in an unregistered store and no instrument records it. **Condition 11 limb (b) is CURED**: s15 ¶2, "THE COMMENTARY KEY OF EVERY SECTION OF THIS ACT IS EXPLANATORY AND NON-OPERATIVE."

---

# 5. OPERABILITY — CONSTITUTIVE CONDITIONS

## Does the omnibus still self-condemn? **NO — and the disclosure is now accurate.**

EXECUTED cross-check, parsed YAML × `GATE_REGISTRY` string extraction from `crates/vjs-lawpack/src/report.rs:36-241` (55 rows):

```
OMNIBUS:    duty tokens 53   hits 0   misses 53
COMMISSION: duty tokens 52   hits 0   misses 52
```

Under s12 as bounded by my C14 — "attaches to a gate-registry row and not to duties the audit already reports UNWIRED" — none of the 53 claims enforcement, so none owes a control. It does not self-condemn, and the mechanism (not a timing accident) is why.

## The Clerk's seven-token claim — **VERIFIED TRUE**

Schedule 2: "Parts 1 and 4 commence on assent and carry seven duty tokens between them (**s1 two, s14 two, s15 three**)". EXECUTED per-section count: s1 `must_not`×2 = 2; s14 `must`×2 = 2; s15 `must`×1 + `must_not`×2 = 3. **Total 7. Exact.**

And **O-1 is CURED**: the false absolute "At no point does a section of this Act stand in force as a duty with no gate" is replaced by "NO PART **2** SECTION STANDS IN FORCE AS A DUTY WITH NO GATE… which bind no gate at commencement and are reported UNWIRED by the audit, honestly. **That is disclosed rather than denied**, because the audit has no representation of commencement at all and reads the whole-Act figure from the day this text lands." Verified against `crates/vjs-lawpack/src/conformance.rs:57-77` (READ): `conformance_audit` iterates every statute section carrying a `kernel_effect` with no filter on status, Part, organ or commencement. **O-2 is cured as disclosure**, though the projected figure my remedial wording required is not stated.

## The projection

EXECUTED baseline, `./target/release/vjs audit --json` in canon: `total 281, wired 43, unwired 238`.

**On assent of both instruments: `total 386, wired 43, unwired 343`. Wired share 15.30% → 11.14%.** The omnibus alone: `total 334, wired 43, unwired 291`, share 12.87%.

## The standalone Commission Act

**Does it self-condemn? NO.** s10 is the right form: "This Act's own conformance status is therefore PUBLISHED WITH THE AUDIT and is not asserted here", naming `crates/vjs-lawpack/src/report.rs:36-241` — verified, that is where `GATE_REGISTRY` begins and ends (READ, `:36` and `:241`).

**Is its commencement coherent? NO — three ways, all at s10 (READ).**

1. **s10 does not commence itself.** "Sections 1, 2, 7 and 9 commence on assent… **Each remaining section** commences on the day the instrument binding it is in force." s10 is a remaining section. The rule that governs commencement is conditioned on machinery that does not exist, so on assent day nothing sets the commencement of anything.
2. **s10 breaches its own stated principle.** It says "no section conferring a power commences before the constraint that bounds it is in force" — yet **s5** (the content-preservation test bounding s4(a) and s4(f)), **s6** (the four carve-outs bounding the stay) and **s8** (the report) are all off the assent list, while **s7** is on it and requires "a report under section 8 having been delivered" before a second extension.
3. **"the instrument binding it" is undefined and unassigned.** No section names it, no organ owes a duty to make it, and s7 spends the Act at 180 days regardless. On its own terms the entire operative core — s3, s4, s5, s6, s8 — is capable of never commencing, and s2 (in force on assent) can warrant a Commission that has no powers, no audit duty, no test, no stay and no report.

**Two `exceptions` limbs are silently discarded.** EXECUTED parse: the Commission Act's only two `exceptions` blocks (s3, s8) sit at **section level**, not inside `kernel_effect`. READ `crates/vjs-lawpack/src/lib.rs:266-273`: `StatuteSection` is `{id, title, text, commentary, kernel_effect}` — no `exceptions` field. EXECUTED `grep -rn deny_unknown_fields crates/vjs-lawpack/src/` → **0 matches**. Both limbs are dropped on load, so seven of the Act's fifty-two tokens (s3's three, s8's four) will sit in the unwired list with no honest "no gate holds this" marker. This is the defect the Act's own `drafting_note:37-38` names in terms: *"eight deserialisation structs with no `deny_unknown_fields`, so a misspelled key in any instrument is silently discarded."* The omnibus placed all seven of its `exceptions` correctly inside `kernel_effect` (EXECUTED).

**The `schedule:` the operative text relies on is not loadable.** READ `lib.rs:255-263`: `Statute` is `{id, citation, title, status, enacted_by, purpose, sections}`. There is no `schedule` field and no `deny_unknown_fields`. s2 ("enumerated in the schedule") and s6(c) ("address in the schedule") point at content the reader discards. The omnibus avoided this by making its schedules `sections`.

---

# 6. RECITAL CHECK — ALL FOUR SEATS

## The five corrections

**(a) s7's DEC recital — TRUE.** EXECUTED over the parsed s7: `removed` → absent; `PC 13` → absent. `condemn` appears once only, in "where a matter later condemns a combination a tombstone" — it does not describe the DEC decision. The claim is now: "the label VJS-DEC 15 is already in use across the corpus to denote a specific prior decision of the unitary-stack programme (EXECUTED grep: 14 files in canon, 41 under the opbox `.justice` subtree, meaning the SERIES DEC 15-22 per SUBMISSION-2026-08-04-221305, not ordinal 15 alone)." The 41 verified (EXECUTED). The series reading verified at `SUBMISSION-2026-08-04-221305:18` (READ): "About forty files cite the series". **Two residues, both ORDINARY:** re-EXECUTED today the canon count is **15**, not 14 — my own round-2 report is the fifteenth file, and the recital carries no date; and the phrase "which is the **stayed** question" is orphaned — nothing in either instrument now stays it, and the same paragraph correctly calls it "pending".

**(b) s9's unreadable count — TRUE, and correctly dated.** "on 2026-08-04 the opbox jurisdiction stood at 55 of 109… ([2026] VJS-CC-OPBOX 160). Executing 160 took that count to 2 without touching a record (161), and it stands at **0 of 116 as at 2026-08-05**". Verified: `2026-VJS-CC-OPBOX-160.yaml:9` (READ) "55 of 109 order files… do not parse"; `2026-VJS-CC-OPBOX-161.yaml:13` (READ) "took the unreadable count from 55 to 2 without touching a record"; **EXECUTED `./scripts/verify-orders-are-readable.sh` in opbox-kernel → "PASS: all 116 filed orders are readable and in the citator."** All three figures correct, the trajectory shown, the date on the face.

**(c) s12's negative-control claim — TRUE.** "No negative-control REGISTER exists (EXECUTED find), so no duty can be shown to carry one; individual controls and at least one debt ratchet do exist in the test tree, and an earlier draft's claim that neither existed anywhere was a bounded search reported as a fact about the corpus - corrected here rather than deleted." Verified: EXECUTED `find . -iname "*negative*control*"` → no results; EXECUTED `grep -rli "negative control" crates/` → **8 files**; READ `crates/vjs-testkit/tests/global_invariants_gate.rs:65` → `fn global_invariants_are_bound_and_debt_ratchets_down`. Both admissions land, and the correction is recorded on the face rather than deleted, which is the discipline s2 exists to create. The `281 / 43 / 238` line re-verified by EXECUTION today.

**(d) s2's ACT-003:s8 / ACT-004:s3 description — TRUE.** "ACT-003:s8 prohibits adding a model call to the kernel core (**subject to its adapter exception**), and ACT-004:s3 fixes a closed predicate registry which forbids evaluating an invariant by model, by cosine or by free-form script." Verified `lawpack/v2/statutes/03-agent-duties.yaml:123-137` (READ): `must_not: add_model_call_to_vjs_core`, `exceptions: adapter_crate`. Verified `lawpack/v2/statutes/04-records-logs-citations.yaml:48-64` (READ): "uses a fixed predicate registry"; `must_not: use_llm_to_evaluate_invariant`, `use_cosine_for_invariant`, `use_free_form_script_for_invariant`. The round-2 error ("the capability is removed, not prohibited") is gone and the right instrument now carries the removal limb. One quibble I do not press: s3's registry list ends "etc.", so "closed" is a fair characterisation rather than a literal one.

**(e) s15's showpiece citation — TRUE.** Now `opbox-prod/opbox-kernel:.justice/judgments/county-court/2026-cc-opbox-16.md:44`, mode READ, "the repository is named because s2 requires it". Verified: EXECUTED `git rev-parse --show-toplevel` in that tree → `/home/jellytot/Projects/opbox-prod/opbox-kernel`, so the named root is the true repository root; line 44 READ and quoted above. The Act's own s2 repo requirement is now satisfied by its own showpiece.

## NEW FALSE RECITALS — three, and one of them blocks

**NEW-1 (omnibus, CONSTITUTIVE). The estate contradiction.** `s15:648` "registers the continuity series as **estate v1**" vs `sch1:674` "**ESTATE V2 AND NOT AN ESTATE V1 FORM**". Both READ. One Act, two operative provisions, opposite classifications of one series.

**NEW-2 (omnibus, CONSTITUTIVE). Schedule 2 names two sections that do not exist.** `sch2:715` (READ): "Tranche 0, on assent: **s20, s21**, and the s13 store register duty in reporting form." EXECUTED parse: this instrument has `s1`–`s15`, `sch1`, `sch2`, `sch3`. `s20` and `s21` are the *round-2* numbers for Commencement and Extent (confirmed by `git show dea3c99:…` — s20 "Commencement by section", s21 "Extent and savings"). Their round-3 successors are s14 and s15, which the **very next paragraph of the same schedule** names correctly. The commencement schedule therefore does not resolve, in an Act whose keystone is machine-resolvable identity.

**NEW-3 (Commission Act, CONSTITUTIVE). The jurisdiction enumeration is wrong in both directions, and the Act's own s3 condemns it.** s2 recites "Eight jurisdictions presently carry a `.vjs/` surface (EXECUTED 2026-08-05, enumerated in the schedule)"; the schedule records the method as `test -d <repo>/.vjs over /home/jellytot/Projects/*/`.
- **EXECUTED, that exact glob, 2026-08-05: NINE**, not eight. The omission is **`/home/jellytot/Projects/Vibe Justice System/`** — a distinct git repository (EXECUTED `git remote -v` → `https://github.com/wlilley93/vibe-justice-system-dev.git`) carrying its own `.vjs/orders` (1 order), its own `.justice` (1 judgment), its own `lawpack/v2/statutes` (10 files), and the live order `[2026] VJS-CC-VJS 1` (EXECUTED `git log`).
- **One of the eight named is a symlink to another.** EXECUTED `ls -ld /home/jellytot/Projects/agent-universe-v2` → `-> vibe-justice-system`; EXECUTED `readlink -f agent-universe-v2/.vjs` → `/home/jellytot/Projects/vibe-justice-system/.vjs`. The same surface is listed twice under two names, and the schedule's note discloses only that three entries are security-scan copies.

s3 of that same Act (READ): "An enumeration that omits a store is not an audit of the jurisdiction, and the report must name the stores it did not reach." The Act's founding measurement fails its own s3. Round three, fourth instance of the family.

**One further false recital, ORDINARY (both instruments).** The Commission Act `drafting_note:30` recites "68 of 116 orders and 61% of governance words about the machinery"; the omnibus commentary at s13 recites "over a hundred citation-bearing judgments". Neither carries an address, a query form or a per-store count — the discipline omnibus s8 imposes on everyone else. And **the omnibus `purpose:19` reads "to create **and** the amendment power the system lacked"** — a mangled clause left by the excision of the Commission limb.

---

# 7. SURVIVING CONDITIONS

## CONSTITUTIVE — these block adoption of the instrument named

**1. OMNIBUS s15 (`:647-649`) — the estate contradiction. [CODIFICATION, GUARDRAIL]**
Delete "The Schedule 1 migration registers the continuity series as estate v1 so that rule becomes visible to the grounding gate for the first time." and substitute:
> "The Schedule 1 migration registers the continuity render form so that rule becomes visible to the grounding gate for the first time; the form is a registered legacy render form of estate v2 (Schedule 1 ¶2), and registering it is an act on identity and never on force."

**2. OMNIBUS sch2 (`:715`) — two sections that do not exist. [CODIFICATION, OPERABILITY]**
Delete "Tranche 0, on assent: s20, s21, and the s13 store register duty in reporting form." and substitute:
> "Tranche 0, on assent: Parts 1 and 4 (s1, s14, s15), Schedules 2 and 3, and the s13 store register duty in reporting form. Schedule 1 commences with s7; Schedule 3 commences, as to each amendment it enumerates, on the commencement of the section that makes it and not before, until when the amended provision stands unvaried."

(This also discharges O-3, which is otherwise ORDINARY.)

**3. OMNIBUS s6 ¶2 — the slip-track test is a deny-list and is under-inclusive. [GUARDRAIL, G-1 uncured]**
After "and every directive's actor", insert:
> "— and, in addition and to the same standard, EVERY OTHER FIELD THE RECORD CARRIES, whether named on the reader's structure or preserved under its catch-all, save only the fields this section expressly permits to be re-rendered. Included beyond doubt: `status`, `assent_source`, `citation`, `issue`, `court`, `jurisdiction`, `repo_code`, `supersedes`, `disposes`, `exceptions`, `cites_authorities`, `bench`, `case_file_digest`, `convened_at`, `vote`, `appeal_of`, `appealable`, and every directive's `id` and `when`. A change to any of them is not content-preserving, is not on the slip track, and is not a rectification of form, whatever it is labelled. The test is stated as what may change and not as what may not, because where loss must be impossible the mechanism cannot be a list of names."

**4. OMNIBUS s2 — the assent bifurcation. [GUARDRAIL, G-5 uncured]**
After "names each defective entry", insert:
> "A record declaring a valid assent_source is never blocked by this section; the defect is surfaced and routed for correction (ACT-ASSENTED-RECORD-PROTECTION:s1, cited by number)."

**5. COMMISSION ACT — no schedule of express amendments. [CODIFICATION — the relocated vehicle ground]**
Add a Schedule of Express Amendments declaring, with section and extent:
> "ACT-002:s6 (s6 of this Act defers the convening of a matter on triggers (1) first impression, (2) distinction, (3) variation or overruling and (4) conflict, for the duration of a warrant only; trigger (5) breach is not deferred); REG-SELF-CONVENE-001 `[2026] VJS-REG 19` (s6 defers the own-motion convening duty for a first-impression question, a distinction, a proposal to overrule and a conflict, and does not defer it for a discovered breach); and ACT-CONSOLIDATION-FRAMEWORK:s21, second limb (rights, standing and due process), cited by number, s6 being drawn so that no derogation arises because s6(d) carves out every matter engaging a floor."
Verified: `lawpack/v2/statutes/02-courts-orders.yaml:88-101` (READ), five court triggers; `lawpack/v2/regulations/REG-SELF-CONVENE-001.yaml:8-21` (READ), a fork is "a first-impression question, a genuine distinction, a proposal to overrule, a discovered breach, or a conflict", disposed of by convening on own motion, "the route gate and the functional hook (REG-HOOKS-001) **fail closed**"; Commission s9 (READ) expressly says a first-impression referral "goes to the court in the ordinary way **after the stay lifts**."

**6. COMMISSION ACT s6 — the stay is a total closure with no lift route and no concurrency cap. [RESTRAINT, GUARDRAIL]**
Replace the first sentence of s6 with:
> "While a warrant is live, the courts of the named jurisdiction shall not convene a new matter **that falls within the rectification schedule the Commission has entered on the public record and certified**, and no other matter is reached. A certification is a DECLARATION, not a finding; it binds no court; and any person, and a court on its own motion, may apply to lift or narrow the stay. **An application to lift or narrow a stay, and any challenge to a warrant, is a floor matter within paragraph (d), is never stayed, and is determined before the matter it concerns is entered on any register.**"
And add to s2: "No more than THREE warrants run concurrently across the realm."

**7. COMMISSION ACT s10 — the commencement section does not commence. [OPERABILITY]**
Substitute the first sentence:
> "Sections 1, 2, **5, 6, 7, 8, 9 and 10** commence on assent, because they are the constraints and the accountability; no section conferring a power commences before the constraint that bounds it is in force. Sections 3 and 4 commence on the day the gate binding them is in force, **and if no such gate is in force at the date this Act is spent under section 7 they never commence, which is intended.**"

**8. COMMISSION ACT s3, s8 and the top-level `schedule:` — silently discarded by the reader. [OPERABILITY]**
Move both `exceptions:` blocks inside their sections' `kernel_effect:` (the only recognised location — READ `crates/vjs-lawpack/src/lib.rs:275-284`), and re-express the top-level `schedule:` as a numbered section within `sections:`, as the omnibus does. Neither key is a field of its struct and neither struct denies unknown fields.

**9. COMMISSION ACT s2 and schedule — the founding measurement is wrong. [ALL SEATS]**
Substitute in s2: "**Nine** directories under `/home/jellytot/Projects/` carry a `.vjs/` surface (EXECUTED 2026-08-05, `test -d <repo>/.vjs` over `/home/jellytot/Projects/*/`), of which **eight are distinct**: `agent-universe-v2` is a symbolic link to `vibe-justice-system` and is the same surface." Add to the schedule's entries: `Vibe Justice System` (a distinct repository, remote `vibe-justice-system-dev`, holding `[2026] VJS-CC-VJS 1`, its own `.justice`, and its own `lawpack/v2/statutes`), with the note that whether it is a jurisdiction, a fork or a stale copy is for the first audit and the Principal.

## ORDINARY — curable at engrossment

10. **OMNIBUS `purpose:19`** — "to create **and** the amendment power the system lacked" → "to create the amendment power the system lacked".
11. **OMNIBUS s15** — move "and s11 as entrenched by s25" out of the protective-floor bracket into the apex-singleness bracket: "apex singleness ([2026] VJS-SC 4; ACT-CONSOLIDATION-FRAMEWORK:s11 as entrenched by s25)".
12. **OMNIBUS s7 commentary** — date the grep and correct the count: "EXECUTED grep 2026-08-05: 15 files in canon, 41 under the opbox `.justice` subtree"; and replace "the **stayed** question" with "the **pending** question", no provision of this Act staying anything.
13. **OMNIBUS s13** — O-4 uncured. EXECUTED `ls -d .justice` in canon → "No such file or directory". Substitute: "A jurisdiction whose law or court records are held in a tree outside its declared roots registers that tree on accession; the opbox jurisdiction registers `.justice` on its accession under s15, and this section states no duty on any store in the canonical jurisdiction that does not exist there."
14. **OMNIBUS s15** — Restraint condition 9, still partial. Add: "As at 2026-08-05 the known divergence is recorded: `opbox-kernel/.worktrees/tablelist-ext/lawpack/v2/statutes/` holds a divergent `03-agent-duties.yaml` and does not hold `10-assented-record-protection.yaml` (EXECUTED `diff -rq`); it is an unregistered store and is routed for correction."
15. **OMNIBUS s14** — add the projected figure my O-2 required: "projected `total 386, wired 43, unwired 343` on assent of this Act and the Commission Act together, against the baseline `281/43/238` measured by EXECUTION of `vjs audit --json` on 2026-08-05".
16. **OMNIBUS, Part numbering** — the Act now runs Parts 1, 2, 4. Renumber Part 4 as Part 3, or state on the face that the gap records the removed Part.
17. **OMNIBUS, schedule ids** — round-2 defect 5 uncured: ids are `:sch1/:sch2/:sch3` but every cross-reference is prose, and `2026-VJS-PC-017.yaml:184` (READ, D5) normalises `:s.n`/`:s n`/`:sn` only. Adopt `:s16/:s17/:s18` or register the `sch` form.
18. **COMMISSION ACT s1(c)** — add the entrenched section: "apex singleness ([2026] VJS-SC 4; ACT-CONSOLIDATION-FRAMEWORK:s11, entrenched at :s25)".
19. **COMMISSION ACT s8** — G-6 partial: add "with the audited duty count before and after" to the s4(e) reclassification reporting duty.
20. **BOTH INSTRUMENTS** — record that the trust-boundary rule's holders, `[2026] CC-OPBOX 16` and `17`, are both dated `2026-06-07` (READ, `:6` of each), two days before V2 commenced on `2026-06-09` (READ `COMMENCEMENT-V2-0001.yaml:4`), were decided under CASE-LAW s.10/s.18, and carry no incorporation record (EXECUTED `grep -rn "CC-OPBOX 16" lawpack/` → zero). The rule's canonical force comes from these Acts restating it, not from those judgments, and each Act should say so in one sentence.

---

# 8. IF ADOPTED — WHAT THE CLERK MUST DO

The omnibus is adopted. To engross it for Sovereign Assent:

1. **Apply conditions 1–4 (constitutive) and 10–17 (ordinary) to the text.** No other change. Every edit is a substitution of stated words at a stated address; none touches a holding.
2. **Strip the draft-only keys** the loader does not model: `drafting_note` and `created_at` are not fields of `Statute` (READ `crates/vjs-lawpack/src/lib.rs:255-263`) and would be silently discarded — remove them expressly rather than let the reader do it. Leave `commentary` in place: it **is** modelled (`StatuteSection.commentary`, READ `:270`) and s15 ¶2 now declares it non-operative.
3. **Set `status`** from `draft`, and **`assent_source`** from `SECOND_DRAFT_PENDING_STANDING_COMMITTEE_ADOPTION` to the value the assent instrument fixes. Note that `assent_source` is likewise not a field of `Statute`; the Clerk should confirm the route by which INV-ASSENT-SOURCE-001 reads it before engrossment, and say which.
4. **Carry NO `citation:` key.** EXECUTED parse confirms none is present. The VJS-ACT ordinal is minted deterministically at commencement. Do not self-mint; the corpus records two void self-mints already.
5. **File this report and the round-1 and round-2 reports** alongside the engrossed text, and record the adoption as `Standing Committee, 4-0 (constitutive, [2026] REALM-SC 8)` — the form verified at `lawpack/v2/provenance/founding/COMMENCEMENT-V2-0001.yaml:14` (READ).
6. **Pin the assent to the digest of the engrossed text**, and record in the assent instrument that four conditions were constitutive and where each landed, so a later reader can check the adopted text against the conditioned one without re-running the Committee.
7. **Return the engrossed text to this Committee for a digest check only** — not a fresh vote. Our four AYEs attach to the text as conditioned, and the only question on return is whether conditions 1–4 landed in the words stated.

**The Commission Act does not go to assent.** It returns to the Clerk-Drafter as a second draft, on conditions 5–9 (constitutive) and 18–20 (ordinary). Its safeguards are sound and were sound before the compression damaged them; what it lacks is a schedule of the amendments its stay makes, a stay narrow enough not to need one, a commencement section that commences, and a founding measurement that survives its own s3.