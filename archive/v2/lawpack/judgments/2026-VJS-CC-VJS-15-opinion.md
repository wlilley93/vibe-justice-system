# [2026] VJS-CC-VJS 15 - the resolver as the sole naming route

**Court:** County, sitting at first instance. Single judge.
**Jurisdiction:** vjs.
**Matter:** SUBMISSION-2026-08-01-143848.
**Convening:** CONVENING-county-2026-08-01-143855, case file
sha256:2689246ff43de11babd0805be93479b3d3283f171aaef5bffa8622e33a28e174.

## 1. The question

Must every crate name the lawpack through `resolve_lawpack_dir`, and is the answer
different for `front_door::governed_record_roots` because its referent is the
repository's own records? The matter comes up on CC-VJS 14 obiter (iii), which
named three surviving `repo.join("lawpack/v2")` sites and expressly declined to
decide them.

## 2. Decision and ratio

**Option A is adopted, varied and extended.** B and C are refused.

**RATIO: where a site names the lawpack in order to READ the canon,
`resolve_lawpack_dir` is the sole source of the directory and
`vjs_engine::load_lawpack` the sole loader, in every crate, so that the refusal in
CC-VJS 12 D1 reaches every door and not only the one the ruling was implemented
against; where a site's referent is the repository's own governed records, it must
NOT be re-pointed at the resolver, but the literal must be collapsed into a single
named declaration of that local referent, and every literal that survives anywhere
must carry a machine-checked marker, so that a site which is correct because local
is distinguishable from one that is merely unfixed without a reader re-deriving
the referent.**

Narrowly: this decides the canon-READ sites in `vjs-cli/src/gazette` and
`vjs-mcp`, and the form of the local-referent roots list. It does not decide write
targets, and it does not disturb the staged-path family reserved by CC-VJS 13
obiter (ii) and CC-VJS 14 obiter (ii).

## 3. Reasoning

### 3.1 The facts were not taken on the pleadings, and three are wrong

**The pleaded enumeration of three sites is an undercount.** Excluding tests, the
tree carries NINE occurrences, and the MCP crate holds THREE, not one: the private
`load_lawpack`, a private `compute_digest` reading `manifest.toml`, and the record
verb's write target. CC-VJS 12 named `load_lawpack` AND `compute_digest` together
as the duplicated pair. Fixing one half of a pair a ruling named together is the
precise error CC-VJS 12's note on blame recorded.

**`LawpackLoader::load` cannot fail on a missing directory.** Every subtree read is
guarded by `.exists()` and the function returns `Ok` with eight empty vectors. So
the `?` at the Gazette's load site is inert: it looks like a refusal and is not
one. The MCP's private loader is the same thing written out longhand.

**The applicant's silence claim is half wrong.** Measured in a scratch jurisdiction
resolving out of tree, `vjs gazette` prints `0 items` and exits 0. The console is
not silent. It says zero. The pleading that it publishes "without saying so" is not
made out on the console, and the applicant should not have put it that way.

### 3.2 The Gazette failure is in the artefact, not the console

The console tells an operator a number. The artefact tells every later reader a
falsehood. Measured, in a fixture carrying a real lock over a canon holding 114
records, the published artefact read:

    "counts": { "archive": 0, "canon": 0, "total": 0 },
    "lawpack": { "digest": "sha256:5481b9e2…" }

That artefact asserts: this is the estate pinned at that digest, and it contains
nothing. It is not incomplete, it is false, and false in exactly the way CC-VJS 12
held a stale lock was false. Exit 0 and a console number do not cure a record; a
later reader has the file, not the terminal.

### 3.3 The MCP failure IS silent, and on one verb it is worse than silent

Measured at HEAD, same binary, same requests:

| call | out of tree | apex repo |
| --- | --- | --- |
| `vjs.lookup` | `{"authorities":[]}` | the full ACT-001 stack |
| `vjs.route` | `binding: []` | `binding: [ACT-001:s1 …]` |
| `vjs.status` | `vjs_installed: true` | `vjs_installed: true` |

