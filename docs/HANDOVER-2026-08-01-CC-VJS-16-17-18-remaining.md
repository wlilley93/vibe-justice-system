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

### CC-VJS 18, all of it

The sequencing is settled and matters: **the re-pin is the LAST mutation.** Entrenching
`vjs-engine/src/lib.rs` before the CC-VJS 16 and 17 edits would make every subsequent commit
fail until re-pinned, which manufactures the cadence problem the ruling assumed away.

- C7 changes `.vjs/enforcement-surface.lock` from flat text to TOML and adds a required
  per-entry `authority`. `check_drift` MUST be updated in the same change or all twelve
  entries report spurious Fatals.
- C7's stricter parse creates a NEW way to fail, and the current code reports a parse
  failure as "no lock, no finding". An `ENFORCEMENT_LOCK_UNREADABLE` Fatal is needed or C7
  introduces a silent disarm on the day it entrenches the file.
- C4's positive control does not exist anywhere in the suite: nothing in the workspace
  proves the assent floor's downgrade ever happens. That is wider than the opinion states.

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
