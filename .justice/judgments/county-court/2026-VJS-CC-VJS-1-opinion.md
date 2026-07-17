# [2026] VJS-CC-VJS 1 - The Citation Allocator Must Not Mint What It Can See Is Taken

**Court:** County Court at canon, First Instance.
**Seat:** county-judge-first-instance-at-canon (sitting alone; a counted member of the
bench, not a synthesiser on top of other views - CASE-LAW s.18).
**Disposal:** GRANT ON CONDITIONS 1 to 5.

## Opinion of county-judge-first-instance-at-canon

I have read the researched intake, the claimant's submission and the respondent's
submission on a symmetric record. I did not see the engineer's preference. The opinion
below is mine.

## 1. The questions

1. Is this first-impression, and is a County Court at the opbox seat competent to hear it?
2. May the `CorrectedToReferral` enum variant land?
3. May the allocator read the order store (`.vjs/orders/`)?
4. **May a deterministic kernel take an ungoverned prose file (`.justice/INDEX.md`) as load-bearing input?**
5. What follows from the operator installing a patched binary ahead of authority?

Q4 is the only genuinely open question. Everything else is disposed of by law already enacted.

---

## 2. Jurisdiction and trigger

**The stated trigger is false on both limbs, and I find it doubly false.** The parties said the gate code was `PERMIT-EXPIRED`. It is not. The three permits from this matter (`ROUTE-20260717-0957xx`) are on disk, and I read them: `status: active`, `expires_at: 2026-07-18`. They are neither expired nor court-triggering. Their only obligations are `OBL-LOG-001` (decision log) and `OBL-VALIDATE-001` (validate). There is no court obligation because the kernel found none was required.

**That is the kernel agreeing that binding law resolves.** Per S-11(c) I follow it and do not re-decide:

- **ACT-004:s8** (primary Act): "Citations are deterministic and unique ... **Collisions are fatal.**" `must: [check_citation_uniqueness]`, `must_not: [allow_duplicate_citations]`.
- **[2026] VJS-PC 13 D2**: allocate from `load_the_live_persisted_citationregistry`, `collisions_fatal`.
- **[2026] VJS-PC 19**, which neither party quoted where it bites hardest: "**capability is not authority**", and "**an under-inclusive kernel gate is a gap to close, not a jurisdiction to occupy**."

The core question (may a clerk mint a number it can see is taken?) is **not first-impression**. It is non-conformance. On Fact 2, the intake clerk is right and I adopt the correction: "Bill 16 s.8" is a V1-estate gazette record ranking last under ACT-001:s3 and only on express incorporation. The in-force provision is ACT-004:s8. Nothing turns on the substitution.

**Venue.** The respondent is right that a County Court *at opbox* cannot reconfigure the canon kernel for the realm, and I reject the claimant's attempt to brush this aside. But the cure is not dismissal. PC-19 routes tiers **above** First Instance to canon; it does not forbid a First Instance sitting **at** canon. Nothing here is on appeal. The patch is to canon crates with federation-wide effect, so the matter is canon's, and I sit as the County Court **at canon**, in the CC-VJS line. The defect was *noticed* at opbox; that confers no jurisdiction on opbox.

It follows that **this judgment is not `[2026] VJS-CC-OPBOX 122`.** Its citation lies in the canon County series and must be **minted, not chosen** (ACT-004:s8). I record that I have not selected it. I note without relying on it that the allocator is convergent for that series (canon has no CC-VJS record, no citator, and an empty `.vjs/orders/`, so pre-fix and post-fix both yield `1`), so the split-brain at §6 does not infect this judgment's own citation.

The sitting was convened on a false premise but is properly constituted at canon, and the decision log it produces discharges `OBL-LOG-001`. I decline to dismiss: ACT-004:s8 does not tolerate a disposal whose effect is that a known-fatal clerk stays in committed source while the paperwork is corrected.

---

## 3. Holding

### 3(a) The enum variant: **GRANT**, on corrected reasoning

Compelled, but **not** for the reason the fix gives. I verified the mechanism three ways and the claimant's concession is properly made: `live_citation_max` is a line-prefix scan calling an **anchored** `parse_citation`; it never deserialises an `Order`; it never reads `status`. `AuthorityStatus` is not in the allocator's call graph. Pre-fix, `next-citation` never opened the order store. **The enum defect and the fail-open are two unrelated defects found in one session.** Fact 3(a) and Fact 4 are wrong, and Fact 4's "the same unreadable store" is mechanically impossible.

