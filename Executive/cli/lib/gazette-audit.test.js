'use strict';

const assert = require('assert');
const fs = require('fs');
const os = require('os');
const path = require('path');
const {
  auditPublicGazetteArtifacts,
  corpusIds,
  linkedGazetteFiles,
  sanitizePublicGazetteText,
} = require('./gazette-audit');

function tmpRoot() {
  return fs.mkdtempSync(path.join(os.tmpdir(), 'vjs-gazette-audit-'));
}

const sensitive = `Push https://github.com/wlilley93/${['agent', 'universe'].join('-')}.git branch codex/example-work at ${'a'.repeat(40)} from /home/example/${['agent', 'universe'].join('-')}.`;
const redacted = sanitizePublicGazetteText(sensitive);
assert(!redacted.includes(['agent', 'universe'].join('-')), 'development repository name should be redacted');
assert(!redacted.includes('codex/example-work'), 'operational branch should be redacted');
assert(!/[0-9a-f]{40}/i.test(redacted), 'commit SHA should be redacted');
assert(!redacted.includes('/home/example'), 'home path should be redacted');

const root = tmpRoot();
const rel = 'Judicature/law-reports/site/corpus.json';
fs.mkdirSync(path.join(root, path.dirname(rel)), { recursive: true });
fs.writeFileSync(path.join(root, rel), JSON.stringify({ note: sensitive }));
let report = auditPublicGazetteArtifacts(root, [rel]);
assert.strictEqual(report.ok, false, 'privacy audit should flag private operational data');

fs.writeFileSync(path.join(root, rel), JSON.stringify({ note: redacted }));
report = auditPublicGazetteArtifacts(root, [rel]);
assert.strictEqual(report.ok, true, 'privacy audit should pass redacted public artifact');

const corpusRel = 'Judicature/law-reports/corpus.json';
const linkedRel = 'Judicature/requests/example.md';
fs.mkdirSync(path.join(root, path.dirname(corpusRel)), { recursive: true });
fs.mkdirSync(path.join(root, path.dirname(linkedRel)), { recursive: true });
fs.writeFileSync(path.join(root, corpusRel), JSON.stringify({
  submissions: [{ sourcePath: linkedRel }],
}));
fs.writeFileSync(path.join(root, linkedRel), sensitive);
assert.deepStrictEqual(linkedGazetteFiles(root, corpusRel), [linkedRel]);
report = auditPublicGazetteArtifacts(root, [corpusRel]);
assert.strictEqual(report.ok, false, 'privacy audit should scan Gazette-linked source files');

fs.writeFileSync(path.join(root, linkedRel), redacted);
report = auditPublicGazetteArtifacts(root, [corpusRel]);
assert.strictEqual(report.ok, true, 'privacy audit should pass redacted linked source files');

assert.deepStrictEqual(corpusIds({
  cases: [{ citation: '[2026] REALM-PC 1' }],
  legislation: [{ no: 31 }],
  instruments: [{ citation: '[2026] REALM-SI 11' }],
  submissions: [{ sourcePath: 'Judicature/requests/example.md' }],
}), [
  'bill:31',
  'case:[2026] REALM-PC 1',
  'si:[2026] REALM-SI 11',
  'submission:Judicature/requests/example.md',
]);

console.log('gazette audit tests OK');
