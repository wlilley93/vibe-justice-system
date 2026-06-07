# Repos House - the registrar of repositories (Public Mechanics Charter)

Repos House is the realm's **registrar of repositories**: a set of **derived, pointer-only registers**, hosted
inside the Ministry of Business, Engineering and Skills (MBES), that **records** the deterministic facts of the
realm's repositories and **confers nothing**. It is **not** a governance organ, court, ministry, apex, or
citation series of its own; it **mints no authority, it indexes provenance** (Bill 20 s. 10(3)).

> **Legal status.** The law is **The Repositories House Instrument 2026** ([2026] REALM-SI 6, under Bill 5 s.
> 18), made by the Standing Committee, hosted by MBES, indexing the MDS-operated gates of [2026] REALM-SI 5. This
> charter is the **public mechanics** of Repos House (Bill 27 s. 5A): the principle and schema of each register.
> The Instrument governs; this charter describes. The operational **facts** are private.

## The one load-bearing rule: declaratory, never constitutive

Repos House **records facts the deterministic chain already established; it decides nothing** (Bill 20 s. 7, s.
8, s. 10; [2026] REALM-PC 14). There is no `create` / `grant` / `approve` / `upgrade` / `downgrade` / `withhold`
/ `revoke` verb in any register, and no field whose value an operator sets to make a repo a court, a skill
trusted, or an egress authorised. Every state change in a register is a faithful **read-out** of a change in the
deterministic facts (the tree changed; the gate result flipped), never an opinion Repos House formed.

Every register is a derived, pointer-only projection ([2026] REALM-PC 4), bearing **CONFORMANT-PROJECTION** or
**DERIVED-INDEX** only, **never AUTHORITATIVE-SOURCE** (reserved to the committed markdown; the relocation
question is reserved to the Supreme Court, Bill 20 s. 14).

## The four registers

| Register | What it records | Visibility | Where the facts live |
|---|---|---|---|
| **Incorporation register** | that a repo is a validly-formed County Court, by recording the [2026] REALM-PC 14 three-condition fact (substance + seat + conformance), read out of gate-plus-git; never conferred | mechanics + schema **public** | the per-repo evidence is a system-data pointer; private facts (if any) gitignored |
| **Project register** | a derived, pointer-only index over the realm's repositories/projects | schema **public** | per-project hosts/config/spend **private** (gitignored) |
| **Skills registry** | the agent self-duty intake surface + provenance index of submitted skills and their inclusion-audit/signing state | schema **public** | per-agent provenance + the signer roster/key custody in the **MDS** private registry |
| **Egress register** | which MDS allowlist policy each repo runs under (provenance pointer) | schema **public** | allowlist entries + resolver config in the **MDS** private registry |

## Incorporation = registration of a deterministic fact (REALM-PC 14)

A County Court exists from the moment the three [2026] REALM-PC 14 conditions hold over its committed tree:
**(a) substance** (system-data-only tracked record, no personal/operational facts; Bill 27 s. 7, Bill 22), **(b)
seat** (a genuine jurisdiction-repo seated as a local seat of the one judiciary, `CC-<repo>` on the single
citator; CASE-LAW s. 22), **(c) conformance** (the Bill 20 integrity chain holds; not QUARANTINED). Those three
**are** the certificate of incorporation. Repos House issues no certificate that does legal work; at most it
renders a derived attestation of a fact gate-plus-git already established.

The substance check polices **content, not the `.gitignore`**: incorporation is never keyed on the presence of a
`.gitignore`, a `.justice/` directory, or any single root token in isolation (form-over-substance, void under
CASE-LAW s. 22(4); [2026] REALM-PC 14). The `.gitignore` is the presumptive **mechanism** whose absence-where-
private-data-is-held is a red flag, never the trigger. The check reuses the **existing** commit gate (no second
scanner).

## The skills registry: self-duty + MDS audit gate

- **Every agent has a continuing self-duty** to submit a new skill for inclusion (the duty of care, CASE-LAW s.
  4-8). The duty lives on the agent; Repos House is only the intake surface. Non-submission is remedied
  restoratively (submit it), never punished.
- **The inclusion audit is the [2026] REALM-SI 5 s. 14 fail-closed signing gate**, operated by MDS: a skill is
  **registrable-as-submitted but unloadable** until it is MDS-audited and signed under the MDS-held revocable
  trust-root audited delegation. The load gate sits at the **runtime** (verify-signature-against-the-MDS-trust-
  root), not at the registrar. The gate carries an audited break-glass; refusal to include is a capability gate,
  not a sanction. **Repos House holds no key and is not the trust root.**

## Egress licensing: indexing, not conferral

"Licensing" is **not** a Repos House conferral (possession of an egress capability is not authority; Bill 18 s.
3). It is the [2026] REALM-SI 5 s. 15 regime, operated by MDS (fail-closed allowlist + audited break-glass + a
complementary soft watchdog for the high-volume class; a break-glass-less hard block is void). Repos House only
**records which MDS allowlist policy each repo runs under**.

## MDS is operate-audit-refer only

MDS performs the inclusion audit and operates the skill-signing and egress gates, but may not adjudicate, find a
breach as binding fact, score, certify, or sanction (Bill 21 s. 5-6). A suspected breach is **referred** to the
single judiciary (MDS as a party, never the bench). Repos House houses no merits-adjudication or sanction surface.

## Public mechanics, private facts

This charter and each register's **schema** are public (Bill 27 s. 5A). The operational **facts** are private,
in `_private-registers/` (gitignored), with the security-owned facts in the MDS private operational registry.
The public realm holds system data only (Bill 27 s. 7; [2026] REALM-PC 14).

**UP:** [`../README.md`](../README.md) (MBES) · the realm: [`../../../README.md`](../../../README.md).
**The law:** [`../../../Legislature/statutes/instruments/2026-realm-si-6-repositories-house.md`](../../../Legislature/statutes/instruments/2026-realm-si-6-repositories-house.md) ·
the security gates it indexes: [`../../../Legislature/statutes/instruments/2026-realm-si-5-agent-security.md`](../../../Legislature/statutes/instruments/2026-realm-si-5-agent-security.md).
