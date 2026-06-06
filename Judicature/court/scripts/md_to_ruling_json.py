#!/usr/bin/env python3
"""Convert a committed VJS judgment markdown file into the renderer's ruling JSON.

The judgment markdown is the canonical record; the renderer eats JSON. This parser
handles both frontmatter styles found in the realm (YAML between --- fences, or a
| Field | Value | table) and maps the `## sections` onto the renderer schema:

  - question / matter / ground of appeal  -> ruling.question_or_charge
  - ratio*                                 -> ruling.ratio
  - obiter*                                -> ruling.obiter
  - remedy                                 -> ruling.remedy
  - lexby* (translation / TL;DR)           -> lexby_translation.*
  - everything else (intake, judgment,     -> ruling.full_judgment_text
    panel opinions, the bench, ...)           (narrative, headings preserved)

Tier, citation, division, repo and a citation-remap table are supplied by the caller
so the same source text can be reconstituted under the new provenance citation scheme.

Usage:
  md_to_ruling_json.py <judgment.md> --tier <tier> --citation '<[YEAR] CODE N>' \
      [--division "Engineering Division"] [--repo acmeco] [--map map.json] > ruling.json
"""
import sys, os, re, json, argparse

SECTION_RE = re.compile(r'^##\s+(.*)$', re.M)

def strip_md(text):
    """Strip the markdown emphasis the renderer does not interpret."""
    text = re.sub(r'\[([^\]]+)\]\([^)]+\)', r'\1', text)   # [label](url) -> label
    text = text.replace('**', '').replace('__', '')          # bold
    text = re.sub(r'`([^`]*)`', r'\1', text)                 # `code` -> code
    text = re.sub(r'^>\s?', '', text, flags=re.M)            # blockquote marker
    return text

def parse_frontmatter(raw):
    """Return (meta dict, body) handling --- YAML --- or a leading | Field | Value | table."""
    meta, body = {}, raw
    if raw.startswith('---'):
        end = raw.find('\n---', 3)
        if end != -1:
            fm = raw[3:end].strip()
            body = raw[end+4:]
            for line in fm.splitlines():
                m = re.match(r'^([A-Za-z_]+):\s*(.*)$', line.strip())
                if m:
                    meta[m.group(1).lower()] = m.group(2).strip().strip('"')
    # table frontmatter: | **Citation** | [2026] ... |
    for m in re.finditer(r'^\|\s*\*{0,2}([A-Za-z /]+?)\*{0,2}\s*\|\s*(.+?)\s*\|\s*$', raw, re.M):
        key = m.group(1).strip().lower()
        if key in ('field',):
            continue
        meta.setdefault(key, strip_md(m.group(2).strip()))
    return meta, body

def split_sections(body):
    """Return ordered list of (heading, text). Text before the first ## is dropped
    (it is the title/Before block, already captured in metadata)."""
    parts = []
    matches = list(SECTION_RE.finditer(body))
    for i, mt in enumerate(matches):
        heading = mt.group(1).strip()
        start = mt.end()
        end = matches[i+1].start() if i+1 < len(matches) else len(body)
        text = body[start:end].strip()
        # drop a trailing signature / status footer block
        parts.append((heading, text))
    return parts

def classify(heading):
    h = heading.lower()
    if h.startswith('ratio'): return 'ratio'
    if h.startswith('obiter'): return 'obiter'
    if h.startswith('remedy'): return 'remedy'
    if h.startswith('lexby'): return 'lexby'
    if any(h.startswith(k) for k in ('matter', 'question', 'the question', 'the questions',
                                     'ground of appeal', 'the reference', 'reference')):
        return 'question'
    if any(h.startswith(k) for k in ('per incuriam', 'status', 'citation')):
        return 'drop'
    return 'narrative'

def extract_question(text):
    # Prefer an explicit "Question as filed:" / "Question:" line if present.
    m = re.search(r'(?:Question as filed|Question)\s*:\s*(.+)', text)
    if m:
        return m.group(1).strip()
    # else first non-empty paragraph
    for para in text.split('\n\n'):
        p = para.strip()
        if p and not p.lower().startswith('kind'):
            return p
    return text.strip()

