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

### 0. 检测输入来源并验证完整性

**场景 A：从 brainstorming 转换**
1. 接收 writing-plans 传递的 change-id
2. 检查 `openspec/changes/<change-id>/design.md` 是否存在
3. 如果存在，读取 design.md 并验证完整性（见下方验证清单）
4. 如果验证通过，使用 design.md 内容创建其他 OpenSpec 文件
5. 跳转到步骤 3（编写 artifacts）

**场景 B：直接创建（需求明确）**
1. 用户已经清楚要做什么
2. 验证需求完整性（见下方验证清单）
3. 继续步骤 1（检查现有工作）

**输入完整性验证清单**：
在创建 OpenSpec 提案前，必须确认以下所有项都已明确：
- [ ] 技术方案已选定（没有"可以用 A 或 B"的情况）
- [ ] 数据结构和 schema 已定义
- [ ] API 接口和契约已明确
- [ ] 错误处理策略已确定
- [ ] 依赖和集成点已识别
- [ ] 性能/安全要求已明确

**如果验证失败**：
- 列出所有未决策的项目
- 告知用户："设计中还有以下未决策项，需要先在 brainstorming 阶段解决"
- 返回 brainstorming 或直接向用户提问澄清
- 不要创建包含 "TBD"、"待定"、"可选方案 A/B" 的提案

**design.md 验证失败处理**：
如果 design.md 存在但格式不符合预期：
- 检查必需章节：Context、Goals、Decisions、Architecture
- 如果缺少章节，列出缺失项并询问用户：
  1. 补充缺失内容（返回 brainstorming/writing-plans）
  2. 手动提供缺失信息
  3. 跳过 design.md，直接创建 proposal.md（适用于简单变更）

### 1. 检查现有工作

```bash
openspec list                    # 查看活跃的 changes
openspec list --specs            # 查看现有 capabilities
```

### 2. 创建目录结构

```bash
# Change-id 必须包含日期前缀：YYYY-MM-DD-<description>
# 例如：2026-03-06-add-dark-mode

# 如果从 brainstorming 转换，change-id 已由 writing-plans 创建
# 如果直接创建，生成带日期的 change-id

# 检查 change-id 是否已存在
if [ -d "openspec/changes/<change-id>" ]; then
  # 如果目录存在但只有 design.md，继续（场景 A）
  # 如果目录存在且有其他文件，报错并要求选择不同名称
  echo "错误：change-id 已存在且包含其他文件，请选择不同的名称"
  exit 1
fi

# 检查归档目录（虽然有日期前缀，但仍需检查）
if ls openspec/changes/archive/ | grep -q "<change-id>"; then
  echo "警告：归档中存在相同的 change-id，建议使用不同描述"
fi

# 创建目录结构
mkdir -p openspec/changes/<change-id>/specs/<capability>
```

### 3. 编写 artifacts

**文档编写规范（必须遵守）**：
- 禁止使用任何 emoji（包括 ✅ ❌ ⚠️ 📋 🎯 等）
- 使用文本替代：[正确] [错误] [警告] [注意]
- 禁止创建 SUMMARY.md、OVERVIEW.md 等总结文档
- 仅创建 OpenSpec 要求的文档：proposal.md、design.md、tasks.md、spec.md

**文档位置检查（创建前必做）**：
在创建或修改任何文档前，必须先确认文档应该放在哪里：
1. 这是项目上下文吗？→ `openspec/project.md`（只放高层信息，不放详细规范）
2. 这是开发规范吗？→ `openspec/specs/development-standards/spec.md`
3. 这是数据库设计规范吗？→ `openspec/specs/database-design-standards/spec.md`
4. 这是功能需求吗？→ `openspec/specs/<capability>/spec.md`
5. 这是技术设计吗？→ `openspec/changes/<change-id>/design.md`

**project.md 内容规范**：
- [正确] 项目定位、目标、技术栈列表、架构模式概述、领域上下文、职责边界、外部依赖、重要约束
- [错误] 详细的开发规范（应放在 `specs/development-standards/spec.md`）
- [错误] 详细的数据库设计规范（应放在 `specs/database-design-standards/spec.md`）
- [错误] 代码示例和最佳实践（应放在 `specs/` 或 `docs/`）

详细规则参考：`openspec/DOCUMENTATION.md`

