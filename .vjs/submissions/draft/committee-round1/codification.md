## VERDICT

**NAY.**

Three independent grounds, each fatal to adoption of *this* text as *this* instrument:

1. **s11 narrows an entrenched floor without citing it by number.** `[2026] VJS-PC 17` (`lawpack/v2/orders/2026-VJS-PC-017.yaml:199`, READ) reserves any narrowing of the `ACT-ASSENTED-RECORD-PROTECTION` floor "exclusively to a sovereign-assented act citing vjs_act_10 and act_consolidation_framework_s7_s25 by number". s11 declares an unloadable instrument "NOT IN FORCE. It binds nothing" — exclusion of an assented record for an unevaluable form defect, the exact case `ACT-ASSENTED-RECORD-PROTECTION:s1` names ("an inert or unevaluable kernel_effect ... is ALWAYS routed for correction, never silently excluded or voided"). The draft cites neither instrument by number. s15 then asserts the Act "does not disturb ... the assented-record floor". s11 and s15 cannot both stand.
2. **The citation is self-minted.** Ground 3 below.
3. **The vehicle is wrong.** Nine of fifteen sections are amendments to four existing Acts or machinery for one regulation. Consolidation over fragmentation is a governing principle of the very Act this draft invokes at s5.

The defect in s4's factual predicate is a fourth ground but is curable by re-measurement.

---

## FALSE CLAIMS FOUND

**FC1 — s4: `vjs next-citation` is not broken; the invocation was.** EXECUTED from `/home/jellytot/Projects/opbox-prod/opbox-kernel`:

```
$ vjs next-citation CC 2026
Next citation: [2026] VJS-CC-OPBOX 165
```

The draft asserts the allocator returns "`[2026] VJS-VJS-CC-OPBOX 1` — a doubled prefix and the number 1 — while [2026] VJS-CC-OPBOX 164 is in force", and concludes "Citations are therefore hand-allocated today." The output it quotes is real (EXECUTED: `vjs next-citation VJS-CC-OPBOX 2026` → `[2026] VJS-VJS-CC-OPBOX 1`) but is caller error. READ at `/home/jellytot/Projects/vibe-justice-system/crates/vjs-cli/src/admin.rs:5-11` the parameter is `series: String`; READ at `admin.rs:32-40` the repo segment is appended only where `s == "CC"`, from `resolve_repo_code(repo)`. Passing `VJS-CC-OPBOX` supplies an unstarted series, hence `1`. **The conclusion drawn is FALSE, and it is the entire factual predicate of s4.** It is also precisely the error s1 forbids: an inference from the shape of a name, asserted without reading the argument contract.

**FC2 — s4: "PC 13 directive 2 is UNDISCHARGED" is unproven and contradicted in code.** READ `/home/jellytot/Projects/vibe-justice-system/crates/vjs-core/src/citation.rs:1-11`: "The live persisted register PC-13 D2 requires it to read is the body of governed records itself ... the former in-memory `CitationRegistry` / `CitationSeries` / `Citation` types ... have been removed to leave one source of truth." READ `crates/vjs-lawpack/src/validator.rs:414-427`: `live_citation_max` takes all roots and the comment records the earlier single-root defect and its cure. D2 is implemented against a different persistence model than "a registry artefact". s4's search for a "registry artefact" was a bounded search reported as a fact about the corpus — the defect s10 forbids.

**FC3 — s7: `[2026] VJS-CC-OPBOX 16, 17, 18` do not hold what is attributed to them, and 17/18 did not affirm 16.** READ `/home/jellytot/Projects/opbox-prod/opbox-kernel/.vjs/orders/`:
- `2026-VJS-CC-OPBOX-016.yaml` — `issue: cc_opbox_15_clarification_form_ingest_nature_and_per_verb_body_cap`
- `2026-VJS-CC-OPBOX-017.yaml` — `issue: durable_flow_waking_design`
- `2026-VJS-CC-OPBOX-018.yaml` — `issue: agent_in_flow`

None is a trust-boundary rule. The rule cited lives in a **different series**: READ `/home/jellytot/Projects/opbox-prod/opbox-kernel/.justice/INDEX.md`, rows for `[2026] CC-OPBOX 16/17/18` (no `VJS-` prefix) — 16 is "breach: Lexby failed to self-refer trust-boundary fork (**remedy quashed in part per CC-OPBOX 17/18**)"; 17 is "CA: CC-OPBOX 16 remedy **inadequate**"; 18 is "SC: duty **not enforceable** unless in agent context". "Affirmed by 17 and 18" is the opposite of the record. s7 saves a rule against the wrong citations, in the wrong series, on a mis-stated appellate history.

**FC4 — s10: "created six days earlier" and the named file.** The source of this passage is READ at `/home/jellytot/Projects/opbox-prod/opbox-kernel/.vjs/submissions/filed/SUBMISSION-2026-08-03-citator-corpus.yaml:44-48`, which names `2026-VJS-CC-OPBOX-CONTROL-BOUNDARY-CONTRAST-002.yaml`. EXECUTED: that file does not exist in `.vjs/orders/` (`ls | grep -i CONTROL-BOUNDARY` → empty; `git log --diff-filter=A` → empty). The record that does exist is `.vjs/orders/2026-VJS-CC-OPBOX-002.yaml` (`created_at: 2026-06-11T10:09:30Z`, `status: binding`; EXECUTED `git log --diff-filter=A` → `2026-06-11`) — **fifty days before 2026-07-31, not six.** Further, `[2026] CC-OPBOX 2` in the `.justice/` citator carries status **`per-incuriam`** (INDEX.md), so the bench's refusal to rely on it may have been correct on the citation it was actually given. s10's exemplar carries no address, in breach of s1.

