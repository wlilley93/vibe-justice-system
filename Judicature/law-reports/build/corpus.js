'use strict';
// Corpus scanner for The Realm Law Reports & Gazette.
//
// Deterministic, pointer-only projection of the committed markdown (the single source of law,
// CASE-LAW s. 1; [2026] REALM-PC 4; Bill 16 s. 12). This module READS the corpus and returns a
// plain in-memory model; it never writes law and stores no ratio/status/citation as authority -
// every record carries POINTERS (sourcePath, pdfPath) back to the canonical markdown/PDF.
//
// Ports the dual-mode front-matter/section parsing proven in court/scripts/md_to_ruling_json.py.

const fs = require('fs');
const path = require('path');

const ROOT = path.resolve(__dirname, '..', '..', '..'); // repo root (build/ is now Judicature/law-reports/build/)

// --- helpers -------------------------------------------------------------

function read(p) { try { return fs.readFileSync(p, 'utf8'); } catch { return ''; } }
function exists(p) { return fs.existsSync(p); }
function listMd(dir) {
  if (!exists(dir)) return [];
  return fs.readdirSync(dir).filter(f => f.endsWith('.md')).sort();
}
function stripMd(s) {
  return String(s || '')
    .replace(/\[([^\]]+)\]\([^)]+\)/g, '$1')   // [label](url) -> label
    .replace(/\*\*|__|`/g, '')                  // bold / code ticks
    .replace(/^>\s?/gm, '')                     // blockquote markers
    .trim();
}

// Parse --- YAML --- front-matter (the style every committed judgment uses). Returns {meta, body}.
function parseFrontmatter(raw) {
  const meta = {};
  let body = raw;
  if (raw.startsWith('---')) {
    const end = raw.indexOf('\n---', 3);
    if (end !== -1) {
      const fm = raw.slice(3, end).trim();
      body = raw.slice(end + 4);
      for (const line of fm.split('\n')) {
        const m = line.match(/^([A-Za-z_]+):\s*(.*)$/);
        if (m) meta[m[1].toLowerCase()] = m[2].trim().replace(/^"|"$/g, '');
      }
    }
  }
  // table-style front-matter: | **Citation** | [2026] ... |
  for (const m of raw.matchAll(/^\|\s*\*{0,2}([A-Za-z /]+?)\*{0,2}\s*\|\s*(.+?)\s*\|\s*$/gm)) {
    const k = m[1].trim().toLowerCase();
    if (k !== 'field' && meta[k] === undefined) meta[k] = stripMd(m[2].trim());
  }
  return { meta, body };
}

// Split a markdown body into {heading, text} sections by `## `.
function sections(body) {
  const out = [];
  const re = /^##\s+(.*)$/gm;
  const idx = [...body.matchAll(re)];
  for (let i = 0; i < idx.length; i++) {
    const heading = idx[i][1].trim();
    const start = idx[i].index + idx[i][0].length;
    const end = i + 1 < idx.length ? idx[i + 1].index : body.length;
    out.push({ heading, text: body.slice(start, end).trim() });
  }
  return out;
}
function sectionByPrefix(secs, prefixes) {
  for (const s of secs) {
    const h = s.heading.toLowerCase();
    if (prefixes.some(p => h.startsWith(p))) return s.text;
  }
  return '';
}

// --- citator (.justice/INDEX.md) one-line ratios + cites -----------------

// Returns { '[2026] REALM-PC 4': {status, ratio, cites}, ... } from a citator's table rows.
function parseCitator(indexText) {
  const map = {};
  for (const line of String(indexText).split('\n')) {
    if (!line.trim().startsWith('|')) continue;
    const cm = line.match(/\[(\d{4})\]\s*(REALM-SC|REALM-PC|REALM-CA|ENG|CHAN|CC-[A-Z0-9-]+)\s+(\d+)/);
    if (!cm) continue;
    const cols = line.split('|').map(c => c.trim()).filter(Boolean);
    // cols: [citation-link, court, status, ratio, cites]
    const citation = `[${cm[1]}] ${cm[2]} ${cm[3]}`;
    map[citation] = {
      court: cols[1] || '',
      status: cols[2] || '',
      ratio: stripMd(cols[3] || ''),
      cites: stripMd(cols[4] || ''),
    };
  }
  return map;
}

// --- citation grammar (mirror cli/lib/citation.js seriesCode/slug) -------

function slugOf(citation) {
  return citation.replace(/[[\]]/g, '').trim().toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-|-$/g, '');
}
function seriesOf(citation) {
  const m = citation.match(/\]\s*([A-Z-]+(?:\([A-Z0-9-]+\))?)\s+\d+/i) || citation.match(/\]\s*(\S+)/);
  return m ? m[1] : '';
}

