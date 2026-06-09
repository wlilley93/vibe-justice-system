#!/usr/bin/env bash
# Thin adapter (REG-HOOKS-001): the kernel decides, this only points at it.
bin="${CLAUDE_PROJECT_DIR:-.}/target/debug/vjs"
[ -x "$bin" ] || exit 0
exec "$bin" hook --event pre_write --stdin-json
