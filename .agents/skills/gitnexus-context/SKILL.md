---
name: gitnexus-context
description: Use when analyzing code impact, tracing dependencies, or understanding call chains. Provides knowledge graph context for OpenSpec planning and debugging.
---

# GitNexus Context Skill

## 何时使用

此 skill 是其他 skills 的**辅助工具**，在以下场景调用：

1. **openspec-planning 阶段** - 创建 proposal.md 时分析变更影响范围
2. **systematic-debugging 阶段** - Phase 1 追踪调用链，Phase 2 查找工作示例
3. **代码重构前** - 评估重命名影响范围，检查隐藏依赖

## 快速开始

### 1. 检查是否可用

```bash
npx gitnexus status
```

### 2. 如果未安装

```bash
bash .opencode/skills/gitnexus-context/install.sh
```

### 3. 使用工具

根据场景选择工具（详见下方"核心工具"）

## 核心工具

### 1. impact - 影响分析
分析符号的依赖关系和影响范围

```bash
# 基本用法
impact({target: "UserService", direction: "upstream"})
```

**何时用**：OpenSpec proposal 写 Impact 部分、评估破坏性变更

详见：`tools/impact.md`

### 2. context - 360度上下文
查看符号的完整上下文（调用方、被调用方、所属进程）

```bash
# 基本用法
context({name: "validateUser"})
```

**何时用**：Systematic debugging Phase 1、理解陌生代码

详见：`tools/context.md`

### 3. query - 智能搜索
按进程分组的混合搜索（BM25 + 语义搜索）

```bash
# 基本用法
query({query: "authentication middleware"})
```

**何时用**：Systematic debugging Phase 2、查找工作示例

详见：`tools/query.md`

### 4. detect_changes - 变更检测
分析 git diff 的影响范围

```bash
# 基本用法
detect_changes({scope: "all"})
```

**何时用**：提交前检查影响、OpenSpec 提案前评估风险

详见：`tools/detect_changes.md`

## 工作流集成

### 与 openspec-planning 配合
创建 proposal.md 时，使用 `impact` 工具分析变更影响范围，将结果写入 Impact 部分

### 与 systematic-debugging 配合
Phase 1 使用 `context` 追踪调用链，Phase 2 使用 `query` 查找工作示例

详见：`workflows.md`

## 使用原则

1. **辅助工具** - 不替代现有 Process Skills，只在需要依赖分析时使用
2. **人工判断** - 工具给出的是建议，需要人工验证
3. **禁用冲突功能** - 不使用 `rename`（与 OpenSpec 冲突）、`wiki`（与文档规范冲突）

## 故障排查

常见问题快速解决：

```bash
# 检查索引状态
npx gitnexus status

# 列出已索引仓库
npx gitnexus list

# 重新索引
npx gitnexus analyze --force --skip-embeddings
```

多仓库场景需指定 repo 参数：
```bash
impact({target: "UserService", repo: "bj-gover-llm"})
```

更多问题详见：`troubleshooting.md`
