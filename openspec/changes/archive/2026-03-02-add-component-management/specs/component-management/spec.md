## ADDED Requirements

### Requirement: 组件管理页面导航
系统 SHALL 在管理端提供"组件管理"菜单入口（位于资源管理模块下），允许管理员和用户管理平台公共组件和私有组件。

#### Scenario: 管理员访问组件管理
- **WHEN** 管理员登录管理端并点击"资源管理 > 组件管理"菜单
- **THEN** 系统显示组件管理页面，包含筛选区、组件列表表格、操作按钮

#### Scenario: 路由权限控制
- **WHEN** 用户角色为 admin（管理员）
- **THEN** 系统允许访问组件管理页面，可管理所有公共组件和私有组件
- **WHEN** 用户角色为 portal（普通用户）
- **THEN** 系统允许访问组件管理页面，仅可管理自己的私有组件

### Requirement: 组件列表展示
系统 SHALL 使用表格布局展示组件列表，支持筛选、搜索和分页功能。

#### Scenario: 组件列表展示
- **WHEN** 用户进入组件管理页面
- **THEN** 系统显示组件列表表格，包含：组件名称、类型（MCP/PLUGIN）、作用域（公共/私有）、所有者、状态（草稿/已发布/已下架）、能力数量、创建时间、操作列

#### Scenario: 按名称搜索
- **WHEN** 用户在搜索框输入组件名称关键词并点击搜索
- **THEN** 系统筛选出名称包含关键词的组件

#### Scenario: 按类型筛选
- **WHEN** 用户选择"MCP"类型筛选器
- **THEN** 系统只显示类型为 MCP 的组件

#### Scenario: 按作用域筛选
- **WHEN** 用户选择"公共"作用域筛选器
- **THEN** 系统只显示作用域为公共的组件

#### Scenario: 按状态筛选
- **WHEN** 用户选择"已发布"状态筛选器
- **THEN** 系统只显示状态为已发布的组件

#### Scenario: 分页展示
- **WHEN** 组件列表超过每页显示数量
- **THEN** 系统使用分页展示，支持切换页码和修改每页数量

### Requirement: 组件创建和编辑
系统 SHALL 提供组件创建和编辑功能，支持填写基础信息和能力描述。

#### Scenario: 创建公共组件
- **WHEN** 管理员点击"新增组件"按钮
- **THEN** 系统跳转到组件详情页面，作用域默认为"公共"
- **WHEN** 管理员填写组件名称、类型、接入地址、协议类型、配置参数、描述、版本号，并添加至少一个能力描述后提交
- **THEN** 系统创建组件，状态为"草稿"，提示"创建成功"

#### Scenario: 创建私有组件
- **WHEN** 普通用户点击"新增组件"按钮
- **THEN** 系统跳转到组件详情页面，作用域默认为"私有"且不可修改
- **WHEN** 用户填写组件信息并提交
- **THEN** 系统创建组件，所有者为当前用户，状态为"草稿"

#### Scenario: 编辑组件
- **WHEN** 用户点击组件列表中的"编辑"按钮
- **THEN** 系统跳转到组件详情页面，显示该组件的现有信息
- **WHEN** 用户修改组件信息并提交
- **THEN** 系统更新组件信息，提示"更新成功"

#### Scenario: 添加能力描述
- **WHEN** 用户在组件详情页面点击"添加能力"按钮
- **THEN** 系统显示新的能力描述表单，包含功能名称、功能描述、参数列表、使用示例字段
- **WHEN** 用户填写能力信息并保存
- **THEN** 系统将该能力添加到组件的能力列表中

#### Scenario: 删除能力描述
- **WHEN** 用户在组件详情页面点击某个能力的"删除"按钮
- **THEN** 系统提示确认删除
- **WHEN** 用户确认删除
- **THEN** 系统从组件的能力列表中移除该能力

### Requirement: 组件上架和下架
系统 SHALL 提供公共组件的上架和下架功能，私有组件直接发布无需审批。

#### Scenario: 公共组件上架
- **WHEN** 管理员点击状态为"草稿"的公共组件的"上架"按钮
- **THEN** 系统提示确认上架
- **WHEN** 管理员确认上架
- **THEN** 系统将组件状态更新为"已发布"，该组件在资源广场和 Agent 申请页面中可见

#### Scenario: 公共组件下架
- **WHEN** 管理员点击状态为"已发布"的公共组件的"下架"按钮
- **THEN** 系统提示确认下架
- **WHEN** 管理员确认下架
- **THEN** 系统将组件状态更新为"已下架"，该组件在资源广场和 Agent 申请页面中不可见，但已绑定的 Agent 继续使用

#### Scenario: 私有组件发布
- **WHEN** 用户创建私有组件并提交
- **THEN** 系统自动将组件状态设置为"已发布"，无需审批流程

#### Scenario: 私有组件下架
- **WHEN** 用户点击自己的私有组件的"下架"按钮
- **THEN** 系统提示确认下架
- **WHEN** 用户确认下架
- **THEN** 系统将组件状态更新为"已下架"，该组件在 Agent 申请页面中不可见

### Requirement: 组件删除
系统 SHALL 提供组件删除功能，仅允许删除未被 Agent 绑定的组件。

