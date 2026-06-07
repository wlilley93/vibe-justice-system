'use strict';
// The Realm Law Reports & Gazette - client app. Loads the deterministic, pointer-only MiniSearch
// index + corpus.json, runs lexical search in the browser (zero tokens, no backend), renders
// pointer cards linking rendered PDFs. Paths are "../../../"-relative so the site works from
// Judicature/law-reports/site/ under the repo root (local or GitHub Pages subpath).

const REL = '../../../'; // site lives at Judicature/law-reports/site/ ; corpus paths are repo-root-relative
let MS = null, CORPUS = null, GRAPH = null, FILTER = 'all';
let GRAPH_NODES = new Map(), GRAPH_OUT = new Map(), GRAPH_IN = new Map();

const el = (id) => document.getElementById(id);

async function boot() {
  const [idxRaw, corpus, graph] = await Promise.all([
    fetch('search-index.json').then(r => r.json()),
    fetch('corpus.json').then(r => r.json()),
    fetch('citator-graph.json').then(r => r.ok ? r.json() : null).catch(() => null),
  ]);
  CORPUS = corpus;
  GRAPH = graph;
  hydrateGraph();
  MS = MiniSearch.loadJS(idxRaw, {
    fields: ['citation', 'title', 'ratio', 'body', 'court', 'status', 'panel', 'cites'],
    storeFields: ['kind', 'citation', 'title', 'series', 'court', 'status', 'ratio',
      'date', 'p_slug', 'p_source', 'p_pdf', 'p_court', 'p_jur', 'p_stage', 'p_no'],
    searchOptions: { boost: { citation: 5, title: 4, ratio: 3 }, prefix: true, fuzzy: 0.2 },
  });
  el('q').addEventListener('input', render);
  el('results').addEventListener('click', openCard);
  el('results').addEventListener('keydown', openCard);
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
    <nav class="quick" aria-label="Law shortcuts">
      <a href="${REL}Legislature/legislature/pdfs/">Act PDFs</a>
      <a href="${REL}Legislature/statutes/instruments/pdfs/">SI PDFs</a>
      <a href="${REL}Judicature/.justice/pdfs/">Judgment PDFs</a>
      <a href="${REL}Judicature/.justice/INDEX.md">Citator</a>
      <a href="citator-graph.json">Graph JSON</a>
    </nav>
    <details>
      <summary>Suggested reading order</summary>
      <ol>
        ${LAW_START.map(item => `<li><strong>${item.label}.</strong> ${item.text}<br>${item.links.map(([label, path]) => `<a href="${REL}${path}">${label}</a>`).join(' &middot; ')}</li>`).join('')}
      </ol>
    </details>
  `;
}

// Browse base set (from corpus.json) when there is no query, as stored docs shape.
function allDocs() {
  const cases = CORPUS.cases.map(c => ({ kind: 'case', citation: c.citation, title: c.citation,
    date: c.date, series: c.series, court: c.courtLabel, status: c.status, ratio: c.ratioOneLine,
    p_source: c.sourcePath, p_pdf: c.pdfPath, p_jur: c.jurisdiction }));
  const bills = CORPUS.legislation.map(b => ({ kind: 'bill', citation: 'Bill ' + b.no, title: b.shortTitle,
    date: b.royalAssent, series: 'BILL', court: 'Legislature', status: b.status, ratio: b.longTitle,
    p_source: b.sourcePath, p_pdf: b.pdfPath, p_stage: b.pipelineStage, p_no: b.no }));
  const instruments = (CORPUS.instruments || []).map(si => ({ kind: 'si', citation: si.citation, title: si.shortTitle,
    date: si.made, series: 'REALM-SI', court: 'Legislature', status: si.status, ratio: si.longTitle,
    p_source: si.sourcePath, p_pdf: si.pdfPath, p_no: si.no }));
  return sortByDate([...instruments, ...bills, ...cases]);
}

const BILL_ORDER = [27, 1, 2, 3, 16, 7, 12, 22, 8, 20, 30];
function dateKey(d) {
  if (d.date) return `${d.date}`;
  const m = `${d.citation || ''}`.match(/\[(\d{4})\].*?\s(\d+)$/);
  return m ? `${m[1]}-00-${String(m[2]).padStart(4, '0')}` : '';
}
function sameDayRank(d) {
  if (d.kind === 'case') return 3000 + citationNumber(d.citation);
  if (d.kind === 'si') return 2000 + citationNumber(d.citation);
  if (d.kind === 'bill') return 1000 + Number(d.p_no || 0);
  return 0;
}
function citationNumber(citation) {
  const m = `${citation || ''}`.match(/(?:REALM-[A-Z]+|Bill)\s+(\d+)$/);
  return m ? Number(m[1]) : 0;
}
function sortByDate(docs) {
  return docs.sort((a, b) => {
    const byDate = dateKey(b).localeCompare(dateKey(a));
    if (byDate) return byDate;
    const bySameDay = sameDayRank(b) - sameDayRank(a);
    if (bySameDay) return bySameDay;
    if (a.kind === 'bill' && b.kind === 'bill') {
      const ai = BILL_ORDER.includes(a.p_no) ? BILL_ORDER.indexOf(a.p_no) : 100 + a.p_no;
      const bi = BILL_ORDER.includes(b.p_no) ? BILL_ORDER.indexOf(b.p_no) : 100 + b.p_no;
      return ai - bi;
    }
    return `${b.citation || ''}`.localeCompare(`${a.citation || ''}`);
  });
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

function hydrateGraph() {
  GRAPH_NODES = new Map(); GRAPH_OUT = new Map(); GRAPH_IN = new Map();
  if (!GRAPH) return;
  for (const n of GRAPH.nodes || []) GRAPH_NODES.set(n.id, n);
  for (const e of GRAPH.edges || []) {
    if (!GRAPH_OUT.has(e.source)) GRAPH_OUT.set(e.source, []);
    if (!GRAPH_IN.has(e.target)) GRAPH_IN.set(e.target, []);
    GRAPH_OUT.get(e.source).push(e);
    GRAPH_IN.get(e.target).push(e);
  }
}

function graphId(d) {
  if (d.kind === 'case') return `case:${d.citation}`;
  if (d.kind === 'bill') return `bill:${d.p_no}`;
  if (d.kind === 'si') return `si:${d.citation}`;
  return '';
}

function nodeHref(n) {
  if (!n) return '';
  return n.pdfPath ? `${REL}${n.pdfPath}` : (n.sourcePath ? `${REL}${n.sourcePath}` : '');
}

function nodeLink(id) {
  const n = GRAPH_NODES.get(id);
  if (!n) return escHtml(id);
  const href = nodeHref(n);
  const label = escHtml(n.label || n.citation || id);
  const tone = graphTone(n);
  const chip = `<span class="node-chip tone-${tone}">${escHtml(toneLabel(tone))}</span>`;
  return href ? `${chip}<a href="${escAttr(href)}">${label}</a>` : `${chip}${label}`;
}

function lineage(d) {
  if (!GRAPH) return '';
  const id = graphId(d);
  const outgoing = (GRAPH_OUT.get(id) || []).slice(0, 8);
  const incoming = (GRAPH_IN.get(id) || []).slice(0, 8);
  const count = (GRAPH_OUT.get(id) || []).length + (GRAPH_IN.get(id) || []).length;
  if (!count) return '';
  const rows = [
    ...outgoing.map(e => `<li><span class="edge-badge tone-${edgeTone(e)}">${escHtml(edgeLabel(e))}</span> ${nodeLink(e.target)}</li>`),
    ...incoming.map(e => `<li>${nodeLink(e.source)} <span class="edge-badge tone-${edgeTone(e)}">${escHtml(edgeLabel(e))}</span></li>`),
  ].join('');
  const clipped = count > outgoing.length + incoming.length ? `<p>${count - outgoing.length - incoming.length} more edge${count - outgoing.length - incoming.length === 1 ? '' : 's'} in citator-graph.json.</p>` : '';
  return `<details class="lineage"><summary>lineage · ${count} public edge${count === 1 ? '' : 's'}</summary><ul>${rows}</ul>${clipped}</details>`;
}

function itemTone(d) {
  if (d.kind === 'bill') return 'act';
  if (d.kind === 'si') return 'si';
  return courtTone(d.court);
}

function courtTone(court) {
  const c = String(court || '').toLowerCase().replace(/[-_]/g, ' ');
  if (c.includes('supreme court') || c.includes('realm sc')) return 'sc';
  if (c.includes('court of appeal') || c.includes('realm ca')) return 'ca';
  if (c.includes('privy council') || c.includes('realm pc')) return 'pc';
  if (c.includes('county court') || c.includes('first instance') || c.includes('high court')) return 'first';
  return 'first';
}

function graphTone(n) {
  if (!n) return 'first';
  if (n.kind === 'bill') return 'act';
  if (n.kind === 'si') return 'si';
  return courtTone(`${n.court || ''} ${n.citation || ''} ${n.sourcePath || ''}`);
}

function edgeTone(e) {
  return graphTone(GRAPH_NODES.get(e.target) || GRAPH_NODES.get(e.source));
}

function toneLabel(tone) {
  return {
    sc: 'SC',
    ca: 'CA',
    first: 'First instance',
    act: 'Act',
    si: 'SI',
    pc: 'PC',
  }[tone] || 'Item';
}

function edgeLabel(e) {
  return e.label || e.type || 'related';
}

function classChip(tone) {
  return `<span class="class-chip tone-${tone}">${escHtml(toneLabel(tone))}</span>`;
}

function card(d) {
  const st = (d.status || 'good-law').toLowerCase().replace(/[^a-z-]/g, '');
  const tone = itemTone(d);
  const href = d.p_pdf ? `${REL}${d.p_pdf}` : (d.p_source ? `${REL}${d.p_source}` : '');
  const when = d.date ? ` &middot; ${d.date}` : '';
  const links = [];
  if (d.p_pdf) links.push(`<a href="${REL}${d.p_pdf}">${d.kind === 'bill' ? 'Act' : d.kind === 'si' ? 'SI' : 'judgment'} &middot; .pdf</a>`);
  if (!d.p_pdf && d.p_source) links.push(`<a href="${REL}${d.p_source}">source</a>`);
  if (d.kind === 'bill') {
    const b = billDetail(d.p_no) || {};
    const sov = b.sovereignConsultation
      ? `<div class="sov"><strong>Sovereign consultation:</strong> ${b.sovereignConsultation.slice(0, 320)}</div>` : '';
    const note = b.committeeNote ? `<div class="cnote"><strong>Committee note.</strong> ${b.committeeNote.slice(0, 700)}</div>` : '';
    const vote = b.voteRecord ? `<div class="vote">${b.voteRecord.slice(0, 500)}</div>` : '';
    return `<div class="card clickable tone-${tone}" data-href="${escAttr(href)}" tabindex="0" role="link" aria-label="Open ${escAttr(d.title)} PDF">
      <div><span class="cite">${d.citation}: ${d.title}</span>
        ${classChip(tone)}
        <span class="badge ${st}">${(d.status || '').replace(/-/g, ' ')}</span></div>
      <span class="court">Legislature${when} &middot; ${b.ayes || ''} ${b.ayes ? 'ayes' : ''}</span>
      ${pipeline(d.p_stage || b.pipelineStage || '', b.rounds || 1)}
      <div class="ratio">${(d.ratio || '').slice(0, 360)}</div>
      ${lineage(d)}
      <details><summary>methodology &amp; committee record</summary>
        ${vote}${sov}${note}
      </details>
      <div>${links.join('')}</div>
    </div>`;
  }
  if (d.kind === 'si') {
    return `<div class="card clickable tone-${tone}" data-href="${escAttr(href)}" tabindex="0" role="link" aria-label="Open ${escAttr(d.title)} PDF">
      <div><span class="cite">${d.citation}: ${d.title}</span>
        ${classChip(tone)}
        <span class="badge ${st}">${(d.status || '').replace(/-/g, ' ')}</span></div>
      <span class="court">Statutory Instrument${when}</span>
      <div class="ratio">${(d.ratio || '').slice(0, 360)}</div>
      ${lineage(d)}
      <div>${links.join('')}</div>
    </div>`;
  }
  const sub = `<span class="court">${d.court}${d.p_jur && d.p_jur.includes('acmeco') ? ' &middot; at acmeco' : ''}</span>`;
  return `<div class="card clickable tone-${tone}" data-href="${escAttr(href)}" tabindex="0" role="link" aria-label="Open ${escAttr(d.citation)} PDF">
    <div><span class="cite">${d.citation}</span>
      ${classChip(tone)}
      <span class="badge ${st}">${d.status || 'good-law'}</span></div>
    ${sub}<span class="court">${when}</span>
    <div class="ratio">${(d.ratio || '').slice(0, 360)}</div>
    ${lineage(d)}
    <div>${links.join('')}</div>
  </div>`;
}

function render() {
  const q = el('q').value.trim();
  let docs;
  if (q) {
    docs = sortByDate(MS.search(q).filter(matchFilter));
  } else {
    docs = allDocs().filter(matchFilter);
  }
  el('meta').textContent = `${docs.length} result${docs.length === 1 ? '' : 's'}`
    + (q ? ` for "${q}"` : ' (browsing)')
    + ` · newest first · ${CORPUS.counts.cases} rulings + ${CORPUS.counts.legislation} Acts + ${(CORPUS.counts.instruments || 0)} SIs in the record`;
  el('results').innerHTML = docs.map(card).join('') || '<p>No matches.</p>';
}

function openCard(e) {
  if (e.type === 'keydown' && e.key !== 'Enter' && e.key !== ' ') return;
  if (e.target.closest('a, button, details, summary')) return;
  const cardEl = e.target.closest('.card[data-href]');
  if (!cardEl || !cardEl.dataset.href) return;
  e.preventDefault();
  window.location.href = cardEl.dataset.href;
}

function escAttr(s) {
  return String(s || '').replace(/&/g, '&amp;').replace(/"/g, '&quot;').replace(/</g, '&lt;');
}

function escHtml(s) {
  return String(s || '').replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');
}

boot();
