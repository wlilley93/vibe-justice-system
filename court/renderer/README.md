# VJS Judgment Renderer

Converts a VJS ruling JSON into a PDF that looks like a real UK court judgment. Court logo at the top, numbered paragraphs, styled ratio/obiter sections, Lexby translation panel, proper A4 margins. Each court tier has its own logo slot.

---

## Install

```bash
cd court/renderer
npm install
```

Puppeteer will download a bundled Chromium on first install (~170 MB). To use your system Chrome instead:

```bash
PUPPETEER_EXECUTABLE_PATH=/usr/bin/google-chrome npm install --ignore-scripts
```

---

## Usage

### From a JSON file

```bash
node index.js ruling.json
# writes ruling.pdf alongside the JSON

node index.js ruling.json /path/to/output.pdf
# writes to a specific path
```

### From stdin

```bash
cat ruling.json | node index.js --stdin /tmp/judgment.pdf
```

### Test render (uses the founding case [2026] LEXBY-FI 1)

```bash
node index.js --test
# writes /tmp/vjs-test-judgment.pdf
```

---

## Input JSON schema

The renderer accepts the raw return value from any court workflow, or a hand-crafted JSON matching:

```json
{
  "tier": "first-instance",
  "date": "5 June 2026",
  "ruling": {
    "citation_id": "[2026] LEXBY-FI 1",
    "tier": "first-instance",
    "judge": "Bowan J",
    "panel": null,
    "kind": "request_for_ruling",
    "question_or_charge": "...",
    "ratio": "...",
    "obiter": "...",
    "per_incuriam": false,
    "remedy": null,
    "status": "good-law",
    "full_judgment_text": null
  },
  "lexby_translation": {
    "plain_english_summary": "...",
    "what_it_means_in_practice": "...",
    "can_it_be_appealed": "..."
  }
}
```

For Court of Appeal and Supreme Council rulings, set `ruling.full_judgment_text` to the full narrative judgment text (from the workflow output). The renderer will number its paragraphs automatically.

For a panel (multi-judge court), set `ruling.panel` to an array of judge names and leave `ruling.judge` null.

---

## Adding logos

Drop your logo files into `assets/` (see `assets/README.md`). The renderer automatically picks them up by filename. A scales-of-justice SVG placeholder is used when no logo file is found.

---

## Integration with court workflows

The court workflows automatically invoke the renderer after every ruling. The path to the generated PDF is included in the workflow return value and Lexby prints:

```
You can read the judgment here: .justice/pdfs/2026-lexby-fi-1.pdf
```

To invoke manually from a workflow agent:

```bash
node court/renderer/index.js .justice/pdfs/2026-lexby-FI-1.json .justice/pdfs/2026-lexby-fi-1.pdf
```

---

## Programmatic use

```js
const { renderJudgment } = require('./court/renderer')

await renderJudgment(rulingJson, '/tmp/judgment.pdf')
```
