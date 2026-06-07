// Realm Law - annotation + review client (vanilla ESM + PDF.js).
// Renders a judgment/Act PDF, lets a reader drop a COMMENT point or a BOUNDING_BOX region; the
// annotation is stored with normalized [0..1] geometry (acmeco INV-6) and raises a review item.
// The right panel is the review queue (claim/resolve). Resolving records the decision only (INV-4).
import * as pdfjs from './vendor/pdf.min.mjs';
pdfjs.GlobalWorkerOptions.workerSrc = './vendor/pdf.worker.min.mjs';

const $ = (s) => document.querySelector(s);
const api = (verb, body) => fetch('/v/' + verb, { method: 'POST', headers: { 'content-type': 'application/json' }, body: JSON.stringify(body || {}) }).then(r => r.json());

let CUR = null;           // { docRef, pdfUrl, viewports:[] }
let tool = () => document.querySelector('input[name=tool]:checked').value;

async function boot() {
  const corpus = await fetch('/corpus.json').then(r => r.json());
  const docs = [
    ...corpus.cases.filter(c => c.pdfPath).map(c => ({ ref: 'judgment:' + c.slug, title: c.citation, sub: c.courtLabel, pdf: c.pdfPath })),
    ...corpus.legislation.filter(b => b.pdfPath).map(b => ({ ref: 'bill:' + b.no, title: 'Bill ' + b.no, sub: b.shortTitle, pdf: b.pdfPath })),
  ];
  $('#docs').innerHTML = docs.map((d, i) => `<div class="doc" data-i="${i}"><b>${d.title}</b><small>${d.sub}</small></div>`).join('');
  $('#docs').querySelectorAll('.doc').forEach(el => el.onclick = () => {
    $('#docs').querySelectorAll('.doc').forEach(x => x.classList.remove('sel')); el.classList.add('sel');
    open(docs[+el.dataset.i]);
  });
}

async function open(d) {
  CUR = { docRef: d.ref, pdfUrl: '/pdf?path=' + encodeURIComponent(d.pdf), viewports: [] };
  $('#vhint').textContent = d.ref;
  const pages = $('#pages'); pages.innerHTML = '<div class="hint">rendering&hellip;</div>';
  const pdf = await pdfjs.getDocument(CUR.pdfUrl).promise;
  pages.innerHTML = '';
  const scale = 1.3;
  for (let n = 1; n <= pdf.numPages; n++) {
    const page = await pdf.getPage(n);
    const vp = page.getViewport({ scale });
    CUR.viewports[n] = vp;
    const wrap = document.createElement('div');
    wrap.className = 'pagewrap'; wrap.style.width = vp.width + 'px'; wrap.style.height = vp.height + 'px';
    const canvas = document.createElement('canvas'); canvas.width = vp.width; canvas.height = vp.height;
    const overlay = document.createElement('div'); overlay.className = 'overlay'; overlay.dataset.page = n;
    wrap.append(canvas, overlay); pages.append(wrap);
    page.render({ canvasContext: canvas.getContext('2d'), viewport: vp });
    wireOverlay(overlay, vp, n);
  }
  await refreshAnnotations(); await refreshQueue();
}

// store: px -> [0..1] (origin top-left, axes right/down; PDF.js viewport already in screen space)
function wireOverlay(overlay, vp, page) {
  let down = null;
  overlay.onmousedown = (e) => { if (tool() === 'BOUNDING_BOX') down = rel(e, overlay); };
  overlay.onmouseup = async (e) => {
    const r = rel(e, overlay);
    if (tool() === 'COMMENT') {
      await create(page, 'COMMENT', r.x / vp.width, r.y / vp.height, 0, 0);
    } else if (down) {
      const x = Math.min(down.x, r.x), y = Math.min(down.y, r.y);
      const w = Math.abs(r.x - down.x), h = Math.abs(r.y - down.y);
      if (w > 4 || h > 4) await create(page, 'BOUNDING_BOX', x / vp.width, y / vp.height, w / vp.width, h / vp.height);
      down = null;
    }
  };
}
function rel(e, el) { const b = el.getBoundingClientRect(); return { x: e.clientX - b.left, y: e.clientY - b.top }; }

async function create(page, type, x, y, w, h) {
  const comment = prompt('Comment for review (' + type + ' on ' + CUR.docRef + ', p.' + page + '):');
  if (comment == null || comment.trim() === '') return;
  const res = await api('annotation.create', { doc_ref: CUR.docRef, type, page_number: page, x, y, w, h, comment, created_by: 'reader' });
  if (res.error) { alert(res.error); return; }
  await refreshAnnotations(); await refreshQueue();
}

async function refreshAnnotations() {
  const { annotations } = await api('annotation.list', { doc_ref: CUR.docRef });
  document.querySelectorAll('.overlay').forEach(o => o.querySelectorAll('.anno').forEach(a => a.remove()));
  for (const a of annotations) {
    const ov = document.querySelector(`.overlay[data-page="${a.page_number}"]`); if (!ov) continue;
    const vp = CUR.viewports[a.page_number]; const d = document.createElement('div');
    if (a.type === 'COMMENT') { d.className = 'anno pt'; d.style.left = (a.x * vp.width) + 'px'; d.style.top = (a.y * vp.height) + 'px'; }
    else { d.className = 'anno'; d.style.left = (a.x * vp.width) + 'px'; d.style.top = (a.y * vp.height) + 'px'; d.style.width = (a.w * vp.width) + 'px'; d.style.height = (a.h * vp.height) + 'px'; }
    d.innerHTML = `<span class="tip">${(a.comment || '').replace(/</g, '&lt;')}</span>`;
    ov.append(d);
  }
}

async function refreshQueue() {
  const { items } = await api('review.list', { doc_ref: CUR.docRef });
  const q = $('#queue');
  q.innerHTML = `<div class="hint">${items.length} comment(s) on this document</div>` + items.map(it => {
    const open = it.status === 'OPEN' || it.status === 'CLAIMED';
    const btns = open
      ? (it.status === 'OPEN' ? `<button data-claim="${it.id}">claim</button>` : '')
        + `<button data-res="${it.id}">resolve</button><button data-dis="${it.id}">dismiss</button>`
      : '';
    return `<div class="qitem ${it.status}"><div class="st">${it.status} &middot; p.${it.payload.anchor?.page || '?'} &middot; ${it.lane}</div>
      <div>${(it.payload.commentText || '').replace(/</g, '&lt;')}</div>
      <div style="margin-top:.3rem">${btns}</div></div>`;
  }).join('');
  q.querySelectorAll('[data-claim]').forEach(b => b.onclick = async () => { await api('review.claim', { reviewId: b.dataset.claim, claimed_by: 'reviewer' }); refreshQueue(); });
  q.querySelectorAll('[data-res]').forEach(b => b.onclick = async () => { await api('review.resolve', { reviewId: b.dataset.res, decision: 'RESOLVE', note: prompt('Resolution note (records the decision; does not edit the law):') || '' }); refreshQueue(); });
  q.querySelectorAll('[data-dis]').forEach(b => b.onclick = async () => { await api('review.resolve', { reviewId: b.dataset.dis, decision: 'DISMISS' }); refreshQueue(); });
}

boot();
