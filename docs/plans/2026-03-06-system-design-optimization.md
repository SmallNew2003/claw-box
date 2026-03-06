# Claw Box 系统设计优化方案

**日期**: 2026-03-06  
**状态**: 已批准  
**作者**: AI Agent

## 概述

本文档记录了 Claw Box 系统设计的全面优化方案，包括后端架构、API 规范、事件协议、安全设计、性能策略和规范文件扩充。

## 背景

通过系统设计评审，发现以下主要问题：

1. 后端架构设计缺失（Rust Core Service 无详细设计）
2. API 契约不完整（缺少错误处理、流式输出、事件格式）
3. 模块设计不具体（Agent Teams、记忆系统等缺少实现细节）
4. 安全实现不明确（IPC 安全、密钥存储等缺少具体方案）
5. 性能指标不可验证（缺少测量方法和具体优化策略）
6. 规范文件过于简单（openspec/specs/ 缺少细节）
7. 技术栈不一致（文档说 Tauri 优先，实际用 Electron）

## 优化目标

1. 建立完整的 DDD 后端架构设计
2. 定义规范的 RESTful API 和流式输出协议
3. 明确 SSE 事件协议和事件类型
4. 制定 Tauri 安全配置和密钥存储方案
5. 建立可测量的性能指标和优化策略
6. 扩充 openspec/specs/ 规范文件
7. 统一技术栈为 Tauri

## 设计方案

### 1. 后端架构（DDD）

采用领域驱动设计（DDD）分层架构：

```
backend/
├── src/
│   ├── interfaces/          # 接口层（HTTP API）
│   ├── application/         # 应用层（用例编排）
│   ├── domain/              # 领域层（业务逻辑）
│   │   ├── session/         # 会话聚合
│   │   ├── team/            # Agent Teams 聚合
│   │   ├── memory/          # 记忆聚合
│   │   ├── skill/           # 技能聚合
│   │   ├── mcp/             # MCP 聚合
│   │   └── shared/          # 共享内核
│   └── infrastructure/      # 基础设施层
│       ├── persistence/     # 持久化实现
│       ├── vector/          # 向量检索实现
│       ├── event_bus/       # 事件总线
│       └── external/        # 外部服务集成
```

**核心概念**：
- 聚合（Aggregate）：Session, AgentTeam, Memory, Skill, MCPConnection
- 值对象（Value Object）：SessionStatus, ModelId, Embedding, TeamMode
- 领域服务（Domain Service）：SessionContextService, TeamOrchestrationService, VectorSearchService
- 仓储（Repository）：接口在领域层，实现在基础设施层
- 领域事件（Domain Event）：SessionCreated, RunStarted, MemoryIndexed

**技术选型**：
- HTTP 框架：Axum
- 数据库访问：sqlx
- 事件总线：内存实现（MVP），可扩展为 Redis Pub/Sub
- 向量检索：sqlite-vss（默认）+ Qdrant（可选）

详细设计参见：`docs/backend-architecture.md`（待创建）

### 2. API 规范

**RESTful 端点**：
```
# 会话管理
GET    /api/sessions
POST   /api/sessions
GET    /api/sessions/:id
PATCH  /api/sessions/:id
DELETE /api/sessions/:id

# Agent Teams
GET    /api/teams
POST   /api/teams
POST   /api/runs
GET    /api/runs/:id

# 记忆系统
GET    /api/memories
POST   /api/memories
GET    /api/memories/search?q=...

# 技能管理
GET    /api/skills
POST   /api/skills
POST   /api/skills/:id/execute

# MCP 集成
GET    /api/mcp/connections
POST   /api/mcp/connections
POST   /api/mcp/tools/:toolId/call
```

**统一响应格式**：
```json
// 成功
{
  "data": { ... }
}

// 错误
{
  "error": {
    "code": "SESSION_NOT_FOUND",
    "message": "会话不存在",
    "details": { ... },
    "timestamp": "2026-03-06T10:30:00Z",
    "requestId": "req-12345"
  }
}
```

**HTTP 状态码**：
- 200 OK - 成功
- 201 Created - 创建成功
- 204 No Content - 成功但无返回
- 400 Bad Request - 参数错误
- 404 Not Found - 资源不存在
- 409 Conflict - 资源冲突
- 422 Unprocessable - 业务规则验证失败
- 500 Internal Error - 服务器错误

**流式输出（SSE）**：
```
POST /api/sessions/:id/messages/stream
POST /api/runs/stream

事件类型：
- message.start
- message.delta
- message.complete
- run.started
- step.started
- step.complete
```

详细设计参见：`docs/api-specification.md`（待创建）

### 3. 事件协议

**全局事件流**：
```
GET /api/events
Accept: text/event-stream
```

**事件类型**：
- 会话事件：session.created, session.status_changed, session.message_added
- Agent Teams 事件：run.started, run.completed, step.started, step.completed
- 记忆事件：memory.created, memory.indexed
- 技能事件：skill.installed, skill.execution_completed
- MCP 事件：mcp.connected, mcp.tool_called

