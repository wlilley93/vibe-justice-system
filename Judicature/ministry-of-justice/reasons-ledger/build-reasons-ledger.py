#!/usr/bin/env python3
"""Reasons Ledger clerk - the EXECUTIVE analogue of the rulings ledger.

Discharges the Public Reasons and Audit Act 2026 (Bill 8). Projects every "significant decision"
(Bill 8 s. 5(2)) into ONE pointer-only index of the four s. 5(3) reasons fields - WHAT was decided,
WHY, what it RESTED ON, and WHO/what AUTHORISED it - by deriving them from three EXISTING traces:

  (1) caselaw forks   - read from the central citator (.justice/INDEX.md) and any sub-repo citators
                        (CC-* hearing-centres). trigger = a court-convening fork; what = the ratio;
                        rested_on = the Cites column; authorised_by = the court + its citation;
                        pointer = the ruling path the citator already links.
  (2) legislative     - read from the bill record (legislature/bills/NN-*.md headers + long title +
        decisions       ORDER-PAPER.md). One enacted Act = one significant decision; trigger = statute;
                        authorised_by = the Sovereign Founder on Royal Assent + the committee vote.
  (3) git commits     - parse a structured "Reasons-record:" trailer block (the four s. 5(3) fields)
                        for FULL records going forward; for PRE-CONVENTION significant commits (those
                        touching .justice/, statutes/, legislature/, or recording an irreversible /
                        outward act - publish/send/deploy/spend/grant), list the extractable subject +
                        a commit-hash pointer + a "(reasons in commit body)" note.

This is a DERIVED, POINTER-ONLY, DETERMINISTICALLY REBUILDABLE projection (the doctrine of
[2026] REALM-PC 4: index-not-replacement). It is NOT a source of law or truth, NOT a new register,
NOT a second governance home. It holds NO ratio, NO status, NO authority - only pointers into the
artefacts the realm already owns (Bill 8 s. 4(2), s. 24, s. 25). Re-running it on the same repo state
yields byte-for-byte identical output (no run-to-run timestamps; stable sort). Regenerate it in
lockstep with its sources, exactly as build-ledger.py is regenerated after a new ruling.

Token-free and deterministic (CASE-LAW s. 19(5)). Reading git uses `git log` via subprocess; the
output is a pure function of repo state.

Usage:  python3 build-reasons-ledger.py    (writes ministry-of-justice/reasons-ledger/INDEX.md)
"""
import re, pathlib, subprocess, sys

REALM = pathlib.Path(__file__).resolve().parents[2]            # ~/agent-universe
LEDGER = REALM / "ministry-of-justice" / "reasons-ledger" / "INDEX.md"

# ---------------------------------------------------------------------------
# Citation form (provenance scheme, CASE-LAW s. 11(d) as amended; mirrors build-ledger.py):
# [YEAR] REALM-SC/PC/CA n, [YEAR] <DIVISION> n (ENG/CHAN), [YEAR] CC-<repo> n.
CITE = re.compile(
    r"\[(20\d\d)\]\s+(?:REALM-(?:SC|PC|CA)|ENG|CHAN|CC-[A-Z0-9-]+)\s+\d+",
    re.I)

# A markdown link target inside a citator row, e.g. [[2026] REALM-SC 1](judgments/.../x.md)
LINKED_CITE = re.compile(
    r"\[\s*(\[(20\d\d)\]\s+(?:REALM-(?:SC|PC|CA)|ENG|CHAN|CC-[A-Z0-9-]+)\s+\d+)\s*\]"
    r"\(([^)]+)\)",
    re.I)

# The structured commit trailer (forward convention). One block, key-value lines, four s.5(3) fields.
TRAILER_KEYS = ("Decided", "Why", "Rested-on", "Authorised-by")

# A pre-convention commit is SIGNIFICANT if it touched a governed surface ...
SIGNIFICANT_PATH_PREFIXES = ("Judicature/.justice/", "Legislature/statutes/", "Legislature/legislature/")
# ... or its subject records an irreversible / outward-facing act (Bill 8 s. 5(2)(b)).
OUTWARD_RE = re.compile(
    r"\b(publish|published|deploy|deployed|royal assent|enact|enacts|enacted|"
    r"reconstitut|grant|granted|send|sent|merge pull request|release)\b",
    re.I)

