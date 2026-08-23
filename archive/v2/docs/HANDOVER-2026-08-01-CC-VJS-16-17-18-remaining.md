# Remaining work on CC-VJS 16, 17 and 18, and the one thing blocked on the Principal

Written 2026-08-01 at commit `6e95a3c`. Records what is landed, what is not, and the exact
shape of the rest, so none of it has to be re-derived.

## Landed and pushed

| ruling | conditions | commit |
|---|---|---|
| CC-VJS 16 to 19 | recorded as orders, opinions, convenings, submissions | `1d514a7` |
| CC-VJS 16 | C8, the write-site enumeration | `e7152cb` |
| CC-VJS 16 | C1, C2, C4 | `6e95a3c` |
| CC-VJS 17 | C4 | `6e95a3c` |
| CC-VJS 19 | C1 to C10 (in `opbox-prod`) | `6800403` |
| CC-VJS 16 | C7, the destination re-pin (in `vibe-design-system`) | `02cd57d` |

276 workspace tests green, clippy clean under `-D warnings`, `vjs local-ci` PASS.

## THE ONE PRINCIPAL DEPENDENCY: six denylist classes

CC-VJS 17 **C7** requires every `.vjs/publication-denylist.txt` entry to carry
`# added=YYYY-MM-DD class=<client|infra|synthetic>`, machine-checked, never plaintext.

I attempted the classification rather than assuming it was blocked, by hashing candidate
terms I could reconstruct from the environment and my own records. Result:

- **line 14** - `class=synthetic`, `added=2026-06-23`. Not reconstructed; the register's
  own header at lines 6-13 documents it as the synthetic private-data sentinel.
- **lines 17, 19, 23, 24, 27, 28, 29** - `class=infra`, `added=2026-07-11`. Identified by
  reconstruction, seven of them.
- **lines 18, 20, 21, 22, 25, 26** - **NOT identified.** Six entries. The header records
  the 2026-07-11 block as "real confidential terms (client + infra)", so these are almost
  certainly the client half.

**These six need Will and nobody else.** I deliberately stopped rather than widening the
brute-force to real client identifiers: a wrong guess written in as `class=client` is a
false provenance record, and C7 exists precisely because a register nobody can audit has
been wrong before and the error was discoverable only by someone who happened to know.

Everything else in CC-VJS 17 is unblocked and C7 is the ONLY item waiting on this.

## What is left, in landing order

Three implementation specs were worked up in full and are the authority for the rest.
Their load-bearing findings, which are not obvious from the opinions:

### Constraints that bind every remaining edit

1. **The 600-line structural ceiling** (`crates/vjs-testkit/tests/structural_ceiling.rs`)
   counts test files too. At HEAD: `vjs-engine/src/lib.rs` 591, `vjs-redact/src/lib.rs` 600,
   `vjs-cli/src/gazette/mod.rs` 600, `vjs-engine/src/staged.rs` 600. The last three have
   ZERO headroom. CC-VJS 16 C3/C5 therefore require extracting the resolver out of
   `vjs-engine/src/lib.rs` into `resolver.rs` first; it is not optional polish.
2. **`compile_time_and_drift.rs` bans `env::var(` anywhere in `crates/vjs-engine/src`.**
   The existing resolver survives on `var_os`. Any new env read must use `var_os`.
3. **CI runs `cargo clippy --workspace --all-targets -- -D warnings`.**

### CC-VJS 16, remaining: C3, C5, C6

- **C3**: `resolve_lawpack`'s vendored candidate is accepted only if it declares itself a
  lawpack (a readable `manifest.toml`) WHERE another source is recorded. An empty
  `VJS_LAWPACK` must be filtered, or the branch changes behaviour in every repository whose
  shell carries it.
- **C5**: a Fatal `LAWPACK_DISPLACED`, distinct from `LAWPACK_LOCK_DRIFT`, naming the
  recorded subscription AND the directory that answered, whose suggested fix is to remove
  the directory and NEVER to re-pin.
- **C6**: `vjs invoke` with no `--lawpack` refuses over a contradicting source, and the lock
  is byte-identical before and after.
- After C3, C5 is unreachable from an UNDECLARED directory, so the C5/C6 fixtures must write
  a `manifest.toml` or they assert absence in a state where the finding cannot be produced.

### CC-VJS 17, remaining: C1, C2, C3, C5, C7

