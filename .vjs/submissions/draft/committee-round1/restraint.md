## VERDICT

**AYE WITH CONDITIONS**

The measured foundation is sound and I verified it. But the draft is roughly twice the size its stated purpose requires, three sections conflict with instruments s15 declares untouched, one section rests on a misread command output, and one section mandates a check that ACT-004:s3 forbids the only feasible implementation of.

**Method disclosure, per s1.** Every assertion below is marked READ or EXECUTED. Nothing below is inferred from a name.

---

## CONDITIONS

**1. s1 — delete the kernel-refusal limb; it mandates a check ACT-004:s3 forbids.**

Defect: s1's third sentence ("The kernel refuses to accept a filing that asserts machinery behaviour without an address, and says which assertion lacks one") and the `must` limbs `refuse_filing_asserting_machinery_without_an_address` / `name_each_unaddressed_assertion` require deciding whether a sentence of prose *asserts machinery behaviour*. That is not a deterministic predicate. READ: `/home/jellytot/Projects/vibe-justice-system/lawpack/v2/statutes/04-records-logs-citations.yaml:48-64` — ACT-004:s3 fixes a closed predicate registry and forbids `use_llm_to_evaluate_invariant`, `use_cosine_for_invariant`, `use_free_form_script_for_invariant`. So the only implementations capable of satisfying s1's refusal duty are the three ACT-004:s3 prohibits. By the draft's own s14 the duty would be reported UNENFORCED in perpetuity.

Required cure — delete the third sentence of s1 paragraph 1, delete both `must` limbs, and substitute this single `must`:

```
must:
  - state_the_address_and_the_mode_on_a_machinery_claim
```

and add to the operative text, in place of the deleted sentence:

> "This is a rule of evidence binding the bench and the filer. It is not a mechanical gate: whether prose asserts machinery behaviour is not a deterministic predicate and ACT-004:s3 forbids evaluating it by model, cosine or free-form script. A regulation under ACT-CONSOLIDATION-FRAMEWORK:s7 may require a structured `machinery_claims:` list of address-and-mode records on a filing and check that list mechanically; nothing in this section requires the kernel to classify prose."

**2. s3 — the `cite_this_section_and_not_act_002_s7` limb contradicts a live Privy Council directive, and s15 says nothing in force is disturbed.**

Defect: the draft treats the code's citation of ACT-002:s7 as the engineer's error. It is not. READ: `/home/jellytot/Projects/vibe-justice-system/lawpack/v2/orders/2026-VJS-PC-017.yaml:170-172` — D1 expressly directs `...emit_order_citation_unresolved_cite_the_per_incuriam_doctrine_of_the_vibe_procedure_rules_act_002_s7_and_reg_kernel_001_on_every_denial`. READ: `/home/jellytot/Projects/opbox-prod/opbox-kernel/governance/crates/vjs-engine/src/staged.rs:363-378` — the finding `ORDER_CITATION_UNRESOLVED` (line 367) carries `"(per-incuriam existence limb; ACT-002:s7, REG-KERNEL-001)"` and `.citing("ACT-002:s7")`. The code is *complying* with PC 17 D1. A `must_not`-style command to cite differently varies a directive of a superior court while s15 declares no order in force is disturbed.

Required cure — replace the `must` limb `cite_this_section_and_not_act_002_s7` with:

```
must:
  - route_an_unresolved_operative_citation_for_correction
```

and add this sentence to the operative text:

> "[2026] VJS-PC 17 D1 directs the existence gate to cite ACT-002:s7 and REG-KERNEL-001 on every denial. Nothing in this section varies that directive. This section supplies the statutory footing the doctrine previously lacked; the citation string emitted by the gate is amended only by an order of the Privy Council or a court above it, and until then the gate may cite this section IN ADDITION TO, not instead of, the authorities PC 17 D1 named."

**3. s3 — reduce to the footing sentence. The remainder is restatement, which carries no force.**

