'use strict';
// Minimal deterministic-numbering tests. Run: node lib/citation.test.js
const assert = require('assert');
const { nextCitation, highestN, tierCode } = require('./citation');

let pass = 0;
function t(name, fn) { fn(); pass++; process.stdout.write(`ok - ${name}\n`); }

const citator = `
| [[2026] LEXBY-SC 1](2026-LEXBY-SC-1.md) | supreme | good-law | ... |
| [[2026] LEXBY-FI 1](2026-LEXBY-FI-1.md) | first-instance | good-law | ... |
| [[2026] LEXBY-CA 2](2026-LEXBY-CA-2.md) | appeal | good-law | ... |
| [[2026] LEXBY-CA 1](2026-LEXBY-CA-1.md) | appeal | good-law | ... |
`;

t('tierCode maps long + short forms', () => {
  assert.strictEqual(tierCode('first-instance'), 'FI');
  assert.strictEqual(tierCode('court-of-appeal'), 'CA');
  assert.strictEqual(tierCode('supreme-court'), 'SC');
  assert.strictEqual(tierCode('SC'), 'SC');
});

t('highestN reads the max for a tier+year', () => {
  assert.strictEqual(highestN(citator, 'CA', 2026), 2);
  assert.strictEqual(highestN(citator, 'FI', 2026), 1);
  assert.strictEqual(highestN(citator, 'SC', 2026), 1);
  assert.strictEqual(highestN(citator, 'CA', 2027), 0);
});

t('nextCitation increments deterministically, tiered form', () => {
  assert.strictEqual(nextCitation(citator, 'first-instance', 2026).citation, '[2026] LEXBY-FI 2');
  assert.strictEqual(nextCitation(citator, 'court-of-appeal', 2026).citation, '[2026] LEXBY-CA 3');
  assert.strictEqual(nextCitation(citator, 'supreme-court', 2026).citation, '[2026] LEXBY-SC 2');
});

t('empty citator yields N=1', () => {
  assert.strictEqual(nextCitation('', 'first-instance', 2026).citation, '[2026] LEXBY-FI 1');
  assert.strictEqual(nextCitation(null, 'SC', 2026).slug, '2026-lexby-sc-1');
});

t('new year resets the sequence', () => {
  assert.strictEqual(nextCitation(citator, 'CA', 2027).citation, '[2027] LEXBY-CA 1');
});

process.stdout.write(`\n${pass} passing\n`);
