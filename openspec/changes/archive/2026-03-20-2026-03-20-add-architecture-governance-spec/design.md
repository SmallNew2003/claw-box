# Architecture Governance Design

## Context

项目将长期迭代并持续引入新能力。当前需要建立统一的架构治理规范，避免不同模块在边界划分、依赖方向和一致性机制上出现分歧。

## Goals / Non-Goals

### Goals
- 定义稳定的限界上下文与职责边界
- 统一分层与依赖方向约束
- 定义读写分离与事件一致性规则
- 建立公共注释与契约标准

### Non-Goals
- 本变更不直接完成全部存量代码重构
- 本变更不引入具体工作流引擎实现
- 本变更不替换现有运行时功能行为

## Decisions

- 采用 DDD 作为领域建模方法，六边形作为实现分层模式
- 对关键状态变化采用领域事件建模
- 使用 Outbox 作为默认一致性机制
- 命令与查询分离，读模型按前端视图优化
- Rust/TypeScript 公共契约必须具备规范注释

## Architecture

1. Domain 层只定义业务规则与领域对象
2. Application 层编排用例与事务边界
3. Ports 层定义外部能力抽象
4. Adapters 层实现 DB/SSE/MCP 等对接
5. Interfaces 层处理 HTTP/SSE 输入输出协议

## Context Map

### Bounded Context 划分

- Session Context
  - 责任：会话生命周期、会话并发约束、会话元数据
  - 对外：会话查询与会话命令入口
- Runtime Context
  - 责任：单次 execution 状态机、运行事件、运行指标
  - 对外：执行命令入口、执行事件输出
- Tool Context
  - 责任：工具注册、调用策略、调用审计、MCP/Skills 适配
  - 对外：工具调用 Port
- Team Context
  - 责任：Leader/Worker 协作编排、任务拆分、任务回收
  - 对外：团队任务编排接口
- Memory Context
  - 责任：记忆写入、检索、压缩与生命周期管理
  - 对外：记忆查询与维护接口
- Model Context
  - 责任：模型能力目录、路由策略、供应商适配
  - 对外：模型调用策略接口

### Context Relationship（第一阶段）

- Session -> Runtime：通过应用服务命令触发 execution
- Runtime -> Tool：通过 Tool Port 调用外部工具
- Runtime -> Session：通过领域事件回写会话最新执行摘要
- Team/Memory/Model：当前阶段与 Session/Runtime 解耦，后续通过契约接入

## Directory Blueprint

```text
backend/src/
  contexts/
    <context>/
      domain/
      application/
      ports/
      adapters/
  interfaces/
    http/
    sse/
  platform/
    eventing/
    observability/
    auth/
```

目录约束：
- Domain 不得依赖 adapters/interfaces/platform
- Application 仅依赖 domain 和 ports
- Adapters 实现 ports，不反向依赖 application 内部细节

## Cross-Context Contract

### 应用服务接口契约

- 所有跨上下文同步调用必须经过 Application Facade
- Facade 仅暴露 DTO，不暴露内部实体或聚合引用

### 事件契约

- 所有跨上下文异步通信必须通过领域事件完成
- 事件版本必须可演进（新增字段向后兼容）
- 事件消费者必须支持幂等处理

## Domain Events Catalog (Phase 1)

### Session Context
- SessionCreated
- SessionArchived
- SessionExecutionLinked

### Runtime Context
- ExecutionStarted
- ExecutionStepProgressed
- ExecutionCompleted
- ExecutionFailed

### Tool Context
- ToolCallStarted
- ToolCallCompleted
- ToolCallFailed

事件元数据最小集合：
- event_id
- aggregate_id
- aggregate_type
- causation_id
- correlation_id
- occurred_at
- version

## Outbox Design

### Outbox Table (logical)

| 字段 | 说明 |
|------|------|
| id | Outbox 主键 |
| event_id | 领域事件 ID（唯一） |
| aggregate_id | 聚合 ID |
| event_type | 事件类型 |
| payload | 事件 JSON |
| status | pending / published / failed |
| retry_count | 重试次数 |
| next_retry_at | 下一次重试时间 |
| created_at | 创建时间 |
| published_at | 发布完成时间 |

### Publish Flow

1. 业务事务内：写聚合 + 写 outbox 记录
2. Outbox Dispatcher 轮询 pending 记录
3. 发布成功后更新 published 状态
4. 发布失败按退避策略重试，超过阈值进入 failed

## CQRS Migration Plan

### 写模型范围
- 会话执行命令（创建会话、发送消息、触发执行）
- 运行时状态流转（idle/running/completed/failed）

### 读模型范围
- 会话列表（摘要、状态、更新时间）
- 会话详情（最近执行摘要）
- 事件时间线（按会话、按 execution）

### 迁移步骤
1. 先保持现有接口不变，内部抽出 command/query handler
2. 再引入读模型投影与查询仓储
3. 最后逐步把复杂查询迁移到读模型

## Annotation and Contract Checklist

### Rust
- 所有 `pub struct/enum/trait/fn` 必须有 `///` 注释
- 注释至少包含：职责、输入约束、错误语义

### TypeScript
- 共享类型和公共 DTO 必须有 TSDoc
- 注释至少包含：字段语义、可空规则、兼容要求

### Review Gate
- PR 审查必须检查注释与契约完整性
- 没有注释的公共接口不得合并

## Risks / Trade-offs

- 初期会增加架构与文档成本
- 团队需要统一术语和边界意识
- 迁移期间存在旧实现与新规范并存阶段

## Migration Plan

1. 先把新增功能严格按治理规范实施
2. 再按上下文逐步改造存量模块
3. 对高耦合路径优先引入 Port 和事件契约
4. 最后推进读写分离与 Outbox 全面化

## Open Questions

- 事件总线落地是否优先采用内置实现或外部中间件
- Outbox 消费调度由定时任务还是工作流引擎驱动