**FC5 — s15: "the opbox mirror was verified byte-identical to canon for all ten statutes" is true of one store and false of the jurisdiction.** EXECUTED `cmp` over all ten:
- `/home/jellytot/Projects/opbox-prod/opbox-kernel/lawpack/v2/statutes/` → 10/10 IDENTICAL.
- `/home/jellytot/Projects/opbox-prod/opbox-kernel-confirmation-fix/lawpack/v2/statutes/` → 10/10 IDENTICAL.
- `/home/jellytot/Projects/opbox-prod/opbox-kernel/.worktrees/tablelist-ext/lawpack/v2/statutes/` → **`10-assented-record-protection.yaml` ABSENT ENTIRELY**; `03-agent-duties.yaml` **truncated at line 152, missing the Sovereign-assented `ACT-003:s10` and `s11`** (EXECUTED `diff` → `153,209d152`).

A third in-repo mirror is missing an entire Act and two assented sections. s15's claim is an unenumerated single-store measurement — the defect s4 ¶2 and s10 both forbid, committed in the section that claims to preserve the property.

**Claims I checked and found TRUE:** `ACT-002:s7` is "Orders bind; opinions explain" and holds no existence limb (`lawpack/v2/statutes/02-courts-orders.yaml:107-121`). "per incuriam" appears in no statute — only `orders/2026-VJS-SC-006.yaml`, `orders/2026-VJS-PC-017.yaml`, `judgments/2026-VJS-SC-4-opinion.md`, `judgments/2026-VJS-SC-6-opinion.md` (EXECUTED `grep -rln`). The kernel finding is at `crates/vjs-engine/src/staged.rs:364-377` with the message at `:369-371` and `.citing("ACT-002:s7")` at `:375`. 163 and 164 both carry `issue: dec15_dec19_recitation_and_order_completion`. `160 O3` reads `default_directive_actor_to_an_explicitly_UNSTATED_value` (`:43-46`). "55 of 109" is at `2026-VJS-CC-OPBOX-160.yaml:9`.

**One address correction:** s3 cites `governance/crates/vjs-engine/src/staged.rs:364-377`. There is no `governance/` directory (EXECUTED `ls -d` → "No such file or directory"). The address is `crates/vjs-engine/src/staged.rs:364-377`.

---

## CONFLICTS TABLE

