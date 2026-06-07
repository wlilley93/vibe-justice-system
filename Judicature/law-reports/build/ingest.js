'use strict';
// Ingest: scan the realm corpus -> a single deterministic corpus.json.
// Deterministic (sorted, no timestamps) so it regenerates byte-identically in lockstep with the
// committed markdown (REALM-PC 4 condition 1 / Bill 16 s. 12(2)).
//
//   node build/ingest.js   ->   law-reports/corpus.json

const fs = require('fs');
const path = require('path');
const { ROOT, scanJudgments } = require('./corpus');
const { parseBills, parseInstruments } = require('./parse-bills');

function main() {
  // Central courts only (Supreme Court, Court of Appeal, Privy Council). Per the VJS (Constitution
  // and Machinery) Act 2026 (Bill 27) s. 14, the public record comprises the central courts' law;
  // LOCAL court judgments (County Courts CC-<repo>, High Court Divisions) record jurisdiction-local
  // precedent that stays in its own repo and is EXCLUDED from the public projections, as it may carry
  // personal or operational facts. (The law of every judgment is public; the facts are sealed/local.)
  const central = scanJudgments(path.join(ROOT, 'Judicature', '.justice'), 'Ministry of Justice (central)');

  const cases = [...central].sort((a, b) => a.citation.localeCompare(b.citation));
  const legislation = parseBills().sort((a, b) => a.no - b.no);
  const instruments = parseInstruments().sort((a, b) => a.no - b.no);

  const seriesCounts = {};
  for (const c of cases) seriesCounts[c.series] = (seriesCounts[c.series] || 0) + 1;

  const out = {
    realm: 'Vibe Justice System (VJS)',
    title: 'The Realm Law Reports & Gazette',
    counts: { cases: cases.length, legislation: legislation.length, instruments: instruments.length, series: seriesCounts },
    note: 'Derived, pointer-only projection of the committed markdown (CASE-LAW s. 1; [2026] REALM-PC 4; Bill 16 s. 12). The markdown is the law; this is a rebuildable index.',
    cases,
    legislation,
    instruments,
  };
  const dest = path.join(ROOT, 'Judicature', 'law-reports', 'corpus.json');
  fs.writeFileSync(dest, JSON.stringify(out, null, 2) + '\n');
  console.log(`corpus.json: ${cases.length} cases + ${legislation.length} Acts + ${instruments.length} SIs -> ${path.relative(ROOT, dest)}`);
  console.log('series:', Object.entries(seriesCounts).map(([k, v]) => `${k}=${v}`).join(' '));
}

main();
