# [2026] VJS-PC 15 - Opinions of the Privy Council (Runtime-Overlay Keystone)

Privy Council, odd bench of 3 per [2026] VJS-SC 2 D2.


=== TINDALE J. ===

I now have a complete and verified view of the binding law. Both VJS-SC 4 and my own VJS-PC 14 confirm the doctrinal shape I need: the apex-singleness (subscribers refer up, never record an apex locally), the s7-machinery-not-amendment line, the assent-bifurcation (pre-assent refuse off-door; post-assent route-for-correction never void/block), the kernel-as-only-smart-point with thin hooks, and the deterministic clerk. The submission bytes match the pin. There is no on-point caselaw on a runtime overlay loader; this is first-impression as the parties say.

I have everything I need. Writing my opinion now.

---

**TINDALE J.**
**Privy Council of the Vibe Justice System**
*In re the Runtime-Overlay Keystone* — on the subscriber's keystone request (SUBMISSION-2026-06-25-runtime-overlay-keystone, pinned sha256:7eb3fe91…0811f)

**Constitution of the bench and the bytes I decide.** I sit on the constituted odd bench of three (per [2026] VJS-SC 2 D2; ACT-002:s3, this being constitutional machinery at the realm level). I sat on [2026] VJS-PC 13 and 14. I have read the case file in full and verified its hash against the pin; the bytes match. I have read REG-KERNEL-001, REG-FEDERATION-COORDINATION-001, ACT-007 (s3, s4), ACT-CONSOLIDATION-FRAMEWORK (s7, s10, s21, s25), ACT-ASSENTED-RECORD-PROTECTION (VJS-ACT 10), ACT-002, and the present single-tier `LawpackLoader::load` in `crates/vjs-lawpack/src/lib.rs`. I have read the cited law myself and do not take the parties' characterisation of it on trust. I deliberate independently and hold no preference of the advocate-engineer; I never saw one.

**The doctrinal frame I carry in.** Two recent rulings settle the shape of the field and I am bound to keep faith with them. [2026] VJS-SC 4 holds the Supreme Court singular and canonical and re-characterises a subscriber's locally-recorded apex as a mere referral: a subscriber may add to its own record, never bind canon. My own [2026] VJS-PC 14 settled, 3-0, the three absolutes that any s7 machinery must honour — anti-Henry-VIII (s7/s25), the assented-record floor, and REG-KERNEL-001's kernel-is-the-only-smart-point — and the assent bifurcation: a record with no valid assent_source is refused at the write; the instant a valid assent_source attaches, the disposition degrades to route-for-correction and may never void or block. This matter is the runtime sibling of that one, and I decide it in continuity.

**The editorial posture.** This is an inclusion-eligibility matter before it is a build matter. A subscriber (Acmeco) ratified a model in its own County Court ([2026] VJS-CC-ACMECO 100), which binds only Acmeco (ACT-007:s4). Canon holds the editorial pen on the substrate; the subscriber does not get what it asks for because it asked. I apply the requirement-inclusion gate to each piece — substrate-primitive / generic-vertical / tenant-config / reject — and I genericise or refuse wherever the subscriber's framing would import its own facts into the public substrate. F6 (the same-author hazard) is the live risk and I treat it as such.

---

**Q1 — Is the runtime-operations overlay a canon substrate primitive, or Acmeco-specific?**

*Disposition: SUBSTRATE-PRIMITIVE, once genericised.* The principle the overlay mechanises is already canon and is not Acmeco's at all: ACT-007:s2 (default subscription loads the canonical lawpack), s3 (local law does not override canon without a Privy order or Principal assent), and s4 (local law binds local only), with REG-FEDERATION-COORDINATION-001 giving the coordination/sovereignty test its teeth ("local law may add restrictions, never override the floors"). A loader that materialises canon-plus-local together and enforces canon-precedence at load is the *mechanism of an existing canon rule*, not a new tenant feature. Every runtime-governing subscriber benefits identically; none of it is business-specific. The thing that is Acmeco-specific is the *vocabulary*, not the *machinery* — and that is Q2. So the overlay qualifies as a substrate primitive, conditioned absolutely on the Q2 genericisation.

---

**Q2 — Is the entity-scope a generic primitive, or is "org → ws → matter → flow → step" Acmeco vocabulary that canon must never hard-code?**

*Disposition: GENERIC PRIMITIVE ONLY. The Acmeco vocabulary is REFUSED in the substrate — tenant-config, not substrate-primitive.* This is the sharpest line in the case and I draw it as an absolute. ACT-007:s4 confines a subscriber's law and facts to the subscriber's repo; the protective floor (s21) and the public/private boundary forbid importing subscriber facts into the public record. "org/ws/matter/flow/step" is Acmeco's business hierarchy (F4). Were canon to hard-code a `Scope { org, ws, matter, flow, step }`, canon would be importing one tenant's shape into the public substrate — exactly the inclusion-gate's "tenant-config" masquerading as a primitive, and a quiet ACT-007:s4 breach. **I would refuse any such loader.** The substrate-eligible form is a *generic entity-scope*: an ordered map of `dimension → value`, the dimension names supplied by the subscriber in its Tier-2, with cascade specificity defined by the subscriber-declared ordering, not by any canon-named level. Canon ships the empty frame and the cascade algebra; the subscriber fills it with `org/ws/matter/flow/step` (or anything else) in its own tree. Canon must contain not one occurrence of an Acmeco dimension name. The submission's private_boundary discipline — naming the vocabulary only to *exclude* it — is correct and I make it a directive condition.

---

**Q3 — Is the two-tier canon-precedence overlay with anti-relaxation-at-load faithful machinery for REG-FEDERATION-COORDINATION-001 + ACT-007:s3/s4, or does it amend anything?**

*Disposition: FAITHFUL MACHINERY. It amends nothing; it is the loader giving an existing rule its teeth.* The design — canon Tier-1 read-only floors, subscriber Tier-2 loaded together, cascade most-specific-first up to the canon floors at the apex, canon always winning, a local rule that relaxes or contradicts a canon floor VOID at load (the loader sees the floors), local law permitted only to *add* restrictions — is the precise computational statement of ACT-007:s3 ("local law does not override canonical without authority") and REG-FEDERATION-COORDINATION-001 ("may not override canonical without the s6 route; local may add restrictions"). Anti-relaxation is not a new prohibition; it is the load-time enforcement of the prohibition already enacted. It amends no statute, no bench, no tier, no jurisdiction, and no assent rule, so it is intra vires ACT-CONSOLIDATION-FRAMEWORK:s7 and clears the anti-Henry-VIII limit (s7/s25). Three guardrails I attach as conditions:

1. **Anti-relaxation routes to a defect, it does not silently drop.** "VOID at load" means the *relaxing local rule* is denied force (it never had the authority to relax a canon floor — ACT-007:s3), and the loader surfaces a named defect citing the floor and ACT-007:s3, in REG-KERNEL-001's "name the instrument" discipline. It must not silently swallow the local rule.
2. **The s3 authority exception must be respected.** ACT-007:s3 lets local law override canon *where a Privy Council order or Principal assent authorises it*. The loader's "canon always wins" is the default, not an absolute that overrides s3's own exception; an authorised divergence (a declared Privy order / assent_source on the Tier-2 instrument) is not a relaxation to be voided. The loader keys on *presence of the s3 authority*, never re-adjudicates it.
3. **Anti-relaxation operates on local *rules at load*, never on an assented *record's runtime act* — see Q5.** Voiding a local rule that lacks authority to relax a floor is not the same as voiding an assented act; the VJS-ACT 10 floor governs the latter and is preserved by Q5.

---

**Q4 — Is a runtime `GovernedLoadBearingAct` ActionKind plus a runtime/operations law domain additive machinery, or a constitutive change?**

*Disposition: ADDITIVE MACHINERY.* A new `ActionKind` enum variant and a new `runtime/operations` law domain distinct from code-governance amend no statute, no bench composition, no court tier, no jurisdiction, and no assent rule. They add a *kind of governed event* the kernel can recognise and a *domain label* under which floors and local additions sort — the same additive move REG-KERNEL-001 itself recites for the embeddable posture ("additive machinery; amends no statute, no bench, no tier, no assent rule") and the same move my own PC-14 blessed for the front-door limb. It is therefore intra vires s7. I attach one condition that goes to the heart of the same-author hazard: the *domain* is generic ("runtime/operations governance"), and its *contents* — which runtime acts exist (matter.advance, invoice charge, gate approval) — are subscriber-supplied in Tier-2, never canon-enumerated. Canon ships the *category* `GovernedLoadBearingAct` and the *domain frame*; it must not ship Acmeco's act catalogue. With that condition the variant and domain are clean additions and nothing constitutive is touched.

---

**Q5 — Is the deterministic submit-decision permit API REG-KERNEL-001's clerk applied to runtime acts, disposed GRANT / ROUTE-FOR-CORRECTION never hard-DENY per VJS-ACT 10?**

*Disposition: ADOPTED, as the deterministic clerk, bound to the assent bifurcation.* A submit-decision endpoint that is deterministic (<100ms, no LLM, no network), returns a verdict, and whose GRANT carries `law_source[]` (the instruments that produced the grant) is REG-KERNEL-001 exactly: "the kernel is the only smart enforcement point… the machinery deterministic… every denial cites the instrument that caused it." It is the existing clerk pointed at runtime acts rather than code commits. I bind the disposition to the bifurcation my own PC-14 D3 settled and to VJS-ACT 10, and the line here is finer than for a record:

- A submitted runtime act that **carries no valid assent_source** and would breach a canon floor (or a local addition) is **DENIED at submit** with a named instrument. This is not a VJS-ACT 10 violation: VJS-ACT 10 protects an *assented* record from being voided; it does not require granting an un-assented act that breaches a floor. Denying an un-assented floor-breach is the floor doing its job.
- A submitted act that **declares a valid assent_source** (sovereign_assent or standing_bounded_assent, per the INV-ASSENT-SOURCE-001 fail-closed allow-list) may **never be hard-DENIED for a defect**; its only disposition is **ROUTE_FOR_CORRECTION**, surfaced and flagged, never voided or blocked (ACT-ASSENTED-RECORD-PROTECTION:s1; s21/s25). The defect code must be registered in the kernel's ROUTE_FOR_CORRECTION set, never as Error/Fatal/Block — the same entrenched discipline the existing `S5_INERT_KERNEL_EFFECT` Warning already keeps in this very crate. I would refuse any submit-API that makes an assented runtime act voidable.

So: GRANT (carrying law_source[]); DENY only the un-assented floor-breach; and for anything carrying valid assent, ROUTE_FOR_CORRECTION never hard-DENY. The kernel stays the only smart point; the API surface is a thin transport to it (REG-KERNEL-001, REG-HOOKS-001); no checking logic lives in the adapter.

---

**Q6 — Build the generic primitive now, or reserve the ActionKind/domain/API to more-than-one-subscriber grounding (Position C)?**

