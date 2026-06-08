'use strict';

const fs = require('fs');
const path = require('path');

const PUBLIC_VJS_REMOTE_URLS = new Set([
  'https://github.com/wlilley93/vibe-justice-system',
  'https://github.com/wlilley93/vibe-justice-system.git',
  'git@github.com:wlilley93/vibe-justice-system.git',
  'ssh://git@github.com/wlilley93/vibe-justice-system.git',
]);

const KNOWN_FIELDS = [
  'AUTHORISED_OUTWARD_ACT',
  'AUTHORISED_BY',
  'AUTHORISED_AT',
  'AUTHORISED_REMOTE_URL',
  'AUTHORISED_REMOTE_REF',
  'AUTHORISED_LOCAL_SHA',
  'INTENDED_EFFECT',
  'LEGAL_AUTHORITY',
  'PUBLIC_DATA_BOUNDARY_CHECK',
  'PRIVATE_BACKUP_STATE',
  'CHECKS_RUN',
];

function isPublicVjsRemote(remoteUrl) {
  return PUBLIC_VJS_REMOTE_URLS.has(String(remoteUrl || '').trim());
}

function parseWarrantText(text) {
  const fields = {};
  for (const rawLine of String(text || '').split(/\r?\n/)) {
    const line = rawLine.trim();
    if (!line || line.startsWith('#')) continue;
    const eq = line.indexOf('=');
    const colon = line.indexOf(':');
    let idx = -1;
    if (eq >= 0 && (colon < 0 || eq < colon)) idx = eq;
    else if (colon >= 0) idx = colon;
    if (idx <= 0) continue;
    const key = line.slice(0, idx).trim();
    if (!KNOWN_FIELDS.includes(key)) continue;
    fields[key] = line.slice(idx + 1).trim();
  }
  return fields;
}

function candidateWarrantPaths(root) {
  const fixed = [
    'Judicature/ministry-of-justice/reasons-ledger/outward-act-authorisations/public-vjs-publish.md',
    '.vjs/checkpoints/public-vjs-publish-authorisation.env',
  ];
  const rels = [...fixed];
  const privateDir = path.join(root, '_private', 'release-warrants');
  if (fs.existsSync(privateDir)) {
    for (const name of fs.readdirSync(privateDir).sort()) {
      const rel = path.join('_private', 'release-warrants', name);
      const full = path.join(root, rel);
      if (fs.statSync(full).isFile()) rels.push(rel);
    }
  }
  return rels;
}

function summariseWarrant(root, relPath) {
  const fullPath = path.join(root, relPath);
  if (!fs.existsSync(fullPath)) return null;
  const fields = parseWarrantText(fs.readFileSync(fullPath, 'utf8'));
  return {
    path: relPath,
    act: fields.AUTHORISED_OUTWARD_ACT || null,
    authorisedBy: fields.AUTHORISED_BY || null,
    authorisedAt: fields.AUTHORISED_AT || null,
    scopedRemoteUrl: fields.AUTHORISED_REMOTE_URL || null,
    scopedRemoteRef: fields.AUTHORISED_REMOTE_REF || null,
    scopedLocalSha: fields.AUTHORISED_LOCAL_SHA || null,
    intendedEffect: fields.INTENDED_EFFECT || null,
    legalAuthority: fields.LEGAL_AUTHORITY || null,
    publicDataBoundaryCheck: fields.PUBLIC_DATA_BOUNDARY_CHECK || null,
    privateBackupState: fields.PRIVATE_BACKUP_STATE || null,
    checksRun: fields.CHECKS_RUN || null,
  };
}

function loadReleaseWarrants(root) {
  return candidateWarrantPaths(root)
    .map((rel) => summariseWarrant(root, rel))
    .filter(Boolean);
}

function warrantMatch(record, proposed = {}) {
  const reasons = [];
  if (record.act !== 'public-vjs-publish') {
    return { match: false, reasons: [`wrong act: ${record.act || 'missing'}`] };
  }
  if (!record.authorisedBy) reasons.push('missing AUTHORISED_BY');
  if (!record.authorisedAt) reasons.push('missing AUTHORISED_AT');

  const scoped = [
    ['remote URL', record.scopedRemoteUrl, proposed.remoteUrl],
    ['remote ref', record.scopedRemoteRef, proposed.remoteRef],
    ['local SHA', record.scopedLocalSha, proposed.localSha],
  ];
  for (const [label, expected, actual] of scoped) {
    if (expected && actual && expected !== actual) reasons.push(`${label} mismatch: expected ${expected}, got ${actual}`);
    else if (expected && !actual) reasons.push(`${label} scoped to ${expected}; no proposed ${label} supplied`);
  }

  return { match: reasons.length === 0, reasons };
}

function releaseWarrantReport(root, proposed = {}) {
  const records = loadReleaseWarrants(root).map((record) => {
    const check = warrantMatch(record, proposed);
    return { ...record, match: check.match, matchReasons: check.reasons };
  });
  const publicVjsRemote = isPublicVjsRemote(proposed.remoteUrl);
  const requiresWarrant = publicVjsRemote;
  const matchingRecords = records.filter((record) => record.match);
  return {
    ok: !requiresWarrant || matchingRecords.length > 0,
    root,
    proposed: {
      remoteUrl: proposed.remoteUrl || null,
      remoteRef: proposed.remoteRef || null,
      localSha: proposed.localSha || null,
    },
    publicVjsRemote,
    requiresWarrant,
    matchingRecords: matchingRecords.map((record) => record.path),
    records,
    note: 'release warrants are authority evidence for the pre-push gate; they do not create legal force by CLI output alone',
  };
}

module.exports = {
  isPublicVjsRemote,
  parseWarrantText,
  loadReleaseWarrants,
  releaseWarrantReport,
  warrantMatch,
};
