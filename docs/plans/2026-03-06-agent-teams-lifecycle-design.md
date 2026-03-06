# Agent Teams 生命周期与知识库设计

## 设计目标

设计 Agent Teams 的完整生命周期管理，包括团队创建、运行、归档，以及配套的分层知识库系统和成员交互机制。

## 核心问题

1. **团队生命周期**：团队实例从创建到归档的完整流程
2. **知识库作用域**：临时团队 vs 长期项目的知识管理
3. **成员交互**：如何在不共享完整上下文的情况下实现协作（避免 token 爆炸）

## 设计方案

### 1. 团队生命周期

团队实例的完整生命周期包含以下阶段：

```
创建 → 配置 → 激活 → 运行 → 暂停 → 恢复 → 停止 → 归档/删除
```

#### 1.1 状态机

```typescript
type TeamState = 
  | 'draft'      // 草稿：刚创建，未配置完成
  | 'ready'      // 就绪：配置完成，等待激活
  | 'running'    // 运行中：正在执行任务
  | 'paused'     // 暂停：临时挂起
  | 'completed'  // 完成：任务执行完毕
  | 'failed'     // 失败：执行出错
  | 'archived';  // 归档：已归档，只读

interface Team {
  id: string;
  name: string;
  projectId?: string;  // 可选关联项目
  state: TeamState;
  mode: 'sequential' | 'parallel' | 'expert' | 'pipeline';
  roles: Role[];
  createdAt: number;
  startedAt?: number;
  completedAt?: number;
  archivedAt?: number;
}
```

#### 1.2 状态转换规则

- `draft` → `ready`：配置完成（至少有 1 个角色）
- `ready` → `running`：用户激活或接收到任务
- `running` → `paused`：用户暂停或系统资源不足
- `paused` → `running`：用户恢复
- `running` → `completed`：所有任务完成
- `running` → `failed`：执行出错且无法恢复
- `completed/failed` → `archived`：用户归档或自动归档（7 天后）

#### 1.3 生命周期操作

```typescript
interface TeamLifecycleAPI {
  // 创建团队（draft 状态）
  create(config: TeamConfig): Promise<Team>;
  
  // 配置团队（draft → ready）
  configure(teamId: string, config: TeamConfig): Promise<Team>;
  
  // 激活团队（ready → running）
  activate(teamId: string): Promise<Team>;
  
  // 暂停团队（running → paused）
  pause(teamId: string): Promise<Team>;
  
  // 恢复团队（paused → running）
  resume(teamId: string): Promise<Team>;
  
  // 停止团队（running → completed/failed）
  stop(teamId: string, reason?: string): Promise<Team>;
  
  // 归档团队（completed/failed → archived）
  archive(teamId: string, options?: ArchiveOptions): Promise<void>;
  
  // 删除团队（仅 draft/archived 状态）
  delete(teamId: string): Promise<void>;
}
```

### 2. 分层知识库系统

#### 2.1 知识库层级

```
全局知识库（Global Memory）
  ↓ 继承
项目知识库（Project Memory）
  ↓ 临时引用
团队运行时上下文（Team Runtime Context）
```

#### 2.2 知识
```typescript
interface KnowledgeItem {
  id: string;
  content: string;
  summary: string;           // LLM 生成的摘要（< 200 tokens）
  embedding: number[];       // 向量 embedding
  scope: 'global' | 'project' | 'team';
  source: {
    teamId?: string;
    projectId?: string;
    agentId?: string;
    createdAt: number;
  };
  metadata: {
    importance: number;      // 0-1，LLM 评分
    accessCount: number;     // 访问次数
    lastAccessedAt: number;
    tags: string[];
  };
  lifecycle: {
    state: 'active' | 'cold' | 'archived' | 'deleted';
    nextTransitionAt?: number;
  };
}
```

#### 2.3 知识库生命周期策略

**全局知识库**：
- 生命周期：永久（用户可标记"重要"防止淘汰）
- 清理策略：容量上限（10GB）+ LRU（仅针对未标记"重要"的知识）
- 状态机：Active（永久）→ Cold（90 天无访问）→ Archived（180 天无访问）

