# Realm Law - Annotation & Review service (neutral)

A standalone annotation + comment-for-review surface for the realm's judgment/Act PDFs. **Neutral**:
it reuses acmeco's *concepts* (the `pdf_annotation` [0..1] geometry model + the `review_item` lifecycle)
but couples to **no acmeco server** - zero runtime deps (`node:http` + `node:sqlite`), one SQLite file.

- **Backend** (`server/index.js`): an acmeco-shaped verb door `POST /v/<noun>.<verb>`. Verbs:
  `annotation.create/list/remove`, `review.open/claim/resolve/list/unclaim/expire/reassign`. Ports the
  `review.rs` semantics: atomic claim (exactly-one-winner), idempotent open (one non-terminal review per
  subject), SLA computed at read, terminal-immutable resolve, and **INV-4** (resolving records the decision;
  it never edits the law - a corpus amendment is a separate explicit commit).
- **Schema** (`db/schema.sql`): `pdf_annotation` + `review_item`, a strict SUBSET of acmeco's tables
  (dropped: workspace/RLS, the file_record binary FK, matter/gate). `[0..1]` CHECK backstops kept exactly.
- **Client** (`web/`): a PDF.js viewer + annotation overlay (click = COMMENT point, drag = BOX; normalized
  `[0..1]` anchors that survive zoom) + the review queue. Lanes `JUDGMENT_COMMENT` / `BILL_COMMENT`.

## Run
```bash
cd law-reports/review-service && npm install   # devDep: pdfjs-dist (vendored to web/vendor)
node server/index.js                            # http://localhost:8790
```

## Future: fold into acmeco
The verb door + schema are an acmeco subset, so folding in is: re-point the client base URL at acmeco's
kernel, add back `workspace_id`/`file_record`, and make the two lanes enum values (per
`docs/REALM-DATABASE-INTEGRATION.md` §7). Gated on the text-is-law vs DB-register decision.