# De minimis (CASE-LAW s. 15; Bill 8 s. 5(2) closing words): a routine commit that weakens no governed
# surface owes no reasons-record EVEN IF it touches a governed path. Excluded from the pre-convention
# backfill by subject prefix, so the index does not over-capture typo/format/doc churn as "significant".
DE_MINIMIS_RE = re.compile(
    r"^\s*(readme|docs?|doc|fmt|format|style|typo|comment|comments|chore|"
    r"lint|whitespace|rename|banner|slim|tidy|cleanup|nit)\b[:\s]",
    re.I)

PROJECTION_REDACTIONS = (
    (re.compile("Bee" + "link", re.I), "local-development host"),
    (re.compile("Het" + "zner", re.I), "public-production host"),
    (re.compile(r"gh" + r"p_[A-Za-z0-9_]+"), "<github-token>"),
    (re.compile(r"sk" + r"-ant-[A-Za-z0-9_-]+"), "<anthropic-token>"),
)

def redact_projection(s: str) -> str:
    """Redact historical operational facts when projecting public ledger text from git history."""
    for pattern, repl in PROJECTION_REDACTIONS:
        s = pattern.sub(repl, s)
    return s

def norm(s: str) -> str:
    return redact_projection(re.sub(r"\s+", " ", s).strip())

def esc(s: str) -> str:
    """Escape for a markdown table cell."""
    return norm(s).replace("|", "\\|")

# ---------------------------------------------------------------------------
# (1) CASELAW FORKS - harvested from the citator(s). Each ruling is a court-convening fork
#     (Bill 8 s. 5(2)(a)); the citator already structures it (s. 4(2) reuse, no new artefact).

def harvest_caselaw():
    """Yield reasons-records for every ruling listed in a citator (central + sub-repo)."""
    records = []
    seen = set()
    citators = []
    central = REALM / ".justice" / "INDEX.md"
    if central.exists():
        citators.append(central)
    # sub-repo citators (CC-* hearing-centres): any .justice/INDEX.md below the root.
    for f in sorted(REALM.rglob(".justice/INDEX.md")):
        if ".git" in f.parts:
            continue
        if f != central and f not in citators:
            citators.append(f)
    citators = sorted(set(citators), key=lambda p: str(p))

    for cit in citators:
        try:
            text = cit.read_text(encoding="utf-8", errors="ignore")
        except Exception:
            continue
        rel_cit = str(cit.relative_to(REALM))
        court_label = court_of_citator(cit)
        for line in text.splitlines():
            if not line.startswith("|"):
                continue                          # rows only - never prose examples
            cols = [c.strip() for c in line.strip().strip("|").split("|")]
            if len(cols) < 2:
                continue
            m = LINKED_CITE.search(cols[0]) or CITE.search(cols[0])
            if not m:
                continue
            if hasattr(m, "lastindex") and m.re is LINKED_CITE:
                cite = norm(m.group(1)); ruling_rel = m.group(3)
            else:
                cite = norm(m.group(0)); ruling_rel = ""
            if cite in seen:
                continue                          # first occurrence wins (mirrors build-ledger.py)
            seen.add(cite)
            # Column meanings differ between central (Citation|Court|Status|Ratio|Cites) and
            # sub-repo (Citation|Court|Status|Reconstituted from). Read positionally + defensively.
            court  = cols[1] if len(cols) > 1 else court_label
            status = cols[2] if len(cols) > 2 else ""
            ratio  = cols[3] if len(cols) > 3 else ""
            cites  = cols[4] if len(cols) > 4 else (cols[3] if len(cols) > 3 else "")
            # Build the pointer the citator already provides.
            if ruling_rel:
                pointer = norm(str((cit.parent / ruling_rel).resolve().relative_to(REALM)))
            else:
                pointer = rel_cit
            has_cites = len(cols) > 4 and bool(cols[4])
            records.append({
                "kind": "Caselaw",
                "trigger": "court-convening fork (s. 5(2)(a))",
                "sortcite": cite,
                "label": cite,
                "decided": ratio or f"Ruling {cite} ({court})",
                # pointer-only: the full reasoning lives in the judgment, the row points there
                # (never restated here - that would make the index a competing source, REALM-PC 4).
                "why": "first-impression fork; full reasoning in the judgment (see Pointer)",
                "rested_on": cites if has_cites else "authorities in the judgment (see Pointer)",
                "authorised_by": f"{court} per {cite}" if court else cite,
                "pointer": pointer,
                "status": status,
            })
    return records