Two points decide the silence question against Option C. First, `vjs.validate`
delegates to `vjs_engine::validate` and therefore already resolves correctly, while
`lookup`, `route` and `record` read a hardcoded directory: one server answers "is
this jurisdiction sound?" from the subscribed canon and "what is the law?" from a
directory that is not there. Second, the MCP's consumer is an agent, and an empty
`binding` array is not a visible failure, it is an ANSWER. `vjs.status` returning
`vjs_installed: true` is the aggravating fact CC-VJS 12 named: a silent failure
that prints a reassurance.

**And on `vjs.record` it is not silence at all, it is permission.** The verb loads
the lawpack to find the courts constitution and runs `verify_bench` inside
`if let Some(constitution)`. With an empty lawpack there is no constitution, so the
block is skipped. Measured, identical request, same binary: the out-of-tree
jurisdiction RECORDED a two-judge County order with no opinion source, where the
vendored control refused it with BENCH_SIZE_MISMATCH and BENCH_OPINION_MISSING.
The code's own comment says bench integrity is constitutive and never
assent-softenable, because assent cannot manufacture a quorum the constitution did
not seat. A hardcoded path switched that gate off wherever canon is remote. This is
no longer tidiness about a duplicated function.

### 3.4 The strongest argument against A, at its strongest

No party pleaded it. It is this: routing the Gazette through the resolver makes
publication depend on another repository's bytes, so a subscriber running
`vjs gazette` would publish the apex's records as its own estate. Today it
publishes nothing, which is inert; after the fix it misappropriates a canon. And
REG-GAZETTE-CONTINUITY-001 makes publication constitutively inert, so this is where
the applicant's urgency is weakest.

It is a good argument and it fails on the record. The Gazette hardcodes the apex
identity in its bases, its site link and its feed tag. Measured, the scratch
jurisdiction's feed carried the apex realm's tag and self-linked to the apex site.
`vjs gazette` is not today a per-subscriber publication instrument at all: every
artefact it emits, anywhere, already claims the apex realm. Against that
background the choice is between publishing the canon the lock actually pins and
publishing zero records under that same pin, and the second is worse, because it
is the one a reader can be misled by. Inertness answers whether publication makes
law. It does not answer whether a record may lie. The residue is met by a
condition, not by declining the cure: the artefact must record which tree it
published.

### 3.5 Option B is not merely wrong, it is mechanically broken

CC-VJS 9 D1 and CC-VJS 14 C3 dispose of it as authority. Two mechanical facts
confirm it from source. `check_citation_uniqueness` does `strip_prefix(repo)` to
name each record and filters canon YAML by a path suffix. Re-point the roots at a
resolved lawpack and both break: `strip_prefix` fails and findings report absolute
paths outside the repository, and a `lawpack_path` not literally ending in
`lawpack/v2` silently disables the canon-YAML filter. Separately,
`is_governed_record` classifies repo-relative paths taken from the git index, and
an out-of-tree lawpack is never in this repository's index. The predicate and the
scan would describe two different trees, which is the same defect one layer down.

So the answer to the second half of the question is yes: it is different for the
roots list, and different because of the referent, exactly as CC-VJS 14 held.

## 4. Must the surviving local literal be marked?

**Yes, and a comment is not enough.** The repository already tried the comment: a
redact-crate note says the manifest is read "LITERALLY, never through
`resolve_lawpack_dir`" and gives a good reason. It is a model of the reasoning and
it is unenforced prose. Two instruments are required, in order.

**First, structural.** `front_door.rs` states the root list TWICE while its doc
comment claims the second is derived from the first. It is a second hand-written
copy, which is CC-VJS 12's ratio breached inside the very function the applicant
asks to leave alone. The cure is one named const from which both the predicate and
the scan derive. Distinguishability by structure beats distinguishability by
annotation, because nothing has to be trusted to be read.

**Second, machine-checked.** Every literal surviving after that carries a marker of
fixed form naming its referent and citing authority, enforced by a workspace test
modelled on the live structural-ceiling precedent. An unmarked literal fails the
build. That is what converts a convention nobody enforces into a gate. The marker
is a DECLARATION, not an approval: a reserved site is marked as reserved, citing
the ruling that reserved it, so marking does not silently decide what stays open.

