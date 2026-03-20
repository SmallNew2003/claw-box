# Client-Core Communication Alignment Design

## Context

Claw Box 的差异化优势在于客户端承担高价值交互与配置能力。为保证在高负载下依旧保持稳定和可观测，需要在协议层明确控制面与数据面的职责边界。

## Goals / Non-Goals

### Goals
- 统一客户端通信模型，避免文档和提案冲突
- 保留客户端操作效率，同时保障高频流式数据稳定性
- 建立可追踪链路（命令到事件）

### Non-Goals
- 本变更不直接改写具体 API
- 本变更不替换现有传输实现

## Decisions

- Decision: 客户端采用双通道架构
  - 控制面：Tauri Command
  - 数据面：本地 HTTP + SSE
- Decision: 双通道共享 trace_id
- Decision: 控制面不承载高频事件流，数据面不承载配置写入

## Architecture

```text
src-ui/
  control-plane/
    commands/
  data-plane/
    queries/
    streams/

backend/
  interfaces/command/
  interfaces/http/
  interfaces/sse/
```

## Risks / Trade-offs

- 双通道增加实现复杂度 -> 通过统一 DTO 与契约测试降低风险
- trace 贯通需要统一规范 -> 在治理规范中强制字段

## Migration Plan

1. 先更新治理规范和项目高层描述
2. 后续变更按双通道模型新增或迁移
3. 对既有提案逐步补齐通道边界说明

## Open Questions

- 本地 HTTP 是否统一走 UDS（Unix Domain Socket）以进一步降低开销
