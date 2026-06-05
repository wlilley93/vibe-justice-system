#!/usr/bin/env node
// VJS Judgment Renderer
// Converts a VJS ruling JSON into a formally structured court judgment PDF.
//
// Usage:
//   node index.js <ruling.json> [output.pdf]
//   node index.js --stdin < ruling.json
//   node index.js --test         (renders a sample judgment to /tmp/vjs-test.pdf)
//
// The input JSON must match the schema documented in README.md.

'use strict'

const puppeteer = require('puppeteer')
const Handlebars = require('handlebars')
const fs = require('fs')
const path = require('path')

// ---------------------------------------------------------------------------
// Paths
// ---------------------------------------------------------------------------

const TEMPLATES_DIR = path.join(__dirname, 'templates')
const ASSETS_DIR = path.join(__dirname, 'assets')

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

// Format a citation slug for use in filenames: "[2026] LEXBY 1" -> "2026-lexby-1"
function citationSlug(citation) {
  return citation.replace(/[\[\]\s]+/g, '-').replace(/^-|-$/g, '').toLowerCase()
}

// Format a date as "5 June 2026"
function formatDate(d) {
  if (!d) {
    const now = new Date()
    const day = now.getDate()
    const month = now.toLocaleString('en-GB', { month: 'long' })
    const year = now.getFullYear()
    return `${day} ${month} ${year}`
  }
  if (typeof d === 'string') return d
  const day = d.getDate()
  const month = d.toLocaleString('en-GB', { month: 'long' })
  const year = d.getFullYear()
  return `${day} ${month} ${year}`
}

// Resolve the logo path for a given tier. Returns a data URI if the file exists,
// otherwise returns the placeholder SVG data URI.
function resolveLogoDataUri(tier) {
  const candidates = [
    path.join(ASSETS_DIR, `${tier}-logo.png`),
    path.join(ASSETS_DIR, `${tier}-logo.svg`),
    path.join(ASSETS_DIR, `${tier}-logo.jpg`),
  ]
  for (const candidate of candidates) {
    if (fs.existsSync(candidate)) {
      const ext = path.extname(candidate).slice(1)
      const mime = ext === 'svg' ? 'image/svg+xml' : `image/${ext}`
      const data = fs.readFileSync(candidate).toString('base64')
      return `data:${mime};base64,${data}`
    }
  }
  // Fallback: placeholder SVG (scales of justice)
  return placeholderLogoDataUri(tier)
}