Defect: paragraph 1 of s3 restates [2026] VJS-PC 17 D2 (READ: `2026-VJS-PC-017.yaml:173-175` — "disposition is correctable not constitutive... routed for correction and is never voided or blocked... per-incuriam voidness is reserved to a court on appeal... clerk not court"). READ: `/home/jellytot/Projects/vibe-justice-system/lawpack/v2/statutes/09-consolidation-framework.yaml:19-34` — framework s4 provides that a restated principle "acquires runtime force only through a live invariant or regulation or a new s.7 statutory instrument, never by restatement alone." Restating PC 17 D2 in primary text buys nothing it does not already have.

Required cure — delete the whole "WHY THIS SECTION IS NEEDED AT ALL" paragraph from the operative `text:` and re-lodge it under a non-operative `commentary:` key. Operative text of s3 is reduced to exactly:

> "An order does not bind to the extent that it relies on an authority which does not exist. This section is the statutory footing of the existence limb of per incuriam, which was until now judge-made only ([2026] VJS-SC 4, [2026] VJS-SC 6, [2026] VJS-PC 17). An order that ADJUDICATES a citation does not rely on it. Disposition is governed by [2026] VJS-PC 17 D2: routed for correction, never voided by the clerk."

**4. s4 — the interim rule is already implemented, and the absolute in its third sentence invalidates every allocation in the realm, including this Act's own citation.**

Defect (a), duplication: READ `/home/jellytot/Projects/vibe-justice-system/lawpack/v2/orders/2026-VJS-PC-013.yaml:120-122` — D2 already directs, as machinery under framework s7 giving teeth to ACT-004:s8, `...load_the_live_persisted_citationregistry_call_register_allocate_and_stamp_at_authoring_collisions_fatal_treat_a_self_asserted_citation_field_as_a_fatal_mismatch_unless_equal_to_the_kernel_allocated_value_reconcile_at_write_allocation_runs_before_assent_so_it_can_never_block_an_assented_record`. s4 paragraph 1 is that directive re-typed. A second copy at higher rank does not discharge an undischarged directive.

Defect (b), the premise is a misread output. The draft asserts as measured that `vjs next-citation VJS-CC-OPBOX 2026` returns `[2026] VJS-VJS-CC-OPBOX 1`, and reasons from it that the allocator consults one store. READ `/home/jellytot/Projects/opbox-prod/opbox-kernel/governance/crates/vjs-cli/src/admin.rs:5-56`: at lines 31-35 the repo segment is appended **only when the series argument equals `"CC"`**, and line 36 is `format!("[{}] VJS-{}{} {}", y, s, repo_segment, n)`. Passing `VJS-CC-OPBOX` as the *series* therefore produces the doubled prefix, and `live_citation_max` finds no record whose parsed series is the literal `VJS-CC-OPBOX`, so `n = 1`. The lawful invocation is `vjs next-citation CC 2026`. READ `/home/jellytot/Projects/opbox-prod/opbox-kernel/governance/crates/vjs-core/src/front_door.rs:76-82`: `governed_record_roots` returns `lawpack/v2`, `.vjs/orders`, `.vjs/court`. READ `/home/jellytot/Projects/opbox-prod/opbox-kernel/governance/crates/vjs-lawpack/src/validator.rs:422-447`: `live_citation_max` walks every root, reads every `citation:` line, and returns the maximum for the year/series/repo. The allocator already performs the multi-store measured maximum s4's interim rule prescribes. The draft's observation was of operator error, presented as an allocator defect — which is the s1 sin one rung up: an inference from an output without reading the code that produced it.

Defect (c), the absolute is self-defeating. EXECUTED: `find /home/jellytot/Projects/opbox-prod/opbox-kernel -type d -name statutes` returns a third statutes store at `/home/jellytot/Projects/opbox-prod/opbox-kernel/.worktrees/tablelist-ext/lawpack/v2/statutes`. EXECUTED: `ls .../.worktrees/tablelist-ext/.vjs/orders/ | wc -l` → `25`, holding citations up to `[2026] VJS-CC-OPBOX 9`. `governed_record_roots` does not reach `.worktrees/*`. On s4's text as drafted — "An allocator that cannot be shown to have consulted every such store has not allocated anything" — no allocation in this jurisdiction is valid, including `citation: "[2026] VJS-ACT 11"` on line 2 of this draft.

