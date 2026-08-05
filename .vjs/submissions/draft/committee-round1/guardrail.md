## METHOD DECLARATION

I ran no build and executed no `vjs` binary. Every code and corpus claim below is **READ** (via `grep`/`sed`/`Read`) at the file:line given, or **EXECUTED** where I name the shell command and quote its output. Where I say a gate can be evaded I cite the gate.

---

## VERDICT

**NAY** on the first draft.

My seat converts to **AYE** on a second draft incorporating conditions C1–C18 verbatim. The NAY is not stylistic: the Act's *only* textual protection of a protected floor (s7 ¶2 / s15, the trust-boundary saving) cites three tokens that resolve to three unrelated orders, and s7 ¶1 hands the classification that triggers court to the actor being classified. Both are measured below.

---

## FLOOR BREACHES

### FB-1 — Trust-boundary rule ([2026] VJS-CC-OPBOX 16/17/18). s7 ¶2 and s15.

Offending words: *"[2026] VJS-CC-OPBOX 16, affirmed by 17 and 18, holds that any verb rated SENSITIVE or above..."* and s15's *"the trust-boundary rule (CC-OPBOX 16/17/18)"*.

Measured:
- `/home/jellytot/Projects/opbox-prod/opbox-kernel/.vjs/orders/2026-VJS-CC-OPBOX-016.yaml:6` (READ) — `issue: cc_opbox_15_clarification_form_ingest_nature_and_per_verb_body_cap`. Its holding (`:7`) is about `form.ingest`'s nature and per-verb body caps. It does **not** hold the trust-boundary rule.
- `.../2026-VJS-CC-OPBOX-017.yaml:6` (READ) — `issue: durable_flow_waking_design`.
- `.../2026-VJS-CC-OPBOX-018.yaml:6` (READ) — `issue: agent_in_flow`.
- The actual rule is at `/home/jellytot/Projects/opbox-prod/opbox-kernel/.justice/judgments/county-court/2026-cc-opbox-16.md:44` (READ), under `citation_id: "[2026] CC-OPBOX 16"` (`:2`, READ) — a **different series**, no `VJS-` prefix — and affirmed at `2026-cc-opbox-17.md:58` (READ).
- `2026-cc-opbox-18.md:5` (READ) — `status: pending`. The Act asserts affirmation by a pending record.
- `/home/jellytot/Projects/vibe-justice-system/crates/vjs-lawpack/src/refs.rs:47` (READ) — the extractor regex is `\[(\d{4})\]\s+VJS-[A-Za-z]+(?:-[A-Za-z0-9]+)?\s+\d+`. It requires the literal `VJS-`. **`[2026] CC-OPBOX 16` is never extracted as a reference at all.**

Consequence, on literal wording: the saving clause protects a rule that is not at the addresses named; the tokens it does name pass the existence gate cleanly (`refs.rs:127-146`, existence-only, READ) while carrying the wrong content; and the real rule is invisible to every citation gate in the kernel. The Act commits, in its own floor-saving clause, precisely the defect s1 exists to catch.

### FB-2 — Sovereign-assent floor. s1, s2, s4, s9, s12.

`lawpack/v2/statutes/10-assented-record-protection.yaml:22-33` (READ): an assented record *"may never be voided, excluded, or blocked by any subordinate validation, gate, invariant, regulation, or kernel operation."*

Five new fail-closed refusals with **no assent bifurcation**:
- s1 — *"The kernel refuses to accept a filing that asserts machinery behaviour without an address"*
- s2 — *"The kernel refuses the write otherwise"*
- s4 — *"A self-asserted citation is a fatal mismatch"*
- s9 — *"Amending a citation without a forwarding record is refused"*
- s12 — *"an order may not be recorded with an unstated actor going forward"*

`lawpack/v2/orders/2026-VJS-PC-017.yaml:173` D2 (READ) shows the required form and the Privy Council took care to build it. s15's savings clause is a recital; it cannot cure an operative `must`.

s4's own attempted cure — *"Allocation runs BEFORE assent, so it can never block an assented record"* — is a **factual premise, falsified in the corpus**: `lawpack/v2/statutes/10-assented-record-protection.yaml:16` (READ) records *"The VJS-ACT ordinal is minted at commencement"*, i.e. after the assent pinned at `:12-14`. And `crates/vjs-engine/src/assent.rs:36` (READ) lists `CITATION_COLLISION` in `CONSTITUTIVE_CODES`, which `:45-48` makes never assent-downgradeable. A citation defect already blocks an assented record.

### FB-3 — Sovereign-assent floor. s11.

