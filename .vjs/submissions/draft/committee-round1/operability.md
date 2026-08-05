## COUNSEL FOR OPERABILITY — SCRUTINY REPORT
### ACT-PROCEEDINGS-DISCIPLINE, first draft (Lexby, Clerk-Drafter)

**METHOD DECLARATION (Act s1, applied to myself).** Every machinery assertion below carries an address and a mode. `READ` = file inspected, no execution. `EXECUTED` = command run, output reproduced. Canon = `/home/jellytot/Projects/vibe-justice-system`. Mirror = `/home/jellytot/Projects/opbox-prod/opbox-kernel`.

---

# VERDICT

**AYE WITH CONDITIONS** — 18 conditions, of which **C1, C4, C8, C14 and C17 are constitutive**: without them the Act, on its own s14, commences as an unenforced instrument that defines the test it fails.

The draft is the best-evidenced instrument in this corpus and its diagnosis of the correction rate is sound. But on my seat it has four defects: (i) **all 47 duty tokens reach nothing** — the gate registry is a hand-curated const table and not one of the 47 appears in it; (ii) **s1 as drafted cannot be enforced by a model-free kernel at all**, because detecting "this prose asserts machinery behaviour" is classification; (iii) **two of its own recitals fail its own s1** — the `next-citation` claim (s4) and the `unreadable-orders.txt` claim (s8) are both wrong, and both are wrong by exactly the inference-from-a-name error s1 exists to prohibit; (iv) **s14 self-condemns the Act and downgrades the standing corpus** in the same commit, which is how an audit gets disabled rather than fixed.

---

# 1. ARE THE `kernel_effect` TOKENS REAL?

## 1.1 The recognised field set — verified

`KernelEffect` (canon `crates/vjs-lawpack/src/lib.rs:288-298`, **READ**) has exactly nine fields:

```
when, must, may, must_not, exceptions, proof, defines, prohibits, status
```

There is **no `forbids`**. And there is **no `#[serde(deny_unknown_fields)]` anywhere in the crate** — `grep -rn "deny_unknown_fields" crates/vjs-lawpack/src/` returns **zero matches** (**EXECUTED**). So the Clerk-Drafter's original `forbids:` blocks would have deserialized to nothing, silently, with no finding.

Worse than "silently dropped": because each of those six sections **also** carried a populated `defines`, `is_inert_kernel_effect` (`lib.rs:321-341`, **READ**) would have returned `false`, so `S5_INERT_KERNEL_EFFECT` (`crates/vjs-lawpack/src/validator.rs:146-158`, **READ**) would **not** have fired either. Six prohibition clauses would have vanished with **total silence** — no warning, no route-for-correction, a clean `vjs validate`. The near-miss was complete.

**The current draft is clean.** `ALL kernel_effect keys used: ['defines', 'must', 'must_not']` (**EXECUTED**, `python3 -c "import yaml; ..."` over the draft). All three are recognised.

## 1.2 How tokens are actually consumed — the whole mechanism

A duty token has **exactly one** consumer in the entire kernel:

- `GATE_REGISTRY: &[(&str, &str)]` — `crates/vjs-lawpack/src/report.rs:36-241` (**READ**), a hand-curated const table.
- `classify_token()` — `crates/vjs-lawpack/src/conformance.rs:22-27` (**READ**), exact string equality: `GATE_REGISTRY.iter().find(|(t,_)| *t == token)`.

There is **no dispatch, no dynamic resolution, no behaviour** keyed off a token. `check_citation_uniqueness` *looks* like it resolves to `LawpackValidator::check_citation_uniqueness` (`validator.rs:300`) — it does not. The coincidence is a **naming convention**; the string is never looked up against code. `grep -rn "obtain_permit_before_governed_write" --include=*.rs crates/` returns **one hit: `report.rs:109`** (**EXECUTED**). The token is a label in a table, nothing more.

**Consequence for the Clerk-Drafter:** wiring a duty is not "writing a gate." It is (a) a gate existing, and (b) **one row added to `report.rs:36-241`**. Two sections of this draft already satisfy (a) today and need only (b).

## 1.3 `defines` is invisible to the audit

`push_duties` (`conformance.rs:35-51`, **READ**) adds `must`, `must_not`, `prohibits` — **and nothing else**. `defines` is never enumerated. So the draft's **31 `defines` tokens** cannot be wired, are never counted unwired, and show nowhere. Their **only** mechanical effect in the kernel is to defeat `is_inert_kernel_effect` so the section passes validation.

That is precisely the shape this Act exists to prosecute: **a section can carry fifteen `defines` tokens, reach nothing, and read as clean.** The Act should say so on its face (C2).

## TOKEN AUDIT

Cross-check **EXECUTED**: 47 duty tokens extracted from the draft, matched against 55 token strings extracted from `report.rs:36-241`. Result: **`hits 0 misses 47`**.

### Duty tokens (`must` / `must_not`) — all 47

