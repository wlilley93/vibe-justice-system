# STANDING COMMITTEE — ROUND 5, FULL BENCH

*Filed verbatim per the re-certification bench's direction; returned in-session 2026-08-05.*

**Instrument:** `/home/jellytot/Projects/vibe-justice-system/.vjs/submissions/draft/ACT-RECTIFICATION-COMMISSION.yaml`
**Fourth draft, at `eab8d99`.** 637 lines, 42,017 bytes, `sha256:ba9693e26889245a9bdc4a6d9ae542494c71ae8202e9f9d233b368027bc11912` (EXECUTED `sha256sum`).

## Verdicts

| Seat | Verdict |
|---|---|
| Restraint | **AYE** (all four round-4 conditions landed; no new constitutive finding) |
| Codification | **AYE** (all three landed; two recital corrections at engrossment) |
| Guardrail | **AYE WITH CONDITIONS** — two constitutive-as-to-words, G-A and G-B below |
| Operability | **AYE** (both constitutive landed; method reproduced exactly; ordinaries landed) |

**THE ACT IS ADOPTED, 4–0, AS CONDITIONED.**

---

## Structural measurement (EXECUTED, `python3` + `yaml.safe_load`)

| Check | Result |
|---|---|
| Top-level keys | `assent_source, enacted_by, id, purpose, sections, status, title` |
| Against `Statute` (`crates/vjs-lawpack/src/lib.rs:256-265`, READ) | one unmodelled: `assent_source`. `citation` absent — `Option<String>`, defaults None; correct, and s1's self-mint recital explains why |
| Section keys vs `StatuteSection` (`:267-274`) | **zero** extras across all 12 |
| `kernel_effect` keys vs `KernelEffect` (`:287-298`) | **zero** extras across all 12 |
| Sections | **12**, s1–s12, ids well-formed |
| Duty tokens (`must`/`must_not`) | **63** (34 must, 29 must_not) |
| Duplicate tokens within the Act | **zero** |
| Collisions with ACT-PROCEEDINGS-DISCIPLINE (52 tokens) | **zero** |
| Collisions across all enacted `lawpack/v2` | **zero** |
| Tokens in `GATE_REGISTRY` (`report.rs:36-241`, READ) | **zero** — s10's assertion is TRUE by execution |
| `status: draft` vs `AuthorityStatus` (`types.rs:40-51`, READ) | `Draft` variant exists; loads |

Every code address cited by the Act was opened and confirmed: `front_door.rs:22` (`VALID_ASSENT_SOURCES = ["sovereign_assent","standing_bounded_assent"]`), `front_door.rs:42` (`declares_valid_assent`, column-zero scan), `front_door.rs:84-97` (the three roots), `refs.rs:47` (regex literally requires `VJS-`), `vjs-core/src/lib.rs:45-50` (`.yaml`/`.yml` only), `gazette/mod.rs:386` (reads `assent_source`), `types.rs:440-447` ("where loss must be impossible, the mechanism cannot be a list of names"), `types.rs:371-423` (`Order`), `lib.rs:257-298`. EXECUTED `grep -rn "serde(flatten\|deny_unknown_fields" crates/vjs-lawpack/src/` → **0**. The draft's `assent_source` value is off the allow-list, so `declares_valid_assent` returns false and the draft claims no floor: **as stated**.

---

## RESTRAINT — AYE

**(1) Tokenise the stay-lift route and warrant challenge in s6. LANDED.** s6 carries `determine_an_application_to_lift_or_narrow_a_stay_before_the_matter_it_concerns_is_entered_on_a_register` (must), `staying_an_application_to_lift_or_narrow_a_stay_or_a_challenge_to_a_warrant` (must_not), and `commission.stay.lift_route_never_stayed` (defines). The warrant-challenge limb is tokenised twice — again at s9 (`refusing_a_warrant_challenge_on_the_stated_grounds`). The prose limb is not left naked.

**(2) s10 must commence s11 and s12. LANDED.** s10 names 1, 2, 5, 6, 7, 8, 9, 10, 11, 12 on assent and 3, 4 on the gate. Union = all twelve; no section unaccounted. The added reason — "an amending provision does not commence apart from the schedule that states its extent" — is the right reason, not a patch.

