#!/usr/bin/env bash
set -euo pipefail

root=$(git rev-parse --show-toplevel)
cd "$root"
mode=${1:?expected init, staged or post-commit}
source_files=('*.rs' '*.java' '*.kt' '*.ts' '*.tsx' '*.vue' '*.js' '*.jsx' '*.mjs' '*.cjs' '*.xml' '*.yml' '*.yaml' '*.sql')

if [[ "$mode" == post-commit ]]; then
    while IFS= read -r -d '' file; do
        case "$file" in
            *.rs|*.java|*.kt|*.ts|*.tsx|*.vue|*.js|*.jsx|*.mjs|*.cjs|*.xml|*.yml|*.yaml|*.sql) mode=refresh; break ;;
        esac
    done < <(git diff-tree -z --root --first-parent -m --no-commit-id --name-only -r HEAD)
    [[ "$mode" == refresh ]] || exit 0
fi

if [[ "$mode" == staged ]]; then
    # Preserve Git's GIT_INDEX_FILE, including the temporary index used by git commit.
    if git diff --cached --quiet -- "${source_files[@]}"; then
        exit 0
    elif [[ $? != 1 ]]; then
        exit 1
    fi
fi

if command -v gitnexus >/dev/null 2>&1; then
    runner=(gitnexus)
else
    runner=(npx --yes gitnexus)
fi

case "$mode" in
    init|refresh)
        printf '[gitnexus] 刷新 worktree=%s commit=%s\n' "$root" "$(git rev-parse HEAD)"
        exec "${runner[@]}" analyze "$root" --index-only --allow-duplicate-name
        ;;
    staged)
        indexed_head=$(node -e 'try { console.log(require(process.argv[1]).lastCommit ?? "") } catch {}' "$root/.gitnexus/meta.json")
        if [[ -z "$indexed_head" || "$indexed_head" != "$(git rev-parse --verify HEAD 2>/dev/null)" ]] &&
            ! bash "$root/scripts/gitnexus/worktree.sh" init; then
            echo '[gitnexus] 覆盖未确认：无法初始化当前 worktree 的索引。' >&2
            git diff --cached --name-only -- "${source_files[@]}" >&2
            exit 0
        fi
        node "$root/scripts/gitnexus/staged.mjs" "$root" "${runner[@]}" || {
            echo '[gitnexus] 覆盖未确认，请核对以下暂存文件。' >&2
            git diff --cached --name-only -- "${source_files[@]}" >&2
        }
        ;;
    *) exit 2 ;;
esac
