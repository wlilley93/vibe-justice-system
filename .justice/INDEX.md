# Caselaw Index (Citator)

The master citator for the realm's central courts. Authoritative index of every judgment handed down at the
Ministry of Justice (Supreme Court, Court of Appeal, Privy Council). Updated whenever a ruling is committed to
`.justice/judgments/`. The deterministic gate (`cdd check-citator`) fails closed on a citation collision or a
filing break. (County Court and High Court Division precedent is indexed in its own `.justice/INDEX.md`; the
universal ledger at `ministry-of-justice/ledger/INDEX.md` unites them all.)

> **Citation scheme (provenance, CASE-LAW s. 11(d) as amended):** the series code encodes the court's authority -
> `REALM-SC` (Supreme Court), `REALM-CA` (Court of Appeal), `REALM-PC` (Privy Council, constitutional first
> instance), `<DIVISION>` (High Court, e.g. `ENG`/`CHAN`), `CC-<repo>` (County Courts). Form: `[YEAR] <CODE> N`.

## How to use it

- Settled? Scan the `Ratio` column. If a binding ratio is on all fours, the VPR 2 fast path disposes on citation.
- Still binding? Read the `Status` column (see Status Key).
- Full text? Open `.justice/judgments/<court>/<file>` (e.g. `.justice/judgments/privy-council/2026-realm-pc-1.md`).

## Rulings

| Citation | Court | Status | Ratio (one line) | Cites |
|----------|-------|--------|------------------|-------|
| [[2026] REALM-SC 1](judgments/supreme-court/2026-realm-sc-1.md) | Supreme | good-law | Breach is the tort of negligence (duty + graded endeavours + remediation only); the system is a unitary realm of parliamentary sovereignty (one global CASE-LAW, jurisdiction-local case law, one apex court). | s. 1-3 (founding); enacts s. 4-12 |
| [[2026] REALM-SC 2](judgments/supreme-court/2026-realm-sc-2.md) | Supreme | good-law | s. 4 imposes a proactive disclosure obligation: Lexby must raise known material risks to the principal's project even when not asked; silence in the face of a known material risk is breach; remedy under s. 6 is disclosure and restoration. Majority 4:1. | s. 3-7, s. 13; extends [2026] REALM-SC 1 |
| [[2026] REALM-SC 3](judgments/supreme-court/2026-realm-sc-3.md) | Supreme | good-law | Six reforms (VARIED 8:1): leapfrog as reviewable executive routing act; two-sided researched intake as the content of the s. 3 case file; apex-only observer; independent leave-judge; non-adjudicating per-turn watchdog; deterministic pre-commit citation-integrity gate. | s. 1-3, s. 11-13, s. 17-18; enacts s. 19, s. 20; confirms REALM-PC 1, REALM-CA 1, REALM-SC 1, REALM-SC 2 |
| [[2026] REALM-CA 1](judgments/court-of-appeal/2026-realm-ca-1.md) | Appeal | good-law | Appeal dismissed unanimously. The s. 8/s. 5 sequencing complaint is not made out; s. 8 cannot be applied without implicitly applying s. 5, so the complaint describes a structural impossibility. REALM-PC 1 confirmed good law. | s. 5, s. 8, s. 12; affirms [2026] REALM-PC 1 |
| [[2026] REALM-PC 1](judgments/privy-council/2026-realm-pc-1.md) | Privy Council | good-law | A governance system is fit for alpha release where the core legal model is coherent, statute + procedure + founding caselaw are committed, constitutional enforcement is in place, and known gaps are disclosed; convenience tooling is not a prerequisite for alpha; deterministic citation numbering is a necessary condition for v1. | s. 4, s. 5, s. 7, s. 8, s. 11(d), s. 14 |
| [[2026] REALM-PC 2](judgments/privy-council/2026-realm-pc-2.md) | Privy Council | good-law | VJS is ready for public outreach under reasonable skill and care provided the README carries a prominently placed known-limitations section first; public outreach is a materially distinct standard from alpha release; coherence and honest disclosure are necessary and sufficient. | s. 4, s. 5, s. 8, s. 11(d); distinguishes [2026] REALM-PC 1 |
| [[2026] REALM-PC 3](judgments/privy-council/2026-realm-pc-3.md) | Privy Council | good-law | Where a feature is the load-bearing differentiator a first-time reader must grasp before adopting, the README must carry it prominently via summary-with-pointer (canonical text stays in plugin/CLAUDE.md; verbatim duplication forbidden). | s. 4, s. 5, s. 8, s. 11(d); extends [2026] REALM-PC 2 |
| [[2026] REALM-PC 4](judgments/privy-council/2026-realm-pc-4.md) | Privy Council | good-law | A deterministic, token-free retrieval index over the committed markdown is a permitted screening device under s. 12 on three cumulative conditions (index-not-replacement; adoption gated on a pre-stated measured trigger and presently FORBIDDEN; cheaper token-free intermediates exhausted first). Until the gate trips the s. 8 duty is to monitor and report, not build. | s. 1, s. 8, s. 11(c), s. 12, s. 15, s. 19(5); extends [2026] REALM-PC 3 |

## Status Key

| Status | Meaning |
|--------|---------|
| **good-law** | Stands and binds; may be cited and applied without qualification. |
| **distinguished** | Good law on its own facts; a later court found the current facts materially different. |
| **overruled** | A higher court (or the same court later) set the ruling aside; it no longer binds. |
| **superseded-by-statute** | The ratio has been replaced by an enacted Act of the Realm; the statute controls. |
| **per-incuriam** | Made in ignorance of binding statute or precedent; void; a fresh sitting is required. |
| **void** | Of no effect (e.g. an improperly constituted bench). |

## Adding a new ruling

1. Mint the next citation deterministically: `cdd next-citation <court> [--division <D>] [--repo <R>]`.
2. Add a row above in ascending order within the series. Fill every column; one tight ratio line.
3. Commit the ruling file and this index together. The pre-commit gate blocks a commit that would leave the
   citator inconsistent (collision, or a ruling file with no row / a row with no file).
