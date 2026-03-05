# Change: 资源申请移除关联已有 + 智能体绑定抽屉选择

## Why
模型/知识库申请不再需要关联已有资源，避免重复概念。智能体绑定仍需支持选择已有资源，但下拉选择不直观，需要改为右侧抽屉卡片多选，并在卡片头部提供编辑按钮。

## What Changes
- 移除模型/知识库申请条目中的“来源类型/关联已有”能力，仅支持新申请
- 智能体绑定模型/知识库改为右侧抽屉卡片多选，不使用下拉
- 绑定选择列表同时包含“本次申请资源”与“已有资源”，并用标签区分
- 申请卡片头部新增“编辑”按钮（保留删除）

## Impact
- Affected specs: resource-application
- Affected code:
  - bj-gover-llm-portal/src/types/resource.ts
  - bj-gover-llm-portal/src/views/ResourceApply.vue
