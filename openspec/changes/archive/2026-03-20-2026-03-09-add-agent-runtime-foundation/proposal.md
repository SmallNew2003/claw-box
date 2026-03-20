# Change: 添加单 Agent 运行时基础能力

## Metadata

- Created: 2026-03-12T08:55:20Z
- Affected Specs:
  - agent-runtime (new)
  - session-management (modified: 2026-03-09T09:24:59Z)

## Why

当前项目还没有真正可运行的 Agent 基础闭环。

在开始 Agent Teams、长期记忆、知识库分层等复杂能力之前，必须先具备最小的单 Agent 执行能力，确保前后端可以围绕同一条主链路稳定开发和验证。

第一阶段需要优先解决的问题是：

1. **没有基础运行时**：缺少单 Agent 的会话执行、状态流转和结果记录
2. **没有最小交互闭环**：前端无法真实驱动后端执行，后端也没有统一运行事件输出
3. **没有垂直切片基线**：后续模型管理、Skills、MCP、Agent Teams 都缺少统一挂载点

## What Changes

### 1. 单 Agent 运行时
- 实现单 Agent 会话执行状态机（idle → running → completed/failed）
- 支持用户消息触发一次执行
- 记录执行结果、错误信息和基本运行指标

### 2. 基础工具调用能力
- 定义运行时工具调用接口
- 支持执行过程中产生工具调用记录
- 保留后续接入 Skills 与 MCP 的扩展点

### 3. 运行事件与日志
- 提供基础运行事件流（开始、步骤、完成、失败）
- 支持前端按会话查看执行日志和当前状态

### 4. 前后端最小闭环
- 后端提供会话创建、执行、详情、日志查询 API
- 前端提供会话列表、消息输入、执行状态和结果面板
- 按垂直切片同步开发前后端，而不是先堆单侧能力

### 5. 实施边界
- 本提案不包含模型供应商管理与模型切换
- 本提案不包含 `build/plan` 模式切换与思考级别配置
- 本提案不包含 Skills、MCP、Agent Teams、长期记忆

## Impact

### Affected specs
- `agent-runtime` - 新增单 Agent 运行时能力
- `session-management` - 补充会话与运行时的衔接关系

### Affected code
- 后端（新增/修改）：
  - `backend/src/services/agent_runtime.rs` - 单 Agent 运行时服务
  - `backend/src/services/session_service.rs` - 会话与运行时衔接
  - `backend/src/models/session.rs` - 会话模型扩展
  - `backend/src/models/runtime_event.rs` - 运行事件模型
  - `backend/src/api/sessions.rs` - 会话与执行 API

- 前端（新增/修改）：
  - `src-ui/components/Sessions/` - 会话列表、输入框、结果面板
  - `src-ui/api/sessions.ts` - 会话 API 客户端
  - `src-ui/stores/session-store.ts` - 会话运行状态管理

### Breaking changes
无破坏性变更，属于项目第一阶段基础能力新增。

### Migration
当前无需迁移。