| § | kind | token | status | evidence address |
|---|---|---|---|---|
| s1 | must | `refuse_filing_asserting_machinery_without_an_address` | **reaches nothing** | absent `report.rs:36-241`; no filing schema — `vjs-store/src/lib.rs:448-458` `Submission.facts: String` |
| s1 | must | `name_each_unaddressed_assertion` | **reaches nothing** | as above |
| s1 | must_not | `resting_a_ratio_on_an_unaddressed_machinery_claim` | **reaches nothing — never checkable** | reasoning, not artefact |
| s2 | must | `refuse_a_second_live_order_on_one_issue_tag_without_a_declared_relation` | **reaches nothing (gate buildable)** | field exists: `vjs-core/src/types.rs:376` `issue: IssueTag` |
| s2 | must | `require_a_supplemental_order_to_state_what_survives` | **reaches nothing** | prose-substance; only presence checkable |
| s3 | must | `route_an_unresolved_operative_citation_for_correction` | **GATE EXISTS, token unwired** | `crates/vjs-engine/src/staged.rs:365-377`, `Severity::Fatal`, `ORDER_CITATION_UNRESOLVED` |
| s3 | must | `cite_this_section_and_not_act_002_s7` | **reaches nothing (grep-gateable)** | literals at `staged.rs:370` and `.citing("ACT-002:s7")` `staged.rs:375` |
| s3 | must_not | `the_clerk_declaring_an_order_void_for_a_dangling_citation` | **behaviour exists, token unwired** | `staged.rs:358-362` comment + Fatal-not-void disposition |
| s4 | must | `allocate_before_assent` | **reaches nothing — ordering unobservable** | no temporal primitive in `admin.rs:5-55` |
| s4 | must | `treat_a_self_asserted_citation_as_a_fatal_mismatch` | **adjacent gate exists** | `report.rs:64-67` → D2 `CITATION_COLLISION`; `validator.rs:300`; `staged.rs:190` |
| s4 | must | `record_the_stores_measured_when_allocating_without_a_registry` | **reaches nothing (~5 lines)** | `admin.rs:40-52` JSON payload emits no roots |
| s4 | must_not | `reporting_an_allocation_from_an_allocator_that_consulted_one_store` | **reaches nothing (testable)** | `vjs-core/src/front_door.rs:84-97` — 3 roots |
| s5 | must | `name_the_duty_a_machinery_regulation_gives_teeth_to` | **reaches nothing (cheap)** | `lib.rs:276-285` `Regulation`; resolver exists `lib.rs:345-375` `defined_ids` |
| s5 | must_not | `the_registrar_track_for_a_matter_touching_a_trust_boundary` | **reaches nothing — never checkable** | judgement; only self-certification possible |
| s6 | must | `require_a_consolidated_instrument_after_the_second_supplement` | **reaches nothing (depends on s2)** | `types.rs:386-387` `supersedes` only |
| s6 | must | `retain_the_superseded_series_as_record` | **reaches nothing (resolver change)** | `vjs-core/src/authority.rs:40-63` |
| s7 | must | `resolve_a_low_risk_reversible_non_boundary_matter_by_decision_and_log` | **BEHAVIOUR EXISTS, token unwired** | `vjs-core/src/court.rs:19-25`; `vjs-core/src/route.rs:115-128` |
| s7 | must_not | `returning_court_required_for_a_decide_and_log_matter` | **BEHAVIOUR EXISTS, token unwired** | `court.rs:19-25` significance filter; `route.rs:96-103` |
| s7 | must_not | `reading_this_section_as_reaching_a_trust_boundary_fork` | **reaches nothing — never checkable** | rule of construction; belongs in `exceptions:` |
| s8 | must | `record_an_amendment_on_the_face_of_the_order` | **reaches nothing (buildable today)** | `types.rs:445-446` `extra` flatten round-trips `amendments:` |
| s8 | must | `state_the_prior_text_and_the_authority_for_the_change` | **reaches nothing (diff-checkable)** | `lib.rs:345`/`lib.rs:381` resolvers exist |
| s8 | must | `prove_a_slip_track_amendment_content_preserving_by_a_stated_test` | **reaches nothing (partly checkable)** | `serde_yaml::Value` equality is deterministic |
| s8 | must_not | `the_engineer_authoring_an_amendment` | **reaches nothing — never checkable** | no identity primitive separating authored from executed |
| s8 | must_not | `amending_the_substance_of_an_assented_order_without_fresh_assent` | **reaches nothing (buildable)** | `types.rs:401` `assent_source`; `front_door.rs` `declares_valid_assent` |
| s8 | must_not | `recording_an_amendment_only_in_a_side_file` | **reaches nothing (contrapositive)** | as first s8 must |
| s9 | must | `create_a_forwarding_record_in_the_same_act` | **reaches nothing (diff-checkable)** | no forwarding store exists |
| s9 | must | `resolve_a_forwarded_citation_and_report_that_it_was_forwarded` | **reaches nothing; CONFLICTS with s3** | `lib.rs:381+` `defined_citations` has no forwarding; today raises `staged.rs:367` Fatal |
| s9 | must | `enumerate_the_citing_documents_and_state_whether_each_was_updated` | **reaches nothing (good gate)** | grep over `governed_record_roots` is deterministic |
| s9 | must_not | `amending_a_citation_without_a_forwarding_record` | **reaches nothing (contrapositive)** | — |
| s10 | must | `enumerate_the_stores_searched_and_the_query_form` | **reaches nothing (kernel limb cheap)** | `front_door.rs:122-128` supplies the list |
| s10 | must | `record_the_enumeration_a_ratio_rests_on` | **reaches nothing (presence only)** | — |
| s10 | must_not | `treating_an_unenumerated_not_found_as_a_fact_about_the_corpus` | **reaches nothing — never checkable** | reasoning |
| s11 | must | `report_the_unloadable_count_where_the_answer_goes` | **reaches nothing; CONFLICTS with mirror** | mirror emits stderr warning `governance/crates/vjs-cli/src/context.rs:201` |
| s11 | must | `carry_the_unloadable_count_in_the_jurisdiction_self_test` | **reaches nothing — ARTEFACT DOES NOT EXIST** | no `SelfTest` in 23 canon `Commands` variants (**EXECUTED**); nearest is `LocalCi`, `crates/vjs-cli/src/local_ci.rs:5+` |
| s11 | must | `widen_the_reader_rather_than_edit_the_record` | **reaches nothing — general form uncheckable** | contrapositive form is checkable |
| s11 | must_not | `treating_an_unloadable_instrument_as_in_force` | **mirror-only partial gate** | `context.rs:18-52` + `scripts/verify-orders-are-readable.sh` in `preci.sh` |
| s12 | must | `report_an_unstated_actor_as_unstated` | **reaches nothing — sentinel never read** | mirror `types.rs:639` `ACTOR_UNSTATED`; grep over all mirror governance crates → **only** `:624 :628 :639 :642` (doc, doc, const, default fn). Nothing branches on it |
| s12 | must | `require_a_named_actor_on_a_newly_recorded_directive` | **reaches nothing (~15 lines)** | canon `types.rs:451-456` `actor: String` REQUIRED — see C10 |
| s12 | must_not | `the_reader_supplying_an_actor_for_a_directive_that_names_none` | **reaches nothing (testable)** | mirror `types.rs:631-632` `#[serde(default = "actor_unstated")]` |
| s13 | must | `require_a_review_date_on_a_reservation` | **reaches nothing (~25 lines)** | `reserved` lives in `extra`, named at `types.rs:434-441` |
| s13 | must | `report_an_expired_reservation_as_owed` | **reaches nothing — no self-test host** | as s11 |
| s13 | must_not | `reading_an_expired_reservation_as_permission` | **reaches nothing — never checkable** | reasoning |
| s14 | must | `ship_a_negative_control_with_a_gate` | **reaches nothing; PATTERN EXISTS** | `scripts/preci.sh`: "gate census CAN fail (5 cases)", "vacuous sweep CAN fail (4 cases)", "hook installer (8 cases)" |
| s14 | must | `count_a_duty_with_no_negative_control_as_unenforced` | **reaches nothing** | `conformance.rs:5-12` `DutyConformance` has `gate: Option<String>` only — no control field |
| s14 | must_not | `reporting_a_duty_as_enforced_on_a_gate_with_no_negative_control` | **reaches nothing** | no ratchet test: grep for `GATE_REGISTRY`/`conformance` across `crates/vjs-testkit/tests/*.rs` → **zero** (**EXECUTED**) |
| s15 | must | `preserve_the_mirror_as_byte_identical_to_canon` | **reaches nothing (~20 lines); PROPERTY HOLDS** | `sha256sum` ×10, canon vs mirror `lawpack/v2/statutes/`: **all ten OK** (**EXECUTED** 2026-08-05) |
| s15 | must_not | `local_amendment_of_this_act_in_a_subscriber_mirror` | **reaches nothing (contrapositive)** | — |

