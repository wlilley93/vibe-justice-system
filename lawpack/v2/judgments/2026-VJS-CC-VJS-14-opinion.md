# [2026] VJS-CC-VJS 14 - the guard on the lawpack integrity gates

**Court:** County, sitting at first instance. Single judge.
**Jurisdiction:** vjs.
**Matter:** SUBMISSION-2026-08-01-134856.
**Convening:** CONVENING-county-2026-08-01-134919, case file
sha256:0e0a33ff9b0cabe3eeefa706bda9c4a07e32f6f28a1406bc4283d13a90eb636d.

## 1. The question

`validate` computed `let lawpack_dir = repo.join("lawpack/v2")` and wrapped three
checks in `if lawpack_dir.exists()`: referential integrity, citation uniqueness
(ACT-004:s8, collisions Fatal) and LAWPACK_LOCK_DRIFT (ACT-007:s7, Fatal). The
kernel resolves the lawpack elsewhere, through `resolve_lawpack_dir`, which
honours config `lawpack_path`. Where a jurisdiction's lawpack resolves out of
tree the guard is false and all three checks are skipped silently.

Must those gates run against the lawpack actually loaded, and what is the correct
source of the directory they test?

## 2. Decision and ratio

**Option A is adopted, varied.** A is right that the hardcoded path must go. It
is wrong to assume all three checks take the same cure, because they do not share
a referent.

**RATIO: a gate's guard must be keyed to the same referent as the gate. Where a
check's referent is the lawpack, `resolve_lawpack_dir` is the sole source both of
the directory it tests and of whether it runs; where a check's referent is
something else, the existence of a local `lawpack/v2` is not a lawful condition on
it at all and must be removed rather than re-sourced.**

This is CC-VJS 13's ratio applied one layer out. That case held a gate must test
the thing actually governed rather than a locally convenient proxy. A guard is
part of the gate. A gate aimed correctly and switched on by a proxy is switched on
wrongly, which in this repository means switched off exactly where the law is
remote.

| check | referent | correct condition |
| --- | --- | --- |
| referential integrity | the lawpack tree | `resolve_lawpack_dir(repo)` is `Some`, and that same `PathBuf` is scanned |
| citation uniqueness | `governed_record_roots(repo)`, the LOCAL records | none: it runs unconditionally |
| LAWPACK_LOCK_DRIFT | the pinned lock versus the resolved digest | a lock exists |

Narrowly: this decides the three checks inside `validate`. It does not decide
staged-path gates, and it does not decide whether every crate must name the
lawpack through the resolver.

## 3. Reasoning

### The measured state

The facts were not taken on the pleadings. On this machine, this day:
`vibe-design-system` printed `Validation: OK`, exit 0, while its lock pinned
`sha256:8f724ac0` and the canon it loads hashed to `sha256:f16a14c4`. A negative
control in scratch went further: invoke a fresh jurisdiction out of tree, then
overwrite the lock digest with a falsified value. `vjs validate` printed
`Validation: OK`, exit 0. **A deliberately falsified pin could not be detected in
an out-of-tree jurisdiction.** ACT-007:s7's Fatal was not dormant there. It was
unreachable.

### The strongest argument against, at its strongest

The file pleads that Option A turns three checks on where they have never run and
makes validate depend on a path outside the repo. Neither is the best form. The
best form is: under A, a subscriber's build can go red because of bytes a stranger
changed in another repository, with no local edit and no local act.

It fails on three grounds.

**First, it is spent.** `validate` already depends on that path, before it reaches
the guard: `load_lawpack(repo)?` resolves through the same function, and where the
configured path is absent it does not warn, it refuses, citing CC-VJS 12 D1.
`compute_digest` already resolves too. A adds no new dependency and no new failure
mode. It removes an incoherence in which one side of a single comparison resolves
and the other does not.

**Second, the dependency is what a subscription is.** CC-VJS 12 held a lawpack is
never acquired by operating; it is subscribed to in a deliberate act. A subscriber
is by definition bound by law it does not hold, and the instrument that makes that
safe is the pin. Switching the pin's own check off in precisely the configuration
where the law is remote inverts the instrument. In tree, a canon change is visible
in the repository's own diff. Out of tree, the pin is the only observation there
is, and the guard disabled the check in the sole case that is not otherwise
observable.

**Third, the statutes make these unconditional.** ACT-007:s7 carries
`must: check_lawpack_lock_consistency`; ACT-004:s8 carries
`must: check_citation_uniqueness` with `collision_policy: fatal`. Neither mentions
a directory layout. A `must` conditioned on a filesystem convention the statute
never names has been narrowed by implementation, not by law.

One further fact decides it against any reading that this was a tolerable gap.
**The test written to protect this rule was vacuous.**
`the_lock_invoke_writes_is_the_one_validate_checks` invoked a scratch repo with an
out-of-tree lawpack and asserted the ABSENCE of LAWPACK_LOCK_DRIFT. In that
fixture the guard is false, so the assertion passed whatever the lock said. It was
the exact configuration in which a falsified pin goes green. Its own comment says
the defect it catches "cost the whole first implementation of this order". It
caught nothing.

### Options B and C

B is the Fatal's own warning about itself, deferred, and is incoherent to
implement honestly: to report "these checks were skipped and it mattered" you must
already have resolved the directory, at which point declining to run them is a
choice. B is weaker here than usual, because CC-VJS 13 recorded that the lock
carries no upstream attestation and is regenerated locally, so drift detection is
already the thin end of the defence.

