# Proposal Template

## 文件位置
`openspec/changes/<change-id>/proposal.md`

## 基本结构

```markdown
# Change: [简短描述]

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

1. **Why 部分**：说明业务价值或技术必要性
2. **What Changes**：列出具体变更，标记破坏性变更
3. **Impact**：明确影响范围，便于评估风险
4. **简洁性**：每部分 1-3 句话，不要写成详细设计文档