**事件格式**：
```
id: evt-001
event: session.created
data: {"sessionId":"s-01","name":"...","timestamp":"2026-03-06T10:00:00Z"}
```

**连接管理**：
- Keep-Alive：每 15 秒发送心跳
- 超时：60 秒无数据自动断开
- 重连：客户端指数退避（1s, 2s, 4s, 8s, 最大 30s）

详细设计参见：`docs/event-protocol.md`（待创建）

### 4. 安全设计（Tauri）

**Tauri 安全配置**：
```json
{
  "tauri": {
    "allowlist": {
      "all": false,
      "http": {
        "scope": ["http://localhost:3000/*"]
      },
      "shell": { "all": false }
    },
    "security": {
      "csp": "default-src 'self'; connect-src 'self' http://localhost:3000"
    }
  }
}
```

**Command API**：
- 所有前后端通信通过 Tauri Command
- 输入验证和清理
- 权限检查

**密钥存储**：
- 使用系统密钥链（keyring crate）
- macOS: Keychain
- Windows: Credential Manager
- Linux: Secret Service API

**技能沙箱**：
- 权限声明和检查
- 资源限制（超时、内存）
- 隔离执行环境

详细设计参见：`docs/security-design.md`（待创建）

### 5. 性能策略

**性能指标**：
- 应用启动时间 < 3000ms
- 会话切换响应 < 200ms
- 消息渲染流畅（60fps）
- 前端内存占用 < 500MB

**前端优化**：
- 虚拟滚动（react-window）
- React.memo 优化重渲染
- Zustand 分片状态
- 防抖与节流
- 路由级代码分割

**后端优化**：
- 数据库索引和分页查询
- 连接池配置
- 缓存策略（moka）
- 异步处理耗时操作
- 向量检索优化

**打包优化**：
- Rust: LTO、strip、opt-level=3
- 前端: Terser 压缩、代码分割

详细设计参见：`docs/performance-strategy.md`（待创建）

### 6. 规范文件扩充

扩充 openspec/specs/ 下的规范文件，包含：

1. **功能需求**：详细的 Requirement + Scenario
2. **非功能需求**：Performance、Scalability、Reliability
3. **数据模型**：完整的 TypeScript 接口定义
4. **状态机**：关键实体的状态转换
5. **错误场景**：所有可能的错误情况
6. **设计文档引用**：链接到对应的技术设计文档

**Agent Teams 新增功能**：
- 动态任务分配（Agent Swarm）
- Worker 请求支援
- Worker 协商机制
- 动态缩容

示例场景：
```
场景：处理 100 张图片
1. Leader 评估任务 → 分配 5 个 Worker
2. Worker-3 发现超大图片 → 请求支援
3. Leader 响应 → 创建 Worker-6 并重新分配
4. 所有 Worker 完成 → 释放多余 Worker
```

详细规范参见：`openspec/specs/*/spec.md`（待更新）

## 技术栈统一

**变更**：
- 移除 Electron 依赖
- 统一使用 Tauri
- 更新 package.json
- 删除 electron.vite.config.ts 和 electron-builder.yml
- 重组目录结构（src/ → src-ui/）

**理由**：
- 符合 openspec/project.md 定义（Tauri 优先）
- 更小的打包体积
- 更好的安全性
- 原生 Rust 集成

## 文档结构

```
docs/
├── architecture.md              # 总览（保留，添加索引）
├── backend-architecture.md      # 后端 DDD 架构（新增）
├── api-specification.md         # API 规范（新增）
├── event-protocol.md            # 事件协议（新增）
├── security-design.md           # 安全设计（新增）
├── performance-strategy.md      # 性能策略（新增）
└── product-spec.md              # 产品规格（保留）

openspec/specs/
├── session-management/spec.md   # 扩充
├── agent-teams/spec.md          # 扩充（含动态任务分配）
├── memory-system/spec.md        # 扩充
├── skills-management/spec.md    # 扩充
└── mcp-integration/spec.md      # 扩充
```

## 实施计划

实施计划将通过 `writing-plans` skill 生成，包含：

1. 创建后端 DDD 架构文档
2. 创建 API 规范文档
3. 创建事件协议文档
4. 创建安全设计文档
5. 创建性能策略文档
6. 扩充 openspec/specs/ 规范文件
7. 更新 docs/architecture.md 添加文档索引
8. 创建 OpenSpec 提案（如需要）

## 验收标准

1. 所有新文档已创建并通过评审
2. 所有规范文件已扩充并包含完整的场景定义
3. 技术栈已统一为 Tauri
4. 文档之间的引用关系清晰
5. 符合 openspec/DOCUMENTATION.md 定义的职责划分

## 参考资料

- openspec/project.md - 项目上下文
- openspec/DOCUMENTATION.md - 文档组织规范
- openspec/AGENTS.md - AI Agent 工作指南
- docs/architecture.md - 现有架构设计
- docs/product-spec.md - 产品规格说明