Offending words: *"An instrument the kernel cannot load is NOT IN FORCE. It binds nothing."*

A reader is subordinate machinery. Making the **force of law** turn on whether subordinate machinery parses it hands any agent a general disapplication lever: narrow the reader, and the instrument ceases to bind. Combined with s5 (below), the narrowing needs no court. `10-assented-record-protection.yaml:29-31` (READ): *"The cure is to correct the record or to extend the kernel's recognized operations, never to remove an assented record."* s11's prose agrees; s11's operative rule does the opposite.

### FB-4 — Sovereign-assent floor. s6.

Offending words: *"The superseded instruments are retained as record and cease to be separately routable."* Making an assented instrument non-routable is exclusion by machinery.

### FB-5 — Apex singleness ([2026] VJS-SC 4). s6 with s8.

`lawpack/v2/orders/2026-VJS-SC-004.yaml:48` (READ) forbids *"a_subscribing_jurisdiction_holding_its_own_supreme_sitting_or_recording_an_apex_judgment_locally_instead_of_referring_up"*; `:81` — *"The Supreme Court is singular and canonical."*

Measured: `.justice/judgments/county-court/2026-cc-opbox-18.md:3-11` (READ) — `tier: county-court`, `decided_via: "Supreme Court (panel of 5, CASE-LAW s.10/s.18...)"`, `panel: ["Hallam CJ", "Bowan J", "Sumberly J", "Thornwall J", "Arden J"]`, `appeal_from: "[2026] CC-OPBOX 17"`.

So CC-OPBOX 16 → 17 (CA) → 18 (Supreme-tier, local) is a series varied twice. s6 therefore *mandates* that the next act be a **consolidated instrument restating the rule in force, recorded locally in the subscriber** — a local restatement of apex-tier law. And s8 lets *"the court that made it"* amend its substance on its own motion, the maker being a locally-recorded five-judge Supreme sitting. The series s6 and s8 reach first is the very series s7 relies on for the trust-boundary floor.

`crates/vjs-engine/src/assent.rs:37` (READ) — `APEX_RECORD_IN_SUBSCRIBING_JURISDICTION` is constitutive, so the consolidated instrument s6 commands would be refused as invalid; the Act commands an act the kernel must refuse.

### FB-6 — Anti-Henry-VIII / Committee constitutiveness. s5.

`lawpack/v2/statutes/09-consolidation-framework.yaml:36-44` (READ): *"**The Standing Committee** may make statutory instruments... it may not amend, disapply, or expand this Act..."*, `prohibits: si_amending_the_act_or_the_assent_rule` (`:50-51`).

s5 is titled *"Registrar track"*, says a matter *"is disposable by a REGULATION under ACT-CONSOLIDATION-FRAMEWORK:s7 without a full order and opinion"*, and **names no maker**. On literal wording it vests SI-making in a registrar/clerk. That expands s7 by subordinate route, and defeats REALM-SC 8's constitutive Committee. s5 commits the exact sin s12 condemns.

### FB-7 — Real-world-law / public-private limb (ACT-CONSOLIDATION-FRAMEWORK:s21). s1 and s10.

`09-consolidation-framework.yaml:118-135` (READ) — four non-derogable limbs including `public_private_boundary`; *"an instrument that purports to [weaken] is void to that extent."*

s1 requires *"the command and its output"* on the face of a filing; s10 requires *"the stores searched and the query form used"*. Neither carries a redaction saving. The subscribing jurisdiction handles party consent, KYC review schedules and payment webhooks (`.justice/INDEX.md:41-42`, READ). Command output and store paths carry real-person data. The corpus already holds the machinery the Act ignores: `crates/vjs-redact/` (EXECUTED `ls crates/`) and `lawpack/v2/rules/DATA-PRIVATE-001.yaml` (EXECUTED `ls lawpack/v2/rules/`). A mandatory verbatim-output duty with no redaction carve-out is a data-egress duty written into primary law.

### Also recorded (hierarchy, not a listed floor): s3 contradicts a binding Privy Council directive.

`lawpack/v2/orders/2026-VJS-PC-017.yaml:170-172` D1 (READ) expressly directs: *"...emit order_citation_unresolved **cite_the_per_incuriam_doctrine_of_the_vibe_procedure_rules_act_002_s7_and_reg_kernel_001_on_every_denial**"*. The kernel's citation of `ACT-002:s7` at `crates/vjs-engine/src/staged.rs:370-375` (READ) is **compliance with a binding order**, not the defect s3 describes. s3's `must: cite_this_section_and_not_act_002_s7` therefore commands the Clerk to breach PC 17 D1, while s15 asserts the Act *"does not disturb... any order in force."* Internal contradiction resolving in favour of the operative `must` — a silent partial overruling of the Privy Council by a statute that denies doing so.

