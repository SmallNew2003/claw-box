## ADDED Requirements

### Requirement: Context Gateway Unified Entry
系统 SHALL 提供统一的 Context Gateway API，作为 UI 与 Agent 的唯一上下文入口。

#### Scenario: 所有上下文请求走网关
- **WHEN** 客户端发起上下文搜索、读取、浏览或提交请求
- **THEN** 请求必须进入 Core 的 Context Gateway
- **AND** Gateway 负责将业务请求映射到 OpenViking API
- **AND** 客户端不得直接调用 OpenViking

### Requirement: URI Canonicalization and Mapping
系统 SHALL 对外使用 `ctx://` 规范，并在网关层映射到 OpenViking `viking://` URI。

#### Scenario: ctx URI 转换为 viking URI
- **WHEN** 网关接收到 `ctx://tenant/{tid}/project/{pid}/...` 请求
- **THEN** 网关将其转换为对应的 OpenViking URI
- **AND** 返回结果中的 URI 保持 `ctx://` 规范

### Requirement: Explainable Retrieval
系统 SHALL 在检索结果中返回可解释信息，支持检索回放与运维诊断。

#### Scenario: 检索返回命中解释
- **WHEN** 客户端执行 context search
- **THEN** 每个命中项包含 `retrieval_path` 与 `rank_features`
- **AND** 响应包含 `trace_id` 与 `usage` 指标

### Requirement: Async Commit Workflow
系统 SHALL 采用异步 commit 任务机制，不以同步 commit 作为默认路径。

#### Scenario: 发起异步 commit
- **WHEN** 客户端调用 session commit
- **THEN** 网关返回 `task_id`
- **AND** 客户端可通过任务查询接口获取状态
- **AND** 任务状态必须为 `pending|running|completed|failed|cancelled` 之一

### Requirement: Commit Idempotency and Concurrency Control
系统 SHALL 对消息写入和 commit 提供幂等保障与并发控制。

#### Scenario: 重复消息写入
- **WHEN** 相同 `idempotency_key` 的消息重复写入
- **THEN** 系统不得重复创建写入副本
- **AND** 返回一致结果

#### Scenario: 同会话重复触发 commit
- **WHEN** 同一 `session_id` 已有活跃 commit 任务
- **THEN** 新请求返回已有 `task_id`
- **AND** 不创建第二个活跃任务

### Requirement: Tenant and Project Isolation
系统 SHALL 在 V1 以 tenant/project 级硬隔离作为主要安全边界。

#### Scenario: 非授权项目访问
- **WHEN** 请求尝试读取其他 project 范围上下文
- **THEN** 网关拒绝请求并返回授权错误
- **AND** 记录审计日志

### Requirement: Observability and Audit
系统 SHALL 对上下文关键流程提供结构化日志、指标与 trace 贯穿。

#### Scenario: 检索与提交全链路观测
- **WHEN** context search 或 context commit 完成或失败
- **THEN** 系统记录结构化事件日志
- **AND** 记录延迟、token、成功率等核心指标
- **AND** 日志中包含 `trace_id`、`session_id`、`run_id`（如存在）

### Requirement: V1 Scope Guardrails
系统 SHALL 明确限制 V1 范围，避免提前引入高复杂度能力。

#### Scenario: V1 需求边界验证
- **WHEN** 评审功能范围
- **THEN** 系统不得包含跨租户共享知识空间
- **AND** 系统不得包含可视化策略 DSL
- **AND** Memory Scoring 必须作为独立异步任务（非 commit 强耦合）
