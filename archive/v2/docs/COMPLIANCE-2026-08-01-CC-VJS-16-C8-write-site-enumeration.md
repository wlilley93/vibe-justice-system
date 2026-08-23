# CC-VJS 16 C8: every write site in the workspace that targets a governed-record root

Compliance record required by `[2026] VJS-CC-VJS 16` C8. The condition is deliberately
wider than the cure: it demands the whole enumeration, "including the ones this order
does not decide", because CC-VJS 15 obiter (iv) holds that an enumeration offered in
passing gets quoted as complete.

Measured at commit `1d514a7` on 2026-08-01.

## 0. How this was measured, and one correction to my own method

Governed-record roots are the three declared at `crates/vjs-core/src/front_door.rs:86-94`:
`lawpack/v2`, `.vjs/orders`, `.vjs/court`.

The enumeration is every `fs::write`, `create_dir_all`, `File::create`, `OpenOptions`
and `Store::write_order` call under `crates/*/src/`, **excluding code at or after the
first `#[cfg(test)]` in each file**. My first pass excluded only files *named* `tests`,
which left in-file test modules in the set and produced two wrong dispositions. That is
the standing hazard that a bounded search is not a fact about the system, and it is
recorded here rather than quietly corrected. Production write sites: **51**.

## 1. The write sites that touch a governed-record root

| site | destination | disposition |
|---|---|---|
| `vjs-mcp/src/lib.rs:371,375` `handle_record` | `lawpack/v2/orders`, with `create_dir_all` | **The defect CC-VJS 16 cures.** The only production site in the workspace that writes into the canon tree, and the only one that can bring the resolver's directory into being. C1 redirects it to `Store::write_order`; C2 removes the `create_dir_all`. |
| `vjs-store/src/lib.rs:76-84` `write_order` | `.vjs/orders` (hardcoded) | **Compliant.** The destination CC-VJS 16 D1 requires. |
| `vjs-cli/src/lifecycle.rs:344` | calls `Store::write_order` | **Compliant.** `vjs order apply` already takes the door C1 sends the MCP verb to. |
| `vjs-store/src/lib.rs:200-214` `write_convening` | `.vjs/court/convenings` (hardcoded) | **Compliant.** Local court store. |

**No production site other than `vjs-mcp/src/lib.rs:371,375` writes a LITERAL path into a
governed-record root it does not own.** After C1 and C2 land,
`grep -n 'lawpack/v2' crates/vjs-mcp/src/lib.rs` must return nothing inside
`handle_record`, and no production source under `crates/` may call `create_dir_all` on a
`lawpack/v2` literal or a child.

> ### CORRECTION, 2026-08-01, after the cure was adversarially verified
>
> **The paragraph above was wrong as a statement of the CLASS, and it is left standing with
> this correction rather than quietly rewritten.** It enumerated LITERAL targets and then
> asserted a rule about all write paths. Two verbs take an operator-supplied output path and
> create its parent, so neither appears in any `lawpack/v2` grep:
>
> - `crates/vjs-cli/src/admin.rs` `cmd_conformance` (`vjs audit --out`)
> - `crates/vjs-cli/src/admin.rs` `cmd_migrate_v1` (`vjs migrate-v1 --out`)
>
> Measured on a fresh repository with no canon at all:
>
>     $ vjs audit --out <repo>/lawpack/v2/orders/probe.md
>     Conformance audit: 0 duties, 0 wired, 0 unwired -> .../lawpack/v2/orders/probe.md
>     $ ls -d <repo>/lawpack/v2
>     <repo>/lawpack/v2        # it did not exist before the command
>
> That is exactly what CC-VJS 16 D2 forbids, done by a REPORT WRITER, and this record
> asserted the rule held. The condition says in terms that it is "stated as a class and not
> as one caller, because the defect IS the class" - and a class needs a guard, not an
> enumeration. Cured by `vjs_engine::refuse_write_into_canon_tree`, applied at both sites,
> with a red seed and a negative control at
> `crates/vjs-cli/tests/lawpack_displacement.rs::no_operator_supplied_output_path_can_manufacture_the_canon_tree`.
>
> The general lesson, which is why this correction is verbose: a grep over literals cannot
> discharge a condition stated over a class, and I offered one that could. A bounded search
> is not a fact about the system.