def court_of_citator(path: pathlib.Path) -> str:
    rel = path.relative_to(REALM).parts
    if rel[:1] == (".justice",):
        return "Ministry of Justice (central courts)"
    # sub-repo: name the hearing-centre by its parent dir
    return "County Court at " + (rel[-3] if len(rel) >= 3 else "sub-repo")

# ---------------------------------------------------------------------------
# (2) LEGISLATIVE DECISIONS - harvested from the bill record. Each enacted Act is a significant
#     decision (a statute; authorised on Royal Assent). s. 4(2) reuse: read the bill's own header
#     + long title, do not restate or duplicate the Act.

HEADER_RE = re.compile(
    r"status:\s*(?P<status>[\w-]+)\s*\|\s*royal-assent:\s*(?P<assent>[\w-]+)\s*\|\s*"
    r"outcome:\s*(?P<outcome>[\w-]+)\s*\|\s*ayes:\s*(?P<ayes>[\w/]+)\s*\|\s*"
    r"drafting rounds:\s*(?P<rounds>\w+)",
    re.I)
BILL_NO_RE = re.compile(r"Bill\s+(\d+)\s+of", re.I)

def parse_bill_header(text: str):
    """Extract the structured header fields from a bill file's HTML-comment header line."""
    h = {}
    mh = HEADER_RE.search(text)
    if mh:
        h.update({k: norm(v) for k, v in mh.groupdict().items()})
    mn = BILL_NO_RE.search(text)
    if mn:
        h["number"] = int(mn.group(1))
    # short title = first H1
    mt = re.search(r"^#\s+(.+)$", text, re.M)
    if mt:
        h["title"] = norm(mt.group(1))
    # long title: the paragraph following a "## Long title" heading
    ml = re.search(r"##\s+Long title\s*\n+(.+?)(?:\n\s*\n|\n---)", text, re.S | re.I)
    if ml:
        h["long_title"] = norm(ml.group(1))
    return h

def harvest_legislation():
    """Yield one reasons-record per enacted Act, derived from its bill file + the bill record."""
    records = []
    bills_dir = REALM.parent / "Legislature" / "legislature" / "bills"
    if not bills_dir.exists():
        return records
    for f in sorted(bills_dir.glob("[0-9][0-9]-*.md")):
        try:
            text = f.read_text(encoding="utf-8", errors="ignore")
        except Exception:
            continue
        h = parse_bill_header(text)
        if not h.get("number"):
            continue
        n = h["number"]
        title = h.get("title", f.stem)
        status = h.get("status", "")
        assent = h.get("assent", "")
        outcome = h.get("outcome", "")
        ayes = h.get("ayes", "")
        rounds = h.get("rounds", "")
        long_title = h.get("long_title", "")
        # what was decided: enact the Act (the long title is the Act's own statement of what it does).
        decided = f"Enact the {title} (Bill {n})"
        what = (long_title[:300] + "...") if len(long_title) > 300 else long_title
        if what:
            decided = f"{decided} - {what}"
        # authorised_by: the Sovereign Founder on Royal Assent + the committee vote (the bill record).
        auth_bits = []
        if assent and assent.lower() not in ("", "none", "-"):
            auth_bits.append(f"Sovereign Founder (Royal Assent {assent})")
        else:
            auth_bits.append("Sovereign Founder")
        if ayes:
            vote = f"Standing Committee {ayes}"
            if rounds and rounds not in ("1",):
                vote += f", {rounds} drafting rounds"
            auth_bits.append(vote)
        auth_bits.append("under Bill 2 (Legislature of the Realm Act: committee draft + Royal Assent procedure)")
        records.append({
            "kind": "Legislative",
            "trigger": "statute / Royal Assent (s. 5(2)(b) enact)",
            "sortcite": f"BILL-{n:03d}",
            "label": f"Bill {n}",
            "decided": decided,
            "why": f"enacted to operationalise the founding programme; outcome: {outcome or status}",
            "rested_on": f"recitals + long title of Bill {n}; CASE-LAW s. 2 (Sovereign), Bill 1 Acts of Union (statute supremacy) - see Pointer",
            "authorised_by": "; ".join(auth_bits),
            "pointer": norm(str(f.relative_to(REALM.parent)))
                       + f"; Legislature/statutes/{f.name}",
            "status": status,
        })
    return records