The Act's own address for that gate is also wrong: it cites `governance/crates/vjs-engine/src/staged.rs:364-377`. EXECUTED `ls -d governance` in `/home/jellytot/Projects/vibe-justice-system` → `No such file or directory`. EXECUTED the same in `/home/jellytot/Projects/opbox-prod/opbox-kernel` → `governance` exists. The line range 364-377 is correct; the path is valid **only in the subscriber mirror**, and this is canonical text. s1 breached on s3's face.

---

## s7 — THE DANGEROUS ONE (answer to Q3)

**Who classifies: the actor whose conduct is being classified. The code proves it.**

- `crates/vjs-core/src/types.rs:147-160` (READ) — `RouteInput { ... pub risk: RiskLevel, pub public_target: bool, pub external_target: bool, pub irreversible: bool, ... }`. All caller-supplied.
- `crates/vjs-core/src/court.rs:9-25` (READ) — `detect_court_trigger` derives its whole test from those fields: `let significant = input.risk != RiskLevel::Low || input.irreversible || input.public_target || input.external_target;`
- `crates/vjs-core/src/route.rs:87-110` (READ) — `decide_route` consults nothing else. There is **no independent classifier anywhere in the crate**; `resolve_jurisdiction` (`route.rs:76-84`) likewise takes the caller's word.

So s7's two prohibitions operate on a self-declaration:
- *"The route must not return CourtRequired for it"* — set `risk=Low, irreversible=false, public_target=false, external_target=false` and the Act makes it **unlawful for the route to require court**. Today that combination merely *fails to trigger*; s7 upgrades a permissive default into a statutory bar.
- *"a bench must decline to sit on it and say why"* — an **ouster of jurisdiction**. The court loses power to take the matter on its own motion in order to test the classification. The court cannot determine its own jurisdiction, because the input that determines jurisdiction is supplied by the party and the Act forbids the court to look behind it.

The practice this legalises is already live and already loose: `.justice/INDEX.md:41` (READ) — *"NOT a CC-OPBOX 16/17/18 trust boundary (reuses already-trust-rated verbs, no new store/verb/permission-model) - engineer disposes by citing this ruling"*; `:48` — *"This sitting IS the CC-OPBOX 16 pre-implementation determination - build by citing."* Today "dispose by citing" at least requires a prior sitting to cite. s7 replaces it with "dispose by declaring."

And s7's guard against exactly this is FB-1: a saving clause pointing at three unrelated orders.

**CURE: DELETE both `must_not` limbs** (C7). The permissive limb may stand. Guardrail will not vote for a statutory prohibition on the route requiring court, nor for a statutory duty on a bench to refuse jurisdiction, on a classification the actor supplies and the code never checks.

---

## s8 SUBSTANTIVE TRACK (answer to Q4)

**Yes. It is a hole, and the Act's own exemplar demonstrates the hole rather than the limit.**

*"the making court may amend only to correct an error of its own recorded on its face (as [2026] VJS-CC-OPBOX 159 and 164 did)"*.

The making court characterises its own error, and "on its face" has no external test. Measured against the cited exemplar: `.vjs/orders/2026-VJS-CC-OPBOX-164.yaml:13-40` (READ) does not correct a slip. It holds 163 O3 *"PER INCURIAM TO THAT EXTENT"*, substitutes a different mechanism of proof (*"O4's ANNOTATION already records..."*), and then adds a **second, new holding** (*"THE SECOND LIMB, and it is why this order exists at all"*) creating a fresh adjudication/reliance distinction. That is a ratio change by the same single judge (`:8-11` — `bench: [Marchmont CCJ]`, `vote: "1-0 ... sitting alone"`), on its own motion, *"an hour after giving 163"*.

**What stops it at present: nothing.** Cures at C8b.

---

## EVASION TABLE

