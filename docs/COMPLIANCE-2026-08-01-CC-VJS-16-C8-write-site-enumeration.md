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

**No production site other than `vjs-mcp/src/lib.rs:371,375` writes into a
governed-record root it does not own.** After C1 and C2 land,
`grep -n 'lawpack/v2' crates/vjs-mcp/src/lib.rs` must return nothing inside
`handle_record`, and no production source under `crates/` may call `create_dir_all` on
`lawpack/v2` or a child.

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
