# Two defects in the order pipeline: the citator is write-only, and `order apply` destroys the record

Found on 2026-07-27 while filing [2026] VJS-CC-OPBOX 5. Neither was being looked for. Both are
load-bearing, because both sit on the path the governing instruction actually names.

## Defect 1: the citator is write-only

The standing instruction is: **check the citator FIRST; if a binding ratio is on all fours, follow it
and cite the ruling, do not re-litigate (S-11(c)).** Convening is reserved for genuinely
first-impression questions.

No command surfaces an order. Both of the ones that look like they should return the constitutional
Acts and nothing else:

```
$ vjs lookup --issue credential_return_through_audited_path | sha256sum
eef4b7ff5a8536179603825990461c3a7bcba485e80c8add8453d0ddaa521a77
$ vjs lookup --issue this_issue_does_not_exist_zzz          | sha256sum
eef4b7ff5a8536179603825990461c3a7bcba485e80c8add8453d0ddaa521a77
```

Byte-identical. The second issue does not exist. The first is the `issue:` field of a `status:
binding` County Court order sitting in `.vjs/orders/`, filed minutes earlier and passing
`vjs validate`.

`vjs route` is the same. Asked about the decided issue it answers `AllowedWithConditions`,
`Court required: false`, and lists five `ACT-001` sections as the binding authorities. The order that
decides that exact question is not among them.

**Why it matters more than it looks.** This is not a missing convenience. The resolver's authority
hierarchy (`ACT-001:s3`) expressly ranks "County Court orders" above "local decision logs". The orders
are written, validated, committed, and then never read back by anything. So:

- every issue presents as first-impression, because the check for existing law returns the same
  answer whether or not there is any;
- re-litigating settled law is the *default* behaviour, not an error case;
- the S-11(c) prohibition on re-litigation cannot be complied with through the tooling, only from
  memory, which is exactly the failure mode a citator exists to remove.

It is the "check that cannot fail" shape, one layer up from the code: a lookup whose output is
independent of its input cannot tell you anything, and its green is indistinguishable from its red.

## Defect 2: `vjs order apply` silently deletes most of an order

Running `vjs order apply` on a filed order rewrote the file in place and removed **69 lines**. It
reported success and said nothing about what it dropped:

```
$ vjs order apply .vjs/orders/2026-VJS-CC-OPBOX-CREDENTIAL-AUDITED-PATH-005.yaml
Order applied: 2026-VJS-CC-OPBOX-CREDENTIAL-AUDITED-PATH-005
$ git diff --stat
 1 file changed, 69 deletions(-)
```

Fields destroyed: `title`, `question`, `fact_corrections`, `execution_findings`, `reserved`,
`rows_already_written`, `full_case_file`, `full_case_file_digest`, `filed_submission`, `convening`,
`permission_to_appeal`.

The holding, directives and prohibitions survive. What does not survive is everything that makes the
holding *checkable*: the question it answers, the case file it was decided on, the corrections to the
filing's facts, and the questions expressly left open.

**Why that is worse than losing prose.** `fact_corrections` is not commentary. In [2026] VJS-CC-OPBOX
4 the bench recorded ten of them, and called FC-3 - that two advocates had quoted a source-file
comment believing it was the words of the court - "the most important correction in the case". An
`apply` over that order would have deleted it, leaving a ruling whose reasoning cites facts that the
same ruling had found to be wrong, with no record that they were found wrong.

`reserved` is similarly load-bearing: it is the list of things the order deliberately did NOT decide.
Deleting it converts "not decided" into "silent", and a later reader cannot distinguish a question
the court left open from one nobody asked.

The 005 file was restored from the commit and re-validated. `vjs validate` passes both before and
after the deletion, so validation does not notice, and the loss is invisible to every existing check.

## What follows

These are recorded, not fixed, and the split is deliberate: the credential order that surfaced them
is executed and filed, and a kernel change to the resolver is its own piece of work with its own
record. Two things to be decided when it is picked up, both of which are forks rather than
implementation details:

1. Whether `lookup`/`route` should return orders alongside statute, and how a County order that is
   *distinguishable* on its facts should present, since surfacing an inapplicable ratio confidently
   is its own hazard.
2. Whether `apply` should be non-destructive (write a derived runtime projection beside the order,
   leaving the filed record untouched) or whether the dropped fields are genuinely not part of what a
   runtime needs, in which case the projection belongs in a different file and the in-place rewrite
   is still wrong.

Until then: **do not run `vjs order apply` on a filed order.** It is not needed for the order to be
good law - 005 was binding, validated and committed before `apply` was ever run, and the four orders
before it appear never to have been applied at all.

---

## Resolved, same day

All three defects are fixed and pushed (`cfe6cfb`). The two forks this document left open were
answered by the code rather than needing a bench, because neither turned out to be a genuine choice:

1. **Should `lookup`/`route` return orders?** `ACT-001:s3` already ranks County Court orders in the
   authority hierarchy. The resolver was not implementing its own statute, so this was a bug, not a
   design question. Orders are overlaid onto the authority graph with their `issue` as a tag;
   `resolve_authority` hoists on-point authority so it survives route's truncation, and route still
   *displays* by the s3 rank hierarchy. Inclusion is guaranteed, presentation stays lawful.

2. **Should `apply` be non-destructive?** Losing `fact_corrections` and `reserved` is data loss that
   nobody would design, so there was nothing to weigh. `#[serde(flatten)]` makes any unknown key
   round-trip; verified by applying an order and comparing parsed YAML (zero keys lost, zero values
   changed). Chosen over naming the missing fields because the existing comment above `citation`
   shows that cure already failed once - it leaves the next author of the next field to remember.

**A third defect was found only because the fix was made loud.** Six of fourteen filed orders did
not parse at all (`missing field supersedes`, `missing field runtime_summary`, `court: privy`). They
were binding, committed, and invisible to the kernel. The first version of the overlay swallowed the
parse error and silently emptied the entire citator - a fail-open inside the fix for a fail-open,
presenting identically to the bug being cured. Reading each file separately and printing which order
failed is what surfaced them.

The standing prohibition on `vjs order apply` is **lifted**.
