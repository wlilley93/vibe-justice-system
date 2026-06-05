'use strict';
// Deterministic citator audit for the Vibe Justice System.
//
// This is the engine behind the hard gate (`cdd check-citator`, the pre-commit hook). It does
// NOT use a model: filing and citation integrity are mechanical facts, not judgment calls, so
// they are checked deterministically and fail closed. It catches the two failure modes that
// silently corrupt a jurisdiction:
//   1. citation collisions - the same [YEAR] LEXBY-<TIER> N issued twice (the manual-numbering
//      hazard: two sessions both grab "N+1" and the citator now has two of the same number).
//   2. filing breaks - a ruling file with no citator row, or a citator row with no ruling file
//      (the "judgment returned but never filed" hazard).

const fs = require('fs');
const path = require('path');

const TIER_DIR = { FI: 'high-court', CA: 'appeals-court', SC: 'supreme-court' };

// Walk up from `start` until a directory containing .justice/ is found.
function findRepoRoot(start) {
  let dir = path.resolve(start || process.cwd());
  for (let i = 0; i < 12; i++) {
    if (fs.existsSync(path.join(dir, '.justice'))) return dir;
    const parent = path.dirname(dir);
    if (parent === dir) break;
    dir = parent;
  }
  return null;
}

function citationKey(year, code, n) { return `[${year}] LEXBY-${code} ${n}`; }

// Pull citations only from MARKDOWN TABLE ROWS of the citator (lines starting with `|`), so
// that prose examples and the "how to cite" section never register as phantom duplicates.
function citationsFromIndex(indexText) {
  const re = /\[(\d{4})\]\s*LEXBY-(FI|CA|SC)\s+(\d+)/;
  const out = [];
  for (const rawLine of String(indexText || '').split('\n')) {
    const line = rawLine.trim();
    if (!line.startsWith('|')) continue;
    const m = line.match(re);
    if (!m) continue;
    out.push({ year: +m[1], code: m[2], n: +m[3], key: citationKey(m[1], m[2], m[3]) });
  }
  return out;
}

// Expected ruling filename for a citation, matching the committed convention
// (.justice/judgments/<tier-dir>/<YEAR>-LEXBY-<CODE>-<N>.md).
function expectedRulingPath(root, c) {
  return path.join(root, '.justice', 'judgments', TIER_DIR[c.code], `${c.year}-LEXBY-${c.code}-${c.n}.md`);
}

// Scan the judgments tree for ruling files and parse their citation from the filename.
function rulingFilesOnDisk(root) {
  const files = [];
  for (const code of Object.keys(TIER_DIR)) {
    const dir = path.join(root, '.justice', 'judgments', TIER_DIR[code]);
    if (!fs.existsSync(dir)) continue;
    for (const name of fs.readdirSync(dir)) {
      const m = name.match(/^(\d{4})-LEXBY-(FI|CA|SC)-(\d+)\.md$/i);
      if (!m) continue;
      files.push({ year: +m[1], code: m[2].toUpperCase(), n: +m[3], key: citationKey(m[1], m[2].toUpperCase(), m[3]), path: path.join(dir, name) });
    }
  }
  return files;
}

// Returns { ok, root, problems: [{type, message}] }.
function auditCitator(start) {
  const root = findRepoRoot(start);
  if (!root) return { ok: false, root: null, problems: [{ type: 'no-justice', message: 'no .justice/ directory found from ' + (start || process.cwd()) }] };

  const problems = [];
  const indexPath = fs.existsSync(path.join(root, '.justice', 'INDEX.md'))
    ? path.join(root, '.justice', 'INDEX.md')
    : path.join(root, 'caselaw', 'INDEX.md');

  if (!fs.existsSync(indexPath)) {
    problems.push({ type: 'no-citator', message: 'citator not found (.justice/INDEX.md)' });
    return { ok: false, root, problems };
  }

  const indexText = fs.readFileSync(indexPath, 'utf8');
  const cites = citationsFromIndex(indexText);

  // 1. Collisions: any citation key appearing in more than one table row.
  const counts = new Map();
  for (const c of cites) counts.set(c.key, (counts.get(c.key) || 0) + 1);
  for (const [key, count] of counts) {
    if (count > 1) problems.push({ type: 'collision', message: `citation ${key} appears ${count} times in the citator (collision)` });
  }

  // 2a. Citator row with no ruling file.
  const seen = new Set();
  for (const c of cites) {
    if (seen.has(c.key)) continue;
    seen.add(c.key);
    if (!fs.existsSync(expectedRulingPath(root, c))) {
      problems.push({ type: 'missing-file', message: `citator lists ${c.key} but ruling file is missing (${path.relative(root, expectedRulingPath(root, c))})` });
    }
  }

  // 2b. Ruling file with no citator row.
  const citeKeys = new Set(cites.map(c => c.key));
  for (const f of rulingFilesOnDisk(root)) {
    if (!citeKeys.has(f.key)) {
      problems.push({ type: 'missing-row', message: `ruling file ${path.relative(root, f.path)} (${f.key}) has no row in the citator` });
    }
  }

  return { ok: problems.length === 0, root, problems };
}

module.exports = { auditCitator, citationsFromIndex, findRepoRoot, TIER_DIR };