The variant lands on an independent ground: the correction mechanism **already writes** `corrected_to_referral`. **A status the kernel writes must be a status the kernel can represent.** The assented-record floor requires retention, and retention of a record that bricks the store it is retained in is retention in form and destruction in effect. The variant is safe: `is_live()` admits only `Binding | InForce`, and the test correctly asserts it resolves to 0 in the AuthoritySet.

I **do not certify the store as readable**. It is not. I ran it: `vjs status` at opbox still fails with `missing field 'holding'`. The variant is necessary and not sufficient. Reserved at §5.

### 3(b) Reading the order store: **GRANT**

Unobjectionable. `.vjs/orders/` is governed, schema-shaped, machine-checkable YAML: the paradigm of a persisted register under ACT-COMPUTER-FIRST-REALM. At canon the directory exists and is empty, so the limb is inert and no canon regression arises.

**Finding neither party made:** four order files (`2026-VJS-CC-OPBOX-023`, `-024`, `-025`, `-026`) carry **no `citation:` key at all**. The scanner cannot see them. The order store is not uniformly shaped even where it is complete. That is a gap in the store, not in the fix; routed at §5.

### 3(c) Reading the markdown citator: **GRANT, as a one-directional FLOOR only**

This is the constitutional question and I decide it on its merits.

**The respondent is right about what the file is.** `.justice/INDEX.md` is prose, hand-maintained, outside `is_lawpack_yaml`, schema-checked by nothing. I found it carries **at least three incompatible row conventions**: legacy linked table rows (`| [[2026] CC-OPBOX 23](...)`), current blockquote rows (`> **[2026] VJS-CC-OPBOX 59**`), and current plain table rows (`| [2026] VJS-CC-OPBOX 121 |`). The respondent's demonstration that a prose aside moves the register is correct, and I improved on it against myself: **in the course of this analysis my own anchored grep produced a false positive (matching a legacy cross-reference buried in the prose of the row for ruling 61) and a false negative (missing row 121 entirely, because it uses the third format).** Two parsing errors in five minutes. No better proof exists that this file is not a schema.

**But the respondent's remedy would cause the very collision the statute calls fatal, and this is the finding that decides the question.** A *structured* parse of this file is brittle against three formats: it would have missed row 121 and allocated **104, straight into occupied good-law**. The *unanchored* scrape is safe here **precisely because it over-matches**. Over-matching raises `max`; raising `max` skips numbers; skipping is not fatal, colliding is. The over-breadth the respondent attacks is what makes the read safe. It is a feature of a floor and would be a bug in a register.

So the kernel is not taking prose **as law**. It is taking prose as **evidence of occupancy**, in the one direction where misreading is harmless. A clerk who can see a number is taken must not issue it, and the law does not ask where the clerk's knowledge came from. The claimant's best point is good: an instrument the Principal's own standing instruction requires the bench to consult *first* is not one the clerk may refuse to read before allocating.

**Ground 6 (entrenchment) fails on its facts.** The respondent says: complete the order store, then read only it. I tested that and it does not hold. The divergence is **bidirectional**: 41 citator numbers have no order file, *and* 13 order files are absent from the current-grammar citator. Neither store dominates. The union is not a convenience, it is the only safe read available today. **A sunset ordered today would mandate a collision.** I refuse the sunset and address the entrenchment risk by declaring the scrape a floor, not a register, which preserves the pressure without commanding the harm.

### 3(d) The legacy-grammar limb: **REFUSE as drafted** (my own finding, made by neither party)

The fix skips the legacy grammar on the stated ground that "the two run as separate sequences (opbox: legacy to 59, current to 121)". **That premise is false and I verified it.** Ruling 23 exists exactly once: order file `2026-VJS-CC-OPBOX-023.yaml` (`issue: opbox_storage_seam_implementation_determination`), recorded in the citator as `[2026] CC-OPBOX 23`. All 13 order files "missing" from the citator are present under the legacy spelling, and the union of both grammars leaves **zero** order files unaccounted for. Bill 16 s.7 replaced a **form**. It did not open a second sequence. One sequence, re-spelled.

