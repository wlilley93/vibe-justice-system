# The Gazette Graph Database Instrument 2026

**Citation:** [2026] REALM-SI 9 (under Bill 5, Bill 7, Bill 8, Bill 16, Bill 20, Bill 22, Bill 26, Bill 27 and Bill 29)

**Made by:** the Standing Committee of the Legislature, in exercise of the operational-detail statutory-instrument power conferred by section 18 of the Ministries and Offices Act 2026 (Bill 5) as inserted by the Statutory Instruments (Framework) Act 2026 (Bill 26); the parent authority being the Ministry of Justice for law-reporting, public-law record, and Gazette process, read with the Memory, Records and Archives Act 2026 (Bill 7), the Public Reasons and Audit Act 2026 (Bill 8), the Neutral Citations and Law Reporting Act 2026 (Bill 16), the Repositories and Records Certification Act 2026 (Bill 20), the Data, Disclosure and Confidentiality Act 2026 (Bill 22), the VJS (Constitution and Machinery) Act 2026 (Bill 27), and the Ministerial Policy Arm Act 2026 (Bill 29).

**Status:** made

**Procedure:** negative (Bill 14 s.14 objection window)

**Made:** 2026-06-07

**Coming into force:** on expiry of the Bill 14 s.14 objection window without valid objection.

---

## Recitals

In exercise of the powers conferred by the public-record, reasons, law-reporting, repository-integrity, data-boundary, VJS machinery, and ministerial-policy provisions cited above, the Standing Committee makes the following Regulations.

The Realm Law Reports & Gazette is the public reading room and derived projection over committed public case law, Acts, and statutory instruments.

The public record is large enough that users and agents need lineage, not only search. A Gazette item should show what it cites, changes, applies, confirms, distinguishes, overrules, amends, repeals, commences, implements, or depends on, and should give a brief public reason for each relationship.

This Instrument requires that graph feature while preserving the public/private boundary and leaving the technical specification and execution to the Ministry of Business, Engineering and Skills.

---

## PART 1 - INTERPRETATION

### 1. Definitions

In these Regulations:

**"Gazette"** means the Realm Law Reports & Gazette, including the public law-report and legislative reading-room projections.

**"Gazette item"** means a public judgment, Act, statutory instrument, bill record, citator entry, reasons-ledger entry, or other public law-report item that the Ministry of Business, Engineering and Skills includes in the Gazette graph under the implementation specification.

**"graph database"** includes a graph-native database, relational graph table, static graph JSON projection, or other graph-equivalent store that can represent directed typed relationships between Gazette items.

**"node"** means the graph representation of a Gazette item.

**"edge"** means a directed typed relationship from one Gazette item to another.

**"lineage"** means the chain of edges by which a user can trace the relationship between rulings, Acts, statutory instruments, and other Gazette items.

**"brief why"** means a concise public explanation of why an edge exists, sufficient to orient a user without reproducing the full reasoning.

**"backfill"** means the initial and continuing process of deriving graph nodes and edges from Gazette items that already existed before this Instrument came into force.

**"forward-facing filing"** means the process by which a new or amended Gazette item declares, derives, validates, and publishes graph edges from the point of filing onward.

---

## PART 2 - GAZETTE GRAPH REQUIREMENT

### 2. Graph feature required

(1) The Gazette must feature a graph database or graph-equivalent lineage store.

(2) The graph must represent each included Gazette item as a node.

(3) The graph must represent public relationships between Gazette items as directed typed edges.

(4) The graph must be exposed through the Gazette in a form that lets a user trace the lineage of rulings and law.

### 3. Minimum edge record

(1) Each edge must record at least:

1. source Gazette item;
2. target Gazette item;
3. relationship kind;
4. direction;
5. optional pinpoint;
6. brief why;
7. source of the edge, including whether it is backfilled, filed, derived, reviewed, provisional, or court/committee ordered; and
8. status of the edge.

(2) The edge record must be public/system-data only.

(3) The edge record must not expose private facts, secret material, operational details, or local/private evidence.

### 4. Initial public relationship vocabulary

(1) The initial public relationship vocabulary must include:

- cites;
- applies;
- distinguishes;
- overrules;
- reverses;
- affirms;
- confirms;
- supersedes;
- amends;
- repeals;
- commences;
- enacts;
- implements;
- refers-to;
- referred-by;
- depends-on;
- interprets;
- authorises;
- limits; and
- corrects.

(2) The Ministry of Business, Engineering and Skills may implement additional internal labels, but the Gazette-facing labels must remain stable unless amended or superseded through the public process.

---

## PART 3 - BACKFILL

### 5. Backfill duty

(1) The Ministry of Business, Engineering and Skills must backfill the Gazette graph from existing public Gazette items.

(2) The backfill must cover, at minimum:

- central judgments;
- Acts;
- statutory instruments;
- citator rows;
- reasons-ledger rows; and
- Gazette corpus records.

(3) Backfill must derive edges first from explicit public sources, including `Cites` fields, citator rows, reasons-ledger relationship language, statutory-instrument parent authority, amendment language, commencement language, and express appeal, referral, or supersession statements.

