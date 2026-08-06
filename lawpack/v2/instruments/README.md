# Engrossed instruments

The texts that Sovereign assent records pin **by digest**, held here so that a
subscriber who clones this canon can read the thing that was actually signed.

Established by [2026] VJS-CC-VJS 21 D6.

## Why these files are here

An assent record in `../provenance/assent/` says who signed what, when, in what words,
and pins the instrument by sha256. Until 2026-08-06 those pins pointed into
`.vjs/submissions`, which [2026] VJS-CC-VJS 20 held may be untracked from publication.
Untracking it while the pins pointed there would have exported a reference that resolves
nowhere to every subscriber's clone, and CC-VJS 20 had already refused a cure of exactly
that shape: one that satisfies a gate on the author's disk and fails it in every
subscriber's clone.

So the pinned texts moved to where they can be read.

## The rule about bytes

**A file in this directory is byte-identical to the text that was signed, and nothing may
be added to it. Not a header, not a projection note, not a licence banner, not a
whitespace fix.**

The digest in the assent record is taken over these bytes. A note explaining that a file
is a copy would change the bytes and break the signature it exists to preserve, which is
the note defeating its own purpose in the most literal way available. Anything that needs
saying about these files is said here, or in the assent record. Never in the instrument.

Verify any of them:

```
sha256sum lawpack/v2/instruments/<file>
grep instrument_digest lawpack/v2/provenance/assent/<the matching record>.yaml
```

## What is NOT here, and why that is deliberate

Four pinned texts are absent, and their assent records say so in terms:
`instrument_text_published: false`, with a reason and the authority for it.

Those four carry a registered subscriber term inside their bytes. The bytes cannot change
without breaking a Sovereign signature, and cannot be published without publishing the
term the pseudonymity Acts exist to keep out of a public tree. Neither horn is available,
so they are held in `.vjs/submissions` - a store that is **registered** in
`.vjs/store-register.yaml` and **not published** - and the absence is declared.

A declared absence is not a dangling reference, and the difference is worth being precise
about. **The digest still travels.** Every subscriber can see who signed the instrument,
when, in what words, and what its bytes hash to. What a subscriber cannot do is read it.
What a subscriber retains is the ability to verify it against the pin if it is ever put in
front of them. CC-VJS 21 held that this is the narrower of the two losses on offer, and
the one the pseudonymity Acts require be borne.

## One of these was recovered rather than copied

`2026-06-12-assented-record-protection-act-void-first-draft.yaml` was not moved here from
a live file. It was gone.

It was deleted by commit `b549208` - the very commit that COMMENCED the Act it is the
draft of - so from 2026-06-12 its assent record pinned a file that did not exist, and
nothing noticed for nearly two months. The blob was recovered from `27c2d80`, hashes
exactly to its pin `0fffd4f8...`, and carries no denylisted term. Restoring it is a strict
improvement on every axis, which is why CC-VJS 21 D7 directed it rather than leaving it to
the next person to find.
