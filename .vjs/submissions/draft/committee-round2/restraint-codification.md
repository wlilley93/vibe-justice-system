I have completed the review. All findings below are READ at a stated address or EXECUTED with the command named.

---

# RESTRAINT VERDICT: **AYE WITH CONDITIONS** (7 new, on Part 3)
# CODIFICATION VERDICT: **NAY** (ground 3 maintained and enlarged; two new grounds)

---

## CONDITION LEDGER — RESTRAINT (round 1, 16 conditions + drop list)

| # | Round-1 condition | Status | Where |
|---|---|---|---|
| 1 | s1 kernel-refusal limb mandates a check ACT-004:s3 forbids | **CURED** | `ACT-PROCEEDINGS-DISCIPLINE.yaml:136-149` (s2): duty attaches to a structured `machinery_claims` block; "The kernel does not, and may not, decide by reading prose whether a sentence asserts machinery behaviour". `must` limbs are form-only. |
| 2 | `cite_this_section_and_not_act_002_s7` varies PC 17 D1 | **CURED** | s4 `:228-233` — "the gate cites this section IN ADDITION TO, never instead of, the authorities D1 named". Verified against `lawpack/v2/orders/2026-VJS-PC-017.yaml:172` (READ), which does direct the ACT-002:s7 + REG-KERNEL-001 citation. Sch 3 `:889` declares D1 "varied not at all". |
| 3 | Reduce s3 to the footing sentence | **CURED** | s4 `:221-234` operative text is footing + non-variation + the new UNGROUNDED limb; the "WHY THIS SECTION IS NEEDED" argument is at `commentary: :235-243`. |
| 4 | s4 recital false; interim rule duplicative; absolute self-defeating | **CURED structurally** | First-draft s4 deleted; replaced by s7 `:364-385` + Sch 1 `:814-849`, allocating over the STORE REGISTER rather than an absolute "every store". |
| 5 | s7 bench-binding limb — a statute may not command a court not to sit | **CURED at s5 — but REOPENED at s17.** | s5 `:266-269`: "A bench retains jurisdiction to sit… on its own motion… Nothing in this section prevents the route returning CourtRequired." **However s17 `:684-686` now does by statute exactly what condition 5 forbade**: it stops first-instance courts convening on a whole class. See NEW CONDITION R-2. |
| 6 | Trust-boundary savings cite the wrong authority | **CURED** | s21 `:796-799`. Verified: `/home/jellytot/Projects/opbox-prod/opbox-kernel/.justice/judgments/county-court/2026-cc-opbox-16.md:44` (READ) holds the rule in those words; `2026-cc-opbox-17.md:54-62` (READ) quashes remedy items 3 and 4 and carries the duty forward. Both the file and the line are as cited. |
| 7 | s8 must amend ACT-004:s9 expressly or yield | **CURED** | s6 `:318-323` + Sch 3 `:885-887`. Checked against `lawpack/v2/statutes/04-records-logs-citations.yaml:135-149` (READ): `must_not: delete_old_records` is left standing and the verbatim-prior-text device is a coherent answer to it. |
| 8 | s11 "NOT IN FORCE" breaches the entrenched floor; lowers 160 O5 | **CURED** | s9 `:441-456`: UNREADABLE-IN-FORCE; fail-closed-and-exit-non-zero restored verbatim; ACT-010:s1/s2 cited by number. |
| 9 | s15 mirror claim breaches s10; divergent worktree | **PARTIAL** | The false claim is gone and s21 `:784-787` states a store-by-store duty. But my condition required the measurement and the known divergence **on the face**, and neither appears anywhere in the Act. Re-EXECUTED today: `diff -rq` canon↔`opbox-kernel/lawpack/v2/statutes/` → clean; canon↔`opbox-kernel/.worktrees/tablelist-ext/lawpack/v2/statutes/` → `03-agent-duties.yaml` still differs and `10-assented-record-protection.yaml` is **still absent**. The defect is live and unrecorded in any instrument. |
| 10 | No-local-amendment removes preserved routes | **CURED** | s21 `:778-783`. Verified `ACT-007:s3` (`07-federation.yaml:37-49`) and `ACT-001:s9` (`01-authority.yaml:149-163`) carry the exceptions cited. |
| 11 | Narrative out of operative text | **PARTIAL** | Limb (a) done: 12 sections carry a `commentary:` key (parsed; `commentary: Option<String>` is a recognised field at `crates/vjs-lawpack/src/lib.rs:272`, READ — so no loader breakage). **Limb (b) OUTSTANDING**: EXECUTED `grep -ni "non-operative\|explanatory"` over the draft → **zero hits**. The only statement that commentary is non-operative sits at `:32`, inside `drafting_note`, which `:27-28` declares "DRAFT-ONLY KEY, stripped at enactment". On enactment nothing says commentary is non-operative and nothing bars a ratio resting on it. |
| 12 | Superseded instruments must remain resolvable | **CURED** | s6 `:335-337`. |
| 13 | "Small enough to enumerate" is not a rule | **CURED** (stricter rule taken) | s7 `:381-382`. |
| 14 | OWED list needs a disposal route | **CURED** | s11 `:508-512`. |
| 15 | s14 prospective only; three audit states | **CURED** | s12 `:534-537`. |
| 16 | Remove the self-minted citation | **CURED** | EXECUTED `python3 yaml.safe_load` → top-level keys are `['id','assent_source','title','status','created_at','enacted_by','purpose','drafting_note','sections']`. **No `citation:` key.** |
| Drop list | s4, s5, s3¶2, s7 (preference), drafting_note | **CURED / properly declined** | s4 and s5 gone; s3¶2 in commentary; s7 retention declined with a measurement I accept; drafting_note declared draft-only. |
| Rank opinion | nine-section minimum | **NOT MET, and further from met** | 24 entries, **71 duty tokens** (EXECUTED count over the parsed YAML). See "still too big" below. |

