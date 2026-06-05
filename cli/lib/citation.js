'use strict';
// Canonical deterministic citation numbering for the Vibe Justice System.
// SOURCE OF TRUTH. The court Workflow scripts inline a minimal mirror of nextN()
// because the Workflow sandbox has no `require`; keep them in sync with this file.
//
// Neutral citation form (operative practice + the citator's "how to cite"): [YEAR] LEXBY-<TIER> N.
// SPEC-LAW S-11(d) abbreviates this as "[YEAR] LEXBY n"; the tiered form is the form used by every
// committed ruling and by .justice/INDEX.md, so it is the form produced here.

const TIER_CODES = {
  'first-instance': 'FI', 'court-of-appeal': 'CA', 'supreme-court': 'SC', 'supreme-council': 'SC',
  fi: 'FI', ca: 'CA', sc: 'SC',
};

function tierCode(tier) {
  if (!tier) throw new Error('tier is required (first-instance | court-of-appeal | supreme-court, or FI/CA/SC)');
  const code = TIER_CODES[String(tier).toLowerCase()];
  if (!code) throw new Error(`unknown tier: ${tier}`);
  return code;
}

// Highest N already issued for this tier+year in the citator text (0 if none).
function highestN(citatorText, code, year) {
  const re = new RegExp(`\\[${year}\\]\\s*LEXBY-${code}\\s+(\\d+)`, 'gi');
  let max = 0, m;
  const text = citatorText || '';
  while ((m = re.exec(text)) !== null) {
    const n = parseInt(m[1], 10);
    if (Number.isFinite(n) && n > max) max = n;
  }
  return max;
}

// Next deterministic citation for a tier. year defaults to the current calendar year.
function nextCitation(citatorText, tier, year) {
  const code = tierCode(tier);
  const yr = year || new Date().getFullYear();
  const n = highestN(citatorText, code, yr) + 1;
  return {
    citation: `[${yr}] LEXBY-${code} ${n}`,
    n,
    code,
    year: yr,
    slug: `${yr}-lexby-${code.toLowerCase()}-${n}`,
  };
}

module.exports = { tierCode, highestN, nextCitation, TIER_CODES };
