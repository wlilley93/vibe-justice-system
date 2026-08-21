I have completed my verification. Delivering the opinion.

---

# [2026] VJS-CC-VJS 16 - the write verb and the directory the resolver prefers

**Court:** County, sitting at first instance. Single judge.
**Jurisdiction:** vjs.
**Matter:** SUBMISSION-2026-08-01-145337, filed under [2026] VJS-CC-VJS 15 C8.
**Repository state:** HEAD `c7b3ba2` ("[2026] VJS-CC-VJS 15: one resolver, one loader, every crate"), working tree clean but for four untracked court records.

---

## 1. The question

May a WRITE verb create the directory the resolver prefers, and if not, what prevents a jurisdiction's canon being replaced by its own record-keeping?

Two subsidiary questions are expressly put by the relief, and I answer both because they are the difference between a cure and a patch: whether the resolver's vendored-first preference should require the directory to be a DECLARED mirror rather than merely present, and whether displacement needs a finding distinct from drift.

CC-VJS 15 obiter (i) reserved this exact write target and decided nothing about it. This is first impression.

---

## 2. Decision and ratio

**Option B is adopted, varied and extended. A and C are refused.**

**RATIO: a verb that records a jurisdiction's own governed record writes it to that jurisdiction's local record store and never into the canon tree, and no kernel write path may bring into being the directory the resolver reads the canon from; where the canon tree is a lawful write target at all it is by a deliberate, permitted authoring act, not as the side effect of a verb. And the resolver's vendored-first branch must be keyed on a directory that DECLARES itself a lawpack rather than on a path merely existing, so that where a jurisdiction's recorded subscription and its resolved source disagree the kernel says DISPLACEMENT in its own words, a finding a re-pin does not answer.**

Narrowly: this decides the MCP `record` verb's write target, the class of write sites that may create the resolver's directory, and the predicate on the vendored branch where a subscription is recorded. It does not decide when a filed order is promoted into canon, it does not decide whether vendored-before-config is the right order where BOTH sources declare, and it does not disturb the staged-mirror family reserved by CC-VJS 13 obiter (ii) and CC-VJS 14 obiter (ii).

---

## 3. What I measured, and where the case file is wrong

I did not take the facts on the pleadings. I built the fixture the court asked for: a scratch jurisdiction at `/var/tmp/claude/cc16/sub`, invoked with `--lawpack ../canon-host/lawpack/v2` against a copy of this canon, vendoring nothing.

**The core pleading is made out, in order, on this machine, today:**

| step | measured |
| --- | --- |
| `vjs lookup --issue enforcement` | the five ACT-001 constitutional sections, exit 0 |
| `vjs status` | `Lawpack: vjs-v2@0.1.0@0.1.0 (/var/tmp/claude/cc16/sub/../canon-host/lawpack/v2)` |
| MCP `vjs.record`, one valid County order | `{"recorded":"2026-EXAMPLE-CC-999","path":".../sub/lawpack/v2/orders/2026-EXAMPLE-CC-999.yaml"}`, exit 0 |
| `vjs lookup --issue enforcement` | `2026-EXAMPLE-CC-999 (CountyCourt): fixture` and nothing else, exit 0 |
| `vjs status` | `Lawpack: vjs-v2@0.1.0@0.1.0 (/var/tmp/claude/cc16/sub/lawpack/v2)` |
| `vjs validate` | `[Fatal] LAWPACK_LOCK_DRIFT lock=sha256:60aead5c… computed=sha256:bd940d97…`, exit 1 |

One `record` call replaced a 160-file constitutional canon with a one-file directory. The order was valid: bench of one, a real opinion at `opinions/fixture-opinion.md`, `verify_bench` reached and passed. Nothing was malformed. This is what the verb does when it works.

Five corrections to the record follow. Three are against the applicant, and two are corrections in the applicant's favour: this is worse than pleaded.

### 3.1 The pleaded digests are not this defect's digests (against the applicant)

Fact 6 pleads `lock=sha256:5481b9e2 computed=sha256:de8b8802`. Neither number is reproducible today. Computed independently, with a script I wrote to replicate `digest_of_lawpack_dir` and validated against the kernel's own output (it reproduces `60aead5c…` and `bd940d97…` byte for byte):

