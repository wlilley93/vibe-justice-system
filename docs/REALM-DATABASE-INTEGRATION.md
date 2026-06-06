# Realm Database Integration - legislation & case law on the acmeco kernel

**Goal (Founder, 2026-06-06):** legislation and case law should *live in the acmeco database* as the single live
register, tracking each instrument's **status** on the UK model (in force / repealed / superseded for statutes;
good-law / overruled / distinguished for cases) and the relationships between them (amends, repeals, cites,
overrules). The committed markdown remains the vendored, human-readable canonical text; the database is the live,
queryable register of *status, lifecycle, and relationships* that markdown cannot track well.

This is a plan, not yet built. It is grounded in acmeco's **actual** kernel (42 migrations: `workspace`, `actor`,
`matter`, `event` immutable audit chain, `document`/`document_version`/`version`, `convening` SINGLE/PANEL,
`signing_envelope`/`signing_event`/`signing_recipient`, `gate`, `review_item`, `policy`, `global_identifier`).

## 1. Why acmeco is the right substrate

acmeco is the realm's legal-operations data plane. It already models, as first-class primitives, almost everything a
statute-and-case register needs:

| Realm concept | acmeco primitive it builds on |
|---|---|
| The realm / a jurisdiction (RLS scope root) | `workspace` |
| The Founder, judges, committee members, agents | `actor` (kind = HUMAN/AGENT/SYSTEM/EXTERNAL) |
| **Neutral citation register** (REALM-SC/PC/CA, ENG/CHAN, CC-<repo>) | `global_identifier` (deterministic, unique) |
| The **text** of an Act or judgment, append-with-supersede | `document` + `document_version` / `version` |
| A bill's passage / a case's progress through court | `matter` + `matter_step` (status lifecycle) |
| A court sitting or a committee sitting | `convening` (shape = SINGLE / PANEL) |
| **Royal Assent**; a judgment hand-down | `signing_envelope` + `signing_recipient` + `signing_event` |
| The committee vote; the clerk's integrity gate | `gate` + `review_item` / `review_policy` |
| The immutable constitutional audit log | `event` (append-only, bigserial-ordered, INV-8) |
| Relationships (amends/cites/overrules) | `entity_edge` (typed graph) or a new `law_relation` table |

The deterministic citation engine (`cli/lib/citation.js`) becomes the minter for `global_identifier`; the universal
ledger (`ministry-of-justice/ledger/INDEX.md`) becomes a **read projection** of the DB, not a hand-maintained file.

## 2. New schema (a kernel migration slice: `00xx_realm_law.sql`)

Two instrument families - **legislation** and **case law** - sharing the citation register, the document subsystem,
and the audit chain.

### 2.1 Status vocabularies (UK-modelled enums)

```sql
-- Bills (a legislative matter in flight)
CREATE TYPE bill_status AS ENUM (
  'DRAFTING','COMMITTEE','REFERRED_PRIVY','BEFORE_SUPREME','AWAITING_SOVEREIGN',
  'VOTED_PASS','DEADLOCK_SECOND_ROUND','PRESENTED_FOR_ASSENT','ENACTED','WITHDRAWN');

-- Acts / statutes (the enacted instrument)
CREATE TYPE statute_status AS ENUM (
  'PROSPECTIVE','IN_FORCE','PARTIALLY_IN_FORCE','AMENDED','REPEALED','SUPERSEDED','SPENT');

-- Case law (a judgment)
CREATE TYPE case_status AS ENUM (
  'GOOD_LAW','DISTINGUISHED','DOUBTED','OVERRULED','REVERSED','AFFIRMED',
  'SUPERSEDED_BY_STATUTE','PER_INCURIAM','VOID');

-- Court / instrument provenance level (drives citation series + crest/wordmark)
CREATE TYPE law_level AS ENUM (
  'SUPREME','COURT_OF_APPEAL','PRIVY_COUNCIL','HIGH_COURT','COUNTY_COURT','STATUTE','CONSTITUTIONAL');
```

### 2.2 Core tables

```sql
-- The instrument: one row per Act OR judgment. Text lives in document_version (versioned, append-supersede).
CREATE TABLE instrument (
  id             text PRIMARY KEY,
  workspace_id   text NOT NULL REFERENCES workspace(id) ON DELETE RESTRICT,
  kind           text NOT NULL CHECK (kind IN ('STATUTE','CASE')),
  level          law_level NOT NULL,
  citation_id    text NOT NULL REFERENCES global_identifier(id),   -- the neutral citation
  short_title    text NOT NULL,
  long_title     text,
  division       text,             -- High Court only (Engineering / Chancery)
  repo           text,             -- County Court only (acmeco, Operator, ...)
  current_version_id text REFERENCES document_version(id),         -- the in-force text
  enacted_at     timestamptz,
  created_by     text NOT NULL REFERENCES actor(id),
  created_at     timestamptz NOT NULL DEFAULT now(),
  CONSTRAINT uq_instrument_citation UNIQUE (citation_id)           -- the s.19(5) integrity invariant
);

-- Status is a temporal record, not a single column: every status change is an immutable row (UK "law in force"
-- model - you can ask "what was the status on date D"). The instrument's live status is the latest row.
CREATE TABLE instrument_status (
  id             text PRIMARY KEY,
  instrument_id  text NOT NULL REFERENCES instrument(id) ON DELETE CASCADE,
  statute_status statute_status,
  case_status    case_status,
  bill_status    bill_status,
  effective_from timestamptz NOT NULL DEFAULT now(),
  by_instrument  text REFERENCES instrument(id),   -- the Act/judgment that caused the change
  reason         text,
  event_id       bigint REFERENCES event(id),      -- the audit-chain anchor
  CHECK (num_nonnulls(statute_status, case_status, bill_status) = 1)
);

-- The law graph: amends / repeals / supersedes / cites / overrules / distinguishes / confirms / enacts.
CREATE TABLE law_relation (
  id           text PRIMARY KEY,
  from_id      text NOT NULL REFERENCES instrument(id) ON DELETE CASCADE,
  to_id        text NOT NULL REFERENCES instrument(id) ON DELETE CASCADE,
  kind         text NOT NULL CHECK (kind IN
                 ('amends','repeals','supersedes','cites','overrules','distinguishes','confirms',
                  'enacts','reverses','affirms','consolidates','commences')),
  pinpoint     text,             -- "s. 4", "ratio I"
  created_at   timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX ix_law_relation_to ON law_relation (to_id, kind);
```

