#!/bin/bash
# 从 superpowers fork 拉取更新

set -e

SKILLS_DIR=".opencode/skills"
FORK_REMOTE="superpowers"
FORK_BRANCH="main"

echo "🔄 从 fork 拉取 skills 更新..."

# 检查是否有未提交的修改
if ! git diff --quiet "$SKILLS_DIR" || ! git diff --cached --quiet "$SKILLS_DIR"; then
    echo "⚠️  检测到未提交的 skills 修改，请先提交或暂存："
    git status "$SKILLS_DIR" --short
    exit 1
fi

# 从 fork 拉取更新（使用 subtree pull）
echo "📥 从 $FORK_REMOTE/$FORK_BRANCH 拉取..."
git subtree pull --prefix="$SKILLS_DIR" "$FORK_REMOTE" "$FORK_BRANCH" --squash

echo "✅ 更新完成！"
echo ""
echo "⚠️  请检查是否有冲突，参考 .opencode/SKILLS_CUSTOMIZATIONS.md 解决"
