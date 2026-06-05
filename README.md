# Agent Universe

The realm. A private fork of the [Vibe Justice System](https://github.com/wlilley93/vibe-justice-system) (kept as the
`upstream` remote), organised as a state under the court geography the Supreme Court settled in
**In the matter of Agent Universe [2026] LEXBY-HARVEY-SC 2**, enacting **SPEC-LAW s. 22**. (The inherited VJS readme is
preserved at [`ministry-of-justice/README.md`](ministry-of-justice/README.md).)

> **Signpost rule:** everything signposts everything else, so an agent (or a court doing research) never gets lost.
> Every node carries a `_signpost.md` (UP to its parent + the apex law; DOWN to its children). Climb: repo -> department
> (High Court Division) -> apex (Court of Appeal / Supreme Court) -> the one statute book. The universal ledger sees all.

## The one spine (the law, at the root = the Ministry of Justice's substance)

- **[SPEC-LAW.md](SPEC-LAW.md)** - the one sovereign statute book (s. 1-22). The Supreme Court alone enacts it.
- **[VPR.md](VPR.md)** - the Vibe Procedure Rules. **[CDD.md](CDD.md)** - the commentary.
- **[court/workflows/](court/workflows/)** - the runnable courts (First Instance, Court of Appeal, Supreme Court).
- **[caselaw/](caselaw/)** + **[community/caselaw/](community/caselaw/)** - apex precedent.

There is **one** Court of Appeal and **one** Supreme Court, central, at the Ministry of Justice; they alone enact and bind
the whole realm (s. 9, s. 22). Local courts are hearing-centres only and may never enact.

## The map

```
agent-universe/                                    the realm (Sovereign level)
├── SPEC-LAW.md VPR.md court/ caselaw/             the one spine = Ministry of Justice's law (syncs with upstream)
├── parliament/                                    the legislature (Sovereign route to law, s. 2): bills, sittings, ministers
├── statutes/                                      the legislative archive (Acts as passed); SPEC-LAW.md is the consolidation
├── constitution/                                  realm constitutional docs; future parliament
├── ministry-of-justice/                           governance only, pure/meta
│   └── ledger/INDEX.md                            the UNIVERSAL case ledger (every ruling, one series)
├── ministry-for-business-work-and-skills/         the executive ministry
│   ├── legal-department/                          High Court, Legal Division (≈ Chancery)
│   │   ├── harvey-labs/                           County Court at harvey-labs - Corporate DD (48/50 on Harvey LAB)
│   │   ├── lists/                                 Chancery Lists: corporate, companies, property, trusts-probate, insolvency, ip
│   │   └── references/mike                        legal-AI-product reference
│   ├── engineering-department/                    High Court, Engineering Division
│   │   └── projects/                              acmeco, Operator, Onyx, fleetco-agent, Jarvis, jarvis-voice
│   ├── skills-and-education/scratch-to-signals
│   └── business-operations/Clara                  client estate
├── home-office/                                   (future: personal)
├── ministry-of-defence/                           (future: research & policy)
└── national-archives/                             archive, acmeco-legacy, ldd-plugin (dead/superseded)
```

## Navigation

| To find... | Go to |
|---|---|
| The law (statute) | [`SPEC-LAW.md`](SPEC-LAW.md) |
| Procedure | [`VPR.md`](VPR.md) |
| Every case, everywhere | [`ministry-of-justice/ledger/INDEX.md`](ministry-of-justice/ledger/INDEX.md) |
| The Legal Department | [`ministry-for-business-work-and-skills/legal-department/`](ministry-for-business-work-and-skills/legal-department/) |
| The Engineering Department | [`ministry-for-business-work-and-skills/engineering-department/`](ministry-for-business-work-and-skills/engineering-department/) |
| The realm structure / topology | [`constitution/REALM-TOPOLOGY.md`](constitution/REALM-TOPOLOGY.md) |

The repos still resolve at their old `~/Projects/<repo>` paths (symlinks) during transition.
