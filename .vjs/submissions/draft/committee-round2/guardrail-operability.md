## METHOD DECLARATION (this Act's own s2, applied to this bench)

Every machinery assertion below carries an address and a mode. **READ** = text inspected. **EXECUTED** = command run, output recorded. Canon = `/home/jellytot/Projects/vibe-justice-system`. Subscriber = `/home/jellytot/Projects/opbox-prod/opbox-kernel`. I ran no build and no test suite.

---

# GUARDRAIL VERDICT: **NAY**

The two round-1 NAY grounds are cured. Part 3 reconstitutes one of them in a worse form and adds a floor inversion in the floor section itself.

# OPERABILITY VERDICT: **AYE WITH CONDITIONS** (O-1 and O-2 constitutive)

All five round-1 constitutive conditions are in the text. Part 3 does **not** self-condemn — but only because of C14's bounding and a timing accident, not because s20 works. Schedule 2 makes a false machinery claim on the Act's own face.

---

# 1. CONDITION LEDGER — ROUND 1

## Guardrail

| # | Status | Citation / evidence |
|---|---|---|
| **FB-1** floor savings cite wrong orders | **CURED** | s21 ¶2 now cites `[2026] CC-OPBOX 16` at `.justice/judgments/county-court/2026-cc-opbox-16.md:44`, mode READ. Verified: `:2` carries `citation_id: "[2026] CC-OPBOX 16"`; `:44` is remedy item 3 and **is** the trust-boundary rule (READ). s21's added claim — "whose remedy was quashed in part and whose duty was carried forward by [2026] CC-OPBOX 17" — verified at `2026-cc-opbox-17.md:59-62` (READ): "remedy item 3 … **QUASHED and replaced**", "remedy item 4 … **QUASHED**". The mis-citation is recorded on the face, not silently fixed. Correct, and correctly qualified. |
| **FB-2** five refusals, no assent bifurcation | **PARTIAL** — see G-5 |
| **FB-3** reader narrowing as disapplication lever | **CURED** | s9 (UNREADABLE-IN-FORCE; regression rule; deviations by court order with expiry) + s16 `must_not: narrowing_any_reader` |
| **FB-4** non-routability = exclusion | **CURED** | s6 consolidation ¶: "remains visible, citable and routable" |
| **FB-5** consolidation reaching apex tier | **CURED** | s6 ¶6 + s15(c) |
| **FB-6** s5 names no maker | **MOOT** — first-draft s5 dropped |
| **FB-7** disclosure vs private-data floor | **PARTIAL** — cured at s2/s8, **reopened at s18**; see G-7 |
| **C1–C3, C6–C15, C17, C18** | **CURED** | C1→s2; C2→s1+s3; C3→s4; C6→s6¶6; C7→s5 (both prohibitions deleted, declaration-not-finding, own-motion jurisdiction restored); C8a/C8b→s6; C9→s7; C10→s8; C11→s9; C12→s10; C13→s11; C14→s12; C15→s21; C17→s13; C18→7 `exceptions` tokens (EXECUTED enumeration below) |
| **C4** store register / allocation voidness | **CURED AS MODIFIED** — softening to refuse-at-authoring is correct and I accept the dispositions ledger's reasoning (my own FB-2 compelled it) |
| **C16** | **CURED** with the corrections recorded rather than hidden |
| **s7-as-ouster-of-jurisdiction** | **CURED at s5 — RECONSTITUTED AT s17.** See G-2/G-3. |

**Unaddressed failure mode 2 remains live and now bites Part 3.** `.justice` is still not a governed-record root: `crates/vjs-core/src/front_door.rs:84-97` (READ) is still exactly three roots. EXECUTED `python3` against `crates/vjs-lawpack/src/refs.rs:47`'s regex: `'[2026] CC-OPBOX 16'` → **False**, `'[2026] REALM-SC 8'` → **False**, `'[2026] VJS-SC 4'` → True. The trust-boundary rule on which every Part 3 floor depends is **invisible to every citation gate in the kernel**, and Schedule 1's cure is Part 2 tranche work while Part 3 commences on assent.

## Operability — the five constitutive conditions

| # | Status | Citation |
|---|---|---|
| **C1** structured `machinery_claims` | **CURED** — s2 ¶1, with the mode enumeration and the "kernel does not, and may not, decide by reading prose" limb verbatim in substance |
| **C4** bound to OPERATIVE PARTS | **CURED** — s1 definition; s2 opens "Where the OPERATIVE PARTS of a filing or order…" |
| **C8** `unreadable-orders.txt` correction | **CURED, and my own recital corrected against me.** The dispositions ledger's C8 row records that my round-1 claim of "ZERO non-comment entries" was stale. I accept the correction; s6 commentary states the true facts (improvised amendment + patch outside every store) and s16(i) reaches the stranded artefact. |
| **C14** negative control bounded to claims | **CURED** — s12 ¶1: "attaches to a gate-registry row and not to duties the audit already reports UNWIRED" |
| **C17** commencement-by-section | **TEXTUALLY CURED, MECHANICALLY INOPERATIVE** — s20 + Schedule 2. See O-1. |

