# Agent Teams

## Purpose
定义 Agent Teams 的团队编排、动态任务分配与协作约束，确保多 Agent 协同执行过程可观测、可控且具备稳定的生命周期管理。

## Requirements

### Requirement: Teams 状态展示
系统 SHALL 展示 Agent Teams 的成员、角色与运行状态。

#### Scenario: 查看团队状态
- **WHEN** 用户进入 Teams 页面
- **THEN** 系统显示团队成员列表与当前状态

#### Scenario: 查看 Worker 详情
- **WHEN** 用户点击某个 Worker
- **THEN** 系统显示该 Worker 的任务历史和性能指标

### Requirement: 任务分派可视化
系统 SHALL 显示团队任务分派与执行进度。

#### Scenario: 查看任务分派
- **WHEN** 团队任务开始执行
- **THEN** 系统展示任务负责人与进度状态

#### Scenario: 实时进度更新
- **WHEN** Worker 报告任务进度
- **THEN** 系统实时更新进度条和状态信息

### Requirement: 动态任务分配
系统 SHALL 支持 Leader 根据任务复杂度动态分配 Worker 数量。

#### Scenario: 自动评估任务复杂度
- **WHEN** Leader 接收到新任务
- **THEN** 系统分析任务复杂度并决定需要的 Worker 数量（1-10）

#### Scenario: 动态扩容
- **WHEN** 任务复杂度高于预期
- **THEN** Leader 自动创建额外的 Worker 协助完成任务

#### Scenario: 动态缩容
- **WHEN** Worker 完成任务且无新任务分配
- **THEN** 系统自动释放空闲 Worker 以节省资源

### Requirement: Worker 请求支援
系统 SHALL 支持 Worker 在遇到困难时请求 Leader 支援。

#### Scenario: Worker 请求支援
- **WHEN** Worker 无法独立完成子任务
- **THEN** Worker 向 Leader 发送支援请求，说明遇到的问题

#### Scenario: Leader 提供上下文
- **WHEN** Leader 收到支援请求
- **THEN** Leader 分析问题并提供额外的上下文信息或指导

#### Scenario: Leader 分配协助者
- **WHEN** Worker 需要协作完成任务
- **THEN** Leader 分配另一个 Worker 协助完成任务

### Requirement: Worker 与 Leader 协商
系统 SHALL 支持 Worker 与 Leader 之间的任务协商机制。

#### Scenario: Worker 提出任务调整建议
- **WHEN** Worker 发现任务定义不合理
- **THEN** Worker 向 Leader 提出调整建议

#### Scenario: Leader 批准任务调整
- **WHEN** Leader 评估 Worker 的建议
- **THEN** Leader 批准或拒绝调整，并更新任务定义

#### Scenario: Worker 请求任务重新分配
- **WHEN** Worker 认为自己不适合当前任务
- **THEN** Worker 请求 Leader 将任务重新分配给其他 Worker

### Requirement: 上下文管理优化
系统 SHALL 采用主代理+子代理架构，减少 Token 消耗。

#### Scenario: 主代理保持完整上下文
- **WHEN** Leader 管理团队
- **THEN** Leader 维护完整的任务上下文和团队状态

#### Scenario: 子代理接收精简上下文
- **WHEN** Worker 执行子任务
- **THEN** Worker 只接收与子任务相关的精简上下文

#### Scenario: 上下文同步
- **WHEN** Worker 完成子任务
- **THEN** Worker 将结果摘要同步给 Leader，而非完整输出

### Requirement: 团队生命周期管理
系统 SHALL 支持创建、激活、暂停、解散团队。

#### Scenario: 创建团队
- **WHEN** 用户创建新团队
- **THEN** 系统初始化 Leader 和最小数量的 Worker

#### Scenario: 激活团队
- **WHEN** 用户启动团队任务
- **THEN** 系统激活团队并开始任务分配

#### Scenario: 解散团队
- **WHEN** 所有任务完成或用户手动解散
- **THEN** 系统释放所有 Worker 并归档团队记录

## Non-Functional Requirements

### Performance
- 任务分配延迟 < 200ms
- Worker 创建时间 < 1s
- 上下文同步延迟 < 500ms
- 支持单团队最多 10 个 Worker

### Scalability
- 支持最多 50 个并发团队
- 支持最多 500 个并发 Worker
- 任务队列容量 1000 个任务

### Reliability
- Worker 失败自动重试（最多 3 次）
- Leader 失败自动恢复或重新选举
- 任务状态持久化，系统重启后可恢复

