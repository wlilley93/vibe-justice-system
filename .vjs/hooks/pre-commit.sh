#!/usr/bin/env bash
# Pre-commit gate: deterministic validation, not law prose.
exec vjs validate --staged