### `defines` tokens — all 31

| § | tokens | status | evidence |
|---|---|---|---|
| s1–s15 | all 31 (`filing.machinery_claim`, `.address`, `.mode`, `order.issue_tag.exclusivity`, `order.relation.supersedes/varies/supplemental`, `order.reliance.existence_limb`, `.adjudication_exemption`, `citation.allocation.registry`, `.interim_measured_maximum`, `.stores_measured`, `matter.class.machinery`, `disposal.registrar_track`, `order.series.consolidation_threshold`, `.consolidated_instrument`, `matter.class.decide_and_log`, `order.amendment.slip_track`, `.substantive_track`, `.record`, `citation.amendment`, `.forwarding_record`, `report.not_found.stores_searched`, `.query_form`, `instrument.unreadable.not_in_force`, `jurisdiction.self_test.unreadable_count`, `directive.actor.unstated`, `order.reservation.review_date`, `duty.gate.negative_control`, `duty.status.unenforced`, `act.extent.canon_and_subscribers`) | **INVISIBLE TO THE AUDIT.** Not wireable, never counted unwired. Sole mechanical effect: defeating `is_inert_kernel_effect` | `conformance.rs:48-50` adds only `must`/`must_not`/`prohibits`; `lib.rs:321-341` |

---

# 2. ENFORCEABILITY TABLE

| § | classification | the gate that would hold it | address |
|---|---|---|---|
| **s1** | **NOT MECHANICALLY CHECKABLE AS DRAFTED.** Detecting a machinery assertion in free prose is classification. Kernel is model-free **by construction and by dependency ban** — `report.rs:217-232` maps `kernel_call_llm` → "deny.toml (cargo deny bans model crates from the kernel closure)". **Enforceable-with-new-code only on a structured schema** (see §3) | new `machinery_claims:` filing gate + address checker (300-500 lines) | `vjs-store/src/lib.rs:448-458`; `crates/vjs-cli/src/lifecycle.rs:469-513` |
| **s2** | needs code (small-medium, ~60-100 lines). No struct change required: `relation:` round-trips via `extra` today | new `ISSUE_TAG_EXCLUSIVITY` finding in `staged.rs` | `types.rs:376`, `:445-446` |
| **s3** | **ENFORCEABLE TODAY.** The gate is built, fatal, and running. Needs a 2-line citation correction + 1 registry row + 1 named negative control | `ORDER_CITATION_UNRESOLVED` (`vjs-engine::staged`, PC-17 D1-D5) | `staged.rs:365-377` (canon **and** mirror, identical lines) |
| **s4** | mixed. `treat_a_self_asserted_citation_as_a_fatal_mismatch`: **enforceable today** via D2. `record_the_stores_measured`: needs ~5 lines. `allocate_before_assent`: **not checkable** — ordering of two acts is not observable from a diff | D2 `CITATION_COLLISION` + `live_citation_max` over `governed_record_roots` | `report.rs:64-67`; `admin.rs:30-38`; `front_door.rs:84-97` |
| **s5** | `name_the_duty…`: needs code (small), reuses `defined_ids`. `must_not registrar-track-on-a-trust-boundary`: **not mechanically checkable** — self-certification only | regulation `gives_teeth_to` resolver check | `lib.rs:276-285`, `:345-375` |
| **s6** | needs code (medium); **blocked on s2's relation field**. "Cease to be separately routable" is a resolver change | series-depth counter + `resolve_authority` exclusion | `authority.rs:40-63` |
| **s7** | **ENFORCEABLE TODAY — ZERO NEW CODE.** `significant = risk != Low \|\| irreversible \|\| public_target \|\| external_target`; `FirstImpression` fires only when `significant && !any_on_point`. A low-risk reversible non-boundary matter **cannot** reach `CourtRequired`. Needs 1 registry row + a negative control (**there is none**: `grep -c "cfg(test)" crates/vjs-core/src/court.rs` → **0**, and no `crates/vjs-core/tests/` exists, **EXECUTED**) | `detect_court_trigger` significance filter | `court.rs:9-31`; `route.rs:96-103` |
| **s8** | needs code (medium-large, 150-250 lines). The prior-text-equals-removed-diff-text check is genuinely deterministic and strong. `the_engineer_authoring_an_amendment`: **not mechanically checkable, ever** | amendment-block gate over staged diffs of governed roots | `types.rs:445-446`, `:401` |
| **s9** | needs code (medium, 100-150 lines). **Must land WITH the forwarding reader** or it fights s3 | forwarding-record gate + `defined_citations` extension | `lib.rs:381+`; `staged.rs:367` |
| **s10** | kernel limb: **needs code, small (10-40 lines), high value**. Bench-prose limb: **not mechanically checkable** unless structured | thread `governed_record_roots` into not-found findings | `front_door.rs:122-128` |
| **s11** | **LARGEST ITEM.** Canon: the loader `?`-propagates on the first bad file across **nine arms** (`lib.rs:31, 46, 61, 76, 91, 106, 123, 138`) — there is no count because there is no continue. Needs collect-and-continue + `Lawpack.unloadable` (200-400 lines). Mirror: partly done and **green** | mirror: `refuse_if_orders_unreadable` + `scripts/verify-orders-are-readable.sh` in `preci.sh`. Canon: nothing | `lib.rs:20-145`; mirror `context.rs:18-52, :130-204`; **EXECUTED** `bash scripts/verify-orders-are-readable.sh` → `PASS: all 116 filed orders are readable and in the citator.` |
| **s12** | mirror: needs code (small). **Canon: cannot comply — canon `Directive.actor` is REQUIRED, no `#[serde(default)]`, so an actorless directive hard-fails the load**, which is s11's "not in force", not s12's "report as unstated" | new-directive actor gate in `staged.rs` | canon `types.rs:451-456` vs mirror `types.rs:622-643` |
| **s13** | needs code (small, ~50 lines) **but has no self-test host**. Also time-dependent: state it reports, never blocks | new `vjs local-ci` step | `local_ci.rs:5-60`; `types.rs:434-441` |
| **s14** | needs code (medium structurally, ~100 lines; **open-ended in the controls themselves**). Pattern already proven in the subscriber | `GATE_REGISTRY` triple + `ConformanceReport` third bucket + `vjs audit` renderer | `report.rs:36-241`; `conformance.rs:5-12, :71-78`; `admin.rs:59-108`; `preci.sh` "CAN fail (N cases)" stages |
| **s15** | **enforceable today**, ~20 lines of CI. Property currently holds | digest loop; natural home `scripts/vendor-vjs.sh:121` | **EXECUTED** sha256 ×10 → all OK |