## CONDITION LEDGER — CODIFICATION (the three NAY grounds)

| Ground | Status |
|---|---|
| **1 — s11 narrows an entrenched floor uncited** | **CURED.** s9 `:444-448` cites ACT-ASSENTED-RECORD-PROTECTION:s1 and s2 by number and expressly reserves the stronger rule to the route those sections prescribe — which matches `2026-VJS-PC-017.yaml:199` D10 (READ) and `10-assented-record-protection.yaml:39-56` (READ). One residual wording point at C-1 below. |
| **2 — self-minted citation** | **CURED.** No `citation:` key (EXECUTED parse). `enacted_by:14-16` records the void self-mint, correctly analogised to `08-computer-first-realm.yaml:16-18`. |
| **3 — wrong vehicle** | **NAY MAINTAINED, and the ground has grown.** Schedule 3 *declares* amendments; it does not draft them, and it now makes a false closing statement of the amendment set. See C-2. |

My round-1 conditions C1–C17 and the definitions section are otherwise incorporated as claimed; I verified C3 (`ACT-002:s9` as the per-incuriam anchor — confirmed at `2026-VJS-PC-017.yaml:175` D2 and `:202`), C4 (OPERATIVE PARTS, **D1 alone** — confirmed at `:172`), C13 (four limbs — confirmed at `09-consolidation-framework.yaml:118-136`), and C17 (fifty days — confirmed, `.vjs/orders/2026-VJS-CC-OPBOX-002.yaml:34` `created_at: 2026-06-11T10:09:30Z`, `:5 status: binding`).

---

## NEW CONDITIONS ON PART 3 (RESTRAINT)

**R-1 — s19: Part 3 does not expire. The purpose says it does.**
`purpose:25` states "Part 3 expires by its own terms." s19 `:733-735` dissolves the Commission at the earliest of (a) thirty days after its **first warrant issues**, (b) the expiry of its **last warrant**, (c) the Principal's direction. If no warrant ever issues, none of the three occurs and Part 3 stands in force indefinitely. Add to s19 after "or the Principal's written direction":

> "; and in any event Part 3 expires ninety days after commencement if no warrant has issued by then, whereupon this Part is spent without further act."

**R-2 — s17: the pause is drawn wider than its purpose requires, and it has no classifier.**
Its stated purpose (`commentary::694-695`) is that "the sweep does not reproduce the pathology it repairs: seven sittings in a day, four of them corrective." I verified the pathology (EXECUTED: orders 158–164 all bear `created_at: 2026-08-04`; 159, 161, 162, 164 are corrective on their faces). But s17 closes a **class** of court business, not the matters the Commission is actually working, and it nowhere says **who decides** that a matter is "machinery and editorial". That is the identical vice I struck at first-draft s7 in condition 5: a court closed on a classifier's say-so. Replace the first sentence of s17 with:

> "While a jurisdiction's warrant runs, and only then, a matter is stayed from a new first-instance sitting if and only if the Chief Commissioner has entered it in the s18 register as within the current rectification schedule and has so certified in writing. The certification is a DECLARATION, not a finding; it binds no court; and any person, and the court on its own motion, may apply to the canonical Privy Council to lift the stay, which remains available throughout. A matter not so entered and certified is not paused."

**R-3 — s17: "editorial" is undefined and "machinery matters" is not the defined term.**
s1 defines MACHINERY and RECTIFICATION; it defines neither "editorial matter" nor "machinery matter" nor PAUSE. Add to s1:

> "EDITORIAL, of a matter, means a matter whose whole disposal is an act of RECTIFICATION as defined above and which decides no question of substance. A matter is not editorial merely because its remedy is a file change."

**R-4 — s16(a) and s16(f) take from the courts a power s6 of this same Act gives them.**
s6 `:289-290` provides that a filed order is opened and amended "by order of the court that made it or of a superior court", and `:330-332` that a consolidation is "made under this section **by the court** that made the latest instrument or a superior court". s16 `:643-645, :656-657` gives the same two powers to an executive organ on a Principal's warrant, with no court. That is a transfer of judicial power to the executive by implication, unreconciled inside one instrument, and Schedule 3 does not declare it. Add to s16, after the opening words:

> "Nothing in paragraphs (a) and (f) dispenses with s6's maker: the Commission may prepare and execute an amendment or consolidation, and may never author or make one. Each such act is made by the order of the competent court under s6, which the Commission obtains before the act, and the engineer's disability in s6 applies to a commissioner who is an agent seat."

**R-5 — s16(g) reaches outside the warranted jurisdiction.**
s16 opens "Within its warranted jurisdiction"; (g) `:657-658` authorises porting "across kernels, mirrors and worktrees so **canon** and every vendored copy agree". Canon is a different jurisdiction. Add to (g): "and no act under this paragraph touches a store of a jurisdiction for which the commissioner holds no warrant; a fix owed to canon is referred to the Standing Committee."

**R-6 — s16(d): retiring a store may exclude an assented record.**
(d) `:651-653` permits retiring stores that are copies. I have EXECUTED the case: `/home/jellytot/Projects/opbox-prod/opbox-kernel/.worktrees/tablelist-ext/lawpack/v2/statutes/` is exactly such a store, and it holds a **divergent unique copy** of `03-agent-duties.yaml`. Add to (d): "A store is not retired while it holds the only copy of any record, and never where the record declares a valid assent_source, until the unique content is relocated into a registered store and the relocation recorded."

**R-7 — Part 3 commences on assent but every one of its powers is keyed to a Part 2 section not yet in force.**
s20 `:757-758` commences Part 3 on assent. Schedule 2 puts s6 at tranche 3, Schedule 1's allocator at tranche 2, s2 at tranche 5 and s12 **last**. Yet s16 requires acts "on the s6 slip track" (a), "under s12's counting rules" (c), "to the Schedule 1 form" (e), "under s6's consolidation discipline" (f), and every act to carry "mode… under s2's discipline". A Commission with a thirty-day life will be dissolved before most of those commence. Add to s20:

> "For the purposes of Part 3 only, sections 2, 6, 7, 12 and Schedule 1 are in force from assent as to the standards they state, and Part 3 acts are measured by those standards whether or not the section has commenced as a Part 2 duty."

---

## NEW CONDITIONS ON PART 3 AND ELSEWHERE (CODIFICATION)

