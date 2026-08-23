#!/bin/sh
set -e
cp gate/pre-commit .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
echo "vjs: gate installed to .git/hooks/pre-commit"
