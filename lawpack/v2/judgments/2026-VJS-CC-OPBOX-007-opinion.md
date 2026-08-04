# IN THE COUNTY COURT OF THE VIBE JUSTICE SYSTEM
## Jurisdiction: <subscriber>
### SUBMISSION-2026-08-03-212454
### Before: **Aldermarsh CCJ**, sitting alone ([2026] VJS-SC 2)
### Delivered: 2026-08-03

---

## PRELIMINARY: WHAT I HAVE READ AND WHAT I HAVE MEASURED

I have read the submission, the five opinions in MATERIAL.md, and [2026] VJS-CC-OPBOX 6 in full at `~/Projects/vibe-justice-system/.vjs/orders/2026-VJS-CC-OPBOX-SINGLE-BODY-AUTHORITY-006.yaml`.

The five opinions bind nothing. I have adopted some of their reasoning where my own measurements support it, and I reject two of their central holdings outright, one of which is factually wrong and would, if acted on, have set aside the whole register on the strength of a mis-chosen reader.

Everything below marked as a finding is my own measurement, taken tonight against the named paths, with the command stated. Where the pleading and my measurement differ, my measurement governs.

---

## DISPOSITION

**GRANT_IN_PART.**

Granted: the freeze (in an enforceable form the applicant did not ask for), the retirement of `CURRENT CODE` from **this jurisdiction's own record only**, and the audit (narrowed, and partly performed on the face of this judgment).

Refused: restoration of the demoted layers; re-signature sequenced so that authority never dips; the prospective clean-hands condition as framed; re-derivation with the corrected puller; and the confirmation sought for the 122.

---

## RATIO

**R1. A digest that moves when the capture is re-serialised is not bound to the content it names.**
Where an estate's authority holds only while a frame's current content digest equals its signed digest, the *recipe that produced the digested input* is part of the record and not an implementation detail. A register whose inputs cannot be reproduced from the preserved evidence is authority the estate cannot re-establish, and a bare regeneration of the ledger is a vacatur of every signature bound to it. It follows that (a) the reproduction recipe must be recorded with the ledger and demonstrated by reproducing the ledger's own header digest, and (b) the ledger must not be regenerated except in an operation that also re-signs the affected rows.

**R2. Relief goes to the element that does the work, and a measured-inert cause is not reversed by court order.**
Where the counterfactual shows that undoing the impugned act leaves the register node-for-node unchanged, and that a different, unpleaded act is what converted the frames from refusable to registrable, the pleaded relief fails. That is so *a fortiori* where the reversal would move every affected content digest and so destroy the very signatures it is sought to protect. A court does not order the undoing of a cause it has measured to be inert.

**R3. A prohibition whose predicate no instrument in the estate can compute is not ordered.**
Where a pleaded condition precedent turns on *who authored a layer name*, and no artefact in the estate records authorship, the condition is refused expressly on that ground and not left to be reported satisfied on every run. A computable duty may be substituted; an uncomputable one may not be dressed as a control.

**R4. A measurement of a record is a measurement of a reader.**
Where a register's readability depends on which build of the instrument is on the operator's PATH, the proposition "this estate holds N registrations" is not a fact until the instrument and its commit are named. A refusal, a parse failure or a NOT FOUND is a claim about the reader and never a fact about the record. This is the same rule as "a bounded search is not a fact about the system", applied to instruments rather than to searches, and it is now the law of this jurisdiction.

**R5. A hash-bound attestation over a ledger is not a label-resolution act on a locus.**
Signing an aggregate digest of a record which itself classifies 122 loci as carrying no declaration does not supply the declaration. Cure (b) under VJS-CC-OPBOX 6 requires an act that is express, verified, hash-bound, Principal, **and** label-resolving on a named locus. The conjunction is the whole point of it.

I carry **no ratio** on: whether an agent-authored marker layer can ever constitute the Principal's Order 25 declaration; the status of `SOURCE AUTHORITY` as a configured marker; and the sufficiency of depth-limited frame hashes. Those are addressed under RESERVATIONS.

---

## FINDINGS OF FACT

