# Agent Teams 生命周期与协作设计

## Context

Agent Teams 被定义为单 Agent 运行时之上的增强层，而不是项目第一阶段的基础设施。

在当前项目阶段，更合理的建设顺序是：

1. 单 Agent 运行时基础能力
2. 模型供应商与模型管理
3. `build/plan` 模式切换与思考级别
4. Skills 与 MCP 集成
5. Agent Teams 协作能力

本设计只覆盖第 5 步中的团队生命周期与协作机制，不覆盖长期记忆或知识库体系。

## Goals / Non-Goals

- Goals:
  - 定义团队生命周期状态机与约束
  - 定义团队内最小共享上下文
  - 定义消息总线与共享工作区的最小协作协议
  - 定义团队归档与资源清理行为
- Non-Goals:
  - 不实现长期记忆、项目知识库、全局知识库
  - 不实现 embedding 检索、知识评分和知识提升
  - 不实现模型管理、Skills、MCP 或思考级别配置
  - 不作为项目第一阶段交付目标

## Decisions

- Decision: 团队采用显式生命周期状态机
  - `draft -> ready -> running -> paused -> running -> completed/failed -> archived`
  - 只有 `draft` 和 `archived` 允许删除

- Decision: 成员之间不共享完整上下文
  - 每个成员维护自己的局部执行上下文
  - 共享信息只通过消息摘要和工作区产出物传递

- Decision: 使用消息总线 + 共享工作区作为最小协作面
  - 消息总线用于短消息、任务分派、进度同步、结果通知
  - 共享工作区用于保存结构化产出物和任务目标

- Decision: 团队归档时只处理团队自身运行资源
  - 清理工作区
  - 可选导出消息日志
  - 更新团队状态为 `archived`

- Decision: 长期知识管理单独规划
  - 团队运行产出的长期沉淀能力后续通过独立的 `memory-system` change 接入

- Decision: 客户端通信采用控制面/数据面分离
  - 控制面命令（生命周期控制、团队配置）走 Tauri Command
  - 数据面（团队状态、消息与工作区事件）走本地 HTTP + SSE
  - 双通道通过 `trace_id` 关联命令与事件

## Architecture

### 1. 团队状态机

```typescript
type TeamState =
  | 'draft'
  | 'ready'
  | 'running'
  | 'paused'
  | 'completed'
  | 'failed'
  | 'archived';

interface Team {
  id: string;
  name: string;
  projectId?: string;
  state: TeamState;
  mode: 'sequential' | 'parallel' | 'expert' | 'pipeline';
  roles: Role[];
  createdAt: number;
  startedAt?: number;
  completedAt?: number;
  archivedAt?: number;
}
```

### 2. 消息总线

```typescript
interface TeamMessage {
  id: string;
  type: 'task_assigned' | 'progress_update' | 'help_request' | 'result_ready';
  from: string;
  to?: string;
  teamId: string;
  timestamp: number;
  payload: {
    summary: string;
    details?: string;
    artifacts?: string[];
  };
}
```

### 3. 共享工作区

```typescript
interface SharedWorkspace {
  teamId: string;
  structure: {
    goal: string;
    constraints: string[];
    tasks: Record<string, TaskDefinition>;
    artifacts: Record<string, Artifact>;
  };
}
```

### 4. 归档流程

1. 校验团队状态必须为 `completed` 或 `failed`
2. 可选导出消息历史
3. 清理共享工作区与产出物
4. 更新团队状态为 `archived`

### 5. 通信通道

1. 控制面（Command）
   - `create_team`
   - `update_team_config`
   - `activate_team`
   - `pause_team`
   - `resume_team`
   - `archive_team`
2. 数据面（HTTP + SSE）
   - 团队详情查询
   - 团队消息历史查询
   - 团队事件实时订阅（SSE）

## Risks / Trade-offs

- 风险：如果在单 Agent 基础能力未稳定前提前实现，会重复定义运行态与事件流
  - Mitigation：将本提案定位为后置能力，不作为第一阶段实现项

- 风险：消息总线和工作区协议过早复杂化
  - Mitigation：只保留最小摘要消息和按需读取产出物，不引入额外知识层

- 风险：后续接入长期记忆时需要再次调整团队归档行为
  - Mitigation：明确长期知识沉淀由后续 change 处理，本提案只定义团队资源归档

## Migration Plan

当前阶段无需迁移。

后续实施时：

1. 先确认单 Agent 运行时已稳定
2. 再落团队模型、消息总线、共享工作区
3. 最后评估是否需要与长期记忆系统集成

## Open Questions

- 团队消息历史是永久保留，还是仅保留最近 N 条
- 工作区单团队的大小和产出物数量限制如何设定
- 团队运行失败后的恢复边界是否允许部分重试
