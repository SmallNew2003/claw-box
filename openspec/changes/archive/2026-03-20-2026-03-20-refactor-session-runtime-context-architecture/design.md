# Session/Runtime Context Refactor Design

## Context

当前后端已具备单 Agent 运行闭环能力，但模块组织偏向实现导向而非领域导向。需要在不改变外部行为的前提下，落地 architecture-governance 的第一批上下文改造。

## Goals / Non-Goals

### Goals
- 完成 Session/Runtime 两个上下文的目录与依赖分层
- 抽出 Port 接口并将存储与事件分发放入 Adapter
- 保持现有会话 API 行为兼容
- 为 Agent Teams 和 Context Gateway 提供稳定接入边界
- 满足客户端高频操作下的高性能与高稳定要求

### Non-Goals
- 本次不改造 Team/Memory/MCP/Model 全量上下文
- 本次不引入完整 Outbox 基础设施实现（先保留接口与占位）
- 本次不改变前端交互语义

## Decisions

- Decision: 先从 Session/Runtime 开始，形成可复用迁移模板
- Decision: 旧模块保留过渡期，新增 Context Facade 作为唯一新入口
- Decision: 写路径先迁移，读路径按查询复杂度逐步迁移
- Decision: 事件发布先通过 Port 抽象，后续接 Outbox Adapter
- Decision: 客户端采用控制面/数据面双通道
  - 控制面：Tauri Command 处理配置与管理命令
  - 数据面：本地 HTTP + SSE 承载高频状态与事件流
- Decision: 前端是产品控制中心，但执行一致性由本地 Core Service 负责

## Architecture

```text
backend/src/
  contexts/
    session/
      domain/
      application/
      ports/
      adapters/
    runtime/
      domain/
      application/
      ports/
      adapters/
  interfaces/http/
    sessions.rs
  interfaces/command/
    sessions.rs
```

```text
src-ui/
  control-plane/
    commands/      # 创建会话、更新配置、触发执行
  data-plane/
    streams/       # SSE 订阅运行事件与日志
    queries/       # 会话摘要、历史事件查询
```

依赖规则：
- `domain` 仅依赖本上下文 domain 对象
- `application` 依赖 `domain + ports`
- `adapters` 实现 `ports`
- `interfaces/http` 仅调用 `application facade`
- `interfaces/command` 仅负责命令输入映射，不承载运行时业务规则

## Contract Plan

### Client Communication Contract
- 控制面命令：
  - `create_session`
  - `update_session_config`
  - `execute_session_message`
- 数据面事件：
  - `execution_started`
  - `step_progress`
  - `tool_call_started`
  - `tool_call_completed`
  - `execution_completed`
  - `execution_failed`

控制面与数据面共享 `trace_id`，保证命令与事件可追踪关联。

### Session <-> Runtime
- Session application 触发 Runtime command
- Runtime 返回 execution result + runtime events
- Session 持久化最新执行摘要

### Runtime Event Port
- `publish_runtime_event(event)`
- `store_runtime_event(event)`

## Migration Plan

1. 创建 Session/Runtime context 目录与骨架接口
2. 搬迁 RuntimeExecution 状态机与执行服务到 context
3. 搬迁 Session 聚合与会话应用服务到 context
4. 迁移 HTTP sessions 入口到 application facade
5. 为旧路径保留兼容适配，完成后删除旧服务模块

## Risks / Trade-offs

- 双轨期存在重复代码风险 -> 用单一 facade 约束入口
- 迁移中可能出现依赖环 -> 在 code review 中检查依赖方向
- 测试调整成本上升 -> 先保留行为测试再补上下文单测
- 双通道增加接口复杂度 -> 通过统一 DTO、统一 trace_id 与契约测试控制复杂度

## Open Questions

- Outbox 在本阶段是否仅落接口还是同步落地 SQLite adapter
