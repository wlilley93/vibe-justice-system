# Ministry of Justice (MoJ)

The realm's **governance** ministry, in the Judicature branch. Constituted as the one governance home
(CASE-LAW s. 22(3): **governance only, pure and meta**). It does **not** hold operational repos: the law,
the procedure, and the apex sit here, and nothing else. This charter is **public** (Bill 27 s. 5A
public-mechanics rule): it states the Ministry's mechanics; there are no private operational facts to seal,
because the Ministry holds none.

## Mechanics (what the Ministry does)

- **Is the seat of the central apex courts.** The single **Court of Appeal** and the single **Supreme
  Court** sit at this level (CASE-LAW s. 22(2)). They are the sole organs of appeal and the sole enactors of
  realm-wide statute, and they **belong to the realm, not to this Ministry**: the Ministry hosts the seat,
  it does not own the bench. The Privy Council (constitutional first instance) sits here too. The court
  machinery itself lives in [`../court/`](../court/) (the renderer + the workflows); the apex precedent in
  [`../caselaw/`](../caselaw/) and the Community Record in [`../community/`](../community/).
- **Holds the universal rulings ledger** ([`ledger/`](ledger/)): the realm's master index of every ruling,
  one series, one citator (CASE-LAW s. 11(d) / s. 22). It is a **derived, pointer-only projection** of the
  citator, permitted only on the [2026] REALM-PC 4 terms (wholly derived, deterministically rebuildable,
  a pointer only, never a store of any ratio / status / authority, regenerated in lockstep with its source).
  Rebuild it by re-running [`ledger/build-ledger.py`](ledger/build-ledger.py) after any new ruling.
- **Holds the reasons ledger** ([`reasons-ledger/`](reasons-ledger/)): the executive analogue, the realm's
  pointer-only index of every significant decision and its reasons-record, discharging the Public Reasons
  and Audit Act 2026 (Bill 8). Same REALM-PC 4 discipline: a projection, not a source of law or truth.
- **Superintends the citator and law reporting.** The one citator
  ([`../.justice/INDEX.md`](../.justice/INDEX.md)) is the source the rulings ledger projects from; the
  Ministry keeps the ledgers in lockstep with it and oversees **The Realm Law Reports & Gazette**
  ([`../law-reports/`](../law-reports/)), the deterministic, searchable reading room over the committed
  record (central-courts judgments only).
- **Develops governance policy** through its policy arm ([`policy/`](policy/), Ministry policy -> Standing
  Committee drafting) for the law, the procedure, and the judgment lifecycle (e.g. deterministic judgment
  render-and-lodge).

## Registers it holds

| Register | Visibility | Where |
|---|---|---|
| Universal rulings ledger (derived) | **public** | [`ledger/INDEX.md`](ledger/INDEX.md) |
| Reasons ledger (derived) | **public** | [`reasons-ledger/INDEX.md`](reasons-ledger/INDEX.md) |
| The one citator (source) | **public** | [`../.justice/INDEX.md`](../.justice/INDEX.md) |
| Apex caselaw / Community Record | **public** (law of every judgment) | [`../caselaw/`](../caselaw/) · [`../community/`](../community/) |

## The principle: governance pure and meta, projections never a second record

Per CASE-LAW s. 22(3) and Bill 8: this Ministry is the realm's single governance home and holds no
operational repos and no private facts. The ledgers it holds are **derived projections** (REALM-PC 4): they
store no ratio, status, or authority of their own and point into the citator, the bill files, and the git
history that already hold it. If a ledger ever disagrees with its source, the **source governs** and the
projection is stale - rebuild it. The apex courts seated here are the realm's, exercised under the one
CASE-LAW for the whole realm.

**UP:** [`../`](../) (the Judicature branch) · the realm: [`../../README.md`](../../README.md).
**The law:** [`../../Constitution/CASE-LAW.md`](../../Constitution/CASE-LAW.md) ·
[`../../Constitution/VPR.md`](../../Constitution/VPR.md). **The citator:**
[`../.justice/INDEX.md`](../.justice/INDEX.md).