**F1 — The ledger.** 167 rows: 122 `frame_own_children`, 19 `named_layer`, 26 `unlabelled`. Header `content_digest: sha256:fbc5a5e978a35594…`, `capture_depth: 6`, `truncated_leaves: 11207`.
`grep -c '^- node_id:' /var/tmp/claude/parity-wt/.vds/ledgers/frames.yaml`; `grep '^  authority_by:' … | sort | uniq -c`; `head -9 …`.

**F2 — The register.** 19 sign-off files. Their `nodeId` set is **exactly** the 19 `named_layer` node ids. Every `frameDigest` equals that frame's ledger `content_digest`. All carry `signedAt: 2026-08-03T16:49:22Z` and cite one evidence file at `sha256:bd267c4ded5cc531…`, which I verified with `sha256sum internal-docs/design/evidence/principal-frame-signature-2026-08-03.json`. (PyYAML parse of `.vds/signoffs/SGN-*.yaml` cross-referenced against the ledger.)

**F3 — The LEGACY UNDERLAY layers.** Eleven frames carry `LEGACY UNDERLAY` entries in `quarantined`, four each, 44 in total. All eleven are `named_layer`. **No layer is named "LEGACY UNDERLAY".** The names are `LEGACY UNDERLAY · rail | · cmdbar | · body | · statusbar`; `LEGACY UNDERLAY` is a *quarantine marker* matched against the **leading segment only** (`crates/vds-figma/src/frames.rs:311-319`). `disclaimed` is **false for all 167 rows**, and no frame is governed by a LEGACY UNDERLAY layer.

**F4 — The pleaded counts are wrong in three respects.**
(a) Ten authority layers contain `cloned from`, not six. (b) `CURRENT CODE` occurs **zero** times in the ledger: `grep -ic 'CURRENT CODE' .vds/ledgers/frames.yaml` → `0`. The nine nodes carrying that name sit at depths 2 to 4, where `authority_root` cannot see them. (c) 6 + 4 = 10, which does not reach the pleaded eleven, and the eleventh is never identified.

**F5 — "Uncommitted" is wrong, and the truncation ground is causally inert.** The `pull.rs` change is committed as `8b5813c` and pushed to `origin/feat/proof-kinds-visual-gap`. It is unmerged to master, which is a different fact. More importantly, `vds figma frames` has **no network transport at all**: its own `--help` states "There is deliberately no API transport here", and `crates/vds-cli/src/figma.rs:497` calls `frames::from_saved`, which reads local files. The puller could not have touched this ledger.

**F6 — I REPRODUCED THE LIVE LEDGER EXACTLY, AND THE RECIPE IS THE FINDING.**
Deriving from the preserved capture as it sits on disk gives **0 of 167** matching per-frame digests. Deriving over all 205 captured roots gives **0 of 167**. But extracting the 167 registered node ids from that same capture, writing them through a JSON round-trip into one file, and running

```
vds figma frames --root <scratch> --file-key iq2MHVcTcWPSvKOiG9MCQW \\
    --captured-at 2026-08-03T14:35:33Z --from subset.json
```

produces `truncated_leaves: 11207`, **167 of 167** identical per-frame digests, and header `content_digest: sha256:fbc5a5e978a35594e64e53aaa8467aed1d8777d5f0b8ed832ee6597e3650d5a5` — **the exact digest all nineteen sign-offs bind**. `diff` of the two ledger bodies, ignoring `generated_at`, differs on **one line**: the provenance note.

Two consequences, both load-bearing.

*First*, the per-frame `content_digest` is **not a function of the frame's design content**. It is a function of the byte-form of whatever JSON writer last wrote the capture file. Identical parsed content, written by two different serialisers, yields two different "content" digests. The live ledger's own note records its inputs as `/tmp/opbox-vds-regenerate.xFPMY5/registered/nodes-1.json` — a filtered, re-serialised copy in tmpfs, which no longer exists and which nobody preserved.

*Second*, both open questions in `~/Backups/opbox-frame-evidence-2026-08-03/A4-A5-RE-DERIVATION-RECORD.md` are now closed. The row-set discrepancy (205 against 167) is explained: the ledger is filtered to the registered set. And "every one of the nineteen digests moved" is an **artefact of method, not a fact about the evidence**. The evidence does reproduce the ledger. Nobody should act on that alarm.

