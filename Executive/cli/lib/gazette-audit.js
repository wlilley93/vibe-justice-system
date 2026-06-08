'use strict';

const fs = require('fs');
const path = require('path');
const { spawnSync } = require('child_process');

const DEFAULT_GAZETTE_ARTIFACTS = [
  'Judicature/law-reports/corpus.json',
  'Judicature/law-reports/site/corpus.json',
  'Judicature/law-reports/site/search-index.json',
];

function escapeRegExp(value) {
  return String(value).replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

const DEVELOPMENT_REPO_NAME = ['agent', 'universe'].join('-');
const PRIVATE_HOST_LABELS = [
  ['onyx', 'prod'].join('-'),
  ['bee', 'link'].join(''),
];

const PUBLIC_GAZETTE_REDACTIONS = [
  {
    name: 'private home path',
    pattern: /\/home\/[A-Za-z0-9._-]+(?:\/[^\s"')\]}]*)?/g,
    replacement: '[local path]',
  },
  {
    name: 'private record path',
    pattern: /(^|[\s"'(])_private\/[^\s"')\]}]*/g,
    replacement: (_m, prefix) => `${prefix}[private record]`,
  },
  {
    name: 'development repository URL',
    pattern: new RegExp(`https://github\\.com/wlilley93/${escapeRegExp(DEVELOPMENT_REPO_NAME)}(?:\\.git)?`, 'g'),
    replacement: '[development repository]',
  },
  {
    name: 'development repository name',
    pattern: new RegExp(`\\bwlilley93/${escapeRegExp(DEVELOPMENT_REPO_NAME)}\\b|\\b${escapeRegExp(DEVELOPMENT_REPO_NAME)}\\b`, 'g'),
    replacement: '[development repository]',
  },
  {
    name: 'operational branch ref',
    pattern: /\b(?:codex|origin|publish|community)\/[A-Za-z0-9._/-]+/g,
    replacement: '[repository ref]',
  },
  {
    name: 'git refs heads path',
    pattern: /\brefs\/heads\/[A-Za-z0-9._/-]+/g,
    replacement: 'refs/heads/[branch]',
  },
  {
    name: 'commit sha',
    pattern: /\b[0-9a-f]{40}\b/gi,
    replacement: '[commit]',
  },
  {
    name: 'workflow run id',
    pattern: /\b(?:workflow\s+)?runs?\s+\d{8,}\b/gi,
    replacement: 'workflow runs [run]',
  },
  {
    name: 'private host label',
    pattern: new RegExp(`\\b(?:tail${'scale'}|${PRIVATE_HOST_LABELS.map(escapeRegExp).join('|')})\\b`, 'gi'),
    replacement: '[private host]',
  },
  {
    name: 'secret-like assignment',
    pattern: /\b(?:API|AUTH|ACCESS|SECRET|TOKEN|KEY)[A-Z0-9_]*\s*[=:]\s*["']?[A-Za-z0-9_./+=-]{8,}["']?/g,
    replacement: '[secret assignment]',
  },
];

const PUBLIC_GAZETTE_SOURCE_AUDIT_RULES = [
  PUBLIC_GAZETTE_REDACTIONS.find((rule) => rule.name === 'private home path'),
  PUBLIC_GAZETTE_REDACTIONS.find((rule) => rule.name === 'private record path'),
  PUBLIC_GAZETTE_REDACTIONS.find((rule) => rule.name === 'development repository URL'),
  PUBLIC_GAZETTE_REDACTIONS.find((rule) => rule.name === 'development repository name'),
  {
    name: 'development branch ref',
    pattern: /\bcodex\/(?!hooks\.json\b)[A-Za-z0-9._/-]+/g,
    replacement: '[repository ref]',
  },
  {
    name: 'private host label',
    pattern: new RegExp(`\\b(?:${PRIVATE_HOST_LABELS.map(escapeRegExp).join('|')})\\b`, 'gi'),
    replacement: '[private host]',
  },
  PUBLIC_GAZETTE_REDACTIONS.find((rule) => rule.name === 'secret-like assignment'),
].filter(Boolean);

const TEXT_FIELD_NAMES = new Set([
  'summary',
  'searchBody',
  'ratioOneLine',
  'longTitle',
  'committeeNote',
  'voteRecord',
  'sovereignConsultation',
  'ratio',
  'body',
  'title',
  'status',
  'route',
  'filedBy',
  'cites',
  'panel',
  'obiter',
  'remedy',
  'appealRoute',
  'appeal_route',
]);

function sanitizePublicGazetteText(value) {
  let out = String(value || '');
  for (const rule of PUBLIC_GAZETTE_REDACTIONS) {
    out = out.replace(rule.pattern, rule.replacement);
  }
  return out;
}

function sanitizePublicGazetteValue(value, key = '') {
  if (typeof value === 'string') {
    return TEXT_FIELD_NAMES.has(key) ? sanitizePublicGazetteText(value) : value;
  }
  if (Array.isArray(value)) return value.map((item) => sanitizePublicGazetteValue(item, key));
  if (value && typeof value === 'object') {
    const out = {};
    for (const [childKey, childValue] of Object.entries(value)) {
      out[childKey] = sanitizePublicGazetteValue(childValue, childKey);
    }
    return out;
  }
  return value;
}

function scanTextForPrivateGazetteData(text, relPath = 'input', rules = PUBLIC_GAZETTE_REDACTIONS) {
  const findings = [];
  for (const rule of rules) {
    const re = new RegExp(rule.pattern.source, rule.pattern.flags.includes('g') ? rule.pattern.flags : `${rule.pattern.flags}g`);
    let match;
    while ((match = re.exec(text)) !== null) {
      findings.push({
        path: relPath,
        type: rule.name,
        match: match[0].slice(0, 160),
        index: match.index,
      });
      if (match[0].length === 0) re.lastIndex += 1;
    }
  }
  return findings;
}

function isPdfPath(relPath) {
  return /\.pdf$/i.test(relPath);
}

function readGazetteScanText(root, relPath) {
  const full = path.join(root, relPath);
  if (isPdfPath(relPath)) {
    const res = spawnSync('pdftotext', [full, '-'], {
      cwd: root,
      encoding: 'utf8',
      stdio: 'pipe',
      maxBuffer: 20 * 1024 * 1024,
    });
    if (res.status !== 0) {
      throw new Error(`pdftotext failed for ${relPath}: ${res.stderr || res.stdout || `exit ${res.status}`}`);
    }
    return res.stdout;
  }
  return fs.readFileSync(full, 'utf8');
}

function linkedGazetteFiles(root, corpusRel = 'Judicature/law-reports/corpus.json') {
  const full = path.join(root, corpusRel);
  if (!fs.existsSync(full)) return [];
  const corpus = JSON.parse(fs.readFileSync(full, 'utf8'));
  const rels = new Set();
  for (const group of ['cases', 'legislation', 'instruments', 'submissions']) {
    for (const record of corpus[group] || []) {
      if (record.sourcePath) rels.add(record.sourcePath);
      if (record.pdfPath) rels.add(record.pdfPath);
    }
  }
  return [...rels].sort();
}

function auditPublicGazetteArtifacts(root, files = DEFAULT_GAZETTE_ARTIFACTS) {
  const scanned = [];
  const findings = [];
  const queue = new Set(files);
  const linked = new Set();
  for (const rel of files) {
    if (rel.endsWith('corpus.json')) {
      for (const linkedRel of linkedGazetteFiles(root, rel)) {
        queue.add(linkedRel);
        linked.add(linkedRel);
      }
    }
  }
  for (const rel of [...queue].sort()) {
    const full = path.join(root, rel);
    if (!fs.existsSync(full)) continue;
    scanned.push(rel);
    try {
      const rules = linked.has(rel) ? PUBLIC_GAZETTE_SOURCE_AUDIT_RULES : PUBLIC_GAZETTE_REDACTIONS;
      findings.push(...scanTextForPrivateGazetteData(readGazetteScanText(root, rel), rel, rules));
    } catch (e) {
      findings.push({
        path: rel,
        type: 'scan failure',
        match: e.message,
        index: 0,
      });
    }
  }
  return {
    ok: findings.length === 0,
    scanned,
    findings,
  };
}

function corpusIds(corpus) {
  return [
    ...(corpus.cases || []).map((r) => `case:${r.citation}`),
    ...(corpus.legislation || []).map((r) => `bill:${r.no}`),
    ...(corpus.instruments || []).map((r) => `si:${r.citation}`),
    ...(corpus.submissions || []).map((r) => `submission:${r.sourcePath}`),
  ].sort();
}

function corpusLatestDates(corpus) {
  const dates = [];
  for (const record of corpus.cases || []) if (record.date) dates.push(record.date);
  for (const record of corpus.legislation || []) if (record.royalAssent || record.date) dates.push(record.royalAssent || record.date);
  for (const record of corpus.instruments || []) if (record.made || record.date) dates.push(record.made || record.date);
  for (const record of corpus.submissions || []) if (record.date) dates.push(record.date);
  return [...new Set(dates)].sort();
}

module.exports = {
  DEFAULT_GAZETTE_ARTIFACTS,
  PUBLIC_GAZETTE_REDACTIONS,
  auditPublicGazetteArtifacts,
  corpusIds,
  corpusLatestDates,
  linkedGazetteFiles,
  sanitizePublicGazetteText,
  sanitizePublicGazetteValue,
  scanTextForPrivateGazetteData,
};
