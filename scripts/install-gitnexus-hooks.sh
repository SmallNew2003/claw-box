#!/usr/bin/env bash
set -euo pipefail

mode=${1:-install}
case "$mode" in
    install|--check) ;;
    *) echo 'Usage: bash scripts/install-gitnexus-hooks.sh [--check]' >&2; exit 2 ;;
esac

root=$(git rev-parse --show-toplevel)
hook_source="$root/scripts/gitnexus/hook.sh"
common_dir=$(git rev-parse --path-format=absolute --git-common-dir)
hooks="$common_dir/hooks"

if git config --get core.hooksPath >/dev/null; then
    echo '已配置 core.hooksPath，请将 GitNexus 检查接入当前生效的钩子。' >&2
    exit 1
fi

rc=0
for hook in pre-commit post-commit; do
    target="$hooks/$hook"
    previous="$target.before-gitnexus"
    if [[ -x "$target" ]] && cmp -s "$hook_source" "$target"; then
        continue
    fi
    if [[ "$mode" == --check ]]; then
        echo "钩子缺失或过期：$target" >&2
        rc=1
        continue
    fi

    mkdir -p "$hooks"
    if [[ -L "$target" ]]; then
        echo "请将 GitNexus 接入现有符号链接钩子：$target" >&2
        rc=1
        continue
    fi
    if [[ -f "$target" ]] && ! grep -Fxq '# claw-box GitNexus hook dispatcher' "$target"; then
        if [[ -e "$previous" || -L "$previous" ]]; then
            echo "已有钩子的保留路径被占用：$previous" >&2
            rc=1
            continue
        fi
        cp -p "$target" "$previous"
    fi
    install -m 755 "$hook_source" "$target"
    echo "已安装 $target"
done
exit "$rc"