**项目知识库**：
- 生命周期：项目存续期间 + 归档后 30 天
- 清理策略：
  - 项目归档时，提示用户：
    - 选项 A：删除所有项目知识
    - 选项 B：保留 30 天后删除
    - 选项 C：提升重要知识到全局
- 状态机：Active → Cold（30 天无访问）→ 项目归档时处理

**团队运行时上下文**：
- 生命周期：临时（团队运行期间 + 7 天）
- 清理策略：
  - 团队归档时，自动评估知识重要性（LLM 评分）
  - 高价值（importance > 0.8）→ 提升到项目/全局
  - 中等价值（0.5-0.8）→ 保留 7 天
  - 低价值（< 0.5）→ 立即删除
- 状态机：Active → 团队归档时立即处理

#### 2.4 知识重要性自动评估

使用 LLM 对知识进行评分（0-1）：

```typescript
interface ImportanceScore {
  score: number;  // 0-1
  reasoning: string;
  factors: {
    novelty: number;      // 新颖性（是否是新知识）
    reusability: number;  // 可复用性（是否适用于其他任务）
    specificity: number;  // 特异性（是否过于具体）
  };
}
```

评分规则：
- 通用经验、最佳实践 → 高分（提升到全局）
- 项目特定知识 → 中分（保留在项目）
- 临时日志、中间结果 → 低分（删除）

#### 2.5 知识库 API

```typescript
interface KnowledgeAPI {
  // 存储知识
  store(content: string, scope: 'global' | 'project' | 'team', metadata?: Partial<KnowledgeItem>): Promise<KnowledgeItem>;
  
  // 检索知识（向量搜索）
  search(query: string, scope?: 'global' | 'project' | 'team', limit?: number): Promise<KnowledgeItem[]>;
  
  // 列出知识
  list(filters?: KnowledgeFilters): Promise<KnowledgeItem[]>;
  
  // 更新知识
  update(id: string, data: Partial<KnowledgeItem>): Promise<KnowledgeItem>;
  
  // 删除知识
  delete(id: string): Promise<void>;
  
  // 提升知识层级（team → project → global）
  promote(id: string, targetScope: 'project' | 'global'): Promise<KnowledgeItem>;
  
  // 评估知识重要性
  evaluateImportance(id: string): Promise<ImportanceScore>;
  
  // 批量清理（按生命周期策略）
  cleanup(scope?: 'global' | 'project' | 'team'): Promise<{ deleted: number; archived: number }>;
}
```

### 3. 团队成员交互机制

#### 3.1 设计原则

- **不共享完整上下文**：每个成员维护独立的完整上下文（避免 token 爆炸）
- **结构化消息传递**：通过消息总线传递关键信息（< 200 tokens/消息）
- **共享工作区**：产出物写入共享工作区，按需读取

#### 3.2 消息总线

```typescript
interface TeamMessage {
  id: string;
  type: 'task_assigned' | 'progress_update' | 'help_request' | 'result_ready' | 'knowledge_shared';
  from: string;  // agent id
  to?: string;   // 可选，点对点消息（默认广播）
  teamId: string;
  timestamp: number;
  payload: {
    taskId?: string;
    summary: string;      // 简短摘要（< 100 tokens）
    details?: string;     // 详细内容（按需加载）
    artifacts?: string[]; // 产出物引用（文件路径、知识库 ID）
  };
}

interface MessageBus {
  // 发送消息
  send(message: Omit<TeamMessage, 'id' | 'timestamp'>): Promise<TeamMessage>;
  
  // 订阅消息（按类型）
  subscribe(teamId: string, agentId: string, types?: TeamMessage['type'][]): AsyncIterator<TeamMessage>;
  
  // 获取历史消息
  getHistory(teamId: string, filters?: MessageFilters): Promise<TeamMessage[]>;
}
```

#### 3.3 共享工作区