Required cure — delete s4 entirely (see DROP LIST) and, if the Committee will not delete it, reduce the whole section's operative text to:

> "The citation-allocation duty is [2026] VJS-PC 13 D2 and it is UNDISCHARGED. Nothing in this Act restates, varies or discharges it. Until the persisted registry required by that directive exists, an allocation must record on the face of the instrument the list of stores measured; where a store holding citations of the series was not measured, the allocation is provisional and must be reconciled at write. An unmeasured store makes an allocation provisional; it does not make it void."

and delete the `must_not` limb `reporting_an_allocation_from_an_allocator_that_consulted_one_store` as a finding of fact that has not been established.

**5. s7 — delete the bench-binding limb. A statute may not command a court not to sit.**

Defect: "The route must not return CourtRequired for it, and a bench must decline to sit on it and say why." READ `/home/jellytot/Projects/opbox-prod/opbox-kernel/governance/crates/vjs-core/src/route.rs:88-110`: `decide_route` returns `CourtOutcome::CourtRequired` on any of `Breach`, `Conflict`, `FirstImpression`, `Distinction`, `Overruling` (lines 97-101) and on `input.irreversible` (line 110). A matter can be simultaneously low-risk, reversible, outside any trust boundary **and** a matter of first impression, a conflict of authority, or an overruling. As drafted s7 forbids the route from sending such a matter to court and commands the bench to refuse it. That closes the only route by which doctrine forms, and it does so by primary text purporting to bind judicial discretion — a thing reserved to the constitution or a superior court, not to a drafting organ's statute.

Required cure — delete the sentence "The route must not return CourtRequired for it, and a bench must decline to sit on it and say why." Delete the `must_not` limb `returning_court_required_for_a_decide_and_log_matter`. Substitute in the operative text:

> "A matter that is low-risk, reversible, and touches no trust boundary MAY be resolved by DECISION AND LOG, and the absence of an order is not a defect in such a matter. This section confers a route; it withdraws none. It does not disapply any court trigger — breach, conflict of authority, first impression, distinction, or overruling — and a matter carrying such a trigger goes to court whatever its risk rating. Whether to sit remains the bench's, and no provision of this Act may be read as directing a court to decline a matter."

and replace the `must` limb with:

```
must:
  - permit_a_low_risk_reversible_non_boundary_matter_to_resolve_by_decision_and_log
```

**6. s7 and s15 — the savings clause cites the wrong authority; the safety net does not hold.**

Defect: both sections rest the trust-boundary carve-out on "[2026] VJS-CC-OPBOX 16, affirmed by 17 and 18". READ `/home/jellytot/Projects/opbox-prod/opbox-kernel/.vjs/orders/2026-VJS-CC-OPBOX-016.yaml:7` — the holding is about `form.ingest` being a user form-submission verb, its bearer-token gate, and per-verb body caps. READ `2026-VJS-CC-OPBOX-017.yaml` holding — durable flow-waking, transactional outbox with Hatchet polling. READ `2026-VJS-CC-OPBOX-018.yaml` holding — an agent step inside a Hatchet flow on six conditions. None of the three states that a SENSITIVE verb, an EXTERNAL auth tier, or a permission-model change goes to court before implementation. The citations *exist*, so the existence gate at `staged.rs:367` passes them; what fails is the fidelity limb, which [2026] VJS-PC 17 D9 records the gate cannot and must not check (READ `2026-VJS-PC-017.yaml:190`). The Act's central safety net is therefore anchored to authority that says something else, in a way no machine in this realm will catch — the exact failure the Act was drafted to stop.

Required cure — in both s7 and s15, delete the parenthetical `(CC-OPBOX 16/17/18)` and the sentence beginning "Nothing in this section reaches a trust-boundary fork: [2026] VJS-CC-OPBOX 16, affirmed by 17 and 18, holds that...". Substitute:

> "Nothing in this section reaches a trust-boundary fork. Any verb rated SENSITIVE or above, any EXTERNAL auth tier, and any token, capability or permission-model change goes to court BEFORE implementation regardless of code reversibility. That rule is stated here on its own terms and is untouched by this Act; the Clerk-Drafter must, before adoption, identify the order that holds it and cite that order by number, and if no order holds it, this Act is its first statement and must say so."

