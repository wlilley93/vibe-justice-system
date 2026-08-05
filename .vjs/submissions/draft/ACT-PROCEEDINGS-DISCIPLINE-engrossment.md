# Engrossment certificate: the Proceedings Discipline Act 2026

**Instrument:** `.vjs/submissions/draft/ACT-PROCEEDINGS-DISCIPLINE.yaml`
**Engrossed by:** Lexby, Clerk-Drafter
**Date:** 2026-08-05
**Authority:** Standing Committee adoption, 2026-08-05, 4-0, all four seats AYE-WITH-CONDITIONS
(`committee-round3/full-bench.md`); adoption is CONSTITUTIVE, `[2026] REALM-SC 8`.

| | digest |
|---|---|
| ADOPTED text (canon `5791128`) | `sha256:ac3a099d84c207b6bbeb43f19a968ad1806f06be53369d36effedd4307a344f6` |
| **ENGROSSED text (this)** | **`sha256:8690040675a2779b5a3c0168f8d1fa70a57182911b1c00735cb6e00a8c458375`** |

51,734 bytes, 719 lines, 18 sections, 53 duty tokens.

---

## The digest check the bench ordered ("a digest check only, not a fresh vote")

EXECUTED, `diff` of the adopted text at `5791128` against the engrossed text:

```
diff <(sed -n '/^sections:/,$p' adopted) <(sed -n '/^sections:/,$p' engrossed)
  -> no output
```

**Every byte from `sections:` onward is identical.** The whole of the operative law - all 18
sections, all 53 duty tokens, both Schedules - is the text the four seats adopted, unaltered. The
diff is confined to the header, and to four changes, each set out below with its authority.

---

## The four header changes

### 1. `drafting_note` STRIPPED

Not a field of `Statute` (`crates/vjs-lawpack/src/lib.rs:256-263`: `id, citation, title, status,
enacted_by, purpose, sections`), and no consumer reads it by raw scan. It was drafting history, and
drafting history belongs in the record of the drafting, not on the face of the law. Its content is
preserved in `ACT-PROCEEDINGS-DISCIPLINE-second-draft-dispositions.md` and the drafting brief.

### 2. `created_at` STRIPPED - **and the bench's stated reason for stripping it is wrong**

The bench directed this key be removed because it "is not a field of `Statute` and would be
**silently discarded**". The first half is true. **The second half is false, and I record it rather
than inherit it**, because a later reader relying on "`created_at` is discarded" will set a wrong
date on the public register and not know it.

MEASURED (READ `crates/vjs-cli/src/gazette/mod.rs:329-343`): the Gazette generator reads
`created_at` **by raw value lookup**, out of the file's own YAML, and uses it as

- the register's **displayed enactment date** (`date`), and
- the newest-first **sort key** (`ts`), where the value carries a `T`.

It falls back to the git-history add date only when the key is absent. So `created_at` is not
discarded; it is discarded *by the loader* and *read by the Gazette*. Two consumers, one key, and
reading only the struct definition finds one of them.

**This is the exact failure this Act exists to catch.** s2 requires an assertion about machinery to
name the address where it was observed, because an inference drawn from one consumer is not an
observation of the system. The bench read `lib.rs:255-263`, stopped, and asserted a system-wide
behaviour - which is `[2026] VJS-CC-OPBOX 159` and `164` in miniature, on the drafting of the very
section that forbids it. It is recorded here, not tidied away.

**The key is still stripped, on a different and sound ground.** 9 of the 10 canon statutes carry no
`created_at` (EXECUTED, `grep -c '^created_at:'` over `lawpack/v2/statutes/*.yaml`; the sole
exception is `10-assented-record-protection.yaml`). With the key absent the Gazette dates the record
from **git history**, which is a measurement; with it present the register shows a **string the
drafter typed**, which is an assertion. Where the two disagree the register would publish the
assertion. Omitting it is therefore the more truthful record, not merely the tidier one.

### 3. `assent_source` UPDATED, and deliberately left OUTSIDE the allow-list

`SECOND_DRAFT_PENDING_STANDING_COMMITTEE_ADOPTION` -> `NOT_YET_ASSENTED_ADOPTED_BY_STANDING_COMMITTEE_2026-08-05`.
The old value was stale on its face: it described a text awaiting a vote that has since been taken.

**The bench's step 2 said to "set `status` and `assent_source`". `assent_source` is NOT set to
`sovereign_assent`, and must not be.** Doing so would be the Clerk asserting the Sovereign's own
act, on a text the Sovereign has not seen - the identical vice to the void self-minted citation this
Act refuses to carry, and to `BREACH-2026-06-09-self-commenced-instrument`.

I confirmed the route the bench asked about, and it is not the one the question assumed. `assent_source`
is not a `Statute` field, but that is irrelevant to enforcement, because **nothing reads it through the
loader**:

- `crates/vjs-core/src/front_door.rs:42-50`, `declares_valid_assent`: a raw line scan, `strip_prefix("assent_source:")`, column zero, against a two-value allow-list.
- `crates/vjs-engine/src/assent.rs:154`, `assent_resolves`: `top_level(content, "assent_source")` - again the file's own bytes.

So the field is fully enforced on any file placed in a governed root, model or no model. And on the
mechanics it would fail anyway: for `sovereign_assent`, `assent_resolves` requires a pinned
`assent_instrument_digest`, **or** a Sovereign-assent provenance record naming this id or citation,
**or** the record already established at HEAD (`assent.rs:150-175`). This text satisfies none of the
three. A `sovereign_assent` declaration here would be both unlawful and inert.

### 4. `status` KEPT at `draft` - which is a command, not a formality

`ACT-001:s7` provides in terms: *"Proposed law must be marked draft. Binding law requires authorised
adoption."* Its kernel effect carries `must: mark_proposed_law_as_draft` and `must_not:
agent_draft_becomes_binding_by_fact_of_being_written`. `AuthorityStatus::is_live()`
(`crates/vjs-core/src/types.rs:53-65`) admits only `Binding` and `InForce`, and its own comment
records that even `Proposed` is pre-enactment and must not resolve as live law. Committee adoption
is constitutive of the *text*; it is not enactment. `draft` stays until assent.

---

## No `citation` key

Confirmed absent (`'citation' in d -> False`). The ordinal is minted deterministically at
commencement. The corpus records two void self-mints on this ground and this text will not be a
third.

## What lodgement looks like, so nobody has to guess later

Assent pins **this** digest. The lodged statute is a **different file**, and that is the corpus's own
established route, not an irregularity: the assent record for the Assented-Record Protection Act pins
`.vjs/submissions/draft/2026-06-12-...-void-first-draft.yaml` at
`sha256:0fffd4f8...`, while the law itself lives at `lawpack/v2/statutes/10-assented-record-protection.yaml`
and recites that digest in its `purpose` (READ
`lawpack/v2/provenance/assent/2026-06-12-assented-record-protection-assent.yaml`). On assent, and not
before, three things are created together: the assent provenance record naming this instrument and
digest; the lodged statute carrying the minted citation, `status: in_force` and `assent_source:
sovereign_assent`; and the commencement entry.

## What has NOT happened

Nothing is enacted. No lawpack statute has been touched, no digest re-pinned, no subscriber mirror
altered. `vjs validate` -> **OK**. **Sovereign Assent is the Principal's alone and has not been
sought or presumed.**
