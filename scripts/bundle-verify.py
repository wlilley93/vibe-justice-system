#!/usr/bin/env python3
"""Fail-closed verify for a deployment bundle.lock (REG-BUNDLE-001).

Checks the manifest: every component carries every prescribed field, every digest
is a well-formed sha256, and the licence firewall holds (no AGPL component consumed
into a permissive distribution boundary by co-resident source). Exits non-zero,
loudly, on the first violation. Enacted by [2026] VJS-PC 11."""
import sys, re

try:
    import tomllib
except ModuleNotFoundError:
    sys.exit("bundle-verify needs Python 3.11+ (tomllib)")

REQUIRED_TOP = ["schema_version", "bundle", "distribution_licence"]
REQUIRED_COMPONENT = ["id", "repo", "digest", "source_commit", "licence", "adoption_mode"]
SHA256 = re.compile(r"^sha256:[0-9a-f]{64}$")
COPYLEFT = {"AGPL-3.0-only", "AGPL-3.0", "GPL-3.0-only", "GPL-3.0", "LGPL-3.0"}
PERMISSIVE = {"MIT", "Apache-2.0", "BSD-3-Clause", "BSD-2-Clause", "ISC"}


def fail(msg):
    print(f"FAIL: {msg}")
    sys.exit(1)


def main(path):
    with open(path, "rb") as f:
        m = tomllib.load(f)
    for k in REQUIRED_TOP:
        if k not in m:
            fail(f"manifest is missing the prescribed top-level field '{k}'")
    comps = m.get("component", [])
    if not comps:
        fail("manifest declares no components")
    dist = m["distribution_licence"]
    seen = set()
    for c in comps:
        cid = c.get("id", "<unnamed>")
        for k in REQUIRED_COMPONENT:
            if not c.get(k):
                fail(f"component '{cid}' is missing the prescribed field '{k}'")
        if cid in seen:
            fail(f"duplicate component id '{cid}'")
        seen.add(cid)
        if not SHA256.match(c["digest"]):
            fail(f"component '{cid}' digest is not a well-formed sha256: {c['digest']}")
        # the licence firewall (PC-11): AGPL into a permissive boundary only as a
        # vendored, re-stamped, pinned copy
        if dist in PERMISSIVE and c["licence"] in COPYLEFT:
            if c["adoption_mode"] != "vendored-restamped-readonly":
                fail(
                    f"licence firewall: copyleft component '{cid}' ({c['licence']}) is "
                    f"consumed into a {dist} distribution boundary with adoption_mode "
                    f"'{c['adoption_mode']}'; AGPL is permitted only as "
                    f"vendored-restamped-readonly"
                )
    print(f"OK: bundle '{m['bundle']}' verified - {len(comps)} components, "
          f"distribution {dist}, licence firewall holds.")
    for c in comps:
        print(f"   {c['id']:8} {c['licence']:16} {c['adoption_mode']:28} {c['digest'][:19]}…")


if __name__ == "__main__":
    if len(sys.argv) != 2:
        sys.exit("usage: bundle-verify.py <bundle.lock>")
    main(sys.argv[1])