C2, C3, C5, C6, C7, C9–C13, C15, C16, C18 all **CURED**. EXECUTED enumeration of `exceptions` tokens in the second draft: 7 — s2×2, s5, s6, s8, s9, s11. C18's list is fully accounted (its s5 entry moot on the drop).

---

# 2. PART 3 — FLOOR & EVASION TABLE

### The governing measurement first

There are **two live drafts of this Commission**: Part 3 (s14–s19) and the standalone `.vjs/submissions/draft/ACT-RECTIFICATION-COMMISSION.yaml` (10 sections, READ in full). Part 3 is a **compression of the standalone, and the compression dropped the safeguards.** That is not an inference — it is a line-by-line comparison of two files both on this bench's desk.

| Power / mechanism | Can it cross a floor, mechanically | Cure |
|---|---|---|
| **s16(a) rectify FORM on the s6 slip track** | **YES — the most exploitable.** The fixed test (s6 ¶2) preserves only "every OPERATIVE PART … and every directive's actor". s1 defines OPERATIVE PARTS as "holding, each directive's must text, and each forbidden clause, **and nothing else**". `Order` (READ `crates/vjs-core/src/types.rs:369-447`) also carries: `status: AuthorityStatus`, `issue`, `exceptions`, `supersedes`, `assent_source`, `citation`, `cites_authorities`, and `extra` (flatten). `Directive` (READ `:449-455`) is `{id, actor, must, **when**}`. **None of these is in the test.** So a commissioner may, with a *passing* content-preservation proof: (i) flip `status: Binding` → `Spent`/`Superseded`/`Void` (READ `types.rs:40-51`, all are valid variants) — under s1's LIVE definition the instrument ceases to be live, which is a **void by editorial act on an assented record**, crossing s15(a); (ii) rewrite `directives[].when`, the condition on which the `must` fires, gutting the directive while its `must` text stays byte-identical; (iii) add an entry to the order's `exceptions:` list, carving out of its own directive; (iv) alter `assent_source`, stripping the record of the very hook the floor keys on. | **G-1** |
| **s16(e) MIGRATE CITATIONS** | **YES.** `crates/vjs-engine/src/staged.rs:322-338` (READ) computes the grounded operative surface as holding + `directives[].must` + `forbidden[]` **+ `cites_authorities[]`** (PC-17 D7). The Act's OPERATIVE PARTS is *narrower* than the surface the kernel actually grounds over: it omits `cites_authorities`. So rewriting `cites_authorities` — swapping the authority a directive rests on, my own round-1 s8-slip evasion — passes the s6 fixed test and is expressly authorised by s16(e). The standalone Act's s4(c) required "**on proof of both**" for exactly this act; Part 3 dropped the proof. | **G-1**, **G-4** |
| **s16(b) WIDEN READERS** | **NO** on the literal wording. `must_not: narrowing_any_reader` (s16) + s9's regression rule close it. This one holds. |
| **s16(h) COMPLETE a record "by transcription of terms the record already carries"** | **YES — and this power is NEW in the compression; it is in no section of the standalone Act.** s1 states that "a reference in the issue, vote, question, runtime_summary, source opinion or case-file digest is **recital and is not operative**." A record therefore "carries" terms that are, by this Act's own definition, non-operative. s16(h) authorises transcribing them — i.e. **promoting recital into operative text**. That is authorship of an operative term by an executive organ, which the same clause purports to forbid in its second half. The clause defeats itself. | **G-4** |
| **s16(c) WIRE DUTIES / reclassify** | Partial. `reclassify honestly — recording that a duty no gate can hold is enforced by the courts` is a power to move a `must_not` to `exceptions`, which (READ `crates/vjs-lawpack/src/conformance.rs:35-51`) **removes it from the audit entirely**. A commissioner may therefore lower the measured duty population by reclassification and report the fall as work. s12's counting rules do not bound this. | **G-6** |
| **s16(d)/(f)/(g)/(i)** | Bounded adequately by s6, s15(c) and s13. No evasion found. |
| **s16 as a whole — is the list exhaustive?** | **NO, on the literal wording.** s16 says "the Commission may:" and lists (a)–(i). The standalone s4 said "**MAY … do the following and nothing else. The list is EXHAUSTIVE: a power not on it is not conferred**", with `must: confine_a_commission_to_the_exhaustive_power_list`. Part 3 dropped the word, the sentence and the token. s15's closing "its powers are these" is a saving in a different section and is not a rule of construction on s16. Also dropped from the standalone's `must_not` list: `a_commission_deciding_a_reserved_question`, `a_commission_enacting_amending_or_reading_down_an_instrument`, `a_commission_overruling_anything`. | **G-4** |
| **s15 — the three floors, LITERAL wording** | The floors themselves hold in the `text:`. But **all three are `defines` tokens** (`commission.floor.assent`, `commission.floor.protective_four_limbs`, `commission.floor.apex_singleness`), and this Act's own s12 says: "A defines token is declarative vocabulary … **is not asserted to bind anything**." The Act expressly declares its own three Commission floors to be non-binding vocabulary. Same for s14's `commission.no_law_no_merits_no_precedent` — the single most important prohibition in Part 3, filed in the one field the audit never enumerates. | **G-8** |
| **s15 vs s17 — TRUST BOUNDARY: a direct internal contradiction, and the floor loses** | **THE CENTRAL NAY GROUND.** s15: "a fork involving a TRUST BOUNDARY is registered and **STAYED**; the Commission may not dispose of it and **it awaits the courts**." s17: "Unaffected and **never paused**: floor matters under s15". Both operative, in the same Part, on the same facts. The standalone Act's s6(c) resolved this the other way and expressly: a trust-boundary fork is a carve-out the stay **does not reach**, with `must_not: staying_a_trust_boundary_fork`. Part 3 **inverted it**. And the underlying rule (READ `2026-cc-opbox-16.md:44`, affirmed `2026-cc-opbox-17.md:59`) is that the fork "goes to the court **before implementation**". "Registered and STAYED … awaits the courts" for up to 30 days is the opposite of "before implementation". Part 3 crosses a floor **in the section that states the floors**. | **G-2** |
| **s17 — the pause: who decides "machinery and editorial"?** | **NOBODY IS NAMED, and the matter is routed to the classifier.** s17 confers no classification on any organ. The kernel cannot supply one: `RouteInput` (READ `crates/vjs-core/src/types.rs:147-160`) has no machinery/editorial field, and every field it does have is caller-supplied. "MACHINERY" is defined in s1 breathtakingly broadly — "code, a gate, a check, an invariant, a hook, a binary, a command, a schema, a reader, or any other runtime artefact". "Editorial" is defined **nowhere**; s1's RECTIFICATION definition uses "editorial" in its own definiens. So the pause's scope term is half-undefined and half-unbounded, and s17's operative consequence is that a matter so classified "**goes to the Commission's register instead**" — the commissioner receives the matter and decides whether it is his. **This is the first draft's s7 exactly: the actor supplies the classification that determines jurisdiction, and no organ may look behind it.** It is worse than first-draft s7 in one respect: s19 says "**The pause defers review**", so the court cannot test the classification until the classifier has finished. | **G-3** |
| **s17 vs ACT-003:s10 — an unvaried entrenched trigger, silently paused** | **A SECOND HARD BREACH.** `lawpack/v2/statutes/03-agent-duties.yaml:160-173` (READ) makes an unsatisfiable enforcement gate **auto-justiciable "on that single fire"**, and closes: "**the pause is recorded, fail-loud, and court-tested: unsatisfiability is never self-declared**". An enforcement-gate matter is a MACHINERY matter on s1's own definition, so s17 pauses it. s17's carve-out list — floors, breach, Principal-certified urgency, appeals in flight, PC/SC — **omits it**. The standalone Act's s6(b) carved it out expressly (`must_not: staying_an_unsatisfiable_enforcement_gate`, citing ACT-003:s10 and VJS-SC 4). Schedule 3 declares s17 varies **ACT-002:s6 only** and "varies no other instrument". So Part 3 pauses an entrenched auto-justiciability trigger **without express variation**, and does so with a self-declared pause — which is the one thing the corpus's own law on pauses forbids by name. | **G-2** |
| **s17 — scope: is it really "per-jurisdiction and rolling"?** | **NO. That is commentary asserting what the operative text does not secure.** s14 constitutes "a Chief Commissioner **and one commissioner per warranted jurisdiction**". Nothing in s14, s17 or s19 caps concurrent warrants. The standalone s2 capped it and said why: "Each warrant names exactly **ONE** jurisdiction … a single body with concurrent power over all of them **could propagate an error to the whole estate before any of it was reviewed**", with `must_not: a_commission_acting_outside_the_jurisdiction_named_in_its_warrant`. The standalone's own schedule enumerates **7 jurisdictions carrying a `.vjs` surface**. Seven simultaneous warrants = every first-instance machinery court in the realm dark for 30 days. Also dropped: the standalone's "not fewer than three seats … carries the four portfolios of the Standing Committee in substance". Part 3 permits **one commissioner, who may be an agent seat**, per jurisdiction. | **G-3** |
| **s17 — "triggers are DEFERRED, not extinguished: the route continues to detect them"** | **FALSE AS A MACHINERY CLAIM.** EXECUTED `grep -rn "CourtTrigger::" --include=*.rs crates/`: the only construction sites are `crates/vjs-core/src/court.rs:10`, `:24` and `crates/vjs-testkit/src/lib.rs:104`, **all three `FirstImpression`**. `Breach`, `Conflict`, `Distinction` and `Overruling` are declared variants the route never constructs (READ `court.rs:9-31`). The route detects **one of five**. s17's carve-out for "breach matters" therefore rests on no detection whatever: it is a human classification, made by the person the pause benefits. | **G-3** |
| **s17 — tolling** | **DROPPED IN COMPRESSION, and it collides with s11.** The standalone s6 tolled "every reservation and every review date for its duration" and convened deferred matters "on its expiry **without fresh application**". Part 3 s17 has neither. Meanwhile s11 caps review dates at 90 days and requires the named actor to close an OWED reservation "**within one sitting of the report**" — while s17 has paused first-instance sittings on machinery matters. Two sections of one Act command incompatible acts. And "deferral, not extinguishment" has no restoration mechanism: s17 says only that the pause "lifts … whatever remains undone." | **G-9** |
| **s19 — is dissolution automatic and unfakeable?** | **The powers side holds; the accountability side does not.** "Dissolution is automatic and requires no act" + "any purported exercise is void" is sound, and s17's automatic lift is sound. But the standalone s7 named the property and armed it: powers lapse **fail-closed**, stay lifts **fail-open**, with `must_not: extending_a_stay_by_inaction_or_an_unfinished_sweep` and `must_not: a_warrant_exceeding_ninety_days` and **`a_second_extension_without_a_delivered_report`**. Part 3 kept the conclusion and dropped all three tokens. Consequence: warrant (14d) + renewal (14d) = 28 days with **no report at any point**, because s18 puts the report "**within seven days of dissolution**" — the standalone required it **before expiry**. Nothing outside the Commission observes the Commission for its entire life. | **G-10** |
| **s19 — is every act truly reviewable?** | **NO, not on the literal wording.** The standalone s9 said every act is "**APPEALABLE AS OF RIGHT** … and an appeal **is not stayed** by section 6", with `must_not: staying_an_appeal_from_a_commission_act`, `must: keep_the_prior_text_routable_until_the_appeal_period_closes`, and five enumerated set-aside grounds. Part 3 s19 says acts are "**REVIEWABLE** by the ordinary courts **after resumption**" and adds "**The pause defers review**". That is a 30-day immunity window plus a new immunity — "good-faith reliance on a Commission rectification before it is set aside is not a breach" — with no prior-text-routability duty, no as-of-right, and no set-aside grounds. Review deferred by the reviewee's own timetable is not review. | **G-3**, **G-11** |
| **Can Part 3 touch the pending DEC footing matter?** | **Not directly — s15 stays it and Sch 1 ¶4 reserves DEC 15–22.** Verified the matter exists: `opbox-kernel/.vjs/submissions/filed/SUBMISSION-2026-08-04-221305.yaml:1` (EXECUTED grep). **But Schedule 1 ¶4 gives the Commission a power over a constitutional Act's face**: "the Commission determining under its warrant how the live file's citation line and the register are reconciled" for `ACT-COMPUTER-FIRST-REALM`. That is a Sovereign-assented constitutional Act. `ACT-CONSOLIDATION-FRAMEWORK:s10` (READ `lawpack/v2/statutes/09-consolidation-framework.yaml:53-58`) makes Sovereign Assent "non-automatable, non-presumable, **non-delegable**", and s14 says the Commission's authority is "**delegated** and revocable at will by the Principal". Schedule 1 has no Part assignment and therefore no commencement rule (below), so this power's start date is unstated. | **G-12** |