- this canon hashes to `sha256:60aead5ca30946dc…` over 160 files;
- `sha256:5481b9e2…` is the digest this canon carried BEFORE CC-VJS 15 landed, and it is still what the live subscriber `vibe-design-system` pins in `.vjs/lawpack.lock`.

No impropriety: the fixture was built before the canon moved under it. But a pleading that quotes a digest pair is asserting a measurement, and a court that repeats it without checking republishes a stale one. The pleading should have been re-measured before filing. (The agreed facts also skip from 4 to 6; there is no fact 5.)

### 3.2 `vjs status` does not say what the pleading says it says (against the applicant, and worse)

Fact 4 pleads that status "reported the lawpack at `./lawpack/v2`". It does not. It prints the absolute resolved path, and it prints it beside the label read from the lock:

    Lawpack: vjs-v2@0.1.0@0.1.0 (/var/tmp/claude/cc16/sub/lawpack/v2)

That is worse than pleaded, and it is worse in the exact way this court has already condemned twice. CC-VJS 12 D6 held against a status line that "printed a reassurance" over a jurisdiction with no law; CC-VJS 15 §3.3 named `vjs_installed: true` as the aggravating fact. Here the reassurance is stronger: the line asserts subscription to the canonical lawpack `vjs-v2@0.1.0` while naming a directory that holds one local order and no manifest. The cure landed in CC-VJS 12 keys on whether a lawpack RESOLVED, and one always does after displacement.

### 3.3 The pleading is not reproducible on a config `vjs invoke` writes, and the masking is an aggravation (in the applicant's favour)

My first `record` call failed:

    {"error":{"code":-32603,"message":"serialization error: TOML parse error at line 8, column 1\n8 | [paths]\nmissing field `specs`\n"}}

`crates/vjs-cli/src/invoke.rs:45` writes a `[paths]` table carrying `orders, logs, submissions, proofs, permits, private`. `PathsConfig` (`crates/vjs-store/src/lib.rs:388`) requires nine fields including `specs`, `decisions` and `cache`, none defaulted. So `Store::read_repo_config` errors, and the record verb calls it at `crates/vjs-mcp/src/lib.rs:317` for the PC-19 apex-routing check before it writes anything. In a jurisdiction `vjs invoke` actually created, `record` refuses.

I reproduced the pleaded displacement only after adding the three missing keys by hand.

This is not a mitigation. It is an aggravation, on two grounds. First, the protection is an accident: no test asserts it, no comment claims it, and nothing would notice its removal. Second, it is due to be removed - `invoke`'s template must be corrected, because the same defect makes `vjs validate --staged` and `vjs local-ci` hard-error before any gate runs (obiter (i)). The day that is fixed, this displacement becomes reachable in every subscriber, and the fix will look unrelated. One defect masks the next, and the mask is scheduled for deletion.

### 3.4 The pin does not detect displacement; it ratifies it (in the applicant's favour)

Fact 6 pleads that the defect "is detected AFTER the fact" by LAWPACK_LOCK_DRIFT, "which is CC-VJS 14's cure working as designed". Measured, the Fatal's own remedy completes the harm. Its `suggested_fix` reads: "Re-pin the lock (vjs invoke regenerates .vjs/lawpack.lock) only after confirming the lawpack change is intended." So I did that, on the displaced fixture:

    $ vjs invoke --jurisdiction acmeco --principal will
      lawpack: vjs-v2@0.1.0 (sha256:bd940d9738208e40...)
      config written: false
    $ vjs validate      -> Validation: OK, exit 0
    $ grep lawpack_path .vjs/config.toml
      lawpack_path = "../canon-host/lawpack/v2"
    $ vjs lookup --issue enforcement
      2026-EXAMPLE-CC-999 (CountyCourt): fixture

The jurisdiction is now green and lawless. Its config still declares a subscription to the real canon. Its lock now certifies the digest of a one-order directory under the id `vjs-v2@0.1.0`, which is precisely CC-VJS 12 D3's vice: a label recorded as though a subscription had happened when it had not. `config written: false` is the sharp edge - the re-pin does not even disturb the false declaration it contradicts.

