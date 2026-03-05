# AI Agent 工作指南

## 快速开始

**收到用户消息 → 立即调用 `using-superpowers` skill**

这是唯一需要记住的规则。所有其他规则都在 skills 中按需加载。

## 核心原则

1. **必须先调用 `using-superpowers` skill**
   - 即使只有 1% 可能性也必须检查
   - 在做任何响应（包括澄清问题）之前先执行
   - 这是强制要求，不可跳过

2. **调用 skill 后的标准流程**
   - 宣布使用目的："使用 [skill] 来 [目的]"
   - 如果 skill 有 checklist，创建 TodoWrite 任务
   - 严格遵循 skill 的指引

3. **所有详细规则都在 skills 中**
   - 不要试图记住所有规则
   - 按需调用对应的 skill
   - Skills 会告诉你该做什么

## 反模式警告（Red Flags）

如果你有以下想法，立即停止 - 你在合理化跳过 skill：

| 想法 | 真相 |
|------|------|
| "这只是个简单问题" | 问题也是任务，检查 skills |
| "我需要先了解上下文" | Skill 会告诉你如何了解 |
| "让我先探索代码库" | Skills 告诉你如何探索 |
| "我记得这个 skill" | Skills 会演进，重新读取 |
| "这不需要正式的 skill" | 如果 skill 存在，就使用它 |
| "我先做这一件事" | 先检查 skills 再做任何事 |

## 基本规则

- 使用中文回复
- 提交格式：`feat(模块): 中文内容`
- 新功能/破坏性变更必须先通过 `npm run openspec:preflight`
- **禁止使用 emoji**（适用于所有输出：代码、文档、skills、注释）
- 禁止创建临时分析文档（`*_ANALYSIS.md`、`*_REVIEW.md` 等）
- Skills 以 `.agents/skills` 为唯一来源

**重要**：这些规则适用于你创建的所有内容，包括：
- 代码和注释
- 文档和 skills
- 提交信息
- 与用户的对话

## 工作流总览

```
using-superpowers (总调度)
    ↓
    ├─ 信息查询 → 读取 `openspec/project.md`
    ├─ 需求不明确 → brainstorming → openspec-planning
    ├─ 新功能/破坏性变更 → openspec-planning
    ├─ Bug 修复 → systematic-debugging
    ├─ Java 代码 → 读取 `openspec/project.md` § Java 规范
    └─ 其他场景 → 对应 skill

openspec-planning (规范化提案)
    ↓
executing-plans (执行实现)
    ↓
verification-before-completion (验证)
    ↓
openspec archive (归档)
```

## 任务状态同步（强制）

完成任务时**必须同时**更新：
1. TodoWrite 工具标记 completed
2. `openspec/changes/*/tasks.md` 改为 `- [x]`

## 常用 Skills 速查

| 场景 | 调用 Skill |
|------|-----------|
| 收到任何消息 | `using-superpowers` |
| 新功能（需求不明确） | `brainstorming` → `openspec-planning` |
| 新功能（需求明确） | `openspec-planning` |
| Bug 修复 | `systematic-debugging` |
| 完成前验证 | `verification-before-completion` |

详细决策流程在 `using-superpowers` skill 中。

## 常用文档速查

| 需求 | 文档位置 |
|------|---------|
| 项目上下文、技术栈 | `openspec/project.md` 第 1-145 行 |
| Java 开发规范 | `openspec/project.md` 第 49-93 行 |
| OpenSpec 完整流程 | `openspec/AGENTS.md` |
| 规范变更历史 | `openspec/CHANGELOG.md` |

## Skill 优先级

当多个 skills 可能适用时：

1. **Process skills 优先**（brainstorming, debugging）- 决定如何处理任务
2. **Implementation skills 其次**（领域实现相关 skills）- 指导执行

示例：
- "构建 X" → brainstorming 优先，然后实现 skills
- "修复 bug" → debugging 优先，然后领域 skills

## Skill 类型

- **Rigid**（TDD, debugging）：严格遵循，不要适应掉纪律
- **Flexible**（patterns）：根据上下文调整原则

Skill 本身会告诉你它是哪种类型。

## 详细规范（按需查阅）

- **OpenSpec 规范**：调用 `openspec-planning` skill 时自动加载
- **项目上下文**：读取 `openspec/project.md`
- **Java 规范**：读取 `openspec/project.md` § Java 规范
- **完整 OpenSpec 文档**：`openspec/AGENTS.md`（仅在遇到具体问题时读取）
- **项目设计文档**：`openspec/project.md`（仅在需要深入了解时读取）