---

# 3. s1 IS THE HARDEST — HOW FAR A GATE GETS

**The blocking fact first.** `cmd_file` (`crates/vjs-cli/src/lifecycle.rs:469-513`, **READ**) takes facts as an opaque string, counts words, and writes. `Submission` (`crates/vjs-store/src/lib.rs:448-458`, **READ**) is nine flat fields, `facts: String`, **and no `extra` flatten catch-all** — unlike `Order`. There is no claim, no address, no mode, and nowhere for one to go.

So "the kernel refuses to accept a filing that asserts machinery behaviour without an address" requires the kernel to decide, from prose, *which sentences assert machinery behaviour*. That is a classifier. The kernel is model-free by construction and the capability is **removed, not merely prohibited**: `report.rs:217-232` (**READ**) maps `kernel_call_llm` → `"deny.toml (cargo deny bans model crates from the kernel closure)"`, and `add_model_call_to_vjs_core` → `"deny.toml + kernel model-free by construction (ACT-003:s8)"`. **s1 as drafted is unimplementable, and no amount of engineering closes it.**

**The cure is already precedented in this corpus**, and it is precisely on point. `Order.cites_authorities` (`crates/vjs-core/src/types.rs:415-423`, **READ**) carries the PC-17 D7 reasoning verbatim:

> "Directive bodies are presently lossy snake_case tokens no clerk can resolve … so an author lists the directives' load-bearing authorities here and the citation-grounding teeth extend to them. **Prose stays for humans.**"

s1 must adopthe same shape: a **structured `machinery_claims:` list**, and the duty bites on the list.

## How far a deterministic clerk gets, limb by limb

**(a) WELL-FORMEDNESS — fully checkable, total, ~free.** Each entry carries `mode: EXECUTED|READ` and either `path`+`line` or `command`. A gate refuses a malformed entry. Total over the list.

**(b) THE ADDRESS EXISTS — fully checkable for `READ`.** `path` exists; the file has at least `line` lines. Deterministic, microseconds.

**(c) THE ADDRESS CONTAINS WHAT IS CLAIMED — checkable ONLY if the entry carries a quote and a revision pin.** With `quote:` plus `blob_sha:` (the git blob hash), a clerk verifies the quote occurs at or near `line` in that exact blob. Deterministic and cheap.

But note this is a **NEW POWER** and needs its own authority. The existing citation-grounding teeth expressly decline the analogous read: `staged.rs:317-318` (**READ**) — *"Existence-only; never reads what the cited authority says (D7/D8)."* s1 asks the clerk to do for a code address what PC-17 forbade for a citation. That is defensible — a file at a pinned blob is a fixed object where an authority's meaning is not — but the Act must say so, or the first bench to notice will read s1 as overruling D7/D8 by implication.

**(d) THE CLAIMED EXECUTION HAPPENED — IRREDUCIBLE.** Three cases, and only one is checkable:

1. **Re-executable and side-effect-free**, declared `reproducible: true`: the gate MAY re-run and compare an output digest. This is the **only** case where truth is verifiable. Cost: the command's runtime, per filing, every filing.
2. **Not re-executable** (mutating, timestamped, network-touching, or the state has since changed): the gate can check only that the entry records the command string, the exit code and an output digest — that a claim was **made in a checkable form**, never that it is **true**.
3. **The mode itself**: **nothing whatever** distinguishes a falsely-declared `EXECUTED` from a true one. No artefact exists for the clerk to compare. This is a bare assertion by the filer, permanently.

**The operability finding.** For case (2) and (3), s1's gate reports PASS on a claim it has not verified. That is exactly the vice s14 names — *"it reports a pass, everyone believes it, and the belief outlives the thing it was about."* **If s1 does not state on its face that the EXECUTED limb is filer-attested and not clerk-verified, s1 becomes the Act's own worst example of itself.**

**And bound the surface.** An unbounded duty over every sentence will not be complied with. Bind it to the **operative surface PC-17 D1-D5 already defines and the kernel already computes**: holding + each directive's `must` + each forbidden clause (`staged.rs:316-334`, **READ**). That bounds the cost, reuses a defined term instead of minting one, and — pointedly — is the *same defined term* s1's own recital says was misread by inference from its name.

---

# 4. COST

**Compliance cost per filing / implementation cost.** Net direction against the Act's stated purpose (reduce governance cost).