The consequence is a latent collision generator in **canon code that runs at every subscriber**: wherever a subscriber's legacy max exceeds its current max, skipping legacy allocates into occupied ground. The fix's reasoning has the safety direction exactly backwards: **counting** legacy can only raise `max` (safe); **skipping** it is what can under-allocate (fatal). At opbox it is harmless (legacy row-max 56 < 121) and the condition is free: max(56, 121) = 121, so opbox still allocates 122. I note in passing that "legacy to 59" is itself prose contamination; the true legacy row-max is 56.

### 3(e) The canon series at a subscriber seat: **ORDERED as a condition**, and **both parties mischaracterised it**

**Correction to both.** The claimant says "pre-fix the same seat mints SC 3 or SC 1". Wrong. The respondent presents it as a vice the patched allocator "commits". Also wrong. I computed it from source and confirmed against the binary: opbox's lawpack mirror holds SC max 5; the referral file's `citation: "REFERRAL -> [2026] VJS-SC 4 (canonical)"` fails the anchored parse and contributes 0; the citator contributes 4. **Pre-fix and post-fix both yield `[2026] VJS-SC 6`.** The fix neither causes, aggravates, nor cures it. Ground E therefore **cannot** be a ground to refuse a fix that does not touch it.

The defect is nonetheless real, live, and fatal: canon holds `[2026] VJS-SC 6` as good law, and opbox mints it. Both seats mint `PC 20`. A subscriber-minted SC citation is identical in form to canon's, with nothing in the string marking its origin.

It is **not first-impression**. PC-19 is on all fours: capability is not authority; an under-inclusive gate is a gap to close, not a jurisdiction to occupy. Following binding law is squarely within First Instance competence, I sit at canon, and the gate is small. I therefore order it as a **condition** rather than reserving it: this court will not certify a clerk that mints into canon good-law.

---

## 4. The ratio

> **A deterministic allocator must not issue an identifier it can see is already taken, and must therefore read every store within its reach that evidences an allocation; an ungoverned prose store may be read for this purpose only as a one-directional floor - able to raise the next value, never to lower it, and never to establish that a value is free.**
>
> **Where the allocator's reach over a series is definitionally partial, as when a subscriber is asked for a canon series it does not own, it must refuse to mint rather than allocate from what it can see: capability is not authority ([2026] VJS-PC 19).**

Everything else in this judgment is obiter.

---

## 5. Reserved, and where it belongs

