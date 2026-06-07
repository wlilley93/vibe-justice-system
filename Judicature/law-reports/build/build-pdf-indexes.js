'use strict';
// Build deterministic index.html files for PDF directories linked from the Gazette home page.
// GitHub Pages does not expose directory listings, so the linked directories need explicit pages.

const fs = require('fs');
const path = require('path');
const { ROOT } = require('./corpus');

const INDEXES = [
  {
    title: 'Rendered Acts',
    dir: path.join(ROOT, 'Legislature', 'legislature', 'pdfs'),
    gazetteHref: '../../../Judicature/law-reports/site/',
  },
  {
    title: 'Rendered Statutory Instruments',
    dir: path.join(ROOT, 'Legislature', 'statutes', 'instruments', 'pdfs'),
    gazetteHref: '../../../../Judicature/law-reports/site/',
  },
  {
    title: 'Rendered Judgments',
    dir: path.join(ROOT, 'Judicature', '.justice', 'pdfs'),
    gazetteHref: '../../law-reports/site/',
  },
];

function label(file) {
  return file
    .replace(/\.pdf$/i, '')
    .replace(/-/g, ' ')
    .replace(/\b\w/g, ch => ch.toUpperCase())
    .replace(/\bRealm (Si|Pc|Sc|Ca)\b/g, (_, series) => `REALM-${series.toUpperCase()}`);
}

function esc(s) {
  return String(s).replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');
}

function render({ title, dir, gazetteHref }) {
  const files = fs.readdirSync(dir).filter(f => f.endsWith('.pdf')).sort();
  const links = files.map(file => `      <li><a href="${esc(file)}">${esc(label(file))}</a></li>`).join('\n');
  return `<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>${esc(title)} - VJS Law Reports & Gazette</title>
<style>
  body { margin:0; background:#fcf7f1; color:#15202b; font-family:'Iowan Old Style','Palatino Linotype','Times New Roman',serif; line-height:1.5; }
  main { max-width:820px; margin:0 auto; padding:2rem 1rem 4rem; }
  h1 { color:#1a2744; font-size:1.8rem; margin:.2rem 0 .4rem; }
  p { color:#5b6472; }
  a { color:#1a2744; }
  .back { display:inline-block; margin-bottom:1rem; }
  ol { background:#fffdfa; border:1px solid #c8d0dc; border-radius:6px; padding:1rem 1rem 1rem 2.25rem; }
  li { margin:.35rem 0; }
</style>
</head>
<body>
<main>
  <a class="back" href="${esc(gazetteHref)}">Back to Law Reports &amp; Gazette</a>
  <h1>${esc(title)}</h1>
  <p>Rendered PDF record. Search across Acts, statutory instruments, and judgments in the Gazette.</p>
  <ol>
${links}
  </ol>
</main>
</body>
</html>
`;
}

for (const item of INDEXES) {
  const dest = path.join(item.dir, 'index.html');
  fs.writeFileSync(dest, render(item));
  console.log(`pdf index: ${path.relative(ROOT, dest)}`);
}
