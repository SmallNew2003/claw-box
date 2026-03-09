# Superflows - OpenSpec 工作流技能体系

Superflows 是一套完整的 OpenSpec 工作流实现，适用于任何采用 OpenSpec 方法论的项目。

## 什么是 Superflows

Superflows 提供了从需求探索到代码交付的完整工作流：
- 规范化的需求管理和变更流程
- 强制的验证和审查机制
- 自动化的文档和任务同步
- 可追溯的决策和实现历史

**与通用 skills 的区别**：
- **通用 skills**：独立的工具和模式（如 systematic-debugging、test-driven-development）
- **Superflows**：完整的工作流链，skills 之间紧密协作

## 完整工作流

```
using-superpowers (总调度)
    ↓
brainstorming (探索需求)
    ↓
writing-plans (编写设计)
    ↓
openspec-planning (创建提案)
    ↓
用户批准
    ↓
执行方式选择：
    ├─→ subagent-driven-development (默认，3+ 独立任务)
    ├─→ executing-plans (备选，任务高度耦合)
    └─→ 主会话直接执行 (1-2 个简单任务)
    ↓
verification-before-completion (验证完成)
    ↓
requesting-code-review (请求审查)
    ↓
receiving-code-review (接收反馈)
    ↓
finishing-a-development-branch (完成分支)
```

## Skills 详细说明

### 规划阶段

#### using-superpowers
**用途**：总调度器，决定调用哪个 skill

**触发条件**：收到任何用户消息

**核心功能**：
- 分析任务类型
- 决定调用哪个 skill
- 提供决策树和场景表

**关键规则**：
- 即使 1% 可能性也要检查 skills
- 在任何响应（包括澄清问题）前先调用

#### brainstorming
**用途**：探索需求，澄清设计

**触发条件**：
- 需求不明确
- 新功能或新能力
- 用户说"我想..."、"能不能..."

**核心功能**：
- 理解项目上下文
- 一次一个问题澄清需求
- 提出 2-3 种方案供选择
- 分段展示设计并获得批准
- 完成后调用 writing-plans

**关键规则**：
- 必须等待用户选择方案
- 不能假设用户意图
- 设计必须获得批准才能继续

#### writing-plans
**用途**：编写技术设计文档

**触发条件**：brainstorming 完成后自动调用

**核心功能**：
- 生成 change-id（格式：YYYY-MM-DD-<description>）
- 创建 `openspec/changes/<change-id>/design.md`
- 记录技术决策、架构、权衡
- 完成后自动调用 openspec-planning

**输出格式**：
```markdown
# [Feature Name] Design

## Context
## Goals / Non-Goals
## Architecture
## Decisions
## Risks / Trade-offs
## Migration Plan
```

#### openspec-planning
**用途**：创建 OpenSpec 提案

**触发条件**：
- writing-plans 完成后自动调用
- 用户说"等我确认"、"proposal"、"spec"
- 需求明确的新功能

**核心功能**：
- 创建 proposal.md（为什么做、做什么、影响范围）
- 创建 specs/<capability>/spec.md（需求增量）
- 创建 tasks.md（实现清单）
- 运行 openspec validate 验证
- 等待用户批准

**关键规则**：
- 必须验证输入完整性（无 TBD、无待定）
- 必须通过 openspec validate
- 必须等待用户批准才能开始实现

#### document-location
**用途**：确保文档放在正确位置

**触发条件**：创建或修改任何文档前

**核心功能**：
- 文档位置决策树
- 防止将详细规范混入 project.md
- 防止创建临时总结文档
- 强制执行文档组织规范

### 执行阶段

#### subagent-driven-development（默认执行方式）
**用途**：使用子代理并行执行任务

**触发条件**：
- 用户批准提案后（默认）
- 3+ 个独立任务
- 多文件修改、批量重构、文档生成等

**核心功能**：
- 为每个任务创建独立子代理
- 并行执行，互不干扰
- 两阶段审查：规范符合性 → 代码质量
- 自动同步 TodoWrite 和 tasks.md

**关键优势**：
- **节省 60-70% tokens**：每个子代理独立上下文，不累积
- **并行执行**：多个任务同时进行，速度更快
- **失败隔离**：一个任务失败不影响其他
- **专注度高**：每个子代理只关注单一任务
- **可重试**：失败的任务可以单独重试

**Token 消耗对比**：
```
单会话（executing-plans）：
任务1: 10K + 任务2: 20K + 任务3: 35K + 任务4: 55K = 120K tokens

子代理（subagent-driven-development）：
任务1: 10K + 任务2: 10K + 任务3: 10K + 任务4: 10K = 40K tokens

节省：66% tokens
```

**适用场景**：
- 多文件修改（每个文件一个子代理）
- 批量重构（每个模块一个子代理）
- 文档生成（每个文档一个子代理）
- 测试编写（每个测试套件一个子代理）
- 代码审查（每个文件一个审查子代理）
- 数据迁移（每个迁移脚本一个子代理）

#### executing-plans（备选执行方式）
**用途**：串行执行高度耦合的任务

**触发条件**：
- 任务高度耦合（必须按顺序执行）
- 需要共享复杂状态
- 用户明确要求串行执行

