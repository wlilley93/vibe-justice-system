# RE-CERTIFICATION CERTIFICATE — ACT-RECTIFICATION-COMMISSION (engrossed)

*Filed verbatim per the certifying bench's own direction; returned in-session 2026-08-05.*

**DISPOSITION: CERTIFIED**, 4-0, with one hunk expressly ruled on (below) and one direction to the Clerk that does not condition this text.

**DIGEST ASSENT SHOULD PIN (measured by this bench, not copied):**
`sha256:a0a7018d335d70a33fe218e7aebee622abe6244f1fbef90b6d1ee486d7669b3d`

---

## 1. The diff, every hunk (`git diff eab8d99 f6c1a3d -- .vjs/submissions/draft/ACT-RECTIFICATION-COMMISSION.yaml`)

SEVEN hunks, no more. Certificate claims seven. Exact match, none undisclosed.

| # | hunk | bench direction | ruling |
|---|---|---|---|
| 1 | `assent_source` `FOURTH_DRAFT_PENDING_STANDING_COMMITTEE_ADOPTION_2026-08-05` -> `NOT_YET_ASSENTED_ADOPTED_BY_STANDING_COMMITTEE_2026-08-05_R5` | not on the amendment list; Clerk maps it to direction 5 | RULED ON — see §7 |
| 2 | `:6` -> `:7` | C-A | in authority, verified independently |
| 3 | `84` -> `85` distinct `.vjs` surfaces | C-B | in authority |
| 4 | s4(e) + "A reclassification occurs IN THE AUDIT AND THE REPORT, never in the instrument..." | s4(e) audit-only clause | in authority |
| 5 | s5 + G-A/G-B paragraph (10 lines) | G-A, G-B | in authority |
| 6 | s5 + s4(f) superseded-record clause (4 lines) | s4(f) clause | in authority |
| 7 | `+ compare_every_node_at_its_path_including_sequence_items` (must), `+ re_rendering_a_record_so_it_ceases_to_load` (must_not) | tokens | in authority, tokens byte-identical to the bench's |

No text was deleted anywhere in the diff except the three intended substitutions (`assent_source` value, `:6`, `84`). No section added or removed. No `citation`, no `status` change.

## 2. G-A / G-B verbatim, and certificate-vs-file

`python3` normalised comparison of the certificate's block-quoted s5 against `sections[s5].text` in the YAML: **`NORMALISED MATCH: True`** — word-for-word identical; the only difference is blockquote re-wrapping, which the folded YAML scalar erases anyway. The certificate quotes the real text; it is not a paraphrase.

G-A carries every element the bench specified: node tree not key names, mapping key with value, sequence item at its index, at every depth, loss/gain/reorder all FAILURE, root-as-sequence compared as a node, duplicate key compared per occurrence, comments preserved. G-B carries the loadability limb: style/quoting/folding permitted *only where the record remains loadable by the reader that loaded it before*, and parsed-type change or unloadability is "not a permitted difference and is not content-preserving."

**Recorded defect, not fatal:** `committee-round[1-4]/` exist; there was **no `committee-round5/`**. The only repo record of the bench's verbatim words was the Clerk's own commit message and certificate. The Clerk is DIRECTED to file the round-5 record. This does not condition the text, which we have read in full. *(Discharged by the filing of this directory.)*

## 3. Re-parse (executed, `python3` + `yaml.safe_load`)

```
sections: 12
duty tokens: 65   (must 35 + must_not 30; plus 28 defines, 2 exceptions, not duties)
duplicate duty tokens: none
collisions with ACT-PROCEEDINGS-DISCIPLINE.yaml (52 tokens): none
collisions with lawpack/v2/statutes (10 files) + regulations (30 files), 290 tokens: none
section keys present: id, kernel_effect, text, title        -> all modelled (StatuteSection)
kernel_effect keys present: defines, exceptions, must, must_not -> all modelled (KernelEffect)
top-level: only `assent_source` unmodelled against Statute; disclosed on the certificate's face
grep -rn "serde(flatten|deny_unknown_fields" crates/vjs-lawpack/src/ -> 0   (s5's own recital, confirmed true)
```

