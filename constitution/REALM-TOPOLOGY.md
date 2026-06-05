# Realm topology

Constitutional structure of Agent Universe, as settled by **In the matter of Agent Universe [2026] LEXBY-HARVEY-SC 2**
(Supreme Court, full nine, affirming 8-1) and entrenched by **SPEC-LAW s. 22**, building on SC-DC 1 / s. 21
("divisions over one spine, never domain courts").

## The state

- **Agent Universe** (this repo) - the realm/Sovereign level. Houses the constitutional documentation and (future) a
  system of parliament: ministers, convened sittings, bills.
- **Ministry of Justice** - governance only, pure and meta. The one spine lives here: SPEC-LAW, the VPR, the single
  Court of Appeal and Supreme Court (which alone enact and bind the realm), and the **universal ledger** of all cases.
  (Built on the VJS fork; the VJS files sit at the realm root and sync with the public `upstream`.)
- **Ministry for Business, Work and Skills** - the executive ministry holding the operational departments.
- **Home Office** (future: personal matters), **Ministry of Defence** (future: research & policy; commissions
  think-tanks under Business, Work and Skills), **National Archives** (dead/superseded work).

## The judiciary (one court, sitting in many places)

```
Supreme Court            apex; foundational/constitutional; the ONLY enactor of SPEC-LAW   ── Ministry of Justice
Court of Appeal          apex; single & central; hears from every department (cross-cutting) ┘
      ▲ appeal (leave)
High Court (X Division)  DEPARTMENT level; sets the department's jurisdiction-local rule-set
   ├ Legal Division (≈ Chancery)  → Lists: corporate, companies, property, trusts-probate, insolvency, ip
   └ Engineering Division
      ▲ refer up (transfer)
County Court at <repo>   REPO level; a project's own matters; refers weightier / rule-setting questions up
```

- **One** Court of Appeal and **one** Supreme Court, central, at the Ministry of Justice. Their singleness may never be
  relaxed (s. 22(2)); a per-department appellate court or apex would be the competing sovereign s. 9 forbids.
- Local courts (District Circuits / County Courts at a repo; High Court Divisions at a department) are **hearing-centres**
  of the one judiciary: they apply the one SPEC-LAW, record jurisdiction-local precedent only, and **never enact**
  realm-wide statute (reserved to the single Supreme Court, s. 9, s. 22).
- A department's **product** deliberation feature (e.g. the Legal Department's diligence review panel) is **not a court**
  (s. 14, s. 21(3)): it uses the deliberation pattern as method, takes a product-local id, and never the LEXBY series.

## Naming and citation (s. 22(4), s. 11(d))

- Cause titles are descriptive and non-operative: *In the matter of Acmeco*, *Re \<project\>*, alongside the single
  neutral-citation series **`[YEAR] LEXBY-<TIER> n`** (e.g. *In the matter of Acmeco [2026] LEXBY-... n*, on the model of
  *Re Spectrum Plus Ltd [2005] UKHL 41*). No per-subject or per-domain citation series is ever minted.
- The single LEXBY series under one citator (the universal ledger) remains the sole canonical handle; the s. 19(5)
  integrity gate keys on the neutral citation alone.

Full judgment: `ministry-for-business-work-and-skills/legal-department/harvey-labs/harvey-caselaw/2026/lexby-harvey-sc-2-agent-universe-court-geography.md`.
