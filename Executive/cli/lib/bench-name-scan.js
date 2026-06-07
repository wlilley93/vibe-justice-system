'use strict';
// Deterministic bench-name scanner.
//
// VJS benches use invented labels. This check looks for configured real jurist labels in
// judgment records and in the derived law-report case corpus. It is intentionally lexical:
// prohibited labels are concrete strings such as "Hale J" or "Lord Bingham", not fuzzy names.

const fs = require('fs');
const path = require('path');

const PROHIBITED_REAL_JURIST_SURNAMES = [
  'Hale',
  'Bingham',
  'Neuberger',
  'Denning',
  'Diplock',
  'Wilberforce',
  'Scarman',
  'Woolf',
  'Steyn',
  'Hoffmann',
  'Nicholls',
  'Hope',
  'Rodger',
  'Walker',
  'Mance',
  'Kerr',
  'Dyson',
  'Sumption',
  'Carnwath',
  'Clarke',
  'Phillips',
  'Reed',
  'Hodge',
  'Lloyd-Jones',
  'Lloyd Jones',
  'Arden',
  'Black',
  'Briggs',
  'Leggatt',
  'Sales',
  'Burrows',
  'Richards',
  'Hamblen',
  'Stephens',
  'Rose',
  'Toulson',
];

const JUDICIAL_SUFFIXES = ['J', 'JJ', 'LJ', 'LJJ', 'CJ', 'LCJ', 'MR', 'P', 'DPSC', 'PSC'];
const JUDICIAL_TITLES = ['Lord', 'Lady', 'Baroness', 'Sir', 'Dame'];

function exists(p) { return fs.existsSync(p); }

function regexEscape(s) {
  return String(s).replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

function labelRegex() {
  const surname = PROHIBITED_REAL_JURIST_SURNAMES
    .map(regexEscape)
    .sort((a, b) => b.length - a.length)
    .join('|');
  const suffix = JUDICIAL_SUFFIXES.map(regexEscape).join('|');
  const title = JUDICIAL_TITLES.map(regexEscape).join('|');
  return new RegExp(`\\b(?:(${surname})\\s+(?:${suffix})\\.?|(?:${title})\\s+(${surname}))\\b`, 'g');
}

const LABEL_RE = labelRegex();

function findRepoRoot(start) {
  let dir = path.resolve(start || process.cwd());
  for (let i = 0; i < 64; i++) {
    if (exists(path.join(dir, 'Judicature', '.justice'))) return dir;
    if (exists(path.join(dir, '.justice'))) {
      if (path.basename(dir) === 'Judicature') return path.dirname(dir);
      return dir;
    }
    const parent = path.dirname(dir);
    if (parent === dir) break;
    dir = parent;
  }
  return path.resolve(start || process.cwd());
}

function walkFiles(dir, predicate) {
  if (!exists(dir)) return [];
  const out = [];
  for (const name of fs.readdirSync(dir).sort()) {
    const full = path.join(dir, name);
    const st = fs.statSync(full);
    if (st.isDirectory()) out.push(...walkFiles(full, predicate));
    else if (!predicate || predicate(full)) out.push(full);
  }
  return out;
}

function judgmentSourceTargets(root) {
  const targets = [];
  const seen = new Set();
  for (const dir of [
    path.join(root, 'Judicature', '.justice', 'judgments'),
    path.join(root, '.justice', 'judgments'),
    path.join(root, 'Judicature', 'community', 'caselaw'),
  ]) {
    const real = path.resolve(dir);
    if (!exists(real) || seen.has(real)) continue;
    seen.add(real);
    targets.push(...walkFiles(real, (p) => p.endsWith('.md')).map((p) => ({ kind: 'judgment-source', path: p })));
  }
  return targets;
}

function corpusTargets(root) {
  return [
    path.join(root, 'Judicature', 'law-reports', 'corpus.json'),
    path.join(root, 'Judicature', 'law-reports', 'site', 'corpus.json'),
  ].filter(exists).map((p) => ({ kind: 'law-report-corpus', path: p }));
}

function contextLine(line) {
  return String(line || '').trim().slice(0, 240);
}

function scanText(text, visit) {
  const lines = String(text || '').split(/\r?\n/);
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    LABEL_RE.lastIndex = 0;
    let m;
    while ((m = LABEL_RE.exec(line))) {
      visit({
        jurist: m[1] || m[2],
        match: m[0],
        line: i + 1,
        column: m.index + 1,
        context: contextLine(line),
      });
    }
  }
}

function scanSourceFile(root, target) {
  const findings = [];
  const rel = path.relative(root, target.path);
  scanText(fs.readFileSync(target.path, 'utf8'), (hit) => {
    findings.push({
      type: 'prohibited-bench-name',
      source: target.kind,
      path: rel,
      ...hit,
    });
  });
  return findings;
}

function scanValue(root, rel, pointer, value, meta, findings) {
  if (typeof value === 'string') {
    scanText(value, (hit) => {
      findings.push({
        type: 'prohibited-bench-name',
        source: 'law-report-corpus',
        path: rel,
        pointer,
        citation: meta.citation || '',
        ...hit,
      });
    });
  } else if (Array.isArray(value)) {
    value.forEach((v, i) => scanValue(root, rel, `${pointer}[${i}]`, v, meta, findings));
  } else if (value && typeof value === 'object') {
    for (const key of Object.keys(value).sort()) {
      scanValue(root, rel, `${pointer}.${key}`, value[key], meta, findings);
    }
  }
}

function scanCorpusFile(root, target) {
  const rel = path.relative(root, target.path);
  const findings = [];
  let data;
  try {
    data = JSON.parse(fs.readFileSync(target.path, 'utf8'));
  } catch (e) {
    return [{
      type: 'invalid-corpus-json',
      source: 'law-report-corpus',
      path: rel,
      message: e.message,
    }];
  }
  const cases = Array.isArray(data) ? data.filter((r) => r && r.type === 'case') : (data.cases || []);
  cases.forEach((record, i) => {
    if (!record || record.type !== 'case') return;
    const meta = { citation: record.citation || `cases[${i}]` };
    scanValue(root, rel, `cases[${i}]`, record, meta, findings);
  });
  return findings;
}

function scanBenchNames(start, opts = {}) {
  const root = path.resolve(opts.root || findRepoRoot(start));
  const includeSources = opts.sources !== false;
  const includeCorpus = opts.corpus !== false;
  const targets = [
    ...(includeSources ? judgmentSourceTargets(root) : []),
    ...(includeCorpus ? corpusTargets(root) : []),
  ];
  const findings = [];
  for (const target of targets) {
    if (target.kind === 'law-report-corpus') findings.push(...scanCorpusFile(root, target));
    else findings.push(...scanSourceFile(root, target));
  }
  if (targets.length === 0) {
    findings.push({
      type: 'no-targets',
      message: 'no judgment source or law-report corpus targets found',
      path: '.',
    });
  }
  return {
    ok: findings.length === 0,
    root,
    scanned: targets.map((t) => path.relative(root, t.path)),
    findings,
  };
}

module.exports = {
  scanBenchNames,
  scanText,
  PROHIBITED_REAL_JURIST_SURNAMES,
  JUDICIAL_SUFFIXES,
  JUDICIAL_TITLES,
};