```typescript
interface SharedWorkspace {
  teamId: string;
  structure: {
    goal: string;           // 任务目标（< 100 tokens）
    constraints: string[];  // 约束条件
    tasks: Record<string, TaskDefinition>;  // 任务定义
    artifacts: Record<string, Artifact>;    // 产出物
  };
}

interface WorkspaceAPI {
  // 创建工作区
  create(teamId: string, goal: string, constraints?: string[]): Promise<SharedWorkspace>;
  
  // 读取工作区
  read(teamId: string): Promise<SharedWorkspace>;
  
  // 写入产出物
  writeArtifact(teamId: string, artifact: Artifact): Promise<void>;
  
  // 读取产出物
  readArtifact(teamId: string, artifactId: string): Promise<Artifact>;
  
  // 列出产出物
  listArtifacts(teamId: string): Promise<Artifact[]>;
}
```

#### 3.4 交互流程示例

```
1. Leader 创建团队，写入 workspace/goal（任务目标，100 tokens）
2. Leader 发送消息 "task_assigned" 给 Worker-1（任务描述，50 tokens）
3. Worker-1 读取 workspace/goal + 自己的任务
4. Worker-1 执行，写入 workspace/artifacts/result-1.json
5. Worker-1 发送消息 "result_ready"（摘要，50 tokens）
6. Leader 读取 workspace/artifacts/result-1.json（按需）
7. Leader 汇总所有结果，生成最终输出
```

#### 3.5 Token 消耗估算

对于 5 个 Worker、10 条消息的团队：
- 共享上下文：500 tokens（团队级 goal + constraints）
- 消息总线：10 × 100 = 1000 tokens
- 每个 Worker 独立上下文：5000 tokens（不共享，并行执行）
- 总消耗：500 + 1000 = 1500 tokens（共享部分）
- 远低于全量共享（5 × 5000 = 25000 tokens 全部累加）

### 4. 团队归档流程

#### 4.1 归档触发条件

- 用户手动归档
- 团队完成/失败后 7 天自动归档
- 系统资源清理时批量归档

#### 4.2 归档操作

```typescript
interface ArchiveOptions {
  deleteWorkspace?: boolean;      // 是否删除工作区（默认 true）
  promoteKnowledge?: boolean;     // 是否提升重要知识（默认 true）
  exportLogs?: boolean;           // 是否导出日志（默认 false）
}

async function archiveTeam(teamId: string, options: ArchiveOptions): Promise<void> {
  // 1. 评估团队运行时知识的重要性
  const knowledge = await knowledgeAPI.list({ scope: 'team', teamId });
  for (const item of knowledge) {
    const score = await knowledgeAPI.evaluateImportance(item.id);
    if (score.score > 0.8) {
      // 高价值知识提升到项目/全局
      const targetScope = item.source.projectId ? 'project' : 'global';
      await knowledgeAPI.promote(item.id, targetScope);
    } else if (score.score < 0.5) {
      // 低价值知识立即删除
      await knowledgeAPI.delete(item.id);
    }
    // 中等价值知识保留 7 天（由生命周期策略自动处理）
  }
  
  // 2. 归档消息历史（可选）
  if (options.exportLogs) {
    const messages = await messageBus.getHistory(teamId);
    await exportToFile(`logs/team-${teamId}.json`, messages);
  }
  
  // 3. 清理工作区（可选）
  if (options.deleteWorkspace) {
    await workspaceAPI.delete(teamId);
  }
  
  // 4. 更新团队状态
  await teamAPI.updateState(teamId, 'archived');
}
```

## 数据库设计

### 4.1 团队表

```sql
CREATE TABLE teams (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  project_id TEXT,
  state TEXT NOT NULL,
  mode TEXT NOT NULL,
  config JSON NOT NULL,
  created_at INTEGER NOT NULL,
  started_at INTEGER,
  completed_at INTEGER,
  archived_at INTEGER,
  FOREIGN KEY (project_id) REFERENCES projects(id)
);

CREATE INDEX idx_teams_state ON teams(state);
CREATE INDEX idx_teams_project_id ON teams(project_id);
```

### 4.2 知识库表

