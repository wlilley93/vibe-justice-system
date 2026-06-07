'use strict';
// The Realm Law Reports & Gazette - client app. Loads the deterministic, pointer-only MiniSearch
// index + corpus.json, runs lexical search in the browser (zero tokens, no backend), renders
// pointer cards linking rendered PDFs. Paths are "../../../"-relative so the site works from
// Judicature/law-reports/site/ under the repo root (local or GitHub Pages subpath).

const REL = '../../../'; // site lives at Judicature/law-reports/site/ ; corpus paths are repo-root-relative
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
  renderStart();
  render();
}

const LAW_START = [
  {
    label: 'Constitutional machinery',
    text: 'Begin with the Act that names VJS, fixes the public structure, and separates public system data from private operational data.',
    links: [
      ['VJS Constitution and Machinery Act', 'Legislature/legislature/pdfs/27-vjs-constitution-and-machinery-act-2026.pdf'],
      ['Acts of Union', 'Legislature/legislature/pdfs/01-acts-of-union-2026.pdf'],
    ],
  },
  {
    label: 'Courts, citations, and precedent',
    text: 'Then read how courts, neutral citations, law reports, and binding precedent work.',
    links: [
      ['Judicature Act', 'Legislature/legislature/pdfs/03-judicature-act-2026.pdf'],
      ['Neutral Citations and Law Reporting Act', 'Legislature/legislature/pdfs/16-neutral-citations-and-law-reporting-act-2026.pdf'],
      ['REALM-PC 4', 'Judicature/.justice/pdfs/2026-realm-pc-4.pdf'],
    ],
  },
  {
    label: 'Rights, records, and confidentiality',
    text: 'Then move to the rules for records, standing, due process, confidentiality, and public reasons.',
    links: [
      ['Memory, Records and Archives Act', 'Legislature/legislature/pdfs/07-memory-records-and-archives-act-2026.pdf'],
      ['Rights, Standing and Due Process Act', 'Legislature/legislature/pdfs/12-rights-standing-and-due-process-act-2026.pdf'],
      ['Data Disclosure and Confidentiality Act', 'Legislature/legislature/pdfs/22-data-disclosure-and-confidentiality-act-2026.pdf'],
    ],
  },
  {
    label: 'Local sovereignty and community record',
    text: 'A downloaded or forked copy starts as its own local jurisdiction. Its Principal is sovereign for that copy, initially subscribed to canonical VJS law.',
    links: [
      ['Multi-Jurisdiction and Community Record Act', 'Legislature/legislature/pdfs/30-multi-jurisdiction-and-community-record-act-2026.pdf'],
      ['REALM-PC 17', 'Judicature/.justice/pdfs/2026-realm-pc-17.pdf'],
      ['Public Push Review SI', 'Legislature/statutes/instruments/pdfs/2026-realm-si-7-super-repo-public-push-review.pdf'],
    ],
  },
  {
    label: 'Real-world law and delegated agent authority',
    text: 'Local sovereignty is sovereignty over the local VJS copy, not immunity from real-world law. Agents may refuse or escalate unlawful or unauthorised external acts.',
    links: [
      ['REALM-PC 18', 'Judicature/.justice/pdfs/2026-realm-pc-18.pdf'],
      ['REALM-SC 9', 'Judicature/.justice/pdfs/2026-realm-sc-9.pdf'],
    ],
  },
  {
    label: 'Superrepo change orders and public entrypoint',
    text: 'Changes to the canonical VJS superrepo require a court order unless an existing order authorises the work on all fours. The public entrypoint is VJS.',
    links: [
      ['REALM-PC 19', 'Judicature/.justice/pdfs/2026-realm-pc-19.pdf'],
    ],
  },
];

function renderStart() {
  el('start').innerHTML = `
    <h2>Find the law and case law</h2>
    <p>Start with the constitutional Acts, then move outwards to courts, rights, records, local sovereignty, and the full precedent record.</p>
    <ol>
      ${LAW_START.map(item => `<li><strong>${item.label}.</strong> ${item.text}<br>${item.links.map(([label, path]) => `<a href="${REL}${path}">${label}</a>`).join(' &middot; ')}</li>`).join('')}
    </ol>
    <p><a href="${REL}Legislature/legislature/pdfs/">All Act PDFs</a> &middot; <a href="${REL}Legislature/statutes/instruments/pdfs/">All SI PDFs</a> &middot; <a href="${REL}Judicature/.justice/pdfs/">All judgment PDFs</a> &middot; <a href="${REL}Judicature/.justice/INDEX.md">Citator</a></p>
  `;
}