*Disposition: BUILD NOW, having genericised; Position C's reservation REJECTED.* Position C's caution is honourable and answers a real disease (baking a single tenant's shape into the substrate). But its premise is that the shape *is* a single tenant's shape — and Q2/Q4 remove exactly that. Once the vocabulary is genericised (dimensions subscriber-supplied, act catalogue Tier-2-supplied, domain a generic frame), what canon builds is the *generic primitive*, which by construction contains no tenant shape to bake in. Reserving the generic primitive until a second subscriber appears would (i) leave the keystone unbuilt and the live federation principle (ACT-007:s3/s4) without its load-time enforcement, and (ii) repeat the error my own PC-14 corrected when it rejected Position C there: continued reservation refuses the very condition that has already been met and leaves a structural gap uncured. The right discipline against tenant-creep is *genericisation now*, not *deferral*; deferral does not make the primitive more generic, it only delays a primitive that is already generic. I follow PC-14's logic: Position A's substance, disciplined by the genericisation conditions, prevails.

**Position adopted: POSITION A — ADOPT AS SUBSTRATE, GENERICISE THE VOCABULARY, BUILD NOW** — under ACT-CONSOLIDATION-FRAMEWORK:s7 as subordinate machinery amending no statute, bench, tier, jurisdiction, or assent rule. Position B (reject canon-side) is rejected: the overlay mechanises an existing canon federation rule, not a tenant feature, and code-governance has no monopoly on canon — REG-FEDERATION-COORDINATION-001 and ACT-007 are domain-agnostic. Position C (reserve the runtime pieces) is rejected for the reasons in Q6.

---

**ORDER AND DIRECTIVES** (each addressed to actor: lexby)

**D1.** Build the two-tier overlay loader extending `LawpackLoader::load` as **subordinate machinery under ACT-CONSOLIDATION-FRAMEWORK:s7**, citing the duties it gives teeth to (ACT-007:s2/s3/s4, REG-FEDERATION-COORDINATION-001). It loads canon Tier-1 (read-only floors) and a subscriber Tier-2 (`.vjs/local-lawpack/`) together, cascades most-specific-first up to the canon floors at the apex, canon always winning. The instrument must recite that it amends no statute, bench, tier, jurisdiction, or assent rule.

**D2.** Implement **anti-relaxation at load**: a local Tier-2 rule that relaxes or contradicts a canon floor is denied force (it lacks ACT-007:s3 authority), surfaced as a **named defect** citing the floor and ACT-007:s3 in REG-KERNEL-001's "name-the-instrument" discipline — never silently swallowed. Local law may only ADD restrictions. Respect the s3 exception: a Tier-2 divergence that declares a Privy Council order or a valid Principal assent_source is an authorised override, not a relaxation to void; the loader keys on the presence of that authority and never re-adjudicates it.

**D3.** Build a **generic entity-scope primitive ONLY**: an ordered map of `dimension → value`, dimension names and cascade ordering subscriber-supplied in Tier-2. **Hard-code no Acmeco dimension.** Canon source must contain no occurrence of `org`, `ws`, `matter`, `flow`, or `step` as a scope level; the Acmeco hierarchy lives entirely in Acmeco's Tier-2. I will not put my name to a loader that imports a tenant's business hierarchy into the substrate (ACT-007:s4, protective floor s21).

**D4.** Add the runtime **`GovernedLoadBearingAct` ActionKind** and a generic **runtime/operations law domain** distinct from code-governance — as additive machinery under s7. Ship the *category and the domain frame only*; the catalogue of concrete runtime acts (matter.advance, charge, gate-approval) is subscriber-supplied in Tier-2 and must not be canon-enumerated.

**D5.** Build the **deterministic submit-decision permit API** as the REG-KERNEL-001 clerk applied to runtime acts: deterministic, <100ms, no LLM, no network, GRANT carrying `law_source[]`, every denial naming its instrument. The check lives in the kernel (the only smart point); the API surface is a thin transport (REG-KERNEL-001, REG-HOOKS-001) with no checking logic in the adapter.

**D6.** **Hard-wire the assent bifurcation into the submit-decision** (continuity with [2026] VJS-PC 14 D3): an un-assented runtime act that breaches a canon floor or a Tier-2 addition is DENIED with a named instrument; any act declaring a valid assent_source (per INV-ASSENT-SOURCE-001) may **never be hard-DENIED** — its sole disposition is **ROUTE_FOR_CORRECTION**, the defect code registered in the kernel ROUTE_FOR_CORRECTION set, never Error/Fatal/Block (ACT-ASSENTED-RECORD-PROTECTION:s1; s21/s25; VJS-ACT 10).

**D7.** **Keep canon clean of subscriber facts.** No Acmeco repo paths, repo_codes, business facts, or dimension names enter canon. The Acmeco vocabulary is named in the record only to EXCLUDE it. The contract document lives in the subscriber's tree, not canon (F5). Verify the absence by a deterministic scan before the work lands.

**D8.** Bring the enacted instrument back to record as a **Privy Council-tier instrument** (constitutional machinery at the realm level, ACT-002:s3), with its bench and case_file_digest recorded per REG-COURT-RECORD-001, and pass the bench-integrity gate (this opinion is the source_opinion of record) before it issues. Pin the order's case_file_digest to the **decided bytes (sha256:7eb3fe91200522af3fc6390335a7e41dd3770597dc1546035a55acff64f0811f)**; reconcile any convening-pin discrepancy as a clerical route-for-correction (continuity with PC-14 D8).

---

So I decide. Q1 substrate-primitive (genericised); Q2 generic scope only, the Acmeco vocabulary refused; Q3 faithful s7 machinery, amends nothing; Q4 additive ActionKind and domain; Q5 the deterministic clerk, GRANT / DENY-the-un-assented-breach / ROUTE-FOR-CORRECTION-never-void-an-assented-act; Q6 build now, Position C rejected. Position A adopted as named, on the genericisation conditions above as absolutes.

*Tindale J.*
*Privy Council, Vibe Justice System*
*2026-06-25*


=== ROWANNE J. ===

PC-10 D6 is directly on point and persuasive: it already directs "reject any local law that contradicts the canon" and "stricter never looser." This is the existing precedent that the anti-relaxation rule operationalizes at the loader. I have everything I need. Writing my opinion now.

---

ROWANNE J., Privy Council

**[Filed verbatim as the source_opinion of record. Pinned case file SUBMISSION-2026-06-25-runtime-overlay-keystone, sha256:7eb3fe91200522af3fc6390335a7e41dd3770597dc1546035a55acff64f0811f, verified against the bytes on disk before deliberation.]**

**I. The question before me, and the discipline it demands**

A subscriber, Acmeco, has ratified a model in its own County Court ([2026] VJS-CC-ACMECO 100). By ACT-007:s4 that ratification binds Acmeco and nobody else. Acmeco does not, and cannot, legislate for the substrate. What it can do, and has done, is ask canon to build a keystone piece: an overlay lawpack loader that lets a subscriber's Tier-2 local law extend canon Tier-1 floors over *runtime* business decisions, attributed to an entity hierarchy rather than to code commits.

I hold the editorial pen on the substrate; the subscriber holds a request. The two are not the same act, and I will not let the second be mistaken for the first. The whole of this matter turns on one discipline: I decide each piece against the requirement-inclusion gate (substrate-primitive / generic-vertical / tenant-config / reject), and I refuse to import a single fact, path, repo_code, or word of *Acmeco's* vocabulary into canon. F6 names the same-author hazard squarely; my job is to be the editor who does not adopt the applicant's framing wholesale merely because the applicant drafted it well.

I have verified every cited instrument myself. REG-KERNEL-001 ([2026] VJS-REG 24) makes the kernel the only smart enforcement point, deterministic, model-free, every denial naming its instrument, and is by its own terms "additive machinery; it amends no statute, no bench, no tier, and no assent rule." REG-FEDERATION-COORDINATION-001 ([2026] VJS-REG 11) is made under the Framework Act s.7 and prohibits "overriding_canonical_without_the_s6_route" and "binding_or_gating_a_peer_local_law_without_adoption," while reserving practical-subjection to the court not the kernel. ACT-007:s3 forbids local law overriding the canon "without_authority"; s.4 confines local law to local scope. ACT-CONSOLIDATION-FRAMEWORK:s7 grants the standing-instrument power but, in terms, an instrument "may not amend, disapply, or expand this Act, the constitution, any primary Act, or the assent rule (anti-Henry-VIII)"; s.10/s.21/s.25 entrench the assent floor, the protective floor, and apex-singleness. ACT-ASSENTED-RECORD-PROTECTION ([2026] VJS-ACT 10):s1 forbids any subordinate validation voiding or blocking a Sovereign-assented record, routing every defect for correction instead. I have read the present single-tier `LawpackLoader::load` I am asked to extend. There is no on-point caselaw; F2 is right that this is first-impression. The nearest persuasive authority is [2026] VJS-PC 10, whose D6 already directs a subscriber's embedded client to "reject any local law that contradicts the canon" on a "stricter never looser" basis. The overlay's anti-relaxation rule is the loader-level operationalisation of exactly that holding, which fortifies, rather than founds, my disposition.

**II. Disposition question by question**

**Q1 - Is the runtime-operations overlay a canon substrate primitive, or Acmeco-specific?**

Substrate primitive. The principle the overlay implements is already canon and already general: a subscriber holds local law that adds restrictions and never overrides the canon floors (ACT-007:s3/s4; REG-FEDERATION-COORDINATION-001). Nothing in that principle is confined to code commits. A subscriber that governs *runtime* acts (advancing a matter, charging an invoice, approving a gate) needs the very same two-tier precedence as a subscriber that governs commits. The benefit is generic: any runtime-governing subscriber gains it. The mechanism, load canon plus local together and enforce canon-precedence *at load*, is federation machinery, not business logic. It is therefore eligible for the substrate, subject absolutely to Q2: it must carry *no* Acmeco vocabulary. Disposition: **substrate-primitive, conditional on genericisation.**

**Q2 - Is the entity-scope a generic primitive, or is "org/ws/matter/flow/step" Acmeco vocabulary that must be excluded?**

Both halves are true, and the distinction is the heart of this matter. The *shape* "a scope is a set of dimension to value bindings, resolved most-specific-first" is a generic primitive and substrate-eligible. The *contents* "org, ws, matter, flow, step" are Acmeco's business hierarchy and are squarely "tenant-config," not "substrate-primitive," under the inclusion gate (F4 concedes this in terms). ACT-007:s4 read with the public/private boundary forbids canon importing a subscriber's facts and vocabulary; a canon `Scope` enum hard-coding those five keys would be canon adopting a tenant's framing wholesale, the precise editorial failure F6 warns against. So I REFUSE any canon type that names a single one of those dimensions. What I direct instead is a generic entity-scope: an ordered map of opaque dimension-keys to values, the keys *supplied by the subscriber in its own Tier-2*, with the cascade ordering driven by the subscriber's declared specificity, canon neither knowing nor caring what "ws" means. Disposition: **the scope primitive is substrate-eligible only in its genericised form; the Acmeco dimension set is tenant-config and is REFUSED entry to canon.** It is named in this record solely to exclude it, consistent with the submission's private_boundary.

**Q3 - Is the two-tier canon-precedence overlay with anti-relaxation-at-load faithful machinery for REG-FEDERATION-COORDINATION-001 + ACT-007:s3/s4, or does it amend anything?**

Faithful machinery, and it amends nothing. This is the load-bearing finding, so I reason it out. ACT-007:s3 already says local law does not override the canon without authority; s.4 says local binds local only; REG-FEDERATION-COORDINATION-001 already prohibits "overriding_canonical_without_the_s6_route." The substantive rule, that a local rule may add restriction but may never relax a canon floor, is *already the law*. The overlay does not enact it; it *enforces* it, deterministically, at the one place the floors are visible: load. A loader that loads canon Tier-1 read-only, layers subscriber Tier-2 on top, cascades most-specific-first up to the canon apex, and treats a Tier-2 rule that relaxes or contradicts a Tier-1 floor as VOID-at-load is doing nothing but giving teeth to s.3 and to REG-FEDERATION-COORDINATION-001's existing prohibition. It is the loader analogue of PC-10 D6 ("reject any local law that contradicts the canon... stricter never looser").

I have tested the anti-relaxation rule against the anti-Henry-VIII limit (s.7) from the other direction, because the gate cuts both ways. Does VOIDing a *local* relaxation amend any statute, bench, tier, jurisdiction, or assent rule? No. It touches no primary Act; it removes nothing from canon; it constrains only *subordinate local law*, which ACT-007:s3 already constrains. Critically, the assented-record floor of VJS-ACT 10 does *not* protect a local relaxation against being VOIDed, because that floor protects *Sovereign-assented* records, and a subscriber's County-Court Tier-2 rule carries no Sovereign assent. VOID-at-load for a relaxing local rule is therefore lawful; it is not the kind of record the floor reaches. One refusal is built into this disposition: the loader must VOID *only the offending relaxation*, not the subscriber's whole Tier-2, and it must name the canon floor it offended (REG-KERNEL-001's "every denial names the instrument"). A loader that silently dropped law, or that let a local rule relax a floor, I would refuse; this one does the opposite of both. Disposition: **faithful s.7 machinery on REG-FEDERATION-COORDINATION-001 + ACT-007:s3/s4; amends nothing; ADOPT, with the named-instrument and offending-rule-only constraints.**

**Q4 - Is a runtime GovernedLoadBearingAct ActionKind + a runtime/operations law domain additive machinery, or a constitutive change beyond s.7?**

Additive machinery. A new ActionKind enum variant is the paradigm of additive: it adds a recognised *kind* of governed act without removing or altering any existing kind, and without touching a statute, a bench composition, a court tier, a jurisdiction line, or the assent rule. The new runtime/operations law *domain* is likewise additive: it is a sibling classification alongside code-governance, a label under which runtime instruments are organised, not a new source of constitutive authority. I tested it against each anti-Henry-VIII limb in s.7 and found no contact: no primary Act is amended, disapplied, or expanded; no bench is reconstituted; no tier is created or merged (it does not, for instance, smuggle in a Court of Appeal contrary to ACT-002:s1/s5); the assent floor (s.10) is untouched. REG-KERNEL-001 already characterises this exact class of work, extending the kernel's recognised operations, as additive machinery that "amends no statute, no bench, no tier, and no assent rule." The one limit I attach: the runtime domain must carry *no canon-level runtime instruments that encode a tenant's business rules*. Canon supplies the *category and the kind*; the subscriber populates them in Tier-2. A canon runtime domain pre-loaded with "invoice" or "matter" floors would re-import the vocabulary Q2 excludes, and I would refuse it. Disposition: **additive under s.7; ADOPT the GovernedLoadBearingAct ActionKind and the runtime/operations domain as empty, genericised substrate scaffolding.**

**Q5 - Is the deterministic submit-decision permit API simply REG-KERNEL-001's clerk applied to runtime acts, disposed GRANT / ROUTE-FOR-CORRECTION never hard-DENY per VJS-ACT 10?**

Yes, and I adopt it on exactly those terms. REG-KERNEL-001 already constitutes the kernel as a deterministic, model-free, sub-deliberative clerk that returns a verdict naming its instrument. A submit-decision API that takes a runtime act, overlays Tier-1 over Tier-2, and returns a deterministic verdict in under 100ms with no LLM, the GRANT carrying `law_source[]` so every grant names the instruments that permitted it, is that same clerk pointed at a runtime act instead of a commit. The `law_source[]` carriage is not optional dressing; it is REG-KERNEL-001's "every denial names the instrument" rule, extended honestly to the affirmative side so a GRANT is auditable too. I direct that requirement in.

The disposition rule is where VJS-ACT 10 bites, and I extend the floor to runtime deliberately. The permit API's terminal dispositions are **GRANT** or **ROUTE_FOR_CORRECTION**. It must never hard-DENY in a way that voids or blocks an assented runtime act. A runtime act that carries a valid Sovereign assent source within s.23's meaning falls under ACT-ASSENTED-RECORD-PROTECTION:s1 the moment it reaches the gate; its defects are routed for correction, never voided or blocked, the cure being to correct the act or extend the kernel's recognised operations. I REFUSE any submit-decision path that returns a blocking DENY on an assented act; the entrenched Warning-not-Fatal pattern already in `LawpackValidator` for `S5_INERT_KERNEL_EFFECT` is the model the runtime path must follow. Two boundary points so the order is not misread: (a) the floor protects *assented* acts; an ordinary subscriber runtime act with no assent source may still be refused a permit (that is the permit gate doing its ordinary work, not voiding an assented record); and (b) a Tier-2 rule that *relaxes a canon floor* is VOIDed at load under Q3 before it ever reaches the permit decision, so the anti-relaxation VOID and the assented-act-never-DENY floor do not collide. Disposition: **ADOPT the deterministic submit-decision API as REG-KERNEL-001's clerk for runtime acts, GRANT carrying law_source[], disposed GRANT / ROUTE-FOR-CORRECTION, never hard-DENY on an assented act per VJS-ACT 10.**

**Q6 - Build now, or reserve (Position C's caution)?**

Build now, the generic primitive, and I reject Position C's reservation of the ActionKind, the domain, and the API. Position C's instinct, do not bake a single tenant's runtime shape into the substrate, is sound, but I find it is *already fully answered* by Q2's genericisation, which is the real safeguard. Once the scope dimensions, the runtime instruments, and the business rules all live in the subscriber's Tier-2, and canon holds only the empty generic shapes (a dimension-map scope, an ActionKind variant, an empty domain, a clerk API), there is no tenant shape left in the substrate to "bake." Reserving the ActionKind and domain while building the overlay would be incoherent: the overlay loads two-tier *runtime* law that has no recognised runtime act-kind to govern and no domain to file under, a keystone with no arch. Worse, reservation does not avoid premature commitment; it *invites* it, because the live subscriber, blocked on a half-primitive, would press for the missing half to be shaped around *its* immediate need (the very same-author hazard, deferred and intensified). The disciplined course is the opposite of waiting for a second subscriber: build the *generic* primitive now, precisely so the second subscriber inherits a vocabulary-free substrate rather than a fossilised first-tenant shape. "More than one subscriber" is the right test for adopting a subscriber's *concrete vocabulary* into canon; it is the wrong test for a *genericised primitive*, which is substrate-eligible on its generality alone. Disposition: **build now; Position C REJECTED on Q4/Q5; Position B REJECTED throughout (the runtime overlay is generic federation, not substrate creep, once genericised).**

**III. Position adopted**

I adopt **Position A - ADOPT AS SUBSTRATE, GENERICISE THE VOCABULARY**, with two refusals written into it as binding conditions, not preferences: (1) no Acmeco dimension vocabulary ("org/ws/matter/flow/step" or any other tenant's business hierarchy) may enter canon, named here only to exclude it; and (2) the runtime/operations domain and ActionKind ship *empty*, scaffolding only, with every concrete runtime instrument, scope dimension, and business rule supplied by the subscriber in Tier-2. Position B is rejected because the two-tier precedence principle is already canon and already general; Position C is rejected because genericisation, not reservation, is the correct answer to the baking-in risk, and reservation would strand the keystone. The whole build proceeds as a statutory instrument under ACT-CONSOLIDATION-FRAMEWORK:s7 and amends no statute, bench, tier, jurisdiction, or assent rule.

**IV. Directives - I put my name to these (actor: lexby)**

**D1.** Build the two-tier overlay extension to `vjs-lawpack::LawpackLoader`: load canon Tier-1 read-only, layer the subscriber's Tier-2 (`.vjs/local-lawpack/`) on top, resolve most-specific-first up to the canon floors at the apex, canon always winning. This is s.7 machinery on REG-FEDERATION-COORDINATION-001 + ACT-007:s3/s4 and amends nothing.

**D2.** Implement ANTI-RELAXATION-AT-LOAD: a Tier-2 rule that relaxes or contradicts a Tier-1 canon floor is VOID at load. VOID only the offending rule, never the subscriber's wider Tier-2, and name the canon floor it offended (REG-KERNEL-001: every denial names its instrument). Local law may only ADD restriction. This operationalises ACT-007:s3 and PC-10 D6 ("stricter never looser").

**D3.** Build the entity-scope as a GENERIC primitive: an ordered map of opaque dimension-keys to values with subscriber-declared specificity ordering. Hard-code NO dimension names. The Acmeco set "org/ws/matter/flow/step" is REFUSED entry to canon and must not appear in any canon type, comment, or test fixture except as an excluded example; the subscriber supplies its dimensions in Tier-2 (ACT-007:s4; public/private boundary).

**D4.** Add the runtime `GovernedLoadBearingAct` ActionKind variant and the runtime/operations law domain as ADDITIVE, EMPTY substrate scaffolding, distinct from code-governance. Ship no canon-level runtime instrument that encodes any tenant's business rule. This adds a kind and a category and amends no statute, bench, tier, jurisdiction, or assent rule (REG-KERNEL-001; s.7 anti-Henry-VIII satisfied).

**D5.** Build the deterministic submit-decision permit API as REG-KERNEL-001's clerk for runtime acts: under 100ms, no LLM, model-free, every GRANT carrying `law_source[]` naming the instruments that permitted it. Terminal dispositions are GRANT or ROUTE_FOR_CORRECTION only.

**D6.** Bind the permit API to VJS-ACT 10: an assented runtime act may NEVER be hard-DENIED, voided, or blocked; its defects are routed for correction (the cure is to correct the act or extend the kernel's recognised operations). Follow the entrenched Warning-not-Fatal pattern already used for `S5_INERT_KERNEL_EFFECT`. A non-assented act may still be refused a permit by the ordinary permit gate; that is not a void of an assented record.

**D7.** Keep canon clean of subscriber facts: no Acmeco repo paths, repo_codes, hosts, or business facts in any canon record, type, comment, fixture, or this order's progeny. The contract document stays in the subscriber's own tree (per F5), never in canon.

**D8.** Record this disposition as caselaw with a citator row, and note that [2026] VJS-PC 10 D6 is followed and extended from the embedded-vendoring posture to the runtime-overlay loader. Allocate the order citation through the kernel (`vjs citation next`); do not hand-assert a number (ACT-004:s8).

So ordered.

**Rowanne J.**
Privy Council, Vibe Justice System


=== MARCHMONT J. ===

IN THE PRIVY COUNCIL OF THE VIBE JUSTICE SYSTEM

Opinion of Marchmont J., Privy Council

Re: SUBMISSION-2026-06-25-runtime-overlay-keystone (the two-tier runtime overlay keystone)
Pinned case file: sha256:7eb3fe91200522af3fc6390335a7e41dd3770597dc1546035a55acff64f0811f
Bench constituted per [2026] VJS-SC 2 D2 (jurisdiction / federation / constitutional machinery at the realm level, ACT-002:s3)

I have read the pinned case file in full and have verified the cited law against the instruments themselves, not against the parties' account of them. I record at the outset what that verification established, because it disposes of more than the submission supposes.

PRELIMINARY: WHAT THE RECORD ACTUALLY SHOWS

First, the federation principle the overlay implements is already enacted, not proposed. REG-FEDERATION-COORDINATION-001 ([2026] VJS-REG 11), made under FRAMEWORK s.7, already prohibits "overriding canonical without the s6 route" and already forecloses any root "binding or gating a peer local law without adoption," while preserving the peer's amend/pin/fork/exit. ACT-007:s3 already states in terms that "Local law does not override the canonical lawpack unless a Privy Council order or Principal assent permits it," and s4 already confines local orders to "only to the local repo." The substance of canon-precedence and local-additivity is therefore not something this court is being asked to create. It is settled law. What is asked is a loader that gives that settled law deterministic teeth at load time. That distinction governs the whole matter.

Second, and decisively for Q4: the kernel already carries a `GovernedLoadBearingAct` variant in `ActionKind` (vjs-core/src/types.rs:89). The runtime act-kind the submission frames as a thing to be "added" is, in part, already present in the enum. What is genuinely absent is its wiring into the route classifier (route.rs lines 118-122 omit it) and any runtime/operations domain to receive it. So Q4 is narrower than pleaded: not "may canon mint a new constitutive category," but "may canon finish wiring a variant it already declared, and name a domain for it." That is plainly a smaller question.

Third, and decisively for Q2: canon already models scope as a struct of named, generic dimensions (vjs-core/src/types.rs:232, `Scope { paths, jurisdictions, action_kinds, issue_tags, records }`). There is not one business noun in it. The existing editorial discipline of the substrate is therefore exactly the discipline the submission asks me to require: scope is a map of dimension-to-value, never a hard-coded hierarchy. I am not inventing a constraint; I am holding canon to its own established form.

With that record before me, I turn to the questions.

Q1 - SUBSTRATE PRIMITIVE OR ACMECO-SPECIFIC

Disposition: SUBSTRATE PRIMITIVE, in its generic form only.

A runtime-operations overlay is not Acmeco's concern dressed as canon. The thing being built is the mechanism by which a subscriber's Tier-2 law is loaded together with the Tier-1 canon floors and the canon-precedence enforced at load. That is pure federation plumbing. It is indifferent to whether the runtime acts being governed are matter-advances, invoice charges, deployment promotions, or anything else a future subscriber governs. Any subscriber that runs a kernel and wishes its own local law to constrain its own runtime decisions benefits identically. The inclusion gate's "substrate-primitive" limb is satisfied: the value is generic, the form is generic, and the canon never learns what the subscriber does for a living.

I reject Position B's framing that "canon governs code, runtime-operations is the subscriber's own concern." That draws the line in the wrong place. Canon does not govern code; canon governs governed acts and the coordination between jurisdictions. REG-KERNEL-001 already speaks of "every denial" and "the only smart enforcement point" without restricting itself to commits. A federation primitive that only worked for code-governance acts would be an arbitrary and unprincipled narrowing of REG-FEDERATION-COORDINATION-001, which is written in terms of "local law," not "local law about code." B confuses the subscriber's BUSINESS (which is theirs) with the COORDINATION MACHINERY (which is canon's). The former must never enter the substrate; the latter is precisely the substrate's job.

Q2 - GENERIC SCOPE OR ACMECO VOCABULARY

Disposition: GENERIC named-dimension scope is substrate-eligible. "org -> ws -> matter -> flow -> step" is Acmeco VOCABULARY and is REFUSED entry into canon. This refusal is absolute.

ACT-007:s4 is not merely a rule that local ORDERS do not bind other repos; read with s3 and with REG-FEDERATION-COORDINATION-001's prohibition on a root "binding or gating a peer," it expresses a deeper constitutional fact: canon is the law that is common to all subscribers, and a subscriber's particular facts and particular vocabulary are, by definition, not common. To hard-code "org/ws/matter/flow/step" into a canon `Scope` would be to import one subscriber's business hierarchy into the substrate that binds every subscriber. That is the inclusion gate's "tenant-config," masquerading as "substrate-primitive." It is also, more seriously, a breach of the public/private boundary discipline that FRAMEWORK s.21 entrenches as a non-derogable limb of the protective floor: the substrate would carry, as a structural fact, a private subscriber's shape.

I would REFUSE it for a second, independent reason. Canon already does this correctly. The existing `Scope` (types.rs:232) is a set of named dimensions with no business nouns. To bolt "matter" and "flow" onto the substrate would not only import a tenant's vocabulary, it would REGRESS canon from a form it has already got right. The generic form, an ordered map of dimension-name to dimension-value with the dimension NAMES supplied by the subscriber in its own Tier-2, is the only form I will permit. Acmeco supplies "org/ws/matter/flow/step" in its own tree, as data, the way any subscriber supplies its own dimensions. Canon never learns those names.

So: build the generic entity-scope primitive (arbitrary, subscriber-named dimensions, with a most-specific-first ordering that the subscriber declares). Hard-coding the Acmeco hierarchy is refused.

Q3 - IS THE TWO-TIER ANTI-RELAXATION DESIGN FAITHFUL TEETH, OR DOES IT AMEND ANYTHING

Disposition: FAITHFUL TEETH on REG-FEDERATION-COORDINATION-001 + ACT-007:s3/s4. It amends nothing. Approved as machinery under FRAMEWORK s.7.

This is the heart of the matter and I have tested it hardest against the anti-Henry-VIII limit. FRAMEWORK s.7 permits subordinate instruments and machinery but forbids any that would "amend, disapply, or expand this Act, the constitution, any primary Act, or the assent rule." The question is therefore: does a loader that (a) loads canon Tier-1 + subscriber Tier-2 together, (b) cascades most-specific-first, (c) makes canon win every tie, and (d) VOIDS at load any local rule that relaxes or contradicts a canon floor, change any statute, or merely enforce one?

It enforces. Point by point. Canon-precedence is ACT-007:s3 in terms ("local law does not override the canonical lawpack"). The void-at-load of a relaxing local rule is the mechanical face of the same words: a local rule that relaxed a canon floor WOULD be a local override of canon, which s3 already forbids absent a Privy order or Principal assent, and which REG-FEDERATION-COORDINATION-001 already lists under "overriding canonical without the s6 route." The loader does not enact "local law may only add restrictions." That proposition is the existing law's necessary consequence: if local law may not override canon (s3) and floors are canon, then local law may tighten but never loosen a floor. The loader is the deterministic clerk that reads that consequence off the loaded instruments, exactly as REG-KERNEL-001 contemplates ("the kernel is the only smart enforcement point... every denial cites the instrument").

I attach three conditions that keep it inside s.7 and inside VJS-ACT 10, and I would refuse the design WITHOUT them:

(i) The void operates only on the OFFENDING TIER-2 RULE, never on canon, and never on a Sovereign-assented record. "Void at load" here means: the local relaxation has no force; the canon floor stands. It is anti-relaxation, not a power to strike assented law. Where a Tier-2 rule that purports to relax a floor itself sits on a valid assent source, VJS-ACT 10 forbids voiding or blocking it; the disposition there is to refuse it RUNTIME EFFECT against the floor and to route it for correction, surfacing the conflict, never silently excluding it. The loader must distinguish "this local rule yields to the canon floor" (always permitted) from "this local record is erased" (forbidden for an assented record). I make this explicit in the directives.

(ii) The floors the loader treats as canon-precedence floors are the floors canon actually enacted. The loader may not invent a floor. It enforces precedence over the instruments it loads; it does not legislate new ones. A floor exists because a canon instrument enacted it, and the denial cites that instrument (REG-KERNEL-001).

(iii) The void must never reach an entrenched guarantee by side-effect. FRAMEWORK s.25 entrenches the assent floor, apex-singleness and the protective floor against amendment "by statutory instrument, kernel, agent." A loader is kernel machinery. It must therefore be incapable, by construction, of an outcome that disapplies any entrenched guarantee. Because the loader only ever RESOLVES IN FAVOUR OF the canon floor (it tightens, never loosens, the realm's protection), it cannot in its nature weaken an entrenched limb. I record that as the reason it survives s.25, and I bind the build to preserve that property.

On those conditions, the two-tier anti-relaxation overlay amends no statute, no bench, no tier, no jurisdiction rule and no assent rule. It is faithful machinery under s.7. Approved.

Q4 - RUNTIME GovernedLoadBearingAct ActionKind + A RUNTIME/OPERATIONS DOMAIN: ADDITIVE OR CONSTITUTIVE

Disposition: ADDITIVE. Approved under s.7, with the wiring finished and the domain named.

As the record shows, `GovernedLoadBearingAct` is already a declared `ActionKind` variant. Adding a variant to a closed enum, or wiring an already-declared variant into the route classifier, changes no statute, creates no court, alters no tier, and touches no assent rule. It is the same species of additive machinery REG-KERNEL-001 expressly blesses ("This Regulation is additive machinery; it amends no statute, no bench, no tier, and no assent rule"). A "runtime/operations" law domain distinct from code-governance is likewise a partition of the SUBJECT MATTER the kernel routes, not a partition of constitutional authority. It does not create a new sovereign, a new apex, or a new assent path. apex-singleness (FRAMEWORK s.11/s.25) is untouched: a runtime domain does not get its own Supreme sitting; runtime constitutional questions route up exactly as [2026] VJS-SC 4 holds for everything else.

I add one limiting condition, again refusing the piece without it: a runtime/operations DOMAIN is a routing-and-classification concept only. It may not become a backdoor by which runtime acts escape the ordinary court triggers or the ordinary assent floor. A runtime act that raises a first-impression fork still routes to court (ACT-002:s6); a runtime act bearing on an assented record is still protected by VJS-ACT 10. The domain sorts subject matter; it does not lower any floor. On that condition, additive. Approved.

Q5 - THE DETERMINISTIC SUBMIT-DECISION PERMIT API

Disposition: ADOPT. It is REG-KERNEL-001's deterministic clerk applied to runtime acts. Disposed GRANT or ROUTE_FOR_CORRECTION, never hard-DENY for an assented act, per VJS-ACT 10.

A submit-decision API that returns in under 100ms, calls no model, and whose GRANT carries `law_source[]` (the instrument that authorised it) is REG-KERNEL-001 almost verbatim: "the kernel never deliberates... the bench is probabilistic, the machinery deterministic," and "every denial cites the instrument that caused it." The `law_source[]` on the GRANT is the affirmative twin of the existing "name the instrument that caused every denial" duty: here we name the instrument that GRANTS. That is good, and I commend it; a permit that cannot say which law lets it through is as defective as a denial that cannot say which law stopped it.

On disposition I am firm and I bind it. The API governs RUNTIME ACTS, and a runtime act may rest on a Sovereign-assented record (a subscriber's assented local instrument, or a canon instrument). VJS-ACT 10 s.1 forbids any subordinate gate from voiding OR BLOCKING such a record; its defects are "ALWAYS routed for correction." A submit-decision gate is exactly such a subordinate gate. It therefore has two terminal dispositions where an assented record is in play: GRANT, or ROUTE_FOR_CORRECTION. A hard-DENY of an assented runtime act is constitutionally unavailable. I note carefully what VJS-ACT 10 does NOT forbid: it "forbids only voiding and blocking; it does not prevent a defect being surfaced, flagged, or routed for correction." So the API may, and must, surface the conflict, refuse to mint a clean GRANT, and route. What it may not do is silently swallow the act or return a bare DENY that blocks an assented record with no route. For NON-assented runtime acts the ordinary blocking dispositions remain available; the assented-record floor protects only what the Sovereign (or, in a subscriber's domain, the subscriber's valid assent source within s.23's meaning) has assented to, and enlarges nothing (VJS-ACT 10 s.2). I adopt the API on that binding.

Q6 - BUILD NOW OR RESERVE

Disposition: BUILD NOW, the generic primitives, with the vocabulary genericised. Position C's reservation is REJECTED.

I have weighed Position C with real sympathy, because its instinct, do not bake one tenant's shape into the substrate, is the correct instinct and is the very instinct this opinion enforces. But C mislocates the danger. The thing that would bake a tenant's shape into the substrate is the VOCABULARY (org/ws/matter/flow/step), and I have already refused that under Q2. Once the vocabulary is genericised to subscriber-named dimensions, there is no tenant shape left in the substrate to "bake." The runtime ActionKind is already in canon. The domain is a generic subject-partition. The API is REG-KERNEL-001's own clerk. Reserving these "until grounded in more than one subscriber" would reserve GENERIC machinery on the ground that only one subscriber has so far asked for it, which is not a principled test: canon does not wait for a quorum of subscribers before it builds a generic primitive any subscriber could use. The "more than one subscriber" test earns its keep precisely against a SPECIFIC, vocabulary-bearing design; against a genericised one it does no work and only leaves the federation principle toothless.

There is also a duty-of-care dimension. The federation principle is already law (REG-FEDERATION-COORDINATION-001, ACT-007:s3/s4) and is, today, enforced only by the agent rung, not at load. A subscriber's local law that relaxed a canon floor would, today, simply load. That is a live gap in the realm's protective floor, and the keystone closes it. To reserve the closure when the generic, floor-respecting design is in hand would be to leave a known floor-relaxation gap open for want of a second customer. I will not do that.

I therefore adopt POSITION A, AS QUALIFIED: adopt as substrate, genericise the vocabulary, build now, every piece bound to the conditions above. I reject Position B (it draws the canon/subscriber line at "code," which no instrument supports) and Position C (it reserves generic machinery against a danger I have already removed by refusing the vocabulary).

A WORD ON THE SAME-AUTHOR HAZARD (F6)

I have not adopted the subscriber's framing wholesale, and the record will show where I declined it. I refused the Acmeco vocabulary outright (Q2). I narrowed Q4 to what is genuinely additive once the existing enum variant is accounted for. I attached floor-preserving and assent-preserving conditions to Q3 and Q5 that the submission did not itself spell out. The inclusion gate did real work here, and it cut against the subscriber on the one piece, the vocabulary, that would have imported a tenant into the substrate. That is the editorial pen functioning as it should.

DIRECTIVES

D1 (actor: lexby). Build the two-tier overlay lawpack loader extending vjs-lawpack::LawpackLoader: load canon Tier-1 floors together with a subscriber's Tier-2 local lawpack, cascade most-specific-first, resolve every conflict in favour of the canon floor (ACT-007:s3, REG-FEDERATION-COORDINATION-001), and enforce ANTI-RELAXATION at load. A Tier-2 rule may only ADD restriction; a Tier-2 rule that relaxes or contradicts a canon floor is denied runtime effect against that floor, and the denial cites the canon instrument (REG-KERNEL-001).

D2 (actor: lexby). The anti-relaxation operation yields the offending TIER-2 rule to the canon floor; it never voids or blocks canon, and never erases a record bearing a valid assent_source (s.23). Where a Tier-2 rule that purports to relax a floor itself bears a valid assent source, do not void or block it: refuse it effect against the floor, SURFACE the conflict, and ROUTE_FOR_CORRECTION (VJS-ACT 10 s.1). By construction the loader must be incapable of any outcome that disapplies an entrenched guarantee (FRAMEWORK s.25); it only ever resolves toward the floor.

D3 (actor: lexby). Build the generic entity-scope primitive as an ordered map of subscriber-named dimension to value, most-specific-first ordering supplied by the subscriber. Do NOT introduce "org", "ws", "matter", "flow", "step" or any subscriber business noun into any canon type, record, or comment except to name them as EXCLUDED. The subscriber supplies its own dimension names in its Tier-2 (REFUSED into canon: the Acmeco hierarchy; ACT-007:s4, FRAMEWORK s.21).

D4 (actor: lexby). Finish wiring the existing `GovernedLoadBearingAct` ActionKind into the route classifier and name a generic runtime/operations law domain as a subject-matter partition only. It confers no new court, no new apex, no new assent path; a runtime fork still routes to court (ACT-002:s6) and a runtime act on an assented record stays under VJS-ACT 10. Record it as additive machinery under FRAMEWORK s.7 (per REG-KERNEL-001's own additive-machinery recital), citing this order.

D5 (actor: lexby). Build the deterministic submit-decision permit API as REG-KERNEL-001's clerk for runtime acts: under 100ms, no model, GRANT carries law_source[] (the authorising instrument). Terminal dispositions for an act resting on an assented record are GRANT or ROUTE_FOR_CORRECTION only; a hard-DENY that voids or blocks an assented runtime act is unavailable (VJS-ACT 10 s.1). Non-assented acts retain the ordinary blocking dispositions. The API enlarges no record's force (VJS-ACT 10 s.2).

D6 (actor: lexby). Land the whole keystone under ACT-CONSOLIDATION-FRAMEWORK:s7 as subordinate machinery, citing this order as authority, and confirm in the build record that it amends, disapplies or expands no statute, no constitution, no primary Act, no bench, no tier, no jurisdiction rule and no assent rule (s.7 anti-Henry-VIII; s.25 entrenchment). Where a future subscriber wishes a runtime act to escape a canon floor, the lawful route is the s.6/ACT-007:s3 route (a Privy order or Principal assent), never a loader relaxation; reserve nothing further, as the generic primitives are grounded now.

So ordered.

Marchmont J.
Privy Council, Vibe Justice System
Constituted per [2026] VJS-SC 2 D2