**(3) Schedule 2's wrong eight. LANDED, by supersession rather than by substitution — and correctly so.** The eight-entry list is gone. `agent-universe-v2` is marked a symlink and expressly *not* a second entry; EXECUTED `ls -ld` → `agent-universe-v2 -> vibe-justice-system`. Restraint asked for `Vibe Justice System` to come **out**; the draft kept it, and on measurement the draft is right and this seat was wrong: EXECUTED, it is a separate `.git` with remote `vibe-justice-system-dev`, its own `.vjs/config.toml`, 1 order, 24 court records, 153 lawpack files and its own `.justice`. It is a distinct invoked, record-bearing surface. The schedule's reservation — "jurisdiction, fork or stale copy is for the first audit and the Principal" — is the correct disposal of a question this Committee cannot decide on the papers. I withdraw the condition as framed and record it as satisfied.

**(4) Strip or justify the unmodelled top-level keys. LANDED.** `drafting_note` and `created_at` are gone. One remains, `assent_source`, justified at lines 15–21 with three addresses I opened and confirmed, and with the honest statement that the loader discards it while two other consumers read it raw. That is the discipline the omnibus engrossment certificate established over `created_at`, applied unprompted. Nothing further.

---

## CODIFICATION — AYE

**(1) Schedule 1 must declare the s7 and s13 variations and the considered-not-varied entries. LANDED, and every extent checked against the varied text.**

- **`ACT-PROCEEDINGS-DISCIPLINE:s7`** — READ: "a forwarding record is created only by the amending court", "A citation is AMENDED only under s6". The declared extent (Commission *prepares and executes*, never *makes*; maker is the competent court whose order is obtained **before** the act; extended to every instrument, not only a filed order) is exact. The extension beyond filed orders is a **real** variation, because s7's route runs through s6, which is titled "Opening and amendment of a filed order". Correctly identified and correctly bounded.
- **`ACT-PROCEEDINGS-DISCIPLINE:s13`** — READ: "A store that is a copy rather than a jurisdiction is registered as a copy or retired, never left ambiguous." The qualification of the *retirement* limb only, and no further, is exact.
- **`ACT-002:s6`** — READ: five triggers. Deferred: (1) first impression, (2) distinction, (3) variation/overruling, (4) conflict. Not deferred: (5) discovered breach. The declaration matches four-of-five precisely and preserves the breach trigger.
- **`REG-SELF-CONVENE-001`** ([2026] VJS-REG 19, in_force) — READ: the fork set is first impression, distinction, overrule, discovered breach, conflict. Four deferred, breach preserved. Exact; the two extra preserved carve-outs are surplus protection, not over-claim.
- **`REG-COURT-RECORD-001`** ([2026] VJS-REG 22) — READ: mandates `bench` and `case_file_digest`, and optionally `convened_at`, `vote`, `appeal_of`, `appealable`. s5 protects **all six**. The considered-not-varied entry understates its own protection; that is the safe direction.
- **`REG-REPOS-REGISTER-001`** ([2026] VJS-REG 18) — READ: "a derived, pointer-only, rebuildable projection, never a source of law". The Schedule quotes it verbatim and correctly reduces the register to evidence.
- **`ACT-003:s5`/`:s10`** — READ: breach self-filing; unsatisfiable-gate auto-justiciability. Carved out at s6(a) and s6(b) respectively. Correct.
- A sixth entry, `ACT-002:s5`, was added unasked and is right: s9 confers **original supervisory** jurisdiction over an executive act, not an appellate tier.

I considered whether s9's new as-of-right routes are an undeclared variation of `ACT-002:s6`'s exhaustive "Court convenes only when". They are not: every ground in s9 (exceeding the power list, failing the test, reaching a floor, acting outside jurisdiction or after expiry, rectifying without an address) and every warrant-challenge ground (non-acceded jurisdiction, over ninety days, fourth concurrent warrant, narrowing a carve-out) is breach-shaped and enters through trigger (5). No gap.

**(2) s5 must protect the named `Order` fields. LANDED.** Checked field-by-field against `types.rs:371-423`: 22 of the 24 named directly, `id` caught at line 287 ("or any `id`"), `extra` caught at line 293. All four Guardrail flagged — `appealable`, `bench`, `case_file_digest`, `court` — are present.

**(3) Schedule 2 must name `opbox-prod/opbox-kernel`. LANDED, and proved.** EXECUTED `git rev-parse --show-toplevel` → `/home/jellytot/Projects/opbox-prod/opbox-kernel`. EXECUTED: `opbox-prod/.vjs` holds `logs` and `permits` only; **zero** governed records under `opbox-prod/{.vjs/orders,.vjs/court,lawpack/v2}`. The added words "and a warrant may not name it" convert the correction into an operative bar. That is better than the condition asked for.

