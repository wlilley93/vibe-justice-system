'use strict';
// Minimal deterministic-numbering tests. Run: node lib/citation.test.js
const assert = require('assert');
const { nextCitation, highestN, seriesCode, parentTag, siDisplay } = require('./citation');

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

// Statutory instruments (REALM-PC 11, Form C hybrid): one flat REALM-SI code + a derived parent tag.
const siCitator = `
| [2026] REALM-SI 1 (under Bill 21) | made | ... |
| [2026] REALM-SI 2 (under Bill 23) | made | ... |
`;

t('REALM-SI is one flat series (one code, one authority level)', () => {
  assert.strictEqual(seriesCode('statutory-instrument'), 'REALM-SI');
  assert.strictEqual(seriesCode('si'), 'REALM-SI');
  // flat ordinal, ignoring the parent tag in existing rows
  assert.strictEqual(highestN(siCitator, 'REALM-SI', 2026), 2);
  const next = nextCitation(siCitator, 'statutory-instrument', { year: 2026 });
  assert.strictEqual(next.citation, '[2026] REALM-SI 3');
  assert.strictEqual(next.slug, '2026-realm-si-3');           // slug stays flat (no parent)
});

t('parentTag is derived from the enabling recital only, sorted ascending', () => {
  const single = 'In exercise of the powers conferred by section 13A of Bill 21, the Ministry of Security and Integrity makes the following Regulations.\n\n## 1. Scope';
  assert.strictEqual(parentTag(single), '(under Bill 21)');
  const multi = 'In exercise of the powers conferred by section 13A of Bill 21 and section 6A of Bill 13, the Ministry makes the following Regulations.';
  assert.strictEqual(parentTag(multi), '(under Bill 13 and Bill 21)');   // ascending, not recital order
  // a Bill mentioned OUTSIDE the enabling clause is not a parent
  const noise = 'In exercise of the powers conferred by section 13A of Bill 21, the Ministry makes the following Regulations.\n\nEnforced under Bill 13 section 6.';
  assert.strictEqual(parentTag(noise), '(under Bill 21)');
  assert.strictEqual(parentTag('no recital here'), '');
});

t('siDisplay composes the flat ordinal with the derived tag', () => {
  const text = 'In exercise of the powers conferred by section 13A of Bill 21, the Ministry makes the following Regulations.';
  assert.strictEqual(siDisplay('[2026] REALM-SI 1', text), '[2026] REALM-SI 1 (under Bill 21)');
  assert.strictEqual(siDisplay('[2026] REALM-SI 1', ''), '[2026] REALM-SI 1');  // no tag if no recital
});

process.stdout.write(`\n${pass} passing\n`);
