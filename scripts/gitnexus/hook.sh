#!/usr/bin/env bash
# claw-box GitNexus hook dispatcher
set -euo pipefail

old_hook="$0.before-gitnexus"
if [[ -x "$old_hook" ]]; then
    "$old_hook" "$@"
fi

root=$(git rev-parse --show-toplevel)
helper="$root/scripts/gitnexus/worktree.sh"
[[ -f "$helper" ]] || exit 0

case "${0##*/}" in
    pre-commit)
        bash "$helper" staged || echo '[gitnexus] 覆盖未确认：提交前分析执行失败。' >&2
        ;;
    post-commit)
        bash "$helper" post-commit || echo "[gitnexus] 索引刷新失败：$root" >&2
        ;;
esac