**F7 — The 25 self-disclaiming frames are NOT being carried as scoreable.** Re-deriving over all 205 captured roots reports `disclaimed: 25`. **Every one of the 25 lies in the 38 captured roots the live ledger does not carry. Zero are inside the live 167.** Seven of the 25 are exactly the seven the Principal's signature instrument excluded by node id (`675:6752, 675:5537, 675:7196, 674:6879, 696:2218, 669:152813, 675:4959`). The live ledger's `disclaimed: 0` is correct for its own population, and `FRAME-REGISTRY.md`'s figure of 25 is a statement about the wider capture. There is no contradiction and no defect here.

**F8 — `CURRENT CODE` is inert, and more inert than pleaded.** Re-deriving the live 167 with an explicit `[screens]` table omitting `CURRENT CODE` reproduces the live ledger's header `content_digest: sha256:fbc5a5e9…` **exactly**, with the `named_layer` set identical at 19 and node-identical. Retirement does not move the digest the nineteen signatures bind. The blast radius is measured at zero **against the live artefact**, not merely against a re-derivation.

**F9 — THE CAUSAL MECHANISM IS THE IMPLANT, NOT THE DEMOTION.** I ran both counterfactuals over the reproduced ledger.
*A:* strip `LEGACY UNDERLAY · ` from all 44 layers and set them visible, keeping the implanted layer. Result: `named_layer` **unchanged at 19, node-identical**. The register does not move. But the ledger digest moves to `sha256:b2c6548a…`, so **restoration would vacate all nineteen signatures while changing no registration**.
*B:* do that *and* remove the implanted marker layers (12 of them across the 11 frames). Result: all eleven fall to `frame_own_children` and `named_layer` drops to 8.
So the demotion is **neither necessary nor sufficient**. The implanted marker layer is what converts a refusable frame into a registrable one. The submission has mis-stated the mechanism of its own principal complaint.

**F10 — The register's readability is build-dependent.** `~/.local/bin/vds` (installed 2026-08-03 18:09) lists all 19 sign-offs as `CURRENT` and exits 0. `vds` built from the vibe-design-system working tree HEAD (`8b5813c`) **refuses**: "unknown field `evidence`", exit 2. The `evidence` block is a **designed schema extension** on branch `staged-write-s7e` (`6577206 add evidence-backed external signoff import`, `19d13b0 Keep Order 25 signoff fail closed`), pushed to origin and **not merged**. The 19 SGN files were written at 18:09:29, the same second that binary was installed.

I therefore reject outright the proposition in the material that this estate holds zero registrations. It is an artefact of running a build from a branch that does not carry the feature. The register is well-formed under the schema that wrote it. What is real, and orderable, is that the schema lives on an unmerged branch, so anyone building `vds` from the main line makes the whole register unreadable.

**F11 — One registration rests on a phrase, not a declaration.** `669:135785`'s authority layer is `/dashboards · current source matter master-detail`. Authority markers match **anywhere and case-insensitively** (`frames.rs:310-326`), a choice the code comment at `frames.rs:305-309` records as deliberate.

**F12 — `675:25422` carries two competing recognised current layers**: `CURRENT SOURCE CONTRACT · /matters · selected-board Board view` and `… · all boards Board view`. VJS-CC-OPBOX 6 expressly reserved that question.

**F13 — The Principal's act.** `actualReply` is four words: `"Signed off. proceed all"`. The `incorporatedStatement` is a drafted paragraph signing "the 167-frame VDS ledger … aggregate digest sha256:fbc5a5e9…". The instrument's own `authorityBoundary` states it "grants no VDS warrant and asserts no route conform verdict", and `warrant` is `false`. Its `aggregateDigest` is the ledger digest I reproduced at F6.

**F14 — The evidence archive is durable and verifies.** `sha256sum -c MANIFEST.sha256` in `~/Backups/opbox-frame-evidence-2026-08-03` passes on all 42 rows. `verify-freeze.py --capture ./capture` reports "freeze intact: 44 layers across 11 frames, 2376 nodes, all unchanged", exit 0. Its `capture/` is byte-identical to `/var/tmp/claude/cap/ex`.

