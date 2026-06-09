# Case-law section -> source resolver (per [2026] VJS-PC 4)

**Status:** provenance register (system-data; archival resolver; not runtime authority). Lodged per [2026] VJS-PC 4 directive D3. Purpose: make every legacy `CASE-LAW s.X` reference in the live V2 corpus self-resolving to its **true source**, so the citation graph stays intact without re-cutting the entrenched V1 document.

**Rule (PC 4):** cite by source. An **SC** (Supreme-Court-made) article resolves to **its case** (neutral citation); a **P** (Sovereign-enacted) constitutional article is genuine sectioned statute and resolves to **its enacting instrument + section** (it legitimately keeps its section, as the entrenched amendment procedure requires). Legacy `LEXBY-*` citations resolve through the Bill 16 old->new map to their `REALM-*` successors.

| CASE-LAW section | kind | true source (resolver) |
|---|---|---|
| s.1 | P / constitutional | Acts of Union 2026 (Bill 1); supremacy inversion by Bill 15 - Sovereign-enacted, cited as the Constitution s.1 |
| s.2 | P / constitutional | Sovereign-enacted constitutional statute - the Constitution s.2 |
| s.3 | P / constitutional | the Constitution s.3; s.3(2)-(7) by Bill 28 giving effect to [2026] REALM-SC 8 |
| s.4 | SC | [2026] REALM-SC 1 (LEXBY-SC 1) - duty of care |
| s.5 | SC | [2026] REALM-SC 1 (LEXBY-SC 1) - standard and breach |
| s.6 | SC | [2026] REALM-SC 1 (LEXBY-SC 1) - remedy (restorative) |
| s.7 | SC | [2026] REALM-SC 1 (LEXBY-SC 1) - no-statute case |
| s.8 | SC | [2026] REALM-SC 1 (LEXBY-SC 1) - one continuous standard |
| s.9 | SC / constitutional | [2026] REALM-SC 1 (LEXBY-SC 1); amended by Bill 30 giving effect to [2026] REALM-PC 17 |
| s.10 | SC / constitutional | [2026] REALM-SC 1 (LEXBY-SC 1); conforming note [2026] REALM-SC 3 (LEXBY-SC 3) |
| s.11 | SC | [2026] REALM-SC 1 (LEXBY-SC 1); citation limb amended by Bill 16 |
| s.12 | SC | [2026] REALM-SC 1 (LEXBY-SC 1) - anti-bloat |
| s.13 | P / constitutional | Sovereign-enacted - the Constitution s.13 (progression) |
| s.14 | P / constitutional | Sovereign-enacted - the Constitution s.14 (jurisdiction) |
| s.15 | SC | [2026] REALM-SC 2 (LEXBY-SC 2) - threshold of the duty |
| s.16 | SC | [2026] REALM-SC 2 (LEXBY-SC 2) - candour scope |
| s.17 | P | Sovereign-enacted - the Constitution s.17 (duty to appeal) |
| s.18 | SC | [2026] REALM-SC 4 (LEXBY-SC 3) - odd benches |
| s.19 | SC | [2026] LEXBY-SC 3 - researched intake / citation gate |
| s.20 | SC | [2026] LEXBY-SC 3 - leapfrog certificate |
| s.21 | SC / declaratory | [2026] REALM-SC 5 (LEXBY-LEGACY-BENCHMARK-SC 1) - divisions over one spine |
| s.22 | SC / declaratory | [2026] REALM-SC 6 (LEXBY-LEGACY-BENCHMARK-SC 2) - court geography / single apex |
| s.23 | SC / constitutional | [2026] REALM-SC 10 - the assent floor (back-filled per [2026] VJS-PC 3) |

**Use.** A reference of the form `CASE-LAW s.N` in the live corpus resolves through this table to its true source. For an SC article, prefer the case citation in new drafting; for a P/constitutional article, the section is the genuine statutory unit and is correct. The principle-20 gate is to fail closed on a `CASE-LAW s.N` reference to an SC-sourced article in newly authored records (a court-made article must be cited by its case).
