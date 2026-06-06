'use strict';
// The Realm Law Reports & Gazette - client app. Loads the deterministic, pointer-only MiniSearch
// index + corpus.json, runs lexical search in the browser (zero tokens, no backend), renders
// pointer cards linking the canonical .md source + .pdf. Paths are "../../"-relative so the site
// works served from the repo root (local or GitHub Pages subpath).

const REL = '../../'; // site lives at law-reports/site/ ; corpus paths are repo-root-relative
let MS = null, CORPUS = null, FILTER = 'all';

const el = (id) => document.getElementById(id);

async function boot() {
  const [idxRaw, corpus] = await Promise.all([
    fetch('search-index.json').then(r => r.json()),
    fetch('corpus.json').then(r => r.json()),
  ]);
  CORPUS = corpus;
  MS = MiniSearch.loadJS(idxRaw, {
    fields: ['citation', 'title', 'ratio', 'body', 'court', 'status', 'panel', 'cites'],
    storeFields: ['kind', 'citation', 'title', 'series', 'court', 'status', 'ratio',
      'p_slug', 'p_source', 'p_pdf', 'p_court', 'p_jur', 'p_stage', 'p_no'],
    searchOptions: { boost: { citation: 5, title: 4, ratio: 3 }, prefix: true, fuzzy: 0.2 },
  });
  el('q').addEventListener('input', render);
  for (const t of document.querySelectorAll('.tab')) {
    t.addEventListener('click', () => {
      document.querySelectorAll('.tab').forEach(x => x.classList.remove('active'));
      t.classList.add('active'); FILTER = t.dataset.f; render();
    });
  }
  render();
}

// Browse base set (from corpus.json) when there is no query, as stored docs shape.
function allDocs() {
  const cases = CORPUS.cases.map(c => ({ kind: 'case', citation: c.citation, title: c.citation,
    series: c.series, court: c.courtLabel, status: c.status, ratio: c.ratioOneLine,
    p_source: c.sourcePath, p_pdf: c.pdfPath, p_jur: c.jurisdiction }));
  const bills = CORPUS.legislation.map(b => ({ kind: 'bill', citation: 'Bill ' + b.no, title: b.shortTitle,
    series: 'BILL', court: 'Legislature', status: b.status, ratio: b.longTitle,
    p_source: b.sourcePath, p_pdf: b.pdfPath, p_stage: b.pipelineStage, p_no: b.no }));
  return [...cases, ...bills];
}

function matchFilter(d) {
  if (FILTER === 'all') return true;
  const [k, v] = FILTER.split(':');
  if (k === 'court') return d.court === v;
  if (k === 'kind') return d.kind === v;
  if (k === 'status') return (d.status || '').toLowerCase().startsWith(v);
  return true;
}

function card(d) {
  const st = (d.status || 'good-law').toLowerCase().replace(/[^a-z-]/g, '');
  const links = [];
  if (d.p_source) links.push(`<a href="${REL}${d.p_source}">source &middot; .md</a>`);
  if (d.p_pdf) links.push(`<a href="${REL}${d.p_pdf}">judgment &middot; .pdf</a>`);
  const sub = d.kind === 'bill'
    ? `<span class="court">Legislature</span> <span class="stage">&middot; ${d.p_stage || ''}</span>`
    : `<span class="court">${d.court}${d.p_jur && d.p_jur.includes('acmeco') ? ' &middot; at acmeco' : ''}</span>`;
  return `<div class="card">
    <div><span class="cite">${d.citation}</span>
      <span class="badge ${st}">${d.status || 'good-law'}</span></div>
    ${sub}
    <div class="ratio">${(d.ratio || '').slice(0, 360)}</div>
    <div>${links.join('')}</div>
  </div>`;
}

function render() {
  const q = el('q').value.trim();
  let docs;
  if (q) {
    docs = MS.search(q).filter(matchFilter);
  } else {
    docs = allDocs().filter(matchFilter);
  }
  el('meta').textContent = `${docs.length} result${docs.length === 1 ? '' : 's'}`
    + (q ? ` for "${q}"` : ' (browsing)')
    + ` · ${CORPUS.counts.cases} rulings + ${CORPUS.counts.legislation} Acts in the record`;
  el('results').innerHTML = docs.map(card).join('') || '<p>No matches.</p>';
}

boot();
