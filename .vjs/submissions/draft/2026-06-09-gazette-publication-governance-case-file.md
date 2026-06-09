# Symmetric Case File: Governance of Publication into the Single Gazette

**For:** the V2 Privy Council (bench of 3)
**Filed by:** Lexby as s.19(1) registrar of the two-sided intake (NOT as advocate; both sides put with equal force; the bench has no access to Lexby's preference)
**Companion to:** the V2 PC reference of 2026-06-09
**Status:** public; system-data only; intake material, not a judgment or law

> Two-sided by design. Case A (V2 governs publication) and Case B (the V1 estate governs its own repo) are each put as strongly as the record allows. The bench decides.

## Part 1 - Agreed facts

1. V2 commenced 2026-06-09 (COMMENCEMENT-V2-0001). V1 is the Gazette and Archive estate; V2 is the live runtime jurisdiction.
2. The single public **Gazette is hosted in the V1 repository** (`vibe-justice-system`); its public site is built from that repo.
3. **Bill 32 s.16** declares one Gazette, two estates, and the **force-source rule** (publication creates no runtime force; estate boundary operative by substance). The Act **delegated the publication mechanics to a V2 Kernel Regulation** which **does not yet exist**.
4. V1's preserved law includes **REALM-SI 7** (a release warrant before every public super-repo push, and a Privy Council post-push review) and **REALM-PC 19** (new publication routes / public-boundary / governance changes need an order).
5. **ACT-COMPUTER-FIRST-REALM s.9 / DEC-V1-SILENCE-001 / INV-NO-V1-GAP-FILLER**: V1 does not bind V2 runtime by silence; V1 binds V2 only by express incorporation. V1 material may be persuasive archive.
6. The V2 founding records and migration history sit on the private development line; only the SC judgment has been published to the public Gazette so far.

## Part 2 - The questions

- **Q1 (governance).** Is publication into the single Gazette governed by V2 (Bill 32 s.16 + a V2 Kernel Regulation) or by V1's SI 7 / PC 19?
- **Q2 (transition).** Until a V2 publication regulation commences, does a push *to the V1-hosted Gazette repo* proceed under V1's SI 7 / PC 19, or is that barred by s.9?
- **Q3 (cure).** Should the bench direct a V2 Gazette-Continuity Regulation, and what must it contain?

## Part 3 - CASE A: V2 governs publication

**A1. Bill 32 s.16 is V2 constitutional law over the whole Gazette.** It declares the single Gazette, the two estates, and the force-source rule, and it expressly *delegates the mechanics to a V2 Kernel Regulation*. The realm chose, in V2's own constitution, to make publication a V2 matter. The host being V1's repo is an implementation detail, not a transfer of governance.

**A2. Relying on V1's SI 7 going forward is exactly what s.9 warns against.** If every V2 publication leaned on V1's release regime, V1 procedural machinery would re-enter V2 operation by the back door, which is the drift s.9 / INV-NO-V1-GAP-FILLER exist to stop. "V2 compiles the law" must include compiling its own publication mechanic.

**A3. The force-source rule must be enforced by V2.** Only a V2 record/check can guarantee that a Gazette entry never becomes a runtime source and that estate labelling is operative by substance. A V1-side warrant cannot certify a V2 invariant.

**A4. The cure is cheap and in-grain.** A V2 Gazette-Continuity Regulation already has a drafted shape (the founding HANDOVER carried REG-GAZETTE-CONTINUITY-001 as draft text). Adopting it through the V2 lawmaking route gives a deterministic export packet, redaction scan, validation report, digest, estate labels and lineage edges - all V2-native.

## Part 4 - CASE B: the V1 estate governs its own repo

**B1. The push target is the V1 estate, and the V1 estate keeps its own law.** Bill 32 preserves V1 as the Gazette/Archive estate. A git push and a default-branch change *to the V1 repo* are operations on V1's own estate; V1's SI 7 / PC 19 were written for exactly this repo and remain its preserved law. Using them is not "revival into V2 runtime" - s.9 bars V1 binding *V2 runtime*, not V1 governing *V1's own repo*.

**B2. REALM-SC 10 left V1 its residual estate control.** The handover had the V1 courts relinquish *runtime* control save the real-world-law floor; it did not strip V1 of authority over its own Gazette repository's publication and integrity. A V2 court has no jurisdiction over the V1 repo's git.

**B3. SI 7 already supplies a working, tested mechanic; the V2 regulation does not yet exist.** Until V2 writes and commences its own regulation, the only operative, deterministic publication control is V1's SI 7 (warrant + post-push review). Insisting on a V2-only mechanic now would leave the realm unable to publish lawfully at all - a worse result than using preserved V1 law for the interim.

**B4. Cross-estate publication is genuinely shared.** V2 governs *what and whether* it publishes (content, force, estate label); V1 governs the *act on its own repo* (warrant, push, post-push review). Forcing the whole thing into one estate's law over-simplifies a two-estate Gazette.

## Part 5 - Narrow questions for the bench

1. Does Bill 32 s.16's delegation place the *mechanic* with V2, while leaving the *act on the V1 repo* with V1's SI 7 - i.e. is the right answer a split (V2 content/force; V1 repo-act), not a winner-take-all?
2. Is a push of the *founding records* "execution steps necessary to implement an existing order" (the REALM-SC 10 handover) that may proceed by citation under PC 19 with an SI 7 warrant, rather than a fresh order?
3. What must a V2 Gazette-Continuity Regulation contain to make the force-source rule and estate boundary deterministically checkable?
4. Should the V2 regulation, once commenced, *supersede* reliance on V1's SI 7 for V2 publication, or *co-exist* (V2 mechanic + V1 repo-act)?

## Part 6 - Relief options

- **(i) Split + direct the regulation.** Hold governance of content/force is V2's (s.16); hold the interim push to the V1 repo proceeds under V1's SI 7 / PC 19 (not barred by s.9); direct adoption of the V2 Gazette-Continuity Regulation through the lawmaking route.
- **(ii) Fully V2.** Hold publication wholly a V2 matter; stay any push until the V2 regulation commences; relying on SI 7 even for the V1 repo-act is disfavoured.
- **(iii) Fully V1 (interim).** Hold the V1 estate governs its repo's publication entirely until V2 chooses to legislate; no direction needed now.
- **(iv) Refer wider.** Identify a constitutional question on cross-estate jurisdiction for the Supreme Court.

The bench is bound by none of these. No publication is authorised by this file; a public push remains a separately warranted act reserved to the Principal.
