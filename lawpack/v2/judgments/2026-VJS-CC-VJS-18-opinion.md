I have measured the pleaded facts. Here is the opinion.

---

# IN THE COUNTY COURT OF THE VIBE JUSTICE SYSTEM (FIRST INSTANCE)

**Marbury J, sitting alone.** On SUBMISSION-2026-08-01-173842. Citation to be allocated through `vjs citation next`; not asserted here.

## 1. The question

Whether `crates/vjs-engine/src/lib.rs`, the file holding `validate()` and (since [2026] VJS-CC-VJS 15) the sole lawpack resolver and the sole unresolvable-lawpack refusal, must be added to `ENFORCEMENT_SURFACE` (PC-16 D4); and, if the surface may grow, what principle bounds it so that re-pinning does not decay into noise.

## 2. Decision and ratio

**Option A, varied.** The file is entrenched, whole, without a prior split.

**Ratio.** A file belongs on the entrenched enforcement surface if, and only if, an edit confined to that file can by itself change whether a bright-line finding is emitted, what severity it carries, or whether the check that produces it runs at all. Call position is not the test; dispositive power is. The surface is bounded not by category but by proof: no file joins on the theory that it is upstream of a gate, and a file joins only when a committed counterexample shows a confined edit to it flipping a bright-line outcome while the lock stays green.

## 3. Reasoning

**The hole is real, and I measured it rather than took it.** On a scratch fixture at `/var/tmp/claude/vjs-cc16` carrying the committed lock and copies of the eleven pinned files, the release binary reported no drift at baseline; appending one comment line to the copy of `crates/vjs-engine/src/lib.rs` still reported no drift; appending the same line to `crates/vjs-core/src/bench.rs` produced one Fatal `ENFORCEMENT_SURFACE_DRIFT`. The fixture is therefore one in which the finding is reachable, which is what CC-VJS 14 obiter (i) requires before an assertion of absence means anything.

**Dispositive power, measured, in this file.** Three sites: (i) the referent-keyed conditions at lines 366 and 406, where one wrong guard made a Fatal unreachable (CC-VJS 14); (ii) the PC-14 D3 / PC-16 assent floor at lines 571 to 586, including the `!crate::assent::is_constitutive(&fd.code)` carve-out, deleting which re-opens the exact forgery PC-16 was convened over; and (iii) line 536, `findings.extend(vjs_core::enforcement::check_drift(repo))`, which is the **only production caller of the entrenchment witness in the workspace**. `enforcement.rs` is pinned as "this witness itself" and its sole invocation sits in an unpinned file. That is a lock bolted to a frame anyone may unscrew, and it was not pleaded.

**PC-16 D4 already reaches most of this.** D4 directs the pinning of "the assent-validity **and floor-enforcement** code path", and PC-16 names `crates/vjs-engine/src/lib.rs` three times as the floor-attachment site. The resolution half went to `assent.rs` and was pinned; the floor half, the severity mutation itself, stayed here and was not. To that extent this order is not an expansion of the surface but the completion of an existing directive.

**Why not Option B.** The split is refused as a precondition, on measurement rather than taste. After CC-VJS 14, `resolve_lawpack_dir` is "the single source of both the directory scanned and of whether the lawpack-referent check runs at all", and after CC-VJS 15 it is the only place in the workspace that may name the lawpack to read canon. Both halves of the file are therefore dispositive, so there is no cut today that puts the disarm sites on one side. Cutting between guard and resolver would sever a rule from itself, which is the defect CC-VJS 12 and CC-VJS 15 have each already cured in this very file. Nor does a split de-entrench: `crates/vjs-redact/src/tests.rs` was split out of `redact/lib.rs` and stayed pinned. Entrenchment follows the code across a split; it does not wait for one.

**The pleaded objection to Option A is foreclosed.** The applicant says ordinary work here will demand a re-pin ceremony, and a ceremony paid often is paid unread. [2026] VJS-CC-VJS 12(d) is on all fours: "the respondent's churn objection is real but is an argument about CADENCE, not about SCOPE, and it is answered by making a re-lock a recorded act rather than by narrowing what is hashed." I follow it (S-11(c)) rather than re-litigate. The precision limb also fails to distinguish: the surface already pins `redact/lib.rs` and `staged.rs`, both at exactly 600 lines, and `staged.rs` has churned five times against this file's six in the visible history.