**核心功能**：
- 读取 tasks.md
- 创建 TodoWrite 任务列表
- 批量执行任务（默认前 3 个）
- 每完成一个任务同步更新 TodoWrite 和 tasks.md
- 批次间等待审查

**关键规则**：
- 必须同步更新 TodoWrite 和 tasks.md
- 不能批量更新，完成一个更新一个
- 批次间必须等待审查

**何时使用**：
- 任务必须严格按顺序执行
- 后续任务依赖前面任务的输出
- 需要在任务间共享复杂状态
- 批次间必须等待审查

#### verification-before-completion
**用途**：验证完成前检查

**触发条件**：准备声称工作完成时

**核心功能**：
- 运行验证命令（测试、构建、lint）
- 检查 TodoWrite 和 tasks.md 同步
- 确认所有任务完是断言

**铁律**：
```
没有新鲜的验证证据，不能声称完成
```

**关键规则**：
- 必须运行完整的验证命令
- 必须检查 tasks.md 所有任务标记为 [x]
- 必须确认 TodoWrite 和 tasks.md 同步
- 证据先于断言

### 完成阶段

#### finishing-a-development-branch
**用途**：完成开发分支

**触发条件**：实现完成，所有测试通过

**核心功能**：
- 验证测试通过
- 展示完成选项（merge、PR、cleanup）
- 执行用户选择
- 清理工作分支

**选项**：
1. 直接合并到主分支
2. 创建 Pull Request
3. 保留分支待后续处理

### 代码审查

#### requesting-code-review
**用途**：请求代码审查

**触发条件**：
- 完成主要功能
- 准备合并前
- 用户要求审查

**核心功能**：
- 总结变更内容
- 列出关键决策
- 标注需要特别关注的部分
- 提供测试证据

#### receiving-code-review
**用途**：接收代码审查反馈

**触发条件**：收到审查反馈

**核心功能**：
- 分析反馈的技术合理性
- 提出澄清问题
- 实施改进建议
- 验证改进效果

**关键规则**：
- 不盲目接受反馈
- 技术严谨优先于礼貌
- 验证改进效果

### 辅助工具

#### java-docs
**用途**：确保 Java 代码文档符合规范

**触发条件**：创建或修改 Java 类时

**核心功能**：
- 强制 Javadoc 注释
- 检查文档完整性
- 遵循 Java 文档最佳实践

## 适用场景

Superflows 适用于：
- 采用 OpenSpec 方法论的项目
- 需要规范化变更管理的团队
- 需要可追溯决策历史的项目
- 多人协作的复杂项目

不适用于：
- 一次性脚本或工具
- 个人实验项目
- 不需要文档的项目

## 与 OpenSpec 的关系

- **OpenSpec 规范**（`openspec/`）：定义"要做什么"
- **Superflows**（`.agents/skills/superflows/`）：定义"怎么做"

例如：
- `openspec/DOCUMENTATION.md` 定义文档组织规则
- `superflows/document-location` 强制执行这些规则

## 项目定制

虽然 Superflows 是通用的，但可以通过以下方式定制：

### 1. 项目特定的 skills
在 `superflows/` 下添加项目特定的 skills：
- 命名规范检查
- 特定框架的使用规范
- 项目特定的验证脚本

### 2. 修改现有 skills
调整 skills 中的：
- 文档路径
- 验证命令
- 工作流步骤

### 3. 配置 OpenSpec
在 `openspec/` 中定义：
- 项目约定（`project.md`）
- 功能规范（`specs/`）
- 文档组织（`DOCUMENTATION.md`）

## 迁移到其他项目

### 步骤 1：复制 Superflows
```bash
cp -r .agents/skills/superflows /path/to/new-project/.agents/skills/
```

### 步骤 2：初始化 OpenSpec
```bash
cd /path/to/new-project
openspec init
```

### 步骤 3：配置项目
1. 编辑 `openspec/project.md`
2. 创建初始 specs
3配置 `openspec/DOCUMENTATION.md`

### 步骤 4：调整 Skills（如需要）
- 修改文档路径引用
- 调整验证命令
- 添加项目特定的 skills

## 维护指南

### 添加新 Skill
1. 在 `superflows/` 下创建新目录
2. 创建 `SKILL.md`，包含：
   - YAML frontmatter（name、description）
   - 触发条件
   - 核心功能
   - 使用示例
3. 更新本 README.md 的工作流图

### 修改现有 Skill
1. 编辑 `.agents/skills/superflows/<skill-name>/SKILL.md`
2. 如果改变触发条件或用途，更新本 README.md
3. 如果改变工作流顺序，更新工作流图

### 删除 Skill
1. 删除对应目录
2. 更新本 README.md
3. 检查其他 skills 的引用并更新

## 参考文档

- OpenSpec 方法论：`openspec/AGENTS.md`
- 文档组织规范：`openspec/DOCUMENTATION.md`
- 项目约定：`openspec/project.md`
- 通用 Skills：`.agents/skills/README.md`

## 版本历史

- v1.0.0 (2026-03-06): 初始版本，整合完整的 OpenSpec 工作流