// Inline SVG scales of justice as a data URI - used when no real logo is provided.
function placeholderLogoDataUri(tier) {
  const colors = {
    'first-instance': '#1a2744',
    'court-of-appeal': '#1a2744',
    'supreme-court': '#1a2744',
  }
  const c = colors[tier] || '#1a2744'
  const svg = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 120 120" width="120" height="120">
  <style>
    .beam { fill: none; stroke: ${c}; stroke-width: 3; stroke-linecap: round; }
    .pan  { fill: none; stroke: ${c}; stroke-width: 2.5; }
    .base { fill: ${c}; }
  </style>
  <!-- Pillar -->
  <line class="beam" x1="60" y1="18" x2="60" y2="90"/>
  <!-- Base -->
  <rect class="base" x="44" y="88" width="32" height="5" rx="2"/>
  <rect class="base" x="38" y="93" width="44" height="4" rx="2"/>
  <!-- Crossbeam -->
  <line class="beam" x1="20" y1="28" x2="100" y2="28"/>
  <!-- Left pan suspension -->
  <line class="beam" x1="26" y1="28" x2="20" y2="50"/>
  <line class="beam" x1="20" y1="28" x2="26" y2="50"/>
  <!-- Right pan suspension -->
  <line class="beam" x1="94" y1="28" x2="100" y2="50"/>
  <line class="beam" x1="100" y1="28" x2="94" y2="50"/>
  <!-- Left pan -->
  <path class="pan" d="M14 50 Q20 62 26 50" stroke-width="2.5" fill="none"/>
  <!-- Right pan -->
  <path class="pan" d="M94 50 Q100 62 106 50" stroke-width="2.5" fill="none"/>
  <!-- Crown atop pillar (3 circles) -->
  <circle class="base" cx="52" cy="14" r="3"/>
  <circle class="base" cx="60" cy="10" r="3"/>
  <circle class="base" cx="68" cy="14" r="3"/>
  <line class="beam" x1="49" y1="16" x2="71" y2="16"/>
</svg>`
  return 'data:image/svg+xml;base64,' + Buffer.from(svg).toString('base64')
}

// Map the tier identifier to the court name displayed on the document.
function courtName(tier) {
  switch (tier) {
    case 'first-instance':   return 'FIRST INSTANCE COURT'
    case 'court-of-appeal':  return 'COURT OF APPEAL'
    case 'supreme-court':  return 'SUPREME COURT'
    default:                 return 'COURT'
  }
}

// Format the "Before:" line from a ruling.
function formatBefore(ruling) {
  if (ruling.panel && Array.isArray(ruling.panel) && ruling.panel.length > 0) {
    return ruling.panel.join(', ')
  }
  if (ruling.judge) return ruling.judge
  return 'Unknown'
}

// Break a block of judgment text into numbered paragraphs.
// Handles text that is already numbered ("1. Foo" style) and raw prose.
function numberedParagraphs(text) {
  if (!text) return []
  const lines = text.split(/\n+/).map(l => l.trim()).filter(Boolean)
  const paras = []
  let n = 1
  for (const line of lines) {
    // If the line already starts with a number, preserve it as-is.
    const already = line.match(/^(\d+)\.\s+(.*)$/)
    if (already) {
      paras.push({ n: parseInt(already[1], 10), text: already[2] })
      n = parseInt(already[1], 10) + 1
    } else if (line.startsWith('#')) {
      // Markdown heading - render as a section heading with no paragraph number.
      const heading = line.replace(/^#+\s*/, '')
      paras.push({ heading: true, text: heading })
    } else {
      paras.push({ n: n++, text: line })
    }
  }
  return paras
}

// Build the template data object from the VJS ruling JSON.
function buildTemplateData(input) {
  const ruling = input.ruling || {}
  const lexby = input.lexby_translation || input.lexby || {}
  const tier = input.tier || ruling.tier || 'first-instance'

  // Decide what body text to render.
  // For First Instance: compose the judgment from structured fields.
  // For CoA / Supreme: use the full_judgment_text if available, else compose.
  let bodyParagraphs = []
  let bodyIsFreeText = false

  if (ruling.full_judgment_text) {
    bodyParagraphs = numberedParagraphs(ruling.full_judgment_text)
    bodyIsFreeText = true
  } else {
    // Compose a structured judgment from the fields we have.
    const composed = []
    if (ruling.kind === 'request_for_ruling') {
      composed.push('This court was asked to rule upon the following question: ' + (ruling.question_or_charge || ''))
    } else if (ruling.kind === 'breach') {
      composed.push('This matter comes before the court as a charge in negligence (breach of the duty of care) pursuant to SPEC-LAW s. 4 through s. 8. The charge is: ' + (ruling.question_or_charge || ''))
    }
    if (ruling.fast_path && ruling.fast_path_cite) {
      composed.push('The matter falls to be disposed of on citation under VPR 2. A binding ratio on all fours governs: ' + ruling.fast_path_cite + '.')
    }
    if (ruling.ratio) {
      composed.push('RATIO DECIDENDI: ' + ruling.ratio)
    }
    if (ruling.obiter) {
      composed.push('OBITER DICTA: ' + ruling.obiter)
    }
    if (ruling.remedy) {
      composed.push('ORDER: ' + ruling.remedy)
    }
    bodyParagraphs = numberedParagraphs(composed.join('\n\n'))
  }

  return {
    tier,
    court_name: courtName(tier),
    citation: ruling.citation_id || '[YEAR] LEXBY n',
    date: formatDate(input.date),
    before: formatBefore(ruling),
    kind_label: ruling.kind === 'breach' ? 'BREACH PROCEEDINGS' : 'REQUEST FOR RULING',
    matter: ruling.question_or_charge || '',
    is_breach: ruling.kind === 'breach',
    body_paragraphs: bodyParagraphs,
    ratio: ruling.ratio || '',
    obiter: ruling.obiter || '',
    remedy: ruling.remedy || '',
    per_incuriam: ruling.per_incuriam || false,
    status: ruling.status || 'good-law',
    lexby_summary: lexby.plain_english_summary || '',
    lexby_practice: lexby.what_it_means_in_practice || '',
    lexby_appeal: lexby.can_it_be_appealed || '',
    logo_uri: resolveLogoDataUri(tier),
  }
}

// ---------------------------------------------------------------------------
// Template loading
// ---------------------------------------------------------------------------

let _compiledTemplate = null

function getTemplate() {
  if (_compiledTemplate) return _compiledTemplate
  const html = fs.readFileSync(path.join(TEMPLATES_DIR, 'judgment.html'), 'utf8')
  Handlebars.registerHelper('eq', (a, b) => a === b)
  Handlebars.registerHelper('hasText', (s) => s && s.trim().length > 0)
  _compiledTemplate = Handlebars.compile(html)
  return _compiledTemplate
}

// ---------------------------------------------------------------------------
// Core render function
// ---------------------------------------------------------------------------

async function renderJudgment(rulingJson, outputPath) {
  const template = getTemplate()
  const data = buildTemplateData(rulingJson)
  const html = template(data)

  const browser = await puppeteer.launch({
    headless: true,
    args: ['--no-sandbox', '--disable-setuid-sandbox'],
    executablePath: process.env.PUPPETEER_EXECUTABLE_PATH || undefined,
  })

  try {
    const page = await browser.newPage()
    await page.setContent(html, { waitUntil: 'networkidle0' })
    await page.pdf({
      path: outputPath,
      format: 'A4',
      printBackground: true,
      margin: { top: '2.5cm', bottom: '2.5cm', left: '3cm', right: '2.5cm' },
      displayHeaderFooter: true,
      headerTemplate: '<span></span>',
      footerTemplate: `
        <div style="font-family: 'Times New Roman', serif; font-size: 9pt; color: #444; width: 100%;
                    display: flex; justify-content: space-between; padding: 0 3cm 0 3cm; box-sizing: border-box;">
          <span style="font-style: italic;">Vibe Justice System</span>
          <span><span class="pageNumber"></span></span>
        </div>`,
    })
  } finally {
    await browser.close()
  }

  return outputPath
}

// ---------------------------------------------------------------------------
// CLI entry point
// ---------------------------------------------------------------------------

async function main() {
  const args = process.argv.slice(2)

  if (args.includes('--test')) {
    console.log('Running test render...')
    const sample = sampleRuling()
    const out = '/tmp/vjs-test-judgment.pdf'
    await renderJudgment(sample, out)
    console.log('Test PDF written to:', out)
    return
  }

  let rulingJson
  let outputPath

  if (args.includes('--stdin')) {
    const raw = fs.readFileSync('/dev/stdin', 'utf8')
    rulingJson = JSON.parse(raw)
    outputPath = args[args.indexOf('--stdin') + 1] || '/tmp/vjs-judgment.pdf'
  } else if (args[0] && args[0].endsWith('.json')) {
    rulingJson = JSON.parse(fs.readFileSync(args[0], 'utf8'))
    outputPath = args[1] || args[0].replace('.json', '.pdf')
  } else {
    console.error('Usage: node index.js <ruling.json> [output.pdf]')
    console.error('       node index.js --stdin < ruling.json')
    console.error('       node index.js --test')
    process.exit(1)
  }

  const out = await renderJudgment(rulingJson, outputPath)
  console.log(`Judgment PDF written to: ${out}`)
  console.log(`You can read the judgment here: ${out}`)
}

// ---------------------------------------------------------------------------
// Sample ruling (for --test mode)
// ---------------------------------------------------------------------------

function sampleRuling() {
  return {
    tier: 'first-instance',
    date: '5 June 2026',
    ruling: {
      citation_id: '[2026] LEXBY-FI 1',
      tier: 'first-instance',
      judge: 'Bowan J',
      kind: 'request_for_ruling',
      question_or_charge: 'Is the Vibe Justice System (VJS), as currently built at commit 3ff820a, fit for release as an alpha?',
      standing: true,
      fast_path: false,
      fast_path_cite: null,
      ratio: 'The VJS at commit 3ff820a is fit for release as an alpha under the standard of reasonable skill and care (s. 4, s. 5). The core legal model is demonstrably instantiated and internally coherent; the governing rules, procedure, and founding caselaw are committed and self-consistent; constitutional enforcement automation is in place; and the known gaps are disclosed, do not undermine legal coherence, and are appropriate to remediate before v1.',
      obiter: 'Before v1 the minimum closure set in order of legal priority is: (1) deterministic citation numbering (necessary condition for v1); (2) submit-request-to-court and submit-breach-to-court as executable commands (necessary condition for v1); (3) lexby cite command (strongly advisable); (4) ruling card renderer (strongly advisable); (5) cdd CLI init (advisable); (6) npm/PyPI packaging (advisable).',
      per_incuriam: false,
      remedy: null,
      status: 'good-law',
    },
    lexby_translation: {
      plain_english_summary: 'The principal asked whether the VJS system is ready to ship as an alpha version. Bowan J - who by temperament tests every argument until it breaks - held that the system is fit for alpha. The statute book is complete, the procedure rules are complete, the court workflows run, the constitutional enforcement bot is live, and the founding case is in the record. The gaps (user-facing CLI commands, citation numbering, packaging) are real but they are named and do not break what is already there.',
      what_it_means_in_practice: 'The VJS can be released as an alpha today. The builder should proceed and use the alpha period to complete deterministic citation numbering and the submit-request-to-court / submit-breach-to-court commands, which are the two items this court considers necessary before a v1 release.',
      can_it_be_appealed: 'Yes. Permission to appeal to the Court of Appeal is available on an arguable point of law or a binding-precedent conflict (VPR 3, s. 10).',
    },
  }
}

// ---------------------------------------------------------------------------
// Export for programmatic use
// ---------------------------------------------------------------------------

module.exports = { renderJudgment, buildTemplateData }

// Run CLI if invoked directly
if (require.main === module) {
  main().catch(err => {
    console.error('Render failed:', err.message)
    process.exit(1)
  })
}