# ---------------------------------------------------------------------------
# (3) GIT COMMITS - the forward trailer convention + the pre-convention significant backfill.
#     Reading git is deterministic given repo state.

GIT_SEP = "\x1e"          # record separator (RS) - won't appear in messages
GIT_FIELD = "\x1f"        # unit separator (US)

def git_commits():
    """Return commits (hash, iso-date, author, subject, body) oldest-first, deterministically."""
    fmt = GIT_FIELD.join(["%H", "%aI", "%an", "%s", "%b"]) + GIT_SEP
    try:
        out = subprocess.run(
            ["git", "-C", str(REALM), "log", "--all", "--reverse", "--no-merges",
             f"--pretty=format:{fmt}"],
            capture_output=True, text=True, check=True).stdout
    except Exception as e:
        sys.stderr.write(f"reasons-ledger: git log failed ({e}); skipping git source\n")
        return []
    commits = []
    for rec in out.split(GIT_SEP):
        rec = rec.strip("\n")
        if not rec.strip():
            continue
        parts = rec.split(GIT_FIELD)
        if len(parts) < 4:
            continue
        h, date, author, subject = parts[0], parts[1], parts[2], parts[3]
        body = parts[4] if len(parts) > 4 else ""
        commits.append((h, date, author, subject, body))
    return commits

def commit_paths(h):
    """Files a commit touched (deterministic).

    Uses `git show --name-only --pretty=format:` (note: NOT --no-patch/-s, which conflicts with
    --name-only and would make git error out). The empty format suppresses the header so only
    the file paths are returned.
    """
    try:
        out = subprocess.run(
            ["git", "-C", str(REALM), "show", "--name-only", "--pretty=format:", h],
            capture_output=True, text=True, check=True).stdout
    except Exception:
        return []
    return [p for p in out.splitlines() if p.strip()]

def parse_reasons_trailer(body: str):
    """Parse a structured 'Reasons-record:' block from a commit body. Returns the four fields or None.

    Format (at the bottom of the commit body, after a blank line):

        Reasons-record:
        Decided: <what was decided / the act itself>
        Why: <the reasoning>
        Rested-on: <governing instruction / statute / case law / ratio>
        Authorised-by: <actor + office + authority>

    Lines may wrap; a continuation line is one that does not start with a known "Key:".
    """
    m = re.search(r"(?im)^\s*Reasons-record:\s*$", body)
    if not m:
        return None
    block = body[m.end():]
    fields = {}
    cur = None
    key_re = re.compile(r"^\s*(Decided|Why|Rested-on|Authorised-by)\s*:\s*(.*)$", re.I)
    for line in block.splitlines():
        km = key_re.match(line)
        if km:
            cur = km.group(1).lower().replace("-", "_")
            fields[cur] = km.group(2).strip()
        elif cur and line.strip() and not re.match(r"^\s*(Co-Authored-By|Signed-off-by):", line, re.I):
            fields[cur] = (fields[cur] + " " + line.strip()).strip()
        elif cur and not line.strip():
            cur = None      # blank line ends the block
    # require at least "decided"; the rest may be sparse but the field set is fixed
    if "decided" not in fields:
        return None
    return {
        "decided": fields.get("decided", ""),
        "why": fields.get("why", ""),
        "rested_on": fields.get("rested_on", ""),
        "authorised_by": fields.get("authorised_by", ""),
    }

def significance_of(paths, subject):
    """Return (is_significant, rule) for a pre-convention commit. The rule NAMES why the commit was
    included, so the backfill is auditable (no opaque judgement; Bill 8 s. 7(1) deterministic enforcer)."""
    for pre in SIGNIFICANT_PATH_PREFIXES:
        if any(p.startswith(pre) for p in paths):
            return True, f"touched {pre}"
    mo = OUTWARD_RE.search(subject)
    if mo:
        return True, f"outward act: {mo.group(1).lower()}"
    return False, ""