### Token Efficiency
- 子代理上下文减少 60-70% Token 消耗
- 结果摘要压缩率 > 80%
- 主代理上下文增长速度 < 10 tokens/任务

## Data Model

### AgentTeam
```typescript
interface AgentTeam {
  team_id: string;              // 唯一标识符
  name: string;                 // 团队名称
  status: TeamStatus;           // 团队状态
  leader: Leader;               // Leader 配置
  workers: Worker[];            // Worker 列表
  config: TeamConfig;           // 团队配置
  task_queue: Task[];           // 任务队列
  created_at: string;           // 创建时间（ISO 8601）
  updated_at: string;           // 更新时间（ISO 8601）
  disbanded_at?: string;        // 解散时间（ISO 8601）
}

enum TeamStatus {
  Created = "created",          // 已创建
  Active = "active",            // 活跃中
  Paused = "paused",            // 已暂停
  Completed = "completed",      // 已完成
  Failed = "failed",            // 失败
  Disbanded = "disbanded"       // 已解散
}

interface TeamConfig {
  min_workers: number;          // 最小 Worker 数量（默认 1）
  max_workers: number;          // 最大 Worker 数量（默认 10）
  auto_scale: boolean;          // 是否自动扩缩容（默认 true）
  context_strategy: ContextStrategy; // 上下文策略
  timeout_seconds: number;      // 任务超时时间（秒）
}

enum ContextStrategy {
  Full = "full",                // 完整上下文（高 Token 消耗）
  Optimized = "optimized",      // 优化上下文（推荐）
  Minimal = "minimal"           // 最小上下文（低 Token 消耗）
}
```

### Leader
```typescript
interface Leader {
  agent_id: string;             // Agent 唯一标识符
  model_id: string;             // 使用的模型 ID
  status: AgentStatus;          // Agent 状态
  system_prompt: string;        // 系统提示词
  context: LeaderContext;       // Leader 上下文
  tasks_assigned: number;       // 已分配任务数
  created_at: string;           // 创建时间（ISO 8601）
}

interface LeaderContext {
  team_goal: string;            // 团队目标
  task_history: TaskSummary[];  // 任务历史摘要
  worker_performance: WorkerPerformance[]; // Worker 性能记录
  total_tokens: number;         // 总 Token 消耗
}

interface TaskSummary {
  task_id: string;
  description: string;
  result_summary: string;       // 结果摘要（压缩后）
  worker_id: string;
  duration_ms: number;
}

interface WorkerPerformance {
  worker_id: string;
  tasks_completed: number;
  tasks_failed: number;
  avg_duration_ms: number;
  success_rate: number;
}
```

### Worker
```typescript
interface Worker {
  agent_id: string;             // Agent 唯一标识符
  model_id: string;             // 使用的模型 ID
  status: AgentStatus;          // Agent 状态
  current_task?: Task;          // 当前任务
  context: WorkerContext;       // Worker 上下文
  tasks_completed: number;      // 已完成任务数
  created_at: string;           // 创建时间（ISO 8601）
}

enum AgentStatus {
  Idle = "idle",                // 空闲
  Running = "running",          // 运行中
  Waiting = "waiting",          // 等待支援
  Failed = "failed"             // 失败
}

interface WorkerContext {
  assigned_task: string;        // 分配的子任务描述
  relevant_context: string;     // 相关上下文（精简）
  constraints: string[];        // 任务约束
  total_tokens: number;         // 总 Token 消耗
}
```

### Task
```typescript
interface Task {
  task_id: string;              // 唯一标识符
  team_id: string;              // 所属团队 ID
  description: string;          // 任务描述
  complexity: TaskComplexity;   // 任务复杂度
  status: TaskStatus;           // 任务状态
  assigned_worker?: string;     // 分配的 Worker ID
  helper_workers?: string[];    // 协助的 Worker ID 列表
  result?: string;              // 任务结果
  support_requests: SupportRequest[]; // 支援请求列表
  created_at: string;           // 创建时间（ISO 8601）
  started_at?: string;          // 开始时间（ISO 8601）
  completed_at?: string;        // 完成时间（ISO 8601）
}

enum TaskComplexity {
  Low = "low",                  // 低复杂度（1 Worker）
  Medium = "medium",            // 中复杂度（2-3 Workers）
  High = "high",                // 高复杂度（4-6 Workers）
  VeryHigh = "very_high"        // 极高复杂度（7-10 Workers）
}

enum TaskStatus {
  Pending = "pending",          // 待分配
  Assigned = "assigned",        // 已分配
  InProgress = "in_progress",   // 进行中
  WaitingSupport = "waiting_support", // 等待支援
  Completed = "completed",      // 已完成
  Failed = "failed"             // 失败
}

interface SupportRequest {
  request_id: string;
  worker_id: string;
  reason: string;               // 请求原因
  requested_at: string;         // 请求时间（ISO 8601）
  response?: SupportResponse;   // Leader 响应
}

interface SupportResponse {
  action: SupportAction;
  details: string;
  responded_at: string;         // 响应时间（ISO 8601）
}

enum SupportAction {
  ProvideContext = "provide_context",     // 提供额外上下文
  AssignHelper = "assign_helper",         // 分配协助者
  Reassign = "reassign",                  // 重新分配任务
  AdjustTask = "adjust_task"              // 调整任务定义
}
```

