# The VJS Gazette

One Gazette, two estates, on the one canonical line (per [2026] VJS-PC 2). Publication is constitutively inert: force comes from the lawpack and the Sovereign's assent, never from being listed here ([2026] REALM-SC 10; REG-GAZETTE-CONTINUITY-001).

> **[Explore the Gazette &rarr;](index.html)** - the law as a constellation: search, the browseable record, the full text of every canon instrument in the reading panel, citation and lineage edges, and the case dockets.
>
> The **[classic reading view](gazette.html)** renders the same record as Law Reports cards. Both are generated from the lawpack by `vjs gazette`; publication is constitutively inert.

---

## The living canon (the current estate)

The computer-first Realm. Live law is the compact lawpack under `lawpack/v2/`, loaded by the deterministic kernel.

**Court jurisprudence of the canon:**
- [2026] VJS-SC 1 - federation, extraction, canonicalisation (Supreme Court of nine).
- [2026] VJS-PC 1 - transition and onboarding.
- [2026] VJS-PC 2 - repository topology and the canonical line.
- [2026] VJS-PC 3 - the s.23 / REALM-SC 10 back-fill.
- [2026] VJS-PC 4 - cite by source (statute by section, case law by its case).
- [2026] VJS-PC 5 - institutions as registries and roles; Repos House.
- [2026] VJS-PC 6 - the canon self-invokes; the Privy Council is its first-instance court. **(latest)**

Opinions: `.vjs/submissions/filed/`. Binding orders: `lawpack/v2/orders/`.

**Primary law:** the Realm Consolidation and Reconciliation Framework Act (assented 2026-06-09; restates V1 law as a whole) and the Computer-First Realm Act, with the raft of subordinate regulations under `lawpack/v2/`.

**Licence:** PolyForm Noncommercial 1.0.0 (`LICENSE`; adoption in `NOTICE.md`).

---

## The honoured archive (read-only)

The first generation, preserved and honoured. On the protected `v1` branch and the immutable `v1-archive-2026-06-09` tag. It binds the canon only by express incorporation; a citation to it is a source edge, not an authority edge.

- **The archive citator:** `Judicature/.justice/INDEX.md` on the `v1` branch.
- **Latest archive authority:** [2026] REALM-SC 10 - the full-court founding settlement (enacted CASE-LAW s.23, the assent floor; back-filled per [2026] VJS-PC 3), above [2026] REALM-PC 24 and the REALM-SC / REALM-PC series.

---

## Trust and verification

Publication is constitutively inert; everything published here is verifiable against the assented record:

- **The provenance colophon** on every page states the lawpack id, its digest, and the commit the data was generated from. Check the digest against `.vjs/lawpack.lock` on the canonical line.
- **Machine-readable copies**: [`gazette-data.json`](gazette-data.json) (the register with treatment, lineage, and docket edges) and [`gazette.xml`](gazette.xml) (Atom; byte-stable when the law is unchanged, so subscribers see no synthetic churn).
- **Documents**: every item opens at `law.html#<id>`. The archive reads as its V1 PDFs, served by the Gazette itself (`archive/pdfs/`; files suffixed `-derived.pdf` are Gazette renderings of the frozen v1-branch sources, marked as such on their face). The canon renders from `gazette-text.js`, which the delivery engine emits with contiguous section numbering: absent ordinals appear as Reserved, the positive drafting convention.
- **Assent**: items show the `assent_source` their law declares; the Gazette never mints one. The schema.org graph on the register page deliberately omits `legislationLegalForce`.
- **The whole pipeline is test-held**: a stale or unfaithful artifact fails the kernel's own suite (`cargo test --workspace`).

---

*This index is the canonical Gazette landing for the one line. An archive source resolves through the citation-map register (`lawpack/v2/provenance/citation-map/`).*