**Two recital corrections at engrossment (ordinary, not constitutive):**
- **C-A.** s1 cites `[2026] CC-OPBOX 16` and `17` as "READ, `:6` of each". EXECUTED `grep -n "^date:"` on both files → **line 7**, not 6. The substance (both dated 2026-06-07) is TRUE; the address is off by one. In an Act whose s3 token is `record_an_address_and_a_mode_on_every_finding`, a wrong line number is a self-demonstrating defect. Correct `:6` → `:7`.
- **C-B.** s2 recites "**84** distinct `.vjs` surfaces"; s12 recites "**85**"; measured today, **85**. s12's own volatility recital explains the 84→85 move, so this is a stale earlier reading left in s2, not a contradiction — but one Act must not carry two totals for one measurement on one date. Conform s2 to 85 and let s12's recital carry the history.

---

## GUARDRAIL — AYE WITH CONDITIONS (two, constitutive as to words)

**(1) s5 inverted to an allow-list over the FILE AS FILED. LANDED in substance.** The test is stated as what MAY change (line 262); the proof is taken over the file and never over the loaded structure, with the reason and both addresses (lines 275–280); deletion and addition are both failures "at every depth" and "whatever its name" (lines 272–273); `Statute` and `Regulation` are covered expressly against vacuity (lines 282–287); the named `Order` fields including all four flagged are covered (lines 289–297). The tokens follow: `take_the_proof_over_the_file_as_filed`, `a_proof_taken_over_a_loaded_structure`.