---

# 3. PART 3 — TOKEN AUDIT

**Baseline, EXECUTED 2026-08-05:** `./target/release/vjs audit --json` → `total 281 wired 43 unwired 238`. Matches `docs/conformance-map.md:5-7` (READ). My round-1 baseline still holds.

**Second draft, EXECUTED cross-check** (yaml parse of the draft × token strings extracted from `GATE_REGISTRY`, `crates/vjs-lawpack/src/report.rs:36-241`):

```
kernel_effect keys used: ['defines', 'exceptions', 'must', 'must_not']   (all recognised — lib.rs:287-298)
audited duty tokens (must/must_not/prohibits): 71   hits 0   misses 71
defines tokens: 64      exceptions tokens: 7
PART 3 (s14-s19): 18 duty tokens, 0 wired.  13 defines tokens.
```

**Does any Part 3 token reach real machinery? No — and not one of them *can*, in the Commission's lifetime.**

- EXECUTED `grep -rn "warrant\|[Cc]ommission\|pause" --include=*.rs crates/`: every hit is `release warrant` (`REG-RELEASE-WARRANT-001`, public-push, `route.rs:190`, `hook.rs:179`) — a different concept. **There is no warrant artefact, no commission artefact, no pause artefact in the kernel.**
- `act_only_under_a_written_warrant_of_the_principal` has no object to bind to. `classify_every_touched_item_into_exactly_one_bucket` needs a register that does not exist. `lift_the_pause_automatically_at_warrant_expiry` needs a pause the route cannot represent.
- `changing_the_substance_of_any_operative_part` and `creating_or_modifying_any_trust_boundary_surface` are judgements no deterministic gate can hold — they are C18-class limbs sitting in `must_not`, where they will inflate the unwired count permanently. **Part 3 carries zero `exceptions` tokens.**
- The Commission dissolves at 30 days (s19). No gate for any of these 18 will be written, reviewed and shipped inside 30 days. **Part 3's tokens are a page of unwired labels by construction, not by neglect.**

