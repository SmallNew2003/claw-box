## MODIFIED Requirements

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
