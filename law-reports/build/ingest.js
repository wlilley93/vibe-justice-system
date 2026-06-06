'use strict';
// Ingest: scan the realm corpus -> a single deterministic corpus.json (cases[] + legislation[]).
// Deterministic (sorted, no timestamps) so it regenerates byte-identically in lockstep with the
// committed markdown (REALM-PC 4 condition 1 / Bill 16 s. 12(2)).
//
//   node build/ingest.js   ->   law-reports/corpus.json

const fs = require('fs');
const path = require('path');
const { ROOT, scanJudgments } = require('./corpus');
const { parseBills } = require('./parse-bills');

function main() {
  // Central courts (Ministry of Justice).
  const central = scanJudgments(path.join(ROOT, '.justice'), 'Ministry of Justice (central)');
  // County Court at acmeco (the realm-tracked nested repo's local .justice).
  const acmecoJustice = path.join(ROOT, 'ministry-for-business-work-and-skills',
    'engineering-department', 'projects', 'acmeco', '.justice');
  const acmeco = scanJudgments(acmecoJustice, 'County Court at acmeco');

  const cases = [...central, ...acmeco].sort((a, b) => a.citation.localeCompare(b.citation));
  const legislation = parseBills().sort((a, b) => a.no - b.no);

  const seriesCounts = {};
  for (const c of cases) seriesCounts[c.series] = (seriesCounts[c.series] || 0) + 1;

  const out = {
    realm: 'Agent Universe',
    title: 'The Realm Law Reports & Gazette',
    counts: { cases: cases.length, legislation: legislation.length, series: seriesCounts },
    note: 'Derived, pointer-only projection of the committed markdown (CASE-LAW s. 1; [2026] REALM-PC 4; Bill 16 s. 12). The markdown is the law; this is a rebuildable index.',
    cases,
    legislation,
  };
  const dest = path.join(ROOT, 'law-reports', 'corpus.json');
  fs.writeFileSync(dest, JSON.stringify(out, null, 2) + '\n');
  console.log(`corpus.json: ${cases.length} cases + ${legislation.length} Acts -> ${path.relative(ROOT, dest)}`);
  console.log('series:', Object.entries(seriesCounts).map(([k, v]) => `${k}=${v}`).join(' '));
}

main();