| § | Instrument conflicted with | Nature |
|---|---|---|
| s1 | `[2026] VJS-PC 17` D1 (`orders/2026-VJS-PC-017.yaml:172`) | PC 17 D1 defines the region as "the holding string, each directive's must text and each forbidden clause" — in **D1 alone**, not "D1-D5" (D2 = existence-not-in-force, D3 = section granularity, D4 = dangling-reference reconciliation, D5 = candid bounded reach). PC 17 uses **"operative parts"**; the draft coins **"operative surface"**, an undefined term, and attributes it to PC 17. |
| s1, s10 | `ACT-003:s4`, `ACT-004:s7` | New evidential duties on filings/logs placed outside the Act that governs agent duties and log content. Duplication risk, no supersession statement. |
| s2 | `ACT-004:s9` (`supersession.explicit`), `ACT-002:s9` | Restates supersession-must-be-explicit and adds `varies`/`supplemental` relations without amending either section. Two loci for one rule. |
| s3 | `[2026] VJS-PC 17` D1 + Q2 holding | PC 17 D1 **directs** the kernel to "cite ... act_002_s7 and reg_kernel_001 on every denial". `staged.rs:375` is therefore **compliance with a binding order**, not the engineer's blunder s3 describes. s3's `must: cite_this_section_and_not_act_002_s7` **silently varies PC 17 D1**. Separately, PC 17 held the defect **correctable, not constitutive**; s3's "An order does not bind to the extent that it relies on an authority which does not exist" makes it self-executingly constitutive — silent overruling of PC 17's ratio. |
| s3 | `REG-KERNEL-001` (`regulations/REG-KERNEL-001.yaml`, full text READ) | The phrase "clerk-not-court" **does not appear in REG-KERNEL-001**. It appears at `statutes/08-computer-first-realm.yaml:165` (`ACT-COMPUTER-FIRST-REALM:s11`) and `:216` (s15). PC 17's forbidden clause (`:202`) anchors per-incuriam voidness to **`ACT-002:s9`**. The draft propagates PC 17's own mis-citation into primary law. |
| s3 | `ACT-002:s9`, `ACT-002:s5` | The correction/variation/overruling anchor already exists at `ACT-002:s9`. s3 creates a second, competing anchor. |
| s4 | `ACT-004:s8`, `REG-003` (`[2026] VJS-REG 3`, authority `ACT-004:s8`), `[2026] VJS-PC 13` D2 | s4 ¶1 is a near-verbatim restatement of PC 13 D2 (`orders/2026-VJS-PC-013.yaml:122`) elevated to primary law with no supersession declared, contra `ACT-004:s9`. PC 13's holding (`:49-51`) already classified citation allocation as **machinery under `ACT-CONSOLIDATION-FRAMEWORK:s7`** and the Court "REFUSES to re-legislate any settled duty". s4 re-legislates it. |
| s4 | `crates/vjs-core/src/front_door.rs:84-97` (`GOVERNED_RECORD_ROOTS`) | s4's interim rule ("every store that can hold a citation") is unimplementable as drafted: the roots are `lawpack/v2` (yaml only), `.vjs/orders`, `.vjs/court`. `.justice/INDEX.md`, which holds the entire `[2026] CC-OPBOX 1-…` series in Markdown, is invisible to `live_citation_max`. s4 does not name it. |
| s4 | `ACT-004:s8`, `REG-003` | Undisclosed live collision the section should have caught: `.justice/INDEX.md` runs `[2026] CC-OPBOX N` (no `VJS-` prefix) in parallel with `.vjs/orders`' `[2026] VJS-CC-OPBOX N` at overlapping ordinals. `ACT-004:s8` sets format `[YYYY] VJS-<COURT>-<REPO> N` and `collision_policy: fatal`. |
| s5 | `ACT-CONSOLIDATION-FRAMEWORK:s7` | s7 vests the SI power in **the Standing Committee**. s5 makes a class of matter "disposable by a REGULATION ... without a full order and opinion", i.e. transfers disposal of a justiciable matter from a court to a non-judicial organ. That is a jurisdictional change; s7 expressly may not "amend, disapply, or expand ... any primary Act". |
| s5, s7 | `ACT-002:s6`, `ACT-003:s5`, `ACT-003:s10` | `ACT-002:s6` triggers court on first impression, distinction, overruling, conflict, **breach**; `ACT-003:s5` requires self-file + court route on breach; `ACT-003:s10` makes an unsatisfiable gate auto-justiciable **on the first fire regardless of risk**. s7's "The route must not return `CourtRequired` for it" disapplies all three for any matter the actor rates low-risk and reversible. `CourtRequired` is returned at `crates/vjs-core/src/route.rs:97-99` for `Breach`, `Conflict`, `FirstImpression` (READ). |
| s6 | `ACT-004:s9`, `ACT-002:s9` | "cease to be separately routable" is a disposition `ACT-004:s9` does not authorise ("The old authority **remains visible**"). Where a superseded instrument is assented, non-routability is exclusion, contra `ACT-ASSENTED-RECORD-PROTECTION:s1`. And a consolidating instrument that supersedes a series engages `ACT-002:s9`'s overruling reservation to the Supreme Court. |
| s7 | `ACT-004:s7`, `ACT-003:s2` | Redefines "reversible" ("judged by blast radius ... not by whether the code can be edited back"). The term is already load-bearing at `ACT-004:s7` (`log.links_to ... reversibility`) and `ACT-003:s2` ("trivial, reversible, and expressly exempt"), where it is undefined. s7 changes its meaning realm-wide from inside a section about deliberation budgets. |
| s8 | `ACT-002:s9`, `ACT-004:s9` | `ACT-002:s9`: "Orders may be corrected for clerical errors. Variation requires a new order." `ACT-004:s9`: "**Corrections are new records**" + `must_not: delete_old_records`. s8's on-the-face `amendments:` block mutates the original record — a direct contradiction of both, and s8 says so of neither. |
| s8 | `ACT-002:s5`, `ACT-COMPUTER-FIRST-REALM:s29`, `[2026] VJS-SC 4` | "anything else requires a superior court on appeal" has no referent for a Supreme Court order (`ACT-002:s4`, apex; `ACT-002:s5` no Court of Appeal in MVP; `ACT-008:s29` CoA tier persists but non-convened). A substantive SC error not on its face is unamendable. `[2026] VJS-SC 4` forbids a subscriber recording an apex judgment locally, so a subscriber's "superior court" must be named as the canonical PC/SC; s8 does not say so. |
| s9 | `ACT-004:s8` (`citation.collision_policy: fatal`), `REG-003` | A forwarding record makes one citation resolve to two objects. Nothing states how the forwarded old value interacts with `check_citation_uniqueness` / `allow_duplicate_citations`. No forwarding mechanism exists in the kernel (EXECUTED `grep -rni forward crates/vjs-lawpack/src/*.rs crates/vjs-core/src/citation.rs` → only unrelated "forward self-reference" at `refs.rs:150,233`). |
| s11 | `ACT-ASSENTED-RECORD-PROTECTION:s1` and `:s2` | Ground 1 of the NAY. Voiding-by-unloadability of an assented record. `:s2` requires amendment "only by a Sovereign-assented constitutional Act citing this Act by number"; the draft does not cite it by number. |
| s11 | `REG-COURT-RECORD-001` (`[2026] VJS-REG 22`) | `must_not: invalidate_a_legacy_ruling_for_want_of_the_structured_fields`; `binds: prospective_rulings_only`. The 55 unparsed opbox orders are exactly legacy rulings lacking structured fields. s11 invalidates them; the Regulation forbids it. `ACT-COMPUTER-FIRST-REALM:s5` supplies the correct disposition: "a validation defect **routed for correction**; the kernel must not silently choose." |
| s11 | draft s3 | Internal: s3 routes for correction and never voids; s11 voids automatically. Same defect class, opposite disposition, no reconciliation. |
| s14 | `[2026] VJS-PC 13` (mandated full-spectrum conformance audit) | s14 alters the audit's counting rules by primary law without naming PC 13 or declaring the variation. |
| s15 | `ACT-007:s3`, `ACT-001:s9`, `ACT-CONSOLIDATION-FRAMEWORK:s15` | `ACT-007:s3` permits local variation with `exceptions: privy_council_order_authorises, principal_assent`. `ACT-001:s9` permits opt-out of specific **regulations** and routes sovereignty changes through Principal assent or PC order. `ACT-CONSOLIDATION-FRAMEWORK:s15` expressly preserves "the local sovereign's amend, pin/decline, fork, and exit routes". s15's absolute "may not amend this Act in its mirror" deletes all three routes without amending any of them. |
| s15 | `ACT-CONSOLIDATION-FRAMEWORK:s21` | s15 calls s21 "the real-world-law floor". s21 is the **protective floor**, of which the real-world-law floor is one of four limbs (the others: rights/standing/due process; public-private boundary; restorative-remedy-not-punishment). Mis-description of an entrenched, non-derogable provision in a savings clause. |
| whole Act | `ACT-COMPUTER-FIRST-REALM` purpose (`:16-18`), `ACT-CONSOLIDATION-FRAMEWORK` purpose (`:15-16`), `ACT-ASSENTED-RECORD-PROTECTION` purpose (`:16`) | Ground 2 of the NAY. See below. |

