# Citation map - the LEXBY -> provenance-scheme reconstitution

Authoritative old -> new citation map for the realm-wide reconstitution settled in session
2026-06-05: every ruling re-cited under the provenance neutral-citation scheme (SPEC-LAW
s. 11(d) as amended), reclassified onto the court geography (Supreme / Court of Appeal /
Privy Council / High Court Division / County Court), and re-rendered as full-bleed cream A4.

## The scheme (s. 11(d), amended)

| Court | Provenance | Citation form | Mark |
|---|---|---|---|
| Supreme Court | realm apex; sole enactor | `[YEAR] REALM-SC n` | crest |
| Court of Appeal | realm appellate | `[YEAR] REALM-CA n` | crest |
| Privy Council | realm constitutional / governance, the CENTRAL first instance (one court, at the MoJ / realm level; climbs by ordinary progression, leapfrog only by the Principal's certificate per s. 20; [2026] REALM-PC 15) | `[YEAR] REALM-PC n` | wordmark |
| High Court | by Division | `[YEAR] <DIVISION> n` (ENG, CHAN) | crest + Division/List |
| County Court | by repo | `[YEAR] CC-<REPO> n` | wordmark |

## Realm (`.justice/`) - clean, unambiguous

| Old | New | Level | Status |
|---|---|---|---|
| [2026] LEXBY-SC 1 | **[2026] REALM-SC 1** | Supreme Court | good-law (founding) |
| [2026] LEXBY-SC 2 | **[2026] REALM-SC 2** | Supreme Court | good-law |
| [2026] LEXBY-SC 3 | **[2026] REALM-SC 3** | Supreme Court | good-law |
| [2026] LEXBY-CA 1 | **[2026] REALM-CA 1** | Court of Appeal | good-law |
| [2026] LEXBY-FI 1 | **[2026] REALM-PC 1** | Privy Council | good-law |
| [2026] LEXBY-FI 2 | **[2026] REALM-PC 2** | Privy Council | good-law |
| [2026] LEXBY-FI 3 | **[2026] REALM-PC 3** | Privy Council | good-law |
| [2026] LEXBY-FI 4 | **[2026] REALM-PC 4** | Privy Council | good-law |

## County Court at acmeco - flat CC-ACMECO series (dissolves the old collisions)

All acmeco matters become County Court at acmeco local precedent; the previous-system tiers
(its own FI/CA/SC) are historical and collapse into one local series.

| Old (acmeco) | New | Notes |
|---|---|---|
| [2026] LEXBY-FI 1..8 | **[2026] CC-ACMECO 1..8** | local first-instance matters |
| [2026] LEXBY-CA 1 | **[2026] CC-ACMECO 9** | was a per-repo appeal (previous system) |
| [2026] LEXBY-CA 2 | **[2026] CC-ACMECO 10** | consolidation-as-sequencing ruling |
| [2026] LEXBY-SC 2 | **[2026] CC-ACMECO 11** | was a per-repo "supreme" matter (previous system) |

## Legacy benchmark-labs - decisive calls (FLAGGED; source has aliasing + void/superseded entries)

The legacy-benchmark rulings are constitutional/governance matters about the system itself, so
they merge into the central REALM series rather than a Chancery commercial list. Source
data is inconsistent (the legacy-benchmark INDEX aliases the same ruling as both `SC 3` and
`LEGACY-BENCHMARK-SC 3`; `LEGACY-BENCHMARK-CA 1` is void ab initio; `LEGACY-BENCHMARK-INC 1` is superseded). Mapping by
decisive call, statuses carried forward; **open to correction**.

| Old (legacy-benchmark) | New | Level | Status |
|---|---|---|---|
| LEGACY-BENCHMARK-SC 3 (bench constitution + anti-cheat; s. 18) | **[2026] REALM-SC 4** | Supreme Court | good-law |
| LEGACY-BENCHMARK-SC-DC 1 / LEGACY-BENCHMARK-SC 1 (divisions over one spine; s. 21) | **[2026] REALM-SC 5** | Supreme Court | good-law |
| LEGACY-BENCHMARK-SC 2 (court geography; s. 22) | **[2026] REALM-SC 6** | Supreme Court | good-law |
| LEGACY-BENCHMARK-CA 1 (appeal of INC 1) | **[2026] REALM-CA 2** | Court of Appeal | VOID ab initio |
| LEGACY-BENCHMARK-INC 1 (inclusion / framework-seeding) | **[2026] REALM-PC 5** | Privy Council | superseded |
| LEGACY-BENCHMARK-FI 1 (harvesting) | **[2026] REALM-PC 6** | Privy Council | good-law |
| LEGACY-BENCHMARK-FI 2 (admissibility) | **[2026] REALM-PC 7** | Privy Council | good-law |
| LEGACY-BENCHMARK-FI 3 (domain-courts vs one spine) | **[2026] REALM-PC 8** | Privy Council | good-law |
| LEGACY-BENCHMARK-FI 4 (practitioner treatise) | **[2026] REALM-PC 9** | Privy Council | good-law |

> Flag: the legacy-benchmark FI rulings are classified Privy Council (constitutional first instance)
> on the view that they concern the system's own constitution/methodology. If any (e.g. the
> treatise, the harvesting method) read better as High Court administrative matters, re-map
> to `[2026] CHAN n` / the relevant Division.

## Propagation checklist (per stage)

For each ruling: rename the judgment `.md`, rewrite its in-text citation + cross-references,
re-render the PDF (full cream, correct level), update the local `.justice/INDEX.md`, the
universal ledger, and any SPEC-LAW / VPR / README / signpost references. SPEC-LAW s. 11(d)
and the ledger header are amended once, centrally.