C is refused. `lawpack_path` is not tolerated legacy: CC-VJS 12 built it, held a
flag must select or go, and directed a subscriber to re-invoke through it. C would
overrule that ratio without a distinction, which is not open to a coordinate
court, and is unnecessary because A delivers the guarantee C wants.

### The transition

"Three checks that have never run, one Fatal" is the correct description of the
risk and the wrong description of the exposure. Counted separately, and measured:
referential integrity yields nil, provably, because after the fix both the scanned
tree and the defined-id set come from the one resolved directory, which is this
repository's own lawpack where the check already runs clean. Citation uniqueness
yields nil, provably, and never needed the lawpack: its register is the local
governed roots, and the one affected repository carries seven orders with seven
distinct citations. Lock drift fires, in one known jurisdiction, because it is
true.

So the transition is one true Fatal, in one enumerated repository, with a
one-command cure and no code change required of the subscriber.

**The Fatals take effect immediately. No stage.** A Fatal held at Warning for a
release is Option B wearing a calendar, and is worse than the present silence in
one specific way: for the window the record would say the lock was checked and
passed, which a later reader can point at as assurance. Silence at least misleads
no one who looks. A grace period is the right instrument when the blast radius
cannot be enumerated. Here it has been enumerated by measurement, and it is one.

What the subscriber is owed is not a softer severity but not being left red. That
duty falls on the implementer, and it is C7 to C9.

## 4. Conditions

**C1** The hardcoded guard is deleted. `grep -n 'join("lawpack/v2")'` over
`vjs-engine/src/lib.rs` returns exactly one hit: the vendored-first branch inside
`resolve_lawpack_dir`.

**C2** Referential integrity scans what was loaded: called with the `PathBuf`
returned by `resolve_lawpack_dir(repo)`, the same value the passed lawpack was
loaded from.

**C3** Citation uniqueness is unconditional and keeps its register
(`governed_record_roots`); it is not re-pointed at the resolved lawpack, which
CC-VJS 9 D1 bars: the allocator and the guard must read the same register, and a
guard wider than the allocator fails on records the allocator cannot see.

**C4** Lock drift is conditioned on the lock: it runs whenever
`read_lawpack_lock` yields one.

**C5** No silent arm. An unparseable lock, or a digest that cannot be computed,
must each produce a finding naming which half failed, rather than deleting the
Fatal as thoroughly as the guard did.

**C6** The vacuous test is repaired, not merely joined. An assertion of absence
run in a fixture where the finding is unreachable is not coverage.

**C7** Enumerate the affected set by measurement before landing: every repository
whose config carries `lawpack_path` or resolves via `VJS_LAWPACK`, with its pinned
and computed digests, recorded in the compliance record.

**C8** Re-pin in this order, last one last: the code change, this opinion and the
order land; then this repository's lock is re-pinned with a recorded reason citing
this order, per CC-VJS 12(d); then and only then the subscriber's lock is re-pinned
to that same final digest. Re-pinning the subscriber first re-stales it the moment
the judgment lands. Checkable: the two locks carry an identical digest.

**C9** Prove it at the destination, not in a fixture. In the subscriber, after C8:
validate exits 0; then temporarily falsify the lock digest and confirm exit 1 with
LAWPACK_LOCK_DRIFT; then restore. A check verified only in the fixture is verified
in the repository that never had the defect.

## 5. An out-of-tree lawpack that cannot be read

**Fatal. Neither Warning nor silent.** This is already binding law and is followed
rather than re-opened: CC-VJS 12 D1 held that in an invoked jurisdiction an
unresolvable lawpack is a failure and not a stage, and it is enforced more strictly
than a Fatal finding, by refusal naming all three candidate sources.

This holding does not create that dependency; `load_lawpack` already had it. What it
must not do is create a new silent path around it. The only circumstances in which
the three checks do not run are a repository that is not a jurisdiction, where
CC-VJS 12 expressly preserved the empty-canon limb and the local-record checks are
self-limiting because no governed root exists, and the refusal already in force.
There is no third state in which validate prints OK without having run them.

Whether that refusal is better expressed as a Fatal finding inside the report than
as a hard error is not decided. It was not argued, and the answer is not silence.

## 6. Obiter (not decided; no part of the ratio)

(i) The vacuous-test point generalises past this order: a test whose assertion is
the ABSENCE of a finding proves nothing unless its fixture is one in which that
finding is reachable.

(ii) CC-VJS 13's obiter (ii) is NOT cured by this ratio and should not be assumed
to be. `in_canon` and its siblings test repo-relative paths taken from the git
index. An out-of-tree lawpack is never staged in the subscriber's index, so the
question there is not "which directory" but whether a read-only mirror should be
gated at all. Different in kind, still open.

(iii) Three `repo.join("lawpack/v2")` sites survive this order, in the gazette, the
MCP crate, and the governed-roots list. Same class, not before this court. A court
asked should consider whether the resolver ought to be the only way any crate names
the lawpack, and whether the answer for the roots list is different because its
referent is local.

(iv) The one affected subscriber is live, with a public remote and seven orders, and
does not appear in the federation subscriber registry. Every protection keyed on
that list, including CC-VJS 13 C4 and the D1 gate's prose limb, therefore does not
reach it.

(v) The lawpack report maps `must: check_lawpack_lock_consistency` to an
implementation, and that mapping was true throughout the period in which the check
could not fire. An attribution records that a gate exists, not that it executes.

**Blame:** none apportioned. The defect is one line left behind by a ruling that
fixed the harder half, and the duty is to make the work good.
