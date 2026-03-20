# Change: 按上下文与六边形分层重构 Session/Runtime 后端架构

## Metadata
- Created: 2026-03-20T11:30:00Z
- Affected Specs:
  - architecture-governance (modified: 2026-03-20T10:30:00Z)
  - session-management (modified: 2026-03-09T09:24:59Z)

## Why

`add-agent-runtime-foundation` 已完成最小可运行闭环，但当前实现仍以扁平模块组织（`services/models/api`）。

随着 Agent Teams、Context Gateway、Memory 等能力推进，若不先把 Session/Runtime 迁移到限界上下文 + 六边形分层，并明确客户端高负载场景下的控制/数据通道职责，后续将出现：

1. 领域规则与基础设施耦合，难以复用和测试
2. 会话写模型与读查询模型混用，演进成本上升
3. 客户端承担大量配置与操作时，控制命令与高频流式数据互相争抢资源
4. 跨模块扩展时缺少稳定 Port/Contract 边界

## What Changes

1. 将后端 Session/Runtime 迁移为上下文结构：
   - `contexts/session/*`
   - `contexts/runtime/*`
2. 在每个上下文内部建立分层：`domain/application/ports/adapters`
3. 定义会话执行链路的跨上下文契约（Application Facade + 领域事件）
4. 引入读写分离的最小落地：
   - 写：会话执行命令
   - 读：会话摘要与事件时间线查询
5. 明确客户端通信策略：
   - 控制面：Tauri Command（配置、操作命令）
   - 数据面：本地 HTTP + SSE（高频状态、日志与事件流）
6. 保持外部 API 行为兼容，不引入业务语义破坏

## Impact

### Affected specs
- `architecture-governance`：从治理原则推进到 Session/Runtime 的首批实施路径
- `session-management`：补充会话执行链路在新分层下的实现约束

### Affected code
- `backend/src/contexts/session/`（新增）
- `backend/src/contexts/runtime/`（新增）
- `backend/src/interfaces/http/sessions.rs`（迁移）
- `backend/src/services/*`、`backend/src/models/*`（逐步收敛）
- `src-ui/`（通过 Command 调用控制操作，通过 SSE 订阅运行事件）

### Breaking changes
无外部 API 破坏性变更。

### Migration
采用渐进迁移：先双轨保留，再切换入口，最终移除旧路径。