**F15 — Neither the retirement nor the freeze is presently in force.** `parity-wt` commit `ac404930` ("Retire CURRENT CODE in <subscriber>'s own record, and freeze the 44 demoted layers", 22:17) was reverted by `39702e04` (22:20) with no stated reason, removing `.vds/config.toml`'s `[screens]` table and `internal-docs/design/FREEZE-2026-08-03-legacy-underlay.md`.

**F16 — The enacted words.** Order 16 reads, in full: *"Machine visual_review classifies against the registered record; it never adjudicates taste and its verdicts create no authority."* Order 25 reads: *"Only frames labelled CURRENT SOURCE are registrable, the signing act resolving any contrary label first…"* Both at `/var/tmp/claude/parity-wt/.vjs/court/2026-VJS-SC-OPBOX-1-design-constitution.md`, lines 433 and 451. The applicant's second sentence, "An agent cannot manufacture authority", is his gloss and is not in the order. The prohibition he needs does exist, but it is `auto_register_or_manufacture_a_principal_signature` in VJS-CC-OPBOX 6's `forbidden` list, and that is the provision I apply.

**F17 — Jurisdiction.** ACT-007:s4, "Local law does not bind other repos": *"Local orders, decisions, and conventions apply only to the local repo. They do not bind other repos unless adopted by those repos"*, `must_not: local_order_bind_other_repos`. Verified at `~/Projects/vibe-justice-system/lawpack/v2/statutes/07-federation.yaml:52-60`.

---

## ORDERS

**1. FREEZE — GRANTED, in the form a court can enforce.**
Lexby and every agent acting in this jurisdiction are prohibited from deleting, renaming, re-parenting, restyling, or altering the visibility of any of the 44 layers named `LEGACY UNDERLAY · rail|cmdbar|body|statusbar` on the eleven frames `674:26005, 674:26476, 674:26867, 674:27264, 675:25422, 675:25720, 675:72640, 675:73272, 675:73718, 675:73974, 675:74319`, or any node beneath them, absent an express Principal instruction naming the individual frame.
This binds **us**, not the Principal, and not the Figma file: a county court will not restrain the Principal from drawing on his own canvas, and an injunction against a remote third-party-hosted file this court cannot observe would be unenforceable on its face.
Re-create `internal-docs/design/FREEZE-2026-08-03-legacy-underlay.md` in `parity-wt`, citing **this** judgment and not the vacated one, and wire `~/Backups/opbox-frame-evidence-2026-08-03/verify-freeze.py` as a check that runs against the current capture before any Figma write. It passes today (F14).

**2. EVIDENCE — the archive stands, and the manifest is to be committed.**
`~/Backups/opbox-frame-evidence-2026-08-03` is adopted as the evidential record of this matter. Commit `MANIFEST.sha256`, `frozen-layers.tsv`, `verify-freeze.py` and `A4-A5-RE-DERIVATION-RECORD.md` into the opbox-frontend repository on branch `parity/at-parity-programme`. `/var/tmp` and `/tmp` are not durable, and the ledger's own inputs were lost to exactly that (F6).

**3. RESTORATION — REFUSED.**
No LEGACY UNDERLAY layer is to be set visible or renamed as a remedy in this matter. Reasons, each independently sufficient: (a) restoration changes no registration at all (F9, counterfactual A); (b) it would move every affected content digest and vacate all nineteen signatures (F9); (c) there is no pre-demotion snapshot anywhere on disk, so "restore" has no measured target, only an inference from a naming convention; and (d) the Principal has said the implanted sidebars are not objectionable, and a court that un-hides forty-four layers puts two rails on his frames against his expressed preference.

**4. RE-SIGNATURE SEQUENCED SO AUTHORITY NEVER DIPS — REFUSED.**
No instrument, script or workflow may be created whose effect is that a frame's authority does not revert to UNSIGNED when its content digest changes. Staleness by hash is the enacted mechanism (`crates/vds-core/src/types/signoff.rs:49-52`: "authority holds while the current digest equals this one, and not a moment longer"), and SC-OPBOX 1 order 7 makes the resulting `no_authority` state lawful. An operation timed so that the dip never appears has pre-committed the Principal's act, which VJS-CC-OPBOX 6 forbids in terms. Any re-signature must be a separate act, later in time, on a fresh digest.

