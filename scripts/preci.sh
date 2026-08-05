#!/usr/bin/env bash
# canon preCI: the full local gate set, run by .vjs/hooks/pre-push so a red push
# never leaves the machine (the subscriber has had this net since 2026-06-10; canon
# ran on manual suite runs until 2026-08-05). PRECI_SKIP=1 git push bypasses for a
# docs-only push. The suite's fixture gits scrub the hook environment (Defect-5
# class), so running it FROM a hook is safe - measured, not assumed.
set -euo pipefail
root="$(git rev-parse --show-toplevel)"
cd "$root"
if [ "${PRECI_SKIP:-0}" = "1" ]; then
  echo "preci: SKIPPED (PRECI_SKIP=1) - lawful only for a docs-only push."
  exit 0
fi
echo "=== preci: cargo fmt --check"
cargo fmt --all --check
echo "=== preci: clippy -D warnings"
cargo clippy --workspace --all-targets -- -D warnings
echo "=== preci: workspace suite"
cargo test --workspace
echo "=== preci: vjs local-ci (binary freshness is the kernel's own gate at validate)"
bin=""
for c in bin/vjs target/release/vjs target/debug/vjs; do
  [ -x "$root/$c" ] && bin="$root/$c" && break
done
[ -n "$bin" ] || bin=vjs
case "$bin" in
  "$root/target/"*)
    if [ -n "$(find "$root/crates" -path '*/src/*' -name '*.rs' -newer "$bin" -print -quit 2>/dev/null)" ]; then
      echo "vjs gate binary is STALE relative to crates/*/src - rebuild: cargo build" >&2
      exit 1
    fi ;;
esac
"$bin" local-ci
"$bin" validate
echo "preci: GREEN - safe to push."
