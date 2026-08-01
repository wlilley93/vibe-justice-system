# Case file: what a kernel must do when it cannot resolve a lawpack

## The measured facts, neither side disputed

**F1.** `crates/vjs-cli/src/context.rs::load_lawpack` reads `<repo>/lawpack/v2`. If that
directory does not exist it returns a `Lawpack` whose eight vectors are all empty. The
comment in the source is `// Fallback: use empty lawpack`. No error is returned and no
warning is printed.

**F2.** `vjs invoke` accepts `--lawpack <LAWPACK>`. The value is used only as a LABEL: it is
printed, written into `.vjs/config.toml` as `lawpack = "..."` and into `.vjs/lawpack.lock`
as `lawpack_id`. It never reaches `load_lawpack`. Passing a correct absolute path to a real
lawpack directory changes nothing about what is loaded. Measured 2026-07-31 in
`vibe-design-system`: invoking with `--lawpack ~/Projects/vibe-justice-system/lawpack/v2`
produced the same empty-lawpack digest as invoking without it.

**F3.** The consequence is a silent, total loss of the citator in any subscriber repo that
does not vendor a copy of the lawpack. Same binary, same flags, measured the same minute:

| repo | `vjs lookup --issue enforcement` |
|---|---|
| `vibe-justice-system` (vendors `lawpack/v2`) | ACT-001:s1, s2, s3, s4 |
| `vibe-design-system` (does not) | *no output at all* |

**F4.** `vjs status` in the affected repo reports `VJS installed: true` and
`Lawpack: vjs-v2@0.1.0@0.1.0`. Nothing distinguishes a jurisdiction with the whole canon
from one with none.

**F5.** `compute_digest` hashes exactly one file: `lawpack/v2/manifest.toml`, 19 lines
carrying an id, a version, a status, two timestamps and a `[limits]` table. Every statute,
regulation, rule, order, spec, invariant, decision and obligation is outside the pin.
Measured: appending a line to `lawpack/v2/statutes/01-authority.yaml` left the digest at
`14cdb3337039ffdb…`, which is byte-identical to the digest `vibe-justice-system` has had
pinned in `.vjs/lawpack.lock` since 2026-07-27.

**F6.** Two of the four repos with a `.vjs/` directory (two subscriber repos) have no
`lawpack.lock` at all. Of the two that do, `agent-universe-v2` vendors the lawpack and
carries the real manifest digest; `vibe-design-system` does not and carries
`e3b0c44298fc1c14…`, which is the sha256 of the empty string.

**F7.** The identical defect one level down was found and fixed on 2026-07-27. The doc
comment on `overlay_filed_orders`, in the same file, twenty lines below `load_lawpack`,
reads: *"every issue presents as first-impression, because the check for existing law
returns the same answer whether or not any exists, so the S-11(c) prohibition on
re-litigating settled law could only ever be honoured from memory."* That was written about
`.vjs/orders/`. It describes the lawpack fallback exactly, and the lawpack fallback was left
in place in the same edit.

**F8.** `overlay_filed_orders` deliberately does NOT fail when the orders directory is
missing, and says why: *"a fresh subscriber repo has no orders yet, and refusing to route in
that state would be worse than the defect being fixed."* So the codebase already draws a
distinction between an absence that is normal and an absence that is a defect.

**F9.** An empty authority set does not fail open at the router. `court.rs::any_on_point`
returns `false` when the authority list is empty, so `detect_court_trigger` fires and
`vjs route` answers `CourtRequired: true`. Measured previously (2026-07-28) on an empty
scratch repo. The loss is to `lookup`, to the citator, to `audit`'s conformance enumeration,
and to every authority the router would otherwise have RETURNED alongside the verdict.

## The question

When the kernel cannot resolve a lawpack for a jurisdiction, must it refuse, or may it
proceed on an empty one? And must the lock pin the law, or is pinning the manifest enough?

## The case FOR keeping the fallback (and pinning the manifest)

1. **ACT-001:s5 says the kernel "does not litigate, judge, or call models. It returns
   bounded instructions."** A kernel that refuses to start is not returning a bounded
   instruction; it is returning nothing. Availability is a property of a kernel. Every
   command in the binary routes through `build_kernel_context`, so making it fallible turns
   a missing directory into a total outage of `status`, `lookup`, `docket`, `gazette` and
   `validate` in that repo - including the very commands an operator would use to diagnose
   the problem.

2. **F8 is the codebase's own settled position on absence.** The court has, in effect,
   already reasoned that a fresh subscriber repo is a legitimate state and that refusing to
   route in it "would be worse than the defect being fixed". A repo mid-invocation has no
   lawpack for the same reason it has no orders. Refusing there makes `vjs invoke` itself
   unable to run, since `invoke` calls `build_kernel_context` on line 20 - BEFORE it has
   written the config that would say where the lawpack is. On this reading the fallback is
   not an oversight but a bootstrap requirement, and the defect is only that it is silent.

