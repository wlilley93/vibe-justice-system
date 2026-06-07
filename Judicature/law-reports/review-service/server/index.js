#!/usr/bin/env node
'use strict';
// Neutral annotation + review verb service (zero deps: node:http + node:sqlite).
// Acmeco-shaped verb door: POST /v/<noun>.<verb> -> {...} | {error, code}. Backed by one SQLite
// file. Ports acmeco's review.rs lifecycle (atomic claim, idempotent open, computed SLA,
// terminal-immutable resolve, INV-4: resolving records the DECISION; it never edits the corpus).
// Decoupled from the personal acmeco server; folds in later (schema is a subset; door is identical).
//
//   node server/index.js            (serves on :8790, db at data/realm-review.db)

const http = require('node:http');
const fs = require('node:fs');
const path = require('node:path');
const { DatabaseSync } = require('node:sqlite');

const ROOT = path.resolve(__dirname, '..', '..', '..');          // repo root
const SVC = path.resolve(__dirname, '..');                        // review-service/
const PORT = process.env.PORT || 8790;
const DB_PATH = path.join(SVC, 'data', 'realm-review.db');
fs.mkdirSync(path.dirname(DB_PATH), { recursive: true });

const db = new DatabaseSync(DB_PATH);
db.exec(fs.readFileSync(path.join(SVC, 'db', 'schema.sql'), 'utf8'));

const now = () => new Date().toISOString();
const uid = (p) => `${p}_${Date.now().toString(36)}${Math.floor(Math.random() * 1e6).toString(36)}`;

// --- verbs ---------------------------------------------------------------

