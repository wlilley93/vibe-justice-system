'use strict';
// Build a DETERMINISTIC, POINTER-ONLY lexical search index (MiniSearch) from corpus.json.
// This is the REALM-PC 4-compliant artifact: a lexical inverted index (NOT embeddings, NOT a
// vector store, NO model), rebuilt in lockstep with the corpus. The stored payload per document
// is pointers + display fields only; the authoritative ratio/status is read from the linked
// markdown, never trusted from the index (Bill 16 s. 12; condition 1).
//
//   node build/build-search-index.js   ->   law-reports/site/search-index.json

const fs = require('fs');
const path = require('path');
const MiniSearch = require('minisearch');
const { ROOT } = require('./corpus');

const corpus = JSON.parse(fs.readFileSync(path.join(ROOT, 'Judicature', 'law-reports', 'corpus.json'), 'utf8'));

const docs = [];
for (const c of corpus.cases) {
  docs.push({
    id: c.citation, kind: 'case',
    citation: c.citation, title: c.citation, series: c.series, court: c.courtLabel,
    status: c.status, panel: (c.panel || []).join(' '), ratio: c.ratioOneLine, cites: c.cites,
    body: c.searchBody,
    // pointer payload (stored) - everything needed to render a result card + link out:
    p_slug: c.slug, p_source: c.sourcePath, p_pdf: c.pdfPath, p_court: c.court, p_jur: c.jurisdiction,
  });
}
for (const b of corpus.legislation) {
  docs.push({
    id: `bill:${b.no}`, kind: 'bill',
    citation: `Bill ${b.no}`, title: b.shortTitle, series: 'BILL', court: 'Legislature',
    status: b.status, panel: '', ratio: b.longTitle, cites: '', body: b.searchBody,
    p_slug: b.slug, p_source: b.sourcePath, p_pdf: b.pdfPath, p_stage: b.pipelineStage, p_no: b.no,
  });
}

const ms = new MiniSearch({
  idField: 'id',
  fields: ['citation', 'title', 'ratio', 'body', 'court', 'status', 'panel', 'cites'],
  storeFields: ['kind', 'citation', 'title', 'series', 'court', 'status', 'ratio',
    'p_slug', 'p_source', 'p_pdf', 'p_court', 'p_jur', 'p_stage', 'p_no'],
  searchOptions: { boost: { citation: 5, title: 4, ratio: 3 }, prefix: true, fuzzy: 0.2 },
});
// deterministic insert order (corpus is already sorted)
ms.addAll(docs);

const dest = path.join(ROOT, 'Judicature', 'law-reports', 'site', 'search-index.json');
fs.mkdirSync(path.dirname(dest), { recursive: true });
fs.writeFileSync(dest, JSON.stringify(ms.toJSON()) + '\n');
console.log(`search-index.json: ${docs.length} documents (lexical, pointer-only) -> ${path.relative(ROOT, dest)}`);