**5. THE LEDGER AND THE SIGNATURES MOVE TOGETHER, AND THE RECIPE IS RECORDED.**
(a) `.vds/ledgers/frames.yaml` must not be regenerated except in the same operation that re-signs every row whose digest moves.
(b) Lexby shall write, and commit alongside the ledger, a **reproduction recipe** stating: the 167 registered node ids; the source capture and its per-file sha256; the normalisation step applied to those bytes before derivation; and the `--captured-at` value. The recipe is discharged only by a script that runs end to end from the preserved capture and reproduces `content_digest: sha256:fbc5a5e978a35594…`. I have done it once tonight; it must be reproducible by someone who was not in the room.
(c) Until (b) exists, no party may assert that the ledger is re-derivable from the filed evidence, and no naive re-derivation is to be run against the live tree.

**6. `CURRENT CODE` — RETIRED FROM THIS JURISDICTION'S OWN RECORD ONLY.**
Re-apply the config half of the reverted `ac404930`: add an explicit `[screens]` table to `/var/tmp/claude/parity-wt/.vds/config.toml` declaring `authority_markers = ["CURRENT SOURCE", "SOURCE AUTHORITY"]`, together with `quarantine_markers`, `name_separator` and `region_names` at their present inherited values. Verify by re-deriving under the Order 5(b) recipe and confirming the header digest is still `sha256:fbc5a5e9…` and `named_layer` is still the same 19 node ids. It is (F8).
Two independent grounds. The measured blast radius is zero, so this disturbs no registration. And an estate whose governing vocabulary appears in no record of its own is governed by silence: <subscriber> presently inherits its registrability vocabulary from another repository's compiled Rust default, which is a defect regardless of which words are in the list.
This is a **narrowing**, so the VJS-CC-OPBOX 6 prohibition on widening the vocabulary with synonyms is not engaged, and the status of other configured markers was expressly reserved and remains open.

**7. NO ORDER AGAINST vibe-design-system.**
The application, so far as it seeks retirement of `CURRENT CODE` from `crates/vds-core/src/config.rs:274` and `:589` or from the example configs, is **REFUSED FOR WANT OF JURISDICTION** under ACT-007:s4 (F17). I sit in <subscriber>. Nothing in this judgment is authority for editing any file under `~/Projects/vibe-design-system`. Lexby has liberty to file in the VDS lane, and should, attaching the F8 measurement.

**8. THE CLEAN-HANDS CONDITION — REFUSED AS FRAMED, AND I SAY EXPRESSLY WHY.**
The pleaded condition turns on whether the Principal applied a marker. **No instrument in this estate can compute that predicate.** `FrameRow` (`frames.rs:127-167`) carries no provenance field; `authority_by` records only how a locus was selected; nothing in VDS parses the `cloned from NNN:NNN` convention; and the capture carries `lastModified` at file level only. A condition precedent whose predicate cannot be computed would be reported satisfied on every run, which is a check that cannot fail. I decline to order it on that ground, as I am required to state.
**SUBSTITUTED**, binding Lexby in this jurisdiction and computable today: no sign-off row shall be created in `parity-wt/.vds/signoffs/` for a frame whose ledger row carries a non-empty `quarantined` list, or which has a hidden direct child, unless the sign-off record itself enumerates each such layer by name with its recursive node and text counts. The signer may sign a displaced frame; he may not sign one without the displacement appearing on the face of what he signs. That is precisely what failed on 2026-08-03 (F13: the instrument discloses 167 rows, 156/11/15, seven excluded artefacts and a parity-ineligible root, and says nothing about fifty-six displaced layers).
Whether the `vds signoff record` door should itself enforce this is a VDS matter and is referred, not ordered.

**9. AUDIT — GRANTED, NARROWED, AND PART-PERFORMED HERE.**
The nineteen authority layer strings are recorded on the face of this judgment at F2, F11 and F12. Lexby shall report, per registered row, whether authority resolves from a **declarative label** or from a **phrase inside a descriptive sentence**, and shall flag:
- `669:135785`, whose authority is `/dashboards · current source matter master-detail`, resolving on the deliberate anywhere-match (F11);
- `675:25422`, which carries two competing recognised current layers (F12). That frame is not to be re-registered on any basis until the question VJS-CC-OPBOX 6 reserved is determined, and Lexby shall bring it as its own matter.
The audit shall **not** attempt to answer who authored each layer. That is the uncomputable predicate of Order 8 and an audit that guesses at it would manufacture the very fact the estate does not hold. Where the answer exists in a decision log or a commit, cite it; otherwise record "not recorded".