const VERBS = {
  // annotation.create: write the [0..1] annotation (CHECKs enforce geometry), then open a review
  // for it (idempotent). One call, one transaction-ish unit (INV-1: one write path).
  'annotation.create'(b) {
    const a = {
      id: uid('anno'), doc_ref: b.doc_ref, type: b.type || 'COMMENT',
      page_number: b.page_number || 1, x: b.x, y: b.y, w: b.w || 0, h: b.h || 0,
      end_x: b.end_x ?? null, end_y: b.end_y ?? null, color: b.color ?? null,
      label: b.label ?? null, criterion: b.criterion ?? null, comment: b.comment ?? '',
      created_by: b.created_by || 'reader', author_kind: b.author_kind || 'HUMAN', created_at: now(),
    };
    db.prepare(`INSERT INTO pdf_annotation
      (id,doc_ref,type,page_number,x,y,w,h,end_x,end_y,color,label,criterion,comment,created_by,author_kind,created_at)
      VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)`).run(
      a.id, a.doc_ref, a.type, a.page_number, a.x, a.y, a.w, a.h, a.end_x, a.end_y,
      a.color, a.label, a.criterion, a.comment, a.created_by, a.author_kind, a.created_at);
    const lane = b.doc_ref.startsWith('bill:') ? 'BILL_COMMENT' : 'JUDGMENT_COMMENT';
    const rev = openReview({ lane, subject_id: a.id, doc_ref: a.doc_ref, opened_by: a.created_by,
      opened_by_source: a.author_kind === 'AGENT' ? 'agent' : 'web',
      payload: { commentText: a.comment, anchor: { page: a.page_number, x: a.x, y: a.y, w: a.w, h: a.h }, criterion: a.criterion } });
    db.prepare('UPDATE pdf_annotation SET review_id=? WHERE id=?').run(rev.reviewId, a.id);
    return { annotationId: a.id, ...rev };
  },
  'annotation.list'(b) {
    const rows = db.prepare('SELECT * FROM pdf_annotation WHERE doc_ref=? ORDER BY page_number, created_at').all(b.doc_ref);
    return { annotations: rows };
  },
  'annotation.remove'(b) {
    db.prepare('DELETE FROM pdf_annotation WHERE id=?').run(b.id);
    return { removed: b.id };
  },
  'review.open'(b) { return openReview(b); },
  'review.claim'(b) {
    const r = db.prepare(`UPDATE review_item SET status='CLAIMED', claimed_by=?, claimed_at=?, updated_at=?
      WHERE id=? AND status='OPEN'`).run(b.claimed_by || 'reviewer', now(), now(), b.reviewId);
    if (r.changes === 0) return { error: 'review not OPEN (already claimed/terminal)', code: 'REVIEW_NOT_OPEN' };
    return { reviewId: b.reviewId, status: 'CLAIMED', claimedBy: b.claimed_by || 'reviewer' };
  },
  'review.resolve'(b) {
    const cur = db.prepare('SELECT status FROM review_item WHERE id=?').get(b.reviewId);
    if (!cur) return { error: 'no such review', code: 'NOT_FOUND' };
    if (['RESOLVED', 'DISMISSED', 'EXPIRED'].includes(cur.status))
      return { reviewId: b.reviewId, status: cur.status, idempotent: true };  // terminal-immutable
    const decision = (b.decision || 'RESOLVE').toUpperCase() === 'DISMISS' ? 'DISMISSED' : 'RESOLVED';
    db.prepare(`UPDATE review_item SET status=?, resolved_by=?, resolution=?, resolution_note=?, resolved_at=?, updated_at=?
      WHERE id=? AND status IN ('OPEN','CLAIMED')`).run(
      decision, b.resolved_by || 'reviewer', b.resolution || '', b.note || '', now(), now(), b.reviewId);
    // INV-4: this records the DECISION only. It never edits the corpus markdown.
    return { reviewId: b.reviewId, status: decision, resolvedBy: b.resolved_by || 'reviewer', enacts_no_change: true };
  },
  'review.list'(b) {
    let sql = 'SELECT * FROM review_item', cl = [], args = [];
    if (b.lane) { cl.push('lane=?'); args.push(b.lane); }
    if (b.status) { cl.push('status=?'); args.push(b.status); }
    if (b.doc_ref) { cl.push('doc_ref=?'); args.push(b.doc_ref); }
    if (cl.length) sql += ' WHERE ' + cl.join(' AND ');
    sql += ' ORDER BY created_at DESC';
    const t = Date.now();
    const items = db.prepare(sql).all(...args).map(r => ({
      id: r.id, lane: r.lane, status: r.status, docRef: r.doc_ref, subjectId: r.subject_id,
      openedBy: r.opened_by, openedBySource: r.opened_by_source, claimedBy: r.claimed_by,
      payload: JSON.parse(r.payload || '{}'), resolution: r.resolution, resolvedBy: r.resolved_by,
      slaBreached: !!(r.sla_due_at && Date.parse(r.sla_due_at) < t),   // computed at read (acmeco INV-6)
      createdAt: r.created_at,
    }));
    return { items, count: items.length };
  },
  'review.unclaim'(b) {
    const r = db.prepare(`UPDATE review_item SET status='OPEN', claimed_by=NULL, claimed_at=NULL, updated_at=? WHERE id=? AND status='CLAIMED'`).run(now(), b.reviewId);
    return r.changes ? { reviewId: b.reviewId, status: 'OPEN', effect: 'released' } : { error: 'not CLAIMED', code: 'NOT_CLAIMED' };
  },
  'review.expire'(b) {
    const r = db.prepare(`UPDATE review_item SET status='EXPIRED', updated_at=? WHERE id=? AND status IN ('OPEN','CLAIMED')`).run(now(), b.reviewId);
    return r.changes ? { reviewId: b.reviewId, status: 'EXPIRED', terminal: true } : { reviewId: b.reviewId, idempotent: true };
  },
  'review.reassign'(b) {
    const r = db.prepare(`UPDATE review_item SET claimed_by=?, updated_at=? WHERE id=? AND status='CLAIMED'`).run(b.toReviewer, now(), b.reviewId);
    return r.changes ? { reviewId: b.reviewId, claimedBy: b.toReviewer, effect: 'ownership-only' } : { error: 'not CLAIMED', code: 'NOT_CLAIMED' };
  },
};

