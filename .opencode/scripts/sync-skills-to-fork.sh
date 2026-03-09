#!/bin/bash
# 将本地 skills 修改同步到 superpowers fork

set -e

SKILLS_DIR=".opencode/skills"
FORK_REMOTE="superpowers"
FORK_BRANCH="main"

echo "🔄 同步 skills 到 fork..."

# 检查是否有未提交的修改
if ! git diff --quiet "$SKILLS_DIR" || ! git diff --cached --quiet "$SKILLS_DIR"; then
    echo "⚠️  检测到未提交的 skills 修改，请先提交："
    git status "$SKILLS_DIR" --short
    exit 1
fi

# 推送到 fork（使用 subtree push）
echo "📤 推送到 $FORK_REMOTE/$FORK_BRANCH..."
git subtree push --prefix="$SKILLS_DIR" "$FORK_REMOTE" "$FORK_BRANCH"

echo "✅ 同步完成！"
echo "🔗 查看你的 fork: https://github.com/SmallNew2003/superpowers"
