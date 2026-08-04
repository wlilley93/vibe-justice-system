# DRAFT CASE FILE: Order 25 and the single-body frame

**status:** DRAFT, not yet filed. No bench convened.
**subject repo:** opbox-frontend, worktree `/var/tmp/claude/parity-wt`, branch `parity/at-parity-programme`
**law in question:** [2026] VJS-SC-OPBOX 1, order 25
**implementation under scrutiny:** `~/Projects/vibe-design-system/crates/vds-cli/src/signoff.rs:86-116`
**prepared by:** Lexby, 2026-08-03
**tier sought:** First Instance

---

## 1. The question

Order 25 makes a frame's **authority label** decide its registrability: only a frame
resolving its authority from a `CURRENT SOURCE` (or resolvable `SOURCE AUTHORITY`) layer
may be registered, and therefore signed.

The question for the court is narrow:

> Where a frame resolves its authority from its own children, and the capture shows it
> has exactly one candidate body with nothing quarantined beside it, does order 25 refuse
> registration?

Nothing here asks the court to overrule order 25. The submission is that the order's
ratio may not reach the single-body case, which is a question of scope, not of merit.

---

## 2. Why this is live, in numbers

Measured 2026-08-03 against `.vds/ledgers/frames.yaml` (167 frames, capture depth 6,
content digest `sha256:fbc5a5e9…d5a5`) and `.vds/signoffs/` (19 records).

The 167 registered signing frames partition exactly:

| authority source | frames | registrable today |
|---|---:|---|
| `named_layer` (CURRENT SOURCE / SOURCE AUTHORITY) | 19 | yes |
| `unlabelled` | 26 | no |
| `frame_own_children` | 122 | no |

Cross-tabulated against whether anything is quarantined beside the governing body:

| bucket | quarantine list empty | quarantine list non-empty |
|---|---:|---:|
| `named_layer` (admitted) | 3 | **16** |
| `frame_own_children` (refused) | **121** | 1 |
| `unlabelled` (refused) | 25 | 1 |

**121 of the 122 refused `frame_own_children` frames have nothing quarantined beside the
body. 16 of the 19 admitted frames do.**

---

## 3. The case FOR the distinction

**3.1 The mischief order 25 addresses is undecidability, and it is absent here.**
The order was forged against a specific, real defect, recorded at
`internal-docs/design/FRAME-REGISTRY.md:56`: `/matters` carries a `LEGACY UNDERLAY` body
of 112 children beside a `CURRENT SOURCE CONTRACT` body of 113. A registration that does
not say which of the two it signed "has signed nothing decidable". That reasoning is
sound and is not challenged. But it is reasoning about a frame with **two** candidate
bodies. Where the capture shows one body and an empty quarantine list, there is nothing
to disambiguate, and the label requirement is discharging no function.

**3.2 On live data the rule is close to inverted.** It admits 16 frames that do carry
quarantined siblings (the ambiguous population the order was written about) and refuses
121 that carry none. A rule whose effect on the estate is the reverse of its stated
purpose invites the court to ask whether its scope was ever settled.

**3.3 The consequence is disproportionate and it is not merely cost.** The remedy for a
refused frame is a Figma redraw adding a marker layer. Figma writes on this estate are
strictly serial under an advisory lock at roughly 20 minutes per frame, so 121 frames is
of the order of 40 hours of serial work. Two prior concurrent-writer incidents destroyed
a family of work, so the serialisation is not negotiable. The programme's AT_PARITY
figure is 0 and cannot move for those 121 routes until this resolves.

**3.4 The registry already records a reading to this effect, though see 4.3.**
`frame-registry.json` `_signatureDirection.effect[0]` states: "Every registered frame is
signed. The label that was pending is resolved BY THE SIGNING ACT, which is what order 25
always said would resolve it."

---

## 4. The case AGAINST the distinction