**The strongest objection was not pleaded, so I state it.** It is that my own ratio does not stop here. Applied evenly it also catches `crates/vjs-core/src/install.rs` (emits Fatals at lib.rs:540 and blocks the pre-write door at front.rs `install_block`, unpinned), `crates/vjs-cli/src/front.rs`, `crates/vjs-cli/src/local_ci.rs` and `crates/vjs-mcp/src/lib.rs:349`. So the order that closes this hole proves the list is at least four short, and a list known to be short but presented as audited misleads worse than an honest gap.

I answer it three ways. First, under-inclusiveness is a reason to decide the case in front of me, not to leave it open: the cure for a short list is a longer list arrived at by adjudication, not a shorter theory. Second, the honesty defect is curable today and I cure it in C5: the surface's own comment currently claims the list "must stay complete", which is prose asserting a property nothing checks, and CC-VJS 15 holds that prose is not enforcement. Third, the anti-noise bound is the proof requirement in C6, which converts "the list is short" from a silent defect into a visible docket: additions cost a counterexample, so nothing joins on speculation, and nothing that has demonstrably disarmed a gate can be kept off.

**What entrenchment does not buy, stated candidly (PC-16 D4).** Pinning this file does not protect line 536 from itself. Delete that line and `validate` stops reporting drift, and the changed digest has no reporter left inside the binary. The only witness that survives that edit is out of band: the workspace test suite, re-run by required CI under K-27. Hence C3. The pin makes an edit loud; it cannot make the binary honest about its own excision.

## 4. Conditions

**C1.** Add `"crates/vjs-engine/src/lib.rs"` to `ENFORCEMENT_SURFACE` with a comment naming the three disarm sites above by what they do, and re-pin. Checkable: the const contains the path; `.vjs/enforcement-surface.lock` carries twelve entries; `vjs validate` reports no `ENFORCEMENT_SURFACE_DRIFT`. The membership assertion in `enforcement.rs` tests is a spelling check, not a proof, and must say so in its own comment; it may not be offered in satisfaction of C3.

**C2.** The comment must record that the pinned file sits at 589 of the 600-line machine-checked ceiling (`crates/vjs-testkit/tests/structural_ceiling.rs`), and that when the ceiling forces the split, the disarm sites carry their entrenchment with them in the same change, on the `vjs-redact/src/tests.rs` pattern.

**C3.** A proof at the governed boundary, not the proxy (CC-VJS 13). A test that builds a fixture repo with an enforcement lock and one pinned file whose bytes differ, calls `vjs_engine::validate`, and asserts the returned `Report` contains a Fatal `ENFORCEMENT_SURFACE_DRIFT`. **What would make it vacuous:** the existing K-25 test asserts the *absence* of drift on a clean tree and calls `check_drift` directly, so it passes whether or not `validate` ever calls it. **Proof it can fail:** delete line 536 and record that the new test goes red while K-25 stays green. I have already measured the fixture shape end to end (0 findings clean, 1 Fatal on a one-byte edit to a pinned file), so the shape is known to be reachable.

**C4.** A positive control on the floor. `e2e_gate_harness.rs:166` asserts that a constitutive finding is *not* downgraded, which is an assertion of absence and proves nothing unless the downgrade loop was reachable on that fixture. Add to the same fixture an assertion that a **non-constitutive** blocking finding on the same path **was** rewritten to the `ROUTE_FOR_CORRECTION` form. **Vacuity:** if `assented_record_paths` is empty there, both assertions pass for the wrong reason. **Proof it can fail:** removing `&& !crate::assent::is_constitutive(&fd.code)` must turn the constitutive assertion red. If the positive control cannot be made to pass, that is itself the finding and must be self-filed.

**C5.** Replace the surface comment's criterion ("the focused, rarely-churning gates") with the dispositive-power test, and strike the claim that the list "must stay complete", replacing it with a statement that the list is curated and known to be under-inclusive, naming the four candidates in section 3 as undecided. Checkable by reading the file.