def harvest_git():
    """Yield reasons-records from git: full records (forward trailer) + pre-convention backfill."""
    records = []
    for idx, (h, date, author, subject, body) in enumerate(git_commits()):
        short = h[:7]
        day = date[:10]
        trailer = parse_reasons_trailer(body)
        if trailer:
            # A trailer is a FULL record only if all four s. 5(3) fields are present; a sparse block
            # still surfaces but is MARKED incomplete (a partial block does not discharge the duty in
            # full, and must not be silently presented as if it did).
            full = all(trailer.get(k) for k in ("decided", "why", "rested_on", "authorised_by"))
            records.append({
                "kind": "Executive",
                "trigger": "significant-decision commit (Reasons-record trailer)" if full
                           else "significant-decision commit (trailer INCOMPLETE - see commit body)",
                "sortcite": f"GIT-{idx:05d}",
                "label": short,
                "decided": trailer["decided"] or subject,
                "why": trailer["why"] or "(not in trailer; see commit body)",
                "rested_on": trailer["rested_on"] or "(not in trailer; see commit body)",
                "authorised_by": trailer["authorised_by"] or f"{author} (commit {short})",
                "pointer": f"git:{short} ({day})",
                "status": "trailer" if full else "trailer-incomplete",
            })
            continue
        # No trailer: skip routine churn (de minimis, CASE-LAW s. 15) even if it touched a governed
        # path; then include ONLY if pre-convention significant, tagging the rule that included it.
        if DE_MINIMIS_RE.search(subject):
            continue
        paths = commit_paths(h)
        sig, rule = significance_of(paths, subject)
        if not sig:
            continue
        records.append({
            "kind": "Executive",
            "trigger": f"pre-convention significant commit (backfill; {rule})",
            "sortcite": f"GIT-{idx:05d}",
            "label": short,
            "decided": subject,
            "why": "(reasons in commit body)",
            "rested_on": "(reasons in commit body)",
            "authorised_by": f"{author} (commit {short})",
            "pointer": f"git:{short} ({day})",
            "status": "backfill",
        })
    return records

# ---------------------------------------------------------------------------
# Deterministic ordering. Group by source (Caselaw, Legislative, Executive), then within a group
# by a stable key. Caselaw uses the same apex-first tier order as build-ledger.py; Legislative by
# bill number; Executive by commit order (oldest-first, already the git --reverse order).

def caselaw_sort_key(c):
    cite = c["sortcite"]
    tier = ("SC" in cite, "CA" in cite, "PC" in cite, "CC" in cite)  # apex first
    mnum = re.search(r"(\d+)\s*$", cite)
    num = int(mnum.group(1)) if mnum else 0
    return (not tier[0], not tier[1], not tier[2], not tier[3], num, cite)

def count_distinct_cites():
    """Independent control count of distinct citations across all citators (for reconciliation).
    Should equal the number of caselaw rows; a mismatch means a row failed to parse - surfaced to
    stderr so silent drift cannot hide (Bill 8 s. 7(1) deterministic enforcer)."""
    seen = set()
    central = REALM / ".justice" / "INDEX.md"
    cits = [central] if central.exists() else []
    for f in sorted(REALM.rglob(".justice/INDEX.md")):
        if ".git" not in f.parts and f not in cits:
            cits.append(f)
    for cit in cits:
        try:
            text = cit.read_text(encoding="utf-8", errors="ignore")
        except Exception:
            continue
        for line in text.splitlines():
            if not line.startswith("|"):
                continue
            m = LINKED_CITE.search(line)
            if m:
                seen.add(norm(m.group(1)))
            else:
                m2 = CITE.search(line)
                if m2:
                    seen.add(norm(m2.group(0)))
    return len(seen)

