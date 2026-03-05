# resource-marketplace Specification

## Purpose
TBD - created by archiving change add-resource-marketplace. Update Purpose after archive.
## Requirements
### Requirement: 资源广场页面导航
系统 SHALL 在 A 端（业务服务门户）提供"资源广场"一级菜单入口（位于"工作台"和"我的申请"之间），允许普通用户浏览平台上所有可用的资源。

#### Scenario: 用户访问资源广场
- **WHEN** 用户登录 A 端并点击"资源广场"菜单
- **THEN** 系统显示资源广场页面，包含模型服务、知识库、智能体、组件四个分类标签，默认显示"模型服务"分类

#### Scenario: 路由权限控制
- **WHEN** 用户角色为 portal（普通用户）
- **THEN** 系统允许访问资源广场页面
- **WHEN** 用户角色为 admin（管理员）
- **THEN** 系统不显示资源广场菜单

#### Scenario: 页面布局结构
- **WHEN** 用户进入资源广场页面
- **THEN** 系统显示面包屑导航、页面标题、Tab 切换、搜索筛选栏、卡片网格、分页器

### Requirement: 资源分类展示
系统 SHALL 提供四种资源分类（模型服务、知识库、智能体、组件），每个分类使用独立的标签页展示。

#### Scenario: 切换资源分类
- **WHEN** 用户点击"模型服务"标签
- **THEN** 系统显示所有可用的模型服务列表
- **WHEN** 用户点击"知识库"标签
- **THEN** 系统显示所有可用的知识库列表
- **WHEN** 用户点击"智能体"标签
- **THEN** 系统显示所有可用的智能体列表
- **WHEN** 用户点击"组件"标签
- **THEN** 系统显示所有可用的组件列表

### Requirement: 资源卡片展示
系统 SHALL 使用卡片式布局展示资源列表，每个资源卡片包含关键信息。

#### Scenario: 模型服务卡片展示
- **WHEN** 用户查看模型服务列表
- **THEN** 每个卡片显示：模型名称、模型描述、模型类型标签、提供商、状态标识（可用/维护中）、"查看详情"按钮

#### Scenario: 知识库卡片展示
- **WHEN** 用户查看知识库列表
- **THEN** 每个卡片显示：知识库名称、知识库描述、文档数量、向量化状态、所属领域标签、"查看详情"按钮

#### Scenario: 智能体卡片展示
- **WHEN** 用户查看智能体列表
- **THEN** 每个卡片显示：智能体名称、智能体描述、应用场景标签、使用次数、安全合规标识、"查看详情"按钮

#### Scenario: 组件卡片展示
- **WHEN** 用户查看组件列表
- **THEN** 每个卡片显示：组件名称、组件描述、组件类型（插件/MCP）、版本号、"查看详情"和"申请使用"按钮

### Requirement: 搜索和筛选功能
系统 SHALL 提供搜索框和筛选器，允许用户快速找到目标资源。支持通用筛选（状态、排序）和分类特定筛选（模型类型/提供商、知识库领域、智能体场景、组件类型）。

#### Scenario: 关键词搜索
- **WHEN** 用户在搜索框输入关键词"文本生成"并点击搜索或按 Enter
- **THEN** 系统在当前分类中筛选出名称或描述包含"文本生成"的资源，并显示"找到 X 个结果"

#### Scenario: 按状态筛选
- **WHEN** 用户选择"可用"状态筛选器
- **THEN** 系统只显示状态为"可用"的资源

#### Scenario: 按类型筛选
- **WHEN** 用户在模型服务分类中选择"大语言模型"类型筛选器
- **THEN** 系统只显示类型为"大语言模型"的模型服务

#### Scenario: 按提供商筛选
- **WHEN** 用户在模型服务分类中选择"百度千帆"提供商筛选器
- **THEN** 系统只显示提供商为"百度千帆"的模型服务

#### Scenario: 排序功能
- **WHEN** 用户选择"最热门"排序方式
- **THEN** 系统按使用次数降序排列资源列表

#### Scenario: 清空筛选条件
- **WHEN** 用户点击"重置"按钮
- **THEN** 系统清空所有搜索和筛选条件，显示完整的资源列表