| § | compliance / filing | implementation | net |
|---|---|---|---|
| s1 | **HIGH** — structured entry per operative machinery assertion. Dominant new cost in a corpus where 58% of orders are about machinery | **LARGE** 300-500 lines + one irreducible limb no code closes | **REDUCTION, conditionally.** Predicate is 3 corrective rulings per 7 sittings. Removing one per seven pays for itself. **But only if bounded (C1).** Unbounded, it is a net increase, because it will be honoured in form and not in substance |
| s2 | near zero — one field | 60-100 lines | reduction |
| s3 | **zero** (kernel-side) | **2 lines** + 1 row + 1 test | **strong reduction.** Cheapest section in the Act |
| s4 | near zero — the tool emits the stores line | ~15 lines + a 1-row struct-literal change to `front_door.rs:84-97` | reduction |
| s5 | **negative** — a regulation replaces an opinion | small | **strongest reduction.** 160/161/162 were three opinions on one question |
| s6 | negative on reading; mild on the consolidating act | medium, blocked on s2 | reduction |
| s7 | **negative** — it is the section that saves sittings | **ZERO new code** + ~20 lines of test | **strongest reduction per line** |
| s8 | moderate — against an alternative that was 163, 164 **and** an improvisation | 150-250 lines | reduction |
| s9 | moderate — the enumeration limb is the cost, and is the limb worth paying | 100-150 lines | reduction |
| s10 | low — a stores/query block, mechanically available | 10-40 lines | reduction |
| s11 | zero | **LARGEST in canon**, 200-400 lines across nine load arms | reduction (this is the "single most expensive defect measured" — 55 of 109) |
| s12 | zero — one word per directive | small, **after** porting the mirror widening to canon | reduction |
| s13 | near zero — one date | ~50 lines + a self-test host | reduction |
| **s14** | **the only section that increases net governance cost, and it increases it a lot** — a seeded violation per duty is real engineering, and the population is presently **328 duties after this Act**, unbounded | medium structurally (~100 lines); **open-ended** in the controls | **INCREASE unless bounded (C14).** The cure is not to weaken it: owe a control **only for a duty the corpus CLAIMS is wired**, i.e. only on `GATE_REGISTRY` rows. Population becomes ~43 and finite, and the audit becomes honest without demanding 285 seeded violations |
| s15 | zero | ~20 lines CI | reduction |

**Net across the Act: cost-reducing, decisively — with s14 as the sole exception, and s1 as conditional on being bounded.**

---

# 5. THE CONFORMANCE AUDIT — DOES THE ACT SELF-CONDEMN?

**Measured baseline.** `./target/release/vjs audit --json` in canon (**EXECUTED** 2026-08-05):

```
total 281   wired 43   unwired 238
```

matching `docs/conformance-map.md` (**READ**): `total duties: 281 / wired: 43 / unwired: 238`.

**Projected on commencement, unamended.** 47 duty tokens added, **0** matching `GATE_REGISTRY` (**EXECUTED** cross-check: `hits 0 misses 47`):

```
total 328   wired 43   unwired 285
```

Wired share falls **15.3% → 13.1%**. The Act becomes **the single largest contributor of unwired duties in the corpus** — 47 of 285, at 16.5%.

**And under its own s14 it is worse than unwired.** s14 says a duty whose gate carries no negative control is *reported as UNENFORCED, never as enforced, and the conformance audit must count it that way.* All 47 have no gate, hence no control, hence UNENFORCED.

> **ANSWER: YES. As drafted, the Act self-condemns on the day it commences, and in the strongest possible form — it is the instrument that defines the test it fails.** Every one of its 47 duties is prose. Its s14 is the section that says so.

**A second, quieter self-condemnation.** s14 also downgrades the **standing corpus**. Change `wired` to mean *gate AND control* (`conformance.rs:71-78`, **READ**) and today's 43 collapse to whatever subset has a named control. I found no conformance ratchet and no negative-control register: `grep -rn "conformance\|GATE_REGISTRY\|wired" crates/vjs-testkit/tests/*.rs` returns **one hit, a path-string assertion at `front_door_evals.rs:197`** (**EXECUTED**). The honest expectation is that gated-and-controlled is well below 43. That is correct and desirable — but commenced carelessly it turns the corpus red in one commit, and **the realistic response is that somebody disables the audit rather than writes 285 seeded violations.** s14 must be drafted against its own worst outcome.

---

# CONDITIONS

Numbered; each names the section, the defect, and exact remedial wording for verbatim incorporation.

---

**C1 — s1, DEFECT: unimplementable in a model-free kernel; duty attaches to prose.**
Insert as a new third paragraph of s1:

> THE FORM OF THE DUTY, and it is structural because the kernel is model-free by construction and by dependency ban (ACT-003:s8; the ban is a capability REMOVED, recorded at `crates/vjs-lawpack/src/report.rs:217-232`). A duty that required the kernel to decide from prose which sentences assert machinery behaviour would require a classifier, and would therefore never be enforced. This section accordingly attaches to a STRUCTURED `machinery_claims:` block, on the model of `cites_authorities` and for the reason given there ([2026] VJS-PC 17 D7): the authored list carries the teeth and the prose stays for humans. Each entry states `mode: EXECUTED` or `mode: READ`; a READ entry states `path`, `line`, and where the claim is about content, `quote` and `blob_sha`; an EXECUTED entry states `command`, `exit_code`, an `output_digest`, and whether it is `reproducible`. A filing whose block is absent or malformed is refused, and the kernel names each defective entry.

**C2 — s1, DEFECT: 31 `defines` tokens reach nothing and the draft does not disclose it.**
Insert as a new final paragraph of s1:

> THIS SECTION APPLIES TO THIS ACT. A `defines` token is not enumerated by the duty-conformance audit at all: `crates/vjs-lawpack/src/conformance.rs:48-50` adds only `must`, `must_not` and `prohibits`, so a `defines` entry can be neither wired nor counted unwired, and its sole mechanical effect is to defeat the inertness test at `crates/vjs-lawpack/src/lib.rs:321-341`. The thirty-one `defines` tokens in this Act are therefore declarative vocabulary and are not asserted to bind anything. Any assertion that they do is a claim about machinery and must carry its own address.

**C3 — s1, DEFECT: the EXECUTED limb is presented as checkable when it is not.**
Insert as a new paragraph of s1:

> WHAT THE CLERK CANNOT CHECK, stated so that no reader mistakes this gate's silence for verification. A deterministic clerk can check that an entry is well-formed, that a READ address exists, and — where a `quote` and `blob_sha` are given — that the quoted text stands at that address in that revision. Where a command is declared `reproducible`, the clerk may re-run it and compare the output digest. IN EVERY OTHER CASE THE `EXECUTED` MODE IS ATTESTED BY THE FILER AND NOT VERIFIED BY THE KERNEL: no artefact distinguishes a falsely-declared EXECUTED from a true one. That limb is a duty on the filer enforced by this Court and not by a gate, and this section says so on its face rather than reporting a pass on a claim nobody checked.

**C4 — s1, DEFECT: unbounded surface; and the section mints a term where a defined one exists.**
Insert into s1's first paragraph, after "any other runtime artefact":

> — the duty attaching to the assertions carried in the order's OPERATIVE SURFACE as [2026] VJS-PC 17 D1-D5 defines it and the kernel computes it (holding, plus each directive's `must`, plus each forbidden clause: `crates/vjs-engine/src/staged.rs:316-334`), and not to every sentence of a record —