**Projected audit on assent:** `total 352  wired 43  unwired 309`. Wired share 15.30% → **12.22%**. This Act becomes the largest single source of unwired duties in the corpus (71 of 309, 23.0%), of which Part 3 is 18.

**Does Part 3 self-condemn under s12? NO — for two reasons, one principled and one accidental.**

1. **Principled (and correct):** s12 as cured by my C14 owes a control **only where enforcement is claimed**, i.e. only on a `GATE_REGISTRY` row. Part 3's tokens have no row, are honestly reported UNWIRED, and owe no control. C14's bounding is what saves Part 3. This is the mechanism, and it works.
2. **Accidental:** s12 commences **last** of Part 2 (s20, Sch 2 tranche 6). Part 3 commences on assent and dissolves within 30 days. **s12 will not be in force at any point during the Commission's existence.** Part 3 escapes the Act's own measure by outliving neither it nor being outlived by it — by timing, not by design. The Committee should know that the reason Part 3 passes s12 is that s12 never sees it.

**Is Part 3 legitimately outside the conformance audit as an executive organ rather than a machinery duty? NO — the audit has no such concept.** READ `crates/vjs-lawpack/src/conformance.rs:56-77`: `conformance_audit` iterates **every** `lawpack.statutes[].sections[]` with a `kernel_effect`, and every `lawpack.regulations[]`. There is **no filter on status, no filter on Part, no filter on organ, and no filter on commencement**. EXECUTED `grep -rn "commence\|commencement" --include=*.rs crates/` → 7 lines across 2 files, **all of them comments** in `assent.rs` and a testkit doc-comment about the founding lock. **The kernel has no representation of commencement at all.** `status: draft` on the statute is not consulted either.

