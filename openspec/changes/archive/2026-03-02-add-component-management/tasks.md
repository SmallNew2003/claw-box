## 1. 数据库设计
- [ ] 1.1 创建 COMPONENT_组件 表结构（Demo 阶段使用 Mock 数据，暂不实现）
- [ ] 1.2 创建 COMPONENT_CAPABILITY_组件能力 表结构（Demo 阶段使用 Mock 数据，暂不实现）
- [ ] 1.3 创建 AGENT_COMPONENT_BINDING_智能体组件绑定 表结构（Demo 阶段使用 Mock 数据，暂不实现）

## 2. 前端类型定义
- [x] 2.1 在 resource.ts 中新增 ComponentItem 接口
- [x] 2.2 在 resource.ts 中新增 ComponentCapability 接口
- [x] 2.3 修改 AgentRequestItem 接口，将 bindPlugins 和 bindMcps 合并为 bindComponents
- [x] 2.4 新增组件类型、协议类型等常量选项

## 3. 组件管理页面
- [x] 3.1 创建 component.vue 页面（参考 model.vue 结构）
- [x] 3.2 实现组件列表展示（表格、筛选、分页）
- [x] 3.3 实现组件的查看、编辑、删除操作
- [x] 3.4 实现公共组件的上架/下架功能
- [x] 3.5 准备 Mock 数据

## 4. 组件详情页面
- [x] 4.1 创建 ComponentDetail.vue 页面
- [x] 4.2 实现基础信息表单（名称、类型、作用域、接入地址等）
- [x] 4.3 实现能力描述表单（支持添加多个能力）
- [x] 4.4 实现表单验证和提交逻辑

## 5. Agent 申请页面调整
- [x] 5.1 修改 ResourceApply.vue 中的组件选择部分
- [x] 5.2 将 bindPlugins 和 bindMcps 合并为 bindComponents 下拉多选
- [x] 5.3 实现组件分组显示（公共组件 / 我的私有组件）
- [x] 5.4 更新相关的 Mock 数据

## 6. 路由配置
- [x] 6.1 在管理端路由中新增 /resource/component 路由
- [x] 6.2 在管理端路由中新增 /resource/component/detail/:id 路由
- [x] 6.3 更新菜单配置，添加组件管理菜单项

## 7. 文档和验证
- [x] 7.1 运行 openspec validate 验证规范
- [x] 7.2 测试组件管理页面的基本功能
- [x] 7.3 测试 Agent 申请页面的组件选择功能