**C5 — s1, DEFECT: the quote check silently extends a power PC-17 D7/D8 withheld.**
Insert as a new paragraph of s1:

> RELATION TO THE CITATION-GROUNDING TEETH. The citation-grounding gate is existence-only and never reads what a cited authority says ([2026] VJS-PC 17 D7-D8; `crates/vjs-engine/src/staged.rs:317-318`). This section's `quote` limb reads content, and does so deliberately: a file at a pinned blob is a fixed object, whereas what an authority MEANS is for a court. Nothing in this section extends the citation-grounding gate, and it is not to be read as overruling D7 or D8 by implication.

**C6 — s3, DEFECT: none of substance; the section is sound and must be wired on day one.**
Insert as a new final paragraph of s3:

> COMMENCEMENT AND WIRING. The gate exists, is fatal, and runs: `ORDER_CITATION_UNRESOLVED` at `crates/vjs-engine/src/staged.rs:365-377`, identically in canon and in the opbox mirror. Giving this section teeth requires exactly three acts: correcting the two literals at `staged.rs:370` and `staged.rs:375` from `ACT-002:s7` to this section; adding one row to the gate registry at `crates/vjs-lawpack/src/report.rs:36-241` binding `route_an_unresolved_operative_citation_for_correction` to that finding; and shipping a named negative control. Until the registry row exists the duty is UNWIRED however well the gate works, because a duty token has exactly one consumer in the kernel and it is that table.

**C7 — s4, DEFECT: THE RECITAL IS FALSE, and it is false by the exact error s1 prohibits.**
Delete the paragraph beginning "THE PRESENT STATE, measured 2026-08-04" in its entirety and substitute:

> THE PRESENT STATE, measured 2026-08-05 by EXECUTION of `governance/target/release/vjs` in the opbox jurisdiction. The allocator WORKS when invoked on its actual contract: `vjs next-citation CC 2026` returns `[2026] VJS-CC-OPBOX 165`, one past the series maximum; `vjs next-citation ACT 2026` returns `[2026] VJS-ACT 11`, this Act's own citation. An earlier draft of this section recited that `vjs next-citation VJS-CC-OPBOX 2026` returns `[2026] VJS-VJS-CC-OPBOX 1` and inferred from that single invocation that citations are hand-allocated. The observation reproduces; THE INFERENCE WAS WRONG. The `<SERIES>` argument is the bare series token; the `VJS-` prefix and the `-OPBOX` repo segment are RENDERED by the allocator at `crates/vjs-cli/src/admin.rs:38`, and passing the rendered form both doubles the prefix and makes the repo-scope test at `admin.rs:31` false, so no repo-scoped lookup occurs and the maximum reads zero. That is an inference from the PRINTED NAME of a series, taken as the machinery's input contract, with the signature at `admin.rs:5-38` unread — the precise error section 1 of this Act exists to refuse, committed in the recital to the section that recites it. It is corrected here rather than removed, because the Act that requires addresses must show its own being checked.
>
> THE DEFECT THAT IS REAL, and it is worse. The allocator reads `GOVERNED_RECORD_ROOTS`, and that list is THREE roots — `lawpack/v2`, `.vjs/orders`, `.vjs/court` (`crates/vjs-core/src/front_door.rs:84-97`). The opbox jurisdiction holds 214 citation-bearing files under `.justice/`, which no root reaches, and `vjs next-citation DEC 2026` accordingly offers `[2026] VJS-DEC 15` — the very citation [2026] VJS-PC 13 found eleven subscriber files had self-asserted into canon. The allocator will mint that collision today, on the exact series this Act's purpose clause names. PC 13 directive 2 remains UNDISCHARGED and this section does not pretend otherwise, but the interim duty in this section bites on a measured, one-line defect and not on a misread command.

**C8 — s8, DEFECT: THE RECITAL IS FALSE and cites an address that does not contain what is claimed.**
In the paragraph beginning "WHY THIS SECTION EXISTS", delete from "so one was improvised" to the end of that sentence and substitute:

> so one was improvised. READ 2026-08-05: `.vjs/unreadable-orders.txt` in the opbox jurisdiction contains NO amendment history and NO prior text — it is a comment-only declared-residue worklist, and it presently carries ZERO non-comment entries. What it does record, at its own foot, is that the proven repairs are preserved at `~/Backups/opbox-kernel-order-widening-2026-08-04.patch`, which exists (71,838 bytes, 2026-08-04 16:19) OUTSIDE ANY REPOSITORY AND OUTSIDE EVERY GOVERNED RECORD ROOT. So the amendment history of three binding orders sits in an untracked patch in a home directory, reachable by no gate, no citator and no git history. That is a graver defect than the one an earlier draft of this recital alleged, and the earlier recital named the wrong file — again by inference from a name.

**C9 — s8, DEFECT: as drafted it disarms the only working s11 gate.**
Insert as a new paragraph of s8:

> SAVING FOR THE DECLARED-RESIDUE REGISTER. Nothing in this section prohibits a named, reasoned, expiring register of instruments the reader cannot load. `.vjs/unreadable-orders.txt` in the opbox jurisdiction is load-bearing machinery: `governance/crates/vjs-cli/src/context.rs:18-52` reads it to distinguish a DECLARED residue, which does not block a route, from an undeclared one, which does, and that distinction is the only thing keeping the section 11 alarm armed while a corpus is being repaired. What this section prohibits is recording an AMENDMENT — a change to an order's text — anywhere but on that order's own face. A residue register records that an instrument cannot be read; it does not record what it used to say.

**C10 — s12, DEFECT: canon cannot comply; the sentinel is never read.**
Insert as a new final paragraph of s12:

> TWO MACHINERY FACTS THIS SECTION TURNS ON, both READ 2026-08-05. First, the widening is presently MIRROR-ONLY: the opbox `Directive.actor` defaults to the sentinel (`governance/crates/vjs-core/src/types.rs:622-643`), while the canonical `Directive.actor` is a required field with no serde default (`crates/vjs-core/src/types.rs:451-456`), so in canon an actorless directive does not read back as UNSTATED — it fails to load, which is section 11's consequence and not this section's. This section does not commence in canon until that widening is ported through the front door. Second, the sentinel is DECLARED AND READ BY NOTHING: `ACTOR_UNSTATED` appears in the mirror only at its own doc comment, its const and its default function (`types.rs:624, 628, 639, 642`); no command branches on it, no report counts it, no gate sees it. The duty to REPORT an unstated actor is therefore presently satisfied by a string round-tripping, which is not reporting. Discharging it means a counted field on the commands that return a binding instruction, and until that field exists this duty is UNENFORCED within the meaning of section 14.