A finding whose prescribed cure cements the defect is not a detection. It is a trap.

### 3.5 Displacement silently disables the canon-sourced half of enforcement (in the applicant's favour, and the largest correction)

The pleading treats the harm as "lookup returns one order". It is much wider. Staging the displaced record and running the commit gate:

    [Info]  INVARIANTS_PASS: 0 invariants evaluated, all passed
    [Fatal] PERMIT-MISSING: 'lawpack/v2/orders/2026-EXAMPLE-CC-999.yaml' not covered by an active permit
    [Fatal] INSTALL_HOOKS_MISSING

**Negative control**, because an assertion of absence proves nothing unless the finding is reachable (CC-VJS 14 obiter (i)): the identical record, at the identical path, in a repository that vendors this real canon (`manifest.toml` declaring `repo_code = "VJS"`):

    [Error] CANON_BOUNDARY_VIOLATION: Canon record carries a subscriber repo_code 'EXAMPLE' (canon is 'VJS')
    [Error] CANON_BOUNDARY_VIOLATION: Canon record id '2026-EXAMPLE-CC-999' embeds subscriber repo_code 'EXAMPLE'
    [Error] CANON_BOUNDARY_VIOLATION: Canon record names subscriber 'EXAMPLE' in its body/prose
    [Fatal] BENCH_OPINION_MISSING
    ... and eight invariant failures

So the gate whose entire purpose is to stop a subscriber's law masquerading as canon goes silent in the one case where a subscriber's law IS masquerading as canon. The mechanism is exact and it is CC-VJS 13's own instrument turned around: `resolve_canon_repo_code` (`crates/vjs-redact/src/lib.rs:75`) reads `repo_root/lawpack/v2/manifest.toml`, the manufactured directory has none, and the documented fallback chain names `config.repo_code` - the subscriber's own code. Against a canon that declares nothing, everything local is native.

And every canon-sourced invariant evaluates over an empty set and reports `all passed`.

That is the finding that decides the "distinct cure" question. Displacement is not a stale pin. It is the silent disabling of the canon-sourced half of the enforcement surface, reported as a pass.

---

## 4. Reasoning

### 4.1 Option A is refused, and the pleaded reason is not the good one

The file pleads that writing into a subscribed lawpack "mutates canon the subscriber does not own, which ACT-007:s4 forbids outright". Read, s4 says local orders "do not bind other repos unless adopted by those repos". It is a rule about the REACH of law, not about filesystem writes. The nearer provision is s3's `must_not: local_law_override_canonical_without_authority`. The conclusion survives the correction, on two stronger grounds.

**First, A converts a one-jurisdiction defect into a federation-wide one.** The digest covers the whole tree (CC-VJS 12 D4). Measured: this canon hashes to `60aead5c…`; adding one order file in my control fixture moved it to `4fa4693d…`. So under A, one subscriber's `record` call puts EVERY other subscriber into LAWPACK_LOCK_DRIFT, with no local act of theirs. CC-VJS 14 held that a subscriber is owed the pin as the instrument that makes remote law safe. A makes the pin fire on strangers' record-keeping.

**Second, and decisively, A writes where no gate can look.** The permit gate, the canon-write scan, the apex bright line, the assent floor and the redact scan all take repo-relative paths from the git index. A file written to `../vibe-justice-system/lawpack/v2/orders/` is in no index of the writing repository. A does not merely put the record in the wrong place; it puts it beyond the front door PC-14 built. That is the same structural error CC-VJS 15 §3.5 found in the mirror-image proposal, one layer over.

### 4.2 Option C is refused

As pleaded, C needs the resolver to know a directory's provenance, and nothing records it. Inventing a provenance file would be a second source of truth about the canon, which is CC-VJS 12's ratio in reverse. It is also unnecessary. The system already HAS the canon's self-declaration, and already relies on it: `manifest.toml`, read by `resolve_canon_repo_code` and by `lawpack_id_of`. Keying the resolver on a declaration the tree already carries is strictly narrower than minting a new one.