**4.1 A bright-line label rule is administrable; a content test is not.** Order 25's
present form asks one question of one field. The proposed distinction asks a reader to
determine that a frame has exactly one candidate body, which depends on capture depth. On
this estate 11,207 leaves are depth-truncated at depth 6. A frame that appears
single-bodied at depth 6 may not be at depth 8, and the register's own convention
(`capture.depth` mandatory) exists precisely because "the frame draws nothing here" and
"we did not look" must never be the same value. A distinction resting on absence rests on
exactly that conflation.

**4.2 An empty quarantine list is not proof of a single body.** It records what the
quarantine pass identified, not what exists. No instrument on this estate has been shown
to enumerate candidate bodies directly, and the 121 figure inherits whatever that pass
missed. The 18 non-empty rows reconcile across buckets (1 + 16 + 1), which is consistent
with the pass working, but consistency is not coverage.

**4.3 The registry's supporting gloss may not be authority.** The `effect` block at 3.4
is an interpretation. The Principal's own words recorded in the same record are: "Everything
in figma is signed with variation, meaning, signed off, but where the site has more
features than the figma suggests, default to finding a nice way to include those additional
pieces on the page." Those words plainly support the "with variation" qualification and
the treatment of shipped excess. They do **not** plainly say that a general signing act
discharges the order 25 label precondition. Order 16 holds that a machine verdict creates
no authority, so if `effect[0]` was authored by an agent rather than by the Principal, it
cannot carry the argument. **The court should establish the provenance of that block
before relying on it, and the submission does not rest on it.**

**4.4 The cost argument cuts both ways.** That compliance is expensive is not a reason to
read a safeguard down. The estate's own history is against it: three separate progress
numbers were reported over this programme and not one measured a page against a frame,
and 154 unearned conforms were withdrawn on 2026-08-03. A weakened precondition on this
estate has a poor record.

---

## 5. What the court is NOT asked to do

- Not asked to overrule order 25 or to disturb the `/matters` outcome, which is correct
  on any reading.
- Not asked to widen the accepted marker vocabulary in `.vds/config.toml`. That route was
  identified and expressly declined: adding `frame_own_children` to the accepted set would
  admit the two-body case as well as the one-body case, which is the precise defect the
  order was forged against.
- Not asked to rule on the 26 `unlabelled` frames, whose posture differs and which are
  reserved.

---

## 6. Relief sought, in the alternative

1. **Primary:** a declaration that order 25 does not refuse a frame resolving authority
   from its own children where the capture establishes a single candidate body, with a
   direction as to the evidential standard for "single body" (in particular, the capture
   depth at which the finding must be made).
2. **Alternative:** a direction as to what would establish a single body to the court's
   satisfaction, so the instrument can be built and the question returned on evidence.
3. **If refused:** a direction on sequencing the 121 redraws, given the serial Figma
   constraint and the 2026-08-30 and 2026-09-15 fences.

---

## 7. Evidence relied on

| item | path |
|---|---|
| frames ledger, 167 frames at depth 6 | `/var/tmp/claude/parity-wt/.vds/ledgers/frames.yaml` |
| frame audit, measured hashes | `internal-docs/design/evidence/figma-frame-audit-vds-0803.json` |
| preserved capture archive | `internal-docs/design/evidence/figma-capture-vds-0803.tar.zst` |
| Principal signature act | `internal-docs/design/evidence/principal-frame-signature-2026-08-03.json` |
| 19 recorded sign-offs | `/var/tmp/claude/parity-wt/.vds/signoffs/SGN-*.yaml` |
| the `/matters` two-body record | `internal-docs/design/FRAME-REGISTRY.md:56` |
| the refusal as implemented | `~/Projects/vibe-design-system/crates/vds-cli/src/signoff.rs:86-116` |

All hash checks in this file were run on 2026-08-03 against the preserved capture: 167 of
167 measured hashes reproduce against the ledger digests, zero mismatches.

## 8. What was not checked

- I did not run `cargo`. Every claim about `signoff.rs` is read from source, not observed.
- The single-body finding is derived from the ledger's `quarantined` key at capture depth
  6. I did not independently enumerate candidate bodies per frame, which is the weakness
  §4.1 and §4.2 identify and which the court may wish cured before ruling.
- The provenance of `_signatureDirection.effect[0]` is unestablished (§4.3).
