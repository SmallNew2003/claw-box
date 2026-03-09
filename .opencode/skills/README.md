# 技能使用指南

本文档说明 OpenCode 技能的分类、依赖关系和使用规则。

## 技能分类

### 1. 调度技能（Dispatcher Skills）
决定调用哪个技能，是所有工作流的入口点。

**示例**：
- `using-superpowers` - 总调度器，每次收到用户消息时必须调用

### 2. Process 技能（Process Skills）
决定如何处理任务，定义工作流程。

**示例**：
- `brainstorming` - 探索需求，澄清目标
- `openspec-planning` - 创建正式提案和规范
- `systematic-debugging` - 系统化调试流程

### 3. Implementation 技能（Implementation Skills）
指导具体执行，提供实施步骤。

**示例**：
- `writing-plans` - 编写实施计划
- `executing-plans` - 执行实施计划
- `test-driven-development` - TDD 开发流程

### 4. Verification 技能（Verification Skills）
验证工作完成度和质量。

**示例**：
- `verification-before-completion` - 完成前验证
- `requesting-code-review` - 请求代码审查

### 5. 领域技能（Domain Skills）
特定领域的专业指导。

**示例**：
- `java-docs` - Java 文档规范
- `mermaid-diagrams` - Mermaid 图表创建

### 6. 工具技能（Tool Skills）
特定工具的使用指南。

**示例**：
- `using-git-worktrees` - Git worktree 使用
- `writing-skills` - 技能编写指南

## 技能依赖关系

### 核心调用链

```
using-superpowers（总调度）
    ├─ brainstorming（需求不明确时）
    │   └─ writing-plans
    │       └─ openspec-planning
    ├─ openspec-planning（需求明确时）
    │   └─ executing-plans
    │       └─ verification-before-completion
    │           └─ 归档
    └─ systematic-debugging（Bug 修复）
        └─ 实现修复
            └─ verification-before-completion
```

### 技能调用模式

**终止技能**（不再调用其他技能）：
- `systematic-debugging` - 调试后直接实现
- `requesting-code-review` - 审查后等待反馈

**中转技能**（会自动调用下一个技能）：
- `brainstorming` → `writing-plans`
- `writing-plans` → `openspec-planning`
- `executing-plans` → `verification-before-completion`

## 技能调用规则

### 规则 1：强制调用 using-superpowers
收到用户消息后，必须立即调用 `using-superpowers` skill，即使只有 1% 可能性也要检查。

### 规则 2：Process 技能优先
当多个技能可能适用时，优先调用 Process 技能（如 brainstorming、debugging），再调用 Implementation 技能。

### 规则 3：遵循技能指引
调用技能后，必须完整遵循其流程，不能跳过步骤或自行简化。

### 规则 4：技能链完整性
如果技能指示调用下一个技能，必须执行，不能中断调用链。

## 常见场景表

| 用户说... | 需求明确？ | 调用顺序 |
|----------|-----------|---------|
| "实现 XX 功能" | 否 | `brainstorming` → `openspec-planning` → 实现 |
| "实现 XX 功能" | 是 | `openspec-planning` → 实现 |
| "添加 XX 模块" | 否 | `brainstorming` → `openspec-planning` → 实现 |
| "添加 XX 模块" | 是 | `openspec-planning` → 实现 |
| "修复 XX bug" | - | `systematic-debugging` → 实现 |
| "这个项目是做什么的" | - | 读取 `openspec/project.md` |
| "怎么写 Java 代码" | - | 读取 `openspec/project.md` § Java 规范 |
| "等我确认" | - | `openspec-planning` |
| "创建提案" | - | `openspec-planning` |
| "归档这个 change" | - | `verification-before-completion` → 归档 |

## 判断需求是否明确

**需求明确**（直接 openspec-planning）：
- 用户提供了详细的功能描述
- 用户明确说明了技术方案
- 用户提供了设计文档或原型

**需求不明确**（先 brainstorming）：
- 用户只说了一个模糊的目标（"优化性能"、"改进用户体验"）
- 没有说明具体实现方式
- 可能有多种实现方案需要对比
- 你不确定用户的真实意图

**经验法则**：如果你需要问 2 个以上的澄清问题，先调用 `brainstorming`。

## 详细文档引用

- **完整技能列表**：查看 `.opencode/skills/` 目录
- **OpenSpec 工作流**：`openspec/AGENTS.md`
- **Agent 规则**：`AGENTS.md`
- **项目约定**：`openspec/project.md`
- **文档规范**：`openspec/DOCUMENTATION.md`

## 技能开发

如需创建或修改技能，请参考：
- `writing-skills` skill - 技能编写指南
- `.opencode/skills/` - 现有技能示例