**Therefore s20 and Schedule 2 do not do what they say.** The moment this Act's YAML lands in `lawpack/v2/statutes/`, the audit reads the **whole-Act** number — 352/43/309 — regardless of which Part has "commenced". Commencement-by-section is a legal fiction with no consumer. That does not condemn the Act (C14 already saved it), but it does mean **Schedule 2's closing sentence is false**:

> "At no point does a section of this Act stand in force as a duty with no gate."

Part 3 commences **whole, on assent** (s20), carrying **18 duty tokens with 0 gates**. The sentence is not scoped to Part 2; it says "a section of this Act". **This Act makes a false machinery claim about itself, on its own face, in the schedule that exists to prevent exactly that, in breach of its own s2 and its own s20 ("Sections 2, 8 and 12 bind this Act itself").** That is the first draft's error class, committed a fourth time.

**Is the structured citation form (s7 + Schedule 1) implementable? Yes. Nothing blocks it.** What the kernel needs: (i) a `CitationTuple {estate, year, series, repo, ordinal}` plus a render-map keyed on (estate, series) — the render/parse pair is pure and testable; (ii) `refs.rs:47`'s single hard-coded regex (READ) replaced by iteration over registered forms — this is the change that makes `[2026] CC-OPBOX 16` and `[2026] REALM-SC 8` visible for the first time; (iii) `GOVERNED_RECORD_ROOTS` (READ `front_door.rs:84-97`, still 3 entries) replaced by the s13 register — the allocator then mints over `.justice` (EXECUTED `find … -name '*.md' | wc -l` → **213** files in the opbox `.justice`; canon has **no** `.justice`, EXECUTED `ls -d .justice` → "No such file or directory"); (iv) tombstone/reservation/forwarding tables — three new stores, all pure data; (v) `Grounding` gains `Tombstoned` and `Forwarded` variants alongside the existing `Unresolved`/`NotInForce` (READ `staged.rs:352-380`). Sequencing per my round-1 C13 is respected at Sch 2 tranche 3. **No blocker.** One caution: s13's "On commencement the opbox register includes .justice" is canonical primary law naming a store that exists only in one subscriber and does not exist in canon — see **O-4**.

**One implementability note on s2 (C1).** `Order` can carry `machinery_claims:` **today** via `#[serde(flatten)] pub extra` (READ `crates/vjs-core/src/types.rs:445-446` region). `Submission` **cannot**: READ `crates/vjs-store/src/lib.rs:449-457` — eight flat fields, `facts: String`, and **no `extra` catch-all**. EXECUTED `grep -rn "deny_unknown_fields" crates/vjs-lawpack/src/` → **0**. So a `machinery_claims:` block on a *filing* is dropped silently on load. Sch 2 tranche 5 accounts for this. Correct as drafted; recorded so no one reads s2 as operative on filings before that tranche.

---

# 4. NEW CONDITIONS

