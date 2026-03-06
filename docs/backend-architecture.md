# Backend Architecture - Rust Core Service

<!-- 版本: v1.0.0 -->
<!-- 最后更新: 2026-03-06 -->
<!-- 维护者: 开发团队 -->

## 概述

Claw Box 后端采用 Rust 实现的 Core Service，提供统一的 HTTP/JSON API 和 SSE 事件流。架构遵循领域驱动设计（DDD）原则，清晰划分业务领域和技术边界。

### 设计目标

- 单机一体化部署（单二进制 + 内嵌前端）
- 模块化能力（会话、Agent Teams、记忆、技能、MCP）
- 高性能本地存储（SQLite 默认，支持 PostgreSQL 扩展）
- 实时事件流（SSE/WebSocket）
- 可扩展的向量检索（本地或外部向量数据库）

### 技术栈

- Rust 1.75+
- Axum（HTTP 框架）
- SQLx（数据库访问）
- Tokio（异步运行时）
- Serde（序列化）
- Qdrant Client（可选，向量检索）

## DDD 分层架构

### 1. 表现层（Presentation Layer）

负责 HTTP API 和事件流的暴露。

```
src/
├── api/
│   ├── routes/
│   │   ├── sessions.rs      # 会话管理端点
│   │   ├── agents.rs         # Agent Teams 端点
│   │   ├── memory.rs         # 记忆系统端点
│   │   ├── skills.rs         # 技能管理端点
│   │   ├── mcp.rs            # MCP 集成端点
│   │   └── models.rs         # 模型管理端点
│   ├── handlers/             # 请求处理器
│   ├── middleware/           # 中间件（认证、日志）
│   └── dto/                  # 数据传输对象
```

职责：
- 路由定义与请求验证
- DTO 与领域对象转换
- 错误处理与响应格式化
- SSE 事件流管理

### 2. 应用层（Application Layer）

编排业务流程，协调领域服务。

```
src/
├── application/
│   ├── services/
│   │   ├── session_service.rs    # 会话应用服务
│   │   ├── agent_service.rs      # Agent Teams 应用服务
│   │   ├── memory_service.rs     # 记忆应用服务
│   │   ├── skill_service.rs      # 技能应用服务
│   │   ├── mcp_service.rs        # MCP 应用服务
│   │   └── model_service.rs      # 模型应用服务
│   ├── commands/                  # 命令对象
│   ├── queries/                   # 查询对象
│   └── events/                    # 应用事件
```

职责：
- 用例编排（创建会话、启动 Agent Team、检索记忆）
- 事务管理
- 事件发布
- 权限检查

### 3. 领域层（Domain Layer）

核心业务逻辑和领域模型。

```
src/
├── domain/
│   ├── session/
│   │   ├── mod.rs
│   │   ├── aggregate.rs      # 会话聚合根
│   │   ├── entities.rs       # 消息、运行态实体
│   │   ├── value_objects.rs  # 会话 ID、状态值对象
│   │   ├── repository.rs     # 仓储接口
│   │   └── events.rs         # 领域事件
│   ├── agent/
│   │   ├── mod.rs
│   │   ├── aggregate.rs      # Agent Team 聚合根
│   │   ├── entities.rs       # Leader、Worker 实体
│   │   ├── value_objects.rs  # 角色、任务值对象
│   │   ├── repository.rs
│   │   └── events.rs
│   ├── memory/
│   │   ├── mod.rs
│   │   ├── aggregate.rs      # 记忆聚合根
│   │   ├── entities.rs       # 记忆条目实体
│   │   ├── value_objects.rs  # 向量、元数据值对象
│   │   ├── repository.rs
│   │   └── events.rs
│   ├── skill/
│   │   ├── mod.rs
│   │   ├── aggregate.rs      # 技能聚合根
│   │   ├── entities.rs       # 技能配置实体
│   │   ├── value_objects.rs  # 权限、版本值对象
│   │   ├── repository.rs
│   │   └── events.rs
│   ├── mcp/
│   │   ├── mod.rs
│   │   ├── aggregate.rs      # MCP 连接聚合根
│   │   ├── entities.rs       # 工具、服务器实体
│   │   ├── value_objects.rs  # 连接状态值对象
│   │   ├── repository.rs
│   │   └── events.rs
│   └── model/
│       ├── mod.rs
│       ├── aggregate.rs      # 模型配置聚合根
│       ├── entities.rs       # 模型实例实体
│       ├── value_objects.rs  # 模型参数值对象
│       ├── repository.rs
│       └── events.rs
```

