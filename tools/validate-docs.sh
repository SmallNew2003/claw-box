#!/bin/bash
# 文档规范验证脚本
# 用途：检查 OpenSpec 文档是否违反编写规范

set -e

ERRORS=0

echo "检查文档规范..."

# 1. 检查 emoji
echo "1. 检查 emoji 使用..."
if grep -r -P "[\x{1F300}-\x{1F9FF}\x{2600}-\x{26FF}\x{2700}-\x{27BF}]" openspec/changes/ 2>/dev/null; then
    echo "   [错误] 检测到 emoji，请移除"
    ERRORS=$((ERRORS + 1))
else
    echo "   [通过] 未检测到 emoji"
fi

# 2. 检查常见 emoji 字符
echo "2. 检查常见 emoji 字符..."
EMOJI_PATTERNS="✅|❌|⚠️|📋|🎯|🔧|📊|📝|🚀|📚|⏱️|📦|💡|🔥|👍|👎"
if grep -r -E "$EMOJI_PATTERNS" openspec/changes/ 2>/dev/null; then
    echo "   [错误] 检测到常见 emoji，请使用文本替代"
    echo "   提示：使用 [正确] [错误] [警告] [注意] 等文本"
    ERRORS=$((ERRORS + 1))
else
    echo "   [通过] 未检测到常见 emoji"
fi

# 3. 检查禁止的文档类型
echo "3. 检查禁止的文档类型..."
FORBIDDEN_DOCS=$(find openspec/changes/ -type f \( \
    -name "*_ANALYSIS.md" -o \
    -name "*_REVIEW.md" -o \
    -name "SUMMARY.md" -o \
    -name "REPORT.md" -o \
    -name "OVERVIEW.md" -o \
    -name "TEMP_*.md" -o \
    -name "DRAFT_*.md" \
\) 2>/dev/null)

if [ -n "$FORBIDDEN_DOCS" ]; then
    echo "   [错误] 检测到禁止的文档类型："
    echo "$FORBIDDEN_DOCS" | sed 's/^/      /'
    ERRORS=$((ERRORS + 1))
else
    echo "   [通过] 未检测到禁止的文档类型"
fi

# 4. 检查 OpenSpec 必需文档
echo "4. 检查 OpenSpec 必需文档..."

# 检查活跃 changes
for change_dir in openspec/changes/*/; do
    if [ -d "$change_dir" ]; then
        change_name=$(basename "$change_dir")
        
        # 跳过 archive 目录
        if [[ "$change_name" == "archive" ]]; then
            continue
        fi

        # 检查 proposal.md
        if [ ! -f "${change_dir}proposal.md" ]; then
            echo "   [警告] $change_name 缺少 proposal.md"
        fi

        # 检查 tasks.md
        if [ ! -f "${change_dir}tasks.md" ]; then
            echo "   [警告] $change_name 缺少 tasks.md"
        fi

        # 检查 specs 目录
        if [ ! -d "${change_dir}specs" ]; then
            echo "   [警告] $change_name 缺少 specs/ 目录"
        fi
    fi
done

# 检查 spec.md 格式
echo "   检查 spec.md 格式..."
for spec_file in openspec/changes/*/specs/*/spec.md; do
    if [ -f "$spec_file" ]; then
        # 检查是否有 Scenario
        if ! grep -q "^#### Scenario:" "$spec_file"; then
            echo "   [警告] $spec_file 缺少 Scenario"
        fi
        
        # 检查是否有 delta 操作
        if ! grep -qE "^## (ADDED|MODIFIED|REMOVED|RENAMED) Requirements" "$spec_file"; then
            echo "   [警告] $spec_file 缺少 delta 操作标记"
        fi
    fi
done

# 检查归档 changes 的日期格式
for archive_dir in openspec/changes/archive/*/; do
    if [ -d "$archive_dir" ]; then
        archive_name=$(basename "$archive_dir")
        
        # 检查日期格式 YYYY-MM-DD-*
        if ! echo "$archive_name" | grep -qE '^[0-9]{4}-[0-9]{2}-[0-9]{2}-'; then
            echo "   [警告] 归档 $archive_name 日期格式不正确（应为 YYYY-MM-DD-<change-id>）"
        fi
    fi
done

echo "   [完成] OpenSpec 结构检查完成"

# 5. 检查任务状态
echo ""
echo "5. 检查任务状态..."
./tools/validate-task-sync.sh

# 6. 总结
echo ""
echo "================================"
if [ $ERRORS -eq 0 ]; then
    echo "验证通过：所有文档符合规范"
    exit 0
else
    echo "验证失败：发现 $ERRORS 个错误"
    echo "请修复后重新提交"
    exit 1
fi