---

## CITATION AND NUMBERING

**Is the ordinal free?** Arithmetically yes. EXECUTED from the canonical repo: `vjs next-citation ACT 2026` → `Next citation: [2026] VJS-ACT 11`. `[2026] VJS-ACT 1-10` are held (EXECUTED `grep -n "citation:" lawpack/v2/statutes/*.yaml`).

**Is a self-minted ordinal lawful?** No, and the draft's own drafting_note concedes the analogy while committing the act. Three in-force Acts state the rule, each of them a precedent the draft relies on elsewhere:

- `statutes/08-computer-first-realm.yaml:16-18`: "The Act's own VJS-ACT ordinal is minted deterministically at commencement per its citation-scheme section; **the earlier self-mint '[2026] VJS-ACT 8' was void and is not used**."
- `statutes/09-consolidation-framework.yaml:15-16`: "Its VJS-ACT ordinal is minted deterministically at commencement; **no self-mint is used**."
- `statutes/10-assented-record-protection.yaml:16`: "The VJS-ACT ordinal is minted at commencement."

A draft that writes `citation: "[2026] VJS-ACT 11"` into its own front matter, before Committee adoption and before Assent, has done exactly what `ACT-COMPUTER-FIRST-REALM` records as void, and what `ACT-001:s7` forbids (`agent_draft_becomes_binding_by_fact_of_being_written`; `must: mark_proposed_law_as_draft`). Note the second-order effect: `live_citation_max` counts `08-computer-first-realm.yaml`'s literal `citation: "[2026] VJS-ACT 8"` as allocated even though that Act's own purpose declares it void, so the allocator's maximum is already polluted by a void self-mint. A second self-mint compounds it.

**Correct handling.** The draft must carry **no `citation:` field at all**, only `id: ACT-PROCEEDINGS-DISCIPLINE`, until commencement. The ordinal is minted at commencement by the deterministic allocator — `LawpackValidator::live_citation_max` at `crates/vjs-lawpack/src/validator.rs:428`, reached via `cmd_next_citation` at `crates/vjs-cli/src/admin.rs:5`, over `front_door::governed_record_roots` — on the Sovereign's assent event, and stamped into the commencement record (the `COMMENCEMENT-V2-000N` pattern at `lawpack/v2/provenance/founding/`). **Not by Lexby, not by the Committee, and not by the draft.** The Clerk-Drafter's role is to leave the field absent.

There is a further inconsistency the second draft should not inherit: `08-computer-first-realm.yaml:2` still carries `citation: "[2026] VJS-ACT 8"` while `:16-18` declares that value void. That is a live canon defect, outside this Act's scope, and should be raised as a separate matter rather than cured here.

---

## PLACEMENT OPINION

Consolidation over fragmentation cuts against this Act as a vehicle. Only s2, s6, s13 and s14 are genuinely new general doctrine, and even those have natural homes.