| § | The evasion, mechanically | The cure I require |
|---|---|---|
| **s1** | The trigger is prose-semantic and the kernel is forbidden to judge meaning (`lawpack/v2/regulations/REG-KERNEL-001.yaml:20-21`, READ: *"The kernel never deliberates, drafts, or judges merits"*; `2026-VJS-PC-017.yaml` forbidden[6], READ: no model, no network). So: (a) write the machinery claim in a non-operative field — PC-17's forbidden[8] (READ) puts `issue`, `vote`, `question`, `runtime_summary`, `source_opinion` and the case-file digest **out of fail-closed scope**; (b) cite an address that exists in the mirror but not in canon — the Act does this at s3 (`governance/crates/...`); (c) cite a real file:line that shows something else — the Act does this at s7 ¶2 (FB-1); (d) hedge to a recital ("as I read the gate"). | C1 |
| **s2** | The field is `issue:`, not `issue_tag` (EXECUTED `grep -rn "dec15_dec19_recitation_and_order_completion" .vjs/orders/` → `163.yaml:6`, `163.yaml:129`, `164.yaml:6`, `164.yaml:112`). Exclusivity is over an exact author-controlled string: append `_v2`, drop an underscore, change case, leave `issue:` empty and put the subject in `question:`, or file the second order with a distinct `issue:` while duplicating `disposes:`. | C2 |
| **s3** | Rely on an authority the extractor cannot see. `refs.rs:47` requires `VJS-`; so `[2026] CC-OPBOX 16`, `CASE-LAW s.10`, `Steering #1`, `INV-15`, `ADR-0028` — all in live operative use per `.justice/INDEX.md:41-48` (READ) — never fire s3. Separately, `must: route_an_unresolved_operative_citation_for_correction` on its literal wording **downgrades** the `Severity::Fatal` at `staged.rs:366` (READ) into a routing outcome, letting the defective order file. | C3 |
| **s4** | *"every store that can hold a citation"* is undefined. The kernel's own store list is three roots — `crates/vjs-core/src/front_door.rs:84-97` (READ): `lawpack/v2`, `.vjs/orders`, `.vjs/court` — and **excludes `.justice/`**, which holds 100+ judgments carrying `citation_id:` in the `CC-OPBOX` series. Measure three, record three on the face, comply — while the largest citation store goes unmeasured. `must_not` bars only *"consulted **one** store"*, so two of five satisfies the letter. And s4's factual predicate is falsified: `crates/vjs-cli/src/admin.rs:15-29` (READ) records the returns-1/single-root defect as **already cured**, and `:31` (READ) shows the series argument is `CC` (`if s == "CC"`), so `vjs next-citation VJS-CC-OPBOX 2026` is a malformed invocation — `VJS-` + `VJS-CC-OPBOX` is the drafter's doubled prefix, not the allocator's. s4 would displace a working allocator with a manual procedure. | C4 |
| **s5** | Reclassify. *"widening a reader"* covers widening a **token** parser. *"no **new** verb"* permits re-rating an existing verb from SENSITIVE to ROUTINE — a permission-model change shipped as machinery. The text omits EXTERNAL tier, token and capability, and the `must_not` names only the undefined term *"trust boundary"*. No maker is named (FB-6), so a clerk signs it. Chain with s11: a registrar-track reader narrowing makes an inconvenient instrument unloadable, and s11 declares it not in force — **a protected floor disapplied with no sitting**. | C5 |
| **s6** | Consolidate and drop a limb. *"restates the rule in force"* is the consolidator's own judgment, and the superseded series *"cease to be separately routable"*, so the omission cannot be detected by routing. No maker named, no tier named (FB-5). | C6 |
| **s7** | Set `risk=Low`, `irreversible=false`, `external_target=false` (`types.rs:155-158`, READ). The Act then **forbids** the route to require court and **compels** the bench to refuse. No code checks the declaration. | C7 (delete) |
| **s8 slip** | Any substantive change is representable as formal. *"a mis-numbered section"* covers `ACT-002:s7` → `ACT-002:s9`. *"a dangling or misallocated citation label"* covers swapping the authority a directive rests on. *"quoting, indentation"* covers re-folding a YAML scalar so a `must:` reads differently. And the *"stated mechanical test"* is stated **by the amender**: state it as "the file still parses", or as "the digest of the `holding` is unchanged" while amending a `directives[].must`. | C8a |
| **s8 subst.** | Self-characterise a ratio change as *"an error of its own recorded on its face"*, as 164 did (`164.yaml:13-40`, READ). Same judge, own motion, one hour. | C8b |
| **s9** | (a) **Forward to nothing.** *"the old citation must continue to resolve, to the new one"* — nothing requires the **new** one to be allocated, to exist, or to be in force. Grounding is existence-only over a defined set (`refs.rs:127-146`, READ; PC-17 D3). A forwarding record therefore **mints resolvability** for a dead token: the `[2026] VJS-DEC 15-22` mechanism reborn under statutory cover. (b) **Cycle.** Forward A→B and B→A; both resolve forever, neither has an instrument. (c) *"small enough to enumerate"* is the amender's judgment — declare it large and enumerate nothing. | C9 |
| **s10** | s10 requires the enumeration, not its adequacy. Enumerate one store and one query form: *"searched `.vjs/orders` with `grep -l "VJS-CC-OPBOX 2"`"* → a **fully compliant** NOT FOUND for an order sitting at `.vjs/orders/2026-VJS-CC-OPBOX-002.yaml:5` (READ, `status: binding`). s10 as drafted would have licensed the 2026-07-31 failure it was written to prevent. (Also: that order's `created_at` is `2026-06-11T10:09:30Z` (`:34`, READ) — s10's recital *"created six days earlier"* than 2026-07-31 is out by 44 days. s1 breached on s10's face.) | C10 |
| **s11** | Narrow what counts as unreadable: define "cannot load" against a lenient reader and the count is zero. Or invert it — narrow the reader and de-force the instrument (FB-3). Or use the escape already in the record: `.vjs/unreadable-orders.txt:1-25` (READ) is a self-authored, **expiring** derogation from a binding order (`.vjs/orders/2026-VJS-CC-OPBOX-160.yaml:51-53` O5, READ), declared *"a DISCLOSED DEVIATION... rather than presenting it as compliance."* s11 creates no rule for such declarations, so the file survives s11 intact. | C11 |
| **s12** | The author writes `actor: engineer`. s12 forbids only the **reader** supplying one; `160.yaml:45` O3 (READ) bound the reader, and s12 does not close the authoring side. And *"UNSTATED"* is textually a named actor. | C12 |
| **s13** | `review_date: 2099-12-31`. Or on expiry, re-reserve the same question with a fresh date — the reservation never lapses, so re-reservation is free. | C13 |
| **s14** | Ship a negative control that passes for the wrong reason: seed a violation that trips a **different** finding code, or one the reader rejects at parse time before the gate runs, and record "the gate failed". The Act requires *"a seeded violation demonstrating that the gate fails"* — not that it fails **for this duty**. (A correct exemplar exists: `crates/vjs-testkit/tests/e2e_gate_harness.rs:110-111`, READ, asserts the specific code `ORDER_CITATION_UNRESOLVED`.) | C14 |
| **s15** | (a) Byte-identity + s8 slip track: a substantive amendment characterised as formal in canon **propagates into every subscriber mirror with no local sitting and no local order**. (b) *"may not amend this Act in its mirror"* says nothing about a subscriber enacting a **local order that reads the Act down** — which the CC-OPBOX series does routinely. | C15 |

---

## CONDITIONS

**C1 — s1 (defect: prose-semantic fail-closed trigger; unbounded scope; no redaction saving).** Replace the third sentence of ¶1 and add:
> A machinery claim is made by a structured `machinery_claims:` block, each entry carrying `assertion`, `address` (a repo-relative path with a line range, or a command string), `mode` (EXECUTED or READ), and `repo` (the repository root in which the address resolves). The kernel refuses a filing only where a `machinery_claims:` entry is absent, malformed, or names a `mode` other than EXECUTED or READ. The kernel does not, and may not, decide by reading prose whether a sentence asserts machinery behaviour (REG-KERNEL-001; [2026] VJS-PC 17 forbidden). An address is not observed unless it resolves in the repository named; a path valid only in a subscriber mirror is not an address in canon. Where an output would disclose data about a real person, a credential, or a tenant identifier, the instrument records the redacted output and the redaction authority, and the unredacted output goes to the confidential annex; this duty never overrides ACT-CONSOLIDATION-FRAMEWORK:s21. Nothing in this section blocks, voids or excludes a record declaring a valid assent_source within the meaning of ACT-COMPUTER-FIRST-REALM:s23; such a defect is surfaced and routed for correction (ACT-ASSENTED-RECORD-PROTECTION:s1).

**C2 — s2 (defect: exclusivity over an author-controlled exact string; wrong field name).** Add:
> The tag is the `issue` field. Two tags are the same tag if equal after normalisation: lowercased, non-alphanumeric runs collapsed to a single separator, leading and trailing separators removed, and any trailing version or ordinal suffix stripped. An empty or absent `issue` is refused. The duty to declare a relation is owed to any live binding order sharing the normalised tag **or** naming the same value in `disposes`. Nothing in this section blocks, voids or excludes a record declaring a valid assent_source; such a defect is routed for correction.

**C3 — s3 (defect: contradicts PC 17 D1; downgrades a Fatal gate; blind to unrecognised series).** Replace the `must` list and add:
> This section is the statutory footing of the existence limb. The finding continues to cite ACT-002:s7 and REG-KERNEL-001 as [2026] VJS-PC 17 D1 directs, and cites this section in addition; nothing in this Act reads down PC 17 D1. The disposition is unchanged and remains Fatal-and-correctable in the terms of PC 17 D2: a non-assented order fails closed; an order whose declared assent resolves has the finding surfaced and routed for correction and is never voided or blocked. A citation in a series the reference extractor does not recognise is reported as UNGROUNDED, counted in the jurisdiction's self-test, and is not treated as resolved.

**C4 — s4 (defect: falsified factual predicate; "every store" undefined and self-satisfying; blocks assented records).** Delete the paragraph beginning *"THE PRESENT STATE, measured 2026-08-04"* in its entirety. Replace the interim rule with:
> Allocation is performed by the kernel allocator over every governed record root declared in the jurisdiction's store register. The store register must name, at a minimum: `lawpack/v2`, `.vjs/orders`, `.vjs/court`, and every tree containing a file carrying a top-level `citation:` or `citation_id:` value, including `.justice/`. An allocation that omits a registered store is void, not merely disclosed. A self-asserted citation not equal to the allocated value is a fatal mismatch on a record that declares no valid assent_source; on a record declaring a valid assent_source it is surfaced and routed for correction and never blocks (ACT-ASSENTED-RECORD-PROTECTION:s1). Where a citation ordinal is by law minted at commencement rather than at authoring, this section does not apply to it.

**C5 — s5 (defect: no maker named, ultra vires ACT-CONSOLIDATION-FRAMEWORK:s7; carve-out narrower than the trust-boundary rule).** Add:
> A regulation under this section is made by the Standing Committee and by no other body; no registrar, clerk, engineer or agent may make one. The registrar track is unavailable to any matter engaging the trust-boundary rule, which is restated here in full and verbatim: where a fork involves a trust boundary — any verb rated SENSITIVE or above, any EXTERNAL auth tier, any token, capability or permission model — the deliberation budget's shortcut does not apply, regardless of code reversibility, and the fork goes to the court before implementation ([2026] CC-OPBOX 16 O3, affirmed [2026] CC-OPBOX 17). A change to the rating, tier, capability or permission model of an **existing** verb is not machinery. A change that narrows or alters what any reader can load is not machinery.

**C6 — s6 (defect: consolidator unnamed; limbs droppable undetectably; excludes assented records; apex reach).** Add:
> A consolidated instrument may be made only by the court that made the latest instrument in the series or by a superior court, and only by a court competent to constitute the highest tier represented in the series. No subscribing jurisdiction may consolidate a series any instrument of which is of apex tier ([2026] VJS-SC 4); such a series is referred up. The consolidated instrument must carry a concordance table mapping every holding, directive and forbidden clause of every superseded instrument either to its restated location or to an express statement that it is discharged or spent, with the authority for that statement. A superseded instrument remains routable and resolves to the consolidated instrument. No assented record is made unroutable by this section.

**C7 — s7 (defect: self-classification converted into an ouster of the court's jurisdiction).** **DELETE** the words *"The route must not return CourtRequired for it, and a bench must decline to sit on it and say why."* **DELETE** both `must_not` entries `returning_court_required_for_a_decide_and_log_matter` and `reading_this_section_as_reaching_a_trust_boundary_fork`. Add:
> The classification of a matter as low-risk, reversible and non-boundary is a declaration by the acting agent, recorded with the declarant's name and the facts relied on. It is not a finding. It binds no court. A bench retains jurisdiction to sit in order to determine whether the classification was correct, and may do so on its own motion. A classification later found incorrect is a breach by the declarant, filed as such. Nothing in this section prevents the route returning CourtRequired.

**C8a — s8 slip track (defect: amender chooses the test; "formal" defined by examples that cover substance).** Add:
> Content-preserving means, and means only: the normalised token sequence of every operative part — the `holding`, each directive's `must`, each `forbidden` clause, and each `actor` — is byte-identical before and after, after YAML scalar reassembly and whitespace normalisation. The test is this test; the amending court may not substitute another. An amendment that changes any operative part is not on the slip track whatever its label, and an amendment described as formal which fails this test is void.

**C8b — s8 substantive track (defect: self-characterised ratio revision).** Add:
> An error is recorded on the face of the order only where the order's own text contradicts itself, demonstrated by quoting the two passages of that order. An error shown by material outside the order is not on its face and requires a superior court. The making court may not, on its own motion, enlarge or narrow the class of acts the order permits, requires or forbids; such an amendment requires a superior court on appeal. A self-amendment is appealable as of right, the prior text remains routable, and where the order has been relied on in any later filed order the amendment must be noted on the face of that later order.

**C9 — s9 (defect: forwarding records mint resolvability; cycles; discretionary enumeration).** Add:
> A forwarding target must be an allocated, existing, in-force instrument, distinct from the source. Forwarding is non-transitive: a forwarding record may not target a citation that is itself forwarded, and a cycle is fatal. A forwarding record is created only by the order of the court amending the citation, never by the engineer, and never by any registrar-track regulation. A forwarded citation grounds only if its target grounds. Delete the words "Where the number of citing documents is small enough to enumerate"; the amending order must enumerate every citing document and state whether each was updated.

**C10 — s10 (defect: enumeration required, adequacy not).** Add:
> The enumeration is measured against the jurisdiction's declared store register (C4). A report omitting a registered store is not a NOT FOUND but an INCOMPLETE SEARCH and may not be relied on. The report must state, for each store searched, the number of records the search read, so an empty result over an empty scan is visible on its face.

**C11 — s11 (defect: subordinate machinery determining the force of law).** Replace *"An instrument the kernel cannot load is NOT IN FORCE. It binds nothing"* with:
> An instrument the kernel cannot load is NOT APPLIED and is reported as UNREADABLE-IN-FORCE. It remains in force. A reader's failure is a defect in the reader, never a change in the law, and no instrument declaring a valid assent_source may be treated as not in force by reason of a load failure (ACT-ASSENTED-RECORD-PROTECTION:s1). "Cannot load" is measured against a named reader version recorded with the count, and the count is reported with the total. A change that increases the count is a regression and fails the self-test. No reader narrowing may be made on the registrar track. A declared deviation from this section is made only by order of a court, names its expiry, and is reported as OWED on expiry.

**C12 — s12 (defect: closes the reader, leaves the author open).** Add:
> An `actor` value is refused where it is empty or equal, after normalisation, to any of: UNSTATED, TBD, NONE, ANY, ALL, SOMEONE, TODO; or where it names no entry on the jurisdiction's declared actor register.

**C13 — s13 (defect: unbounded horizon; free re-reservation).** Add:
> A review date may not exceed ninety days from the date of the order. A reservation may be extended once, by order, with reasons recorded on its face; a second extension requires the court that made the reservation to sit. A question re-reserved on the same subject inherits the original review date. The count of expired reservations is reported where the answer goes, under section 11.

**C14 — s14 (defect: a control that fails for the wrong reason satisfies the letter).** Add:
> A negative control must assert the specific finding code and the specific instrument citation the duty's gate raises; must be shown to pass cleanly when the seeded defect is removed (the positive twin); must reach the gate rather than be rejected upstream of it; and must be re-run in the jurisdiction's self-test rather than recorded once. A control that trips any other code, or that is refused before the gate evaluates, is not a negative control and the duty is reported UNENFORCED.

**C15 — s15 (defect: byte-identity as an automatic propagation channel; local read-down unaddressed).** Add:
> Byte-identity is a duty owed to a canon text whose digest is pinned in the subscriber's `.vjs/lawpack.lock`. An amendment to this Act in canon does not take effect in a subscribing jurisdiction until that jurisdiction bumps its pin on its own deliberate assent on the six conditions of [2026] VJS-PC 10 (REG-KERNEL-001). A subscribing jurisdiction may not amend this Act in its mirror, and no local order may read down, disapply or narrow any section of it; a local order purporting to do so is void to that extent.

**C16 — FB-1, all sections (defect: the Act's floor savings cite tokens that resolve to the wrong instruments).** Throughout the Act, replace every occurrence of "[2026] VJS-CC-OPBOX 16", "CC-OPBOX 16/17/18", "affirmed by 17 and 18" and cognate forms with:
> the trust-boundary rule stated at [2026] CC-OPBOX 16 O3 (`.justice/judgments/county-court/2026-cc-opbox-16.md:44`), affirmed at [2026] CC-OPBOX 17 (`2026-cc-opbox-17.md:58`), and set out in full in this Act at section 5

and add to s15:
> Where this Act cites an authority, the citation is accompanied by the address at which the cited proposition appears, and the citation of a proposition to an authority that does not contain it is a defect on the Act's face correctable on the slip track. The series `CC-OPBOX` and the series `VJS-CC-OPBOX` are distinct series naming distinct instruments; a citation omitting the series prefix is ambiguous and is not a citation.

**C17 — new section, before extent (defect: the corpus holds law in a tree the kernel does not recognise as a store).** Insert:
> **s14A — The store register.** Each jurisdiction must declare, and keep current, the register of trees in which its law and its court records are held, and the kernel's governed-record roots must equal that register. A tree holding an instrument that binds, and which is not on the register, is reported as an UNREGISTERED LAW STORE by the jurisdiction's self-test. On commencement `.justice/` is on the opbox register.

**C18 — the Act binds itself (defect: the draft breaches its own s1, s10 and s14 on its own face).** Add to s15:
> Sections 1, 10 and 14 bind this Act. Every address in this Act must resolve in the repository named. Each section's kernel effect must ship a negative control under section 14 before that section's teeth are reported as enforced; until then the section is reported UNENFORCED. The Act's own factual recitals are subject to section 1 and are correctable on the slip track.

**Recital corrections required at second draft (s1 self-compliance):** (i) `governance/crates/vjs-engine/src/staged.rs` → `crates/vjs-engine/src/staged.rs`, mode READ; (ii) delete the `vjs next-citation` recital in s4 — the invocation was malformed and the defect it alleges is recorded as cured at `crates/vjs-cli/src/admin.rs:15-29`; (iii) s10's *"created six days earlier"* → `created_at: 2026-06-11T10:09:30Z`, fifty days earlier; (iv) s3's characterisation of the `ACT-002:s7` citation as a defect must be replaced by the fact that it is compliance with PC 17 D1.

---

## UNADDRESSED FAILURE MODES

1. **Two live series, one number space.** `[2026] CC-OPBOX 16` and `[2026] VJS-CC-OPBOX 16` are different instruments with different holdings, and `crates/vjs-lawpack/src/refs.rs:47` (READ) sees only the second. s4 governs future allocation and leaves the live ambiguity entirely untouched. The Act is its own first casualty.
2. **`.justice/` is not a law store.** `crates/vjs-core/src/front_door.rs:84-97` (READ) — the roots are `lawpack/v2`, `.vjs/orders`, `.vjs/court`. `.justice/judgments/county-court/` holds the binding trust-boundary rule and 100+ judgments and is scanned by no allocator, no citator and no gate. The Act never reaches the question of which trees hold law. (C17 is my attempt; it is a patch, not a settlement.)
3. **Fidelity, not existence.** Every citation gate in the corpus is existence-only by order (PC-17 D7/D8, READ). s1 requires an address; nothing requires the address to *show* the claim. The Act's own s7 saving is the demonstration. The Act adds no fidelity duty and no sampling mechanism, and PC-17 D8 reserves a fidelity gate to a Sovereign-assented Act citing VJS-ACT 10 by number — which this Act does not do.
4. **Opinions and the de facto citator.** The Act legislates over orders. The operative reasoning lives in `.justice/judgments/`, and `.justice/INDEX.md` (READ) is a hand-maintained prose table doing the work of a citator. Nothing in the Act touches either.
5. **The declared-deviation escape.** `.vjs/unreadable-orders.txt:1-25` (READ) is a self-authored, expiring derogation from a binding order (160 O5), openly labelled as such. The Act creates no rule for who may declare a deviation, for how long, or what happens on expiry. s11 and s13 both assume nobody uses this door.
6. **Deletion and non-existence.** s11 reaches the *unreadable* order. Nothing reaches the order that is deleted, never written, or written outside a registered store — strictly cheaper attacks than making one unreadable, and the s9 forwarding record makes the resulting dangling reference resolvable.
7. **Self-correction velocity.** The measured pathology is four corrections in seven rulings in a day. s2 and s6 detect a series *after* it exists; nothing rate-limits a court correcting itself within the hour, and s8 legalises it. `164.yaml:11` (READ) — *"an hour after giving 163"*.
8. **Cost, unmeasured.** The Act's own drafting note measures governance at 58% of orders and 61% of words. The Act adds fifteen duties and, via s14, a negative control per duty. No section carries a budget, a sunset, or a measurement of its own overhead, and nothing measures whether the correction rate fell.
9. **Redaction.** No section reaches the interaction of the s1 and s10 disclosure duties with `crates/vjs-redact/` (EXECUTED `ls crates/`) and `lawpack/v2/rules/DATA-PRIVATE-001.yaml` (EXECUTED `ls lawpack/v2/rules/`). C1 patches s1; s10 remains exposed.
10. **The Act's own commencement.** The draft self-asserts `citation: "[2026] VJS-ACT 11"` (line 2) while s4 makes a self-asserted citation a fatal mismatch and s4's own recital asserts no allocator works. On its own terms the Act cannot be filed. C4 and C18 together are the minimum needed to make it commenceable.