职责：
- 业务规则验证
- 状态转换
- 领域事件发布
- 聚合根不变性维护

### 4. 基础设施层（Infrastructure Layer）

技术实现和外部依赖。

```
src/
├── infrastructure/
│   ├── persistence/
│   │   ├── sqlite/           # SQLite 实现
│   │   │   ├── session_repo.rs
│   │   │   ├── agent_repo.rs
│   │   │   ├── memory_repo.rs
│   │   │   ├── skill_repo.rs
│   │   │   ├── mcp_repo.rs
│   │   │   └── model_repo.rs
│   │   ├── postgres/         # PostgreSQL 实现（可选）
│   │   └── migrations/       # 数据库迁移
│   ├── vector/
│   │   ├── local.rs          # 本地向量检索
│   │   └── qdrant.rs         # Qdrant 客户端
│   ├── cache/
│   │   ├── memory.rs         # 内存缓存
│   │   └── redis.rs          # Redis 客户端（可选）
│   ├── http_client/          # 外部 API 调用
│   ├── event_bus/            # 事件总线实现
│   └── config/               # 配置加载
```

职责：
- 数据持久化
- 向量检索
- 缓存管理
- 外部服务调用
- 配置管理

## 核心聚合设计

### 1. 会话聚合（Session Aggregate）

聚合根：Session
实体：Message、RunState
值对象：SessionId、SessionStatus

不变性规则：
- 会话必须有唯一 ID
- 消息按时间顺序追加
- 运行态只能从 Idle → Running → Completed/Failed

状态转换：
```
Idle → Running → Completed
              → Failed
              → Cancelled
```

### 2. Agent Team 聚合（AgentTeam Aggregate）

聚合根：AgentTeam
实体：Leader、Worker、Task
值对象：TeamId、Role、TaskStatus

不变性规则：
- 团队必须有一个 Leader
- Worker 数量动态调整（1-10）
- 任务只能分配给空闲 Worker

状态转换：
```
Created → Active → Completed
               → Failed
               → Disbanded
```

### 3. 记忆聚合（Memory Aggregate）

聚合根：MemoryCollection
实体：MemoryEntry
值对象：MemoryId、Vector、Metadata

不变性规则：
- 记忆条目必须有向量表示
- 元数据包含时间戳和来源
- 删除操作软删除（标记为已删除）

### 4. 技能聚合（Skill Aggregate）

聚合根：Skill
实体：SkillConfig、Permission
值对象：SkillId、Version、PermissionLevel

不变性规则：
- 技能必须有唯一标识符
- 权限级别不可降级
- 配置变更需要版本号递增

### 5. MCP 连接聚合（MCPConnection Aggregate）

聚合根：MCPConnection
实体：MCPServer、Tool
值对象：ConnectionId、ConnectionStatus

不变性规则：
- 连接必须有唯一 ID
- 工具索引在连接建立后更新
- 断开连接后工具不可用

状态转换：
```
Disconnected → Connecting → Connected
                         → Failed
Connected → Disconnected
```

### 6. 模型聚合（Model Aggregate）

聚合根：ModelConfig
实体：ModelInstance
值对象：ModelId、ModelParams

