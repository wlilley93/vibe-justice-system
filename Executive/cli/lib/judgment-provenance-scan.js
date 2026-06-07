'use strict';
// Deterministic guard for new judgment records.
//
// This does not prove a court sat. No local string scan can prove that. It fails closed on
// the narrower mechanical hazard: a new judgment file entering the public record without an
// explicit adjudication provenance record saying which workflow or registrar authority produced it.

const fs = require('fs');
const path = require('path');
const { spawnSync } = require('child_process');
const { findRepoRoot } = require('./citator-audit');

const JUDGMENT_RE = /^(?:Judicature\/)?\.justice\/judgments\/(?:supreme-court|privy-council|court-of-appeal)\/\d{4}-realm-(?:sc|pc|ca)-\d+\.md$/;

function repoRelative(root, file) {
  return path.relative(root, path.resolve(root, file)).replace(/\\/g, '/');
}

function parseStatusLine(line) {
  const status = line.slice(0, 2);
  const rest = line.slice(3).trim();
  if (!rest) return null;
  const pathPart = rest.includes(' -> ') ? rest.split(' -> ').pop() : rest;
  return { status, path: pathPart };
}

function gitNewFiles(root) {
  const res = spawnSync('git', ['status', '--porcelain', '--untracked-files=all'], {
    cwd: root,
    encoding: 'utf8',
  });
  if (res.status !== 0) return [];
  return res.stdout.split(/\r?\n/)
    .map(parseStatusLine)
    .filter(Boolean)
    .filter((entry) => entry.status.includes('A') || entry.status === '??')
    .map((entry) => entry.path);
}

function changedJudgmentFiles(root, opts = {}) {
  const files = opts.files || gitNewFiles(root);
  return files
    .map((file) => repoRelative(root, file))
    .filter((file) => JUDGMENT_RE.test(file));
}

function hasCourtWorkflowProvenance(text) {
  return /adjudication_provenance:\s*["']?court-workflow["']?/i.test(text) &&
    /workflow:\s*["']?(?:Judicature\/court\/workflows\/)?(?:first-instance|court-of-appeal|supreme-court)\.js["']?/i.test(text) &&
    /workflow_run(?:_id)?:\s*["']?[^"'\s][^"'\n]*/i.test(text);
}

function hasRegistrarProvenance(text) {
  return /adjudication_provenance:\s*["']?authorised-registrar["']?/i.test(text) &&
    /registrar_authority:\s*["']?[^"'\s][^"'\n]*/i.test(text) &&
    /registrar_note:\s*["']?[^"'\s][^"'\n]*/i.test(text);
}

function scanJudgmentProvenance(start, opts = {}) {
  const root = path.resolve(opts.root || findRepoRoot(start) || start || process.cwd());
  const files = changedJudgmentFiles(root, opts);
  const findings = [];
  for (const rel of files) {
    const full = path.join(root, rel);
    let text = '';
    try {
      text = fs.readFileSync(full, 'utf8');
    } catch (e) {
      findings.push({ type: 'judgment-provenance-unreadable', path: rel, message: e.message });
      continue;
    }
    if (hasCourtWorkflowProvenance(text) || hasRegistrarProvenance(text)) continue;
    findings.push({
      type: 'missing-judgment-provenance',
      path: rel,
      message: 'new judgment file lacks adjudication_provenance court-workflow metadata or authorised-registrar metadata',
    });
  }
  return {
    ok: findings.length === 0,
    root,
    scanned: files,
    findings,
  };
}

module.exports = {
  scanJudgmentProvenance,
  changedJudgmentFiles,
  hasCourtWorkflowProvenance,
  hasRegistrarProvenance,
};
