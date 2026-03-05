# Change: P0 资源申请与资源广场前端重构（Demo）

## Why
- 现有 Demo 无法覆盖最新资源申请字段与审批展示要求，影响演示完整度。
- 需要在门户侧明确资源广场定位，并补齐关键操作入口。

## What Changes
- 门户端资源申请表单新增/调整字段，并同步提交结构（仅前端 Demo，使用静态数据）。
- 管理端审批详情补齐申请字段展示，增加审批意见必填提示（界面提示）。
- 门户端模型/智能体/知识库页面按资源广场定位更新文案与入口。

## Impact
- Affected specs: resource-application-portal, approval-detail-admin, resource-square-portal
- Affected code:
  - bj-gover-llm-portal/src/views/ResourceApply.vue
  - bj-gover-llm-portal/src/types/resource.ts
  - bj-gover-llm-portal/src/views/ModelManagement.vue
  - bj-gover-llm-portal/src/views/AgentManagement.vue
  - bj-gover-llm-portal/src/views/KnowledgeManagement.vue
  - bj-gover-llm-admin/src/views/approval/detail.vue
  - bj-gover-llm-admin/src/types/approval.ts
