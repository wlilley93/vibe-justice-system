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
    case 'statutory-instrument': case 'statutory_instrument': case 'si':
      // Subordinate legislation. ONE code, ONE authority level ("subordinate law"), per
      // [2026] REALM-PC 11 (Form C): a flat REALM-SI ordinal, no division/repo and no per-parent
      // sub-ordinal. The parent linkage is a derived tag (parentTag), never part of the ordinal.
      return 'REALM-SI';
    default:
      throw new Error(`unknown tier: ${tier}`);
  }
}

// Derive the SI parent tag from the instrument's own recitals (the Form C hybrid of
// [2026] REALM-PC 11). The mandatory enabling recital - "In exercise of the powers conferred by
// section X of Bill NN[ and section Y of Bill MM], the <office> makes the following ..." (Bill 14
// s. 6(i), s. 18) - is the single source; this reads ONLY the Bills named in that clause (not every
// Bill mentioned elsewhere), sorts them ascending, and renders " (under Bill 21[ and Bill 13])".
// Pure, deterministic, zero model tokens; the recited power governs, the tag is a derived pointer.
function parentTag(instrumentText) {
  if (!instrumentText) return '';
  // Scope = the enabling clause, from "conferred by" to the maker's "makes the following ...".
  // Primary: stop at "makes the following" (the standard recital formula) so an intervening
  // "of THE <X> Act (Bill N)" is kept in scope. Fallback: stop at ", the <maker> makes" with the
  // comma REQUIRED, so a bare "of the" inside the clause is not mistaken for the maker boundary.
  let m = /In exercise of the powers?\s+conferred by\s+([\s\S]*?)\bmakes the following\b/i.exec(instrumentText);
  if (!m) m = /In exercise of the powers?\s+conferred by\s+([\s\S]*?),\s+the\b[\s\S]*?\bmakes\b/i.exec(instrumentText);
  const scope = m ? m[1] : '';
  const bills = new Set();
  const re = /\bBill\s+(\d+)\b/gi;
  let mm;
  while ((mm = re.exec(scope)) !== null) bills.add(parseInt(mm[1], 10));
  const sorted = [...bills].sort((a, b) => a - b).map(n => `Bill ${n}`);
  if (sorted.length === 0) return '';
  const joined = sorted.length === 1
    ? sorted[0]
    : sorted.slice(0, -1).join(', ') + ' and ' + sorted[sorted.length - 1];
  return `(under ${joined})`;
}

// Compose the canonical SI short-cite: the flat ordinal always shown with its derived parent tag.
//   siDisplay('[2026] REALM-SI 1', instrumentText) -> '[2026] REALM-SI 1 (under Bill 21)'
function siDisplay(citation, instrumentText) {
  const tag = parentTag(instrumentText);
  return tag ? `${citation} ${tag}` : citation;
}

// The closed SI status vocabulary (Bill 16 s. 15, per [2026] REALM-PC 11). Gate-checked.
const SI_STATUSES = ['made', 'in-force', 'amended', 'revoked', 'spent'];

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

module.exports = { seriesCode, divisionCode, repoCode, highestN, nextCitation, parentTag, siDisplay, SI_STATUSES, DIVISION_CODES };
