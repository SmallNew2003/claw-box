# Change: 新增资源广场页面

## Why
当前普通用户（A端）只能在"我的资源"中查看已获批的资源，缺少一个公共的资源广场来浏览和发现平台上可用的模型、知识库、智能体和组件（插件/MCP）。用户需要一个统一的入口来：
- 浏览平台提供的所有可用资源
- 通过搜索、筛选、排序快速找到目标资源
- 查看资源详情（跳转到独立详情页）
- 从资源卡片直接发起申请流程

## What Changes
- 在 A 端（业务服务门户）新增"资源广场"一级菜单（位于"工作台"和"我的申请"之间）
- 创建资源广场主页面，使用 Tab 切换四个资源分类：模型服务、知识库、智能体、组件
- 实现搜索、筛选（状态/类型/提供商）、排序（最新/最热门/名称）功能
- 使用响应式卡片网格布局（大屏 4 列、中屏 3 列、小屏 2 列）
- 资源卡片包含：封面图、名称、描述、标签、状态、使用次数、两个操作按钮（查看详情、申请使用）
- 创建 4 个资源详情页（模型/知识库/智能体/组件），使用占位图片模拟第三方页面
- 从卡片"申请使用"按钮跳转到现有 ResourceApply.vue 页面，自动填充资源信息
- 准备 Mock 数据（每种资源类型 15-20 条）
- 新增路由配置（仅 portal 角色可访问）、API 接口和类型定义

## Impact
- Affected specs: resource-marketplace（新增）
- Affected code:
  - `bj-gover-llm-admin/src/router/routes.ts` - 新增路由配置
  - `bj-gover-llm-admin/src/views/portal/ResourceMarketplace.vue` - 主页面
  - `bj-gover-llm-admin/src/views/portal/Marketplace*Detail.vue` - 4 个详情页
  - `bj-gover-llm-admin/src/api/marketplace.ts` - API 接口
  - `bj-gover-llm-admin/src/types/marketplace.ts` - 类型定义
- Design document: `docs/plans/2026-02-06-resource-marketplace-design.md`
