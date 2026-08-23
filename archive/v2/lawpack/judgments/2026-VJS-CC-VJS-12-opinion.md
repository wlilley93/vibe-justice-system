# [2026] VJS-CC-VJS 12 - Lawpack resolution

**County Court, sitting at first instance. Marbury J.**
Submission: `SUBMISSION-2026-07-31-172302`.
Convening: `CONVENING-county-2026-07-31-172428`, case file `sha256:b2cda5a0…`.
Full-length case file exhibited at `2026-VJS-CC-VJS-12-exhibit-full-case-file.md`.

## The question

A repository is invoked as a VJS jurisdiction. The kernel looks for its lawpack at
`<repo>/lawpack/v2`. If that directory is not there, it returns a lawpack whose eight vectors
are all empty, prints nothing, and carries on. Must it refuse instead? And separately: the
lock that records which lawpack a jurisdiction is bound by is computed over `manifest.toml`
alone. Is that a pin?

## Why the respondent's best argument is also the applicant's

Both sides built on the same twenty lines of source. `overlay_filed_orders` declines to fail
when `.vjs/orders/` is missing, and says why: *"a fresh subscriber repo has no orders yet, and
refusing to route in that state would be worse than the defect being fixed."* The respondent
says a missing lawpack is the same kind of absence. The applicant says it is the opposite kind.

The applicant is right, and the test is acquisition. **A repository acquires orders by
operating. It never acquires a lawpack by operating.** Orders accrue as a jurisdiction does
its work, so their absence is a stage every jurisdiction passes through. A lawpack is
subscribed to, once, in a deliberate act that `vjs invoke` exists to record. Its absence is
not a stage but the failure of that act. The two look alike on disk - a directory that is not
there - and are opposite in law. This court affirms the orders rule and declines to extend it.

## ACT-001:s5 and what "deterministically" secures

The clause is short: *"The kernel loads the lawpack and resolves authority deterministically.
It does not litigate, judge, or call models. It returns bounded instructions to the agent."*

The respondent reads "returns bounded instructions" as a guarantee of availability, and it is
not nothing: every command routes through `build_kernel_context`, so a fallible loader turns a
missing directory into an outage that takes `status` and `lookup` with it - the very commands
an operator would reach for. That is a real cost and this court does not dismiss it.

But it is answered by scoping the refusal, not by keeping the substitution. What
"deterministically" secures is that the same question gets the same answer from the same law.
Measured on 2026-07-31, `vjs lookup --issue enforcement` returned four constitutional sections
in `vibe-justice-system` and nothing at all in `vibe-design-system`: same binary, same flags,
the same minute, and no marking on either answer. That is determinate in form and arbitrary in
substance. A resolver that silently substitutes a canon has not resolved authority; it has
resolved something else and not said so.

## The aggravating fact

`vjs status` in the affected repository prints `VJS installed: true` and names a lawpack.
An operator has no way, short of reading the kernel source, to learn that their jurisdiction
has no law. A silent failure is one thing. A silent failure that prints a reassurance is
another, and it is the reason D6 is in the order: the two states must be distinguishable at
the surface an operator actually reads.

## The bootstrap, which is a real objection and a small one

`vjs invoke` calls `build_kernel_context` on line 20, before it has written the config that
would say where the lawpack is. A refusal keyed on the config would therefore make invocation
itself impossible, and the respondent was right to press it.

The answer is that `invoke` is asking a question it already knows the answer to. It is HANDED
a lawpack in `--lawpack`. It must resolve that and digest it directly, never by way of a
kernel context. The refusal in D1 is keyed on `.vjs/config.toml`, which at invoke time does
not yet exist. The two rules do not collide; they were only made to look as though they did
by an ordering accident.

## The flag that labels

`--lawpack` is accepted, printed, written into `config.toml` and into `lawpack.lock`, and has
no effect on what is loaded. Passing a correct absolute path to a real lawpack changes nothing.

This court declines to treat that as a cosmetic defect. The artefact it produces is not
incomplete but false: `vibe-design-system`'s lock names `vjs-v2@0.1.0` and pins
`e3b0c44298fc1c14…`, the sha256 of the empty string, under a heading that asserts a
subscription. A later court reading that record would be misled by it, which is precisely the
harm a lock exists to prevent. A flag must select or must go.

## The pin that cannot move

`compute_digest` reads one file: `lawpack/v2/manifest.toml`, nineteen lines carrying an id, a
version, a status, two timestamps and a `[limits]` table. Appending a line to
`statutes/01-authority.yaml` left the digest at `14cdb3337039ffdb…`, which is byte-identical
to the digest `vibe-justice-system` has had pinned since 2026-07-27.

So every statute, regulation, rule, order, spec, invariant, decision and obligation sits
outside the pin, and the canon may be rewritten entirely while every subscriber's lock still
verifies. The respondent's churn objection - that hashing the tree moves the digest on any
editorial change and trains operators to re-lock without reading - is a genuine risk and is
an argument about **cadence**, not **scope**. It is met by making a re-lock a recorded act
that carries a reason, as the sibling design system's enforcement lock already requires. It
is not met by narrowing the pin until it cannot detect the change it exists to detect.

## Disposition

Granted in part. (a), (c), (d) and (e) upheld; the availability objection succeeds on (b) and
changes how (a) is implemented, not whether. Directives D1-D6 as in the order.

`vibe-design-system` is to re-invoke against a resolved lawpack once D1-D4 are in force, and
the superseded lock is to be recorded rather than overwritten in silence.

## A note on blame

None is apportioned. The identical defect one level down was found, reasoned about at length,
and fixed on 2026-07-27; the larger half of the same citator was left in place in the same
edit, with a comment beside it saying what it was. The duty under ACT-001 is to make the work
good, and this order is that remedy.