## Guardrail (G-1 … G-12) — G-1, G-2 and G-3 are constitutive

**G-1 — s16(a) and s6 ¶2. DEFECT: the fixed content-preservation test leaves the fields that determine force, condition and identity outside it, so an editorial act may de-force an assented record with a passing proof.**
Insert into s6 ¶2 after "and every directive's actor":

> — and, in addition and to the same standard: the record's `status`, `assent_source`, `citation`, `issue`, `supersedes`, `disposes`, `exceptions`, `cites_authorities`, every directive's `id` and `when`, and every key preserved under the record's catch-all. A change to any of these is not content-preserving, is not on the slip track, and is not a rectification of form within the meaning of Part 3. The test is this test; neither the amending court **nor the Commission** may substitute another, and a rectification described as formal which fails it is void.

**G-2 — s15 and s17. DEFECT: the Act stays a trust-boundary fork that binding law requires to go to court before implementation, and pauses an entrenched auto-justiciability trigger it does not expressly vary. CONSTITUTIVE.**
Delete from s15 the words "is registered and STAYED; the Commission may not dispose of it and it awaits the courts" and substitute:

> is registered and **REFERRED AT ONCE**. The pause does not reach it and no warrant may extend the pause to reach it: a fork involving any verb rated SENSITIVE or above, any EXTERNAL authentication tier, or any token, capability, credential or permission-model change goes to the court **BEFORE implementation**, regardless of code reversibility ([2026] CC-OPBOX 16 remedy 3, `.justice/judgments/county-court/2026-cc-opbox-16.md:44`, READ; remedy quashed in part and the duty carried forward by [2026] CC-OPBOX 17, `2026-cc-opbox-17.md:59`, READ).

And insert into s17's unaffected list, after "breach matters":

> ; a TRUST-BOUNDARY FORK, which is never paused; an UNSATISFIABLE ENFORCEMENT GATE, which is auto-justiciable on its single fire and is never paused (ACT-003:s10, given effect by [2026] VJS-SC 4) — and the Clerk-Drafter records that a self-declared pause is precisely what ACT-003:s10 forbids in terms ("the pause is recorded, fail-loud, and court-tested: unsatisfiability is never self-declared", `lawpack/v2/statutes/03-agent-duties.yaml:171-173`, READ)

and add to s17 `kernel_effect.must_not`: `staying_a_trust_boundary_fork`, `staying_an_unsatisfiable_enforcement_gate`, `a_warrant_purporting_to_narrow_a_carve_out`. Amend Schedule 3 to state that s17 varies ACT-002:s6 only and **varies ACT-003:s5 and ACT-003:s10 not at all**.

**G-3 — s17 and s19. DEFECT: the pause is a self-classified ouster with review deferred by the classifier's own timetable — the first draft's s7, reconstituted. CONSTITUTIVE.**
Insert as a new paragraph of s17:

> WHO CLASSIFIES, and it is not the Commission. The classification of a matter as machinery and editorial is a **DECLARATION** by the person filing it, recorded with the declarant's name and the facts relied on. It is not a finding, and it binds no court. A party may apply **to the court, not to the register**, for a determination that a matter is not within the pause; such an application is itself a floor matter, is never paused, and is determined before the matter is entered on the register. A first-instance bench retains jurisdiction to sit **on its own motion** to determine whether a classification was correct. The Commission may not classify a matter into its own jurisdiction. "Editorial" means an act reaching only the matters in s16(a) to (i), and no other.

Delete from s19 the words "The pause defers review; nothing extinguishes it." and substitute:

> Every act of the Commission is **APPEALABLE AS OF RIGHT**, and an appeal from a Commission act is **NEVER STAYED by s17**. The prior text of every rectified instrument remains routable and resolvable until the appeal period closes. An act is set aside for any of: exceeding s16; failing the content-preservation test of s6 as extended by this Part; reaching a floor under s15; acting outside the warrant's jurisdiction or after expiry; or rectifying without recording the address and mode of the finding relied on.

And insert into s14, after "one commissioner per warranted jurisdiction":

> ; a warrant names exactly **ONE** jurisdiction, no more than **THREE** warrants run concurrently across the realm, and a Commission sits with not fewer than three seats carrying the four portfolios of the Standing Committee in substance. A single body with concurrent power over every jurisdiction could propagate an error to the whole estate before any of it was reviewed.

**G-4 — s16. DEFECT: the power list is not declared exhaustive; three prohibitions from the standalone draft are missing; s16(h) authorises promotion of recital into operative text.**
Open s16 with: "the Commission may do the following **and nothing else. The list is EXHAUSTIVE: a power not on it is not conferred**:". Delete s16(h) and substitute:

> (h) TRANSCRIBE, into a record's operative parts, only text the record **already carries in an operative part** of its own or of an instrument it expressly incorporates; a term standing in the issue, vote, question, runtime_summary, source opinion, case-file digest or any other recital is **not** transcribable into operative text by any Commission act, that being authorship and reserved to the courts and the Sovereign (s1, OPERATIVE PARTS);

