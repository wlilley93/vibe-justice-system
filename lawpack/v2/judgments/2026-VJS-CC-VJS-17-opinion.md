# [2026] VJS-CC-VJS 16 - opinion of the County Court, sitting alone at first instance

Bench of one. Vote 1-0. Submission `SUBMISSION-2026-08-01-173759`. Recorded under ACT-002:s7, within the ACT-002:s2 word limit.

---

## 1. The question

Where two registers name terms that must not enter canon, which governs; and may a gate read a register narrower than the harm it exists to prevent?

## 2. Decision

**Application GRANTED IN PART. Option A refused. Option C refused. Option B adopted, varied and extended.**

**THE RATIO.** Two registers of terms that must not enter canon do not merge merely because both can block a canon write: each answers to the harm it was made for (federation authority under ACT-007:s4; confidentiality under ACT-005:s1), and a register whose entries are hashed *because the terms are private* may never be accessioned into a register whose entries are published in cleartext. A gate must consult every register whose harm it exists to prevent, over every artefact in the class it governs, and must treat an unreadable register as an error and never as an empty one; so widening the register is no answer at all while the gate cannot reach the file.

## 3. What I measured that contradicts the case file

I verified the pleadings rather than taking them. Four things are wrong, and two of them change the disposal.

**(a) The pleaded cause of the Gazette refusal is false.** Agreed fact 4 ties the refusal to the three admitted records. It is not. I copied the tree to a scratch path, kept all three records and the real denylist intact, and `vjs gazette` **published cleanly** (114 items). I then copied that identical tree under a directory named with a denylisted token and it **refused**. I then removed exactly one denylist line and it published again. The refusal is caused by a single field: `crates/vjs-cli/src/gazette/render.rs:214` publishes `resolution.dir.display()`, the **absolute checkout path**, whose operator-account segment is itself a denylist entry. That field was introduced **today**, by the applicant, in `c7b3ba2` ([2026] VJS-CC-VJS 15 C4). Twenty lines away, `gazette/mod.rs:314-316` names this exact hazard in terms ("At the resolver it would publish a checkout path") and guards against it for item paths. ACT-005:s1 forbids "private repo paths" in the public record on its face. Curing the three records would not have unblocked publication by one day.

**(b) Option A would not have blocked any of the three records, and neither would Option B as pleaded.** All three are `.md`. `crates/vjs-redact/src/lib.rs:514-516` skips every non-YAML canon file before any content limb runs. I built a fixture and measured it: a `.yaml` canon record naming a registered subscriber returns one blocking finding (positive control, so the probe can fail); the identical text in a `.md` in the same write-set returns **zero**. Signal 4 has never been reachable for a judgment opinion. Accessioning terms into the register it reads would have changed nothing.

**(c) Option C's premise is false twice over.** "No record escaped" is not so. `origin/master` is a public remote and carries all three files; PUBLISHING.md rule 1 records that landing on it *is* publication. And the backstop said to have held is structurally blind to what it is credited with catching: `gazette/mod.rs:400-405` publishes an opinion's path and a public blob URL but never its body, and only V1 archive markdown is inlined (`mod.rs:476`). The publication gate scans the summary and links to text it does not read.

**(d) Minor corrections.** The obiter relied on is **CC-VJS 14 obiter (iv)**, not CC-VJS 13's (whose obiter ends at (iii)); the pleading and the referral both misattribute it. The denylist holds 14 entries of which one is, by the file's own record, a synthetic sentinel, so 13 real terms. Fact 3 is **accurate as to canon**: exactly three files under `lawpack/v2` carry a denylisted token. It is silent as to the rest of the tree, where I measure **228 occurrences across 43 tracked files**, and where `scripts/boundary-scan.sh` has failed closed on every full-tree run since `9a6004ca` (2026-07-18). That wider condition is not before me.

## 4. Reasoning

**Why A fails, on a stronger ground than the pleaded one.** The applicant says merging would publish private terms in cleartext. True, but the decisive point is in the denylist's own header: the original seeds *were* the accessioned public pseudonym and its citation prefix, and they were removed on 2026-06-23 because they fail-closed the Gazette against a subscriber's **own public law** and "protected nothing private". The registers are deliberately disjoint and were made so by a recorded cure. The registry is a list of public codes serving a jurisdictional harm; a code in it is printed in canon, in cleartext, by design. The denylist is a list of hashes serving an exposure harm. A is not consolidation; it is the reinstatement of a cured defect. "Two copies of a rule is one copy and a disagreement" is a sound maxim and it does not apply here, because these are not two copies of one rule.