// Idempotent open (acmeco US-REVIEW-01): one non-terminal review per (lane, subject).
function openReview(b) {
  const lane = b.lane, subjectId = b.subject_id;
  const existing = db.prepare(`SELECT id FROM review_item WHERE lane=? AND subject_type='pdf_annotation' AND subject_id=? AND status IN ('OPEN','CLAIMED')`).get(lane, subjectId);
  if (existing) return { reviewId: existing.id, lane, status: 'OPEN', deduped: true };
  const id = uid('rev'), ts = now();
  const sla = b.slaDueInSecs ? new Date(Date.now() + b.slaDueInSecs * 1000).toISOString() : null;
  db.prepare(`INSERT INTO review_item (id,lane,status,opened_by,opened_by_source,subject_type,subject_id,doc_ref,payload,sla_due_at,created_at,updated_at)
    VALUES (?,?,'OPEN',?,?,'pdf_annotation',?,?,?,?,?,?)`).run(
    id, lane, b.opened_by || 'reader', b.opened_by_source || 'web', subjectId, b.doc_ref,
    JSON.stringify(b.payload || {}), sla, ts, ts);
  return { reviewId: id, lane, status: 'OPEN', deduped: false };
}

// --- http ----------------------------------------------------------------

const MIME = { '.html': 'text/html', '.js': 'text/javascript', '.mjs': 'text/javascript', '.css': 'text/css', '.json': 'application/json', '.pdf': 'application/pdf' };
function serveFile(res, file) {
  if (!fs.existsSync(file)) { res.writeHead(404).end('not found'); return; }
  res.writeHead(200, { 'content-type': MIME[path.extname(file)] || 'application/octet-stream' });
  fs.createReadStream(file).pipe(res);
}

const server = http.createServer((req, res) => {
  const u = new URL(req.url, `http://localhost:${PORT}`);
  // verb door
  if (req.method === 'POST' && u.pathname.startsWith('/v/')) {
    let body = '';
    req.on('data', c => { body += c; if (body.length > 1e6) req.destroy(); });
    req.on('end', () => {
      const verb = u.pathname.slice(3);
      const fn = VERBS[verb];
      res.setHeader('content-type', 'application/json');
      if (!fn) { res.writeHead(404).end(JSON.stringify({ error: 'unknown verb', code: 'NO_VERB' })); return; }
      try { res.end(JSON.stringify(fn(body ? JSON.parse(body) : {}))); }
      catch (e) { res.writeHead(400).end(JSON.stringify({ error: e.message, code: 'VERB_ERROR' })); }
    });
    return;
  }
  // corpus PDF proxy (whitelisted dirs under the repo root)
  if (u.pathname === '/pdf') {
    const rel = u.searchParams.get('path') || '';
    if (!/^(\.justice\/pdfs|legislature\/pdfs|ministry-for-business-work-and-skills\/.*\/acmeco\/\.justice\/pdfs)\/[\w.-]+\.pdf$/.test(rel))
      { res.writeHead(400).end('bad path'); return; }
    serveFile(res, path.join(ROOT, rel)); return;
  }
  // static web app + corpus.json
  let p = u.pathname === '/' ? '/index.html' : u.pathname;
  if (p === '/corpus.json') { serveFile(res, path.join(ROOT, 'law-reports', 'corpus.json')); return; }
  serveFile(res, path.join(SVC, 'web', p.replace(/^\//, '')));
});

if (require.main === module) {
  server.listen(PORT, () => console.log(`realm review-service on http://localhost:${PORT}  (db: ${path.relative(ROOT, DB_PATH)})`));
}
module.exports = { VERBS, openReview, db };
