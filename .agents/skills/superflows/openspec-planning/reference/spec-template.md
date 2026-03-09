# Spec Template

## 文件位置

**Delta 文件**：`openspec/changes/YYYY-MM-DD-<description>/specs/<capability>/spec.md`
**完整规范**：`openspec/specs/<capability>/spec.md`

## 完整规范格式（归档后）

```markdown
# [capability] Specification

## Purpose
[简短描述此 capability 的目的]

## Change History
- [2026-03-06-add-dark-mode](../../changes/archive/2026-03-06-add-dark-mode/) - 添加暗黑模式支持
- [2026-02-15-refactor-theme](../../changes/archive/2026-02-15-refactor-theme/) - 重构主题系统

## Requirements
### Requirement: 功能名称
系统应当提供...

#### Scenario: 场景名称
- **WHEN** 触发条件
- **THEN** 预期结果
```

## 增量格式（在 changes/ 中）

使用 ADDED/MODIFIED/REMOVED 标记变更类型。

### ADDED Requirements

用于新增的需求。

```markdown
## ADDED Requirements

### Requirement: 新功能名称
系统应当提供...

#### Scenario: 成功场景
- **WHEN** 用户执行操作
- **THEN** 预期结果

#### Scenario: 错误场景
- **GIVEN** 前置条件
- **WHEN** 触发错误条件
- **THEN** 系统响应
```

### MODIFIED Requirements

用于修改现有需求。需要提供完整的修改后需求。

```markdown
## MODIFIED Requirements

### Requirement: 现有功能名称
[完整的修改后需求描述]

#### Scenario: 更新的场景
- **GIVEN** 前置条件
- **WHEN** 触发条件
- **THEN** 预期结果
```

### REMOVED Requirements

用于移除的需求。需要说明原因和迁移路径。

```markdown
## REMOVED Requirements

### Requirement: 旧功能名称
**Reason**: [为什么移除]
**Migration**: [如何迁移到新方案]
```

## 示例

```markdown
## ADDED Requirements

### Requirement: 资源申请审批
系统应当在用户提交资源申请后，启动审批流程，只有审批通过后才分配资源。

#### Scenario: 提交申请触发审批
- **WHEN** 用户提交资源申请
- **THEN** 系统创建审批工单并通知审批人

#### Scenario: 审批通过后分配资源
- **GIVEN** 审批工单已创建
- **WHEN** 审批人批准申请
- **THEN** 系统自动分配资源并通知申请人

#### Scenario: 审批拒绝
- **GIVEN** 审批工单已创建
- **WHEN** 审批人拒绝申请
- **THEN** 系统标记申请为已拒绝并通知申请人原因

## MODIFIED Requirements

### Requirement: 资源申请状态
系统应当跟踪资源申请的状态，包括：待审批、审批中、已批准、已拒绝、已分配。

#### Scenario: 查询申请状态
- **WHEN** 用户查询申请状态
- **THEN** 系统返回当前状态和审批进度
```

## 关键规则

1. **每个 Requirement 至少 1 个 Scenario**
2. **使用 GIVEN/WHEN/THEN 格式**：清晰描述前置条件、触发条件、预期结果
3. **多个 capabilities**：如果影响多个 capabilities，为每个创建独立的 spec.md
4. **完整性**：MODIFIED 需要提供完整的修改后需求，不是增量描述
5. **可测试性**：Scenario 应该可以直接转化为测试用例
