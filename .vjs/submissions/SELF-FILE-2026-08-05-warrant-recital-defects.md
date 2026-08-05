# Self-file (ACT-003:s5): three defects in my own work, found on the Principal's check-your-work order

Filed by Lexby, 2026-08-05. Restorative, not punitive; each defect names its cure.

## 1. The signed warrants mislabel a class: `4f_subscriber_paths` is not a class

Both registered warrants carry schedule items labelled `4f_subscriber_paths`. ACT 12 s4's
exhaustive list gives (f) to CONSOLIDATION; there is no subscriber-path class, and textual
amendment of an assented statute is on nobody's list but the Sovereign's. The items'
substance was right (prepare, never make); the label was wrong. The warrants are signed
instruments and are not edited: the CURE is that the schedule with operative force is the
one a Commission enters and CERTIFIES under s4(g), which will carry correct classes, and
this record. The mislabelled recitals are recitals.

## 2. A mischaracterised finding: the types.rs cite is not stale in the Act's own tree

WARRANT-OPBOX-001's schedule describes "the stale address in the enacted ACT 12 mirror: it
cites types.rs:371 for the Order struct, which a 2026-08-05 structural split moved".
Measured on this check: in CANON, the Act's own jurisdiction, the cite is VALID (the Order
struct sits at types.rs:370). The staleness exists only in the First Subscriber's vendored
tree, whose own structural split moved the code out from under a canon cite. The defect is
divergence-consequence in the subscriber, not an address error in the Act; the cure is the
subscriber-side audit note, not a correction to the enacted text.

## 3. Two vacuous scans before one true one

Before locating the publication blockers correctly, I ran two denylist scans that compared
the register's entries as PLAINTEXT against the Acts. The register holds HASHES, never
plaintext, so both scans matched nothing and could match nothing - a check that cannot
fail - and for a period I recorded the false conclusion that the Acts carried no
denylisted terms and that a warrant recital was wrong. The third scan used the gate's own
tokeniser and hash set and found the nineteen hit lines the warrant recital had correctly
anticipated. The lesson is standing law in my own memory and was not applied: a scan must
use the gate's matcher or it measures nothing. The cure was the tokeniser-faithful scan,
whose output grounds the Subscriber Pseudonymity Act now lodged for scrutiny.