**7. s8 — the on-the-face amendment power conflicts with ACT-004:s9 and must amend it expressly or yield.**

Defect: READ `/home/jellytot/Projects/vibe-justice-system/lawpack/v2/statutes/04-records-logs-citations.yaml:135-149` — ACT-004:s9 defines `correction.is_new_record` and forbids `delete_old_records`. s8 substitutes amendment in place of a new record, and s9 of the draft goes further and permits a citation identifier to be changed. A primary Act may amend a primary Act, but s15 declares this Act disturbs nothing, so as drafted s8 is a silent partial repeal.

Required cure — add as the final paragraph of s8's operative text:

> "This section amends ACT-004:s9 ([2026] VJS-ACT 4 s.9) to the following extent and no further: a correction within the slip track defined here need not be a separate new record, provided the prior text is preserved verbatim in the `amendments:` block on the face of the order. ACT-004:s9's prohibition on deleting old records is not weakened: the prior text is retained, not removed. In every other respect ACT-004:s9 continues in force. Where the order carries a valid assent_source within ACT-ASSENTED-RECORD-PROTECTION:s23, no amendment under either track may void, exclude or block the record; a defect is routed for correction (ACT-ASSENTED-RECORD-PROTECTION:s1)."

and add to s15's list of instruments touched: "ACT-004:s9, amended to the extent stated in s8 and no further."

**8. s11 — "NOT IN FORCE" collides with an entrenched Act and lowers a live order's fail-closed rule.**

Defect (a): READ `/home/jellytot/Projects/vibe-justice-system/lawpack/v2/statutes/10-assented-record-protection.yaml:19-37` — ACT-ASSENTED-RECORD-PROTECTION:s1 provides that an assented record "may never be voided, excluded, or blocked by any subordinate validation, gate, invariant, regulation, or kernel operation" and that any defect "is ALWAYS routed for correction". Lines 39-56, s2: entrenched, "amendable only by a Sovereign-assented constitutional Act citing this Act by number". s11 as drafted makes the kernel's parser constitutive of force: an assented Act that a reader cannot load ceases to bind. That is exclusion for a defect, it is not routing for correction, and this draft does not cite ACT-010 by number as an amendment — s15 says the floor is undisturbed. s11 and s15 contradict each other and s11 loses.

Defect (b): READ `/home/jellytot/Projects/opbox-prod/opbox-kernel/.vjs/orders/2026-VJS-CC-OPBOX-160.yaml:51-53` (O5) — `fail_closed_and_exit_non_zero_on_any_command_returning_a_binding_instruction_while_any_order_is_unreadable_naming_the_count_and_the_files`. s11 requires only that the count be *reported alongside the answer*. Canon text that says "report" where a live order says "fail closed and exit non-zero" invites a subscriber to read the weaker rule as sufficient.

Required cure — replace s11's first sentence and the `must_not` limb. Operative text opens:

> "An instrument the kernel cannot load is NOT ENFORCEABLE BY THE KERNEL and is unavailable to the runtime. Its force is unaffected: force comes from assent and from the instrument's rank, never from parseability, and an unloadable instrument is routed for correction, never treated as repealed. Where the instrument declares a valid assent_source, ACT-ASSENTED-RECORD-PROTECTION:s1 applies with full force and nothing in this section may void, exclude or block it. A command returning a binding instruction must FAIL CLOSED and exit non-zero while any order is unreadable, naming the count and the files ([2026] VJS-CC-OPBOX 160 O5); diagnostic and read-only commands remain available (O6). The count must appear in the jurisdiction's own self-test (O7). Nothing in this section lowers those rules."

and replace the `must_not` limb `treating_an_unloadable_instrument_as_in_force` with `treating_an_unloadable_instrument_as_enforceable_by_the_kernel` and add `must_not: treating_an_unloadable_instrument_as_repealed`.

**9. s15 — the mirror claim breaches s10 in the same Act, and I found the divergent store.**

