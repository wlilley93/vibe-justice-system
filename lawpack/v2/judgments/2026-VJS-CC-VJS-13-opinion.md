# [2026] VJS-CC-VJS 13 - canon-write gate repo-code resolution in subscriber jurisdictions

**Court:** County (single judge, first instance).
**Jurisdiction:** vjs.
**Matter:** SUBMISSION-2026-08-01-123507.
**Convening:** CONVENING-county-2026-08-01-124120, case file
sha256:c0e288ed834c2d8e6d367cbeef9b7c49783b2d8039214200477e275eca72af12.

## 1. The question

When the PC-13 D1 canon-write gate runs in a jurisdiction whose `lawpack/v2` is a
lawful read-only mirror of foreign canon, from where must `canon_repo_code` be
resolved: the local jurisdiction config (the behaviour at the time of this
matter, `crates/vjs-engine/src/staged.rs` and `crates/vjs-cli/src/front.rs`), the
mirrored lawpack's own declaration, or should the gate not run in a mirror at all?

## 2. Decision and ratio

**Option A is adopted, with conditions.**

**RATIO: the repo_code a canon-write gate tests against is a property of the CANON
being written to, not of the repository hosting it. Where the resolved lawpack
declares its own `repo_code`, that declaration is the sole source of
`canon_repo_code`; the local jurisdiction config supplies it only when the lawpack
is silent.**

## 3. Reasoning

The defect is a category error, not a tuning problem. `config.repo_code`
truthfully answers "what is this repository's code". The gate asks a different
question: "whose canon is this, so that any other code in it is contamination". In
the canonical repo those two facts coincide and the bug is invisible. In a mirror
they diverge, and thirteen enacted canon records read as subscriber law filed into
canon. A gate that asks the right question of the wrong referent is not
over-strict; it is wrong.

The codebase already holds the characterisation that decides this.
`crates/vjs-core/src/hook.rs` records that in a subscribing jurisdiction the
lawpack "is the SUBSCRIBED-LAW MIRROR (read-only canon)" and excludes it from the
local jurisdiction's own court function for that reason. PC-19's runtime summary
says the same: a subscriber "may mirror read-only". The kernel therefore already
treats the mirrored tree as foreign canon everywhere except the one place where it
silently re-labels it as local.

The strongest argument AGAINST Option A is not the digest churn the submission
pleads. It is that under A the subscriber's local enforcement is configured by
bytes the subscriber received from upstream, which is uncomfortably close to the
"gates HOW, not WHAT" defect the submission rightly holds against Option B.

That argument is rejected on three grounds. First, it proves too much: the
mirrored statutes ARE the law this kernel loads and resolves authority from. A
subscriber who cannot trust the mirrored manifest cannot trust the mirrored
ACT-007 either, and the answer to that is the pin, not a second local opinion about
what the law says. Second, the value is not free-floating: `digest_of_lawpack_dir`
hashes the whole tree including `manifest.toml`, and a mismatch is Fatal
LAWPACK_LOCK_DRIFT. Under CC-VJS 12(d) a re-pin must carry a reason a reader can
answer. The declaration is therefore the most tamper-evident place in the
repository to put it, and strictly more protected than `.vjs/config.toml`, which is
merely permit-required. Third, the alternative is not "locally controlled" but
"locally wrong".

Option B fails on the file's own facts: a subscription records a lawpack id
(`lawpack = "vjs-v2@0.1.0"`), never a repo_code, so B must invent an inference or
carry N per-subscriber declarations, and it leaves the gate blind to foreign-coded
law authored locally.

Option C fails for a reason stronger than the one pleaded. The submission says C
leaves lock drift as the sole defence. It is worse: `.vjs/lawpack.lock` carries no
upstream attestation and `vjs invoke` regenerates it locally, so a local actor who
tampers with the mirror can silence the drift by re-pinning. C would leave a
defence that the tamperer holds the switch for. C also cannot be squared with
PC-13 D1, which directs the gate at `pre_write` and `validate --staged` in the
jurisdiction, without a seat-keyed exception.

The digest-churn objection is disposed of by binding precedent and is not
re-litigated: CC-VJS 12(d) held that churn is "an argument about CADENCE, not about
SCOPE", answered by making a re-lock a recorded act.