Add to s16 `kernel_effect.must`: `confine_a_commission_to_the_exhaustive_power_list`. Add to `must_not`: `a_commission_deciding_a_reserved_question`, `a_commission_enacting_amending_reading_down_or_overruling_any_instrument`, `citing_a_commission_act_as_authority`, `substituting_another_content_preservation_test`. Amend s16(e) to require, on any substitution of a cited authority, "**proof of both**: that the named authority does not contain the proposition and that the substitute does, each with address and mode."

**G-5 — s2. DEFECT: FB-2 is not cured at s2; a form-only refusal is still a refusal, and the floor names form defects specifically.**
`lawpack/v2/statutes/10-assented-record-protection.yaml:22-30` (READ) forbids an assented record being "voided, excluded, or **blocked** by any subordinate validation, gate, invariant, regulation, or kernel operation", and names as routable "an inert or unevaluable kernel_effect, **an operation not yet implemented in the kernel registry**" — i.e. exactly the form class s2 refuses on. The dispositions ledger's "s2 by form-check-only" is an argument, not a saving. Add to s2, after "names each defective entry":

> A record declaring a valid assent_source is never blocked by this section; the defect is surfaced and routed for correction (ACT-ASSENTED-RECORD-PROTECTION:s1, cited by number).

**G-6 — s16(c). DEFECT: reclassification silently shrinks the audit population.** Add to s16(c): "a reclassification is recorded in the s18 register with the duty, the prior classification, the new classification, the reason a gate cannot hold it, and **the count before and after**; a reclassification that lowers the audited duty count without a stated reason is a breach."

