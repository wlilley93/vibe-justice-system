#!/usr/bin/env bash
# Thin adapter (REG-HOOKS-001): call the kernel function; fail closed on exit.
args=(); for p in "$@"; do args+=(--path "$p"); done
exec vjs hook --event pre_write "${args[@]}"
