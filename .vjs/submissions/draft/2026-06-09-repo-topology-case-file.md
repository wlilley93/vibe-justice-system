# Symmetric Case File: Repository Topology and the Canonical Line

**For:** the V2 Privy Council (bench of 3)
**Filed by:** Lexby as s.19(1) registrar of the two-sided intake (NOT advocate; both sides equal; bench has no access to Lexby's preference)
**Companion to:** the repo-topology reference of 2026-06-09
**Status:** public; system-data only; intake material, not a judgment.

> Two-sided by design. The bench decides. No repo is moved, pushed, or re-licensed by this file.

## Part 1 - Agreed facts

1. Canonicalisation is settled ([2026] VJS-SC 1, Q6): the commenced successor is the unqualified canon; "V1" survives only as the Archive/Gazette estate label; de-naming is prospective and **strictly non-disturbing** (assented digest `8e1d3f51...`, lawpack lock `4d2639cc...`, citations `[YEAR] VJS-ACT N` and `REALM-*`, and record IDs are immutable; governed rename, never a mass edit).
2. Bill 32 s.16: one Gazette, two estates (V1 Archive, V2 Current); publication creates no runtime force; estate boundary operative by substance.
3. Two repos exist: `vibe-justice-system` (Rust runtime/kernel; `origin/main` at an old draft; work on a branch) and `vibe-justice-system` (the public **Gazette** + Archive; Node/Pages build; default `public-vjs-canonical-preview`, also `master`).
4. V1/REALM cases bind V2 only by incorporation (s.8/s.9); they are otherwise archival/persuasive. The Principal wants them **referenced as archival source law by transitioning V2 law**, so the citation graph stays intact.
5. Operability finding (Harlan J, SC-1): the lawpack lock is a tree digest; many invariants embed literal `lawpack/...` paths in `scope.paths`; a blind move/rename breaks the lock and un-scopes invariants fail-open.

## Part 2 - The question

How should the canon and the V1 Archive estate be arranged across repos/branches, given canonicalisation, the single Gazette of two estates, the graph-intact requirement, and non-disturbance?

## Part 3 - The options

**T1 - Consolidate onto the established public repo (`vibe-justice-system`).** The canon lives on `main` of the established repo (reusing its README, public presence, and Gazette Pages site); the V1 archive becomes a read-only `v1` branch (the Archive estate); the single Gazette is built on the canonical line and includes the V1 archive law, so the graph from transitioning V2 law to its REALM source resolves in one repo. *For:* one canonical line; reuses the public Gazette site and established name; the single-Gazette-two-estates model maps naturally to one repo with `main` (Current) and `v1` (Archive); drops the "v2" in the runtime repo name (canonicalisation). *Against:* the runtime is Rust and the Gazette is a Node/Pages build - merging two stacks; the migration must move the lawpack into the established repo without breaking the lock/paths (a real, but governable, non-disturbance task); history reconciliation across two repos.

**T2 - Consolidate onto `vibe-justice-system` (renamed canonically).** Pull the Gazette into the runtime repo. *For:* the runtime/kernel is the live canon; keep its history. *Against:* loses the established public Gazette site/README/presence; "vibe-justice-system" itself carries the spent "v2" ordinal; bigger public-facing disruption.

**T3 - Keep separate.** `vibe-justice-system` = runtime canon; `vibe-justice-system` = Gazette/Archive; cross-linked. *For:* least disruption; clean separation of runtime vs public Gazette; no lock/path migration. *Against:* two canonical-ish lines; the single-Gazette graph spans two repos; the spent "v2" name persists; cross-estate references are split.

## Part 4 - Sub-questions

- **main = canon, v1 = archive branch?** A `main`/`v1` split cleanly expresses "Current estate / Archive estate" in one repo and keeps the graph in one place; but a branch is a weak archival boundary (branches move) - is a branch the right vehicle for an *estate*, or should the archive be a directory/tag, or a preserved read-only branch with protection?
- **Gazette on the canonical line including V1 law?** s.16 supports one Gazette with both estates; including V1 law on the canonical line keeps the graph intact and is where transitioning law cites its source. Risk: must not let V1 archive law read as live authority (estate labelling operative-by-substance; the INV-NO-V1-GAP-FILLER discipline).
- **Graph intact / REALM archival-only:** transitioning V2 instruments cite REALM cases as source law (edges resolve) while those cases remain archival, not live (s.8/s.9). This is the right way to keep the graph whole without reviving V1.
- **Non-disturbance:** any move is a governed migration - lock, digests, citations, IDs preserved; paths moved only behind a compatibility/redirect or a freshly-recorded lock under a commencement addendum (Harlan J).

## Part 5 - Relief options

- **(i)** Adopt T1 (canon on `main` of the established repo; V1 as a protected read-only `v1` archive branch/estate; single Gazette on the canonical line including V1 archival law with the graph intact and REALM cases archival-source-only), as a **governed migration** honouring non-disturbance; direct the legislature to codify in the Framework Act + raft.
- **(ii)** Adopt T3 (keep separate) now; revisit consolidation later.
- **(iii)** Adopt T1 in principle but stage it: settle the topology, defer the physical migration behind the Canonicalisation instrument's governed-rename guarantee.
- **(iv)** Refer a sub-point (e.g. whether an estate may be a mere branch) for further settlement.

The bench is bound by none of these. The actual repo move, branch protection, push, and any re-licence are the Principal's warranted acts; this Council settles the topology principle only.