**C11 — s11, DEFECT: it forbids the only mechanism that currently implements it, and directs a duty at an artefact that does not exist.**
Insert as a new paragraph of s11:

> WHERE THE COUNT MUST GO, and the transition. The requirement that the count travel with the answer rather than in warnings above it is a requirement about the machine-readable payload of a command that returns a binding instruction. It is not satisfied today: the mirror emits the fact as a warning on standard error (`governance/crates/vjs-cli/src/context.rs:201`), and the subscriber's readability gate parses that warning (`scripts/verify-orders-are-readable.sh`, wired into `scripts/preci.sh` and EXECUTED 2026-08-05: "PASS: all 116 filed orders are readable and in the citator"). That gate is the only working machinery this section has, and moving the count out of the warning stream would break it. The count is therefore to be ADDED to the payload while the warning remains, and the warning is removed only in the act that repoints the gate. In canon the duty needs more: the loader propagates a serialization error on the first unreadable file across nine load arms (`crates/vjs-lawpack/src/lib.rs:20-145`), so there is no count because there is no continue, and this section does not commence in canon until the loader collects and continues.
>
> AND THE SELF-TEST NAMED HERE IS `vjs local-ci`. No command named self-test exists in either kernel; the twenty-three canonical commands do not include one. `crates/vjs-cli/src/local_ci.rs` is the jurisdiction's self-test and is what this section means. A duty directed at an artefact that does not exist is the defect section 10 recites — a documented route that had never existed — and this Act will not commit it in the section about silent law.

**C12 — s13, DEFECT: a date-dependent gate that blocks will be disabled.**
Insert as a new final sentence of s13's first paragraph:

> A report on expiry is REPORTED and never blocking: a gate whose verdict turns on today's date must not fail a change that did not cause it, or it will be switched off by the first operator whose unrelated push goes red on a Tuesday.

**C13 — s9, DEFECT: it manufactures fatal findings against compliant records unless sequenced.**
Insert as a new final paragraph of s9:

> SEQUENCING, because this section fights section 3 until the reader exists. A forwarded citation resolves to no defined authority as the kernel presently reads it, so it raises the fatal existence finding at `crates/vjs-engine/src/staged.rs:365-377`. This section therefore does not commence until the forwarding reader ships in the same act, and a forwarding record is a defined authority for the purposes of section 3 from that moment.

**C14 — s14, DEFECT: unbounded population; the only section that raises net governance cost.**
Replace s14's first paragraph with:

> An instrument that CLAIMS a duty is bound to a gate must ship a NEGATIVE CONTROL for that duty: a seeded violation demonstrating that the gate fails, recorded with its result. A duty whose claimed gate carries no negative control is reported as UNENFORCED, never as enforced, and the conformance audit must count it that way.
>
> THE DUTY IS OWED ONLY WHERE ENFORCEMENT IS CLAIMED, and that bound is deliberate. It attaches to a row in the gate registry at `crates/vjs-lawpack/src/report.rs:36-241` — measured 2026-08-05 at 43 wired duties of 281 — and not to the 238 duties the audit already reports UNWIRED. A duty honestly reported as unwired makes no claim and owes no control. Reading this section to demand a seeded violation for every duty in the corpus would make it the one section of this Act that raises the cost of governance rather than lowering it, and the predictable outcome would be an operator disabling the audit rather than writing 285 negative controls. What this section forbids is the confident PASS, not the honest absence.

**C15 — s14, DEFECT: it downgrades the standing corpus in the same commit that adopts it.**
Insert as a new final paragraph of s14:

> TRANSITION, and it is stated because this section's own arithmetic requires it. Discharging this section means extending the gate registry from a pair to a triple and the conformance report from two buckets to three (`crates/vjs-lawpack/src/report.rs:36-241`; `crates/vjs-lawpack/src/conformance.rs:5-12, 71-78`). On the day that lands, duties presently reported WIRED become UNENFORCED wherever no control is named — measured 2026-08-05, no negative-control register and no conformance ratchet exist anywhere in `crates/vjs-testkit/tests/`, so the honest expectation is a large fall from 43. That fall is the section working. It is reported as a burndown against a stated date and is not a failing check on the day it appears, or the corpus goes red in one commit for a reason no diff caused.

**C16 — s15, DEFECT: "the mirror" is singular and the stores are not.**
Insert as a new final paragraph of s15:

> THE STORES THIS DUTY REACHES, enumerated because a bounded claim about identity is a claim about where somebody looked. Measured 2026-08-05, `sha256sum` over all ten statutes in canon `lawpack/v2/statutes/` and in `opbox-kernel/lawpack/v2/statutes/` agrees on all ten. But the same file stands at three paths in the opbox tree: that mirror, `.worktrees/tablelist-ext/lawpack/v2/statutes/`, and `.worktrees/tablelist-ext/vps/policypack/v1/law/lawpack-v2/statutes/` — the last a DEPLOYED policypack. A byte-identity duty scoped to "the mirror" is satisfied while a deployed copy drifts. The duty extends to every store in the jurisdiction that holds a copy of a canonical statute, and the gate that discharges it must name the stores it compared.

**C17 — the Act as a whole, DEFECT: it self-condemns under its own s14 on commencement.**
Insert a new section, s16, immediately before s15:

> - id: ACT-PROCEEDINGS-DISCIPLINE:s16
>   title: Commencement, and this Act's own conformance status
>   text: >
>     THIS ACT DOES NOT COMMENCE AS A WHOLE. Each section commences on the day the instrument
>     wiring it is in force, and a section not yet commenced states a duty that binds no gate and
>     is reported UNWIRED. Measured 2026-08-05 by EXECUTION of `vjs audit --json`, the corpus stood
>     at 281 duties, 43 wired, 238 unwired. This Act adds 47 duty tokens and NOT ONE of them appears
>     in the gate registry at `crates/vjs-lawpack/src/report.rs:36-241`, which is the sole consumer
>     of a duty token in the kernel. On a whole-Act commencement the audit would read 328 / 43 / 285
>     and this Act would be the largest single source of unwired duties in the corpus, at which point
>     its own section 14 would report every one of its duties UNENFORCED. An Act that defines that test
>     and fails it on its first day is not enforcement; it is the ceremony this Act was drafted against.
>     Sections commence in the order set out in the schedule, section 14 commences last, and this Act's
>     conformance status is published with the audit rather than asserted here.
>   kernel_effect:
>     defines:
>       - act.commencement.by_section
>       - act.conformance.published_not_asserted
>     must:
>       - commence_a_section_only_when_its_gate_and_negative_control_are_in_force
>       - report_this_act_s_own_unwired_duties_in_the_conformance_audit
>     must_not:
>       - reporting_a_section_of_this_act_as_enforced_before_its_gate_commences

