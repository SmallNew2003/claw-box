## 1. 类型定义
- [x] 1.1 在 `src/types/` 创建 `marketplace.ts`，定义资源广场相关类型
- [x] 1.2 定义 BaseResource 通用字段接口
- [x] 1.3 定义 ModelResource、KnowledgeResource、AgentResource、ComponentResource 接口
- [x] 1.4 定义搜索和筛选参数类型（GetResourceListParams）
- [x] 1.5 定义分页响应类型

## 2. API 接口
- [x] 2.1 在 `src/api/` 创建 `marketplace.ts`，实现资源广场 API
- [x] 2.2 实现 getResourceList 接口（支持搜索、筛选、分页、排序）
- [x] 2.3 实现 getResourceDetail 接口（根据 type 和 id 获取详情）
- [x] 2.4 准备模型服务 Mock 数据（20 条，包含不同类型和提供商）
- [x] 2.5 准备知识库 Mock 数据（18 条，包含不同领域）
- [x] 2.6 准备智能体 Mock 数据（17 条，包含不同应用场景）
- [x] 2.7 准备组件 Mock 数据（15 条，包含插件和 MCP 类型）

## 3. 路由配置
- [x] 3.1 在 `src/router/routes.ts` 添加资源广场主路由（/portal/marketplace）
- [x] 3.2 添加 4 个详情页子路由（model/:id, knowledge/:id, agent/:id, component/:id）
- [x] 3.3 配置路由权限（仅 portal 角色可访问）
- [x] 3.4 配置路由元信息（标题、图标 Shop、hideInMenu 等）

## 4. 主页面组件
- [x] 4.1 创建 `src/views/portal/ResourceMarketplace.vue` 主页面
- [x] 4.2 实现页面布局（面包屑 + 标题 + Tab + 搜索筛选 + 卡片网格 + 分页）
- [x] 4.3 实现 Tab 切换（模型服务、知识库、智能体、组件），默认显示模型服务
- [x] 4.4 实现搜索框（支持 Enter 键搜索）
- [x] 4.5 实现筛选器（状态、类型/领域/场景、提供商、排序）
- [x] 4.6 实现重置按钮清空筛选条件
- [x] 4.7 实现资源卡片组件（封面图、名称、描述、标签、状态、使用次数、两个按钮）
- [x] 4.8 实现响应式网格布局（大屏 4 列、中屏 3 列、小屏 2 列）
- [x] 4.9 实现分页器（每页 12 条）
- [x] 4.10 实现空状态提示

## 5. 详情页组件
- [x] 5.1 创建 `src/views/portal/MarketplaceModelDetail.vue` 模型详情页
- [x] 5.2 创建 `src/views/portal/MarketplaceKnowledgeDetail.vue` 知识库详情页
- [x] 5.3 创建 `src/views/portal/MarketplaceAgentDetail.vue` 智能体详情页
- [x] 5.4 创建 `src/views/portal/MarketplaceComponentDetail.vue` 组件详情页
- [x] 5.5 实现详情页布局（面包屑 + 返回按钮 + 占位图片）
- [x] 5.6 准备占位图片资源（使用 placeholder 图片服务）

## 6. 交互功能
- [x] 6.1 实现搜索功能（输入关键词 + Enter 或点击搜索按钮）
- [x] 6.2 实现筛选功能（下拉选择器，自动刷新列表）
- [x] 6.3 实现排序功能（最新上架、最热门、名称 A-Z）
- [x] 6.4 实现重置按钮清空所有筛选条件
- [x] 6.5 实现分页切换和每页数量调整
- [x] 6.6 实现卡片"查看详情"按钮跳转到详情页
- [x] 6.7 实现卡片"申请使用"按钮跳转到 ResourceApply.vue 并传递参数（type、id、name）
- [x] 6.8 实现详情页返回按钮（使用 router.back()）

## 7. 样式和响应式
- [x] 7.1 实现卡片样式（封面图、标题、描述省略、标签、状态徽章）
- [x] 7.2 实现响应式网格（>1440px: 4列, 1024-1440px: 3列, <1024px: 2列）
- [x] 7.3 实现加载状态（骨架屏）
- [x] 7.4 实现空状态样式（el-empty 组件）
- [x] 7.5 实现卡片悬停效果（阴影、位移动画）
- [x] 7.6 实现详情页占位图片样式

## 8. 测试和优化
- [x] 8.1 测试四个分类的 Tab 切换和数据加载
- [x] 8.2 测试搜索功能（关键词匹配、无结果提示）
- [x] 8.3 测试筛选功能（状态、类型、提供商、排序）
- [x] 8.4 测试重置按钮清空筛选条件
- [x] 8.5 测试分页切换和每页数量调整
- [x] 8.6 测试"查看详情"跳转到详情页
- [x] 8.7 测试"申请使用"跳转到 ResourceApply.vue 并自动填充
- [x] 8.8 测试详情页返回保持主页面状态
- [x] 8.9 测试响应式布局（大中小屏幕）
- [x] 8.10 测试空状态和加载状态
- [x] 8.11 性能优化（已实现基础优化，虚拟滚动和图片懒加载可在后续迭代中添加）
