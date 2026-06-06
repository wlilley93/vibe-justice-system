# The Realm Law Reports &amp; Gazette

A **deterministic, pointer-only, searchable** site over the realm's committed case law and legislation. Case law =
the Law Reports; the legislative Order Paper / Acts = the Gazette. The committed markdown is the law; **this site is
a derived, rebuildable index that stores no ratio/status/citation as authority** - every result links back to the
canonical `.md` (and the rendered `.pdf`).

## Lawfulness (REALM-PC 4 / Bill 16 s. 12)

[2026] REALM-PC 4 forbids a *semantic/vector, token-saving AGENT* retrieval index at the current corpus size, but
expressly permits the **deterministic keyword tier**. This site is exactly that: a **lexical** inverted index
(MiniSearch - no embeddings, no model, no vector store), built at build time, searched in the reader's browser
(**zero model tokens**), and regenerated in lockstep with the corpus. It is a human-facing reading room, not an
agent retrieval layer; it stays inside REALM-PC 4's permitted envelope and is given statutory effect by Bill 16
s. 12. (Decision: proceed without a fresh ruling.)

## Build &amp; serve

```bash
cd law-reports
npm install                 # one dep: minisearch
npm run build               # ingest corpus -> corpus.json -> search-index.json -> site/
# serve from the REPO ROOT so the ../../ links to .justice/ and legislature/ resolve:
cd .. && python3 -m http.server 8787      # open http://localhost:8787/law-reports/site/
```

`npm run build` = `node build/ingest.js && node build/build-search-index.js && cp corpus.json site/`. Re-run after
any corpus change (a pre-commit hook / CI step keeps it in lockstep - REALM-PC 4 condition 1).

## Layout

```
law-reports/
  build/corpus.js            scan + dual-mode front-matter/section parser (ports court/scripts/md_to_ruling_json.py)
  build/parse-bills.js       legislation projection (BillRecord) from legislature/bills/*.md
  build/ingest.js            -> corpus.json  (cases[] + legislation[], pointer-only, deterministic/sorted)
  build/build-search-index.js  -> site/search-index.json  (MiniSearch lexical index, pointer-only payload)
  site/index.html, app.js    static reading room: search + browse-by-court + Order Paper; cream house style
  site/minisearch.umd.js      vendored client lib
  corpus.json, site/*.json    derived artifacts, committed in lockstep with the corpus
```

The index covers all central rulings (REALM-SC/PC/CA), the County Court at acmeco (CC-ACMECO), and the 25 Acts.
Future: the bill PDF renderer + the PDF annotation / comment-for-review surface (a neutral Node+SQLite service
porting acmeco's `pdf_annotation` + `review_item`) - see `docs/REALM-DATABASE-INTEGRATION.md` and the plan.

**UP:** the realm - [`../README.md`](../README.md). **Constraint:** [`../.justice/judgments/privy-council/2026-realm-pc-4.md`](../.justice/judgments/privy-council/2026-realm-pc-4.md).