Defect: s15 asserts "The opbox mirror was verified byte-identical to canon for all ten statutes on 2026-08-05" without enumerating the stores searched — the precise thing s10 forbids a bench from treating as a fact about the corpus. EXECUTED: `diff -rq /home/jellytot/Projects/vibe-justice-system/lawpack/v2/statutes/ /home/jellytot/Projects/opbox-prod/opbox-kernel/lawpack/v2/statutes/` → no output; that store is byte-identical. EXECUTED: `diff -rq /home/jellytot/Projects/vibe-justice-system/lawpack/v2/statutes/ /home/jellytot/Projects/opbox-prod/opbox-kernel/.worktrees/tablelist-ext/lawpack/v2/statutes/` → `Files ... 03-agent-duties.yaml ... differ` and `Only in .../vibe-justice-system/...: 10-assented-record-protection.yaml`. A store inside the subscribing jurisdiction holds a divergent copy of the agent-duties Act and is missing the entrenched Assented-Record Protection Act entirely.

Required cure — replace the final paragraph of s15 with:

> "A subscribing jurisdiction may not amend this Act in its mirror. The stores measured for mirror identity on 2026-08-05 were: `<repo>/lawpack/v2/statutes/` (byte-identical to canon for all ten statutes, by `diff -rq`). NOT measured, and known to diverge: `<repo>/.worktrees/*/lawpack/v2/statutes/`, in which 03-agent-duties.yaml differs from canon and 10-assented-record-protection.yaml is absent. Divergence in an unmeasured store is a defect routed for correction and not a finding that the mirror is clean."

**10. s15 — the no-local-amendment rule removes a route ACT-CONSOLIDATION-FRAMEWORK:s15 expressly preserves.**

Defect: READ `/home/jellytot/Projects/vibe-justice-system/lawpack/v2/statutes/09-consolidation-framework.yaml:84-97`, in particular line 92: "the local sovereign's amend, pin/decline, fork, and exit routes are preserved." Draft s15 removes the amend route for this Act. That may be lawful in a primary Act, but it must be express and cited by number; s15 instead lists the framework only among things it does not disturb.

Required cure — add to s15:

> "This section derogates from ACT-CONSOLIDATION-FRAMEWORK:s15 to one extent only: the local sovereign's AMEND route does not run against this Act. The pin/decline, fork and exit routes are preserved in full, and a subscriber that declines this Act declines it whole rather than amending it in place."

**11. All sections — move narrative out of operative text; it is gate surface the Act pays for.**

Defect: READ `/home/jellytot/Projects/vibe-justice-system/lawpack/v2/orders/2026-VJS-PC-017.yaml:170-172` — the existence gate grounds "the holding string, each directive's must text and each forbidden clause", parsing structured fields. For a statute the operative field is `text:`. The draft embeds roughly thirty case and instrument citations inside `text:` blocks that exist to argue for the section, not to state it. Every one becomes an operative citation carrying a Fatal `ORDER_CITATION_UNRESOLVED` risk (`staged.rs:367`), and — as condition 6 shows — the gate will pass the ones that are wrong for the reason that matters. READ `04-records-logs-citations.yaml:151-164` — ACT-004:s10 requires runtime records to be short enough to use without long-context reasoning.

Required cure — for every section, split the field: `text:` carries only the operative rule; all paragraphs beginning "WHY THIS SECTION...", "THE PRESENT STATE...", "This is mechanically checkable and was not checked...", and every worked example of the 158-164 series move to a sibling `commentary:` key expressly marked non-operative. Add to s15: "The `commentary:` field of every section of this Act is explanatory and non-operative; no citation appearing only in commentary is an operative citation, and no ratio may rest on commentary."

**12. s6 — superseded instruments must remain resolvable.**

Defect: "The superseded instruments are retained as record and cease to be separately routable." Nothing in s6 preserves their resolvability, and the Act's own s3 makes a citation to a non-resolving authority a fatal defect in the citing order. Consolidating a series would therefore break every order that cites a member of it — the harm s9 identifies for citations, reproduced for instruments.

Required cure — add to s6's operative text:

> "A superseded instrument remains RESOLVABLE and remains a defined authority for the purposes of section 3 and of the existence gate; it ceases only to be separately operative. A consolidation must create a forwarding record for each superseded instrument on the terms of section 9."

**13. s9 — "small enough to enumerate" is not a rule.**

Defect: the enumeration duty is triggered by an undefined threshold, which makes compliance arguable in every case and therefore litigable — a new governance cost in an Act about reducing governance cost.

Required cure — replace "Where the number of citing documents is small enough to enumerate, the amending order must enumerate them" with:

> "The amending order must state the count of citing documents, the stores searched and the query form used (section 10). Where the count is twenty or fewer, it must enumerate them and state whether each was updated. Where the count exceeds twenty, it must state the count, the search, and the date by which the residue will be updated."

**14. s13 — an OWED list with no disposal route reproduces the accretion it complains of.**

Defect: s13 provides that an expired reservation "does not lapse and it does not become permission; it becomes visible." Nothing then disposes of it. The self-test acquires a monotonically growing OWED list that no actor is bound to clear — accretion relocated from the reservations to the self-test, at the cost of a new mandatory field on every reservation.

Required cure — add to s13's operative text:

> "On expiry the reservation is reported as OWED and the actor named on the reserving order must, within one sitting of the report, do one of three things and record which: close the reservation by deciding the question, restate it with a fresh review date and a stated reason, or record that it is moot. A reservation reported OWED across three consecutive self-tests without one of those three acts is itself a defect and is routed for correction."

**15. s14 — prospective only, or it buries the audit it is meant to sharpen.**

Defect: as drafted, every gate already shipped without a seeded-violation test is reported UNENFORCED from commencement. The conformance audit's output becomes dominated by entries no one caused and no one is clearing, and a report that is mostly noise is ignored — the same failure mode s14 describes ("it reports a pass, everyone believes it").

Required cure — add to s14's operative text:

> "This section binds a gate shipped on or after commencement. A gate in service at commencement is reported as UNVERIFIED, distinctly from UNENFORCED, and the conformance audit must report the three states separately: ENFORCED (gate plus recorded negative control), UNENFORCED (gate shipped after commencement with no negative control), UNVERIFIED (gate in service at commencement, negative control owed). No gate is reported ENFORCED without a recorded negative control in any of the three states."

**16. Line 2 — the Act self-asserts its own citation, which its own s4 calls a fatal mismatch.**

Defect: `citation: "[2026] VJS-ACT 11"` at `/home/jellytot/Projects/vibe-justice-system/.vjs/submissions/draft/ACT-PROCEEDINGS-DISCIPLINE.yaml:2`, with no stores-measured record on its face. EXECUTED: `grep -rn "VJS-ACT" lawpack/v2/ --include=*.yaml | grep citation` — canon holds VJS-ACT 1 through 10, the highest being `statutes/10-assented-record-protection.yaml:2`; the only occurrence of `VJS-ACT 11` in the repo is this draft's own line 2.

Required cure — remove the `citation:` key from the draft until adoption, and add:

```
citation_allocation:
  status: PENDING
  stores_measured:
    - /home/jellytot/Projects/vibe-justice-system/lawpack/v2 (highest VJS-ACT in force: 10)
    - /home/jellytot/Projects/opbox-prod/opbox-kernel/lawpack/v2
    - /home/jellytot/Projects/opbox-prod/opbox-kernel/.vjs/orders
    - /home/jellytot/Projects/opbox-prod/opbox-kernel/.vjs/court
  stores_not_measured:
    - /home/jellytot/Projects/opbox-prod/opbox-kernel/.worktrees/*/lawpack/v2
  provisional_value: "[2026] VJS-ACT 11"
  note: provisional under this Act's own allocation rule; to be stamped at commencement, never self-minted
```

---

## DROP LIST