(4) Ambiguous edges must be marked provisional or referred for Ministry of Justice review. They must not be silently promoted to settled lineage.

(5) Backfill must be append-with-supersede. It must not silently rewrite old legal meaning.

### 6. Backfill report

(1) The Ministry of Business, Engineering and Skills must produce a public backfill report.

(2) The report must state:

- number of nodes created;
- number of edges created;
- edge kinds used;
- items with no detected edges;
- ambiguous edges needing review; and
- unresolved source-of-truth or public/private boundary issues.

(3) The report must not reproduce private facts.

---

## PART 4 - FORWARD-FACING FILING

### 7. Forward-facing edge declaration

(1) Every new Gazette item must, at filing or ingestion, declare or derive its graph edges.

(2) For each outgoing edge, the filing must identify:

- the target item;
- the relationship kind;
- the pinpoint if available;
- the brief why; and
- whether the edge is deterministic, reviewed, or provisional.

(3) Where a new item changes the status or lineage of an earlier item, the process must create the corresponding incoming edge and status update for the earlier item.

(4) Where a new item has no graph edges, the filing must say so expressly.

### 8. Validation

(1) The Ministry of Business, Engineering and Skills must provide validation tooling to check that forward-facing Gazette filings include required graph metadata or a reasoned no-edge declaration.

(2) The validation tooling may fail closed for malformed public graph metadata where the filing is a Gazette publication.

(3) The tooling must not adjudicate legal merits. Disputed legal relationships are routed to Ministry of Justice review, the Court, or the Legislature as appropriate.

---

## PART 5 - USER-FACING LINEAGE

### 9. Gazette display

Each Gazette item page should show, at minimum:

- this item relies on;
- this item changes;
- this item is changed by;
- this item is cited by;
- lineage paths where available;
- brief why text for each displayed edge; and
- links to canonical markdown and rendered PDF where available.

### 10. Agent retrieval

(1) The graph may be used by agents as a retrieval and orientation tool.

(2) The graph does not replace the canonical text, the citator, the judgment, the Act, the statutory instrument, or the competent organ's decision.

(3) A graph edge is evidence of a public relationship. It is not itself the ratio, statutory text, remedy, or legal force unless a competent organ has made it so.

---

## PART 6 - OWNERSHIP AND IMPLEMENTATION

### 11. Ministry of Justice ownership

The Ministry of Justice owns:

1. the public-law relationship taxonomy;
2. the route for resolving ambiguous or disputed legal edges;
3. the public/private boundary for Gazette lineage; and
4. the policy for backfill reports.

### 12. Ministry of Business, Engineering and Skills ownership

(1) The Ministry of Business, Engineering and Skills must decide the technical specification and execute the implementation.

(2) The Ministry of Business, Engineering and Skills ownership includes:

- graph database or graph-equivalent store selection;
- schema details beyond the minimum required by this Instrument;
- build scripts and migrations;
- backfill tooling;
- forward-facing filing integration;
- UI and API design;
- validation gates;
- derived artifact regeneration; and
- performance and maintainability.

(3) The Ministry of Business, Engineering and Skills must publish a public implementation specification containing principles, schema, build commands, and validation rules. Private operational facts, if any, must remain in the appropriate private registry.

### 13. Ministry of Data Security advice

The Ministry of Data Security should advise on any ingestion, display, or API risk that could expose private facts, secrets, hostnames, credentials, local paths, or operational details.

---

## PART 7 - SOURCE OF FORCE AND LIMITS

### 14. Source of force

(1) The graph is a public lineage and retrieval feature.

(2) Unless a later court or statute expressly provides otherwise, the canonical public text remains the committed public markdown and rendered public record. The graph is a derived or hash-pinned relationship register.

(3) A graph edge does not create, amend, repeal, overrule, distinguish, or commence law by itself. Legal effect comes from the Gazette item or competent organ to which the edge points.

### 15. No private facts

The graph must contain public/system-data only. Where a relationship depends on private or local evidence, the public graph may record only a lawful redacted pointer or neutral public summary.

### 16. No new court or sanction

This Instrument creates no new court, tribunal, sanction, penalty, or merits-adjudication surface.

---

## PART 8 - COMMENCEMENT

### 17. Commencement

This Instrument comes into force on expiry of the Bill 14 s.14 objection window without valid objection.

---

## Made by the Standing Committee

**Counsel Aldous (Restraint):** The graph is a derived reading aid and lineage register. It is not a new source of law and cannot itself amend, repeal, overrule, or commence anything.

**Counsel Verity (Codifier):** The minimum schema is public: node, edge, relationship kind, direction, pinpoint, source, status, and brief why. MBES owns the technical form, but the public record must expose the lineage.

**Counsel Marlowe (Guardrail):** The graph must remain system data only. Private evidence may justify a lawful redacted pointer, but the public graph cannot become a leak surface for operational facts.

**Counsel Drummond (Pragmatist):** A static JSON projection is enough to start. The instrument permits a graph-native database later, but does not force infrastructure before the first backfill can serve users.

**Clerk's Note:** Made 2026-06-07. Commencement on lapse of the Bill 14 s.14 objection window without valid objection.

---

**END OF INSTRUMENT**