**The strongest objection to B, which was not pleaded, and my answer.** The pleaded objection (a blocked author is not told which term) is weak. The real objection is this: *a denylist limb on the canon-write gate is unauditable by construction and therefore invites false confidence.* Nobody can review a hash list for completeness or correctness; the 2026-06-23 entry proves the register has been wrong before and that the error was discoverable only by someone who happened to know the plaintext. Wiring a gate to it produces an instrument that reports "canon is clean" when it means "canon holds none of thirteen terms someone remembered". That is the vice CC-VJS 12 refused for the lawpack pin: a register whose scope no reader can answer.

I accept the objection and it does not defeat B. It is an argument for conditions on the register, not for the gate reading none of it. The counterfactual is not a better gate; it is today's gate, which fires on zero terms, over `.yaml` only, and passed a green `vjs validate` this afternoon on a tree the publication scanner rejects. An incomplete gate that fires on thirteen terms is strictly better than one that fires on none, **provided it never asserts completeness** and **provided the register carries a per-entry reason a reader can answer without learning the term** (CC-VJS 12, applied). Hence C6 and C7 below.

**Why the variation.** The pleading would re-point signal 4 at the denylist. CC-VJS 14 forbids that shape: a gate's guard, and its finding, must answer to the gate's referent. Signal 4's finding says "names subscriber X" and cites ACT-007:s4. A confidentiality hit on infrastructure vocabulary is not a subscriber finding and must not wear that message. The limb must be separate, separately coded, and cite ACT-005:s1.

## 5. Conditions

**C1. A separate denylist limb on the canon-write gate.** In `scan_canon_writes`, a new limb tokenises a canon record exactly as `gazette/render.rs:264-280` does and blocks on a hash hit, with its own finding code and a message citing ACT-005:s1. It must **not** print the term. It **must** print the file and the 1-indexed line, which discloses nothing to anyone who does not already hold the file.
*Vacuous if* the fixture denylist is empty or the token is absent. *Prove it can fail:* a fixture denylist seeded with the sha256 of a synthetic token, one record containing it asserting a block, and a negative control without it asserting none. An absence assertion alone is barred by CC-VJS 14 obiter (i).

**C2. The gate reaches the artefact it governs.** Delete the `if !is_yaml { continue; }` skip at `lib.rs:514-516` for the content limbs, so signal 4 and C1 run over every canon file, `.md` included. Structured-field limbs (signals 1 to 3) may remain YAML-keyed; the *prose* limb may not.
*Vacuous if* the positive control is a `.yaml`. *Prove it can fail:* run one fixture body twice, once as `.yaml` and once as `.md`, and assert both block. My scratch probe is the red seed: it currently records `MD findings: 0` against `YAML findings: 1`.

**C3. No register is silently empty.** `load_subscriber_codes` (`lib.rs:437-441`) returns an empty vector when the registry is missing or unparseable; `render.rs:254` skips the denylist limb entirely when the file is unreadable; `boundary-scan.sh` and `promote-canonical.sh` both pass on `FileNotFoundError`. All four are fail-open. Each must become an error naming the path it could not read (CC-VJS 12 D1, on all fours; not re-litigated).
*Vacuous if* every fixture supplies a readable register. *Prove it can fail:* four fixtures (missing, unparseable, present-but-empty, readable), the first three asserting an error and the fourth asserting none.

**C4. The publication surface publishes no absolute path.** `render.rs:214` must publish a repo-relative directory, or omit the field, matching the treatment already reasoned at `mod.rs:314-316`. This is the direct application of an unambiguous ACT-005:s1 prohibition and decides no new law.
*Vacuous if* asserted by grepping the source. *Prove it can fail:* generate the Gazette from a checkout whose absolute path contains a synthetic denylisted token and assert the artefact carries no `/`-rooted path. That fixture exists and currently refuses.

**C5. The publication gate reads what it links.** Every `source_opinion` body reachable from a published item is included in the text scanned at `render.rs:233-287`, whether or not it is rendered.
*Vacuous if* the fixture opinion is not linked from a published order. *Prove it can fail:* a fixture order whose `source_opinion` carries the synthetic token; assert refusal, then remove it and assert publication.

**C6. No instrument reports canon boundary-clean on the strength of C1 alone.** The finding is positive only.

