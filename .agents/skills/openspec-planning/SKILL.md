---
name: openspec-planning
description: Use when creating proposals, specs, or planning changes. Triggers include "proposal", "spec", "等我确认", or new features/breaking changes. This is a PROCESS skill - invoke BEFORE implementation.
---

# OpenSpec Planning Workflow

## 触发条件

必须使用此 skill：
- 用户说"等我确认"、"proposal"、"spec"、"计划"
- 新功能或新能力
- 破坏性变更（API、schema、架构）
- 性能优化（改变行为）
- 安全策略变更
- 用户要求创建提案或规范

跳过此 skill：
- Bug 修复（恢复既定行为）
- 拼写、格式、注释
- 纯重构（不改变行为）
- 依赖更新（非破坏性）
- 为既有行为补充测试

## 核心原则

与 superpowers 配合：
1. 这是 Process Skill - 在实现前调用
2. 如果用户请求涉及新功能，先调用 `brainstorming`，再调用此 skill
3. 计划完成并获得用户批准后，才开始实现

OpenSpec 哲学：
- 先对齐需求，再写代码
- 轻量级流程，不是瀑布模型
- 规范与代码共存，持续演进

## 快速工作流

### 0. 检测输入来源（新增）

**场景 A：从 brainstorming 转换**
1. 检查 `docs/plans/` 是否有最近的设计文档（最近 1 小时内创建）
2. 如果有，读取设计文档并转换为 OpenSpec 格式
3. 跳转到步骤 2

**场景 B：直接创建（需求明确）**
1. 用户已经清楚要做什么
2. 继续步骤 1

### 1. 检查现有工作

```bash
openspec list                    # 查看活跃的 changes
openspec list --specs            # 查看现有 capabilities
```

### 2. 创建目录结构

```bash
# 选择唯一的 change-id（kebab-case，动词开头）
mkdir -p openspec/changes/<change-id>/specs/<capability>
```

### 3. 编写 artifacts

**如果从 brainstorming 转换**，使用以下映射：

| Brainstorming 输出 | OpenSpec 文件 | 提取内容 |
|-------------------|--------------|---------|
| 设计文档 § Purpose/Goals | `proposal.md` § Why | 目标、背景、问题 |
| 设计文档 § Recommended Approach | `proposal.md` § What Changes | 推荐方案、变更内容 |
| 设计文档 § Components/Architecture | `proposal.md` § Impact | 影响的模块、文件 |
| 设计文档 § Architecture/Decisions | `design.md` | 技术决策、方案对比 |
| 设计文档 § Components/Features | `specs/[capability]/spec.md` | 功能需求、场景 |
| 设计文档 § Implementation Steps | `tasks.md` | 实现步骤 |

**如果直接创建**，按顺序创建：
1. `proposal.md` - 为什么做、做什么、影响范围
2. `specs/<capability>/spec.md` - 规范增量（ADDED/MODIFIED/REMOVED）
3. `tasks.md` - 实现清单
4. `design.md`（可选）- 技术决策

需要模板和示例时，读取：
- `reference/proposal-template.md`
- `reference/spec-template.md`
- `reference/tasks-template.md`

### 4. 验证

```bash
openspec validate <change-id> --strict --no-interactive
```

修复所有问题后再分享给用户。

### 5. 等待批准

在用户明确批准前，不要开始实现。

用户批准后：
1. 使用 TodoWrite 创建任务列表（基于 tasks.md）
2. 标记第一个 todo 为 in_progress
3. 开始实现
4. **每完成一个任务，同步更新两处状态**：
   - TodoWrite 中标记为 completed
   - tasks.md 中将 `- [ ]` 改为 `- [x]`

## 决策树

```
收到用户请求
    ↓
是否涉及新功能/破坏性变更？
    ↓ 是
需要探索/头脑风暴？
    ↓ 是
调用 brainstorming skill
    ↓
调用 openspec-planning skill（本 skill）
    ↓
创建 proposal.md
    ↓
创建 specs/ 增量
    ↓
创建 tasks.md
    ↓
（可选）创建 design.md
    ↓
运行 openspec validate --strict
    ↓
展示给用户，等待批准
    ↓
用户批准？
    ↓ 是
开始实现（遵循 tasks.md）
```

## 与其他 Skills 的配合

### 与 brainstorming 配合（增强）

**完整流程**：
```
用户: "我想添加暗黑模式"
  ↓
using-superpowers: 检查 skills
  ↓
调用 brainstorming（探索需求）
  ↓
brainstorming 输出: docs/plans/2026-03-05-dark-mode-design.md
  ↓
调用 openspec-planning（本 skill）
  ↓
检测到 docs/plans/ 有最近的设计文档
  ↓
读取并转换为 OpenSpec 格式:
  - proposal.md ← 提取 Why/What/Impact
  - design.md ← 提取技术决策
  - specs/ui-theme/spec.md ← 提取功能需求
  - tasks.md ← 提取实现步骤
  ↓
运行 openspec validate --strict
  ↓
展示给用户，等待批准
  ↓
用户批准
  ↓
开始实现
```

**转换示例**：

假设 brainstorming 输出：
```markdown
# Dark Mode Design

## Purpose
Users need a dark theme to reduce eye strain in low-light environments.

## Recommended Approach
Use CSS variables for theme switching...

## Components
- ThemeProvider component
- Theme toggle button
- CSS variable definitions
```

转换为 OpenSpec：

`proposal.md`:
```markdown
# Change: 添加暗黑模式支持

## Why
用户需要暗黑主题以减少低光环境下的眼睛疲劳。

## What Changes
- 添加主题切换功能
- 使用 CSS 变量实现主题切换
- 添加主题切换按钮

## Impact
- Affected specs: ui-theme
- Affected code: src/components/ThemeProvider.vue, src/styles/variables.css
```

`specs/ui-theme/spec.md`:
```markdown
## ADDED Requirements

### Requirement: 暗黑模式切换
系统 SHALL 提供暗黑模式切换功能。

#### Scenario: 用户切换到暗黑模式
- **WHEN** 用户点击主题切换按钮
- **THEN** 界面切换为暗黑主题
- **AND** 用户偏好保存到 localStorage
```

### 与 writing-plans 配合
- writing-plans：仅用于探索性开发，输出到临时目录
- openspec-planning：正式变更的需求、行为、验收标准（放在 `openspec/changes`）

### 与 test-driven-development 配合
```
OpenSpec 提案批准
  ↓
调用 test-driven-development
  ↓
根据 specs/ 中的 Scenarios 编写测试
  ↓
实现功能使测试通过
```

## 文档分工

- **需求/行为/验收**：`openspec/specs`、`openspec/changes`
- **执行步骤/验证命令**：`openspec/changes/*/tasks.md`（可以写得很详细）
- **技术设计**：`openspec/changes/<id>/design.md`（可选）
- **系统级设计**：`openspec/project.md`

## 关键提醒

1. 批准门槛：未经批准不得实现
2. 唯一 change-id：kebab-case，动词开头
3. 验证优先：分享前运行 `openspec validate --strict`
4. 引用规范：需要项目上下文时，读取 `openspec/project.md`
5. 中文提交：提交格式 `feat(模块): 中文内容`
6. **任务状态同步**：完成任务时必须同时更新 TodoWrite 和 tasks.md 的 checkbox

## 按需参考

遇到具体问题时，使用 Read 工具读取对应文档的相关章节：
- `openspec/AGENTS.md` - 完整的 OpenSpec 规范（CLI 命令、delta 操作规则、故障排查）
- `openspec/project.md` - 项目约定、架构、领域模型
