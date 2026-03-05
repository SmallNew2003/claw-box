# Change: 新增组件管理功能

## Why

当前系统已支持算力、模型、知识库、智能体四类资源的管理，但缺少对 Agent 扩展能力（MCP 和插件）的统一管理。为了增强智能体的可扩展性，需要新增组件管理功能，支持平台公共组件和用户私有组件的接入与管理。

## What Changes

- 新增组件管理页面（管理端 /resource/component），支持组件的创建、编辑、上架/下架
- 新增组件详情/编辑页面，支持填写组件基础信息和能力描述
- 修改 Agent 申请页面，将 bindPlugins 和 bindMcps 合并为 bindComponents
- 新增数据库表：COMPONENT_组件、COMPONENT_CAPABILITY_组件能力、AGENT_COMPONENT_BINDING_智能体组件绑定
- 新增前端类型定义：ComponentItem、ComponentCapability
- 支持平台公共组件（需审批）和用户私有组件（无需审批）两种模式
- 支持 Agent 申请时预选组件 + 运行时动态调整组件

## Impact

- 受影响的规范：新增 component-management 规范
- 受影响的代码：
  - 前端类型定义：`bj-gover-llm-admin/src/types/resource.ts`
  - 前端页面：新增 `bj-gover-llm-admin/src/views/resource/component.vue`
  - 前端页面：新增 `bj-gover-llm-admin/src/views/resource/ComponentDetail.vue`
  - 前端页面：修改 `bj-gover-llm-admin/src/views/resource/agent.vue`
  - 路由配置：`bj-gover-llm-admin/src/router/index.ts`
  - 数据库：新增 3 张表
- Demo 阶段使用 Mock 数据，暂不对接后端 API