**G-7 — s18. DEFECT: FB-7 reopened. An executed-output report to three organs with no redaction saving.** s18 requires "each by EXECUTED runs of the audits". Add: "Redaction under s2 applies to every recorded output in the register and the report; where an output would disclose data about a real person, a credential or a tenant identifier, the published form is redacted with its redaction authority and the unredacted form goes to the confidential annex. This never overrides ACT-CONSOLIDATION-FRAMEWORK:s21." (The standalone Act's s8 carried exactly this as an `exceptions` entry; Part 3 dropped it.)

**G-8 — s14 and s15. DEFECT: the three floors and the no-law/no-merits/no-precedent rule are `defines` tokens, which this Act's own s12 declares bind nothing.** Move to `must_not` in s15: `crossing_the_sovereign_assent_floor`, `derogating_from_any_limb_of_the_protective_floor`, `constituting_or_rectifying_an_apex_record_locally`. Move to `must_not` in s14: `a_commission_making_law_adjudicating_merits_or_creating_precedent`.

**G-9 — s17 and s11. DEFECT: no tolling; deferred matters have no restoration mechanism; s11 and s17 command incompatible acts.** Add to s17: "A pause **TOLLS** every reservation, review date and appeal period in the jurisdiction for its duration, and a matter it defers is convened on its expiry **without fresh application**. A reservation is not reported OWED under s11 for any period in which the pause ran."

**G-10 — s18 and s19. DEFECT: no report before renewal; the Commission is unobserved for its entire life.** Amend s19: "A warrant is renewed **only after a report under s18 covering the warrant it extends has been delivered**." Amend s18: an **interim** report is delivered before any renewal and, in any event, at seven-day intervals while a warrant runs. Add to s19 `must_not`: `extending_a_warrant_or_a_pause_by_inaction_or_an_unfinished_sweep`, `a_second_extension_without_a_delivered_report`.

**G-11 — s19. DEFECT: the good-faith-reliance immunity is unbounded.** Add: "Good-faith reliance excuses the relying party only, is available only where the Commission act was recorded in the s18 register before the act was complete, and never excuses the commissioner who made the act."

**G-12 — Schedule 1 ¶4 and the schedules generally. DEFECT: a Commission power over a constitutional Act's face, in a schedule with no commencement rule.** Amend Sch 1 ¶4: the Commission "**reports** how the live file's citation line and the register may be reconciled, and makes no act on the face of ACT-COMPUTER-FIRST-REALM; reconciliation of a constitutional Act's citation line is reserved to the Sovereign (ACT-CONSOLIDATION-FRAMEWORK:s10, assent non-delegable)."

## Operability (O-1 … O-5) — O-1 and O-2 constitutive

**O-1 — Schedule 2, closing sentence. DEFECT: A FALSE MACHINERY CLAIM ON THE ACT'S OWN FACE, in breach of the Act's own s2 and s20. CONSTITUTIVE.**
Part 3 commences whole on assent (s20) carrying 18 duty tokens, 0 of which appear in `GATE_REGISTRY` (EXECUTED cross-check). Delete "At no point does a section of this Act stand in force as a duty with no gate." and substitute:

> At no point does a section of **Part 2** stand in force as a duty with no gate. **PART 3 IS DIFFERENT AND SAYS SO.** Part 3 commences whole on assent and its eighteen duty tokens bind no gate at commencement, none being in the gate registry at `crates/vjs-lawpack/src/report.rs:36-241` (EXECUTED cross-check 2026-08-05: 71 duty tokens in this Act, 0 hits, 71 misses). That is deliberate and is not cured by delay: the Commission dissolves within thirty days (s19) and no gate for an executive organ can be built, reviewed and shipped in that window. Part 3's duties are therefore enforced by the Principal on the s18 register and report, and by the courts on review under s19, and by nothing else. They are reported UNWIRED by the audit, honestly, and this Act asserts nothing to the contrary.

**O-2 — s20 and the audit. DEFECT: commencement-by-section has no consumer, so the whole-Act arithmetic lands on assent day. CONSTITUTIVE (as disclosure).**
READ `crates/vjs-lawpack/src/conformance.rs:56-77`: `conformance_audit` iterates every statute section carrying a `kernel_effect` with no filter on status, Part, organ or commencement. EXECUTED `grep -rn "commence" --include=*.rs crates/` → 7 lines, 2 files, **all comments**. Insert into s20:

> WHAT THE AUDIT ACTUALLY READS, stated so this section is not itself the ceremony it forbids. The duty-conformance audit has **no representation of commencement**: it enumerates every section of every loaded statute (`crates/vjs-lawpack/src/conformance.rs:56-77`, READ), so from the day this Act's text lands in the lawpack the audit reads the whole-Act figure — projected `total 352, wired 43, unwired 309` against the baseline of 281/43/238 measured by EXECUTION of `vjs audit --json` on 2026-08-05. Commencement by section is a rule binding this legislature and the courts; it is not a rule the audit can apply until the audit reads a commencement field, and it does not read one. Until it does, no report of this Act's conformance may be described as reflecting commencement, and the tranche schedule is a duty on the Clerk and not a filter on the audit.

**O-3 — the Schedules. DEFECT: sch1, sch2 and sch3 are assigned to no Part and therefore have no commencement rule.** s20 assigns Parts 1, 3 and 4 to assent and Part 2 to Schedule 2's tranches. The three schedules sit under a "SCHEDULES" heading outside every Part, and Schedule 1 carries three `must` tokens (including `reserve_dec_15_to_22_pending_the_footing_matter`) and a Commission power (G-12). Schedule 3 declares amendments to ACT-002:s9 and ACT-004:s9 that the sections making them (s6) do not commence until tranche 3 — so on assent day the old rule is varied and the new one is not yet in force. Add to s20:

> Schedule 1 commences with s7. Schedule 3 commences, **as to each amendment it enumerates, on the commencement of the section that makes it and not before**; until then the amended provision stands unvaried. Schedule 2 commences on assent.

**O-4 — s13, third sentence. DEFECT: canonical primary law names a store that exists only in one subscriber.** EXECUTED `ls -d .justice` in canon → "No such file or directory"; the tree exists only in the subscriber (213 `.md` files, EXECUTED `find`). Substitute: "A jurisdiction whose law or court records are held in a tree outside its declared roots registers that tree on accession; **the opbox jurisdiction registers `.justice` on its accession under s21**, and this section states no duty on any store in the canonical jurisdiction that does not exist there."

**O-5 — s16 and Part 3 generally. DEFECT: Part 3 carries zero `exceptions` tokens; five of its eighteen duties can never be mechanically held and will sit in the unwired list indistinguishably from work not yet done.** Per my round-1 C18, move to `exceptions` with the enforcer named in the section text: s16 `changing_the_substance_of_any_operative_part` and `creating_or_modifying_any_trust_boundary_surface`; s15 `deriving_any_power_from_urgency_or_oral_direction`; s14 `act_only_under_a_written_warrant_of_the_principal`; s19 `void_and_route_for_correction_any_act_beyond_power`. Each to carry: "This limb is a duty on the commissioner and no gate holds it; it is enforced by the Principal on the s18 register and by the courts on review under s19."

---

## Closing note for the Clerk-Drafter

The second draft is a substantially better instrument than the first, and Part 2 is nearly adoptable on both my seats. **Part 3 is not — and the reason is recoverable, because the cure already exists in this drafter's own hand.** `ACT-RECTIFICATION-COMMISSION.yaml` (10 sections) contains, in terms, almost every safeguard Part 3 is missing: the exhaustive power list, the trust-boundary and unsatisfiable-gate carve-outs from the stay, tolling, convening-without-fresh-application, the one-jurisdiction warrant, the three-seat/four-portfolio bench, appeal as of right unstayed, the report before expiry with the duty to name what was **not** done and every store not reached, the fail-closed/fail-open asymmetry, and `must_not: extending_a_stay_by_inaction_or_an_unfinished_sweep`.

Compressing that Act into six sections of a Bill dropped them. The compression was not neutral: **it inverted the trust-boundary rule** — the one rule whose mis-citation was my central NAY ground at round 1, and the one this very Act corrects on its face at s21. That is the finding this bench cannot vote past.