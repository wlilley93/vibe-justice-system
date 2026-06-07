'use strict';
// Focused tests for the deterministic bench-name scanner.

const assert = require('assert');
const fs = require('fs');
const os = require('os');
const path = require('path');
const { scanBenchNames, scanText } = require('./bench-name-scan');

let pass = 0;
function t(name, fn) { fn(); pass++; process.stdout.write(`ok - ${name}\n`); }

function tmpRepo() {
  return fs.mkdtempSync(path.join(os.tmpdir(), 'vjs-bench-name-scan-'));
}

function writeFile(p, text) {
  fs.mkdirSync(path.dirname(p), { recursive: true });
  fs.writeFileSync(p, text);
}

t('scanText detects configured real jurist labels', () => {
  const hits = [];
  scanText('panel: ["Hale J", "Lord Bingham", "Baroness Hale"]', (hit) => hits.push(hit.match));
  assert.deepStrictEqual(hits, ['Hale J', 'Lord Bingham', 'Baroness Hale']);
});

t('scanText does not flag adjacent invented VJS names', () => {
  const hits = [];
  scanText('panel: ["Steyne J", "Blackmere J", "Aldermere J"]', (hit) => hits.push(hit.match));
  assert.deepStrictEqual(hits, []);
});

t('scanBenchNames scans judgment source files', () => {
  const root = tmpRepo();
  writeFile(path.join(root, 'Judicature/.justice/judgments/supreme-court/2026-realm-sc-1.md'), [
    '---',
    'citation: "[2026] REALM-SC 1"',
    'panel: ["Hale J", "Coade J"]',
    '---',
    '',
  ].join('\n'));
  const res = scanBenchNames(root, { corpus: false });
  assert.strictEqual(res.ok, false);
  assert.strictEqual(res.findings.length, 1);
  assert.strictEqual(res.findings[0].match, 'Hale J');
  assert.match(res.findings[0].path, /2026-realm-sc-1\.md$/);
});

t('scanBenchNames scans case records in law-report corpus', () => {
  const root = tmpRepo();
  writeFile(path.join(root, 'Judicature/law-reports/corpus.json'), JSON.stringify({
    cases: [
      { type: 'case', citation: '[2026] REALM-SC 2', panel: ['Lord Neuberger'], searchBody: 'clean' },
      { type: 'legislation', title: 'Not scanned', body: 'Bingham J' },
    ],
  }, null, 2));
  const res = scanBenchNames(root, { sources: false });
  assert.strictEqual(res.ok, false);
  assert.strictEqual(res.findings.length, 1);
  assert.strictEqual(res.findings[0].match, 'Lord Neuberger');
  assert.strictEqual(res.findings[0].citation, '[2026] REALM-SC 2');
});

t('scanBenchNames resolves the repo root when invoked from Judicature/law-reports', () => {
  const root = tmpRepo();
  writeFile(path.join(root, 'Judicature/.justice/judgments/privy-council/2026-realm-pc-1.md'), 'panel: ["Coade J"]\n');
  writeFile(path.join(root, 'Judicature/law-reports/corpus.json'), JSON.stringify({
    cases: [{ type: 'case', citation: '[2026] REALM-SC 3', panel: ['Bingham J'] }],
  }, null, 2));
  const res = scanBenchNames(path.join(root, 'Judicature/law-reports'));
  assert.strictEqual(res.ok, false);
  assert(res.scanned.includes('Judicature/law-reports/corpus.json'));
  assert.strictEqual(res.findings[0].match, 'Bingham J');
});

t('scanBenchNames passes on clean judgment records', () => {
  const root = tmpRepo();
  writeFile(path.join(root, 'Judicature/.justice/judgments/privy-council/2026-realm-pc-1.md'), 'panel: ["Coade J"]\n');
  const res = scanBenchNames(root, { corpus: false });
  assert.strictEqual(res.ok, true);
  assert.deepStrictEqual(res.findings, []);
});

process.stdout.write(`\n${pass} passing\n`);
