# architecture-governance Specification

## Purpose
定义 Claw Box 的长期架构治理约束，统一限界上下文、分层依赖方向、读写分离、事件一致性与公共契约注释标准，确保系统在持续迭代中保持可维护性与可演进性。
## Requirements
### Requirement: 限界上下文与职责边界
系统 SHALL 以限界上下文组织核心业务能力，并为每个上下文定义稳定职责边界。

#### Scenario: 定义核心上下文
- **WHEN** 团队规划新的能力模块
- **THEN** 必须首先确定该模块归属的限界上下文
- **AND** 明确该上下文的职责、输入输出和跨上下文依赖

#### Scenario: 控制跨上下文调用
- **WHEN** 一个上下文需要使用另一个上下文能力
- **THEN** 必须通过显式应用服务接口或事件契约交互
- **AND** 禁止直接依赖对方内部领域对象

### Requirement: 分层与依赖方向
系统 SHALL 采用六边形分层，确保依赖方向从外向内，领域层不依赖基础设施实现。

#### Scenario: 领域层纯净性
- **WHEN** 在领域层实现实体、值对象、聚合或领域服务
- **THEN** 该层代码不得直接依赖 HTTP、数据库、消息中间件和 UI 框架
- **AND** 外部能力必须通过 Port 接口抽象

#### Scenario: 适配器实现
- **WHEN** 系统接入数据库、SSE、MCP 或外部服务
- **THEN** 必须在 Adapter 层实现对应 Port
- **AND** 应用层只依赖 Port 抽象而非具体实现

#### Scenario: 首批上下文迁移落地
- **WHEN** 系统推进架构治理的首批实施
- **THEN** 必须优先完成 Session 和 Runtime 两个上下文的分层迁移
- **AND** 对外接口保持行为兼容

#### Scenario: Application 层职责边界
- **WHEN** 在 Application 层实现用例服务
- **THEN** Application 层只负责用例编排、事务边界与端口协作
- **AND** 不得承载领域不变量、状态机规则或实体校验逻辑

#### Scenario: Application 依赖 Port 抽象
- **WHEN** Application 层访问持久化、事件流或外部能力
- **THEN** 只能依赖 Port 接口
- **AND** 不得直接依赖具体 Adapter 实现类型

#### Scenario: 测试代码组织规范
- **WHEN** 在后端模块新增或维护单元测试
- **THEN** 测试代码必须放在独立测试文件（例如 `*_tests.rs`）
- **AND** 生产实现文件默认不得内联 `#[cfg(test)] mod tests`

### Requirement: 读写分离与一致性
系统 SHALL 对命令写模型与查询读模型进行分离，并通过事件机制保持跨边界最终一致。

#### Scenario: 命令写模型
- **WHEN** 用户触发会话执行、工具调用或状态流转
- **THEN** 系统必须通过写模型处理业务规则和聚合一致性
- **AND** 在聚合边界内保证强一致

#### Scenario: 查询读模型
- **WHEN** 前端查询会话列表、执行时间线或统计视图
- **THEN** 系统应优先使用读模型查询
- **AND** 禁止将写模型对象直接作为复杂查询视图返回

#### Scenario: 首批读写分离落地
- **WHEN** Session/Runtime 架构迁移实施
- **THEN** 会话执行命令必须走写模型
- **AND** 会话摘要与事件时间线查询必须通过独立查询路径输出

### Requirement: 领域事件与 Outbox
系统 SHALL 将关键业务变化建模为领域事件，并通过 Outbox 机制保证持久化与发布的一致性。

#### Scenario: 发布关键事件
- **WHEN** 发生执行开始、完成、失败或工具调用完成等关键状态变化
- **THEN** 系统必须生成对应领域事件并落盘
- **AND** 事件至少包含 event_id、aggregate_id、occurred_at 和版本信息

#### Scenario: 发布一致性
- **WHEN** 业务写入与事件发布处于同一事务边界
- **THEN** 系统必须使用 Outbox 或等效机制保证两者一致
- **AND** 支持幂等消费和失败重试

### Requirement: 注释与契约文档标准
系统 SHALL 对公共接口和共享契约提供规范注释，以保证跨团队协作和长期可维护性。

#### Scenario: Rust 公共接口注释
- **WHEN** 新增或修改 Rust 的公共类型与公共方法
- **THEN** 必须提供 `///` 文档注释说明职责、输入约束与错误语义

#### Scenario: TypeScript 共享契约注释
- **WHEN** 新增或修改前后端共享类型或公共 API DTO
- **THEN** 必须提供 TSDoc 注释说明字段语义、可空规则和兼容要求

### Requirement: 客户端控制面与数据面分离
系统 SHALL 在客户端高负载场景中采用控制面与数据面的职责分离，保证高性能与高稳定性。

#### Scenario: 控制面命令
- **WHEN** 用户执行配置修改、会话操作或任务触发
- **THEN** 系统必须通过控制面命令通道处理
- **AND** 控制面不得承载高频事件流

#### Scenario: 数据面流式事件
- **WHEN** 系统输出高频运行状态、日志或时间线事件
- **THEN** 系统必须通过流式数据通道输出
- **AND** 数据面不得反向承担配置写入职责

#### Scenario: 端到端追踪
- **WHEN** 一条控制面命令触发后续运行事件
- **THEN** 控制面请求与数据面事件必须具备统一 trace_id
- **AND** 系统必须支持按 trace 回放命令到事件链路

