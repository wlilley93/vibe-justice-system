# Repos House register schemas (PUBLIC)

The **schema** (structure) of each Repos House register is public (Bill 27 s. 5A); the **contents** (operational
facts) are private (`_private-registers/`, gitignored; security facts in the MDS private operational registry).
Every register is a **derived projection**: pointer-only, deterministically rebuildable from the committed
markdown, regenerated in lockstep, bearing `CONFORMANT-PROJECTION` / `DERIVED-INDEX` only (never
`AUTHORITATIVE-SOURCE`). A status field is a **read** of the gate, never a **write** by the registrar
([2026] REALM-SI 6; [2026] REALM-PC 4, REALM-PC 14; Bill 20).

## Incorporation register

One row per jurisdiction-repo. Each field is a pointer/derived value, never a conferred status.

| Field | Meaning |
|---|---|
| `repo` | the `CC-<repo>` handle (the County Court at the repo) |
| `seat` | seat evidence: the `.justice/` node applying the one CASE-LAW/VPR + a `CC-<repo>` series on the single citator (CASE-LAW s. 22) |
| `substance` | the result of the existing gate's content scan (system-data-only, no personal/operational facts; Bill 27 s. 7) - read, not decided |
| `conformance` | the Bill 20 integrity-chain / s. 19(5) gate result + git presence (zero-token); `CONFORMANT-PROJECTION` / `DERIVED-INDEX` / `QUARANTINED` |
| `formed` | derived boolean: all three of (seat, substance, conformance) hold (the [2026] REALM-PC 14 R1 certificate) - a read-out, never a grant |
| `last-decider` | for a human/office-initiated trust-class change only: the named decider + reasoned-act pointer + judicial-review route (Bill 20 s. 11); restorative-only; tombstone on de-certification |

## Project register

One row per repository/project in the realm.

| Field | Meaning | Visibility |
|---|---|---|
| `project` | the repository/project identifier | public |
| `kind` | system-repo / jurisdiction-repo (County Court) / operational | public |
| `infrastructure` | local / remote / none (at the level of principle) | schema public; **facts private** |
| `owner-dept` | the MBES department that holds it | public |

## Skills registry

One row per submitted skill. The registry is the intake + provenance index; loadability is decided at the runtime
(the [2026] REALM-SI 5 s. 14 verify-against-the-MDS-trust-root step), not here.

| Field | Meaning | Visibility |
|---|---|---|
| `skill` | the skill identifier | public |
| `submitted-by` | the submitting agent (the self-duty intake) | schema public; provenance private |
| `inclusion-audit` | the MDS audit-event pointer (performed / pending) | public state, private detail |
| `signature` | signed / unsigned under the MDS-held audited delegation; `signer`/`key-custody`/`revocation` live in the **MDS** private registry | public state, private detail |
| `loadable` | **derived**: signed-and-valid at the runtime gate (read-out); unsigned = registrable-but-unloadable | public |

## Egress register

One row per repository.

| Field | Meaning | Visibility |
|---|---|---|
| `repo` | the repository | public |
| `egress-policy` | which **MDS** allowlist policy the repo runs under (a provenance pointer; the allowlist entries + resolver config live in the MDS private registry, [2026] REALM-SI 5 s. 15) | schema public; **facts private** |
| `mode` | fail-closed allowlist (default) + audited break-glass; soft watchdog for the high-volume reversible class | public |

## Governance

- **Maintained by** MBES (the host), as engineering it owns ([2026] REALM-SI 6 s. 13; Bill 27 s. 5B). The
  security-owned gates (skill signing, egress) are MDS's; Repos House indexes them.
- **Rebuild:** each register is deterministically rebuilt from the committed tree + git (zero model tokens);
  trust/formation status is a read of the existing gate, never a registrar write.
- **Change governance:** schema/principle changes go MBES policy arm -> Standing Committee draft -> Sovereign
  enact (Bill 29); a durable-principle change amends [2026] REALM-SI 6.

**UP:** [`README.md`](README.md) (the Repos House charter).
