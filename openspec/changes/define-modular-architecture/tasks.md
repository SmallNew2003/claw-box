## 1. 架构骨架
- [x] 1.1 创建 modules 与 shared-kernel 目录结构
- [x] 1.2 定义模块分层与命名规范
- [x] 1.3 输出模块骨架模板（空实现）

## 2. shared-kernel
- [x] 2.1 定义租户上下文与权限上下文契约
- [x] 2.2 定义审计事件、统一异常与基础值对象

## 3. ports/adapters
- [x] 3.1 定义 AuthPort/TenantPort/AuditPort/WorkflowPort/MessagingPort
- [x] 3.2 定义宿主适配层接口与装配规范

## 4. 宿主接入
- [x] 4.1 抽取 szadmin 鉴权与 RBAC 能力到 Adapter
- [x] 4.2 抽取租户隔离能力到 Adapter
- [x] 4.3 抽取审计日志能力到 Adapter
- [x] 4.4 对接流程引擎到 Workflow Adapter

## 5. 样板模块
- [x] 5.1 选定一个核心业务模块作为样板
- [x] 5.2 实现领域模型、用例服务与接口层
- [x] 5.3 通过 ports 接入宿主能力

## 6. 持久化与数据库
- [x] 6.1 以 MyBatis-Flex 实现仓储适配
- [x] 6.2 建立 MySQL/达梦方言隔离策略

## 7. 消息中间件
- [x] 7.1 定义东方通消息适配器
- [x] 7.2 验证领域事件到消息发布链路

## 8. 测试
- [x] 8.1 样板模块单元测试
- [x] 8.2 关键适配层集成测试

## 9. 文档
- [x] 9.1 编写模块开发指南
- [x] 9.2 编写宿主适配指南