不变性规则：
- 模型配置必须有唯一 ID
- 参数变更需要验证
- 切换模型需要保存上下文

## 仓储接口设计

### SessionRepository

```rust
#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn save(&self, session: &Session) -> Result<()>;
    async fn find_by_id(&self, id: &SessionId) -> Result<Option<Session>>;
    async fn find_all(&self) -> Result<Vec<Session>>;
    async fn delete(&self, id: &SessionId) -> Result<()>;
    async fn find_by_status(&self, status: SessionStatus) -> Result<Vec<Session>>;
}
```

### AgentTeamRepository

```rust
#[async_trait]
pub trait AgentTeamRepository: Send + Sync {
    async fn save(&self, team: &AgentTeam) -> Result<()>;
    async fn find_by_id(&self, id: &TeamId) -> Result<Option<AgentTeam>>;
    async fn find_active_teams(&self) -> Result<Vec<AgentTeam>>;
    async fn delete(&self, id: &TeamId) -> Result<()>;
}
```

### MemoryRepository

```rust
#[async_trait]
pub trait MemoryRepository: Send + Sync {
    async fn save(&self, memory: &MemoryEntry) -> Result<()>;
    async fn find_by_id(&self, id: &MemoryId) -> Result<Option<MemoryEntry>>;
    async fn search_by_vector(&self, vector: &Vector, limit: usize) -> Result<Vec<MemoryEntry>>;
    async fn delete(&self, id: &MemoryId) -> Result<()>;
}
```

## 领域事件设计

### 事件类型

```rust
pub enum DomainEvent {
    // 会话事件
    SessionCreated { session_id: SessionId, timestamp: DateTime<Utc> },
    SessionStarted { session_id: SessionId, timestamp: DateTime<Utc> },
    SessionCompleted { session_id: SessionId, timestamp: DateTime<Utc> },
    MessageAdded { session_id: SessionId, message_id: MessageId, timestamp: DateTime<Utc> },
    
    // Agent Team 事件
    TeamCreated { team_id: TeamId, timestamp: DateTime<Utc> },
    WorkerAdded { team_id: TeamId, worker_id: WorkerId, timestamp: DateTime<Utc> },
    TaskAssigned { team_id: TeamId, task_id: TaskId, worker_id: WorkerId, timestamp: DateTime<Utc> },
    TaskCompleted { team_id: TeamId, task_id: TaskId, timestamp: DateTime<Utc> },
    
    // 记忆事件
    MemoryCreated { memory_id: MemoryId, timestamp: DateTime<Utc> },
    MemoryDeleted { memory_id: MemoryId, timestamp: DateTime<Utc> },
    
    // 技能事件
    SkillInstalled { skill_id: SkillId, version: Version, timestamp: DateTime<Utc> },
    SkillExecuted { skill_id: SkillId, timestamp: DateTime<Utc> },
    
    // MCP 事件
    MCPConnected { connection_id: ConnectionId, timestamp: DateTime<Utc> },
    MCPDisconnected { connection_id: ConnectionId, timestamp: DateTime<Utc> },
    ToolInvoked { connection_id: ConnectionId, tool_name: String, timestamp: DateTime<Utc> },
    
    // 模型事件
    ModelSwitched { from: ModelId, to: ModelId, timestamp: DateTime<Utc> },
}
```

### 事件总线

```rust
#[async_trait]
pub trait EventBus: Send + Sync {
    async fn publish(&self, event: DomainEvent) -> Result<()>;
    async fn subscribe<H: EventHandler>(&self, handler: H) -> Result<()>;
}
```

## 技术选型

### 数据库

默认：SQLite
- 单机部署，零配置
- 支持 JSON 字段（存储元数据）
- 全文检索（FTS5）

可选：PostgreSQL
- 多实例部署
- pgvector 扩展（向量检索）
- 更强的并发能力

### 向量检索