## 5. Must an out-of-tree load that resolves nothing produce a finding or error?

**Yes, and it already must; this order only stops the two doors routing around it.**
CC-VJS 12 D1 is binding and followed: in an invoked jurisdiction an unresolvable
lawpack is a failure and not a stage. For the MCP the cure and the refusal are one
act: delete the private loader, call the kernel's, and the refusal arrives with it.
The Gazette needs the directory rather than the loaded struct, so it takes the
directory from the resolver and reproduces the refusal, and it must refuse rather
than publish, because a publication step is the one place where continuing leaves
an artefact behind. For the MCP read verbs the refusal must be a JSON-RPC ERROR:
CC-VJS 14 reserved only whether a refusal is better expressed as a finding inside a
REPORT, and `lookup` produces no report and has no warning channel, so "empty with
a caveat" is unavailable in principle.

## 6. Conditions

**C1** No canon-READ site outside the resolver names the lawpack; the MCP's private
`load_lawpack` and `compute_digest` are gone and its verbs build context from the
kernel's.

**C2** The refusal reaches both doors: gazette exits non-zero naming all three
candidate sources and writes no artefact; the MCP read verbs return a JSON-RPC
error. The not-a-jurisdiction limb is unchanged.

**C3** No vacuous tests (CC-VJS 14 obiter (i)): every refusal assertion sits in a
fixture where success is reachable, proven by seeding.

**C4** The Gazette artefact names the tree it published: `meta.lawpack` records the
resolution source and path, and a test asserts a non-null digest never appears
beside a zero total.

**C5** One declaration of the local referent, from which the predicate and the scan
both derive, with a test that they cannot drift.

**C6** A marker gate that CAN fail, proven by seeded counterexample.

**C7** The enumeration is recorded, not just the cure: every surviving occurrence
listed with its referent and disposition, including those this order does not
decide.

**C8** The discovered write-target defect is filed as a fresh submission and cured
on its own record, not sub silentio inside this implementation.

**C9** Prove it at the destination, not in a fixture: record before and after.

## 7. Obiter (not decided; no part of the ratio)

**(i) A hardcoded WRITE target can silently displace a jurisdiction's whole canon.**
The record verb writes to a hardcoded `lawpack/v2/orders`, and the resolver prefers
a vendored directory over `lawpack_path`. Measured in one fixture, in order: lookup
returned the ACT-001 constitutional sections; the verb wrote one order to the
hardcoded path; the same command then returned only that one order, and status
reported the lawpack at the local path instead of the configured one, exit 0
throughout. The subscription was displaced by a one-order directory a write verb
created. Two honest qualifications: this order neither creates nor worsens it, and
it is not undetectable, because the next validate reported LAWPACK_LOCK_DRIFT as
Fatal, which is CC-VJS 14's cure working. But the pin detects it after the fact and
calls it drift, not displacement, and the two have different cures. That is C8.

**(ii) The overlay-floors default resolves to nothing.** A CLI default points the
canon Tier-1 floors at a directory that does not exist in this canon at all, and
the loader returns empty for a missing directory, so the floor set is empty
everywhere and local rules dispose with no canon ceiling above them. A different
defect from the one before me, the same shape.

**(iii) The MCP's private digest was inert today.** It hashed the manifest alone,
the superseded computation CC-VJS 12 D4 rejected, but its value reached nothing
load-bearing. It is ordered deleted because it is a wrong copy of a rule, not
because harm has been measured through it. A wrong copy waits for a caller.

**(iv) CC-VJS 14 obiter (iii) undercounted, and obiter is where undercounts
survive.** It named three sites; there are nine. No criticism attaches: an obiter
note is not a survey. The general point is that an enumeration offered in passing
tends to be quoted as complete. C7 exists because of it.

**(v) The staged-path family remains reserved.** The question there is not which
directory but whether a read-only mirror should be gated at all. C6's marker
requires those sites to be labelled reserved, which records the open question
rather than closing it.

**Blame:** none apportioned. The duty under ACT-001 is to make the work good, and
the conditions are that remedy.