// --- the scan ------------------------------------------------------------

const COURT_LABELS = {
  'supreme-court': 'Supreme Court',
  'court-of-appeal': 'Court of Appeal',
  'privy-council': 'Privy Council',
  'county-court': 'County Court',
  'high-court': 'High Court',
};

// Scan a .justice/judgments tree (central, or a repo's local one) into case records.
function scanJudgments(justiceDir, jurisdiction) {
  const cases = [];
  const citator = parseCitator(read(path.join(justiceDir, 'INDEX.md')));
  const judgDir = path.join(justiceDir, 'judgments');
  if (!exists(judgDir)) return cases;
  for (const court of fs.readdirSync(judgDir).sort()) {
    const courtDir = path.join(judgDir, court);
    if (!fs.statSync(courtDir).isDirectory()) continue;
    for (const file of listMd(courtDir)) {
      const full = path.join(courtDir, file);
      const raw = read(full);
      const { meta, body } = parseFrontmatter(raw);
      const citation = meta.citation_id || meta.citation || ('[?] ' + file);
      const secs = sections(body);
      const ratio = sectionByPrefix(secs, ['ratio']) || (citator[citation] || {}).ratio || '';
      const obiter = sectionByPrefix(secs, ['obiter']);
      const lexby = sectionByPrefix(secs, ['lexby']);
      const narrative = secs.filter(s => !/^(ratio|obiter|remedy|lexby|status|per incuriam|citation)/i.test(s.heading))
        .map(s => s.text).join('\n');
      const slug = slugOf(citation);
      const pdfRel = path.relative(ROOT, path.join(justiceDir, 'pdfs', `${slug}.pdf`));
      const citatorRatio = (citator[citation] || {}).ratio || '';
      const sectionRatio = ratio.split('\n')[0].trim().slice(0, 300);
      // Prefer the curated citator one-liner, unless it is absent/short or is a provenance pointer
      // (the County Court citator's column is "reconstituted from", not a ratio).
      const ratioOneLine = (citatorRatio && citatorRatio.length > 15 && !/\.md|reconstituted/i.test(citatorRatio))
        ? citatorRatio : sectionRatio;
      cases.push({
        type: 'case',
        citation,
        date: meta.date || '',
        slug,
        series: seriesOf(citation),
        court,
        courtLabel: COURT_LABELS[court] || court,
        jurisdiction,
        status: (meta.status || (citator[citation] || {}).status || 'good-law').trim(),
        panel: meta.panel ? meta.panel.replace(/^\[|\]$/g, '').split(',').map(s => s.trim().replace(/^"|"$/g, '')).filter(Boolean) : [],
        kind: meta.kind || 'request_for_ruling',
        ratioOneLine,
        cites: (citator[citation] || {}).cites || '',
        sourcePath: path.relative(ROOT, full),
        pdfPath: exists(path.join(ROOT, pdfRel)) ? pdfRel : null,
        searchBody: stripMd([ratio, obiter, lexby, narrative].join('\n')).slice(0, 20000),
      });
    }
  }
  return cases;
}

module.exports = { ROOT, read, exists, listMd, stripMd, parseFrontmatter, sections, sectionByPrefix,
  parseCitator, slugOf, seriesOf, scanJudgments, COURT_LABELS };