#### Scenario: 删除未绑定的组件
- **WHEN** 用户点击未被任何 Agent 绑定的组件的"删除"按钮
- **THEN** 系统提示确认删除
- **WHEN** 用户确认删除
- **THEN** 系统删除该组件及其所有能力描述，提示"删除成功"

#### Scenario: 删除已绑定的组件
- **WHEN** 用户点击已被 Agent 绑定的组件的"删除"按钮
- **THEN** 系统提示"该组件已被 X 个智能体使用，无法删除。请先解除绑定或下架组件。"

### Requirement: Agent 申请时选择组件
系统 SHALL 在 Agent 申请页面提供组件选择功能，支持选择公共组件和用户私有组件。

#### Scenario: 选择组件
- **WHEN** 用户在 Agent 申请页面填写智能体信息
- **THEN** 系统显示"绑定组件"下拉多选框，分组显示"公共组件"和"我的私有组件"
- **WHEN** 用户选择一个或多个组件
- **THEN** 系统将选中的组件 ID 添加到 bindComponents 字段

#### Scenario: 组件信息展示
- **WHEN** 用户在组件下拉列表中查看组件
- **THEN** 系统显示组件名称、类型标签（MCP/PLUGIN）、能力数量

#### Scenario: 提交申请
- **WHEN** 用户填写完 Agent 信息并选择组件后提交申请
- **THEN** 系统创建 Agent 申请单，包含选中的组件信息，绑定类型为"INITIAL"

### Requirement: Agent 运行时动态调整组件
系统 SHALL 支持在 Agent 创建后动态添加或移除组件，无需重新审批。

#### Scenario: 添加组件
- **WHEN** 用户进入已创建的 Agent 详情页面并点击"添加组件"按钮
- **THEN** 系统显示组件选择对话框，列出可用的公共组件和用户私有组件
- **WHEN** 用户选择组件并确认
- **THEN** 系统将组件绑定到 Agent，绑定类型为"DYNAMIC"，记录操作日志，提示"添加成功"

#### Scenario: 移除组件
- **WHEN** 用户在 Agent 详情页面点击某个已绑定组件的"移除"按钮
- **THEN** 系统提示确认移除
- **WHEN** 用户确认移除
- **THEN** 系统解除该组件与 Agent 的绑定，绑定状态更新为"INIVE"，记录操作日志，提示"移除成功"

### Requirement: 组件权限控制
系统 SHALL 实现组件的权限控制，确保用户只能管理自己的私有组件。

#### Scenario: 管理员查看所有组件
- **WHEN** 管理员访问组件管理页面
- **THEN** 系统显示所有公共组件和所有用户的私有组件

#### Scenario: 普通用户查看组件
- **WHEN** 普通用户访问组件管理页面
- **THEN** 系统只显示公共组件和该用户自己的私有组件

#### Scenario: 编辑权限校验
- **WHEN** 用户尝试编辑不属于自己的私有组件
- **THEN** 系统提示"无权限操作该组件"

#### Scenario: 删除权限校验
- **WHEN** 用户尝试删除不属于自己的私有组件
- **THEN** 系统提示"无权限操作该组件"

### Requirement: 组件在资源广场展示
系统 SHALL 在资源广场的"组件"分类中展示已发布的公共组件和用户私有组件。

#### Scenario: 资源广场组件列表
- **WHEN** 用户在资源广场点击"组件"标签
- **THEN** 系统显示所有状态为"已发布"的公共组件和该用户自己的已发布私有组件

#### Scenario: 组件卡片展示
- **WHEN** 用户查看组件列表
- **THEN** 每个卡片显示：组件名称、组件描述、组件类型（MCP/PLUGIN）、版本号、"查看详情"和"申请使用"按钮

#### Scenario: 查看组件详情
- **WHEN** 用户点击组件卡片的"查看详情"按钮
- **THEN** 系统跳转到 `/portal/marketplace/component/:id` 路由，显示组件的完整信息（基础信息、能力描述）

#### Scenario: 申请使用组件
- **WHEN** 用户点击组件卡片的"申请使用"按钮
- **THEN** 系统跳转到 `/portal/resource-apply?type=component&id=xxx&name=xxx`，ResourceApply.vue 自动填充该组件信息

### Requirement: 数据库表结构
系统 SHALL 创建三张数据库表来存储组件相关数据。

#### Scenario: COMPONENT_组件表
- **WHEN** 系统初始化数据库
- **THEN** 系统创建 COMPONENT_组件 表，包含字段：component_id（主键）、name、type（MCP/PLUGIN）、scope（PUBLIC/PRIVATE）、owner_id、status（DRAFT/PUBLISHED/OFFLINE）、endpoint、protocol、configuration（JSON）、description、version、created_at、updated_at

#### Scenario: COMPONENT_CAPABILITY_组件能力表
- **WHEN** 系统初始化数据库
- **THEN** 系统创建 COMPONENT_CAPABILITY_组件能力 表，包含字段：capability_id（主键）、component_id（外键）、function_name、description、parameters（JSON）、examples（JSON）、sort_order

#### Scenario: AGENT_COMPONENT_BINDING_智能体组件绑定表
- **WHEN** 系统初始化数据库
- **THEN** 系统创建 AGENT_COMPONENT_BINDING_智能体组件绑定 表，包含字段：binding_id（主键）、agent_id（外键）、component_id（外键）、binding_type（INITIAL/DYNAMIC）、binding_time、status（ACTIVE/INACTIVE）