**C-1 — s15(a) mislabels the assent floor and omits the two provisions that hold it.**
s15(a) `:609-611` calls "ACT-CONSOLIDATION-FRAMEWORK:s7 and s25" the SOVEREIGN-ASSENT FLOOR. READ `09-consolidation-framework.yaml:36-51`: s7 is **Power to make statutory instruments**. READ `:53-68`: **s10 is "The assent floor"**. READ `:137-154`: s25 entrenches s10, s11 and s21 — not s7. The draft has copied PC 17 D10's *route* formula and used it as the *floor's* name. Further, framework **s10** and **s11** are cited nowhere in the Act (EXECUTED grep), while s21 `:791` names apex singleness only by the judgment `[2026] VJS-SC 4` and not by the entrenched section that holds it. Substitute in s15(a): "ACT-ASSENTED-RECORD-PROTECTION:s1 and s2 and ACT-CONSOLIDATION-FRAMEWORK:s10 and s25, cited by number", and add framework s10 and s11 to the s21 savings list.

**C-2 — Schedule 3's closing sentence is false, and that is the vehicle ground.**
Sch 3 `:888-889`: "It varies no other instrument." I can show four instruments varied and undeclared:
- **REG-SELF-CONVENE-001** (READ `lawpack/v2/regulations/REG-SELF-CONVENE-001.yaml:7-20`): "the kernel route returns court_required on a fork, with must_do `convene_the_named_court_on_own_motion`… the route gate and the functional hook (REG-HOOKS-001) fail closed so the duty fires whether or not it is stated in context." s17 suspends that convening duty.
- **ACT-003:s10** (READ `03-agent-duties.yaml:159-183`): an unsatisfiable enforcement gate is auto-justiciable "on that single fire", and the agent "must, on its own motion, route the gate to court". An unsatisfiable gate is the paradigm machinery matter; s17 defers it. s17 exempts breach matters but not this.
- **[2026] VJS-SC 4 D1** (READ `lawpack/v2/orders/2026-VJS-SC-004.yaml:54`): `route a repeatedly or conduct unsatisfiable gate to court on own motion`. Deferred by s17 without a word.
- **REG-COURT-RECORD-001** (READ `REG-COURT-RECORD-001.yaml:20-22, :27, :34`): `binds: prospective_rulings_only`; `must_not: invalidate_a_legacy_ruling_for_want_of_the_structured_fields`. s10 `:483-485` provides that a legacy directive with no recorded actor "binds NOBODY". EXECUTED `grep -L "actor:" *.yaml | wc -l` in `.vjs/orders/` → **42**. s10 therefore disapplies every directive in 42 filed orders for want of a structured field.

A schedule of express amendments that mis-states the amendment set is a worse vehicle than none, because a reader will treat the schedule as exhaustive. **Ground 3 is maintained.** The cure is (a) delete "It varies no other instrument", (b) add the four instruments above with the section and the extent, and (c) for s10, replace "binds NOBODY" with: "the reader reports it UNSTATED and never supplies a bearer; whether the directive binds, and whom, is reserved to the court, and nothing here invalidates a legacy ruling for want of a structured field (REG-COURT-RECORD-001)."

**C-3 — s16(c) operates in ACT-CONSOLIDATION-FRAMEWORK:s7's field without saying so.**
Wiring a duty to a gate is the making of machinery. READ `09-consolidation-framework.yaml:36-44`: the s7 power is vested in **the Standing Committee**. READ `2026-VJS-PC-013.yaml:35-51`: the Privy Council classified allocation, canonicity and install gates as "machinery under ACT-CONSOLIDATION-FRAMEWORK:s7" and "REFUSES to re-legislate any settled duty". s16(c) puts that power in an executive organ under a Principal's warrant. A primary Act may do it, but only expressly. Add to Sch 3: "ACT-CONSOLIDATION-FRAMEWORK:s7 (s16(c), for a warrant's duration only, and no Commission act is a statutory instrument)."

**C-4 — the Schedule 1 cure to the trust-boundary ground defeats itself.**
Sch 1 ¶2 `:826-829` registers "the bare '[YEAR] SERIES-REPO N' of the continuity citator" under **estate v1**, and s21 `:800-802` says this makes the trust-boundary rule "visible to the grounding gate for the first time". But READ `01-authority.yaml:68-84` — ACT-001:s4 `must_not: treat_v1_judgments_as_binding_without_incorporation`; READ `09-consolidation-framework.yaml:99-115` — framework s20: "V1 law not in Schedule 1 has live force only by an express incorporation record… A fail-closed incorporation-validity invariant rejects a missing element". `[2026] CC-OPBOX 16` is not a V1 archive judgment; it is a live 2026 subscriber ruling (`2026-cc-opbox-16.md:5 status: good-law`, `:7 date: 2026-06-07`). Classifying its series as estate v1 makes the rule *visible and non-binding* and risks tripping the s20 invariant. Cure: register the continuity citator as a **registered render form within estate v2** (a repo-scoped legacy form of the CC series), not as estate v1; or, if v1 is meant, supply the framework s20 incorporation record and ledger row for CC-OPBOX 16 and 17.

