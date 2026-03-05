# Change: 资源申请智能体绑定改为弹窗并扩展插件/MCP

## Why
当前智能体绑定使用抽屉选择模型/知识库，叠加编辑抽屉时易造成混淆；同时智能体需要支持插件与 MCP 绑定。

## What Changes
- 将智能体绑定选择从右侧抽屉改为弹窗，并使用 Tabs 组织模型/知识库/插件/MCP
- 扩展智能体绑定能力，支持插件与 MCP 资源多选绑定并显示来源标签
- 绑定弹窗卡片展示关键指标标签（模型/知识库/插件/MCP）
- 绑定弹窗支持搜索、宫格布局与前端分页
- 绑定弹窗卡片升级为媒体卡片（封面+描述+标签）
- 同步更新绑定选择状态、展示与空态文案

## Impact
- Affected specs: resource-application
- Affected code: bj-gover-llm-admin/src/views/portal/ResourceApply.vue, bj-gover-llm-admin/src/types/resource.ts