`instrument_status` as an append-only temporal table is the UK "as-amended / point-in-time" model: the citator can
answer *"was this good law on 5 June 2026?"*, and a repeal is a new status row, never a deletion - mirroring
CASE-LAW's "append with supersede; never silently repealed".

## 3. Lifecycle state machines (status transitions)

**Bill -> Act** (drives, and is driven by, the Standing Committee workflow):

```
DRAFTING -> COMMITTEE -> (REFERRED_PRIVY -> BEFORE_SUPREME -> AWAITING_SOVEREIGN)? ->
  VOTED_PASS -----------------------------> PRESENTED_FOR_ASSENT -> ENACTED -> [statute: IN_FORCE]
  DEADLOCK_SECOND_ROUND -> (re-draft) ----^
  WITHDRAWN
```

**Case** (driven by court rulings + later treatment):

```
GOOD_LAW -> { DISTINGUISHED | DOUBTED | OVERRULED | REVERSED | SUPERSEDED_BY_STATUTE | PER_INCURIAM | VOID }
```

A later instrument that overrules an earlier one writes BOTH a `law_relation('overrules')` and a new
`instrument_status` row on the target (case_status = OVERRULED, by_instrument = the later case) in one transaction,
anchored to one `event`. The citator projection updates automatically.

## 4. Workflow mapping (the institutions become rows)

- **Standing Committee** sitting -> `convening` (PANEL of 4). Each member memo -> `review_item`. The vote ->
  `gate` (pass = >= 3 ayes; deadlock -> second round). Privy Council referral -> a child `convening`.
- **Royal Assent** -> a `signing_envelope` with the Sovereign as the sole `signing_recipient` (role APPROVER);
  the `signing_event` SIGNED transition flips the bill `ENACTED` and the statute `IN_FORCE` and stamps `enacted_at`.
- **Judgment hand-down** -> a `signing_envelope` signed by the panel; minting the `global_identifier` is gated on
  the deterministic citation engine (no duplicate series+N), enforcing the s. 19(5) integrity gate at the DB layer.
- **The clerk's pre-commit gate** -> a `policy` + `gate` that fails closed on a duplicate citation or a missing
  citator row, exactly as the case law requires, now enforced by a DB constraint (`uq_instrument_citation`) rather
  than by model judgement.

## 5. Ingestion (what to load first)

1. Mint a `global_identifier` for every existing citation under the new scheme (the `docs/CITATION-MAP.md` set:
   realm REALM-*, acmeco CC-ACMECO-*, harvey).
2. Insert one `instrument` per ruling and per Act; load each text as a `document_version`.
3. Backfill `instrument_status` from the current ledger (good-law / void / superseded) and `law_relation` from the
   "cites / confirms / enacts / overrules" columns already in the citators.
4. Insert the 25 Bills (from the committee workflow) as instruments in `DRAFTING -> ... -> ENACTED`, each carrying
   its committee note, vote record, and any Privy Council / Supreme / Sovereign flag as `review_item`s + events.
5. Regenerate `ministry-of-justice/ledger/INDEX.md` and each `.justice/INDEX.md` as **projections** of the DB.

## 6. Source-of-truth reconciliation (a flagged fork for the court / Founder)

Case law **[2026] REALM-PC 4** (ex LEXBY-FI 4) held the committed markdown is the *sole source of law* and any index
is *derived, pointer-only, never the store of a ratio*. Moving the live register into the database is in tension with
that ratio. The reconciliation proposed here, **for the Privy Council / Founder to settle**:

- The **canonical text** of every instrument lives in BOTH the git markdown (vendored, human-readable, the thing a
  human reads and the courts cite) AND a `document_version` (byte-identical, hash-pinned). They are kept in lockstep:
  a commit regenerates the DB row; the DB export regenerates the markdown. Neither silently diverges.
- The database is authoritative for **status, lifecycle, relationships, audit, and assent** - facts markdown cannot
  hold reliably. The markdown remains authoritative for **text**. This keeps the REALM-PC 4 principle (text is law,
  in the readable record) while giving the realm a real register. If the court prefers, the DB is demoted to a pure
  derived index per REALM-PC 4 and the markdown stays sole authority for everything - a one-line config either way.

## 7. acmeco refactor dependency

This integration is the natural home for the **acmeco refactor** (strip its parallel VJS spine, keep it a project
repo): acmeco stops being a second VJS and instead *hosts the realm's law database* as the Engineering Division's
County Court data plane. acmeco's own matters become `CC-ACMECO` instruments in the same register.

**UP:** [`../README.md`](../README.md). **Citations:** [`CITATION-MAP.md`](CITATION-MAP.md).
**Legislature:** [`../legislature/README.md`](../legislature/README.md).
