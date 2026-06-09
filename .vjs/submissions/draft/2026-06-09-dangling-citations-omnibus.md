# DRAFT: The Lawpack (Dangling Citations) Omnibus 2026

**status:** VOID FIRST DRAFT pending the Sovereign's assent. When adopted, each schedule item lodges as its own law object under `lawpack/v2/` with `assent_source: sovereign_assent` (or a standing-bounded route tracing to specific assent).
**Occasion:** the 2026-06-09 repo review found 21 law-object citations that resolve to nothing. The kernel now reports these mechanically (`vjs validate`, code `DANGLING_REFERENCE`, warning severity). One was a deliberately negated mention (PC-005's "no DEC-INSTITUTIONS-001") and is excluded by the checker's negation rule; the remaining 20 cite objects that were never enacted. The specs that cite them claim runtime force, so the gap is drift the citator cannot be trusted under until cured.

## Recitals

1. Specs are first-class law objects (ACT-001:s8); a spec citing a decision, invariant, or obligation that exists nowhere asserts authority it does not have.
2. The remedy is enactment or excision by due process, not silent authoring by the engineer (the assent rules: nothing carries runtime force without an assent_source resolving to Sovereign assent).
3. The obligations category (`OBL-*`) is cited five times and defined zero times; it needs both a directory convention (`lawpack/v2/obligations/`) and its first instruments.

## Schedule: the twenty objects to enact (or excise)

Each item carries the content its citing context already promises. Drafting follows the audit of 2026-06-09; the Sovereign may strike any item, in which case the citation is removed from the citing spec instead.

### Decisions
1. **DEC-AGENT-001** (cited by SPEC-AGENT-001): the route-permit-obligations-proof-log-validate lifecycle binds agents on load-bearing work (implementation decisions, public record changes, external acts, security-sensitive acts).
2. **DEC-KERNEL-001** (SPEC-KERNEL-001): the kernel determinism mandate - local-first, model-judgment-independent, scope crates/vjs-core/** and crates/vjs-lawpack/**.
3. **DEC-KERNEL-002** (SPEC-KERNEL-001): kernel refactor and dependency constraints - isolation rules, network/model-call prohibitions, authority-ranking-change gates.
4. **DEC-ASSENT-DRAFT-001** (SPEC-ASSENT-DRAFT-001): AI legislature competence bounds, delegated lawmaking conditions, assent_source semantics, prohibited subjects.
5. **DEC-ASSENT-DRAFT-002** (SPEC-ASSENT-DRAFT-001): assent lifecycle gates from draft to Gazette entry.

### Invariants
6. **INV-AGENT-001** (SPEC-AGENT-001): agents cannot bypass the lifecycle on load-bearing work.
7. **INV-AGENT-002** (SPEC-AGENT-001): permit precedence, contemporaneous obligation log, post-execution validation.
8. **INV-KERNEL-001** (SPEC-KERNEL-001): kernel code is deterministic - no randomness, datetime, or environment affecting law output.
9. **INV-KERNEL-002** (SPEC-KERNEL-001): kernel makes no network or model calls; local-first is non-negotiable.
10. **INV-KERNEL-003** (SPEC-KERNEL-001): dependency additions, network capability, model calls, and authority-ranking changes require judicial review gates.
11. **INV-ASSENT-DRAFT-001** (SPEC-ASSENT-DRAFT-001, INV-ASSENT-SOURCE-001): assent_source mandatory on every statute, regulation, order, decision; self_authorised forbidden.
12. **INV-ASSENT-DRAFT-002** (same citing records): delegated lawmaking must declare prohibited_subjects; removals from the prohibited list forbidden.
13. **INV-ENTRENCHED-GATE-001** (ACT-COMPUTER-FIRST-REALM): the s14 entrenchment gate - s14 amendable only by a sovereign-assented constitutional act citing s14 by number.

### Obligations (new category: lawpack/v2/obligations/)
14. **OBL-LOG-001** (SPEC-KERNEL-001, SPEC-AGENT-001): governed actions carry contemporaneous decision logs as proof of performance. (The kernel already mints a runtime obligation with this id on routes; the lawpack instrument makes the duty law rather than a code artifact.)
15. **OBL-TEST-001** (SPEC-KERNEL-001): kernel refactors and dependency changes pass determinism tests before merge.
16. **OBL-VALIDATE-001** (SPEC-AGENT-001): validation proof attaches to agent actions; absence triggers a proof_missing review.
17. **OBL-ASSENT-DRAFT-001** (SPEC-ASSENT-DRAFT-001): assent_source mandatory at commencement; absence is kernel-rejected.
18. **OBL-ASSENT-DRAFT-002** (SPEC-ASSENT-DRAFT-001): delegated lawmaking validates every change against the prohibited_subjects list.

### Statute section and regulation
19. **ACT-COMPUTER-FIRST-REALM:s29** (cited by 2026-VJS-COURTS-CONSTITUTION-001 at its supplementary-bench provision): the section the courts constitution order already relies on; to be authored consistently with s22(2).
20. **REG-REPOS-HOUSE-001** (cited twice by 2026-VJS-PC-005): Repos House as a non-sovereign two-layer registry (Court Registry + Gazette Clerk; Toolchain + Skills/Agent Registry), certification as deterministic kernel restatement, V1 ministry names demoted to archive aliases.

## Commencement
Item by item, on the Sovereign's specific assent recorded against each instrument's text. Until then the kernel keeps reporting each as `DANGLING_REFERENCE`; the warning is the law working, not noise to suppress.