**C-5 — Part 3 is keyed to a register that does not exist.**
s14 `:592` ("the realm's jurisdiction register") and Sch 1 ¶1 `:820` ("Repo is a value on the jurisdiction register"). EXECUTED `grep -rn "jurisdiction register\|jurisdiction_register" lawpack/v2/ crates/` → **zero hits**. The instrument that exists is `lawpack/v2/regulations/REG-REPOS-REGISTER-001.yaml:1-4`, `[2026] VJS-REG 18`, "Repos Register and Subscription Regulation", made under framework s7 and s15-16. Substitute that citation in both places, or establish the register in s13.

**C-6 — Part 3 defines four organs while s14 says three, and claims reach s21 does not give it.**
`purpose:20-22` claims powers "over every jurisdiction running VJS". s21 `:778-780` binds subscribers only "on accession", and an amendment reaches a subscriber only when it bumps its pin. Add to s14: "No warrant issues for a jurisdiction that has not acceded to this Act by its own deliberate pin."

**C-7 — Part 3 terms undefined.** Defined: WARRANT `:108`, RECTIFICATION `:111`, TOMBSTONE `:97`, RESERVED ORDINAL `:100`. **Undefined:** PAUSE, editorial (R-3), warranted jurisdiction, Chief Commissioner, commissioner, "agent seat", "stranded governance artefact" (s16(i)), "fleet drift" (s16(g)), and "first-instance court" as a tier (framework s11 `:70-83` names County/Privy/Supreme, and the Privy Council sat at first instance in PC 13 and PC 17 — so s17's exclusion of "first-instance courts" and its preservation of the Privy Council are in tension on their face).

---

## NEW DEFECTS ANYWHERE

1. **Schedule 2's closing sentence is contradicted by s20.** Sch 2 `:876`: "At no point does a section of this Act stand in force as a duty with no gate." s20 `:757` commences **Part 1, Part 3 and Part 4 on assent**. EXECUTED count over the parsed YAML: Part 1 + Part 3 + Part 4 carry **25 duty tokens** (s1: 2; s14–s19: 18; s20–s21: 5) and there is no Commission machinery in either kernel (EXECUTED `grep -rli "commission" --include=*.rs crates/` → no hits; same in `opbox-kernel/governance/crates/`). Sch 2 tranche 0 also lists s20 and s21, which are Part 4, inside a schedule titled "commencement tranches for Part 2".

2. **Part 3 carries 18 duty tokens, zero `exceptions` limbs, and no gate.** The Act's own s12 `:529-532` says a claimed-but-uncontrolled duty is reported UNENFORCED. EXECUTED `vjs --json audit` from canon → `total 281 wired 43 unwired 238`. On assent this Act adds 71 duty tokens, of which 25 commence immediately unwired — worsening on day one the exact number s18 `:717` makes the Commission report as its measure of worth. Part 3's never-checkable limbs ("act only under a written warrant", "register every act before it is complete", "dissolve automatically") belong in `exceptions` with the courts named, as Operability C18 required for Part 2 and as the draft did for the other 7 limbs.

3. **The Act makes machinery claims and carries no `machinery_claims` block**, while s20 `:764` says "Sections 2, 8 and 12 bind this Act itself". Claims with no address and no mode include: s9 `commentary::458` ("55 of 109 filed orders unreadable"); s12 `commentary::541` (the 281/43/238 line and the negative-control line); s13 `commentary::568` ("over a hundred citation-bearing judgments"); s2 `commentary::177-179` ("whose scope error sat four lines above the finding"); s9 `:454` ("no separately-named self-test artefact existing").

4. **s21's flagship cured citation names no repository.** s2 `:141` requires an entry to state "repo (the repository root in which the address resolves)" and s20 `:765` requires every address in the Act to resolve in the repository named. s21 `:797-798` gives `.justice/judgments/county-court/2026-cc-opbox-16.md:44` with no repo. It resolves in `opbox-prod/opbox-kernel` (and, separately, in that repo's `.worktrees/tablelist-ext`). Slip-track correctable, but it is the Act's own showpiece.

