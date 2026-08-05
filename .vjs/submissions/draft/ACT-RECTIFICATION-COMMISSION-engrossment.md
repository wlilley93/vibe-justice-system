# Engrossment certificate: the Executive Rectification Commission Act 2026

**Instrument:** `.vjs/submissions/draft/ACT-RECTIFICATION-COMMISSION.yaml`
**Engrossed by:** Lexby, Clerk-Drafter, 2026-08-05
**Authority:** Standing Committee round 5, full bench: **ADOPTED 4-0 AS CONDITIONED** (Restraint
AYE, Codification AYE, Guardrail AYE WITH CONDITIONS, Operability AYE). Adoption is CONSTITUTIVE
([2026] REALM-SC 8). This engrossment discharges the bench's six-point instruction and awaits the
Committee's re-certification BEFORE any digest is put to Sovereign Assent.

| | digest |
|---|---|
| ADOPTED text (`eab8d99`) | `sha256:ba9693e26889245a9bdc4a6d9ae542494c71ae8202e9f9d233b368027bc11912` |
| **ENGROSSED text (this)** | **`sha256:a0a7018d335d70a33fe218e7aebee622abe6244f1fbef90b6d1ee486d7669b3d`** |

## THE DEPARTURE FROM PRECEDENT, RECORDED EXPRESSLY (direction 4)

The omnibus engrossment was header-only and was certified by a digest check. **THIS ONE IS NOT.**
It touches operative text (s5, s4(e), s4(f) clause, s1 and s2 recitals) and adds two duty tokens,
every word of it directed by the bench. The full diff against `eab8d99` is SEVEN changes, each
mapped below to its numbered direction; any change not on this list would be outside the Clerk's
authority, and there is none.

| # | change | direction |
|---|---|---|
| 1 | `assent_source` re-staged to `NOT_YET_ASSENTED_ADOPTED_BY_STANDING_COMMITTEE_2026-08-05_R5` (still off the `front_door.rs:22` allow-list; justification paragraph carried forward unaltered) | 5 |
| 2 | s1 recital `:6` -> `:7` (verified: `grep -n "^date:"` on both `.justice` files -> line 7) | C-A |
| 3 | s2 `84` -> `85`, conforming to s12's measured total | C-B |
| 4 | s4(e): reclassification occurs in the audit and the report, never in the instrument | finding 3 |
| 5 | s5: the G-A node-tree paragraph and G-B loadability sentence, VERBATIM as supplied | 1 |
| 6 | s5: the s4(f) superseded-record clause | finding 2 |
| 7 | s5 tokens: `compare_every_node_at_its_path_including_sequence_items` (must), `re_rendering_a_record_so_it_ceases_to_load` (must_not) | 1 |

## STRUCTURAL RE-PARSE (direction 4), EXECUTED

```
sections: 12   duty tokens: 65   duplicates: none
collisions with ACT-PROCEEDINGS-DISCIPLINE: none
collisions with enacted lawpack/v2: none
unmodelled section keys: none   unmodelled kernel_effect keys: none
top-level: only assent_source unmodelled (retained deliberately, raw-scan consumers stated on the face)
citation key: ABSENT (direction 6 - the ordinal is minted at commencement)
vjs validate: OK
```

## THE AMENDED s5, IN FULL (direction 4)