## State Machine

### Team State Transitions
```
Created ────────────────────────────────────────────────────────┐
  │                                                              │
  │ activate()                                                   │
  ▼                                                              │
Active ─────────────────────────────────────────────────────────┤
  │                                                              │
  │ complete_all_tasks()                                         │
  ▼                                                              │
Completed ──────────────────────────────────────────────────────┤
  │                                                              │
  │ disband()                                                    │
  └──────────────────────────────────────────────────────────> Disbanded

Active ─────────────────────────────────────────────────────────┐
  │                                                              │
  │ pause()                                                      │
  ▼                                                              │
Paused ─────────────────────────────────────────────────────────┤
  │                                                              │
  │ resume()                                                     │
  └──────────────────────────────────────────────────────────> Active

Active ─────────────────────────────────────────────────────────┐
  │                                                              │
  │ critical_error()                                             │
  ▼                                                              │
Failed ─────────────────────────────────────────────────────────┤
  │                                                              │
  │ disband()                                                    │
  └──────────────────────────────────────────────────────────> Disbanded
```

### Task State Transitions
```
Pending ────────────────────────────────────────────────────────┐
  │                                                              │
  │ assign_to_worker()                                           │
  ▼                                                              │
Assigned ───────────────────────────────────────────────────────┤
  │                                                              │
  │ start_execution()                                            │
  ▼                                                              │
InProgress ─────────────────────────────────────────────────────┤
  │                                                              │
  │ complete_task()                                              │
  ▼                                                              │
Completed

InProgress ─────────────────────────────────────────────────────┐
  │                                                              │
  │ request_support()                                            │
  ▼                                                              │
WaitingSupport ─────────────────────────────────────────────────┤
  │                                                              │
  │ receive_support()                                            │
  └──────────────────────────────────────────────────────────> InProgress

InProgress ─────────────────────────────────────────────────────┐
  │                                                              │
  │ error_occurred()                                             │
  ▼                                                              │
Failed
```

## Error Scenarios

### Team Creation Errors
- **VALIDATION_ERROR**: 团队名称为空或超过 100 字符
- **MODEL_NOT_FOUND**: 指定的模型 ID 不存在
- **QUOTA_EXCEEDED**: 团队数量超过限制（50 个）
- **INVALID_CONFIG**: Worker 数量配置无效（min > max）

### Task Assignment Errors
- **NO_AVAILABLE_WORKER**: 没有空闲的 Worker
- **TASK_QUEUE_FULL**: 任务队列已满（1000 个）
- **INVALID_COMPLEXITY**: 任务复杂度评估失败
- **WORKER_OVERLOADED**: Worker 负载过高

### Worker Management Errors
- **WORKER_CREATION_FAILED**: Worker 创建失败
- **WORKER_NOT_FOUND**: Worker ID 不存在
- **WORKER_ALREADY_ASSIGNED**: Worker 已分配任务
- **MAX_WORKERS_REACHED**: 已达到最大 Worker 数量

### Support Request Errors
- **INVALID_SUPPORT_REQUEST**: 支援请求格式错误
- **LEADER_UNAVAILABLE**: Leader 不可用
- **SUPPORT_TIMEOUT**: 支援请求超时（5 分钟）

### Context Synchronization Errors
- **CONTEXT_TOO_LARGE**: 上下文超过大小限制
- **SYNC_FAILED**: 上下文同步失败
- **COMPRESSION_ERROR**: 结果摘要压缩失败

## Design References

- 后端架构：`docs/backend-architecture.md` § Agent Team 聚合设计
- API 规范：`docs/api-specification.md` § Agent Teams API
- 事件协议：`docs/event-protocol.md` § Agent Team 事件