**I then did what I said I would do, and tried to break it.** Six constructions failed and are caught: list-item deletion inside a mapped key (changes that key's token sequence); key rename (fails twice, once as deletion, once as addition); duplicate-key collapse to the first value (value changes); merge-key `<<:` inlining (`<<` is itself a key, deleted); sequence reordering under a key; anchor/alias expansion (benign). **Two got through.**

> **G-A — CONSTITUTIVE. The root-sequence hole, on a real file in canon.**
> s5 anchors everything to "*every key* the file carries, at every depth". Where a file's **root is a sequence**, the root is not a key and has no key against which "its normalised token sequence" can be compared; and where two sequence items carry the same key *names*, deleting one item in full leaves every key **name** present.
>
> This is not hypothetical. EXECUTED over `lawpack/v2/**` and `.vjs/orders/*`: exactly one list-rooted governed record exists in canon — `/home/jellytot/Projects/vibe-justice-system/lawpack/v2/provenance/assent/2026-06-24-act003-s10-s11-void-first-draft.yaml` — root a **list of 2 mappings**, both carrying key-name sets `['id','kernel_effect','text','title']`. **Delete item [1] in full.** Every key name is present after; no key name is added; the surviving item's values are byte-identical; the root has no key to compare. On the available "key-as-name" reading, the proof is **green** and half a governed record is gone.
>
> The rival "key-as-path" reading (which "at every depth" supports, and which the drafter plainly intends) closes it. But s5's whole warrant is that **loss must be impossible**, and a test that forbids loss only on the better of two available readings has not made it impossible. This is a hole the *cure* opened: the third draft's deny-list did not turn on key presence at all.
>
> **Words to insert**, after line 273 ("...a key absent before and present after is a FAILURE."):
> *"THE COMPARISON IS OVER THE FILE'S NODE TREE AND NOT OVER ITS KEY NAMES. Every NODE the file carries must be present after at the same path: every mapping key with its value, and every SEQUENCE ITEM at its index, at every depth. A sequence that loses an item, gains an item, or changes the order of its items is a FAILURE, whatever key names survive elsewhere in the file; and where the file's root is a sequence, the root is itself compared as a node. A duplicate key name is compared per occurrence and not per name. Every comment the file carries is preserved."*
> **Token to add** to s5 `must`: `compare_every_node_at_its_path_including_sequence_items`.

> **G-B — CONSTITUTIVE. The re-quoting type-flip.**
> `appealable` is `Option<bool>` (READ, `types.rs:423`). Change `appealable: true` to `appealable: "true"`. Every key is present; no key added; the normalised token sequence `true` is byte-identical after scalar reassembly; and the difference is a **change of quoting**, which line 270 expressly permits as the only permitted difference. The proof is **green on every reading**. The order then fails to deserialise, and — on this Act's own recital at line 48, "a loader that propagates on the first unreadable file" — takes the load with it. Content is perfectly preserved; the record is excluded.
>
> It is not unguarded: for an assented record s1(a) voids the act "without more" and lapses the warrant, and for any record s4(a) confers repair *of* schema conformance, so destroying it is outside the exhaustive list and appealable under s9. But **no token catches it**, and s5 — the section whose job this is — blesses it in terms.
>
> **Words to append** to the same paragraph:
> *"A permitted change of style, quoting or folding is permitted only where the record remains loadable by the reader that loaded it before; a re-rendering that changes a scalar's parsed type, or that renders any record unloadable, is not a permitted difference and is not content-preserving."*
> **Token to add** to s5 `must_not`: `re_rendering_a_record_so_it_ceases_to_load`.

**(2) The s4(g) certification power conferred expressly, with s6 tokens binding the stay to the certified schedule. LANDED.** s4(g) is on the exhaustive list at lines 223–226 and is a *scheduling* power "and nothing else", with a schedule naming a jurisdiction, store, series or corpus expressly certifying nothing. s6 opens by confining the stay to the schedule "ENTERED ON THE PUBLIC RECORD AND CERTIFIED" and states "NO STAY ATTACHES before" certification. Four tokens carry it: `enter_and_certify_the_rectification_schedule_before_a_stay_attaches`, `confine_a_stay_to_the_certified_rectification_schedule`, `staying_a_matter_outside_the_certified_rectification_schedule`, `certifying_a_rectification_schedule_naming_a_corpus_as_a_whole`. The stay cannot attach before certification and cannot exceed it. Nothing left naked.

**(3) The three-warrant cap counted against a canonical register, with tokens. LANDED.** s2 makes the warrant a public record in **both** the jurisdiction and canon and states in terms that "THE CANONICAL RECORD IS THE REGISTER AGAINST WHICH THE CONCURRENCY CAP IS COUNTED", with the reason ("a cap over a population held only per-jurisdiction is a cap no actor can check"). Tokens: `record_a_warrant_in_canon_as_well_as_in_the_named_jurisdiction` (must), `a_fourth_concurrent_warrant` (must_not), and the ground is challengeable at s9.

---

## OPERABILITY — AYE

Operability said: *"I would move to AYE on conditions 1 and 2 landing in the words stated, with 3 to 5 at engrossment."* Both landed. I move to **AYE**.

**(1) Re-enumerate Schedule 2 by a find-based method. LANDED — and the method reproduced EXACTLY.** I ran the schedule's own command, not its summary:

| Schedule 2 recites | I measured (EXECUTED, 2026-08-05) | |
|---|---|---|
| 85 distinct surfaces after symlink resolution | `find /home/jellytot/Projects -maxdepth 6 -name .vjs -type d -not -path '*/node_modules/*'` → 85; `readlink -f` + `sort -u` → **85** | ✔ |
| 59 holding ≥1 governed record | classified against the three roots at `front_door.rs:84-97` → **59** | ✔ |
| 21 invoked (`config.toml` present) | **21** | ✔ |
| same command over `/home/jellytot` returns 75 | **75** | ✔ exact |
| `agent-universe-v2` is a symlink, not a second entry | `ls -ld` → `-> vibe-justice-system` | ✔ |
| `opbox-prod/opbox-kernel` a distinct repo root | `git rev-parse --show-toplevel` | ✔ |
| `opbox-prod` a container, ZERO governed records | 0 files under all three roots | ✔ |
| each scan container holds an opbox-kernel copy of ~109 orders one level down | **109, 109, 109** exactly | ✔ |
| `boltrig` 24 orders, 22 court records, NO config.toml | 24 / 22 / absent | ✔ |
| `tablelist-ext` worktree: 25 orders, 16 court, 123 lawpack | 25 / 16 / **123** (`.y*ml`) | ✔ |
| `boltrig/.claude/worktrees/*` seventeen orders-bearing | **17** | ✔ |
| `Vibe Justice System` a distinct repo, own remote | remote `vibe-justice-system-dev`; 1 order, 24 court, 153 lawpack, own `.justice` | ✔ |
| `opbox-kernel` 116 orders | **117** | drift of one |

**The volatility recital is honest.** It states the count is "a MOVING NUMBER, not a fact about the estate", records the 84→85 move within one drafting day and names its cause, and s3 obliges the first audit to re-enumerate rather than trust the recital. The single drift I found (116→117 orders in opbox-kernel) is exactly the behaviour the recital predicts and is inside its own disclosure. The `bounded_claim` names what it does not reach (`/home/jellytot` at large, `Backups/`, `.cache/`) and consigns it to the first audit. This is the honest form.

**(2) s12 must have enumerable `must` tokens. LANDED.** Two: `state_the_method_and_the_bound_of_every_enumeration_in_this_schedule`, `name_every_store_this_schedule_does_not_reach`. Both enumerable, neither duplicated anywhere in the corpus.

**Ordinaries, all landed.** `assent_source` is true-staged (`FOURTH_DRAFT_PENDING_STANDING_COMMITTEE_ADOPTION_2026-08-05`) and deliberately off the allow-list. `drafting_note` is stripped. The unaddressed recital is now inside `purpose` (lines 38–52), carrying its address (`ACT-PROCEEDINGS-DISCIPLINE-drafting-brief.md`) and its mode.

**One new observation, not a condition.** `opbox-security-scan-latest-r2` additionally holds `opbox-kernel-confirmation-fix` (78 orders) and `opbox-frontend-ship` (1), which the schedule does not name. Both are *inside* the measured 85 and 59, so the enumeration is not wrong — only the prose exemplar is thinner than the measurement. That is the architecture working: s12's token `name_every_store_this_schedule_does_not_reach` and s3's re-enumeration duty carry it to the first audit. No change required.

---

## New findings the cure created, and two the bench should see

1. **G-A and G-B above** — both created by the inversion of s5. Recorded plainly: this is the risk the bench named, and it materialised.
2. **s4(f) is not inoperable, but the reason is not on the face.** Read alone, "compare the record as filed before and after" has no *before* for a consolidated restatement, which would make s4(f) a dead power. It survives because `ACT-PROCEEDINGS-DISCIPLINE:s6` (READ) leaves every superseded instrument "visible, citable and routable" and unedited, so the test passes trivially over each, and the concordance is the proof for the restatement. s4(f) already cites s6 and disclaims displacing the concordance. **Ordinary:** add one clause to s5 recording that in a 4(f) consolidation the test is taken over each superseded record, which is not edited.
3. **s4(e) reclassification.** "Reclassify a duty ... from an enforceable duty to a duty enforced by a court" would, if done in the instrument, move a token out of `must` — which is amending a statute, forbidden by s4's own `must_not`. The only lawful reading is that 4(e) reclassifies in the **audit and the report**, never in the instrument, and s8's before-and-after duty-count token supports that. **Ordinary:** say so in one clause.

---

## ENGROSSMENT INSTRUCTION

To Lexby as Clerk-Drafter. The Act is **ADOPTED 4–0 as conditioned**; adoption is constitutive ([2026] REALM-SC 8). Engross as follows and do not pin a digest to Sovereign Assent until this Committee has certified the amended text.

1. **Apply G-A and G-B to s5 in the verbatim words set out above**, and add the two tokens `compare_every_node_at_its_path_including_sequence_items` (must) and `re_rendering_a_record_so_it_ceases_to_load` (must_not). The bench has supplied the words; you have no discretion in them.
2. **Apply C-A** (s1: `:6` → `:7` for `[2026] CC-OPBOX 16` and `17`) and **C-B** (s2: 84 → 85, conforming to s12).
3. **Apply the two ordinaries at findings 2 and 3** — one clause each, s5 and s4(e).
4. **Record the departure from precedent expressly.** The omnibus engrossment was confined to the header and certified by "a digest check only". **This one is not.** It touches operative text and adds two duty tokens. Your certificate must therefore carry: the amended s5 **in full**; both new tokens; a re-run of the structural parse showing 12 sections, **65** duty tokens, zero duplicates, zero collisions with `ACT-PROCEEDINGS-DISCIPLINE` and with `lawpack/v2`, and zero unmodelled section or `kernel_effect` keys; and a `diff` of the adopted text at `eab8d99` against the engrossed text showing **every** change and matching it to a numbered direction above. Any change not on this list is outside your authority.
5. **Retain `assent_source` and re-stage it** to the adopted-not-assented value, keeping it off the `front_door.rs:22` allow-list, and carry forward the justification at lines 15–21 unaltered.
6. **Do not mint a citation.** The ordinal is minted deterministically at commencement, as s1's own recital of the two prior self-mints requires.

Adopted text digest of record: `sha256:ba9693e26889245a9bdc4a6d9ae542494c71ae8202e9f9d233b368027bc11912` (`eab8d99`).

**IS THE ACT ADOPTED? YES — 4–0, as conditioned**, with Guardrail's two constitutive conditions to be discharged in the words stated and re-certified before assent.
