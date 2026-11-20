#!/usr/bin/env bash
set -euo pipefail

source_root=$(git rev-parse --show-toplevel)
scratch=$(mktemp -d)
scratch=$(cd "$scratch" && pwd -P)
trap 'rm -rf "$scratch"' EXIT
export GIT_CONFIG_GLOBAL=/dev/null GIT_CONFIG_NOSYSTEM=1
export GITNEXUS_HOME="$scratch/index-registry"

main="$scratch/main"
worktree="$scratch/feature checkout"
git init -q "$main"
git -C "$main" config user.name 'Hook Tester'
git -C "$main" config user.email 'hook-tester@openai.com'
mkdir -p "$main/scripts/gitnexus"
cp "$source_root"/scripts/gitnexus/{worktree.sh,staged.mjs,hook.sh} "$main/scripts/gitnexus/"
cp "$source_root/scripts/install-gitnexus-hooks.sh" "$main/scripts/"
printf 'pub fn sample() -> u32 {\n    let value = 1;\n    value\n}\n' > "$main/sample.rs"
git -C "$main" add .
git -C "$main" commit -qm baseline
git -C "$main" worktree add -qb feature "$worktree"
for hook in pre-commit post-commit; do
    printf '#!/bin/sh\nprintf "legacy hook: %%s\\n" "%s"\n' "$hook" > "$main/.git/hooks/$hook"
    chmod +x "$main/.git/hooks/$hook"
done
(cd "$main" && bash scripts/install-gitnexus-hooks.sh && bash scripts/install-gitnexus-hooks.sh --check)
printf 'pub fn sample() -> u32 {\n    let value = 2;\n    value\n}\n' > "$worktree/sample.rs"
git -C "$worktree" add sample.rs
git -C "$worktree" commit -m feature > "$scratch/commit.log" 2>&1
cat "$scratch/commit.log"
grep -F '已映射 1 个源码文件' "$scratch/commit.log"
grep -F "刷新 worktree=$worktree commit=$(git -C "$worktree" rev-parse HEAD)" "$scratch/commit.log"
test -z "$(git -C "$main" status --porcelain)"
test ! -e "$main/.gitnexus"
test "$(node -p 'require(process.argv[1]).lastCommit' "$worktree/.gitnexus/meta.json")" = "$(git -C "$worktree" rev-parse HEAD)"
grep -Fx 'legacy hook: pre-commit' "$scratch/commit.log"
grep -Fx 'legacy hook: post-commit' "$scratch/commit.log"

printf 'pub fn sample() -> u32 {\n    let value = 3;\n    value\n}\n' > "$worktree/sample.rs"
git -C "$worktree" add sample.rs
printf '// Unstaged edit\n' >> "$worktree/sample.rs"
git -C "$worktree" commit -m partially-staged > "$scratch/partial.log" 2>&1
grep -F '覆盖未确认' "$scratch/partial.log"
grep -Fx 'sample.rs' "$scratch/partial.log"
git -C "$worktree" diff --name-only | grep -Fx 'sample.rs'