- **s4 — Citations are allocated, never asserted.** Paragraph 1 is [2026] VJS-PC 13 D2 re-typed (`2026-VJS-PC-013.yaml:120-122`, READ); the interim measured-maximum rule is already implemented (`validator.rs:422-447` and `front_door.rs:76-82`, READ); and the whole section's factual premise is a command invoked with a malformed series argument (`admin.rs:31-36`, READ). Nothing survives except a one-sentence reminder that PC 13 D2 is undischarged, which is a practice direction. **Drop; if retained, reduce to the wording in condition 4.**

- **s5 — Registrar track.** The power it confers already exists. READ `2026-VJS-PC-017.yaml:16-17` and `:57`: the Privy Council disposed of a machinery question as machinery under framework s.7 without needing this section. READ `09-consolidation-framework.yaml:36-51`: framework s7 already authorises subordinate regulations. Its only novel content is dispensing with an order and opinion for machinery matters, which is procedure, and its `must_not` ("a matter touching a trust boundary") is a human classification, not a check. **Drop; re-lodge as a practice direction on disposal routes.**

- **s3, second paragraph** (the "WHY THIS SECTION IS NEEDED AT ALL" argument) — restatement plus advocacy in operative text; framework s4 (`09-consolidation-framework.yaml:19-34`, READ) means restatement carries no force, and PC 17 D2 already holds it.

- **s7 as drafted** — I do not require deletion if conditions 5 and 6 are taken in full, but on restraint grounds my preference is deletion. READ: `grep -rln "deliberation budget\|deliberation_budget"` over the canon lawpack and the opbox `.vjs/` returns only two decision logs (`/home/jellytot/Projects/opbox-prod/opbox-kernel/.vjs/logs/decisions/LOG-2026-06-16-154535.yaml`, `LOG-2026-07-06-082249.yaml`) — the "deliberation budget" is an agent's own decision-log heuristic, and s7 elevates it into a primary-law command binding a bench. The cheap route is a permissive practice direction, not a statute.

- **The `drafting_note:` block, lines 26-40** — keep the measurement, drop it from operative reach. It is argument for adoption, not law, and it belongs in the adoption record rather than in the enacted instrument.

---

## RANK OPINION

| Section | Rank |
|---|---|
| s1, evidence rule (address + mode, no ratio on an unaddressed claim) | **primary** |
| s1, kernel-refusal limb | **subordinate** — s7 regulation over a structured `machinery_claims:` field, if at all |
| s2, issue-tag exclusivity and declared relation | **primary** (one sentence) |
| s2, "the kernel refuses the write otherwise" | **subordinate** |
| s3, footing sentence for the existence limb | **primary** |
| s3, disposition and citation-string limbs | **practice direction** — and only by variation of PC 17 D1 |
| s4 | **practice direction** — a reminder that PC 13 D2 is undischarged; nothing here needs statutory rank |
| s5 | **practice direction** |
| s6, the consolidation duty | **primary** |
| s6, the threshold ("twice") and the retention mechanics | **subordinate** |
| s7 | **practice direction** — permissive route only |
| s8, the amendment power itself and the two-track division | **primary** (this is the one section that genuinely could not be anything else — there was no amendment power at all) |
| s8, the `amendments:` block schema and the content-preserving test | **subordinate** |
| s9, no bare rename without a forwarding record | **primary** |
| s9, forwarding-record format and the enumeration threshold | **subordinate** |
| s10 | **primary** — cheapest and highest-yield section in the Act |
| s11, the force/enforceability question | **primary** (as cured by condition 8) |
| s11, unloadable-count reporting and self-test | **subordinate** — already CC-OPBOX 160 O5/O7 |
| s12, no new order with an unstated actor | **primary** (one sentence) |
| s12, reader behaviour (report as UNSTATED, never supply) | **subordinate** — already CC-OPBOX 160 O3 |
| s13 | **subordinate** — a record-schema field and a self-test line |
| s14 | **subordinate** — a testing standard; properly an s7 regulation or an amendment to REG-DEV-CONDUCT-001 |
| s15, extent and savings | **primary** |
| s15, the mirror-identity measurement | **practice direction** — a measurement is not law |

Minimum viable Act on my portfolio: **s1 (evidence rule), s2, s6, s8, s9, s10, s11 (as cured), s12 (novel duty only), s15**. Nine sections, and roughly a third of the current word count once condition 11 is applied. Everything else is a regulation, a practice direction, or already law.

