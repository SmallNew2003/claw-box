# Change: 添加 Agent Teams 生命周期与协作管理

## Why

当前项目仍处于基础能力建设阶段，优先级应放在单 Agent 运行时、模型供应商与模型管理、模式切换、思考级别、Skills 与 MCP 集成。

因此，Agent Teams 不适合作为第一阶段实现目标，而更适合作为后置增强能力进行规划，等基础运行时稳定后再逐步落地。

当项目进入多 Agent 阶段时，仍需要补足以下能力：

1. **团队状态不明确**：团队创建后缺少清晰的生命周期状态与状态转换约束
2. **成员协作低效**：如果共享完整上下文，会带来显著的 token 成本和状态复杂度
3. **资源无法清理**：团队结束后，消息历史、工作区和运行痕迹缺少统一归档与清理机制

## What Changes

### 1. 团队生命周期管理
- 添加团队状态机（draft → ready → running → paused/completed/failed → archived）
- 实现团队生命周期 API（创建、配置、激活、暂停、恢复、停止、归档、删除）
- 添加团队归档流程（工作区清理、日志导出）

### 2. 团队成员交互机制
- 实现消息总线（结构化消息传递，< 200 tokens/消息）
- 实现共享工作区（产出物存储和按需读取）
- 设计最小共享上下文（任务目标、约束条件）

### 3. 数据模型和 API
- 添加团队表、消息表、工作区表
- 实现 TeamLifecycleAPI、MessageBus、WorkspaceAPI

### 3.1 客户端通信边界（对齐架构治理）
- 控制面：通过 Tauri Command 触发团队生命周期与管理命令
- 数据面：通过本地 HTTP + SSE 提供团队状态、消息流、工作区事件
- 命令与事件通过 `trace_id` 贯通，支持端到端追踪

### 4. 实施前置条件
- 依赖单 Agent 运行时基础能力先行完成
- 依赖模型管理、模式切换、思考级别、Skills、MCP 作为独立 change 落地
- 本提案不包含长期记忆、分层知识库和 embedding 检索实现

## Impact

### Affected specs
- `agent-teams` - 添加生命周期管理、成员交互机制

### Affected code
- 后端（新增）：
  - `backend/src/services/agent_teams.rs` - 团队生命周期服务
  - `backend/src/services/message_bus.rs` - 消息总线服务
  - `backend/src/services/workspace.rs` - 工作区服务
  - `backend/src/models/team.rs` - 团队数据模型
  - `backend/src/models/message.rs` - 团队消息模型
  - `backend/src/models/workspace.rs` - 工作区模型
  - `backend/src/api/teams.rs` - 团队 API 端点
  - `backend/src/db/migrations/` - 数据库迁移脚本

- 前端（修改）：
  - `src-ui/components/AgentTeams/` - 团队状态展示、生命周期操作
  - `src-ui/api/teams.ts` - 团队 API 客户端
  - `src-ui/api/messages.ts` - 团队消息 API 客户端
  - `src-ui/api/workspace.ts` - 工作区 API 客户端

### Breaking changes
无直接破坏性变更；该提案为后置能力规划，实施前依赖基础运行时能力先完成。

### Migration
- 当前无需执行迁移
- 当项目进入多 Agent 阶段时，需要根据届时的基础运行时和团队状态模型补充兼容策略
