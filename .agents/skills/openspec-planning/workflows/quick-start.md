# OpenSpec 快速开始

## 最小化流程

这是创建 OpenSpec change 的最简流程，适合快速上手。

## 步骤 1：检查现有工作

```bash
openspec list                    # 查看活跃的 changes
openspec list --specs            # 查看现有 capabilities
```

确保你的 change-id 是唯一的。

## 步骤 2：选择 change-id

格式：kebab-case，动词开头

示例：
- `add-dark-mode`
- `update-auth-flow`
- `fix-login-redirect`

## 步骤 3：创建目录结构

```bash
mkdir -p openspec/changes/<change-id>/specs/<capability>
```

## 步骤 4：编写 proposal.md

创建 `openspec/changes/<change-id>/proposal.md`：

```markdown
# Change: [简短描述]

## Why
[1-2 句话说明问题/机会]

## What Changes
- [变更列表]

## Impact
- Affected specs: [受影响的 capabilities]
- Affected code: [关键文件/系统]
```

参考：`reference/proposal-template.md`

## 步骤 5：编写 spec 增量

创建 `openspec/changes/<change-id>/specs/<capability>/spec.md`：

```markdown
## ADDED Requirements

### Requirement: 功能名称
系统应当...

#### Scenario: 场景名称
- **WHEN** 触发条件
- **THEN** 预期结果
```

参考：`reference/spec-template.md`

## 步骤 6：编写 tasks.md

创建 `openspec/changes/<change-id>/tasks.md`：

```markdown
## 1. 实现阶段
- [ ] 1.1 具体任务

## 2. 测试阶段
- [ ] 2.1 测试任务
```

参考：`reference/tasks-template.md`

## 步骤 7：验证

```bash
openspec validate <change-id> --strict --no-interactive
```

修复所有问题后再分享给用户。

## 步骤 8：等待批准

展示提案给用户，等待明确批准。

**在用户批准前，不要开始实现。**

## 步骤 9：实现

用户批准后，按照 tasks.md 开始实现。

## 步骤 10：归档

实现完成、测试通过、代码合并后：

```bash
openspec archive <change-id> --yes
```

## 可选：design.md

仅在以下情况创建 `openspec/changes/<change-id>/design.md`：
- 架构决策需要记录
- 技术方案有多个选项需要对比
- 复杂的技术实现需要说明

## 下一步

需要更详细的指导，参考：
- `reference/proposal-template.md` - proposal 模板和示例
- `reference/spec-template.md` - spec 增量格式和示例
- `reference/tasks-template.md` - tasks 格式和示例
- `reference/commands.md` - 完整命令参考
