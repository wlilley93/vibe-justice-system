'use strict';
// Legislation projection for The Realm Law Reports & Gazette.
// Reads legislature/bills/NN-*.md (+ the Order Paper) into BillRecords. Pointer-only; the
// markdown stays the source of truth. The pipeline stage is derived from the header comment.

const fs = require('fs');
const path = require('path');
const { ROOT, read, stripMd, sections, sectionByPrefix } = require('./corpus');

const BILLS_DIR = path.join(ROOT, 'Legislature', 'legislature', 'bills');
const INSTRUMENTS_DIR = path.join(ROOT, 'Legislature', 'statutes', 'instruments');

function pipelineStage(status, outcome, rounds) {
  const s = `${status} ${outcome}`.toLowerCase();
  // Check "presented/awaiting" BEFORE the assent check: the status string
  // "presented-for-royal-assent" contains the substring "assent" but is NOT yet assented.
  if (/presented|awaiting/.test(s)) return rounds >= 2 ? 'Presented for Royal Assent (after 2nd round)' : 'Presented for Royal Assent';
  if (/enacted|in-force|assented|royal assent granted/.test(s)) return 'Royal Assent';
  if (/vote|passed/.test(s)) return 'Vote';
  if (/deadlock/.test(s)) return 'Second drafting round';
  if (/committee|drafting/.test(s)) return 'Drafting';
  return 'Presented for Royal Assent';
}

function parseBills() {
  if (!fs.existsSync(BILLS_DIR)) return [];
  const bills = [];
  for (const file of fs.readdirSync(BILLS_DIR).filter(f => /^\d{2}-.*\.md$/.test(f)).sort()) {
    const raw = read(path.join(BILLS_DIR, file));
    const no = parseInt(file.slice(0, 2), 10);
    // header comment fields (parsed independently so extra segments like royal-assent: are tolerated)
    const hc = (raw.match(/<!--([\s\S]*?)-->/g) || []).join(' ');
    const field = (k) => { const m = hc.match(new RegExp(k + ':\\s*([^|>]+)', 'i')); return m ? m[1].trim() : ''; };
    const status = field('status') || 'presented-for-royal-assent';
    const outcome = field('outcome');
    const ayes = field('ayes');
    const royalAssent = field('royal-assent');
    const rounds = parseInt(field('drafting rounds') || '1', 10);
    const titleM = raw.match(/^#\s+(.*)$/m);
    const shortTitle = titleM ? titleM[1].trim() : file.replace(/\.md$/, '');
    const longM = raw.match(/(?:\*\*An Act\*\*|long title[\s\S]{0,40}?An Act)\s*([\s\S]*?)(?:\n\n|\n##|\n\*\()/i);
    const secs = sections(raw);
    const committeeNote = sectionByPrefix(secs, ['committee note']);
    const voteRecord = sectionByPrefix(secs, ['vote record']);
    const flags = sectionByPrefix(secs, ['flags']);
    const sovM = /sovereign consultation[^:]*:\s*([\s\S]*?)(?:\n\n|$)/i.exec(flags || raw);
    bills.push({
      type: 'bill',
      no,
      slug: file.replace(/\.md$/, ''),
      shortTitle,
      longTitle: stripMd(longM ? longM[1] : '').slice(0, 600),
      status, outcome, ayes, rounds, royalAssent,
      pipelineStage: pipelineStage(status, outcome, rounds),
      committeeNote: stripMd(committeeNote).slice(0, 4000),
      voteRecord: stripMd(voteRecord).slice(0, 2000),
      sovereignConsultation: sovM ? stripMd(sovM[1]).slice(0, 600) : '',
      sourcePath: path.relative(ROOT, path.join(BILLS_DIR, file)),
      pdfPath: fs.existsSync(path.join(ROOT, 'Legislature', 'legislature', 'pdfs', `${file.replace(/\.md$/, '')}.pdf`))
        ? path.join('Legislature', 'legislature', 'pdfs', `${file.replace(/\.md$/, '')}.pdf`) : null,
      searchBody: stripMd(raw.replace(/<!--[\s\S]*?-->/g, '')).slice(0, 20000),
    });
  }
  return bills;
}

function mdField(raw, key) {
  const re = new RegExp(`^\\*\\*${key}:\\*\\*\\s*(.+)$`, 'im');
  const m = raw.match(re);
  return m ? stripMd(m[1]).trim() : '';
}

function parseInstruments() {
  if (!fs.existsSync(INSTRUMENTS_DIR)) return [];
  const instruments = [];
  for (const file of fs.readdirSync(INSTRUMENTS_DIR).filter(f => /^2026-realm-si-\d+-.*\.md$/.test(f)).sort()) {
    const raw = read(path.join(INSTRUMENTS_DIR, file));
    const noM = file.match(/^2026-realm-si-(\d+)-/);
    const no = noM ? parseInt(noM[1], 10) : 0;
    const titleM = raw.match(/^#\s+(.*)$/m);
    const shortTitle = titleM ? titleM[1].trim() : file.replace(/\.md$/, '');
    const citationRaw = mdField(raw, 'Citation') || `[2026] REALM-SI ${no}`;
    const citationM = citationRaw.match(/\[\d{4}\]\s+REALM-SI\s+\d+/);
    const citation = citationM ? citationM[0] : citationRaw;
    const status = mdField(raw, 'Status') || 'made';
    const made = mdField(raw, 'Made');
    const procedure = mdField(raw, 'Procedure');
    const force = mdField(raw, 'Coming into force');
    const recitals = sectionByPrefix(sections(raw), ['recitals']);
    const longTitle = stripMd(recitals || raw.replace(/^#.*$/m, '')).slice(0, 600);
    const slug = file.replace(/\.md$/, '');
    const pdfRel = path.join('Legislature', 'statutes', 'instruments', 'pdfs', `${slug}.pdf`);
    instruments.push({
      type: 'instrument',
      no,
      slug,
      citation,
      shortTitle,
      longTitle,
      status,
      made,
      procedure,
      comingIntoForce: force,
      sourcePath: path.relative(ROOT, path.join(INSTRUMENTS_DIR, file)),
      pdfPath: fs.existsSync(path.join(ROOT, pdfRel)) ? pdfRel : null,
      searchBody: stripMd(raw).slice(0, 20000),
    });
  }
  return instruments;
}

module.exports = { parseBills, parseInstruments, pipelineStage };