### 4.3 The pleaded argument against B is half true, and on measurement it supports B

The file pleads: "the apex repository does file canon orders under `lawpack/v2/orders`, so the rule must distinguish canon-authoring from subscriber record-keeping". Measured over this repository:

- `lawpack/v2/orders` holds 37 records: the eleven BOOT orders, the courts constitution, PC 1 to 19, and the SC line.
- `.vjs/orders` holds 22: **every County order this court has ever made**, plus `[2026] VJS-PC 20` and `[2026] VJS-PC 21`.
- The two sets are **disjoint**: zero ids in common.
- **No CLI path writes to `lawpack/v2/orders` at all.** `vjs order apply` calls `Store::write_order`, which targets `.vjs/orders` (`crates/vjs-store/src/lib.rs:77`).

So the distinction B is said to need is one the repository already draws in practice, and only the MCP verb disagrees with it. Two doors to one kind of governed record, with two destinations, is CC-VJS 12's ratio unapplied to writes.

### 4.4 The strongest argument against B was not pleaded. It is this

**At HEAD, the MCP's own read verbs cannot see `.vjs/orders`. Move the write target and the record verb writes into a register the same server cannot read.**

Measured, on a clean fixture with the order in `.vjs/orders` and nothing vendored:

- `vjs lookup --issue scratch-fixture` returns `2026-EXAMPLE-CC-999 (CountyCourt): fixture` first, then the constitutional stack.
- MCP `vjs.lookup` on the same issue in the same repository returns 151 authorities, every one of them canon, and **not the order**.

The CLI builds its graph through `build_kernel_context` (`crates/vjs-cli/src/context.rs:5`), which calls `overlay_filed_orders`. The MCP builds its own in `vjs_mcp::build_context`, which does not. Which means: **the only reason the MCP could ever read back what it recorded was that the write displaced the canon.** The hardcoded target was not arbitrary. It was load-bearing for a coherence the server otherwise lacks.

That is a real objection and it does not save the write target. It identifies a second defect the cure must carry, and the answer is the one this court has now given three times: not two readers with different reach, but one. No criticism attaches to CC-VJS 15 for leaving it: that ratio governs canon READS, and `overlay_filed_orders` is a read of the repository's own records, so it fell outside. But the reasoning applies with full force one layer out. Two context builders are one builder and one silent disagreement. It is C4 below, and the ruling does not land without it.

### 4.5 A second unpleaded objection: does requiring a declaration break existing repositories?

Requiring `manifest.toml` on the vendored branch could make a manifest-less vendored tree unresolvable, and CC-VJS 12 D1 makes unresolvable a hard failure, not a stage. That is a real cost and it must be enumerated by measurement, not assumed away (CC-VJS 14 C7).

Enumerated on this machine, every jurisdiction on disk: the apex and a copy of it, and `agent-universe-v2`, all three vendoring a manifest-bearing tree; and `vibe-design-system`, vendoring nothing and subscribing out of tree. Exposure: nil. In the test suite, the out-of-tree fixtures each assert `!repo.join("lawpack/v2").exists()`, and the vendoring fixtures copy the real canon, manifest included.

And C3 is narrower still: the declaration is required only where the vendored candidate would DISPLACE a recorded subscription. A repository that vendors and subscribes to nothing else keeps today's behaviour exactly. The behaviour change is confined to the configuration in which the harm lives, which is the smallest cure that reaches it.

### 4.6 Displacement needs its own finding, on three grounds

**(i) The existing finding's remedy is wrong for it.** Measured at 3.4: the re-pin ratifies the displacement and returns exit 0. A cure that completes the disease must not be the only thing on offer.

**(ii) In the field the two are already confounded.** `vibe-design-system` pins `5481b9e2…` while the canon it names hashes to `60aead5c…`. It is showing drift RIGHT NOW for an entirely legitimate reason: the canon moved when CC-VJS 15 landed. An operator in that state, reading the drift Fatal and its instruction to re-pin, has no way to tell whether the re-pin will subscribe them to the canon or to a directory a verb made.

