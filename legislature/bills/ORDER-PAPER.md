# Order Paper - the first 25 Bills of the Realm

> **Status: all 25 Bills drafted and presented for Royal Assent (2026-06-06).** Drafted by the live 4-member
> Standing Committee over two sittings; every Bill routed through the Privy Council; Bills 11 and 17 broke a
> deadlock in a second drafting round. **22 of 25 carry a Sovereign-consultation flag** - see
> [`SOVEREIGN-CONSULTATIONS.md`](SOVEREIGN-CONSULTATIONS.md). Affirming **Bill 1 (Acts of Union 2026)** on Royal
> Assent resolves the shared root question (the express amendment seating Acts above case law).


The founding legislative programme. The Standing Committee (Aldous, Verity, Marlowe, Drummond) contributes to every
bill; the Sovereign Founder enacts. Bills 1-5 create the state; 6-14 make it governable, safe, and self-evolving;
11-13 are the guardrail layer; 15-25 operationalise the agent-native realm. Each bill, when drafted, lands in
`legislature/bills/NN-<slug>.md` with its Committee note; on enactment it is recorded in `../../statutes/`.

| No. | Bill | Function | Lead slant |
|---|---|---|---|
| 1 | **Acts of Union 2026** | Creates the realm; the supreme constitutional settlement. | Verity |
| 2 | **Legislature of the Realm Act 2026** | The sovereign-legislature, the Standing Committee, and future automation (renamed from "Parliament of the Realm Act"). | Verity |
| 3 | **Judicature Act 2026** | The courts and hierarchy: Supreme Court, Court of Appeal, Privy Council, High Court Divisions, County Courts. | Verity |
| 4 | **Civil Procedure Code 2026** | Working procedure for civil justice. | Drummond |
| 5 | **Ministries and Offices Act 2026** | The executive machinery of government. | Drummond |
| 6 | **Agents and Duties Act 2026** | Agent status, fiduciary duties, conduct, delegation, sanctions. | Marlowe |
| 7 | **Memory, Records and Archives Act 2026** | Protection and integrity of memory, records, repositories. | Marlowe |
| 8 | **Public Reasons and Audit Act 2026** | Makes power explainable, traceable, auditable. | Marlowe |
| 9 | **Emergency Powers Act 2026** | Temporary crisis powers, bounded and reviewable. | Aldous |
| 10 | **Succession and Amendment Act 2026** | Continuity of authority and constitutional change. | Verity |
| 11 | **Judicial Independence and Lord Chancellor Act 2026** | Separates judiciary from the Ministry of Justice; constitutional guardian. | Marlowe |
| 12 | **Rights, Standing and Due Process Act 2026** | Who may sue; procedural rights; protection from arbitrary action. | Marlowe |
| 13 | **Enforcement, Sanctions and Compliance Act 2026** | Gives orders, audits, and duties practical force. | Drummond |
| 14 | **Delegated Legislative Authority, Law Reform and Codification Act 2026** | Permits autonomous/subordinate law-making without agents becoming sovereign. | Verity |
| 15 | **Interpretation Act 2026** | Definitions, construction of statutes, and the transition of the former SPEC-LAW into case law subordinate to statute. | Verity |
| 16 | **Neutral Citations and Law Reporting Act 2026** | The provenance citation scheme (REALM-SC/PC/CA, Division, CC-repo) and the statuses of cases and statutes. | Verity |
| 17 | **Agent Authentication and Identity Act 2026** | Identity, authentication, and the agent roll. | Drummond |
| 18 | **Autonomous Execution and Safety Act 2026** | Lawful autonomous execution, safety limits, capability-is-not-authority. | Drummond |
| 19 | **Model Validation and Assurance Act 2026** | Validation, assurance, and provenance of models that act in the realm. | Drummond |
| 20 | **Repositories and Records Certification Act 2026** | Certification and trust status of repositories holding law, memory, and evidence. | Verity |
| 21 | **Security and Integrity Act 2026** | Protection against hostile execution, compromise, corruption. | Marlowe |
| 22 | **Data, Disclosure and Confidentiality Act 2026** | Disclosure duties, privilege, confidentiality, and protection of data. | Marlowe |
| 23 | **Resources and Compute Allocation Act 2026** | Allocation, budgets, and audit of computational and operational resources. | Aldous |
| 24 | **Court Rules and Practice Consolidation Act 2026** | Consolidates court rules and practice directions into one accessible code. | Aldous |
| 25 | **Commencement and Transitional Provisions Act 2026** | Commencement, savings, and the transition from the founding case-law settlement to the statute book. | Aldous |

## Status key (and the database it will live in)

Each bill/Act tracks a **status** on the UK model. Statuses are being migrated into the acmeco database as the
single live register of all legislation and case law (see `docs/REALM-DATABASE-INTEGRATION.md`):

- **Bills:** `drafting` -> `committee` -> `referred-privy` / `before-supreme` / `awaiting-sovereign` -> `reported` -> `enacted`.
- **Acts (statutes):** `in-force`, `partially-in-force`, `amended`, `repealed`, `superseded`, `spent`, `prospective`.
- **Cases:** `good-law`, `distinguished`, `overruled`, `superseded-by-statute`, `per-incuriam`, `void`.

**UP:** [`../README.md`](../README.md). **Committee:** [`../committee/CHARTER.md`](../committee/CHARTER.md).
