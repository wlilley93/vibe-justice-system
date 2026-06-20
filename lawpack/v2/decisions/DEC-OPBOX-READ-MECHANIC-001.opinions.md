# [2026] VJS-DEC 18 — First Instance bench opinions (the reads-direct mechanic)

Odd bench of 3, symmetric case file, no access to advocate preference. Recorded in full (the judgment record
behind the compact ratio in DEC-ACMECO-READ-MECHANIC-001.yaml).

## The fork
Under [2026] VJS-DEC 15 (reads-direct, writes-via-verbs, no sync, projections deleted) on ONE shared
Postgres, what is the binding MECHANIC for the Next.js/Prisma frontend to read kernel-owned data? Options:
A call-site verb-dispatch reads · B kernel-owned DB view · C `$queryRaw` per path · D un-`@@ignore` complete
read-only mirror model.

## Judge A — ruled OPTION A (call-site read modules dispatching kernel read verbs)
Ratio: kernel-owned data is read through the kernel's read API, mapped in one per-domain `*-kernel.ts` module,
cross-joining Prisma only for frontend-owned fields; the `@@ignore` boundary stays armed; a view is a sync in
disguise (B rejected), un-`@@ignore` forfeits the safeguard (D rejected), `$queryRaw` is a module-scoped
fallback. Conditions: exhaustiveness grep-gate before dropping a projection; per-domain module not per-call;
tsx fidelity proof; writes stay verb-only. **Noted as MINORITY / per incuriam**: A weighted `@@ignore` as
load-bearing for db-push safety (Judge C disproved this) and read it as DEC-15-conformant, but DEC-15's
`must_not: add_http_read_verbs_for_shared_db_reads` (Judge B) makes verb-dispatch reads non-conformant for the
steady state. A's value survives as the TRANSITION mechanic.

## Judge B — ruled OPTION D (un-`@@ignore` complete read-only model, direct Prisma read)
Dispositive finding: DEC-15's text already chose "reads via Prisma/SQL direct" AND its must_not forbids
HTTP read verbs for shared-DB reads — so A is out at the constitutional level; the live fork is B vs D.
D is the lowest-blast typed path (flip `@@ignore`, `prisma generate`, reads just work), is DEC-15's own
first-named mechanic, and is drop-safe when the mirror is a faithful full-column reflection (the regen script
already produces this). Conditions: regenerate + `prisma validate` before un-ignoring (the mirror is currently
STALE — `regulator_comment` still pre-0081/0082); additive-only db push; read-only by convention enforced by
the write-site audit; tsx read-proof on the isolated `:8088` kernel. B is the carve-out for column-rename /
cross-table shapes; C for inexpressible reads; A prohibited for steady-state shared-DB reads.

## Judge C — ruled HYBRID (D default · B for fidelity divergence · A transition/fallback · C escape hatch)
Decisive empirical finding (isolated pg18 / Prisma 6.19.3): **Blocker A is FALSE as framed.** `@@ignore` has
zero effect on the `db push` diff; a COMPLETE model is "in sync" with or without it; an INCOMPLETE model
proposes the drop (guard-blocked without `--accept-data-loss`). So `@@ignore` governs Client visibility only;
drop-safety = mirror completeness. `db push` never manages VIEWS at all (B is structurally immune). Ruling:
default D for fidelity-aligned domains; **mandatory B (kernel-owned, non-materialized view) for money** (kernel
`BigInt` minor-units vs the UI's float — a naive direct read returns the wrong unit/shape); A only as the
pre-RLS/pre-fold transition and the split-DB reopen path; C a rare escape hatch.
Binding conditions (load-bearing): **C1** CI mirror-completeness gate; **C2** a Prisma `$extends` write-guard
that throws on writes to kernel-owned models (un-ignoring re-exposes `.create/.update/.delete`); **C3** RLS
live before any domain switches verb-read→direct-read (else app-WHERE scoping, DEC-15 rule 3 forbids); **C4**
fold the data (`matters`→`matter`) + delete the projection before redirecting reads; **C5** views kernel-owned
+ non-materialized. Reopen: split DBs → A/CQRS binding; Prisma drops `views` support → D-everywhere + mappers.

## Disposition (binding ratio in the .yaml)
Majority B+C → **default D, escalate to B for fidelity divergence (mandatory money), A transitional, C escape
hatch**, gated on C1–C5. The Blocker-A premise of the convening question is corrected: safety rests on
completeness + Prisma's hands-off view handling, not `@@ignore`. Consequence for the burn-down: land RLS +
the completeness gate + the write-guard FIRST; then domains migrate cheaply via D (reads stop being a
sprawling per-call-site rewrite). Verb-dispatch reads (regulator_comment helper, /invoices|/sign/kernel) are
transitional debt to convert to D/B.