1. **Which register is authoritative** for a subscriber's local series (PC-13 D2's singular "the live persisted CitationRegistry"). Federation topology. **Not decided.** Belongs at **canon, Privy Council**. The union-max is greater than or equal to any single-register max, so no allocation under this order can be retrospectively rendered a collision by that ruling. Forward-compatible with every answer.
2. **Fail-closed on an unreadable store** (the respondent's condition 2). **Reserved, and for a principled reason: it cannot be decided without (1).** A rule to "refuse when a required store is absent" presupposes knowing which stores are required. Q2 and Q3 travel together. I note the reachable case: if a subscriber's citator went missing, the clerk would silently fall back to the order-store max and under-allocate. That is the reserved defect and it is live.
3. **The order-less rulings.** I decline to adopt **any** party's figure. The claimant says 37; the respondent says 28; my own count is 41 citator numbers with no order file, plus 13 order files whose citator row is legacy-only, plus 4 order files with no citation key, plus roughly 5 "citator numbers" that are prose mentions and not rows. **The true figure is not reliably computable from an unparseable file, and that is the finding.** A ruling that recites a contested count as fact is a ruling on a wrong record. Routed for reconciliation.
4. **Whether a retained `corrected_to_referral` record must satisfy the full `Order` shape.** `vjs status` still errors. Not decided, not assumed cured.
5. **The stale opbox lawpack mirror** (SC 6 absent), the operational divergence that makes §3(e) reachable.

---

## 6. The binary/source split-brain

**It is a breach of the duty of reasonable skill and care (S-4 to S-8), and I find it as such.** The operator installed a patched binary (`~/.cargo/bin/vjs`, Jul 17 09:18) while the fix remains staged. The kernel in force differs from the kernel of record, and the divergence was created by the party seeking relief, before the sitting, in the direction of the relief sought. That is an ACT-COMPUTER-FIRST-REALM:s5 discrepancy in its purest form: the ceremonial text and the kernel effect materially disagree.

**Breach is civil. There is no punishment and I impose none.** The only remedy is to make the work good.

I accept the mitigation, which is substantial and which I weigh: it created no governed record; the alternative was worse, since uninstalling restores a clerk the statute calls fatal; and it is why this court has evidence at all, including the §3(e) finding that cuts against the installing party's own case. The respondent's restraint in not seeking removal is correct.

**The trap is real and undocumented.** As the respondent put it, nothing in the repository tells the next operator that the binary they are replacing is better than the code they are building. A rebuild from `main` silently restores the fail-open clerk.

**Remedy and deadline.** Reconverge by landing the corrected fix and rebuilding from it. **The divergence must not survive the next commit to canon.** If the conditions at §7 cannot be met in that window, the operator must record the divergence in the decision log so the next operator is not trapped by silence. **Self-file the s.4-8 breach** for the interval. Relief that leaves source and binary divergent would launder the divergence, and I do not grant it.

---

## 7. Directions

In order:

1. **Strike the false ceremonial text**, as a condition of grant, not an afterthought. Three false statements in the staged text: (a) the enum-to-fail-open causation in `types.rs` and in the test in `authority_route_obligations.rs`, including "counted ZERO orders"; (b) "it is the only store that sees EVERY ruling"; (c) "the two run as separate sequences". State the two defects as independent.
2. **Correct the pre-existing false record in committed source, which neither party sought and which is the original sin.** `admin.rs:16` claims "PC-13 D2: allocate from the LIVE persisted register (the citator index)" and `validator.rs:378-381` claims "This is the persisted register D2 requires the allocator to read - the citator INDEX is the count". **Both were false when written.** The code never opened the citator. The kernel has been certifying a D2 compliance it did not have, and that, not the staged patch, is the gravest s5 discrepancy on this record.
3. **Do not restate contested counts as fact.** Record only what is verifiable: order-store max 84, citator row-max 121 (good law, verified), 78 order files. Not "84 orders / 121 rows / 37 rulings".
4. **Count both grammars** in the citator scan (§3(d)). Free at opbox (still 122); closes a realm-wide latent trap.
5. **Refuse a canon series at a subscriber seat** (§3(e)). `next-citation SC|PC` at a subscriber must error, not offer `[2026] VJS-SC 6`.
6. **Land**, with the decision log this judgment constitutes, discharging `OBL-LOG-001` on the active permit. File in the **canon County line**, citation **minted by the allocator**, never chosen by hand.
7. **Rebuild and reinstall from landed source.** Verify: opbox CC → 122; opbox SC → refuses; canon CC → CC-VJS 1; canon SC → 7; canon PC → 20.
8. **Self-file the s.4-8 breach** for the binary interval (§6).
9. **Route the reserved matters** (§5): items 1 and 2 together to canon/Privy Council; items 3, 4 and 5 as local reconciliation work at opbox.

Obiter, and not ordered: `live_citation_max` is misnamed but **behaviourally correct**. It ignores `status`, which is right - a citation once issued is spent forever, and an overruled ruling still occupies its number. "Live" in D2 modifies the *register* (live and persisted, as against an empty in-memory registry), not the authorities within it. The respondent's reading of the name is understandable and the name invites it; a rename is desirable and I leave it to the engineer.

---

**Summary.** The statute settles the core: a clerk must not mint what it can see is taken. The union-max is compelled, not chosen. The prose citator may be read, but only as a floor that can raise and never lower, because its over-breadth is the only thing making it safe and a rigorous parse would collide. The enum lands on the floor's authority, not on the fix's false story about itself. The legacy skip is refused on a premise I found false. The apex collision is real, orthogonal to this fix, and closed by PC-19 as a condition. The false record - staged **and committed** - is corrected before anything lands. The binary breach is civil, mitigated, and cured by landing and rebuilding, not by refusing relief.

**GRANT ON CONDITIONS 1 to 5. Land under 6. Reconverge under 7 and 8. Reserved matters under 9.**