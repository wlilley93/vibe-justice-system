# Policy Briefing: Gazette Graph Database

**Type:** Ministry policy briefing (policy-arm: Ministry policy -> Standing Committee drafting)  
**To:** the Ministry of Justice (MoJ), governance ministry of the Judicature  
**From:** the Principal, acting in the executive office  
**Subject:** statutory instrument for a graph database in the Realm Law Reports & Gazette  
**Date:** 2026-06-07  
**Status:** referred to the Standing Committee for drafting as a statutory instrument

> This is a policy briefing, not an instrument of law. The MoJ proposes; the Standing Committee drafts; the instrument has no legal force unless made through the statutory-instrument route.

## 1. The Problem

The Realm Law Reports & Gazette is already a deterministic, pointer-only, searchable projection over committed case law, Acts, and statutory instruments. It lets a user find items, but it does not yet make the legal lineage visible enough.

A user should be able to open any Gazette item and see, in one place:

- what it cites;
- what cites it;
- what it amends, repeals, commences, supersedes, confirms, distinguishes, overrules, applies, or refers to;
- the direction of each link;
- a brief reason why that link exists;
- whether the link comes from backfilled existing law or from forward-facing filing of a new item.

Without this graph, the record is too large for agents or users to hold in memory. Search finds words; lineage explains force.

## 2. Existing Legal and Technical Context

The current law-report site is described as a deterministic, pointer-only reading room. The committed markdown remains the law; the site is a rebuildable index.

Relevant existing materials include:

- `[2026] REALM-PC 4` and Bill 16 s. 12 on deterministic, pointer-only indexing;
- CASE-LAW s. 1 and s. 11(d) on authoritative sources and neutral citation;
- CASE-LAW s. 19(5) on citation integrity;
- Bill 7 and Bill 8 on memory, records, reasons, and audit;
- Bill 16 on neutral citations and law reporting;
- Bill 20 on repository and record integrity;
- Bill 27 on the public VJS record and public-mechanics/private-facts boundary;
- Bill 29 on the MoJ policy -> Standing Committee route;
- `Constitution/docs/REALM-DATABASE-INTEGRATION.md`, which already proposes `law_relation` / graph-style relation records.

The policy should preserve the current source-of-force discipline unless a competent organ later changes it: markdown remains the canonical public text of law; the graph is a derived or hash-pinned register of relationship facts and lineage metadata.

## 3. Policy Recommendation

The MoJ recommends a statutory instrument requiring the Gazette to feature a graph database or graph-equivalent relation store.

The instrument should require:

1. **Graph coverage.** Every Gazette item must be capable of being represented as a node.
2. **Typed lineage edges.** Links between Gazette items must be represented as typed directed edges.
3. **Brief reason for every edge.** Each edge must carry a short public reason explaining why the relationship exists.
4. **Backfill.** Existing judgments, Acts, statutory instruments, citator rows, reasons-ledger entries, and Gazette corpus records must be backfilled into the graph.
5. **Forward filing.** New Gazette items must declare or derive their graph edges when filed, rendered, lodged, or ingested.
6. **MBES specification and execution.** The Ministry of Business, Engineering and Skills (MBES) must decide the technical specification and execute the implementation, subject to the legal principles and public/private boundary set by the instrument.
7. **MoJ ownership of public-law meaning.** MoJ owns the legal lineage taxonomy and public-law process; MBES owns the database design, build scripts, migrations, UI, and operational implementation.
8. **Pointer-only public boundary.** The graph must not expose private facts. It records public/system-data relationships between public Gazette items.

## 4. Minimum Graph Schema

The public law should state principles and minimum schema only. MBES should decide the exact engine, tables, file format, migration sequence, API, and UI.

The minimum public schema should include:

| Field | Meaning |
|---|---|
| `from` | the source Gazette item citation or stable identifier |
| `to` | the target Gazette item citation or stable identifier |
| `kind` | relationship type |
| `direction` | outgoing from source to target |
| `pinpoint` | optional section, paragraph, ratio, recital, or table location |
| `why` | brief public reason for the link |
| `source` | backfilled, filed, derived, manual-review, or court-ordered |
| `confidence` | deterministic, reviewed, or provisional |
| `created_by_process` | ingestion, renderer, citator, clerk review, MBES migration, or court/committee filing |
| `status` | active, superseded, disputed, or removed-by-supersession |

## 5. Relationship Types

The first relationship vocabulary should include at least:

- `cites`
- `applies`
- `distinguishes`
- `overrules`
- `reverses`
- `affirms`
- `confirms`
- `supersedes`
- `amends`
- `repeals`
- `commences`
- `enacts`
- `implements`
- `refers-to`
- `referred-by`
- `depends-on`
- `interprets`
- `authorises`
- `limits`
- `corrects`

MBES may add internal implementation labels, but the public Gazette should expose stable public labels.

## 6. Backfill

Backfill should proceed in stages:

1. Build nodes for all current Gazette items: central judgments, Acts, and statutory instruments.
2. Backfill edges from explicit `Cites` fields in judgment front matter and citator rows.
3. Backfill edges from statutory-instrument parent authority and amendment language.
4. Backfill edges from reasons-ledger pointers where they state “applies”, “affirms”, “supersedes”, “referred to”, or equivalent public-law relationships.
5. Mark ambiguous edges as provisional pending MoJ or court/committee review.
6. Publish a redacted backfill report summarising counts, edge kinds, unresolved ambiguities, and any items needing legal review.

Backfill must be append-with-supersede. It must not silently rewrite the meaning of an old item.

## 7. Forward-Facing Filing

For every new Gazette item, the filing or ingestion process should require graph information:

- a list of outgoing edges;
- a brief `why` for each edge;
- any incoming edge updates required on older items;
- status changes caused by the new item;
- whether the edge is deterministic from citation text or reviewed by a human/agent clerk.

If a new item has no edges, the filing should say so expressly.

## 8. User Experience

The Gazette should let a user trace lineage without reading every item in full.

At minimum, each item page should expose:

- “This item relies on”;
- “This item changes”;
- “This item is changed by”;
- “This item is cited by”;
- “Lineage path” for selected chains;
- short `why` text for each edge;
- links to the canonical markdown and rendered PDF where available.

The graph should support both human browsing and agent retrieval. It must remain traceable to public sources and must not become an opaque semantic embedding layer.

## 9. Implementation Owner

MBES should decide and execute the technical specification:

- graph database or graph-equivalent store;
- migration and backfill tooling;
- schema details beyond the minimum public fields;
- UI and API design;
- validation gates;
- derived artifact regeneration.

MoJ should own:

- relationship taxonomy in public-law terms;
- review route for ambiguous or disputed legal links;
- policy for backfill reports and public/private boundary.

MDS should advise where graph ingestion risks exposing protected private facts, but the public graph should contain system-data only.

## 10. Referral

The MoJ refers this briefing to the Standing Committee on the Laws of the Realm to draft a **Gazette Graph Database Instrument** as subordinate legislation.

The requested instrument should require the graph feature while leaving the detailed technical specification to MBES.

**UP:** [`../README.md`](../README.md)