**(iii) They are findings about different things.** Drift is a statement about BYTES: the tree I loaded is not the tree I pinned. Displacement is a statement about IDENTITY: the tree I loaded is not the tree I said I subscribe to. The jurisdiction's own record of its subscription, `lawpack_path` in `.vjs/config.toml`, survives displacement untouched, still true as a declaration and false as a description. Nothing in the kernel compares the two. That comparison IS the finding, and it is cheap: `resolve_lawpack` already returns which source answered (`LawpackSource`, CC-VJS 15 C4), and today only the Gazette consumes it.

---

## 5. Conditions

Each is checkable against the repository, not aspirational.

**C1 (the write target).** `handle_record` writes through `vjs_store::Store::write_order`, the same function `vjs order apply` calls, so the two doors have one destination. Checkable: `grep -n 'lawpack/v2' crates/vjs-mcp/src/lib.rs` returns no line inside `handle_record`, and the `LAWPACK-LITERAL: referent=write-target; status=reserved` marker at `crates/vjs-mcp/src/lib.rs:365` is gone because the site it declared is gone. The PC-19 apex-routing refusal stays exactly where it is: it is a check on the typed `order.court`, and it does not move with the path.

**C2 (no write path creates the resolver's directory).** No non-test source under `crates/` calls `create_dir_all` on a path ending `lawpack/v2` or any child of it. Checkable today: that grep returns exactly one hit, `crates/vjs-mcp/src/lib.rs:371`, and after this order it must return none. Stated as a class and not as one caller, because the defect is the class: the resolver's directory must not be creatable as the side effect of any write.

**C3 (the vendored branch tests a declaration, not a path).** In `resolve_lawpack`, the vendored candidate is accepted only if it declares itself a lawpack (a readable `manifest.toml` in that directory) WHERE the repository records another source (`lawpack_path` in `.vjs/config.toml`, or `VJS_LAWPACK`). Where no other source is recorded the branch is unchanged, so CC-VJS 12 D1's refusal is not widened and no existing fixture moves.

> **What would make this test vacuous:** asserting that `lookup` still answers from the configured path in a fixture that has no vendored directory at all. That fixture passes whatever the predicate says, and it is the exact shape CC-VJS 14 C6 condemned.
> **How to prove it can fail:** the fixture must CREATE `<repo>/lawpack/v2/orders/<id>.yaml`, assert the file is on disk (a seed that silently misses reads like a dead gate), assert `lookup` still returns the canon; then write a `manifest.toml` into that same directory and assert the resolution DOES move. Both directions are required: the first alone would pass on a resolver that had simply been switched to config-first, and it is the DECLARATION that must be doing the work, not the file count.

**C4 (one context builder).** `vjs_mcp::build_context` is deleted; both doors build the authority graph from one function in `vjs-engine` that overlays the local order register. Checkable: `overlay_filed_orders` (or its successor) has exactly one definition in the workspace, and `grep -rn "build_authority_graph" crates/*/src/` shows one assembly site.

> **What would make this test vacuous:** recording through the MCP verb and reading it back in a repository that vendors the canon. The record would be visible either way, so the assertion cannot fail on the defect.
> **How to prove it can fail:** the fixture vendors NOTHING and resolves out of tree, and it must assert BOTH that the local order is returned AND that the canon stack is still returned. The failure being cured is one register replacing the other, so a test that only counts the local order passes on the very displacement this order forbids.

**C5 (displacement is its own Fatal, and it names the disagreement).** Where `.vjs/config.toml` records a `lawpack_path`, or `VJS_LAWPACK` is set, and `resolve_lawpack` answers from a different source, `validate` emits a Fatal distinct from `LAWPACK_LOCK_DRIFT`, naming the recorded subscription AND the directory that answered, whose `suggested_fix` is to remove the directory that should not be there and never to re-pin. It is keyed on the config and the resolution, NOT on the federation subscriber registry, for the reason at obiter (iv).

> **What would make this test vacuous:** asserting the finding is ABSENT in a healthy jurisdiction. CC-VJS 14 obiter (i) squarely: an assertion of absence proves nothing unless the fixture is one where the finding is reachable.
> **How to prove it can fail:** seed the displacement (create the directory, assert it exists), assert the new code is PRESENT, and separately assert that the drift finding's "re-pin the lock" text is NOT what is offered as the cure. The second assertion is the one that matters, because a displacement finding that still recommends a re-pin has changed the label and kept the trap.

**C6 (the re-pin cannot ratify).** `vjs invoke` with no `--lawpack` refuses to pin over a source that contradicts the `lawpack_path` already recorded, rather than regenerating the lock. Checkable by exactly the measurement at 3.4: after seeding a displacement, `vjs invoke --jurisdiction x --principal y` exits non-zero and `.vjs/lawpack.lock` is byte-identical before and after. Today it exits 0, prints a digest, and the jurisdiction goes green.

**C7 (prove it at the destination, not in a fixture; CC-VJS 15 C9).** The destination is `vibe-design-system`. It cannot demonstrate that displacement is distinguishable from drift while it is already IN drift, so first record its measured pinned and computed digests, bring its lock to this canon's current `60aead5c…` with a recorded reason citing this order (CC-VJS 12(d): a re-pin carries a reason a reader can answer), and confirm the two locks carry an identical digest, which is CC-VJS 14 C8's own check and is currently unmet. Then seed a displacement in a scratch CLONE, never the live repository, confirm the C5 Fatal fires and the C6 refusal holds, and remove it.

**C8 (the enumeration is recorded, not just the cure; CC-VJS 15 C7).** The compliance record lists every write site in the workspace that targets a governed-record root, with its destination and disposition, including the ones this order does not decide. CC-VJS 15 obiter (iv) is the reason: an enumeration offered in passing gets quoted as complete.

---

## 6. Obiter (not decided; no part of the ratio)

**(i) `vjs invoke` writes a config the kernel cannot read, and it takes the whole commit gate with it.** The template at `crates/vjs-cli/src/invoke.rs:45` omits `specs`, `decisions` and `cache`; `PathsConfig` requires all three with no `#[serde(default)]`. Measured on a fresh scratch jurisdiction:

    $ vjs validate --staged   -> Error: missing field `specs`   (exit 1)
    $ vjs local-ci            -> Error: missing field `specs`   (exit 1)

Both fail BEFORE any gate runs. So in every jurisdiction `vjs invoke` has ever created, the entire staged suite - permit, canon-write, apex bright line, bench/order, media, destructive-delete, install surface, and the PC-14 D3 assent floor - is unreachable, while plain `vjs validate` prints `OK`. The apex is unaffected only because its config was hand-written correctly and `invoke` declines to overwrite an existing one. `vibe-design-system`'s config has the defective shape. This is larger than the matter before me and I decide nothing about it, but it should be filed on its own record today, and whoever fixes it should know that the fix un-masks the displacement in this order (3.3), so C1 to C6 must land first or together.

**(ii) A pass over an empty set reads as a pass.** The invariant reporter prints "{n} invariants evaluated, all passed" and I measured it at n=0. Displacement is only one way to reach that state; a canon that fails to load its invariants for any reason reaches it too. `INVARIANTS_PASS` over zero invariants should not be an Info that reads as assurance.

**(iii) The apex's two order registers have no rule.** 37 records in `lawpack/v2/orders`, 22 in `.vjs/orders`, zero overlap; PC 1 to 19 are canon, PC 20 and PC 21 are not, and nothing in the tree states when a filed order is promoted. This order settles where a VERB writes. Promotion remains an unwritten convention, and an unwritten convention about which tree holds binding law is the kind of thing that becomes a defect quietly.

**(iv) The subscriber registry still does not name the live subscriber.** CC-VJS 14 obiter (iv) said so, and it is still true at HEAD: `lawpack/v2/federation/subscriber-registry.yaml` lists only a fixture code, while `vibe-design-system` (repo_code `VIBE-DESIGN-SYSTEM`, public remote at github.com/wlilley93, seven County orders, subscribing out of tree) is absent. Every protection keyed on that list still does not reach the one real subscriber. That is why C5 is keyed on the config and the resolution and not on the registry: a new gate keyed to that list would be born not reaching the only repository that needs it.

**(v) CC-VJS 14 C8's checkable condition is currently unmet.** It required the apex and subscriber locks to carry an identical digest. Measured: apex canon `60aead5c…`, subscriber lock `5481b9e2…`. No blame attaches - C8 sequenced one landing and two landed, and CC-VJS 15 moved the canon after the re-pin. The general point is the standing one: a fact recorded as checkable goes stale, and the obligation is to re-check it when the thing it pins moves, not when someone happens to look.

**(vi) The config declares a configurable orders path that the store ignores.** `[paths] orders` exists in `JurisdictionConfig`, and ACT-007:s1 says in terms that "all other record paths are configurable", but `Store::write_order` joins `.vjs/orders` literally. C1 above is deliberately phrased as "the destination `vjs order apply` already uses" rather than "the configured orders path", because deciding which of those two is right is not before me. But a config key the kernel writes and never reads is a declaration with no referent.

**(vii) `vjs status` prints a doubled version, `vjs-v2@0.1.0@0.1.0`.** Cosmetic, and it is the line an operator reads to find out which canon they are on.

**Blame:** none apportioned. The write target predates every ruling that made it dangerous: it was harmless while the only way to resolve a lawpack was to vendor one, and CC-VJS 12 created out-of-tree subscription without anyone noticing that it also created a directory a write could manufacture. The duty under ACT-001 is to make the work good, and the conditions are that remedy.

---

## 7. Stated plainly: what I measured that contradicts the case file

1. **The pleaded digests are stale.** `lock=sha256:5481b9e2 computed=sha256:de8b8802` are not this defect's numbers today. This canon hashes to `sha256:60aead5c…` (160 files, computed independently and cross-checked against the kernel); `5481b9e2…` is the pre-CC-VJS-15 digest, still pinned by the live subscriber. My reproduction gave `lock=60aead5c… computed=bd940d97…`.
2. **`vjs status` does not print `./lawpack/v2`.** It prints the absolute resolved path beside the canonical lawpack LABEL, which is a stronger form of the harm than pleaded and a direct reprise of CC-VJS 12 D6.
3. **The pleading is not reproducible on a config `vjs invoke` writes.** The record verb refuses with `missing field 'specs'` before it writes anything. The displacement is real; it is currently masked by an unrelated and undocumented defect, and the mask is due to be removed.
4. **The pin does not "detect it after the fact".** Following the drift Fatal's own suggested fix re-pins over the displaced directory, `validate` returns OK exit 0, `lookup` still returns only the fixture order, and the config's `lawpack_path` is left standing and false.
5. **The harm is much wider than lookup.** Displacement silences the canon-boundary gate entirely (three CANON_BOUNDARY_VIOLATION findings in the control, none in the displaced fixture) and evaluates zero invariants while reporting `all passed`. The reason is that `resolve_canon_repo_code` reads a manifest the manufactured directory does not have and falls back to the subscriber's own code.
6. **Option B's pleaded weakness is overstated.** The apex's canon and local order registers are disjoint, every County order it has made and its two newest Privy Council orders are in `.vjs/orders`, and no CLI path writes to `lawpack/v2/orders` at all.
7. **Option B's real weakness was not pleaded.** MCP `vjs.lookup` cannot see `.vjs/orders` (151 canon authorities, order absent), while CLI `lookup` returns it first. The hardcoded canon target was the only thing making the MCP's write and read verbs mutually legible, which is why C4 is not optional.
8. **ACT-007:s4 does not "forbid outright" writing into another repo's canon.** It governs the reach of local law. The nearer provision is s3's `must_not: local_law_override_canonical_without_authority`, and the decisive objections to Option A are the federation-wide digest blast and the fact that a write outside the repository is invisible to every gate this system has.

**Files:** opinion above. Fixtures under `/var/tmp/claude/cc16/` (`sub` displaced, `sub2` control with the order in `.vjs/orders`, `sub3` negative control vendoring a declared canon, `canon-host` the out-of-tree canon, `digest.py` the independent digest instrument). Nothing in `~/Projects/vibe-justice-system` was modified.