Option A also widens protection in the direction the law already points: in a
mirror, a record carrying the subscriber's own code is caught, which it previously
was not. That is ACT-007:s4 doing what it says.

## 4. Conditions

**C1 - one resolver.** Exactly one function computes `canon_repo_code`, called by
both `crates/vjs-engine/src/staged.rs` and `crates/vjs-cli/src/front.rs`. The two
chains already differed (only staged.rs carried the `"VJS"` tail). Two copies of a
rule is one copy and a disagreement.

**C2 - source order and root.** `lawpack/v2/manifest.toml` key `repo_code`, if
present and non-empty; else `config.repo_code`; else
`jurisdiction_id.to_uppercase()`; else `"VJS"`. The manifest must be read from the
same root that `in_canon` matches (`repo_root/lawpack/v2`), so the gate and its
referent describe one tree.

**C3 - a negative control.** A test with config `OPBOX` and manifest `VJS`
asserting a `VJS`-coded record passes and an `OPBOX`-coded record blocks; plus a
companion test that deletes `repo_code` from the fixture manifest and asserts the
`VJS`-coded record blocks again. Without the second, the manifest read is untested
decoration.

**C4 - no code capture.** A declared canon `repo_code` equal to any code in
`lawpack/v2/federation/subscriber-registry.yaml` is a blocking Error. Signal 4
skips codes equal to `canon_repo_code`, so an ill-set declaration would silently
blind the prose limb for that subscriber. A must not open that door while closing
another.

**C5 - the re-declaration must be audible, and audible findings must actually be
reported.** A staged change adding or altering `repo_code` in
`lawpack/v2/manifest.toml` emits a finding naming old and new value. This requires
repairing `crates/vjs-engine/src/staged.rs`: canon findings were pushed only
`if !check_public_safe(&canon)`, so every non-blocking canon finding, including the
existing Email and private-hostname warnings, was discarded whenever no Error was
present. A warning nobody can see is not a warning.

**C6 - both locks re-pinned in the same change.**

**C7 - the remedy text names the cure.** The foreign-`repo_code` block message must
tell the author to declare the canon's own `repo_code` in its manifest. This is what
makes the fork case self-curing: a fork under ACT-007:s6 that inherits the manifest
verbatim and then authors its own law will be blocked, and must declare its own
code. That is a feature, being exactly the lineage declaration s6 requires, but only
if the message says so.

## 5. On the re-pin, and the silent-lawpack fallback

The canon `lawpack.lock` re-pin belongs in the same change. `manifest.toml` is
inside `digest_of_lawpack_dir`, so the edit moves the digest the moment it lands;
splitting the re-pin leaves this repository Fatal on every `validate`, and leaves a
reader unable to distinguish intended drift from real drift. The re-pin must carry a
recorded reason citing this order, per CC-VJS 12(d). In the same change,
`.vjs/enforcement-surface.lock` must be re-locked for `crates/vjs-redact/src/lib.rs`
and `crates/vjs-engine/src/staged.rs`, both pinned there under PC-16 D4, with the
rationale recorded. Subscribers re-lock on their next pull; that is the ordinary
post-CC-VJS-12 regime for any canon change and not a reason to withhold the fix.

Fallback where a lawpack does not declare the field: the existing config chain,
unchanged, and non-blocking. Not fail-closed, because every lawpack in the
federation predates the field and failing closed would brick every subscriber to
cure thirteen errors in one. Not a union either: where the lawpack declares, the
config plays no part in this gate. The fallback is a legacy path, not a second
opinion.

## 6. Obiter (not decided; no part of the ratio)

(i) `crates/vjs-cli/src/front.rs` hardcodes `APEX_SEAT: &str = "vjs"` while
REG-FEDERATION-COORDINATION-001 declares `apex_court_jurisdiction: vjs` in the
canon. Same defect class as the one cured here: federation identity read from
somewhere other than the law.

(ii) `in_canon` in `scan_canon_writes` is hardcoded to `lawpack/v2` and never
consults `resolve_lawpack_dir`, so a jurisdiction whose lawpack resolves out of tree
via config `lawpack_path` or `VJS_LAWPACK` gets no D1 gate at all. Worth a look; not
before this court.

(iii) `.vjs/lawpack.lock` is absent from `permit_required` and carries no upstream
attestation. If the federation wants the pin to be a defence against local tampering
rather than only a change detector, that gap is where to start.
