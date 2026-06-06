# Master Digital Estate Registry - schema and principle (PUBLIC)

This document is **public**: it states the **principle** that the realm maintains a digital estate registry,
and the **schema** (structure) of that registry, so any repo user knows it exists and what it records. The
registry's **contents** (the actual hosts, addresses, configuration, and known gaps) are **private**, held by
the Ministry of Data Security in `_private-estate/` (gitignored), per the principles-at-law / facts-in-the-
registry split (Bill 27; `[2026] REALM-SI 1`).

## Principle

The Ministry of Data Security maintains a single canonical inventory of the realm's server estate, and a
per-repository server-config mapping, and is responsible for implementing and auditing the security baseline
against them. The public law (the principles SI `[2026] REALM-SI 1`) states the security principles; this
registry holds the facts those principles are applied to. The law never carries the facts; the registry never
carries the law.

## Schema: the master digital estate registry

Each entry (one per host in the estate) records:

| Field | Meaning |
|---|---|
| `host` | the host identifier (logical name) |
| `role` | local / public-production / other |
| `network` | interfaces and the access vector (private / VPN / public), at the level of principle |
| `services` | the services the host runs (by role, not secret detail) |
| `security-baseline` | the applicable principles from `[2026] REALM-SI 1` |
| `conformance` | last deterministic conformance-check result + date |
| `known-gaps` | identified gaps and their remediation status (reviewed at least annually) |
| `audit` | last audit date + the audit-chain reference (Bill 8) |

## Schema: the repo -> server-config list

Each entry (one per repository in the realm) records:

| Field | Meaning |
|---|---|
| `repo` | the repository (its citation locale, e.g. `CC-<repo>`) |
| `infrastructure` | what it runs on (local, remote/cloud, or none) |
| `security-baseline` | the expected baseline for that deployment |
| `egress` | its egress posture (whether it publishes externally; ties to Repos House egress licensing) |

## Governance

- **Maintained and audited by** the Ministry of Data Security; updated when a host or a repository's
  infrastructure changes, and reviewed at least annually.
- **Private contents:** the populated registries live in `_private-estate/` (gitignored). They are operational
  data, never part of the public record (Bill 27 system-data-only rule).
- **Enforcement:** the machine-checkable parts of the baseline run on the deterministic fail-closed gate with an
  audited break-glass (Bill 13 s. 5A; `[2026] REALM-SI 1`); never punitive.

**UP:** [`README.md`](README.md) (the Ministry charter).
