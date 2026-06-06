#!/usr/bin/env node
'use strict';
// Bill renderer - sibling of court/renderer. Renders a legislature/bills/NN-*.md into a formal
// full-bleed cream A4 PDF (the same house style as judgments), so the Order Paper has rendered
// Acts to read and (later) annotate. Reuses the court renderer's installed Chromium (no second
// download). Minimal markdown->HTML (headings, paragraphs, lists, bold, blockquote, hr, tables).
//
//   node legislature/renderer/index.js <bill.md> [out.pdf]
//   node legislature/renderer/index.js --all      (render every legislature/bills/NN-*.md)

const fs = require('fs');
const path = require('path');
const puppeteer = require(path.resolve(__dirname, '..', '..', 'court', 'renderer', 'node_modules', 'puppeteer'));

const ROOT = path.resolve(__dirname, '..', '..');
const BILLS = path.join(ROOT, 'legislature', 'bills');
const OUTDIR = path.join(ROOT, 'legislature', 'pdfs');

function esc(s) { return s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;'); }
function inline(s) {
  return esc(s)
    .replace(/\[([^\]]+)\]\(([^)]+)\)/g, '<a>$1</a>')
    .replace(/\*\*([^*]+)\*\*/g, '<strong>$1</strong>')
    .replace(/`([^`]+)`/g, '<code>$1</code>');
}
// tiny block-level markdown -> html
function mdToHtml(md) {
  md = md.replace(/<!--[\s\S]*?-->/g, '').replace(/^---\n[\s\S]*?\n---\n/, ''); // drop comments + frontmatter
  const lines = md.split('\n');
  const out = []; let inList = false, inTable = false;
  const closeList = () => { if (inList) { out.push('</ul>'); inList = false; } };
  const closeTable = () => { if (inTable) { out.push('</table>'); inTable = false; } };
  for (const raw of lines) {
    const line = raw.replace(/\s+$/, '');
    if (/^\s*$/.test(line)) { closeList(); closeTable(); continue; }
    let m;
    if ((m = line.match(/^(#{1,4})\s+(.*)$/))) { closeList(); closeTable(); const l = m[1].length; out.push(`<h${l}>${inline(m[2])}</h${l}>`); continue; }
    if (/^(-{3,}|\*{3,})$/.test(line)) { closeList(); closeTable(); out.push('<hr>'); continue; }
    if ((m = line.match(/^>\s?(.*)$/))) { closeList(); closeTable(); out.push(`<blockquote>${inline(m[1])}</blockquote>`); continue; }
    if (/^\s*\|.*\|\s*$/.test(line)) {
      if (/^\s*\|[\s:|-]+\|\s*$/.test(line)) continue; // separator row
      if (!inTable) { closeList(); out.push('<table>'); inTable = true; }
      const cells = line.split('|').slice(1, -1).map(c => `<td>${inline(c.trim())}</td>`).join('');
      out.push(`<tr>${cells}</tr>`); continue;
    } else closeTable();
    if ((m = line.match(/^\s*[-*]\s+(.*)$/))) { if (!inList) { out.push('<ul>'); inList = true; } out.push(`<li>${inline(m[1])}</li>`); continue; }
    if ((m = line.match(/^\s*\d+\.\s+(.*)$/))) { if (!inList) { out.push('<ul>'); inList = true; } out.push(`<li>${inline(m[1])}</li>`); continue; }
    closeList(); out.push(`<p>${inline(line)}</p>`);
  }
  closeList(); closeTable();
  return out.join('\n');
}

const PAGE_CSS = `
  html { background:#fcf7f1; }
  body { margin:0; padding:0 2.5cm 0 3cm; background:#fcf7f1; color:#15202b;
    font-family:'Times New Roman',Times,serif; font-size:12pt; line-height:1.6;
    -webkit-print-color-adjust:exact; print-color-adjust:exact; }
  table.frame { width:100%; border-collapse:collapse; }
  thead td { height:2.2cm; } tfoot td { height:1.8cm; vertical-align:bottom; }
  .foot { font-size:9pt; font-style:italic; color:#777; padding-bottom:.7cm; }
  h1 { font-size:15pt; text-align:center; letter-spacing:.06em; text-transform:uppercase; margin:.2cm 0 .4cm; }
  h2 { font-size:12.5pt; border-bottom:1px solid #c8d0dc; padding-bottom:.1cm; margin-top:.6cm; }
  h3 { font-size:11.5pt; } h4 { font-size:11pt; color:#2a3a55; }
  p { text-align:justify; } blockquote { border-left:2px solid #c8d0dc; margin:.4cm 0; padding-left:.6cm; color:#444; font-style:italic; }
  hr { border:none; border-top:1px solid #111; margin:.4cm 0; }
  table { border-collapse:collapse; width:100%; font-size:10.5pt; margin:.3cm 0; }
  td { border:1px solid #c8d0dc; padding:.15cm .3cm; vertical-align:top; }
  code { font-family:'Courier New',monospace; font-size:10.5pt; }
  .crest { text-align:center; padding-top:.2cm; }
  .crest .wm { font-family:'Times New Roman',serif; font-size:20pt; letter-spacing:.16em; text-transform:uppercase; color:#1a2744; }
  .crest .sub { font-size:10pt; letter-spacing:.06em; text-transform:uppercase; color:#444; }
  .crest hr { width:6cm; margin:.25cm auto; border-top:.75px solid #1a2744; }
`;

async function render(mdPath, outPath) {
  const md = fs.readFileSync(mdPath, 'utf8');
  const html = `<!DOCTYPE html><html><head><meta charset="utf-8"><style>${PAGE_CSS}</style></head><body>
    <table class="frame"><thead><tr><td></td></tr></thead>
    <tfoot><tr><td><div class="foot">The Legislature of the Realm &middot; Agent Universe</div></td></tr></tfoot>
    <tbody><tr><td>
      <div class="crest"><hr><div class="wm">An Act of the Realm</div>
        <div class="sub">The Legislature &middot; Agent Universe</div><hr></div>
      ${mdToHtml(md)}
    </td></tr></tbody></table></body></html>`;
  const b = await puppeteer.launch({ headless: true, args: ['--no-sandbox', '--disable-setuid-sandbox'] });
  try {
    const p = await b.newPage();
    await p.setContent(html, { waitUntil: 'networkidle0' });
    await p.pdf({ path: outPath, format: 'A4', printBackground: true, margin: { top: '0', bottom: '0', left: '0', right: '0' } });
  } finally { await b.close(); }
  return outPath;
}

async function main() {
  const args = process.argv.slice(2);
  fs.mkdirSync(OUTDIR, { recursive: true });
  if (args[0] === '--all') {
    const files = fs.readdirSync(BILLS).filter(f => /^\d{2}-.*\.md$/.test(f)).sort();
    for (const f of files) {
      const out = path.join(OUTDIR, f.replace(/\.md$/, '.pdf'));
      await render(path.join(BILLS, f), out);
      console.log('rendered', path.relative(ROOT, out));
    }
    console.log(`done: ${files.length} bills -> legislature/pdfs/`);
    return;
  }
  if (!args[0]) { console.error('usage: index.js <bill.md> [out.pdf] | --all'); process.exit(1); }
  const out = args[1] || path.join(OUTDIR, path.basename(args[0]).replace(/\.md$/, '.pdf'));
  await render(args[0], out);
  console.log('rendered', out);
}
main().catch(e => { console.error('render failed:', e.message); process.exit(1); });
