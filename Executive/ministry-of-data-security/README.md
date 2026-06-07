# Ministry of Data Security (MDS)

The realm's security ministry, in the Executive branch. Constituted by the Security and Integrity Act 2026
(Bill 21) and renamed from the Ministry of Security and Integrity / Ministry of Defence by the VJS
(Constitution and Machinery) Act 2026 (Bill 27). This charter is **public** (Bill 27 public-mechanics rule):
it states the Ministry's mechanics; the operational **facts** it holds are private.

## Mechanics (what the Ministry does)

- **Owns the security suite** (Bill 27): the operative content of `Judicature/.justice/suites/security.md`,
  legalised and amended as a statutory instrument under the Bill 26 framework, in alignment with the security
  laws (Bill 21, the principles SI `[2026] REALM-SI 1`).
- **Implements and audits security** across the realm's server estate (Bill 21; `[2026] REALM-SI 1`): it is
  ultimately responsible for the security implementation and the ongoing audit.
- **Maintains the estate registries** (the *facts*, private): a master digital estate registry and a
  repo -> server-config list. The **principle and schema** of these registries are public (see
  [`estate-registry-schema.md`](estate-registry-schema.md)); the **contents** are private.
- **Audits skill inclusion** (the Skills Registry, forthcoming): agent skills are executable supply chain, so
  the Ministry audits a skill before it is admitted to the global registry.
- **Develops security policy** through its policy arm (Ministry policy -> Standing Committee drafting), e.g. the
  agent-security policy paper (prompt injection and related risks for public-repo agents).

## Registers it holds

| Register | Visibility | Where |
|---|---|---|
| Security principles (the law) | **public** | `[2026] REALM-SI 1` (Legislature/statutes/instruments/) |
| Estate-registry **schema / principle** | **public** | [`estate-registry-schema.md`](estate-registry-schema.md) |
| Master digital estate registry (facts) | **private** | `_private-estate/` (gitignored) |
| Repo -> server-config list (facts) | **private** | `_private-estate/` (gitignored) |
| Security policy papers | **private** where they cite facts | `_private-estate/policy/` (gitignored) |

## The principle: principles at law, facts in the registry

Per the Founder's rule and Bill 27: durable security **principles** are enshrined at law (the principles SI);
the volatile, sensitive **facts** (IPs, ports, configs, known gaps) live only in the private estate registry,
which the Ministry maintains and audits. The public law never carries the facts; the registry never carries the
law. This keeps the public VJS safe to publish while the estate is fully governed.

**UP:** [`../`](../) (the Executive branch) · the realm: [`../../README.md`](../../README.md).
**The security law:** [`../../Legislature/statutes/instruments/`](../../Legislature/statutes/instruments/) ·
[`../../Legislature/legislature/bills/21-security-and-integrity-act-2026.md`](../../Legislature/legislature/bills/21-security-and-integrity-act-2026.md).