def build():
    caselaw = sorted(harvest_caselaw(), key=caselaw_sort_key)
    legislation = sorted(harvest_legislation(),
                         key=lambda c: int(c["sortcite"].split("-")[1]))
    executive = harvest_git()       # already deterministic (git --reverse), keep insertion order
    total = len(caselaw) + len(legislation) + len(executive)

    # Reconciliation: independent control counts to stderr so a parse drop cannot pass silently.
    n_cites = count_distinct_cites()
    if n_cites != len(caselaw):
        sys.stderr.write(f"reasons-ledger: NOTE caselaw rows={len(caselaw)} vs distinct citator cites={n_cites} (check for an unparsed row)\n")
    bills_dir = REALM.parent / "Legislature" / "legislature" / "bills"
    n_bills = len(list(bills_dir.glob("[0-9][0-9]-*.md"))) if bills_dir.exists() else 0
    if n_bills != len(legislation):
        sys.stderr.write(f"reasons-ledger: NOTE legislative rows={len(legislation)} vs bill files={n_bills}\n")

    out = []
    out.append("# Reasons Ledger")
    out.append("")
    out.append("The realm's pointer-only index of every **significant decision** (Bill 8 s. 5(2)) and its")
    out.append("**reasons-record** (the four s. 5(3) fields: WHAT was decided, WHY, what it RESTED ON, WHO/what")
    out.append("AUTHORISED it). This is the EXECUTIVE analogue of the rulings ledger")
    out.append("([`../ledger/INDEX.md`](../ledger/INDEX.md)).")
    out.append("")
    out.append("> **Derived, pointer-only, deterministically rebuildable. NOT a source of law or truth.**")
    out.append("> It holds no ratio, status, or authority - only pointers into the artefacts the realm already")
    out.append("> owns (Bill 8 s. 4(2), s. 24, s. 25; the index-not-replacement doctrine of [2026] REALM-PC 4).")
    out.append("> It is projected from three EXISTING traces: the citator(s), the bill record, and git history.")
    out.append("> Re-run `build-reasons-ledger.py` to regenerate it in lockstep with those sources. See README.md.")
    out.append("")
    out.append(f"Generated by the clerk: **{total} significant decisions** "
               f"({len(caselaw)} caselaw, {len(legislation)} legislative, {len(executive)} executive).")
    out.append("")

    def section(title, blurb, rows):
        out.append(f"## {title}")
        out.append("")
        out.append(blurb)
        out.append("")
        out.append("| Decision | Trigger | What was decided | Why | Rested on | Authorised by | Pointer |")
        out.append("|---|---|---|---|---|---|---|")
        for c in rows:
            out.append("| " + " | ".join([
                esc(c["label"]),
                esc(c["trigger"]),
                esc(c["decided"]),
                esc(c["why"]),
                esc(c["rested_on"]),
                esc(c["authorised_by"]),
                esc(c["pointer"]),
            ]) + " |")
        out.append("")

    section("Caselaw forks (from the citator)",
            "Every ruling is a court-convening fork (s. 5(2)(a)). Derived from the central citator "
            "`.justice/INDEX.md` and sub-repo citators; the ratio, cites, and ruling pointer are read "
            "from the citator row (no duplication; s. 4(2)).",
            caselaw)
    section("Legislative decisions (from the bill record)",
            "Each enacted Act is a significant decision (a statute). Derived from "
            "`legislature/bills/NN-*.md` headers + long title and the `ORDER-PAPER.md` / "
            "`SOVEREIGN-CONSULTATIONS.md` record. Authorised by the Sovereign Founder on Royal Assent.",
            legislation)
    section("Executive decisions (from git history)",
            "Significant-decision commits. Going forward, a commit carrying a `Reasons-record:` trailer "
            "yields a full record (the four s. 5(3) fields). Pre-convention significant commits (those "
            "touching `.justice/`, `statutes/`, `legislature/`, or recording an irreversible / outward "
            "act) are listed with the extractable subject and a commit-hash pointer; their reasons remain "
            "in the commit body (`git show <hash>`). See `.reasons-convention.md` for the trailer spec.",
            executive)

    LEDGER.write_text("\n".join(out).rstrip("\n") + "\n", encoding="utf-8")
    print(f"reasons-ledger: {total} significant decisions "
          f"({len(caselaw)} caselaw / {len(legislation)} legislative / {len(executive)} executive) "
          f"-> {LEDGER.relative_to(REALM)}")

if __name__ == "__main__":
    build()