def parse_lexby(text):
    """Split a Lexby section into summary / practice / appeal by its bold sub-labels."""
    out = {'plain_english_summary': '', 'what_it_means_in_practice': '', 'can_it_be_appealed': ''}
    # markers
    practice = re.search(r'(?:In practice|What it means in practice|What this means in practice)\s*[:\-]?\s*', text, re.I)
    appeal = re.search(r'(?:Appeal route|Can it be appealed\??|Appeal)\s*[:\-]?\s*', text, re.I)
    idxs = []
    if practice: idxs.append((practice.start(), practice.end(), 'practice'))
    if appeal: idxs.append((appeal.start(), appeal.end(), 'appeal'))
    idxs.sort()
    if not idxs:
        out['plain_english_summary'] = text.strip()
        return out
    out['plain_english_summary'] = text[:idxs[0][0]].strip()
    for i, (s, e, kind) in enumerate(idxs):
        seg_end = idxs[i+1][0] if i+1 < len(idxs) else len(text)
        seg = text[e:seg_end].strip()
        if kind == 'practice': out['what_it_means_in_practice'] = seg
        else: out['can_it_be_appealed'] = seg
    return out

def apply_map(text, cmap):
    for old, new in cmap.items():
        text = text.replace(old, new)
    return text

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('md')
    ap.add_argument('--tier', required=True)
    ap.add_argument('--citation', required=True)
    ap.add_argument('--division', default=None)
    ap.add_argument('--repo', default=None)
    ap.add_argument('--list', dest='list_', default=None)
    ap.add_argument('--date', default='5 June 2026')
    ap.add_argument('--map', default=None, help='JSON file of {old_citation: new_citation}')
    args = ap.parse_args()

    raw = open(args.md, encoding='utf-8').read()
    cmap = json.load(open(args.map)) if args.map else {}
    meta, body = parse_frontmatter(raw)
    sections = split_sections(body)

    fields = {'ratio': '', 'obiter': '', 'remedy': '', 'question': '', 'lexby': None}
    narrative = []
    for heading, text in sections:
        kind = classify(heading)
        text_clean = apply_map(strip_md(text), cmap)
        if kind == 'drop':
            continue
        elif kind == 'question':
            fields['question'] = extract_question(text_clean)
            # the rest of a Matter/Intake section beyond the question is narrative context
        elif kind in ('ratio', 'obiter', 'remedy'):
            fields[kind] = text_clean
        elif kind == 'lexby':
            fields['lexby'] = parse_lexby(text_clean)
        else:
            narrative.append('# ' + heading)
            narrative.append(text_clean)

    panel = None
    if meta.get('panel'):
        pv = meta['panel'].strip()
        if pv.startswith('['):
            panel = [x.strip().strip('"') for x in pv.strip('[]').split(',') if x.strip()]
        else:
            panel = [x.strip() for x in pv.split(',') if x.strip()]

    ruling = {
        'citation': args.citation,
        'citation_id': args.citation,
        'tier': args.tier,
        'kind': meta.get('kind', 'request_for_ruling'),
        'question_or_charge': fields['question'],
        'ratio': fields['ratio'],
        'obiter': fields['obiter'],
        'remedy': fields['remedy'] if fields['remedy'].lower() not in ('none.', 'none', '') else '',
        'per_incuriam': str(meta.get('per_incuriam', 'false')).lower() in ('true', 'yes'),
        'status': meta.get('status', 'good-law'),
        'full_judgment_text': '\n\n'.join(narrative).strip(),
    }
    if panel:
        ruling['panel'] = panel
    elif meta.get('judge'):
        ruling['judge'] = meta['judge']
    if args.repo: ruling['repo'] = args.repo

    out = {
        'tier': args.tier,
        'date': args.date,
        'division': args.division,
        'list': args.list_,
        'repo': args.repo,
        'ruling': ruling,
        'lexby_translation': fields['lexby'] or {},
    }
    json.dump(out, sys.stdout, indent=2, ensure_ascii=False)

if __name__ == '__main__':
    main()