**C6.** The bound. No path is added to `ENFORCEMENT_SURFACE` except on a committed test showing a confined edit to that file flipping a bright-line outcome while the lock stays green, in the manner CC-VJS 15 proved its marker gate red by a seeded counterexample. Checkable per addition: every entry beyond the twelve must cite the test that admitted it.

**C7.** Applying CC-VJS 12(d) to the second lock, not extending it: `vjs enforcement-lock` must record, per entry, the authority under which the digest moved (order citation or decision-log id), and must refuse to write without one. **Vacuity:** a field that defaults to a constant, or accepts empty, checks nothing. **Proof it can fail:** a test in which the write is *refused* and no file is written.

## 5. Obiter (not before me, found by measurement)

(i) **`validate()` is one door of three.** `.vjs/hooks/pre-commit` runs `vjs validate --staged`; `.vjs/hooks/pre-push` runs `vjs local-ci` (`vjs-cli/src/local_ci.rs`, seven steps, which never calls `check_drift` and never checks `LAWPACK_LOCK_DRIFT`); `.vjs/hooks/pre-write.sh` runs `vjs hook`, dispatched in `vjs-cli/src/front.rs` as apex, then install, then canon, then `evaluate_governed`. A fourth door, `vjs-mcp/src/lib.rs`, calls `verify_bench` itself.

(ii) **`local-ci` holds a second, weaker citation check**: an inline duplicate-id loop over `lawpack.orders`, where `validate` calls `check_citation_uniqueness` over the local governed roots. Two copies of one rule, which CC-VJS 9 and CC-VJS 12 have each treated as one copy and a disagreement.

(iii) **`crates/vjs-core/src/install.rs` is an unpinned Fatal-emitting gate** on both the staged and pre-write paths.

(iv) **`check_drift` iterates the code's list, not the lock's**, so a path removed from the const leaves an orphan lock line that nothing reports. A "the surface shrank" finding is cheap.

(v) **No `reason` field exists on `.vjs/lawpack.lock` either**, and I found no superseded-lock ledger, which may be an under-implementation of CC-VJS 12(d) and (e).

## 6. What I measured that contradicts the case file

- **Facts 1 to 6 are accurate.** Eleven pinned entries, verified by count and by digest; the file is absent from the list; CC-VJS 14 was a defect in it; CC-VJS 15 re-pinned exactly five entries (`assent.rs`, `front_door.rs`, `redact/lib.rs`, `staged.rs`, `validator.rs`) while rewriting it.
- **Option C's "Against" is overstated.** `validate()` is **not** "the only entry point through which the gates run at all". It is the pre-commit door of at least three. This cuts against the applicant's rhetoric and *for* the relief: the hole is wider than pleaded.
- **Option A's "Against" does not distinguish.** Two 600-line files are already pinned, one of them the second-most-churned in the set.
- **"Three defects in one day"** is, precisely, three court-ordered cures spanning 2026-07-31 17:46 UTC to 2026-08-01 15:33 UTC, so twenty-two hours across two calendar days, plus a fourth non-court change (`c314d02`) inside that window.
- **The submission understates its own case** by omitting line 536 and by omitting `install.rs`.

Key paths: `~/Projects/vibe-justice-system/crates/vjs-core/src/enforcement.rs`, `~/Projects/vibe-justice-system/crates/vjs-engine/src/lib.rs`, `~/Projects/vibe-justice-system/crates/vjs-testkit/tests/kernel_invariant_bindings.rs`, `~/Projects/vibe-justice-system/crates/vjs-testkit/tests/e2e_gate_harness.rs`, `~/Projects/vibe-justice-system/crates/vjs-testkit/tests/structural_ceiling.rs`, `~/Projects/vibe-justice-system/crates/vjs-cli/src/local_ci.rs`, `~/Projects/vibe-justice-system/crates/vjs-cli/src/front.rs`, `~/Projects/vibe-justice-system/.vjs/enforcement-surface.lock`. Fixture: `/var/tmp/claude/vjs-cc16`.