**10. RE-DERIVATION WITH THE CORRECTED PULLER — REFUSED, and its danger recorded.**
`vds figma frames` makes no network call (F5), so the `pull.rs` guard is causally inert as to this ledger and the ordered step could not change the result. Worse, run naively against the filed capture it moves all 167 digests and vacates all nineteen registrations before any cure exists (F6). What is permitted instead is a deeper re-capture written to a **new** path, as evidence only, with the live ledger untouched, and subject to Order 5.

**11. CURE (b) — THE CONFIRMATION SOUGHT IS REFUSED; TWO DECLARATIONS ARE MADE INSTEAD.**
The application invites me to "confirm" what VJS-CC-OPBOX 6 already holds. Binding law is not re-litigated and does not need my confirmation (S-11(c)), and the confirmation is sought in a form that would be read as "already satisfied". So:
(a) **DECLARED**, undisturbed: an express, verified, hash-bound Principal label-resolution act remains an available cure for a refused frame, per VJS-CC-OPBOX 6.
(b) **DECLARED**: neither existing act is one. The 2026-08-03T16:49:22Z instrument is hash-bound and real, and I verified its binding by reproducing the digest it names (F6), but it signs a **ledger** at an aggregate digest and resolves no **label** at any locus; its own face says `warrant: false` and that it "asserts no route conform verdict"; and the four words that are actually the Principal's are "Signed off. proceed all", everything hash-bound being in an agent-drafted incorporation (F13). Signing a record that classifies 122 loci as carrying no declaration does not supply the declaration (R5).
None of the 122, and none of the 26, may be registered on the strength of that act or of any successor ledger-wide signature. A qualifying act must name the file key, the node id, the frame's current content digest, and the locus adopted.

**12. NAME THE INSTRUMENT.**
No assertion that this estate holds N registrations may be made, in any submission, handover, status document or judgment, without naming the `vds` build that read the register and its commit. Lexby shall record in `parity-wt/.vds/` the build that wrote the nineteen rows (`staged-write-s7e`, `6577206`/`19d13b0`) and the fact that the `evidence` schema is **unmerged to master** (F10). The merge is a VDS matter and is referred, not ordered; until it lands, building `vds` from the main line makes this register unreadable, and that is a hazard the estate must hold in its own record rather than rediscover.

**13. RECORD CORRECTIONS, binding on any future citation of this matter.**
(a) No layer is named `LEGACY UNDERLAY`; it is a quarantine marker matched on the leading segment, and the names are the four region suffixes, 44 in all (F3).
(b) The count of `cloned from` authority layers is **ten**, not six; the count of authority layers named `CURRENT CODE` is **zero** (F4).
(c) The `pull.rs` change is **committed and pushed**; "uncommitted" was the load-bearing word and it was wrong (F5).
(d) The demotion is **not** what registered these frames (F9). Any future pleading of this matter must plead the implant, not the demotion.
(e) The estate holds **nineteen** readable registrations under the build that wrote them, and zero under the main line. Both statements are true of different readers and neither is a fact about the record standing alone (F10, R4).
(f) The A4-A5 record's two open questions are **closed** by F6 and F7 and are not to be carried forward as live concerns.

**14. PLEADING STANDARD.** Every count advanced in a future submission in this jurisdiction shall be **derived at filing time** by a command reproduced verbatim in the submission. A pleaded count without its derivation command is an assertion, not a fact, and shall be treated as such. The present submission pleaded six and four where the measured values are ten and zero, pleaded a mechanism the counterfactual refutes, and called a pushed commit uncommitted. I make no finding of bad faith and I impose no sanction: the duty is to make the work good, not to punish. But every one of those errors ran in the same direction, which is towards relief, and a bench should say so.

**15. PERMISSION TO APPEAL** is GRANTED on Orders 3, 4, 8 and 11, which are the four where I have refused relief a differently-minded bench might grant, and on Order 6. Permission is REFUSED on Orders 12 and 13(e): which build reads a file is a measurement, reproducible by running both binaries.

