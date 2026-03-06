# Change: 添加 Agent Teams 生命周期与知识库管理

## Why

当前系统缺少完整的 Agent Teams 生命周期管理和知识库系统，导致以下问题：

1. **团队状态不明确**：团队创建后无法追踪其运行状态（运行中、暂停、完成、失败）
2. **知识无法沉淀**：临时团队执行任务产生的知识随团队解散而丢失
3. **成员协作低效**：团队成员之间共享完整上下文会导致 token 爆炸，无法高效协作
4. **资源无法清理**：团队归档后，相关资源（工作区、临时知识）无法自动清理

需要设计一套完整的团队生命周期管理机制，配套分层知识库系统和高效的成员交互机制。

## What Changes

### 1. 团队生命周期管理
- 添加团队状态机（draft → ready → running → paused/completed/failed → archived）
- 实现团队生命周期 API（创建、配置、激活、暂停、恢复、停止、归档、删除）
- 添加团队归档流程（知识提升、工作区清理、日志导出）

### 2. 分层知识库系统
- 实现三层知识库架构：全局知识库、项目知识库、团队运行时上下文
- 添加知识生命周期管理（Active → Cold → Archived → Deleted）
- 实现知识重要性自动评估（LLM 评分）
- 添加知识提升机制（团队 → 项目 → 全局）
- 实现知识库容量管理和自动清理

### 3. 团队成员交互机制
- 实现消息总线（结构化消息传递，< 200 tokens/消息）
- 实现共享工作区（产出物存储和按需读取）
- 设计最小共享上下文（任务目标、约束条件）

### 4. 数据模型和 API
- 添加团队表、知识库表、消息表、工作区表
- 实现 TeamLifecycleAPI、KnowledgeAPI、MessageBus、WorkspaceAPI

## Impact

### Affected specs
- `agent-teams` - 添加生命周期管理、成员交互机制
- `memory-system` - 添加分层知识库、生命周期管理

### Affected code
- 后端（新增）：
  - `backend/src/services/agent_teams.rs` - 团队生命周期服务
  - `backend/src/services/knowledge.rs` - 知识库服务
  - `backend/src/services/message_bus.rs` - 消息总线服务
  - `backend/src/services/workspace.rs` - 工作区服务
  - `backend/src/models/team.rs` - 团队数据模型
  - `backend/src/models/knowledge.rs` - 知识数据模型
  - `backend/src/api/teams.rs` - 团队 API 端点
  - `backend/src/api/knowledge.rs` - 知识库 API 端点
  - `backend/src/db/migrations/` - 数据库迁移脚本

- 前端（修改）：
  - `src-ui/components/AgentTeams/` - 团队状态展示、生命周期操作
  - `src-ui/components/Memory/` - 知识库管理界面
  - `src-ui/api/teams.ts` - 团队 API 客户端
  - `src-ui/api/knowledge.ts` - 知识库 API 客户端

### Breaking changes
无破坏性变更（纯新增功能）

### Migration
无需迁移（新功能）