**如果从 brainstorming 转换**，使用以下映射：

| Design.md 章节 | OpenSpec 文件 | 提取内容 |
|---------------|--------------|---------|
| Context/Goals | `proposal.md` § Why | 目标、背景、问题 |
| Decisions/Architecture | `proposal.md` § What Changes | 推荐方案、变更内容 |
| Context/Decisions | `proposal.md` § Impact | 影响的模块、文件 |
| (自动生成) | `proposal.md` § Metadata | 创建时间、受影响 spec 的修改时间 |
| (已存在) | `design.md` | 保持不变，已由 writing-plans 创建 |
| Goals/Decisions | `specs/[capability]/spec.md` | 功能需求、场景 |
| Migration Plan/Decisions | `tasks.md` | 实现步骤 |

**如果直接创建**，按顺序创建：
1. `proposal.md` - 为什么做、做什么、影响范围、元数据（包含创建时间和受影响 spec 的修改时间）
2. `design.md`（可选）- 技术决策（仅在需要时创建）
3. `specs/<capability>/spec.md` - 规范增量（ADDED/MODIFIED/REMOVED）
4. `tasks.md` - 实现清单

需要模板和示例时，读取：
- `reference/proposal-template.md`
- `reference/spec-template.md`
- `reference/tasks-template.md`

### 4. 验证

```bash
openspec validate <change-id> --strict --no-interactive
```

修复所有问题后再分享给用户。

### 5. 等待批准并自动执行

在用户明确批准前，不要开始实现。

**用户批准后，自动判断并执行**：

```
读取 tasks.md，统计任务数量
  ↓
任务数量 >= 3？
  ↓ 是（默认）
自动调用 subagent-driven-development skill
  ↓
为每个任务创建独立子代理并行执行
```

```
任务数量 = 1-2？
  ↓ 是
检查任务复杂度
  ↓
简单任务？
  ↓ 是
主会话直接执行
  ↓
复杂任务？
  ↓ 是
调用 subagent-driven-development
```

**强制规则**：
- **3+ 个任务 → 必须使用 subagent-driven-development**
- **1-2 个任务 → 可以主会话执行（如果简单）**
- **任务高度耦合 → 询问用户是否使用 executing-plans**

**禁止**：
- 在主会话中逐个执行 3+ 个任务
- 不调用子代理就开始实现多任务
- 假设用户想要串行执行

**例外**：
- 用户明确说"不要用子代理"
- 用户明确说"串行执行"

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
brainstorming 完成设计验证
    ↓
调用 openspec-planning skill（本 skill）
    ↓
验证输入完整性（检查清单）
    ↓
有未决策项？
    ↓ 是
返回 brainstorming 或向用户提问
    ↓ 否
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
读取 tasks.md，统计任务数量
    ↓
任务数量 >= 3？
    ↓ 是
自动调用 subagent-driven-development
    ↓ 否（1-2 个任务）
检查任务复杂度
    ↓
简单？→ 主会话直接执行
复杂？→ 调用 subagent-driven-development
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
brainstorming 调用 writing-plans
  ↓
writing-plans 输出: openspec/changes/<change-id>/design.md
  ↓
调用 openspec-planning（本 skill）
  ↓
检测到 changes/<change-id>/design.md 已存在
  ↓
读取 design.md 并创建其他 OpenSpec 文件:
  - proposal.md ← 提取 Why/What/Impact
  - specs/ui-theme/spec.md ← 提取功能需求
  - tasks.md ← 提取实现步骤
  - design.md ← 已存在，保持不变
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

假设 writing-plans 已创建 `openspec/changes/add-dark-mode/design.md`：
```markdown
# Dark Mode Design

## Context
Users need a dark theme to reduce eye strain in low-light environments.

## Goals
- Provide seamless theme switching
- Persist user preference

## Architecture
Use CSS variables for theme switching...

## Decisions
- Decision: CSS variables over styled-components
- Alternatives: styled-components (more bundle size), Tailwind dark mode (requires Tailwind)
```

openspec-planning 读取后创建：

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
- writing-plans：由 brainstorming 调用，创建 `changes/<change-id>/design.md`
- openspec-planning：读取 design.md，创建完整的 OpenSpec 提案（proposal.md、specs/、tasks.md）

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