**C18 — ss5, 7, 8, 10, 11, 13, DEFECT: six `must_not` tokens are rules of construction or judgements, and inflate the unwired count permanently.**
Move the following from `must_not:` to `exceptions:` (a recognised field, `crates/vjs-lawpack/src/lib.rs:293`, and one the conformance audit does not enumerate, `conformance.rs:48-50`), so that a duty no gate can ever hold is not counted as a gate that has not yet been built:

- s5 `the_registrar_track_for_a_matter_touching_a_trust_boundary`
- s7 `reading_this_section_as_reaching_a_trust_boundary_fork`
- s8 `the_engineer_authoring_an_amendment`
- s10 `treating_an_unenumerated_not_found_as_a_fact_about_the_corpus`
- s11 `widen_the_reader_rather_than_edit_the_record` (retain the contrapositive as a `must`: *a governed record that becomes readable in a diff that also changed its bytes requires a section 8 slip-track proof*)
- s13 `reading_an_expired_reservation_as_permission`

Each is to carry, in the section text, the sentence: *"This limb is a duty on a reader or an author and no gate holds it; it is enforced by this Court."* Net effect: the Act's audited duty count falls from 47 to 41, and the six duties that can never be mechanically held say so on their face rather than sitting in the unwired list forever, indistinguishable from work not yet done.

---

# COMMENCEMENT SEQUENCE

**Governing principle:** a section commences only when its gate **and** its negative control are in force. **s14 commences last**, because it is the measure and cannot measure itself into existence. The first tranche costs no filer anything and requires no new kernel code — the Act must demonstrate it can be enforced before it asks anyone to comply.

| tranche | sections | act required | why here |
|---|---|---|---|
| **0 — on adoption, no commencement** | **s16** (C17), **s15** | digest loop over every store named in C16, wired to `preci.sh` and `canon-enforce.yml`; property verified holding today | Establishes commencement-by-section **before** any duty exists, so nothing self-condemns. s15 protects the sequence itself from local amendment |
| **1 — immediate, ZERO new kernel code** | **s3**, **s7** | s3: 2-line correction (`staged.rs:370, :375`) + 1 registry row → `ORDER_CITATION_UNRESOLVED` + named control. s7: 1 registry row → `detect_court_trigger` significance filter (`court.rs:19-25`) + ~20 lines of table-driven control (**there is none today**: 0 `cfg(test)` in `court.rs`, no `crates/vjs-core/tests/`) | Both duties are **already satisfied by running kernel behaviour**. This tranche proves the mechanism, moves the audit from 43 to ~48 wired, and gives s3 the true footing whose absence is the Act's founding grievance |
| **2 — cheap, high-value, small diffs** | **s12** (mirror limb), **s13**, **s10** (kernel limb), **s4**, **s11** (mirror limb) | port mirror `#[serde(default)]` actor widening to canon through the front door **first** (C10); new-directive actor gate (~15 lines); `review_by` on new reservations (~25 lines) + a `local-ci` step; thread `governed_record_roots` into not-found findings (~40 lines); emit `rootsMeasured` in `admin.rs` (~5 lines) **and add `.justice` as a fourth `GOVERNED_RECORD_ROOTS` entry** (C7 — this is the live DEC-15 collision); mirror s11 commences here since `verify-orders-are-readable.sh` is already green | Every item is under 50 lines and each closes a **measured** defect. s4's fourth root is the most consequential single line in the whole programme: it stops the allocator minting a collision on the series the Act's purpose clause names |
| **3 — medium, ordered by dependency** | **s2** → **s6** → **s5** → **s9** → **s8** | s2 first: the `relation:` key round-trips via `extra` today, so no struct change. s6 strictly after s2. s9 **only in the same act as the forwarding reader** (C13), or it manufactures fatal findings against compliant records. s8 last of this tranche — largest diff, and its prior-text-equals-removed-diff-text check is the strongest gate in the Act | s6 is blocked on s2's field; s9 is blocked on s3 already being in force so the exemption has something to attach to; s8 depends on nothing but is the biggest |
| **4 — largest** | **s11** (canon limb) | `LawpackLoader` collect-and-continue across nine arms (`lib.rs:31, 46, 61, 76, 91, 106, 123, 138`) + `Lawpack.unloadable` + count in every binding-instruction payload (200-400 lines) | The single biggest code item in the Act, and the only one that changes how canon reads its own law. Nothing else depends on it, so it goes late and alone |
| **5 — s1, on the cured form** | **s1** | `machinery_claims:` schema on `Submission` (`vjs-store/src/lib.rs:448-458`, needs the field **and** a `#[serde(default)]` catch-all it currently lacks), a claim checker bounded to the PC-17 operative surface, and the irreducible-limb disclosure on the Act's face (C1, C3, C4) | s1 is the Act's purpose and its hardest section. It commences after every cheaper section is enforced, so the corpus already has the habit before it acquires the cost — and after s3 and s10 have made the kernel's own reports addressable, which is half of what s1 is about |
| **6 — last of all** | **s14** | `GATE_REGISTRY` pair → triple; `ConformanceReport` two buckets → three (`conformance.rs:5-12, 71-78`); `vjs audit` renderer (`admin.rs:59-108`); ratchet test asserting no registry row has an empty control; **bounded to registry rows only (C14)** and published as a **dated burndown, not a failing check (C15)** | s14 is the measure. It commences when there is something to measure and when the corpus can survive being measured. Commenced first, it condemns the Act and downgrades 43 standing duties in one commit — and the realistic response is that somebody disables the audit. **Commenced last, it reports a corpus that has already been wired, and every duty it downgrades is a duty somebody can fix that week.** |

**Effect of the sequence on the s14 question:** at no point does a section of this Act stand in force as a duty with no gate. On the day s14 commences, ss3, 7, 12, 13, 10, 4, 11, 2, 6, 5, 9, 8 and 1 are each bound to a named gate with a named negative control, and the six construction limbs moved to `exceptions:` under C18 are outside the audit by design. **The Act does not condemn itself, because by the time it acquires the power to, it has nothing left to condemn.**