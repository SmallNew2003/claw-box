#!/usr/bin/env bash
set -euo pipefail

root=$(git rev-parse --show-toplevel)
cd "$root"
bash scripts/install-gitnexus-hooks.sh
bash scripts/gitnexus/worktree.sh init

if command -v gitnexus >/dev/null 2>&1; then
    gitnexus setup
else
    npx --yes gitnexus setup
fi