| § | Opinion |
|---|---|
| s1 | **Amend into `ACT-003` as `ACT-003:s12`.** Pleading standard on machinery claims is an agent duty; `ACT-003` is the Agent Duties and Route Act, and s1's own subject ("the engineer stated the behaviour of machinery he had not run") is conduct. |
| s2 | **Amend into `ACT-002:s9`** (Correction, variation, and overruling) as new limbs, or into `ACT-004:s9` (Supersession and correction). Issue-tag exclusivity is a supersession rule. Do not create a third locus. |
| s3 | **Do not enact as drafted. Amend into `ACT-002:s9`** — that is the section `[2026] VJS-PC 17:202` already names as the per-incuriam anchor. A new s3 with a competing anchor multiplies the defect it claims to cure. |
| s4 | **Machinery. Not primary law.** `[2026] VJS-PC 13` already directed it under `ACT-CONSOLIDATION-FRAMEWORK:s7` and refused to re-legislate it. If a statutory hook is wanted, **amend `ACT-004:s8`**; the rest belongs in an amendment to `REG-003`. |
| s5 | **Amend into `ACT-CONSOLIDATION-FRAMEWORK:s7`,** or drop. Creating a registrar disposal track from outside s7 is an expansion of s7 by a non-s7 instrument. |
| s6 | **Amend into `ACT-004:s9`.** Consolidation of a series is a supersession rule and `ACT-004:s9` is where supersession lives. Alternatively `ACT-CONSOLIDATION-FRAMEWORK` — it is the consolidating Act. |
| s7 | **Amend into `ACT-002:s6`** as an express sixth limb ("when court does *not* convene"). Enacting a no-court rule anywhere other than the section that lists the court triggers guarantees the two drift. |
| s8 | **Amend into `ACT-002:s9`** (the making court's correction power) and `ACT-004:s9` (correction-is-a-new-record). This is the clearest single case of a section that must be an amendment: it contradicts both sections in force and would sit beside them unreconciled. |
| s9 | **Amend into `ACT-004:s8`** + a `REG-003` amendment for the forwarding mechanism. |
| s10 | **Amend into `ACT-004:s7`** (logs and authority basis) or `ACT-003`. A NOT-FOUND enumeration duty is a record-form duty. |
| s11 | **Keep-in-new-Act only if the entrenchment route is taken** (see Condition 1). Otherwise re-cast as an amendment to `REG-COURT-RECORD-001` in reader-widening terms with **no** not-in-force limb. |
| s12 | **Amend into `ACT-002:s10`** (Order format requirements — `must_not: accept_order_without_directives`). Actor-on-directive is a format requirement and s10 is the format section. |
| s13 | **Keep-in-new-Act,** or amend into `ACT-002:s10`. Genuinely new; no reservation-expiry rule exists in canon (EXECUTED `grep -rln "review_date"` over `lawpack/v2/` → no statute or regulation). |
| s14 | **Keep-in-new-Act.** Genuinely new as statute — "negative control" appears only in `judgments/2026-VJS-CC-VJS-13/16/17-opinion.md`, so this codifies judge-made doctrine, which is a proper use of primary law. Must name `[2026] VJS-PC 13` as varied. |
| s15 | **Keep-in-new-Act,** rewritten. An extent clause is required for whatever survives. |

**Net:** what remains for a new Act is s13, s14, an extent clause, and a definitions section — with s11 joining only if entrenchment is properly invoked. Everything else is five amending schedules (`ACT-002`, `ACT-003`, `ACT-004`, `ACT-CONSOLIDATION-FRAMEWORK`) and two regulation amendments (`REG-003`, `REG-COURT-RECORD-001`). Consider retitling: "citation integrity" is `ACT-004`'s subject, and half of what the title promises is machinery already directed by PC 13.

---

## CONDITIONS

**C1 — front matter, citation (fatal).** Delete the line `citation: "[2026] VJS-ACT 11"` in its entirety. Insert into `purpose`, verbatim:

> This Act carries no citation as drafted. Its VJS-ACT ordinal is minted deterministically at commencement by the kernel allocator (`LawpackValidator::live_citation_max`, `crates/vjs-lawpack/src/validator.rs:428`, reached by `vjs next-citation ACT`, over `front_door::governed_record_roots`), and is stamped into the commencement record pinning the Sovereign-assent event and the assented-text digest. No self-mint is used. The Clerk-Drafter, the Committee and the kernel are each incapable of minting it; `[2026] VJS-ACT 11` appeared in the first draft and was void, on the same ground as the void self-mint "[2026] VJS-ACT 8" recorded at `ACT-COMPUTER-FIRST-REALM` purpose.

**C2 — s11, entrenched floor (fatal).** Delete the words "is NOT IN FORCE. It binds nothing, and" and substitute:

> is a validation defect ROUTED FOR CORRECTION and is never treated as voided, excluded or not-in-force by the clerk (ACT-ASSENTED-RECORD-PROTECTION:s1; ACT-COMPUTER-FIRST-REALM:s5, the kernel must not silently choose; REG-COURT-RECORD-001, which forbids invalidating a legacy ruling for want of the structured fields). Nothing in this section narrows the assented-record floor, and this section is to be read subject to it. Every command that returns a binding instruction must

Then insert a new final paragraph in s11:

> An unloadable instrument is reported as UNRESOLVED-FORM, never as not-in-force. Whether such an instrument binds is a merits question for a court on appeal (ACT-002:s9), never a clerk's conclusion.

If the Committee intends the stronger not-in-force rule, it may not be enacted in this text: it requires a separate Sovereign-assented constitutional Act citing `ACT-ASSENTED-RECORD-PROTECTION` and `ACT-CONSOLIDATION-FRAMEWORK:s7` and `:s25` by number, per `[2026] VJS-PC 17:199` and `ACT-ASSENTED-RECORD-PROTECTION:s2`. Record that reservation on the face of s11 and take it no further here.

**C3 — s3, PC 17 D1 and the correct anchor (fatal).** Replace the sentence beginning "The record is ROUTED FOR CORRECTION" and the parenthetical with:

> The record is ROUTED FOR CORRECTION and is never voided by the clerk; voidness on the ground of per incuriam depends on whether the holding depends on the bad citation, which is a conclusion of law reserved to a court on appeal (ACT-002:s9; ACT-COMPUTER-FIRST-REALM:s11 and :s15, the kernel is clerk, not court). The phrase "clerk-not-court" does not appear in REG-KERNEL-001; the first draft repeated that mis-attribution from [2026] VJS-PC 17 and it is corrected here.

Delete the first sentence "An order does not bind to the extent that it relies on an authority which does not exist." and substitute:

> An order that relies on an authority which does not exist carries a Fatal but CORRECTABLE defect. It is not deprived of force by the defect and not by this section; [2026] VJS-PC 17 (Q2) so held and this section codifies rather than varies that holding.

Replace `must: cite_this_section_and_not_act_002_s7` with:

> - cite_act_002_s9_and_this_section_and_not_act_002_s7_on_an_unresolved_operative_citation

and insert into the s3 text:

> This section VARIES [2026] VJS-PC 17 D1 to the extent that D1 directs the kernel to cite ACT-002:s7 and REG-KERNEL-001 on every denial. The variation is express. The finding at `crates/vjs-engine/src/staged.rs:364-377` (READ) cites ACT-002:s7 in compliance with that directive, not in error, and the first draft's characterisation of it as the engineer's defect is withdrawn.

Correct the address to `crates/vjs-engine/src/staged.rs:364-377` (there is no `governance/` directory; EXECUTED `ls -d` → No such file or directory).

**C4 — s1, PC 17 attribution.** Replace "[2026] VJS-PC 17 D1-D5 defines it as holding plus each directive's must plus each forbidden clause" with:

> [2026] VJS-PC 17 D1 (`lawpack/v2/orders/2026-VJS-PC-017.yaml:172`, READ) defines an order's OPERATIVE PARTS as the holding string, each directive's `must` text, and each forbidden clause, and D1 alone does so. The term used in the first draft, "operative surface", appears nowhere in PC 17 and is not used in this Act; the statutory term is OPERATIVE PARTS.

Replace every occurrence of "operative surface" in the Act with "operative parts".

**C5 — s4, false predicate (fatal to the section as drafted).** Delete the entire third paragraph beginning "THE PRESENT STATE, measured 2026-08-04" and substitute:

> THE PRESENT STATE, re-measured 2026-08-05 and EXECUTED. From the opbox jurisdiction, `vjs next-citation CC 2026` returns "[2026] VJS-CC-OPBOX 165", one past the in-force 164; from the canonical repo, `vjs next-citation ACT 2026` returns "[2026] VJS-ACT 11". The allocator works. The first draft's contrary finding rested on `vjs next-citation VJS-CC-OPBOX 2026`, which supplies an unstarted series to a parameter that takes the bare series code (`crates/vjs-cli/src/admin.rs:5-11` and `:32-40`, READ, where the repo segment is appended only where the series equals "CC"), and the doubled prefix and the "1" were both artefacts of that malformed invocation. Citations are NOT hand-allocated today, and the first draft's conclusion that they are is withdrawn. That finding was itself an inference from the shape of a name, asserted without reading the argument contract, and is the exact defect s1 forbids.
>
> What IS measured and unremedied is different, and narrower. The allocator reads the roots declared at `crates/vjs-core/src/front_door.rs:84-97` (READ): `lawpack/v2` (YAML only), `.vjs/orders`, `.vjs/court`. The `.justice/` citator is not among them, so the entire `[2026] CC-OPBOX 1-…` series recorded in `.justice/INDEX.md` (READ) is invisible to allocation, and that series runs at ordinals overlapping the `[2026] VJS-CC-OPBOX` series in `.vjs/orders`, in a form that omits the `VJS-` element ACT-004:s8 requires. Two parallel series at colliding ordinals in one jurisdiction is a fatal-collision condition under ACT-004:s8 and REG-003, and no gate presently sees it.

Delete "PC 13 directive 2 is UNDISCHARGED and this section does not pretend otherwise" and substitute:

> [2026] VJS-PC 13 D2 is implemented, and implemented against a persistence model the first draft did not look for: the persisted register is the body of governed records itself, each carrying its own top-level `citation:`, from which the allocator takes the highest allocated N (`crates/vjs-core/src/citation.rs:1-11` and `crates/vjs-lawpack/src/validator.rs:414-427`, READ; the former in-memory CitationRegistry types were removed to leave one source of truth). This section therefore does not require a registry ARTEFACT and must not be read to require one; any such reading would invalidate every citation in canon.

Replace the first sentence of s4 with:

> A citation is valid only if allocated by the deterministic allocator over every declared governed-record root of the jurisdiction (ACT-004:s8; REG-003; [2026] VJS-PC 13 D2).

Add to `must`:

> - name_every_store_that_can_hold_a_citation_including_any_citator_outside_the_declared_governed_record_roots

**C6 — s7, court triggers (fatal).** Insert as the second paragraph of s7:

> This section EXPRESSLY VARIES ACT-002:s6 and no further. It does not reach a matter engaging any of the five court triggers: it is no answer that a first-impression question (ACT-002:s6(1)) or a discovered breach (ACT-002:s6(5)) is low-risk and reversible, and ACT-003:s5 (self-file and correct, court route where correction is not straightforward) and ACT-003:s10 (an unsatisfiable enforcement gate is auto-justiciable on its first fire, regardless of risk or reversibility) are untouched. The route may return CourtRequired for such a matter and must (`crates/vjs-core/src/route.rs:97-99`, READ, where CourtRequired is returned for Breach, Conflict and FirstImpression).

Add to `must_not`:

> - reading_this_section_as_disapplying_any_of_the_five_court_triggers_or_act_003_s5_or_act_003_s10

**C7 — s7, trust-boundary savings, wrong citations (fatal).** Replace "[2026] VJS-CC-OPBOX 16, affirmed by 17 and 18" with:

> [2026] CC-OPBOX 16 in the reconstituted County Court at opbox citator (`.justice/INDEX.md`, READ; the series carries no VJS- element), whose remedy was QUASHED IN PART by [2026] CC-OPBOX 17 (the CC-OPBOX 16 remedy was held inadequate) and [2026] CC-OPBOX 18 (the duty is not enforceable unless encoded in the agent's governing instructions),

Insert:

> The first draft cited [2026] VJS-CC-OPBOX 16, 17 and 18 in the `.vjs/orders/` series. Those are different orders on different questions (16: per-verb body cap and form-ingest nature; 17: durable flow waking design; 18: agent in flow), they hold nothing about trust boundaries, and 17 and 18 did not affirm 16 in either series. The mis-citation is recorded rather than quietly fixed.

**C8 — s8, correction-is-a-new-record (fatal).** Insert as the paragraph before "IN BOTH TRACKS":

> This section EXPRESSLY VARIES ACT-002:s9 (orders may be corrected for clerical errors; variation requires a new order) and ACT-004:s9 (corrections are new records) to the extent that an amendment under either track is recorded on the face of the amended order rather than only as a fresh record. Neither section's prohibition on deleting an old record is disturbed: the exact prior text is preserved verbatim in the `amendments:` block, so nothing is deleted and the old text remains visible within the meaning of ACT-004:s9.

Replace "anything else requires a superior court on appeal" with:

> anything else requires a superior court on appeal, which for a County Court order of a subscribing jurisdiction means the canonical Privy Council or Supreme Court and never a locally-constituted apex ([2026] VJS-SC 4). Where the order to be amended is a Supreme Court order there is no superior court (ACT-002:s4, ACT-002:s5, ACT-COMPUTER-FIRST-REALM:s29), and the substantive track is available only for an error of the Court's own recorded on the face of the order; any other substantive amendment of a Supreme Court order is RESERVED and this Act does not provide a route for it.

**C9 — s9, uniqueness reconciliation.** Insert:

> A forwarding record does not create a duplicate citation for the purposes of ACT-004:s8 (`citation.collision_policy: fatal`) or REG-003 (`must_not: allow_duplicate_citations`). The forwarded value resolves to exactly one instrument, the new one, and is marked amended; the uniqueness check must treat a marked forwarding entry as a pointer and not as a second allocation, and REG-003 is to be amended accordingly. No forwarding mechanism presently exists in the kernel (EXECUTED `grep -rni forward crates/vjs-lawpack/src/*.rs crates/vjs-core/src/citation.rs` returns only an unrelated forward self-reference at `crates/vjs-lawpack/src/refs.rs:150` and `:233`), and this section states a duty owed, not machinery in place.

**C10 — s6, retention.** Replace "The superseded instruments are retained as record and cease to be separately routable" with:

> The superseded instruments are retained as record and remain visible and citable (ACT-004:s9). They cease to be separately ROUTED AS THE OPERATIVE RULE, and nothing in this section excludes, blocks or renders unresolvable any instrument, nor any Sovereign-assented instrument in any degree (ACT-ASSENTED-RECORD-PROTECTION:s1). Consolidation under this section is not overruling and does not displace ACT-002:s9's reservation of overruling to the Supreme Court.

**C11 — s14, PC 13.** Insert:

> This section VARIES the counting rules of the full-spectrum conformance audit mandated by [2026] VJS-PC 13. The variation is express.

**C12 — s15, local sovereignty (fatal).** Replace "A subscribing jurisdiction may not amend this Act in its mirror" with:

> A subscribing jurisdiction may not amend this Act in its mirror otherwise than by the routes ACT-007:s3, ACT-001:s9 and ACT-CONSOLIDATION-FRAMEWORK:s15 already preserve: a Privy Council order, or the Principal's assent. Nothing in this section removes the local sovereign's amend, pin/decline, fork or exit routes, and this Act does not purport to amend ACT-007:s3, ACT-001:s9 or ACT-CONSOLIDATION-FRAMEWORK:s15.

**C13 — s15, protective floor mis-description.** Replace "the real-world-law floor (ACT-CONSOLIDATION-FRAMEWORK:s21)" with:

> the protective floor in all four of its limbs — the real-world-law floor; rights, standing and due process; the public/private boundary; and restorative-remedy-not-punishment — which is non-derogable and entrenched (ACT-CONSOLIDATION-FRAMEWORK:s21 and :s25);

**C14 — s15, mirror claim (fatal).** Replace the final paragraph's second sentence with:

> Measured 2026-08-05 and EXECUTED by byte comparison, store by store. `/home/jellytot/Projects/opbox-prod/opbox-kernel/lawpack/v2/statutes/`: all ten identical to canon. `/home/jellytot/Projects/opbox-prod/opbox-kernel-confirmation-fix/lawpack/v2/statutes/`: all ten identical. `/home/jellytot/Projects/opbox-prod/opbox-kernel/.worktrees/tablelist-ext/lawpack/v2/statutes/`: NOT identical — `10-assented-record-protection.yaml` is absent entirely and `03-agent-duties.yaml` is truncated after line 152, omitting the Sovereign-assented ACT-003:s10 and s11. The first draft reported byte-identity across all ten on one store without enumerating the stores, which is the defect s4 and s10 forbid, and the finding is corrected here. The divergent worktree mirror is a matter to be routed, and the property this section preserves is stated as a duty owed, not a state achieved.

**C15 — placement (fatal to the vehicle).** Restructure the second draft as: (a) a short Act retaining s13, s14, the definitions section and the extent clause, plus s11 as cured by C2; and (b) amending schedules inserting the remaining sections into `ACT-002:s6`, `ACT-002:s9`, `ACT-002:s10`, `ACT-003`, `ACT-004:s7`, `ACT-004:s8`, `ACT-004:s9` and `ACT-CONSOLIDATION-FRAMEWORK:s7`, together with amendments to `REG-003` and `REG-COURT-RECORD-001`, each schedule reciting the section amended and the words inserted. Per the PLACEMENT OPINION above. A new Act that should have been five amendments and two regulation amendments is itself a defect, and this one is that Act.

**C16 — every section, address discipline.** Every factual assertion about machinery, a record or the corpus retained in the second draft must carry the address at which it was observed and the mode (EXECUTED or READ), by force of the Act's own s1. The first draft asserts, without any address: "About forty filed orders in the opbox jurisdiction omit `actor`" (s12); "Reservations presently accumulate with no expiry" (s13); "Every material defect found in the opbox governance machinery this week was found either by seeding a violation or by running the instrument" (s14); "68 of 116 orders and 58,034 of 93,759 words" (drafting_note). Supply an address for each or delete it. An Act that enacts a pleading standard and then breaches it in its own recitals will be read down.

**C17 — s10, corrected exemplar.** Replace the exemplar in s10 with:

> On 2026-07-31 a bench recorded [2026] VJS-CC-OPBOX 2 as "NOT FOUND as a primary record ... in any store searched" and expressly declined to rely on it (recited at `.vjs/submissions/filed/SUBMISSION-2026-08-03-citator-corpus.yaml:44-48`). The submission attributes the record to `2026-VJS-CC-OPBOX-CONTROL-BOUNDARY-CONTRAST-002.yaml`, which does not exist in `.vjs/orders/` (EXECUTED: `ls`, and `git log --diff-filter=A`, both empty). The record that does exist is `.vjs/orders/2026-VJS-CC-OPBOX-002.yaml`, `status: binding`, `created_at: 2026-06-11T10:09:30Z`, first committed 2026-06-11 (EXECUTED `git log --diff-filter=A`) — fifty days before the sitting, not six, and the first draft's "created six days earlier" is withdrawn as unsupported at either address. A further fact bears on the exemplar: `[2026] CC-OPBOX 2` in the `.justice/` citator carries status `per-incuriam` (`.justice/INDEX.md`, READ), so a bench declining to rely on a citation in that form may have been right for a reason the first draft did not consider. The point the section makes survives the correction: a bounded search is a claim about where somebody looked, and an unenumerated one cannot be told apart from a fact about what exists — as the first draft's own account of this very incident demonstrates.

---

## PROPOSED DEFINITIONS

Required. Eleven load-bearing terms are used without definition, and three of them already carry a different undefined meaning elsewhere in canon, so the Act as drafted would change the meaning of in-force statutes by implication. Insert as the FIRST section, renumbering the remainder.

```yaml
  - id: ACT-PROCEEDINGS-DISCIPLINE:s0
    title: Definitions
    text: >
      In this Act:

      MACHINERY means code, a gate, a check, an invariant, a hook, a binary, a command, a
      schema, a reader, or any other runtime artefact whose force comes from its function
      rather than from promulgation (ACT-COMPUTER-FIRST-REALM:s11, the governing test being
      the source of the artefact's force). A regulation directing machinery is not itself
      machinery.

      OBSERVED means read at a stated file and line, or executed as a stated command with its
      output recorded. An inference from the name of a field, a flag, a function, a directory
      or a command argument is not an observation. Nor is a bounded search a fact about what
      exists.

      EXECUTED means the artefact was run and its output recorded. READ means the artefact's
      text was inspected without being run. A machinery claim must state which.

      OPERATIVE PARTS, of an order, means its holding, each directive's `must` text, and each
      forbidden clause, and nothing else ([2026] VJS-PC 17 D1). A reference in the issue, vote,
      question, runtime_summary, source, opinion or case_file_digest is recital and is not
      operative ([2026] VJS-PC 17, forbidden). The term "operative surface" is not used in
      this Act.

      LIVE, of an order or instrument, means recorded with `status: in_force` (for an
      instrument) or `status: binding` (for an order), and not superseded, varied away,
      overruled, revoked or spent. Unloadability does not make an instrument other than live
      (s11); it makes its form unresolved.

      IN FORCE has the meaning it bears in ACT-004:s9 and is not extended by this Act.

      ISSUE TAG means the value of an order's `issue` field.

      REVERSIBLE means capable of being undone by a later act of no greater cost, blast radius
      or authority than the act itself. It is not satisfied merely because code can be edited
      back, a commit reverted, or a file restored. This definition governs this Act only; it
      does not alter the meaning of "reversible" in ACT-003:s2 or ACT-004:s7, which remain as
      they stand.

      BLAST RADIUS means the set of actors, records, stores, external systems and third
      parties whose state the act changes or could change, including those reached only on
      failure. An act whose blast radius is not enumerable is not low-risk for the purposes
      of s7.

      TRUST BOUNDARY means any verb rated SENSITIVE or above, any EXTERNAL authentication
      tier, and any token, capability, credential or permission-model change. A matter
      touching a trust boundary is outside s5 and s7 entirely, regardless of reversibility.

      STORE means any location in a jurisdiction capable of holding a governed record or a
      citation, including but not limited to each root declared at
      `crates/vjs-core/src/front_door.rs:84-97` (`lawpack/v2` YAML, `.vjs/orders`,
      `.vjs/court`), any citator outside those roots (in the opbox jurisdiction, `.justice/`),
      and any mirror, worktree or additional checkout of the jurisdiction. An enumeration that
      omits a store has not measured the jurisdiction.

      NEGATIVE CONTROL means a seeded violation, recorded with the command run and its output,
      demonstrating that the gate under test refuses.

      FORWARDING RECORD means an entry causing an amended citation to resolve to its
      replacement and to report that it was forwarded; it is a pointer and not a second
      allocation (s9; ACT-004:s8).
    kernel_effect:
      defines:
        - term.machinery
        - term.observed
        - term.executed
        - term.read
        - term.operative_parts
        - term.live
        - term.issue_tag
        - term.reversible
        - term.blast_radius
        - term.trust_boundary
        - term.store
        - term.negative_control
        - term.forwarding_record
      must_not:
        - reading_the_definition_of_reversible_in_this_act_as_altering_act_003_s2_or_act_004_s7
        - treating_an_enumeration_that_omits_a_store_as_a_measurement_of_the_jurisdiction
```

---

## ONE MATTER OUTSIDE MY PORTFOLIO, FLAGGED

`/home/jellytot/Projects/opbox-prod/opbox-kernel/.vjs/orders/2026-VJS-SC-OPBOX-001.yaml` exists (EXECUTED `ls`). `[2026] VJS-SC 4` forbidden clause (`lawpack/v2/orders/2026-VJS-SC-004.yaml:48`, READ): `a_subscribing_jurisdiction_holding_its_own_supreme_sitting_or_recording_an_apex_judgment_locally_instead_of_referring_up`. Draft s15 recites that this Act "does not disturb ... the singularity of the Supreme Court (VJS-SC 4)". It does not disturb it, but a savings clause reciting a rule already breached in the store it binds should say so rather than imply compliance. Route separately; do not cure here.