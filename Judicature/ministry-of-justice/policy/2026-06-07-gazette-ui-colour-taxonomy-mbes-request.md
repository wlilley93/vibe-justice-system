# Policy Request: Gazette UI Colour Taxonomy

**Type:** Ministry policy request (MoJ public-law process -> competent implementation route)  
**To:** Ministry of Business, Engineering and Skills (MBES), Engineering department  
**From:** Ministry of Justice (MoJ), public-law record and Gazette process  
**Subject:** presentation colour taxonomy for Gazette item strokes and chips  
**Date:** 2026-06-07  
**Status:** referred to MBES for implementation specification and execution

> This is a policy request, not a judgment, statutory instrument, or source of law. MoJ states the
> public-law presentation need and boundary. MBES owns the technical specification and implementation.
> Lexby may coordinate the route but is not the sovereign drafter, bench, Legislature, MoJ, MBES, MDS,
> or source of legal force.

## 1. Problem

The Realm Law Reports & Gazette contains judgments, Acts, statutory instruments, and relationship
metadata. Users and agents should be able to distinguish item classes quickly when scanning result
cards, graph nodes, lineage edges, citation chips, and related-item lists.

At present, item type and court level can require reading text labels that are visually similar. A
small public colour taxonomy would improve orientation without changing the source of force or the
legal status of any item.

## 2. Existing Authority

This request relies on existing Gazette and implementation-route authority:

- `[2026] REALM-SI 9` establishes the Gazette graph as a retrieval and orientation aid.
- `[2026] REALM-SI 9` gives MBES ownership of technical specification and execution for Gazette
  graph and UI implementation.
- `[2026] REALM-SI 8` supports agent lawfulness by requiring proportionate legal research and
  validity checks before governed acts.
- `[2026] REALM-PC 19` requires canonical superrepo changes to proceed by court order or existing
  statutory or precedential authority.

No Privy Council or Supreme Court referral is requested by this policy request. Referral becomes
necessary only if implementation would alter legal status, source of force, public/private boundary,
or disputed legal meaning.

## 3. MoJ Request

MoJ requests that MBES decide and execute a presentation-only colour taxonomy for Gazette UI strokes,
chips, node outlines, edge badges, filters, and similar non-authoritative visual affordances.

The requested taxonomy is:

| Gazette item class | Requested visual colour |
|---|---|
| Supreme Court judgments | gold |
| Court of Appeal judgments | blue |
| First Instance judgments | lighter blue |
| Acts | red |
| Statutory instruments | darker red |
| Privy Council judgments | pink or another distinct colour |

MBES should decide the exact accessible palette, CSS tokens, fallback rendering, dark-mode behaviour,
and component integration.

## 4. Boundary and Legal Effect

The colour taxonomy must preserve these limits:

- Colour is presentation metadata only.
- Colour is not a source of force, priority, hierarchy, remedy, ratio, commencement, repeal,
  validity, or binding status.
- The canonical law remains the committed judgment, Act, statutory instrument, and citator record.
- Text labels, citations, and accessible names must remain sufficient without colour.
- Colour must not encode private facts, local operational facts, user identity, case evidence, or
  non-public data.
- Ambiguity in item classification must be routed to the competent public-law process rather than
  silently resolved by UI code.

## 5. Requested MBES Deliverables

MBES should provide:

1. a public implementation specification for item-class colour tokens;
2. UI implementation for Gazette strokes, chips, graph nodes, and related-item affordances where
   appropriate;
3. accessibility checks for contrast, colour-blind use, and non-colour fallback labels;
4. tests or visual checks sufficient to verify that each item class receives the intended token; and
5. documentation explaining that the taxonomy is an orientation aid only.