3. **The empty lawpack fails CLOSED where it matters (F9).** The one outcome that would be
   dangerous - a governed act sailing through because no authority objected - does not
   happen. `any_on_point` returns false on an empty set and the router convenes a court. The
   system's own protection against an empty canon is already in place and already tested.

4. **On the digest: a lock is a subscription record, not an integrity check.** `lawpack.lock`
   sits beside `lawpack_id` and `lawpack_version`. Its job on this reading is to record WHICH
   lawpack a jurisdiction subscribed to, and the manifest carries the id, the version and the
   `[limits]` that bound every kernel answer. Hashing the whole tree makes the digest move on
   every editorial change to any statute, which would put every subscriber repo permanently
   out of date and train operators to re-lock without reading - the failure the VDS
   enforcement lock was designed to avoid.

5. **Nobody has been harmed.** Two of six repos lack a lock entirely and no ruling has been
   made per incuriam as a result. The defect is theoretical until a court actually decides a
   matter believing there is no binding authority.

## The case AGAINST the fallback (and for pinning the law)

1. **ACT-001:s5 is the clause on point and the fallback contradicts its operative verb.**
   "The kernel loads the lawpack and resolves authority deterministically." A kernel that
   resolves against a canon it silently substituted for the real one is deterministic in
   form and arbitrary in substance: the same question returns four constitutional sections in
   one repo and nothing in another, and neither answer is marked. Determinism that is not
   determinate ABOUT ANYTHING is not what s5 secures.

2. **The harm is precisely the harm the court already recognised in F7.** A ruling made in
   ignorance of binding law is per incuriam and void. In a repo with an empty lawpack EVERY
   matter presents as first-impression, because the check for existing law returns the same
   answer whether or not any exists. That sentence is already in this codebase, already
   accepted as stating a defect, and already acted on - for orders. The lawpack is the larger
   half of the same citator. Fixing the smaller half and leaving the larger is not a
   distinction, it is an omission.

3. **The bootstrap objection (case-for 2) is answered without keeping the fallback.** F8's
   reasoning turns on a state that is NORMAL - a fresh repo has no orders yet, and will
   acquire them by operating. A subscriber repo never acquires a lawpack by operating; it
   subscribes to one, and the subscription is the thing `invoke` is recording. So absence of
   orders is a stage and absence of a lawpack is a failure. The two are not alike. `invoke`'s
   own ordering is a fixable detail: it needs a digest of a lawpack it has been TOLD about
   (F2 shows it is already told and ignores it), not a context built from a config it has not
   written yet.

4. **F2 is independently a defect whichever way this goes.** A flag that is accepted,
   printed, and written into two artefacts, while having no effect on behaviour, is worse
   than no flag: it produces a record asserting a subscription that did not happen.
   `.vjs/lawpack.lock` in `vibe-design-system` currently names `vjs-v2@0.1.0` and pins the
   sha256 of the empty string. The record is not merely incomplete; it is false.

5. **F4 is the aggravating fact.** `vjs status` reports `VJS installed: true` and names a
   lawpack. An operator has no way, short of reading the kernel source, to learn that their
   jurisdiction has no law. A silent failure that also PRINTS A REASSURANCE is the class the
   Principal's standing instructions name repeatedly: configured is not safe; a check that
   cannot fail; an unresolvable dependency falling back is a bypass.

6. **On the digest: a pin over the manifest cannot detect the change it exists to detect.**
   The lock's purpose is that a jurisdiction can prove which law it is bound by. Under F5 any
   statute in the canon may be rewritten and every subscriber's lock still verifies. The
   case-for's churn objection is real but is an argument about CADENCE, not about scope: a
   digest that never moves when the law changes is not a low-churn pin, it is not a pin. The
   VDS precedent cited against this actually cuts the other way - that lock pins gate SOURCE
   digests and requires a rationale on re-pin precisely so a moved digest is a visible,
   answerable act.

7. **Nobody has been harmed YET (case-for 5) is an argument for acting now.** The Principal
   invoked `vibe-design-system` as a jurisdiction on 2026-07-31 with eleven submissions
   waiting for a bench. Those eleven would have been decided by a court that could not see
   ACT-001 at all.

## What the ruling must settle

(a) Whether `load_lawpack` may return an empty lawpack when it cannot resolve one, or must
    return an error.
(b) If it must refuse, how `vjs invoke` bootstraps, given it builds a kernel context before
    writing the config.
(c) Whether `--lawpack` must select what is loaded, or must be removed.
(d) Whether the lawpack digest is computed over the manifest alone or over the lawpack tree.
(e) What a jurisdiction whose lock records a false subscription (F6) must do about the record.
