# Agent Runtime Foundation Design

## Context

当前系统缺少可运行的单 Agent 执行闭环，前后端无法围绕统一的会话执行链路进行开发与验证。本变更需要建立最小可用的运行时能力，包括执行状态机、事件日志与基础工具调用记录。

## Goals / Non-Goals

### Goals
- 单 Agent 会话执行状态机可用
- 支持用户消息触发一次执行
- 运行事件与工具调用可追踪
- 前端可查看运行状态、结果与日志

### Non-Goals
- 模型供应商管理与模型切换
- build/plan 模式切换与思考级别配置
- Skills、MCP、Agent Teams、长期记忆

## Decisions

- 执行模型以一次用户消息触发一个 Execution 为单位，单会话不并发执行
- 会话持有最近一次 Execution 摘要，详细事件单独存储
- 运行事件采用统一结构，支持事件流与历史查询
- 工具调用以结构化记录保存输入、输出与错误
- 事件流对齐 `docs/event-protocol.md` 与 `docs/api-specification.md`

## Architecture

1. SessionService 接收用户消息并创建 Execution
2. AgentRuntimeService 执行并产出运行事件、结果与指标
3. RuntimeEvent 通过事件总线分发到 SSE 流
4. 前端订阅会话事件流并拉取历史日志

## Data Model

### RuntimeExecution
```typescript
interface RuntimeExecution {
  execution_id: string;
  session_id: string;
  input_message_id: string;
  status: "running" | "completed" | "failed";
  started_at: string;      // ISO 8601
  completed_at?: string;   // ISO 8601
  result_summary?: string; // 最近结果摘要
  error_message?: string;  // 失败原因
  metrics?: RuntimeMetrics;
}

interface RuntimeMetrics {
  latency_ms: number;
  input_tokens: number;
  output_tokens: number;
}
```

### RuntimeEvent
```typescript
interface RuntimeEvent {
  event_id: string;
  execution_id: string;
  session_id: string;
  type:
    | "execution_started"
    | "step_progress"
    | "tool_call_started"
    | "tool_call_completed"
    | "execution_completed"
    | "execution_failed";
  payload: Record<string, unknown>;
  created_at: string; // ISO 8601
}
```

### ToolCall
```typescript
interface ToolCall {
  tool_call_id: string;
  execution_id: string;
  tool_name: string;
  status: "running" | "completed" | "failed";
  input: Record<string, unknown>;
  output?: Record<string, unknown>;
  error_message?: string;
  started_at: string;    // ISO 8601
  completed_at?: string; // ISO 8601
}
```

## API

沿用并扩展会话 API：

- `POST /sessions` 创建会话
- `GET /sessions` 会话列表
- `GET /sessions/{session_id}` 会话详情（包含最近执行摘要）
- `POST /sessions/{session_id}/messages` 触发一次执行
- `GET /sessions/{session_id}/events` SSE 事件流
- `GET /sessions/{session_id}/events/history` 历史事件查询（JSON）

事件流格式遵循 `docs/event-protocol.md`，会话 API 细节遵循 `docs/api-specification.md`。

## Error Handling

- `SESSION_NOT_FOUND`: 会话不存在
- `SESSION_RUNNING`: 会话正在运行，拒绝并发执行
- `VALIDATION_ERROR`: 请求参数不合法
- `INTERNAL_ERROR`: 运行时内部错误

错误响应与通用错误码规范保持一致，SSE 中的错误事件使用 `event: error`。

## Dependencies

- 事件总线与 SSE 连接管理（参见 `docs/event-protocol.md`）
- 会话模型与存储层
- API 统一错误码（参见 `docs/api-specification.md`）

## Performance / Security

- SSE 心跳与连接限制遵循事件协议
- 事件写入与查询支持分页与上限控制
- 会话与事件访问需校验用户权限

## Migration Plan

无迁移需求，新增能力与字段默认可空。

## Open Questions

无。