**C7. Per-entry provenance on the denylist.** Each hash line carries a machine-checked trailing comment of the form `# added=YYYY-MM-DD class=<client|infra|synthetic>`, checked by a test. No plaintext. This is CC-VJS 12's re-pin-reason ratio applied to this register, and is what would have made the 2026-06-23 error visible without knowing the terms.

**C8. Cure of the three admitted records, by the applicant.** The prose occurrences are redacted to the generic form signal 4 already prescribes, or to the accessioned pseudonym. Under ACT-004:s9 the correction is a **new record** citing this order and naming file and line, never a silent edit and never a history rewrite. Two limits, both load-bearing:
 (i) The cure is measured against ACT-005:s1, **not** against the gate going quiet. In record 1 the term also sits in the filename and in the neutral citation, where the tokeniser cannot see it because hyphens are token characters. A cure that silences the hash and leaves the citation is a cure aimed at the instrument, not the harm.
 (ii) Whether a canon citation *series* may carry such a term, and how it could be re-seriesed without breaking the citator, is genuinely first-impression and **is not before me**. It is to be filed. Nothing irreversible is to be done to any citation meanwhile.

**C9. C4 is discharged before C8.** The estate does not publish until the path field is cured, and curing the records alone will not move it.

## 6. The self-filed breach

A breach is made out on record 3 and requires **nothing beyond making the work good**. The duty of care is restorative (SPEC-LAW S-4 to S-8). No sanction, no vacatur of CC-VJS 13, no disqualification of the applicant from performing the cure.

I record two corrections to the self-filing, because a plea in mitigation and a plea in aggravation are both parts of the record. **In the applicant's favour:** "did not check" overstates the fault. There was nothing to check with. `validate --staged` runs the gate that skips `.md`; the publication gate cannot see an opinion body; and `scripts/boundary-scan.sh`, the only instrument that would have caught it, is **not on the active hook path at all** (`core.hooksPath` is `.vjs/hooks`, while `scripts/install-boundary-hooks.sh:8` writes to `.git/hooks`, which git therefore ignores). I ran `vjs validate` on the present tree: **OK**. **Against the applicant:** the term was used as a fixture code in a negative control, where the accessioned pseudonym was available and would have served identically; and the applicant authored the register that forbids it. That falls below reasonable skill and care, and more clearly than the pleading admits, because the same commit day produced C4's absolute-path field beside a comment naming that precise hazard.

Blame apportioned: none. The remedy is C1 to C9.

## 7. Obiter (no part of the ratio)

**(i)** `crates/vjs-cli/src/local_ci.rs:46-65` contains a step **named** `boundary_scan` that reads `.vjs` with the PII scanner and never opens the publication denylist. Two instruments share a name and one register between them. A reader who sees `boundary_scan: PASS` on `vjs local-ci` is entitled to think the publication boundary was checked. It was not.

**(ii)** `scripts/promote-canonical.sh:64` excludes `.vjs/logs|permits|submissions|court` on a recorded 2026-06-11 decisive call; `scripts/boundary-scan.sh` reverses it in terms ("Records ARE in scope now") and is invoked by the same script at line 111. Two gates, one artefact, opposite scopes, both live. The stricter has been red since 2026-07-18 (228 hits over the range `ebf34d7..c7b3ba2`, of which 3 are canon). An unrun gate banks debt, and a gate red on 228 standing findings cannot distinguish a new defect from the standing condition. Whether governance records may lawfully carry operational names on a public remote is a real question and squarely first-impression. It should be filed.

**(iii)** `crates/vjs-testkit/tests/publication_boundary.rs:17-25` asserts over the **checked-in** `gazette-data.json` / `gazette-text.js`. Those artefacts were generated at `3d307cd` and predate C4's new field, so the test passes today while the generator refuses. A regression test over a committed artefact tests yesterday's artefact.

**(iv)** CC-VJS 14 obiter (iv) was correct and has now arrived as a defect, though not by the route it predicted: the registry's narrowness was never the operative cause of anything, because the limb keyed on it could not reach a `.md` in the first place. An obiter that names a narrow register, when the gate reading it is unreachable, understates the defect. That is worth remembering the next time a gate is widened rather than reached.

**(v)** `load_subscriber_codes` and `render.rs:254` both carry, or lack, the CC-VJS 15 `LAWPACK-LITERAL` marker discipline. The marker machinery at `crates/vjs-testkit/tests/lawpack_literal_marker.rs` is the right precedent for C7's per-entry provenance check, and should be reused rather than reinvented.

---

**Appealable.** Permission to appeal is granted on the register question only. The reach question (C2) is CC-VJS 13 applied and needs no higher forum.