---

## WHAT I REFUSE, AND WHY

**I refuse to vacate the nineteen registrations, and I refuse to be drawn into doing so.** No party asked me to. The applicant sought restoration and re-signature, not vacatur; the respondent had no notice of a vacatur case and no opportunity to meet it. One of the opinions before me would have voided eleven registrations *ab initio* and another would have struck all nineteen. Both go well beyond the application. A court that vacates what nobody asked it to vacate, on grounds nobody argued, is not deciding a case; it is running one.

**I refuse the proposition that this estate holds zero registrations.** It is wrong. It was produced by running a build from a branch that does not carry the schema the register was written in (F10). I record it not to embarrass the reasoning, which was careful, but because it is the single most instructive thing in this file: the opinion that most loudly announced "a verification performed with the wrong reader answers about the wrong artefact" was itself performed with the wrong reader. That is why R4 exists.

**I refuse the Order 16 ground as pleaded.** Order 16's enacted words govern the machine *classifier* and its *verdicts* (F16). The applicant's added sentence is not in the order. The prohibition he actually needs is VJS-CC-OPBOX 6's `auto_register_or_manufacture_a_principal_signature`, which is binding and which I have applied at Orders 4 and 11. A county court does not expand a constitutional order by paraphrase.

**I refuse restoration** because I have measured it to be inert on the register and destructive to the signatures (F9), because there is no snapshot of the state it would restore, and because the Principal has said he does not mind what is there.

**I refuse the clean-hands condition** because no instrument in this estate can compute its predicate, and I say so expressly rather than order a condition that would be reported satisfied every time it ran.

**I refuse the confirmation for the 122** because it restates binding law and is sought in a form that would be pressed into service as satisfaction.

---

## RESERVATIONS

1. **Whether an agent-authored marker layer can constitute the Principal's Order 25 declaration.** This is the strongest point in the record and it is not pleaded. Ten of the nineteen registrations rest on layers reading `SOURCE AUTHORITY · <route> · cloned from NNN:NNN`; no code parses that string, no field records it, and F9 shows those layers are what does the registering. The Principal's statement that his own frames are not named "current" points hard at one answer. Deciding it tonight would take ten or nineteen registrations on a ground no party argued, and it is properly a matter for a bench of more than one. Any party may bring it, and it should be brought.

2. **The status of `SOURCE AUTHORITY` as a configured marker.** VJS-CC-OPBOX 6 reserved the status of other existing configured markers. I have not touched it, and I note only that it is not a new synonym coined to evade Order 25 but a pre-existing default.

3. **The general sufficiency of depth-limited frame hashes**, reserved by VJS-CC-OPBOX 6, is untouched. My R1 concerns *serialisation* fragility, which is a distinct defect and was not before that court. I do not decide whether a depth-6 hash is an adequate contract instrument.

4. **Multiple competing recognised current layers** at `675:25422`, reserved by VJS-CC-OPBOX 6 and live on these facts. Order 9 holds the frame; it does not decide it.

5. **The 156 rows in `internal-docs/design/frame-registry.json`** and their relationship to the nineteen native rows. Not pleaded, not measured by me, and not decided. I express no view on whether they are void, dormant or sound.

6. **The four /pipelines sign-offs and the Principal's "signed in error".** The submission recites it; it is not in evidence before me in a form I can verify, and no party produced the exchange. I have not treated it as established and I have made no order turning on it. If it is produced, it is a fresh matter and an obvious one.

7. **Depth.** Every layer measurement in this judgment is bounded at capture depth 6, with 11,207 truncated leaves in the live ledger's own header. I have not treated that bound as absence, which VJS-CC-OPBOX 6 forbids, and I have equally declined to treat it as presence. The operative predicates read depth 1 only, but that is a statement about the rule and not about the drawing.

8. **The 18 self-disclaiming frames outside the seven disclosed.** F7 establishes that none is in the ledger and none is scoreable. Whether any of them ought to be in the ledger was not argued and I do not decide it.

---

*Aldermarsh CCJ*
*County Court, <subscriber> jurisdiction*
*2026-08-03*
