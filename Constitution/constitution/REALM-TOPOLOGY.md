# Realm topology

Constitutional structure of VJS, as settled by **In the matter of Agent Universe [2026] REALM-SC 2**
(Supreme Court, panel of 5, affirmed 4-1) and entrenched by **CASE-LAW s. 22**, building on s. 21
("divisions over one spine, never domain courts").

## The realm structure

The Vibe Justice System (VJS) is organised into four constitutional branches:

### Constitution Branch
Houses the constitutional documentation, founding case law, and structural principles.
- **CASE-LAW.md** - The binding statute book (foundational doctrine and enacted statutes).
- **VPR.md** - Vibe Procedure Rules (procedural framework).
- **CDD.md** - Caselaw Driven Development (methodology).
- **AGENTS.md** - Lexby's duties and operational framework.
- `constitution/` - Constitutional instruments and reference documents.
- `docs/` - Design notes and conceptual models.

### Judicature Branch
Houses the judicial spine, case law, and the unified court system.
- **Ministry of Justice** - Governance only, pure and meta. The one spine lives here:
  - CASE-LAW, the VPR, the single Court of Appeal and Supreme Court (which alone enact and bind the realm).
  - The **universal ledger** of all cases (`ministry-of-justice/ledger` and `ministry-of-justice/reasons-ledger`).
  - The citator (INDEX.md).
- **.justice/** - Local jurisdiction registry:
  - `judgments/supreme-court/` - Apex rulings (realm-wide statute).
  - `judgments/court-of-appeal/` - Appeal judgments.
  - `judgments/privy-council/` - Constitutional first-instance rulings.
  - `caselaw/` - Local jurisdiction precedent.
- **Law Reports** - Published reports of significant rulings.
- **Court** - Court procedures, rooms, and administrative support.
- **Community** - Community record of anonymised Supreme Court precedent (persuasive, not binding, in other VJS jurisdictions).

### Legislature Branch
Houses the parliamentary machinery and bill process.
- `legislature/` - Bills, committee records, parliamentary procedures.
- `statutes/` - Enacted Acts and statutory instruments (output of the bill process).

### Executive Branch
Houses the operational ministries and executive departments.
- **Ministry of Business, Engineering and Skills (MBES)** - Operational ministry holding the engineering and business departments.
- **Ministry of Data Security (MDS)** - Data protection, security, and integrity (formerly Ministry of Defence).
- **Home Office** - Personal matters and future jurisdiction.
- **National Archives** - Dead and superseded work.
- **CLI / Plugin** - Claude Code harness and operative tooling.
- **Docker** - Containerised deployment.

## The judiciary (one court, sitting in many places)

```
Supreme Court            apex; foundational/constitutional; the ONLY enactor of CASE-LAW   ── Ministry of Justice
Court of Appeal          apex; single & central; hears from every department (cross-cutting) ┘
      ▲ appeal (leave)
Privy Council (PC)       the realm's CENTRAL constitutional / governance first instance (ONE court, MoJ / realm level; REALM-PC 15); climbs by progression
      ▲ refer up (transfer)
High Court Division      DEPARTMENT level; sets the department's jurisdiction-local rule-set
   ├ Legal Division (≈ Chancery)  → Lists: corporate, companies, property, trusts-probate, insolvency, ip
   └ Engineering Division
      ▲ refer up (transfer)
County Court at <repo>   REPO level; a project's own matters; refers weightier / rule-setting questions up
```

- **One** Court of Appeal and **one** Supreme Court, central, at the Ministry of Justice. Their singleness may never be
  relaxed (s. 22(2)); a per-department appellate court or apex would be the competing sovereign s. 9 forbids.
- Local courts (District Circuits / County Courts at a repo; High Court Divisions at a department) are **hearing-centres**
  of the one judiciary: they apply the one CASE-LAW, record jurisdiction-local precedent only, and **never enact**
  realm-wide statute (reserved to the single Supreme Court, s. 9, s. 22).
- A department's **product** deliberation feature (e.g. the MBES Engineering Division's diligence review panel) is **not a court**
  (s. 14, s. 21(3)): it uses the deliberation pattern as method, takes a product-local id, and never the REALM series.

## Naming and citation (s. 22(4), s. 11(d))

- Cause titles are descriptive and non-operative: *In the matter of Acmeco*, *Re <project>*, alongside the single
  neutral-citation series **`[YEAR] REALM-<TIER> n`** (e.g. *In the matter of Acmeco [2026] REALM-FI n*, on the model of
  *Re Spectrum Plus Ltd [2005] UKHL 41*). No per-subject or per-domain citation series is ever minted.
- The single REALM series under one citator (the universal ledger) remains the sole canonical handle; the s. 19(5)
  integrity gate keys on the neutral citation alone.
- Tiers:
  - **[YEAR] REALM-FI n**: First Instance (single judge, jurisdiction-local hearing-centre).
  - **[YEAR] REALM-CA n**: Court of Appeal (panel of 3, central apex).
  - **[YEAR] REALM-SC n**: Supreme Court (panel of 5 or 9, central apex, sole enactor of CASE-LAW).
  - **[YEAR] REALM-PC n**: Privy Council (the realm's CENTRAL constitutional / governance first instance, one court at the MoJ / realm level, distinct from the distributed County Courts; [2026] REALM-PC 15).

Full judgment example: `Judicature/.justice/judgments/supreme-court/2026-realm-sc-2.md`.