// Browse base set (from corpus.json) when there is no query, as stored docs shape.
function allDocs() {
  const cases = CORPUS.cases.map(c => ({ kind: 'case', citation: c.citation, title: c.citation,
    series: c.series, court: c.courtLabel, status: c.status, ratio: c.ratioOneLine,
    p_source: c.sourcePath, p_pdf: c.pdfPath, p_jur: c.jurisdiction }));
  const bills = CORPUS.legislation.map(b => ({ kind: 'bill', citation: 'Bill ' + b.no, title: b.shortTitle,
    series: 'BILL', court: 'Legislature', status: b.status, ratio: b.longTitle,
    p_source: b.sourcePath, p_pdf: b.pdfPath, p_stage: b.pipelineStage, p_no: b.no }));
  return [...bills.sort(sortBills), ...cases];
}

const BILL_ORDER = [27, 1, 2, 3, 16, 7, 12, 22, 8, 20, 30];
function sortBills(a, b) {
  const ai = BILL_ORDER.includes(a.p_no) ? BILL_ORDER.indexOf(a.p_no) : 100 + a.p_no;
  const bi = BILL_ORDER.includes(b.p_no) ? BILL_ORDER.indexOf(b.p_no) : 100 + b.p_no;
  return ai - bi;
}

function matchFilter(d) {
  if (FILTER === 'all') return true;
  const [k, v] = FILTER.split(':');
  if (k === 'court') return d.court === v;
  if (k === 'kind') return d.kind === v;
  if (k === 'status') return (d.status || '').toLowerCase().startsWith(v);
  return true;
}

// The legislative pipeline (CHARTER): declared topic -> drafting -> vote -> (deadlock 2nd round) -> Royal Assent.
const STAGES = ['Declared topic', 'Drafting round', 'Vote', 'Royal Assent'];
function pipeline(stage, rounds) {
  const reached = /royal assent/i.test(stage) ? 4 : /vote/i.test(stage) ? 3 : /draft|second/i.test(stage) ? 2 : 1;
  const dots = STAGES.map((s, i) => `<span class="pstep ${i < reached ? 'on' : ''}">${s}</span>`).join('<span class="parrow">&rarr;</span>');
  const dl = rounds >= 2 ? ' <span class="pstep dl">+ 2nd round (deadlock broken)</span>' : '';
  return `<div class="pipe">${dots}${dl}</div>`;
}

// Find the full bill record (for methodology detail) by its number.
function billDetail(no) { return (CORPUS.legislation || []).find(b => b.no === no); }

function card(d) {
  const st = (d.status || 'good-law').toLowerCase().replace(/[^a-z-]/g, '');
  const links = [];
  if (d.p_pdf) links.push(`<a href="${REL}${d.p_pdf}">${d.kind === 'bill' ? 'Act' : 'judgment'} &middot; .pdf</a>`);
  if (!d.p_pdf && d.p_source) links.push(`<a href="${REL}${d.p_source}">source</a>`);
  if (d.kind === 'bill') {
    const b = billDetail(d.p_no) || {};
    const sov = b.sovereignConsultation
      ? `<div class="sov"><strong>Sovereign consultation:</strong> ${b.sovereignConsultation.slice(0, 320)}</div>` : '';
    const note = b.committeeNote ? `<div class="cnote"><strong>Committee note.</strong> ${b.committeeNote.slice(0, 700)}</div>` : '';
    const vote = b.voteRecord ? `<div class="vote">${b.voteRecord.slice(0, 500)}</div>` : '';
    return `<div class="card">
      <div><span class="cite">${d.citation}: ${d.title}</span>
        <span class="badge ${st}">${(d.status || '').replace(/-/g, ' ')}</span></div>
      <span class="court">Legislature &middot; ${b.ayes || ''} ${b.ayes ? 'ayes' : ''}</span>
      ${pipeline(d.p_stage || b.pipelineStage || '', b.rounds || 1)}
      <div class="ratio">${(d.ratio || '').slice(0, 360)}</div>
      <details><summary>methodology &amp; committee record</summary>
        ${vote}${sov}${note}
      </details>
      <div>${links.join('')}</div>
    </div>`;
  }
  const sub = `<span class="court">${d.court}${d.p_jur && d.p_jur.includes('acmeco') ? ' &middot; at acmeco' : ''}</span>`;
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