5. **The Act's schedules are unreferenceable.** Their ids are `…:sch1/sch2/sch3` (EXECUTED parse) but every operative cross-reference is prose — "Schedule 1", "Schedule 3", "(Schedule 3)". PC 17 D5 (`2026-VJS-PC-017.yaml:184`, READ) normalises `:s.n`/`:s n`/`:sn` only. In an Act whose keystone is machine-resolvable citation identity, its own schedule references are invisible to grounding.

6. **s9's "NOT APPLIED" is not the language of the floor.** ACT-010:s1 (`10-assented-record-protection.yaml:22-30`, READ) forbids "voided, **excluded**, or blocked". Add to s9: "and 'not applied' is not exclusion within the meaning of ACT-ASSENTED-RECORD-PROTECTION:s1: the kernel does not apply what it cannot read, and that is a fact about the reader, not a disposition of the record."

7. **s2 mis-describes the section it cites.** `:147`: "the kernel is model-free by construction and by dependency ban (ACT-003:s8; **the capability is removed, not prohibited**)". READ `03-agent-duties.yaml:123-137`: ACT-003:s8 is a **prohibition** (`must_not: add_model_call_to_vjs_core`) **with an exception** (`adapter_crate`). The instrument that removes the capability by closed registry is ACT-004:s3 (`04-records-logs-citations.yaml:48-64`), which the second draft no longer cites at all.

8. **s16's disability list omits the assent floor.** s16 `:667-669` forbids trust-boundary change, substance change and reader narrowing. It does not forbid a Commission act touching an assented record. s15(a) does the work, but the disability belongs in s16's own `must_not` given that s16 is the operative grant.

9. **The framework s21 due-process question is unaddressed.** READ `09-consolidation-framework.yaml:118-136`: the protective floor is **non-derogable** and its second limb is "rights, standing, and due process"; `:137-154` entrenches it, amendable only by a Sovereign-assented Act citing the provision by number. s17 defers first-instance access for up to twenty-eight days per jurisdiction. s21 `:790` declares the floor **undisturbed** rather than amended. Either s17 is narrowed as R-2 requires so that no derogation arises, or the Act must take the s25 route. It cannot do neither.

---

## RECITAL CHECK

