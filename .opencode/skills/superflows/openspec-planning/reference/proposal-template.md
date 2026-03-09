# Proposal Template

## 文件位置
`openspec/changes/YYYY-MM-DD-<description>/proposal.md`

## 基本结构

```markdown
# Change: [简短描述]

## Metadata
- Created: YYYY-MM-DDTHH:MM:SSZ
- Affected Specs:
  - [capability-name] (modified: YYYY-MM-DDTHH:MM:SSZ)
  - [capability-name] (modified: YYYY-MM-DDTHH:MM:SSZ)

## Why
[1-2 句话说明问题/机会]

## What Changes
- [变更列表]
- [标记破坏性变更为 **BREAKING**]

## Impact
- Affected specs: [受影响的 capabilities]
- Affected code: [关键文件/系统]
```

## 示例

```markdown
# Change: 添加用户资源申请审批流程

## Metadata
- Created: 2026-03-06T10:30:00Z
- Affected Specs:
  - resource-application (modified: 2026-03-05T15:20:00Z)
  - user-access (modified: 2026-03-01T09:15:00Z)

## Why
当前用户申请资源后直接分配，缺少审批环节，无法控制资源使用。

## What Changes
- 添加审批工作流引擎集成
- 新增审批记录表和 API
- 修改资源申请流程，插入审批步骤
- **BREAKING**: 资源申请 API 响应格式变更

## Impact
- Affected specs: resource-application, user-access
- Affected code: backend/services/resource-service.ts, backend/models/approval.ts
```

## 关键要点

1. **Metadata 部分**：记录创建时间和受影响 spec 的最后修改时间，用于归档时的冲突检测
2. **Why 部分**：说明业务价值或技术必要性
3. **What Changes**：列出具体变更，标记破坏性变更
4. **Impact**：明确影响范围，便于评估风险
5. **简洁性**：每部分 1-3 句话，不要写成详细设计文档
