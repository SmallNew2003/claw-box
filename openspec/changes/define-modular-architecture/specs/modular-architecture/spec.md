## ADDED Requirements

### Requirement: 模块化分层骨架
系统 MUST 为每个业务模块提供独立的 domain/application/interfaces/infrastructure 四层结构，且模块间只允许通过应用层契约交互。

#### Scenario: 创建可移植模块
- **WHEN** 新建业务模块
- **THEN** 模块包含四层目录与最小样板代码

### Requirement: shared-kernel 契约
系统 MUST 提供 shared-kernel 以承载租户上下文、权限上下文、审计事件、统一异常与基础值对象，且业务模块只能依赖 shared-kernel 的契约而不反向依赖宿主。

#### Scenario: 业务模块访问租户上下文
- **WHEN** 业务模块读取租户信息
- **THEN** 通过 shared-kernel 的 TenantContext 获取而非直接访问宿主实现

### Requirement: ports/adapters 标准
系统 MUST 为外部能力提供 ports 接口并在宿主层实现 adapters，至少包含 AuthPort、TenantPort、AuditPort、WorkflowPort、MessagingPort。

#### Scenario: 宿主接入 RBAC 与审计
- **WHEN** 业务模块需要鉴权或记录审计
- **THEN** 通过对应 Port 调用宿主 Adapter

### Requirement: 宿主能力接入
系统 MUST 以 szadmin 作为宿主接入 RBAC、租户、审计、流程能力，业务模块不得直接依赖 szadmin 内部实现。

#### Scenario: 业务模块隔离宿主依赖
- **WHEN** 替换宿主实现
- **THEN** 业务模块无需修改，仅替换 Adapter 实现

### Requirement: 持久化与数据库兼容
系统 MUST 将 MyBatis-Flex 仓储实现放在 infrastructure 层，并提供 MySQL/达梦的方言隔离策略。

#### Scenario: 运行在 MySQL 与达梦
- **WHEN** 业务模块执行仓储操作
- **THEN** 可在两种数据库下按方言策略正常运行

### Requirement: 消息中间件适配
系统 MUST 通过 MessagingPort 对接东方通中间件，且消息发布与消费不依赖具体中间件实现。

#### Scenario: 事件发布
- **WHEN** 领域事件触发消息发布
- **THEN** 通过 MessagingPort 发送并由宿主 Adapter 路由到东方通