| Claim (address in the draft) | Verdict | Basis |
|---|---|---|
| Trust-boundary rule held at `[2026] CC-OPBOX 16`, `.justice` series, no VJS prefix, at `…/2026-cc-opbox-16.md:44` (s21 `:796-798`) | **TRUE** | File exists (EXECUTED `test -f`); line 44 is remedy item 3 stating the rule in those terms (READ). |
| Its remedy "quashed in part" and duty "carried forward by [2026] CC-OPBOX 17" (s21 `:798-799`) | **TRUE** | READ `2026-cc-opbox-17.md:54-62`: items 3 and 4 QUASHED, finding AFFIRMED, rule written into the governing instructions. |
| CC-OPBOX 18 not relied on | **TRUE and correct** | READ `2026-cc-opbox-18.md:5` `status: pending`. |
| "281 duties, 43 wired, 238 unwired" (s12 `:541`) | **TRUE** | EXECUTED `vjs --json audit` from canon → `total 281 wired 43 unwired 238`. Corroborated at `opbox-kernel/docs/conformance-map.md:5-7` (READ). |
| "no negative-control register **or conformance ratchet** anywhere in the test tree" (s12 `:542`) | **FALSE in part** | No *register* file exists (EXECUTED `find`), so that limb holds. But EXECUTED `grep -rli "negative control" crates/` → **8 files**, three of them naming "the gate's own negative control" (`crates/vjs-testkit/tests/enforcement_surface_admission.rs:177`, `crates/vjs-core/src/enforcement.rs:543`, `crates/vjs-testkit/tests/lawpack_literal_marker.rs:273`); and a ratchet exists at `crates/vjs-testkit/tests/global_invariants_gate.rs:65` (`fn global_invariants_are_bound_and_debt_ratchets_down`, `VJS_DEBT_BASELINE`). A bounded search reported as a fact about the corpus — the thing s1 `:55` forbids. |
| "next-citation DEC lawfully offers ordinal 15" (s7 `:388`) | **TRUE** | EXECUTED `vjs next-citation DEC 2026` → `Next citation: [2026] VJS-DEC 15`. |
| "about forty files cite VJS-DEC 15" (s7 `:388-389`) | **UNSUPPORTED AS STATED** | EXECUTED `grep -rl "VJS-DEC 15"`: canon **14** files; opbox (worktrees excluded) **101** files; `.justice` subtree **41**. The source (`SUBMISSION-2026-08-04-221305:18`, READ) says "about forty files cite **the series**" — DEC 15–22, not DEC 15. The draft narrows the subject and drops the store. |
| "**the condemned Unitary Stack decision whose record PC 13 removed**" (s7 `:389-390`) | **FALSE, and it prejudges a reserved matter** | READ `lawpack/v2/orders/2026-VJS-PC-013.yaml`: the series is mentioned once, at `:26`, as the vector of a self-assertion incident; `:170` calls them "fake citations"; **no directive D1–D12 removes anything** and the forbidden list `:153-162` contains no removal. READ `SUBMISSION-2026-08-04-221305:6-7`: the labels were deleted from orders 068/080/081 "**On the Principal's direction**"; `:14-16`: "**No primary record for any of DEC 15-22 exists**" — there was no record to remove. And "condemned" decides the very question s15 `:625` says is stayed and s7 `:394` says "this Act may not prejudge". The ratios are live and in daily use (`opbox-kernel/docs/unitary-stack.md:4, :48`, READ). |
| "the allocator's arithmetic is correct over the roots it reads" (s7 `:387-388`) | **TRUE, but the mode is not stated** | EXECUTED `vjs next-citation ACT 2026` → `[2026] VJS-ACT 11`; `vjs next-citation DEC 2026` → `[2026] VJS-DEC 15`. The dispositions ledger `:61-62` claims "the second draft's s7 commentary states the allocator finding as EXECUTED". EXECUTED `grep -n "EXECUTED" ` over s7 → the word does not appear in s7. **The ledger's claim about its own draft is false.** |
| "the kernel's roots being a three-entry constant" and `.justice` read by no allocator, citator or gate (s13 `:569-570`) | **TRUE** | READ `crates/vjs-core/src/front_door.rs:84-97`: `lawpack/v2`, `.vjs/orders`, `.vjs/court`. |
| "over a hundred citation-bearing judgments" in `.justice` (s13 `:568`) | **TRUE on one reading, unaddressed** | EXECUTED: 181 files under `.justice/judgments/` contain a `[YYYY]` citation; only **67** carry their own `citation_id`. No query form or store count given — the discipline s8 `:415-417` imposes on others. |
| "Orders 163 and 164 both carry the issue `dec15_dec19_recitation_and_order_completion`" (s3 `:203-205`) | **TRUE** | EXECUTED `grep -n "^issue"` → both at line 6. |
| "Forty-two filed opbox orders omit actor" (s10 `:489`) | **TRUE** | EXECUTED `grep -L "actor:" *.yaml \| wc -l` in `.vjs/orders/` → **42** (of 116). |
| "[2026] VJS-CC-OPBOX 2 … a binding order of that name sat in `.vjs/orders`, created 2026-06-11: fifty days earlier" (s8 `:423-426`) | **TRUE** | READ `.vjs/orders/2026-VJS-CC-OPBOX-002.yaml:5, :34`. |
| "55 of 109 filed orders unreadable" (s9 `:458`) | **TRUE as history, STALE as stated** | Sourced to `2026-VJS-CC-OPBOX-160.yaml:9`. But READ `2026-VJS-CC-OPBOX-161.yaml:13-14`: "Executing 160 took the unreadable count from 55 to **2** without touching a record". The draft states 55 in the present tense with no date, no mode and no attribution — the same stale-recital defect the ledger `:171-173` criticises in another counsel. |
| The improvised amendment, the residue worklist, and the patch "at a home-directory backup path outside every governed store" (s6 `:339-343`) | **TRUE** | READ `.vjs/unreadable-orders.txt:35` names `~/Backups/opbox-kernel-order-widening-2026-08-04.patch`; EXECUTED `ls` → the file exists at `/home/jellytot/Backups/…`, 71838 bytes, outside all three governed roots. EXECUTED `grep -c "removed :"` → **6**, all comment-formatted; 7 non-comment lines. |
| "seven sittings in a day, four of them corrective" (s17 `:694-695`) | **TRUE** | EXECUTED: orders 158–164 all `created_at: 2026-08-04`; 159, 161, 162, 164 are corrective on their faces. |
| "adoption is CONSTITUTIVE ([2026] REALM-SC 8)" (`enacted_by:12`) | **TRUE** | READ `lawpack/v2/provenance/founding/COMMENCEMENT-V2-0001.yaml:14`: `adopted_by: "Standing Committee, 4-0 (constitutive, [2026] REALM-SC 8)"`. |
| Void self-mint `[2026] VJS-ACT 8` at ACT-COMPUTER-FIRST-REALM's purpose (Sch 1 ¶4 `:840-841`) | **TRUE** | READ `08-computer-first-realm.yaml:16-18`. |
| The dispositions ledger's "7 such limbs in the second draft" (Operability C18 row) | **TRUE** | EXECUTED parse: `exceptions` limbs total **7** (s2×2, s5, s6, s8, s9, s11). |
| The dispositions ledger's correction of Operability C8 (6 comment-formatted `removed :` entries) | **TRUE** | EXECUTED `grep -c "removed :"` → 6; EXECUTED `grep -vc "^#"` → 7 non-comment lines. |