> A rectification under section 4(a) or 4(f) is lawful only on proof that it preserved content. THE
> TEST IS STATED AS WHAT MAY CHANGE AND NOT AS WHAT MAY NOT, because where loss must be impossible the
> mechanism cannot be a list of names (`crates/vjs-core/src/types.rs:440-446`, READ - the kernel's own
> stated reasoning for its catch-all). Three drafts of this section were deny-lists, each drawn short,
> and the third was shown to pass a proof while an order's `appealable`, `bench`, `case_file_digest`
> and `court` were rewritten.
>
> THE TEST. Compare the record AS FILED before and after. Every key the file carries, at every depth,
> must be present after, and its normalised token sequence must be byte-identical before and after
> following scalar reassembly and whitespace normalisation. THE ONLY PERMITTED DIFFERENCE is a change
> of scalar style, quoting, indentation, line folding or key order which leaves every key present and
> every normalised value identical. A key present before and absent after is a FAILURE, whatever its
> name; a key absent before and present after is a FAILURE.
>
> THE COMPARISON IS OVER THE FILE'S NODE TREE AND NOT OVER ITS KEY NAMES. Every NODE the file carries
> must be present after at the same path: every mapping key with its value, and every SEQUENCE ITEM
> at its index, at every depth. A sequence that loses an item, gains an item, or changes the order of
> its items is a FAILURE, whatever key names survive elsewhere in the file; and where the file's root
> is a sequence, the root is itself compared as a node. A duplicate key name is compared per occurrence
> and not per name. Every comment the file carries is preserved. A permitted change of style, quoting
> or folding is permitted only where the record remains loadable by the reader that loaded it before;
> a re-rendering that changes a scalar's parsed type, or that renders any record unloadable, is not a
> permitted difference and is not content-preserving.
>
> THE PROOF IS TAKEN OVER THE FILE AND NEVER OVER THE LOADED STRUCTURE. `Statute`, `StatuteSection`,
> `KernelEffect` and `Regulation` model no catch-all and deny no unknown field
> (`crates/vjs-lawpack/src/lib.rs:257-298`, READ; EXECUTED `grep -rn "serde(flatten\|deny_unknown_fields"
> crates/vjs-lawpack/src/` -> zero), so a load silently discards every unmodelled key and a proof taken
> after the load cannot see its deletion. A proof taken over a loaded structure is not a proof under
> this section.
>
> THE TEST APPLIES UNCHANGED TO EVERY CLASS OF GOVERNED RECORD and the Commission does not adapt it
> to a schema. In particular it applies to a `Statute` and to a `Regulation`, which carry no `holding`,
> no `directives`, no `forbidden` and no catch-all, and to which a test framed in those terms would be
> vacuous - so that a consolidation under section 4(f) cannot rewrite a section's `text`, a
> `kernel_effect`'s `must`, `must_not`, `may`, `prohibits`, `proof` or `defines`, a `Regulation`'s
> `authority`, or any `id`, with a passing proof.
>
> FOR THE AVOIDANCE OF DOUBT AND WITHOUT NARROWING THE ABOVE, the following are within the test and a
> change to any of them is not content-preserving, whatever it is labelled: `holding`, each directive's
> `must`, `actor`, `id` and `when`, each forbidden clause, `status`, `assent_source`, `citation`,
> `issue`, `court`, `jurisdiction`, `repo_code`, `supersedes`, `exceptions`, `cites_authorities`,
> `bench`, `case_file_digest`, `convened_at`, `vote`, `appeal_of`, `appealable`, `source_opinion`,
> `runtime_summary` and `created_at` (the named fields of `Order`, READ `types.rs:371-423`), and every
> key preserved under `extra` (`:447`). Flipping a `status` de-forces an instrument; rewriting a
> directive's `when` guts the duty while its `must` stays byte-identical; flipping `appealable`
> removes an appeal route with a green proof beside it.
>
> IN A CONSOLIDATION UNDER SECTION 4(f) the test is taken over EACH SUPERSEDED RECORD, which is not
> edited: ACT-PROCEEDINGS-DISCIPLINE:s6 leaves every superseded instrument visible, citable and
> routable, so each passes trivially, and the concordance is the proof for the restatement. There is
> no "before" for a restatement that did not previously exist, and this section does not pretend one.
>
> The Commission PREPARES AND EXECUTES a rectification; it never AUTHORS or MAKES one. Where the act
> is an amendment or a consolidation of a filed order, the maker is the competent court, whose order
> the Commission obtains BEFORE the act, and the disability on the engineer applies identically to a
> commissioner who is an agent seat. The Commission may not substitute another test. A rectification
> described as formal which fails this test is void, and the Commission's own record must carry the
> test and its result.
>
> Where the repair is confined to leading whitespace, the stricter test applies and is available: stripping
> leading whitespace from every line before and after must yield byte-identical text. The rule's force
> in this Act comes from this Act stating it, not from the subscriber county order that first stated
> it ([2026] VJS-CC-OPBOX 162, recorded as the origin and cited as persuasive; a canonical primary Act
> does not incorporate a lower rank's order as its authority).

## WHAT HAS NOT HAPPENED

Nothing is enacted. No citation minted, no lawpack touched, no digest put to assent. **The next step
is the Committee's re-certification of this text; only then does the digest above go to the
Sovereign, whose assent is theirs alone.**
