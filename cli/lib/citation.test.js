'use strict';
// Minimal deterministic-numbering tests. Run: node lib/citation.test.js
const assert = require('assert');
const { nextCitation, highestN, seriesCode } = require('./citation');

let pass = 0;
function t(name, fn) { fn(); pass++; process.stdout.write(`ok - ${name}\n`); }

const citator = `
| [2026] REALM-SC 1 | supreme | good-law | ... |
| [2026] REALM-SC 2 | supreme | good-law | ... |
| [2026] REALM-PC 1 | privy   | good-law | ... |
| [2026] REALM-CA 1 | appeal  | good-law | ... |
| [2026] ENG 3      | high    | good-law | ... |
| [2026] CC-ACMECO 7 | county  | good-law | ... |
`;

t('seriesCode maps each court level (provenance scheme)', () => {
  assert.strictEqual(seriesCode('supreme-court'), 'REALM-SC');
  assert.strictEqual(seriesCode('privy-council'), 'REALM-PC');
  assert.strictEqual(seriesCode('court-of-appeal'), 'REALM-CA');
  assert.strictEqual(seriesCode('high-court', { division: 'Engineering Division' }), 'ENG');
  assert.strictEqual(seriesCode('high-court', { division: 'Legal Division (Chancery)' }), 'CHAN');
  assert.strictEqual(seriesCode('county-court', { repo: 'acmeco' }), 'CC-ACMECO');
  assert.strictEqual(seriesCode('county-court', { repo: 'jarvis-voice' }), 'CC-JARVIS-VOICE');
});

t('highestN reads the max for an exact series + year', () => {
  assert.strictEqual(highestN(citator, 'REALM-SC', 2026), 2);
  assert.strictEqual(highestN(citator, 'REALM-PC', 2026), 1);
  assert.strictEqual(highestN(citator, 'REALM-CA', 2026), 1);
  assert.strictEqual(highestN(citator, 'ENG', 2026), 3);
  assert.strictEqual(highestN(citator, 'CC-ACMECO', 2026), 7);
  assert.strictEqual(highestN(citator, 'REALM-SC', 2027), 0);
});

t('nextCitation increments deterministically per series', () => {
  assert.strictEqual(nextCitation(citator, 'supreme-court', { year: 2026 }).citation, '[2026] REALM-SC 3');
  assert.strictEqual(nextCitation(citator, 'privy-council', { year: 2026 }).citation, '[2026] REALM-PC 2');
  assert.strictEqual(nextCitation(citator, 'high-court', { division: 'Engineering Division', year: 2026 }).citation, '[2026] ENG 4');
  assert.strictEqual(nextCitation(citator, 'county-court', { repo: 'acmeco', year: 2026 }).citation, '[2026] CC-ACMECO 8');
});

t('empty citator yields N=1 and a clean slug', () => {
  assert.strictEqual(nextCitation('', 'supreme-court', { year: 2026 }).citation, '[2026] REALM-SC 1');
  assert.strictEqual(nextCitation(null, 'county-court', { repo: 'acmeco', year: 2026 }).slug, '2026-cc-acmeco-1');
});

t('new year resets the sequence', () => {
  assert.strictEqual(nextCitation(citator, 'court-of-appeal', { year: 2027 }).citation, '[2027] REALM-CA 1');
});

process.stdout.write(`\n${pass} passing\n`);