Net: **one false recital (the DEC-15/PC-13 removal claim, which also prejudges a stayed matter), one false-in-part recital (the negative-control/ratchet line), one false claim by the ledger about its own draft (the EXECUTED mode at s7), one unsupported count (the "about forty"), and one stale count (55 of 109).** Five of the first draft's false recitals were withdrawn and I confirmed each withdrawal; the second draft has introduced new ones of the same family.

---

## IS THE ACT STILL TOO BIG? (Restraint)

Yes, and further from my round-1 minimum. 24 entries, **71 duty tokens** (EXECUTED count), against a realm that currently wires 43 of 281.

Still primary on my portfolio: **s1** (trimmed), **s2** (the evidence rule), **s3**, **s6** (the amendment power — still the one section that could be nothing else), **s7 + Sch 1**, **s9**, **s10**, **s13**, **s21**. Nine, as before.

Still to strike or demote:
- **s4** → one sentence of footing; the rest is PC 17 D2 restated, and framework s4 (`09-consolidation-framework.yaml:19-34`, READ) means restatement carries no force.
- **s8** → subordinate; a record-form duty properly amending ACT-004:s7.
- **s11** → subordinate; a schema field and a local-ci line.
- **s12** → subordinate; a testing standard, properly REG-DEV-CONDUCT-001.
- **s5** → I maintain my preference for a practice direction, but I yielded in round 1 to Operability's measurement and I yield again. Recorded, not pressed.
- **Part 3**: only **s15** (the floors) and **s19** (sunset and reviewability) need primary rank. **s16** is a warrant instrument — nine paragraphs that, on inspection, restate powers Part 2 already confers on the maker of a record with the actor changed; it could be one sentence plus the disabilities. **s18** is a register schedule. **s17** should not exist in this form at all: cure it as R-2 or drop it and let the Commission apply for a stay in the ordinary way.

## ANTI-HENRY-VIII FINDING ON PART 3

Three provisions reach at things reserved elsewhere without the express citation the reservation requires:
- **s16(a)/(f)** exercise, through an executive organ, powers s6 of this same Act vests in courts (R-4).
- **s16(c)** operates in framework s7's field, which vests the machinery power in the Standing Committee (C-3).
- **s17** defers access to first-instance courts, touching the entrenched due-process limb of framework s21, while s21 of this Act declares that floor undisturbed (new defect 9, R-2).

None is incurable by drafting, which is why Restraint is AYE-with-conditions and not NAY. But all three must land, and Codification's vehicle ground stands until Schedule 3 states the amendment set truthfully.