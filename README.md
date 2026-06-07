# VJS - the Vibe Justice System

The realm: a sovereign-agent civilisation governed as a state, under the founding case-law settlement
([`Constitution/CASE-LAW.md`](Constitution/CASE-LAW.md)) and the enacted statute book. The canonical name is
fixed by the **VJS (Constitution and Machinery) Act 2026** (Bill 27), which also mandates the layout below.
(Formerly developed under the code name "agent-universe".)

> **Signpost rule:** every node carries a `_signpost.md` (UP to its parent + the apex law; DOWN to its
> children), so an agent or a court doing research never gets lost.

## The four branches (separation of powers as directory structure, Bill 27)

```
VJS/                                  top-level: GitHub files + the four branches, and nothing else
├── Constitution/                     the founding settlement
│   ├── CASE-LAW.md                   the founding case-law settlement (s. 1-22); subordinate to the Acts
│   ├── VPR.md  CDD.md  AGENTS.md     the Vibe Procedure Rules + commentary + the binding agent spine
│   ├── constitution/                 REALM-TOPOLOGY and the constitutional docs
│   └── docs/                         governance documentation
├── Judicature/                       the judiciary + the public law record
│   ├── .justice/                     the central citator (INDEX.md) + judgments + suites
│   ├── court/                        the judgment renderer + workflows
│   ├── caselaw/  community/          apex + Community-Record precedent
│   ├── ministry-of-justice/          the universal rulings ledger + the reasons ledger (derived, pointer-only)
│   └── law-reports/                  The Realm Law Reports & Gazette (searchable, central-courts only)
├── Legislature/                      law-making + enacted law
│   ├── legislature/                  the Standing Committee + the bills (drafting -> vote -> Royal Assent)
│   └── statutes/                     the enacting archive (Acts) + instruments/ (the SI register)
└── Executive/                        the ministries + machinery
    ├── ministry-of-business-engineering-and-skills/   the executive ministry (owns the refactoring suite)
    ├── ministry-of-data-security/    the security ministry (owns the security suite; private estate registry)
    ├── cli/                          the cdd CLI (citation engine + the citator-integrity gate)
    ├── plugin/                       the agent spine (CLAUDE.md) + the hooks (pre-commit gate, watchdog)
    └── docker/                       the clerk runner
```

## What is public (Bill 27: system data only)

The public realm holds **system data only**: the law (CASE-LAW + the Acts + the SI register), the **central
courts' judgments** (Supreme Court `REALM-SC`, Court of Appeal `REALM-CA`, Privy Council `REALM-PC`), the
procedure rules, and the derived registers. The **law of every judgment is public**; **personal or operational
facts are sealed** (Bill 22) and **local (County Court / Division) judgments stay in their own repos** (Bill 27
s. 14). Personal/operational data lives only in separate, gitignored repos (mainly under the Executive
ministries).

## Navigation

| To find... | Go to |
|---|---|
| Why VJS exists (the pitch / onboarding) | [`Constitution/docs/ABOUT-VJS.md`](Constitution/docs/ABOUT-VJS.md) |
| The founding law | [`Constitution/CASE-LAW.md`](Constitution/CASE-LAW.md) |
| Procedure | [`Constitution/VPR.md`](Constitution/VPR.md) |
| The Acts (statute book) | [`Legislature/statutes/README.md`](Legislature/statutes/README.md) |
| The bills + Standing Committee | [`Legislature/legislature/bills/ORDER-PAPER.md`](Legislature/legislature/bills/ORDER-PAPER.md) |
| Every central ruling | [`Judicature/ministry-of-justice/ledger/INDEX.md`](Judicature/ministry-of-justice/ledger/INDEX.md) |
| The citator | [`Judicature/.justice/INDEX.md`](Judicature/.justice/INDEX.md) |
| Significant decisions (reasons) | [`Judicature/ministry-of-justice/reasons-ledger/INDEX.md`](Judicature/ministry-of-justice/reasons-ledger/INDEX.md) |
| The realm topology | [`Constitution/constitution/REALM-TOPOLOGY.md`](Constitution/constitution/REALM-TOPOLOGY.md) |
| Maintainer notes (paths, couplings) | [`STRUCTURE.md`](STRUCTURE.md) |
