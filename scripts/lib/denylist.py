"""The publication denylist, loaded FAIL-CLOSED.

[2026] VJS-CC-VJS 17 C3: an unreadable register is an ERROR naming the path, never an
empty one. `boundary-scan.sh` passed on FileNotFoundError and `promote-canonical.sh` did
worse - it exited 0 for the whole limb, so a missing register reported "clean".

C7: each entry is `<sha256-hex>  # added=YYYY-MM-DD class=<client|infra|synthetic>`, so
the hash is the field BEFORE the first '#', never the whole line. Every reader must split
the same way or the register silently stops matching anything.
"""
import os
import sys

DEFAULT = ".vjs/publication-denylist.txt"
HEX = set("0123456789abcdef")


def load(path=None):
    path = path or os.environ.get("VJS_DENYLIST") or DEFAULT
    try:
        raw = open(path, encoding="utf-8").read()
    except OSError as e:
        raise SystemExit(
            "FAIL: the publication denylist at %s could not be read (%s). A gate must "
            "treat an unreadable register as an ERROR and never as an empty one "
            "([2026] VJS-CC-VJS 17 C3)." % (path, e)
        )
    hashes = set()
    for n, line in enumerate(raw.splitlines(), 1):
        head = line.split("#", 1)[0].strip()
        if not head:
            continue
        if len(head) != 64 or any(c not in HEX for c in head):
            raise SystemExit(
                "FAIL: %s:%d is not a sha256 hash. Entries are hashes with a provenance "
                "comment, never plaintext ([2026] VJS-CC-VJS 17 C7)." % (path, n)
            )
        hashes.add(head)
    if not hashes:
        raise SystemExit(
            "FAIL: the publication denylist at %s carries no entries, so every gate keyed "
            "on it would fire on nothing while reporting itself as run "
            "([2026] VJS-CC-VJS 17 C3)." % path
        )
    return hashes


if __name__ == "__main__":
    for h in sorted(load(sys.argv[1] if len(sys.argv) > 1 else None)):
        print(h)