```sql
CREATE TABLE knowledge_items (
  id TEXT PRIMARY KEY,
  content TEXT NOT NULL,
  summary TEXT NOT NULL,
  embedding BLOB NOT NULL,
  scope TEXT NOT NULL,
  team_id TEXT,
  project_id TEXT,
  agent_id TEXT,
  importance REAL NOT NULL DEFAULT 0.5,
  access_count INTEGER NOT NULL DEFAULT 0,
  last_accessed_at INTEGER NOT NULL,
  state TEXT NOT NULL DEFAULT 'active',
  next_transition_at INTEGER,
  created_at INTEGER NOT NULL,
  tags JSON,
  FOREIGN KEY (team_id) REFERENCES teams(id),
  FOREIGN KEY (project_id) REFERENCES projects(id)
);

CREATE INDEX idx_knowledge_scope ON knowledge_items(scope);
CREATE INDEX idx_knowledge_team_id ON knowledge_items(team_id);
CREATE INDEX idx_knowledge_project_id ON knowledge_items(project_id);
CREATE INDEX idx_knowledge_state ON knowledge_items(state);
CREATE INDEX idx_knowledge_importance ON knowledge_items(importance);
```

### 4.3 消息表

```sql
CREATE TABLE team_messages (
  id TEXT PRIMARY KEY,
  team_id TEXT NOT NULL,
  type TEXT NOT NULL,
  from_agent TEXT NOT NULL,
  to_agent TEXT,
  payload JSON NOT NULL,
  timestamp INTEGER NOT NULL,
  FOREIGN KEY (team_id) REFERENCES teams(id)
);

CREATE INDEX idx_messages_team_id ON team_messages(team_id);
CREATE INDEX idx_messages_timestamp ON team_messages(timestamp);
```

### 4.4 工作区表

```sql
CREATE TABLE workspaces (
  team_id TEXT PRIMARY KEY,
  goal TEXT NOT NULL,
  constraints JSON,
  created_at INTEGER NOT NULL,
  FOREIGN KEY (team_id) REFERENCES teams(id)
);

CREATE TABLE workspace_artifacts (
  id TEXT PRIMARY KEY,
  team_id TEXT NOT NULL,
  name TEXT NOT NULL,
  type TEXT NOT NULL,
  content TEXT NOT NULL,
  created_at INTEGER NOT NULL,
  FOREIGN KEY (team_id) REFERENCES workspaces(team_id)
);

CREATE INDEX idx_artifacts_team_id ON workspace_artifacts(team_id);
```

## 非功能需求

### 5.1 性能指标

- 团队创建响应时间 < 200ms
- 消息传递延迟 < 100ms
- 知识检索响应时间 < 500ms（向量搜索）
- 归档操作时间 < 5s（单个团队）

### 5.2 容量规划

- 全局知识库：10GB 上限（约 500 万条知识）
- 项目知识库：每个项目 1GB 上限
- 团队运行时：每个团队 100MB 上限
- 消息历史：每个团队保留最近 1000 条消息

### 5.3 可靠性

- 团队状态变更需要事务保证
- 知识库写入需要持久化确认
- 消息传递至少一次送达（at-least-once）

## 实施优先级

### P0（核心功能）
1. 团队生命周期状态机
2. 基础知识库（全局 + 项目）
3. 消息总线
4. 共享工作区

### P1（增强功能）
1. 知识重要性自动评估
2. 知识库生命周期管理
3. 团队自动归档

### P2（优化功能）
1. 知识库容量管理
2. 消息历史压缩
3. 工作区快照

## 开放问题

1. **项目的定义**：项目是否需要独立的生命周期管理？还是只作为命名空间？
2. **知识评分模型**：是否需要训练专门的模型，还是使用通用 LLM？
3. **消息持久化**：消息历史是否需要永久保留？还是定期清理？
4. **工作区大小限制**：单个工作区的产出物数量和大小限制？

## 参考资料

- `openspec/project.md` - 项目上下文
- `docs/architecture.md` - 系统架构
- `openspec/specs/agent-teams/spec.md` - Agent Teams 规范
- `openspec/specs/memory-system/spec.md` - 记忆系统规范