**C7 will silently blank the register unless C1 and C3 land with it.** Every current reader
treats the WHOLE LINE as the hash, so appending a provenance comment turns fourteen hashes
into fourteen strings that match nothing and every gate keyed on the register goes green
while checking nothing. Four readers must change together: `gazette/render.rs`,
`publication_boundary.rs`, `scripts/boundary-scan.sh`, `scripts/promote-canonical.sh`.

- **C1**: a denylist limb on the canon-write gate, its own finding kind and code, naming
  file and 1-indexed line and NEVER the term. Record 3 of the three admitted carries two
  occurrences, so it is one finding per hit LINE, not per record.
- **C2**: two deletions, not one. Removing the `.md` skip does NOT make signal 4 reach
  markdown: `scan_canon_record` returns early when the body is not a YAML mapping, and
  signal 4 sits below that. The prose limb must move ABOVE the parse.
- **C3**: four fail-open register reads become errors naming the path.
- **C5**: every `source_opinion` body reachable from a published item is scanned. All 25
  targets exist, are tracked, and carry no hit, so this will not refuse today's canon.

### CC-VJS 18: DONE, C7 included (2026-08-02, commit e0ed738)

C7 landed. The lock is TOML with a required per-entry `authority`, `--authority` is required
by clap and empty is refused, and the authority is stamped ONLY on entries whose digest
actually moved, so an unmoved entry keeps the authority it already carried rather than
acquiring a false provenance record.

The predicted silent disarm was real and is closed: `read_lock` now returns three states
rather than an `Option`, so a lock that EXISTS and cannot be parsed is Fatal
`ENFORCEMENT_LOCK_UNREADABLE` instead of reading identically to an un-pinned repository. The
pre-C7 flat format is deliberately one of the unreadable cases, because a subscriber whose
binary is upgraded before its lock is re-pinned lands in exactly that state and it must be
loud. Proved at the governed boundary through the real `vjs_engine::validate`, and seeded:
mapping every parse failure back to "no lock" reds it, with the isolation step recorded
(re-pin the seeded tree first, because `enforcement.rs` is itself entrenched so ANY edit to
it also reds the pin test, and two red tests where one is collateral looks exactly like two
red tests where both are real).

Workspace: 295 passed, 0 failed. `enforcement.rs` at 563 of the 600-line ceiling.

**Propagation, measured 2026-08-02 rather than assumed.** A lock FORMAT change is the shape
that breaks every stale subscriber, so it was checked: `vibe-justice-system` is the ONLY repo
on this box holding an `enforcement-surface.lock`, and it is post-C7 TOML with all twelve
entries carrying an authority. No subscriber is exposed.

### NOT OWED: "re-vendor CC-VJS 15+ into opbox-kernel"

This appeared on the earlier task list and is a phantom, recorded here so nobody spends an
afternoon on it. Measured 2026-08-02: `opbox-kernel/lawpack/v2/orders` is BYTE-IDENTICAL to
the publisher's, and **no county-court order is in the lawpack at all** - all 25 CC- orders
live in `.vjs/orders/` and the published canon holds only BOOT, COURTS, PC and SC instruments
(37 files, same count both sides). CC-VJS orders bind THIS repository; they were never canon
for federation, so there is nothing about them to vendor. The kernel's vendor pointer reads
older (`52a0817`, carrying CC-VJS 14) than the publisher's HEAD, which is what made this look
outstanding, but the vendored CONTENT is current because the intervening rulings never
entered the lawpack.

## Findings filed while doing this work, all awaiting the court

| submission | question |
|---|---|
| `SUBMISSION-2026-08-01-181055` | an unparseable local order is dropped with a warning and every door passes |
| `SUBMISSION-2026-08-01-181122` | `vjs invoke` writes two artefacts other doors refuse |
| `SUBMISSION-2026-08-01-181542` | the default config declares two record roots inside the canon tree |
| `SUBMISSION-2026-08-01-184058` | is a reasoned opinion a publishable class of canon record |
| `SUBMISSION-2026-08-01-184104` | does the digest alone record the tree that was read |

## Two Principal questions from CC-VJS 19, still undecided

Filed at part 4 of the CC-VJS 19 opinion and NOT answered: whether The Corporate Brain is
still the objective and the apex record a faithful restatement of it; and whether any
wording a transcriber would have changed should be changed. A court may decide where an
objective is recorded and which record is citable; it may not decide what the objective is.