默认：本地实现
- 基于 HNSW 算法
- 内存索引 + 磁盘持久化
- 适合小规模数据（< 100K 条目）

可选：Qdrant
- 专业向量数据库
- 支持大规模数据
- 分布式部署

### 缓存

默认：内存缓存
- LRU 策略
- 会话状态缓存
- 模型配置缓存

可选：Redis
- 分布式缓存
- 持久化支持
- Pub/Sub 事件通知

### HTTP 框架

Axum
- 基于 Tokio
- 类型安全的路由
- 中间件支持
- SSE 支持

### 序列化

Serde
- JSON 序列化/反序列化
- 自定义序列化器
- 性能优化

## 部署架构

### 单机一体化部署

```
┌─────────────────────────────────────┐
│         Claw Box Binary             │
│                                     │
│  ┌──────────────┐  ┌─────────────┐ │
│  │  Frontend    │  │  Core       │ │
│  │  (Embedded)  │  │  Service    │ │
│  │              │  │  (Rust)     │ │
│  └──────────────┘  └─────────────┘ │
│         │                 │         │
│         └────── HTTP ─────┘         │
│                                     │
│  ┌──────────────┐  ┌─────────────┐ │
│  │   SQLite     │  │   Local     │ │
│  │   Database   │  │   Vector    │ │
│  └──────────────┘  └─────────────┘ │
└─────────────────────────────────────┘
```

### 分离部署（可选）

```
┌──────────────┐         ┌─────────────────┐
│   Frontend   │         │  Core Service   │
│   (Tauri)    │ ─HTTP─> │  (Rust)         │
└──────────────┘         └─────────────────┘
                                │
                    ┌───────────┴───────────┐
                    │                       │
            ┌───────▼──────┐       ┌───────▼──────┐
            │  PostgreSQL  │       │   Qdrant     │
            │  + pgvector  │       │   (Vector)   │
            └──────────────┘       └──────────────┘
```

## 性能考虑

### 数据库优化

- 索引策略：会话 ID、时间戳、状态字段
- 连接池：最大 10 个连接
- 批量操作：消息批量插入
- 事务隔离：Read Committed

### 向量检索优化

- 索引构建：异步后台任务
- 缓存策略：热点向量缓存
- 批量查询：合并相似查询
- 分片策略：按时间分片

### 内存管理

- 会话缓存：最多 100 个活跃会话
- 消息缓存：每个会话最多 1000 条消息
- 向量缓存：最多 10K 个向量
- 定期清理：每小时清理过期缓存

### 并发控制

- 异步 I/O：Tokio 运行时
- 任务队列：有界队列（1000 任务）
- 限流：每秒最多 100 个请求
- 超时控制：API 超时 30 秒

## 安全考虑

### 数据加密

- API Token：AES-256 加密存储
- 敏感配置：环境变量或密钥文件
- 数据库：SQLite 加密扩展（可选）

### 权限控制

- 技能权限：读/写/执行三级权限
- MCP 工具：白名单机制
- 配置访问：只读/读写权限

### 审计日志

- 操作日志：所有写操作记录
- 错误日志：异常堆栈记录
- 访问日志：API 调用记录

## 扩展性设计

### 插件机制

- 技能插件：动态加载
- MCP 服务器：标准协议
- 自定义存储：仓储接口

### 配置分层

- 默认配置：代码内置
- 本地配置：`~/.claw-box/config.toml`
- 工作区配置：`.claw-box/config.toml`
- 运行时配置：环境变量

### 监控与诊断

- 健康检查：`/health` 端点
- 指标暴露：Prometheus 格式
- 日志输出：结构化日志（JSON）
- 性能分析：火焰图支持

## 参考文档

- API 规范：`docs/api-specification.md`
- 事件协议：`docs/event-protocol.md`
- 安全设计：`docs/security-design.md`
- 性能策略：`docs/performance-strategy.md`
- 规范文件：`openspec/specs/*/spec.md`