### Requirement: 分页功能
系统 SHALL 提供分页功能，支持大量资源的浏览。

#### Scenario: 分页展示
- **WHEN** 资源列表超过 12 条
- **THEN** 系统使用分页展示，每页显示 12 个资源卡片

#### Scenario: 切换页码
- **WHEN** 用户点击页码"2"
- **THEN** 系统显示第 2 页的资源列表

#### Scenario: 修改每页数量
- **WHEN** 用户选择每页显示 24 条
- **THEN** 系统重新加载并显示 24 个资源卡片

### Requirement: 资源详情查看
系统 SHALL 提供资源详情页面（独立路由），展示资源的完整信息。演示项目中使用占位图片模拟第三方页面内容。

#### Scenario: 查看模型详情
- **WHEN** 用户点击模型服务卡片的"查看详情"按钮
- **THEN** 系统跳转到 `/portal/marketplace/model/:id` 路由，显示面包屑导航、返回按钮和占位图片

#### Scenario: 查看知识库详情
- **WHEN** 用户点击知识库卡片的"查看详情"按钮
- **THEN** 系统跳转到 `/portal/marketplace/knowledge/:id` 路由，显示面包屑导航、返回按钮和占位图片

#### Scenario: 查看智能体详情
- **WHEN** 用户点击智能体卡片的"查看详情"按钮
- **THEN** 系统跳转到 `/portal/marketplace/agent/:id` 路由，显示面包屑导航、返回按钮和占位图片

#### Scenario: 查看组件详情
- **WHEN** 用户点击组件卡片的"查看详情"按钮
- **THEN** 系统跳转到 `/portal/marketplace/component/:id` 路由，显示面包屑导航、返回按钮和占位图片

#### Scenario: 详情页返回
- **WHEN** 用户在详情页点击返回按钮或面包屑导航
- **THEN** 系统返回到资源广场主页面，保持之前的 Tab 选择和筛选条件

### Requirement: 申请使用入口
系统 SHALL 在资源卡片上提供"申请使用"按钮，允许用户直接发起资源申请流程。

#### Scenario: 从卡片申请模型
- **WHEN** 用户在模型服务卡片上点击"申请使用"按钮
- **THEN** 系统跳转到 `/portal/resource-apply?type=model&id=xxx&name=xxx`，ResourceApply.vue 自动填充该模型信息

#### Scenario: 从卡片申请知识库
- **WHEN** 用户在知识库卡片上点击"申请使用"按钮
- **THEN** 系统跳转到 `/portal/resource-apply?type=knowledge&id=xxx&name=xxx`，ResourceApply.vue 自动填充该知识库信息

#### Scenario: 从卡片申请智能体
- **WHEN** 用户在智能体卡片上点击"申请使用"按钮
- **THEN** 系统跳转到 `/portal/resource-apply?type=agent&id=xxx&name=xxx`，ResourceApply.vue 自动填充该智能体信息

#### Scenario: 从卡片申请组件
- **WHEN** 用户在组件卡片上点击"申请使用"按钮
- **THEN** 系统跳转到 `/portal/resource-apply?type=component&id=xxx&name=xxx`，ResourceApply.vue 自动填充该组件信息

### Requirement: 空状态处理
系统 SHALL 在没有资源或搜索无结果时提供友好的空状态提示。

#### Scenario: 分类无资源
- **WHEN** 某个分类下没有任何资源
- **THEN** 系统显示空状态图标和文案"暂无可用资源"

#### Scenario: 搜索无结果
- **WHEN** 用户搜索的关键词没有匹配的资源
- **THEN** 系统显示空状态图标和文案"未找到相关资源，请尝试其他关键词"

### Requirement: 响应式布局
系统 SHALL 提供响应式布局，适配不同屏幕尺寸。

#### Scenario: 大屏幕展示
- **WHEN** 屏幕宽度大于 1440px
- **THEN** 系统每行显示 4 个资源卡片

#### Scenario: 中等屏幕展示
- **WHEN** 屏幕宽度在 1024px 到 1440px 之间
- **THEN** 系统每行显示 3 个资源卡片

#### Scenario: 小屏幕展示
- **WHEN** 屏幕宽度小于 1024px
- **THEN** 系统每行显示 2 个资源卡片

