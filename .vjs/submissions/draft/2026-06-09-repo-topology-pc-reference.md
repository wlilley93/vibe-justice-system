# V2 Privy Council Reference: Repository Topology and the Canonical Line

**Date:** 2026-06-09
**Filed by:** Lexby (advocate, advisor, engineer; registrar for the bench), on the Principal's reference
**Court:** V2 Privy Council (routing / public-boundary / governance first instance, bench of 3; 2026-VJS-COURTS-CONSTITUTION-001)
**Companions:** the symmetric case file of 2026-06-09; [2026] VJS-SC 1 (canonicalisation Q6; non-disturbance) and order 2026-VJS-SC-001; [2026] VJS-PC 1
**Status:** public; system-data only; not a judgment, order, or law.

> Lexby files and will reduce the bench's already-formed decision to the record as registrar. The bench has no access to Lexby's preference (ACT s.15; CASE-LAW s.19(1)). No repository is moved, renamed, pushed, or re-licensed by this reference; those are the Principal's warranted acts.

## Why now

V2 has commenced; the canon is the unqualified Realm and "V1" is the Archive/Gazette estate label ([2026] VJS-SC 1, Q6). But the canon and the archive currently sit in **two GitHub repositories** that do not yet reflect that settlement:
- `agent-universe-v2` (the live runtime/kernel; `origin/main` at an old draft; the work on a feature branch);
- `vibe-justice-system` (the public **Gazette** + Archive; default branch `public-vjs-canonical-preview`; also `master`).

The Principal asks the Council to settle, as a routing/public-boundary/governance matter, **where the canon and the V1 archive estate should live across the repositories and branches**, consistent with canonicalisation, the single Gazette of two estates (Bill 32 s.16), and the SC-1 non-disturbance rule.

## The question

> How should the canonical Realm and the V1 Archive/Gazette estate be arranged across the GitHub repositories and branches, such that (i) there is one canonical line, (ii) the single Gazette carries both estates with the citation graph intact, (iii) V1/REALM cases remain referenced only as archival source law (not live authority), and (iv) the SC-1 non-disturbance rule is honoured (digests, the lawpack lock, citations, and record IDs immutable; governed rename, never a mass edit)?

Concrete sub-questions the Principal puts:
1. Should the canon be consolidated into the **existing `vibe-justice-system` repo** (reusing the established repo, README, and public Gazette Pages site), or into `agent-universe-v2`, or should the two remain **separate** (runtime vs Gazette)?
2. Should the canonical current Realm live on **`main`**, with the V1 archive preserved as a **`v1` branch** (the Archive estate)?
3. Should the **single Gazette live on the canonical line and include V1 law** (as the archive estate), so the citation graph from transitioning V2 law back to its REALM source resolves in one place?
4. How is the non-disturbance rule honoured through any repo move / branch reorganisation (the lawpack lock `4d2639cc...`, the assented digest `8e1d3f51...`, the `[YEAR] VJS-ACT N` and `REALM-*` citations, and embedded `lawpack/...` paths must not be broken)?

## Proposed disposition (Lexby's case, to be tested against the contrary case)

Consolidate onto one canonical line, reusing the established public repo, with two estates on one Gazette:
- the **canon lives on `main`** of the established public repo (so the README, the Gazette Pages site, and the public presence carry forward); the **V1 archive estate is preserved as a `v1` branch** (read-only, the Archive);
- the **single Gazette is built on the canonical line and includes the V1 archive law**, so the graph from V2 transitioning law to its REALM source resolves in one place;
- **REALM/V1 cases are archival source law only**: live V2 authority binds solely by incorporation (Bill 32 s.8/s.9); transitioning V2 instruments *cite* REALM cases as their source (keeping the graph intact) without making them live authority;
- the move is a **governed migration, not a mass edit**: digests, the lock, citations, and record IDs are preserved; paths move only behind a compatibility/redirect or a freshly-recorded lock under a commencement addendum (Harlan J, SC-1).

## What is for the legislature, not this Council

The detailed codification is for the Standing Committee, which the Principal directs to draft a **Consolidation and Reconciliation Framework Act** (primary, Sovereign-assented) parenting a raft of subordinate regulations: Federation-Coordination, Migration-and-Incorporation (carrying the **archival-status / citation-graph-continuity** rule for REALM cases), Transition-Court, Canonicalisation, Gazette-Continuity, and the adoption of **VJS's own AGPL licence**. This Council settles only the repository-topology principle; it directs the rest to the legislature and notes the AGPL adoption is the Principal's prerogative as copyright holder, recorded by the Act.

## Requested direction

Settle sub-questions 1-4 on the topology principle; confirm REALM cases are archival source law only with the graph intact; require any repo move to honour non-disturbance as a governed migration; and direct the legislature to codify the outcome in the Framework Act + raft. Nothing is moved, renamed, pushed, or re-licensed by this reference.

## Data boundary

System-data only. No secrets, credentials, private hostnames, client facts, personal facts, or unredacted evidence.
