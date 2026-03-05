# Change: 资源申请改为 Tabs + 多条模型/知识/智能体配置

## Why
现有资源申请为向导式步骤，难以支持一次申请中多智能体、多模型、多知识库的组合与共享场景。需要提供更灵活的申请方式，支持“新增/关联已有”混用，以及只申请模型或知识库。

## What Changes
- 将门户端资源申请从步骤向导改为 Tabs 结构：模型申请 / 知识库申请 / 智能体配置
- 支持模型申请列表与知识库申请列表的多条配置，每条可选“新申请/关联已有”
- 支持智能体多条配置，智能体必须绑定至少一个模型，知识库可选
- 提交校验要求：模型/知识库/智能体至少选择其一
- **BREAKING**：资源申请提交 payload 与前端类型结构调整为列表化结构

## Impact
- Affected specs: resource-application
- Affected code:
  - bj-gover-llm-portal/src/views/ResourceApply.vue
  - bj-gover-llm-portal/src/types/resource.ts
  - bj-gover-llm-portal/src/api/resource.ts (如需同步 mock 或提交结构)
