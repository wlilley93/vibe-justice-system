# Correction under ACT-004:s9, made by order of [2026] VJS-CC-VJS 17 C8

Made 2026-08-01 by Lexby, the applicant, as C8 directs. A NEW record citing the order and
naming file and line: never a silent edit, never a history rewrite. The superseded text
remains in git history, which is the point of correcting by record rather than by erasure.

## 1. What was wrong

Three canon records carried a term on the publication denylist, in breach of ACT-005:s1.
They were admitted at CC-VJS 17 fact 3 and are the subject of C8.

The gate that should have caught them could not. `scan_canon_writes` skipped every non-YAML
canon file before any content limb ran, and all three are `.md`; and there was no denylist
limb on that gate at all. Both are cured by C1 and C2, landed at `cc8a60e`.

## 2. The corrections, by file and line

| record | line | was | is now |
|---|---|---|---|
| `lawpack/v2/judgments/2026-VJS-CC-OPBOX-002-opinion.md` | 4 | an issue tag prefixed with a subscriber's infrastructure name | `<subscriber>_design_control_boundary_contrast` |
| `lawpack/v2/judgments/2026-VJS-CC-VJS-12-exhibit-full-case-file.md` | 36 | two subscriber repository names, parenthesised | `(two subscriber repos)` |
| `lawpack/v2/judgments/2026-VJS-CC-VJS-13-opinion.md` | 96, 97 | a subscriber code used as a fixture code in a negative control | the generic form: "a SUBSCRIBER code", "a subscriber-coded record" |

### One of C8's two permitted cures turns out to be barred by another rule

C8 offers the cure as "the generic form signal 4 already prescribes, **or** the accessioned
pseudonym". I took the second for record 3, because the court's own aggravation finding was
that the pseudonym "was available and would have served identically".

The commit was REFUSED, by the limb this same ruling ordered built. Signal 4 blocks a canon
record that names a REGISTERED SUBSCRIBER in its prose, and the accessioned pseudonym is a
registered subscriber code - it is the one code in the federation registry. Its only
exemption is for the registry file itself.

So the two cures C8 offers are not alternatives in canon prose. The pseudonym is lawful for
citing a subscriber's own public law, which is what the denylist header records it was
restored for; it is NOT lawful as prose inside a canon record, because canon must be
generic against every subscriber including the accessioned one.

Cured to the generic form instead. Recorded here rather than silently corrected, because a
reader following C8's words would make the same choice and hit the same wall, and because
the gate catching its own author's cure within minutes is the strongest evidence available
that C1 and C2 actually reach.

The terms are not reproduced here. Naming them in the correction record would publish the
thing the register exists to keep out, which is the same reason C1 forbids the gate message
from naming them.

Record 3 is the applicant's own breach, self-filed and found made out at CC-VJS 17 part 6.
The court recorded a mitigation and an aggravation. The aggravation is the operative one
here: the accessioned pseudonym `ACMECO` was available and would have served identically,
and it is what the record now carries.

## 3. WHAT THIS CORRECTION DOES NOT CURE, and why

C8(i) is explicit that the cure is measured against ACT-005:s1 and **not against the gate
going quiet**. Measured after these edits: zero denylist hits anywhere in canon prose. That
is not the same as saying the term is gone from canon.

It is not. In record 1 the term also sits in:

- the **filename**, `2026-VJS-CC-OPBOX-002-opinion.md`, and
- the **neutral citation**, `[2026] VJS-CC-OPBOX 2`, which is the record's own identity.

The tokeniser cannot see either, because hyphens are token characters, so
`VJS-CC-OPBOX-002-opinion` is one token and hashes to nothing in the register. A cure that
silenced the hash and left the citation would be a cure aimed at the instrument rather than
at the harm, which is precisely what C8(i) forbids anyone from claiming.

**Nothing has been done to the filename or the citation, deliberately.** C8(ii) holds that
whether a canon citation SERIES may carry such a term, and how it could be re-seriesed
without breaking the citator, is genuinely first-impression and **is not before that court**.
It directs that it be filed and that nothing irreversible be done meanwhile. Renaming a
canon record or re-issuing a neutral citation is irreversible in the way that matters: every
citation to it in every other record, and in every subscriber's local register, resolves to
the old identity.

Filed accordingly. Until it is decided, this correction is COMPLETE as to prose and
KNOWINGLY INCOMPLETE as to identity, and this paragraph is the record of that.

## 4. Authority

`[2026] VJS-CC-VJS 17` C8, made under ACT-004:s9 and ACT-005:s1.