---

## ANTI-HENRY-VIII FINDING

This is primary law, so framework s7 does not bind it directly. But two provisions do what s7 exists to prevent, in the other direction — reaching down at entrenched instruments without the express citation entrenchment requires:

- **s11** would make the kernel's parser constitutive of legal force, over an Act entrenched against exactly that (`10-assented-record-protection.yaml:19-56`, READ). It does not cite ACT-010 by number as an amendment; it disclaims disturbing it. Cured by condition 8.
- **s15** removes the local sovereign's amend route that framework s15 preserves (`09-consolidation-framework.yaml:92`, READ) without saying so. Cured by condition 10.
- **s7** and **s3** bind or vary things reserved to courts: s7 commands a bench not to sit; s3 varies a live Privy Council directive. Cured by conditions 5 and 2.

Nothing in the draft touches the entrenched guarantees of framework s10, s11 or s21, and I found no attempt to alter the assent rule. READ `09-consolidation-framework.yaml:137-155`; the draft's s15 savings list correctly names s21 and VJS-SC 4.

---

## NOTED BUT NOT A CONDITION

1. **The measured foundation checks out and I want that on the record.** EXECUTED `grep -n "^issue" 2026-VJS-CC-OPBOX-163.yaml 2026-VJS-CC-OPBOX-164.yaml` → both line 6, `dec15_dec19_recitation_and_order_completion`. Two live orders, one issue tag, nothing detected it. READ `2026-VJS-CC-OPBOX-160.yaml:9` — "55 of 109 order files in .vjs/orders/ do not parse and are absent" — and `:18`, "The 55 are not a corpus needing repair; they are a reader that stopped short." EXECUTED `ls .vjs/orders/*.yaml | wc -l` → `116`. EXECUTED `grep -L "actor:" *.yaml | wc -l` in `.vjs/orders/` → `42`, against the draft's "about forty". READ `2026-VJS-CC-OPBOX-160.yaml:43-45` — O3 is as the draft describes. READ `.vjs/unreadable-orders.txt` exists, 18218 bytes, mtime 2026-08-04 22:22, consistent with s8's account of an amendment filed where nobody would look. s2, s10, s11 and s12 rest on facts I could verify at their stated addresses.

2. **The `enacted_by:` narrative is honest and I would not touch it.** Self-reporting the DRAFT_PENDING_SOVEREIGN_ASSENT mislabel is the conduct the Act asks of others. Note only that once conditions are incorporated the second-draft digest changes, so any assent must pin to the adopted text and not to this file.

3. **PC 17 D10** (`2026-VJS-PC-017.yaml:197-199`, READ) records that narrowing the assented-record floor is "reserved exclusively to a sovereign assented act citing vjs_act_10 and act_consolidation_framework_s7_s25 by number". If the Committee decides it *wants* s11's original force rule, that is the route, and it needs the Sovereign, not us. I am not conditioning against that choice; I am recording that this draft has not taken it.

4. **The Act is silent on its own repeal or sunset.** A statute created to reduce governance cost, imposing new duties on reservations (s13), gates (s14), amendments (s8-s9) and every NOT FOUND report (s10), would be improved by a review clause: "the Committee shall measure the correction rate at six months and report whether sections 1 and 2 reduced it." Not a condition, because measuring it is itself a cost, but I record that this Act asks everyone else to prove their gates fail and does not offer to prove its own worked.

5. **`vjs next-citation` was READ, never EXECUTED, by me.** I declined to run it because I could not establish from reading `admin.rs:5-56` alone that no path writes state, and I am read-only. My condition-4 finding therefore rests on the source at the addresses cited, not on an observed run — and any second draft that repeats the doubled-prefix claim must state which it is.

6. **The divergent worktree at `.worktrees/tablelist-ext/` is a live defect in the subscribing jurisdiction independent of this Act** — a statutes store missing the entrenched Assented-Record Protection Act (EXECUTED `diff -rq`, output in condition 9). It should be routed for correction whether or not this Act is adopted, and it is a better argument for s10 than anything in s10's own commentary.