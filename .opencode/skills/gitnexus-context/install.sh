#!/bin/bash
set -e

echo "🔍 检查 GitNexus 安装状态..."

# 检查当前项目是否已索引
if npx gitnexus status 2>/dev/null | grep -q "Index found"; then
    echo "✓ 当前项目已索引"
    npx gitnexus list
    exit 0
fi

echo "📦 开始安装和索引 GitNexus..."

# 索引当前项目
echo "→ 索引主项目..."
npx gitnexus analyze --skip-embeddings

# 检测并索引子项目
echo "→ 检测子项目..."
subprojects=$(find . -maxdepth 2 -name "package.json" -not -path "./node_modules/*" -not -path "./package.json" | sed 's|/package.json||')

if [ -n "$subprojects" ]; then
    echo "发现子项目："
    echo "$subprojects"
    
    for subproject in $subprojects; do
        echo "→ 索引子项目: $subproject"
        (cd "$subproject" && npx gitnexus analyze --skip-embeddings)
    done
fi

# 配置 MCP
echo "→ 配置 MCP 服务器..."
npx gitnexus setup

# 验证安装
echo ""
echo "✓ GitNexus 安装完成"
echo ""
echo "已索引的仓库："
npx gitnexus list

echo ""
echo "现在可以使用以下工具："
echo "  - impact: 分析变更影响范围"
echo "  - context: 查看符号的完整上下文"
echo "  - query: 智能搜索代码"
echo "  - detect_changes: 检测 git diff 影响"
