# Statutory Instruments register

Derived, pointer-only (see [`README.md`](README.md)). The committed instrument markdown is canonical.

| Citation | Title | Parent (enabling) | Made by | Status | Instrument |
|----------|-------|-------------------|---------|--------|------------|
| [2026] REALM-SI 1 (under Bill 21) | The Security and Integrity (Server Estate) Instrument 2026 | Bill 21 s. 16 (Security and Integrity Act 2026) | Standing Committee | made | [2026-realm-si-1-security-and-integrity-server-estate.md](2026-realm-si-1-security-and-integrity-server-estate.md) |
| [2026] REALM-SI 2 (under Bill 5, Bill 14, Bill 26 and Bill 27) | The Judgment Rendering and Lodgement Instrument 2026 | Bill 5 s. 18 (Ministries and Offices Act 2026), parent authority MBES; read with Bill 14; Bill 16 s. 12 substantive | Standing Committee | made | [2026-realm-si-2-judgment-rendering-and-lodgement.md](2026-realm-si-2-judgment-rendering-and-lodgement.md) |
| [2026] REALM-SI 3 (under Bill 5, Bill 14, Bill 26 and Bill 27) | The Refactoring Suite Instrument 2026 | Bill 5 s. 18 (Ministries and Offices Act 2026), parent authority MBES (owns the Suite, Bill 27 s. 5B); read with Bill 14 | Standing Committee | made | [2026-realm-si-3-refactoring-suite.md](2026-realm-si-3-refactoring-suite.md) |
| [2026] REALM-SI 4 (under Bill 14, Bill 21 and Bill 27) | The Security Suite Instrument 2026 | Bill 21 s. 16 (Security and Integrity Act 2026), parent authority MDS (owns the Suite, Bill 27 s. 5B); read with Bill 14; aligned with [2026] REALM-SI 1 | Standing Committee | made | [2026-realm-si-4-security-suite.md](2026-realm-si-4-security-suite.md) |
| [2026] REALM-SI 5 (under Bill 14, Bill 18, Bill 21 and Bill 27) | The Agent Security Instrument 2026 | Bill 21 s. 16 (Security and Integrity Act 2026), parent authority MDS; read with Bill 14; aligned with Bill 18 and [2026] REALM-SI 1; conforms to [2026] REALM-PC 13 | Standing Committee | made | [2026-realm-si-5-agent-security.md](2026-realm-si-5-agent-security.md) |

## Status key (Bill 16 s. 15(1A))

| Status | Meaning |
|--------|---------|
| **made** | drafted and issued by the Standing Committee; awaiting commencement (in the objection window). |
| **in-force** | commenced (objection window lapsed without valid objection, or affirmed). |
| **amended** | text altered by an amending instrument (Bill 14 s. 27), still in force as amended. |
| **revoked** | removed from the subordinate estate by a later instrument or Sovereign override. |
| **spent** | operation exhausted; of no continuing effect. |

## Adding an instrument

1. The Standing Committee makes the instrument (Bill 26 s. 15) in exercise of a parent office's enabling
   power (Bill 26 s. 14), opening with the Form C enabling recital so the engine derives the parent tag.
2. Add a row above; the citation is `[YEAR] REALM-SI N` minted by `cdd next-citation si`, shown with the
   derived `(under Bill NN)` tag.
3. Commit the instrument file and this index together.