## 4. Digest and cleanliness

```
sha256sum ACT-RECTIFICATION-COMMISSION.yaml
  a0a7018d335d70a33fe218e7aebee622abe6244f1fbef90b6d1ee486d7669b3d   -> MATCHES certificate
git show eab8d99:...yaml | sha256sum
  ba9693e26889245a9bdc4a6d9ae542494c71ae8202e9f9d233b368027bc11912   -> MATCHES certificate
git status --porcelain -- <file>  -> CLEAN; unchanged since f6c1a3d
```

## 5. C-A verified independently

`grep -n "^date:"` on `2026-cc-opbox-16.md` and `17.md` -> `7:date: 2026-06-07` in both. `:7` is correct; `:6` was wrong; both dates are 2026-06-07, two days before commencement. The recital now reads true.

## 6. Front door, status, citation

`front_door.rs:22` — the engrossed `assent_source` value is off the allow-list, so the record is NOT an assented record and gets no protection from the floor. `status: 'draft'`. `citation` key ABSENT (the ordinal is minted at commencement). Nothing here brings the instrument into force.

## 7. The one hunk requiring a ruling — `assent_source`

It is not on the amendment list this bench recited. We rule it **WITHIN authority and RATIFY it**: both the old and new values are off the `front_door.rs:22` allow-list, so the change has nil operative effect; the old string ("PENDING_STANDING_COMMITTEE_ADOPTION") became false the moment we adopted, and leaving it would have engrossed a falsehood onto the face of the instrument; and it is disclosed as change #1 in the Clerk's own table rather than slipped in. It is a staging marker, not operative text. Had it moved *onto* the allow-list, or had it been undisclosed, this certificate would read REFUSED.

## 8. Sanity test of G-A / G-B on their stated terms

**(a) Root-sequence file loses one item.** Old text passed it (every key *name* survived). New text: "A sequence that loses an item... is a FAILURE, whatever key names survive elsewhere in the file; and where the file's root is a sequence, the root is itself compared as a node." **REFUSED.** Construction closed.

**(b) `appealable: true` -> `"true"`.** Old text passed it (quoting was THE ONLY PERMITTED DIFFERENCE). New text: "a re-rendering that changes a scalar's parsed type... is not a permitted difference and is not content-preserving." `types.rs:415` is `Option<bool>`, so the string form fails deserialization — caught twice over, by the parsed-type limb and the loadability limb, backed by `re_rendering_a_record_so_it_ceases_to_load`. **REFUSED.** Construction closed.

---

## THE BENCH

**RESTRAINT:** The Clerk changed seven things and seven things only, each traceable to a direction, and the one that was not an amendment is a nil-effect staging marker disclosed on the face — no power was taken that we did not give.

**CODIFICATION:** C-A now cites line 7 where the dates actually live and C-B recites 85 conforming to Schedule 2's own measured total, so the two recitals this Act rests on are true against the files rather than against a prior draft's memory.

**GUARDRAIL:** Both constructions I drove through my own allow-list cure are dead on the new text — the root-sequence deletion fails as a node-tree loss and the `true` -> `"true"` type-flip fails twice, at parsed type and at loadability — and the two tokens land in `must`/`must_not` exactly as supplied.

**OPERABILITY:** The instrument re-parses at 12 sections and 65 duty tokens with zero duplicates, zero collisions against the Discipline Act or the 40 enacted v2 files, and zero unmodelled section or `kernel_effect` keys, and the digest I measured is the digest the certificate claims.

**Digest to be put to Sovereign Assent: `sha256:a0a7018d335d70a33fe218e7aebee622abe6244f1fbef90b6d1ee486d7669b3d`.** Nothing is enacted by this certificate; assent is the Sovereign's alone.
