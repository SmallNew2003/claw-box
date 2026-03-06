#!/bin/bash
# 规则检查脚本

echo "检查规则违规..."

# 检查 emoji
echo "1. 检查 emoji..."
if git diff --cached --name-only | xargs grep -l "✅\|❌\|⏳\|📝\|🔧\|💡" 2>/dev/null; then
    echo "错误：发现 emoji，请移除"
    exit 1
fi

# 检查临时文档
echo "2. 检查临时文档..."
if git diff --cached --name-only | grep -E "_SUMMARY\.md$|_ANALYSIS\.md$|_REVIEW\.md$|_NOTES\.md$"; then
    echo "错误：发现临时文档，请删除或重命名"
    exit 1
fi

echo "规则检查通过"