## 2. The record writers that land outside the governed roots

All hardcode their destination; none is configurable; none can reach the canon tree.

| site | destination |
|---|---|
| `vjs-store/src/lib.rs:49-71` `write_log` | `.vjs/logs/decisions` |
| `vjs-store/src/lib.rs:89-97` `write_submission` | `.vjs/submissions/filed` |
| `vjs-store/src/lib.rs:125-133` `write_permit` | `.vjs/permits` |
| `vjs-store/src/lib.rs:161-169` `write_proof` | `.vjs/proofs` |
| `vjs-store/src/lib.rs:279-296` `write_lawpack_lock` | `.vjs/lawpack.lock` |

Kernel-surface writers (`vjs-core/src/install.rs`, `enforcement.rs`,
`vjs-git/src/lib.rs`, `vjs-cli/src/invoke.rs`) write `.vjs/install.lock`,
`.vjs/enforcement-surface.lock`, `.vjs/hooks/*`, `.vjs/config.toml`,
`.vjs/invocation/*`. These are surface, not records. Two defects found in `invoke.rs`
while doing this enumeration are filed separately at `SUBMISSION-2026-08-01-181122`.

Publication writers (`vjs-cli/src/gazette/render.rs:293,300,304,391,431`) write
`gazette-data.json`, `gazette-text.js`, `gazette.xml` and `gazette.html` at the repo
root. Artefacts, not records. `vjs-testkit/src/lib.rs:73-80` writes fixture repos under
paths its caller supplies.

## 3. Found while enumerating, NOT decided by CC-VJS 16

**(a) The default config declares two record roots inside the canon tree.**
CC-VJS 16 obiter (vi) recorded that `[paths] orders` is a config key the kernel writes
and never reads. It is not confined to `orders`:

    crates/vjs-store/src/lib.rs:487   specs:     "lawpack/v2/specs"
    crates/vjs-store/src/lib.rs:488   decisions: "lawpack/v2/decisions"

Both point **into the canon tree**. Nothing reads either key to resolve a write: the
four apparent readers (`vjs-lawpack/src/lib.rs:362,368`,
`vjs-lawpack/src/validator.rs:38,44`) iterate the *loaded lawpack's* `specs` and
`decisions` collections, which are unrelated to these path strings.

This is inert because the keys are dead, not because anyone decided they should not
point there - the distinction CC-VJS 16 drew when it called an accidental protection an
aggravation rather than a mitigation. The day something honours the configured path,
which is what ACT-007:s1 contemplates in saying "all other record paths are
configurable", two more record kinds acquire a default destination inside the canon
tree, and one of them (`decisions`) already has a live writer aimed elsewhere
(`.vjs/logs/decisions`). Same shape as the cured defect; not before CC-VJS 16, which
decided where a *verb* writes. It should be filed.

**(b) `vjs-cli/src/lifecycle.rs:210` defaults overlay floors to `lawpack/v2/overlay-floors`.**
Verified a **read**, not a write: the value goes to `OverlayLoader::load`. The site
already carries a CC-VJS 15 `LAWPACK-LITERAL` marker declaring it and recording that
"whether a subscriber's floors should come from the subscribed canon instead is a live
question and was not put to this court". Properly declared and properly open. No action.

## 4. What this record does not settle

- Whether `.vjs/orders` records are ever promoted into `lawpack/v2/orders`, and by what
  act. CC-VJS 16 obiter (iii): 26 local, 37 canon, zero overlap, no written rule.
- Whether the configured path keys should be honoured, making 3(a) urgent, or removed,
  making it moot. Both are live readings of ACT-007:s1.
- Section 2's dispositions describe the code as it stands, not guarantees about
  callers of the caller-supplied-path writers.
