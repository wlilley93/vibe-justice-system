'use strict';
// Canonical deterministic citation numbering for the Vibe Justice System.
// SOURCE OF TRUTH. The court Workflow scripts inline a minimal mirror of nextCitation()
// because the Workflow sandbox has no `require`; keep them in sync with this file.
//
// Provenance-based neutral citation (CASE-LAW s. 11(d), as amended): the series code
// encodes the court's authority level, not a flat house style. Form: [YEAR] <CODE> N.
//
//   Supreme Court     -> REALM-SC   (realm apex; sole enactor of CASE-LAW)
//   Privy Council     -> REALM-PC   (realm constitutional first instance; leapfrogs to SC)
//   Court of Appeal   -> REALM-CA   (realm appellate)
//   High Court        -> <DIVISION> (cites by its Division: Engineering -> ENG, Chancery -> CHAN)
//   County Court      -> CC-<REPO>  (cites by its repo: acmeco -> CC-ACMECO)
//
// The three central (REALM-*) courts share the realm-authority anchor; the High Court
// cites by Division; the County Court cites by repo. This dissolves cross-repo collisions
// (every local court has its own series) while keeping one citator.

// Division name -> series code (High Court). Extend as Divisions are constituted.
const DIVISION_CODES = {
  'engineering division': 'ENG',
  'engineering': 'ENG',
  'legal division (chancery)': 'CHAN',
  'legal division': 'CHAN',
  'chancery': 'CHAN',
};

function divisionCode(division) {
  if (!division) throw new Error('high-court citations require a division (e.g. "Engineering Division")');
  const code = DIVISION_CODES[String(division).toLowerCase().trim()];
  if (code) return code;
  // Fallback: derive an UPPERCASE token from the division's first word.
  return String(division).toUpperCase().replace(/[^A-Z0-9]+/g, ' ').trim().split(' ')[0];
}

// Sanitise a repo name into a citation token: acmeco -> ACMECO, jarvis-voice -> JARVIS-VOICE.
function repoCode(repo) {
  if (!repo) throw new Error('county-court citations require a repo (e.g. "acmeco")');
  return String(repo).toUpperCase().replace(/[^A-Z0-9]+/g, '-').replace(/^-|-$/g, '');
}

// Resolve the series code for a (tier, {division, repo}). Accepts the long tier
// names and the short forms used across the renderer and workflows.
function seriesCode(tier, opts = {}) {
  if (!tier) throw new Error('tier is required');
  const t = String(tier).toLowerCase().replace(/_/g, '-');
  switch (t) {
    case 'supreme-court': case 'supreme-council': case 'supreme': case 'sc':
      return 'REALM-SC';
    case 'privy-council': case 'privy': case 'pc':
      return 'REALM-PC';
    case 'court-of-appeal': case 'appeal': case 'appeals-court': case 'ca':
      return 'REALM-CA';
    case 'high-court': case 'high': case 'hc':
      return divisionCode(opts.division);
    case 'county-court': case 'county': case 'cc':
      return `CC-${repoCode(opts.repo)}`;
    default:
      throw new Error(`unknown tier: ${tier}`);
  }
}

// Highest N already issued for this exact series code + year in the citator (0 if none).
function highestN(citatorText, code, year) {
  // Escape regex metacharacters in the code (e.g. the '-' and '(' that codes may carry).
  const esc = String(code).replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  const re = new RegExp(`\\[${year}\\]\\s*${esc}\\s+(\\d+)`, 'gi');
  let max = 0, m;
  const text = citatorText || '';
  while ((m = re.exec(text)) !== null) {
    const n = parseInt(m[1], 10);
    if (Number.isFinite(n) && n > max) max = n;
  }
  return max;
}

// Next deterministic citation for a (tier, opts). year defaults to the current year.
//   nextCitation(citator, 'supreme-court')                       -> [YEAR] REALM-SC N
//   nextCitation(citator, 'high-court', { division: 'Engineering Division' }) -> [YEAR] ENG N
//   nextCitation(citator, 'county-court', { repo: 'acmeco' })      -> [YEAR] CC-ACMECO N
function nextCitation(citatorText, tier, opts = {}) {
  // Back-compat: allow nextCitation(text, tier, yearNumber).
  if (typeof opts === 'number') opts = { year: opts };
  const code = seriesCode(tier, opts);
  const yr = opts.year || new Date().getFullYear();
  const n = highestN(citatorText, code, yr) + 1;
  return {
    citation: `[${yr}] ${code} ${n}`,
    n,
    code,
    year: yr,
    slug: `${yr}-${code.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-|-$/g, '')}-${n}`,
  };
}

module.exports = { seriesCode, divisionCode, repoCode, highestN, nextCitation, DIVISION_CODES };
