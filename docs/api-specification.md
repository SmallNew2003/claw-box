# API Specification - Core Service HTTP API

<!-- 版本: v1.0.0 -->
<!-- 最后更新: 2026-03-06 -->
<!-- 维护者: 开发团队 -->

## 概述

Claw Box Core Service 提供 RESTful HTTP API 和 SSE 事件流，前端通过这些接口与后端通信。

### 设计原则

- RESTful 风格：资源导向，HTTP 动词语义化
- JSON 格式：统一使用 JSON 序列化
- 流式输出：长时间运行任务使用 SSE
- 错误处理：统一错误响应格式
- 版本控制：URL 路径包含版本号

### 基础信息

- Base URL: `http://localhost:8080/api/v1`
- Content-Type: `application/json`
- 认证方式：Bearer Token（可选）
- 超时时间：30 秒（普通请求），无限制（SSE）

## 通用响应格式

### 成功响应

```json
{
  "data": { ... },
  "meta": {
    "timestamp": "2026-03-06T10:00:00Z",
    "request_id": "req_abc123"
  }
}
```

### 错误响应

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid session ID format",
    "details": {
      "field": "session_id",
      "reason": "Must be a valid UUID"
    }
  },
  "meta": {
    "timestamp": "2026-03-06T10:00:00Z",
    "request_id": "req_abc123"
  }
}
```

### 错误码

| 错误码 | HTTP 状态码 | 说明 |
|--------|------------|------|
| VALIDATION_ERROR | 400 | 请求参数验证失败 |
| UNAUTHORIZED | 401 | 未授权访问 |
| FORBIDDEN | 403 | 权限不足 |
| NOT_FOUND | 404 | 资源不存在 |
| CONFLICT | 409 | 资源冲突 |
| INTERNAL_ERROR | 500 | 服务器内部错误 |
| SERVICE_UNAVAILABLE | 503 | 服务不可用 |

## 会话管理 API

### 创建会话

```
POST /sessions
```

请求体：
```json
{
  "model_id": "claude-3-opus",
  "system_prompt": "You are a helpful assistant",
  "config": {
    "temperature": 0.7,
    "max_tokens": 4096
  }
}
```

响应：
```json
{
  "data": {
    "session_id": "sess_abc123",
    "model_id": "claude-3-opus",
    "status": "idle",
    "created_at": "2026-03-06T10:00:00Z"
  }
}
```

### 获取会话列表

```
GET /sessions?status=active&limit=20&offset=0
```

查询参数：
- `status`: 会话状态（idle, running, completed, failed）
- `limit`: 每页数量（默认 20，最大 100）
- `offset`: 偏移量（默认 0）

响应：
```json
{
  "data": {
    "sessions": [
      {
        "session_id": "sess_abc123",
        "model_id": "claude-3-opus",
        "status": "idle",
        "created_at": "2026-03-06T10:00:00Z",
        "updated_at": "2026-03-06T10:05:00Z"
      }
    ],
    "total": 42,
    "limit": 20,
    "offset": 0
  }
}
```

### 获取会话详情

```
GET /sessions/{session_id}
```

响应：
```json
{
  "data": {
    "session_id": "sess_abc123",
    "model_id": "claude-3-opus",
    "status": "idle",
    "system_prompt": "You are a helpful assistant",
    "config": {
      "temperature": 0.7,
      "max_tokens": 4096
    },
    "message_count": 10,
    "created_at": "2026-03-06T10:00:00Z",
    "updated_at": "2026-03-06T10:05:00Z"
  }
}
```

### 发送消息（流式）

```
POST /sessions/{session_id}/messages
```

请求体：
```json
{
  "content": "Hello, how are you?",
  "role": "user"
}
```

响应（SSE 流）：
```
event: message_start
data: {"message_id": "msg_abc123", "timestamp": "2026-03-06T10:00:00Z"}

event: content_delta
data: {"delta": "Hello", "index": 0}

event: content_delta
data: {"delta": "! I'm", "index": 1}

event: content_delta
data: {"delta": " doing well", "index": 2}

event: message_complete
data: {"message_id": "msg_abc123", "finish_reason": "stop", "usage": {"input_tokens": 10, "output_tokens": 15}}
```

### 获取消息历史

```
GET /sessions/{session_id}/messages?limit=50&before=msg_xyz
```

查询参数：
- `limit`: 每页数量（默认 50，最大 200）
- `before`: 游标（消息 ID）
- `after`: 游标（消息 ID）

响应：
```json
{
  "data": {
    "messages": [
      {
        "message_id": "msg_abc123",
        "role": "user",
        "content": "Hello",
        "timestamp": "2026-03-06T10:00:00Z"
      },
      {
        "message_id": "msg_def456",
        "role": "assistant",
        "content": "Hi there!",
        "timestamp": "2026-03-06T10:00:05Z"
      }
    ],
    "has_more": true,
    "next_cursor": "msg_ghi789"
  }
}
```

### 删除会话

```
DELETE /sessions/{session_id}
```

响应：
```json
{
  "data": {
    "session_id": "sess_abc123",
    "deleted_at": "2026-03-06T10:00:00Z"
  }
}
```

## Agent Teams API

### 创建团队

```
POST /agent-teams
```

请求体：
```json
{
  "name": "Research Team",
  "leader_config": {
    "model_id": "claude-3-opus",
    "system_prompt": "You are a research team leader"
  },
  "worker_config": {
    "model_id": "claude-3-sonnet",
    "min_workers": 1,
    "max_workers": 5
  }
}
```

响应：
```json
{
  "data": {
    "team_id": "team_abc123",
    "name": "Research Team",
    "status": "created",
    "leader_id": "agent_leader_123",
    "worker_count": 1,
    "created_at": "2026-03-06T10:00:00Z"
  }
}
```

### 启动团队任务（流式）

```
POST /agent-teams/{team_id}/tasks
```

请求体：
```json
{
  "task": "Research the latest AI trends",
  "context": {
    "sources": ["arxiv", "github"],
    "time_range": "last_month"
  }
}
```

响应（SSE 流）：
```
event: task_start
data: {"task_id": "task_abc123", "timestamp": "2026-03-06T10:00:00Z"}

event: worker_assigned
data: {"worker_id": "agent_worker_1", "subtask": "Search arxiv"}

event: worker_progress
data: {"worker_id": "agent_worker_1", "progress": 0.5, "message": "Found 10 papers"}

event: worker_completed
data: {"worker_id": "agent_worker_1", "result": "..."}

event: task_complete
data: {"task_id": "task_abc123", "result": "...", "duration_ms": 5000}
```

### 获取团队状态

```
GET /agent-teams/{team_id}
```

响应：
```json
{
  "data": {
    "team_id": "team_abc123",
    "name": "Research Team",
    "status": "active",
    "leader": {
      "agent_id": "agent_leader_123",
      "status": "running"
    },
    "workers": [
      {
        "agent_id": "agent_worker_1",
        "status": "idle",
        "tasks_completed": 5
      },
      {
        "agent_id": "agent_worker_2",
        "status": "running",
        "current_task": "task_abc123"
      }
    ],
    "created_at": "2026-03-06T10:00:00Z",
    "updated_at": "2026-03-06T10:05:00Z"
  }
}
```

### 解散团队

```
DELETE /agent-teams/{team_id}
```

响应：
```json
{
  "data": {
    "team_id": "team_abc123",
    "disbanded_at": "2026-03-06T10:00:00Z"
  }
}
```

## 记忆系统 API

### 创建记忆

```
POST /memories
```

请求体：
```json
{
  "content": "The user prefers dark mode",
  "metadata": {
    "source": "session_abc123",
    "category": "preference",
    "tags": ["ui", "theme"]
  }
}
```

响应：
```json
{
  "data": {
    "memory_id": "mem_abc123",
    "content": "The user prefers dark mode",
    "vector": [0.1, 0.2, ...],
    "metadata": {
      "source": "session_abc123",
      "category": "preference",
      "tags": ["ui", "theme"]
    },
    "created_at": "2026-03-06T10:00:00Z"
  }
}
```

### 搜索记忆

```
POST /memories/search
```

请求体：
```json
{
  "query": "user interface preferences",
  "limit": 10,
  "filters": {
    "category": "preference",
    "tags": ["ui"]
  }
}
```

响应：
```json
{
  "data": {
    "results": [
      {
        "memory_id": "mem_abc123",
        "content": "The user prefers dark mode",
        "score": 0.95,
        "metadata": {
          "source": "session_abc123",
          "category": "preference",
          "tags": ["ui", "theme"]
        },
        "created_at": "2026-03-06T10:00:00Z"
      }
    ],
    "total": 1
  }
}
```

### 删除记忆

```
DELETE /memories/{memory_id}
```

响应：
```json
{
  "data": {
    "memory_id": "mem_abc123",
    "deleted_at": "2026-03-06T10:00:00Z"
  }
}
```

## 技能管理 API

### 安装技能

```
POST /skills
```

请求体：
```json
{
  "name": "web-search",
  "version": "1.0.0",
  "source": "https://github.com/user/skill-web-search",
  "config": {
    "api_key": "sk_xxx"
  }
}
```

响应：
```json
{
  "data": {
    "skill_id": "skill_abc123",
    "name": "web-search",
    "version": "1.0.0",
    "status": "installed",
    "permissions": ["network", "read"],
    "installed_at": "2026-03-06T10:00:00Z"
  }
}
```

### 获取技能列表

```
GET /skills
```

响应：
```json
{
  "data": {
    "skills": [
      {
        "skill_id": "skill_abc123",
        "name": "web-search",
        "version": "1.0.0",
        "status": "installed",
        "permissions": ["network", "read"],
        "installed_at": "2026-03-06T10:00:00Z"
      }
    ],
    "total": 1
  }
}
```

### 执行技能

```
POST /skills/{skill_id}/execute
```

请求体：
```json
{
  "input": {
    "query": "latest AI news"
  }
}
```

响应：
```json
{
  "data": {
    "execution_id": "exec_abc123",
    "output": {
      "results": [...]
    },
    "duration_ms": 1500,
    "executed_at": "2026-03-06T10:00:00Z"
  }
}
```

### 卸载技能

```
DELETE /skills/{skill_id}
```

响应：
```json
{
  "data": {
    "skill_id": "skill_abc123",
    "uninstalled_at": "2026-03-06T10:00:00Z"
  }
}
```

## MCP 集成 API

### 连接 MCP 服务器

```
POST /mcp/connections
```

请求体：
```json
{
  "name": "GitHub MCP",
  "url": "http://localhost:3000",
  "config": {
    "api_token": "ghp_xxx"
  }
}
```

响应：
```json
{
  "data": {
    "connection_id": "mcp_abc123",
    "name": "GitHub MCP",
    "status": "connected",
    "tools": [
      {
        "tool_name": "search_repos",
        "description": "Search GitHub repositories"
      }
    ],
    "connected_at": "2026-03-06T10:00:00Z"
  }
}
```

### 获取连接列表

```
GET /mcp/connections
```

响应：
```json
{
  "data": {
    "connections": [
      {
        "connection_id": "mcp_abc123",
        "name": "GitHub MCP",
        "status": "connected",
        "tool_count": 5,
        "connected_at": "2026-03-06T10:00:00Z"
      }
    ],
    "total": 1
  }
}
```

### 调用 MCP 工具

```
POST /mcp/connections/{connection_id}/tools/{tool_name}/invoke
```

请求体：
```json
{
  "arguments": {
    "query": "rust web framework"
  }
}
```

响应：
```json
{
  "data": {
    "invocation_id": "inv_abc123",
    "result": {
      "repositories": [...]
    },
    "duration_ms": 800,
    "invoked_at": "2026-03-06T10:00:00Z"
  }
}
```

### 断开连接

```
DELETE /mcp/connections/{connection_id}
```

响应：
```json
{
  "data": {
    "connection_id": "mcp_abc123",
    "disconnected_at": "2026-03-06T10:00:00Z"
  }
}
```

## 模型管理 API

### 获取模型列表

```
GET /models
```

响应：
```json
{
  "data": {
    "models": [
      {
        "model_id": "claude-3-opus",
        "name": "Claude 3 Opus",
        "provider": "anthropic",
        "capabilities": ["chat", "vision"],
        "max_tokens": 200000
      },
      {
        "model_id": "gpt-4",
        "name": "GPT-4",
        "provider": "openai",
        "capabilities": ["chat"],
        "max_tokens": 128000
      }
    ],
    "total": 2
  }
}
```

### 配置模型

```
POST /models/{model_id}/config
```

请求体：
```json
{
  "api_key": "sk_xxx",
  "base_url": "https://api.anthropic.com",
  "default_params": {
    "temperature": 0.7,
    "max_tokens": 4096
  }
}
```

响应：
```json
{
  "data": {
    "model_id": "claude-3-opus",
    "status": "configured",
    "configured_at": "2026-03-06T10:00:00Z"
  }
}
```

### 测试模型连接

```
POST /models/{model_id}/test
```

响应：
```json
{
  "data": {
    "model_id": "claude-3-opus",
    "status": "available",
    "latency_ms": 150,
    "tested_at": "2026-03-06T10:00:00Z"
  }
}
```

## SSE 事件流

### 订阅会话事件

```
GET /sessions/{session_id}/events
```

事件类型：
- `session_started`: 会话开始
- `session_completed`: 会话完成
- `message_added`: 消息添加
- `message_updated`: 消息更新
- `error`: 错误发生

示例：
```
event: session_started
data: {"session_id": "sess_abc123", "timestamp": "2026-03-06T10:00:00Z"}

event: message_added
data: {"message_id": "msg_abc123", "role": "user", "content": "Hello"}

event: message_added
data: {"message_id": "msg_def456", "role": "assistant", "content": "Hi!"}

event: session_completed
data: {"session_id": "sess_abc123", "timestamp": "2026-03-06T10:05:00Z"}
```

### 订阅团队事件

```
GET /agent-teams/{team_id}/events
```

事件类型：
- `team_created`: 团队创建
- `worker_added`: Worker 添加
- `task_assigned`: 任务分配
- `task_completed`: 任务完成
- `team_disbanded`: 团队解散

示例：
```
event: team_created
data: {"team_id": "team_abc123", "timestamp": "2026-03-06T10:00:00Z"}

event: worker_added
data: {"team_id": "team_abc123", "worker_id": "agent_worker_1"}

event: task_assigned
data: {"team_id": "team_abc123", "task_id": "task_abc123", "worker_id": "agent_worker_1"}

event: task_completed
data: {"team_id": "team_abc123", "task_id": "task_abc123", "result": "..."}
```

### 订阅全局事件

```
GET /events
```

事件类型：
- 所有会话事件
- 所有团队事件
- 系统事件（启动、关闭、错误）

## 流式输出协议

### 消息流式输出

使用 SSE（Server-Sent Events）协议：

1. 客户端发起请求：`POST /sessions/{session_id}/messages`
2. 服务器返回 `Content-Type: text/event-stream`
3. 服务器推送事件流
4. 客户端接收并处理事件

事件格式：
```
event: <event_type>
data: <json_payload>

```

### 连接管理

- 心跳：每 30 秒发送 `event: ping`
- 重连：客户端断线后自动重连
- 超时：无活动 5 分钟后关闭连接

### 错误处理

流式输出中的错误：
```
event: error
data: {"code": "INTERNAL_ERROR", "message": "Model API error"}
```

## 批量操作

### 批量创建记忆

```
POST /memories/batch
```

请求体：
```json
{
  "memories": [
    {
      "content": "Memory 1",
      "metadata": {...}
    },
    {
      "content": "Memory 2",
      "metadata": {...}
    }
  ]
}
```

响应：
```json
{
  "data": {
    "created": [
      {"memory_id": "mem_abc123", "index": 0},
      {"memory_id": "mem_def456", "index": 1}
    ],
    "failed": []
  }
}
```

### 批量删除会话

```
DELETE /sessions/batch
```

请求体：
```json
{
  "session_ids": ["sess_abc123", "sess_def456"]
}
```

响应：
```json
{
  "data": {
    "deleted": ["sess_abc123", "sess_def456"],
    "failed": []
  }
}
```

## 分页与游标

### 基于偏移量的分页

```
GET /sessions?limit=20&offset=40
```

适用场景：
- 总数已知
- 数据相对静态
- 需要跳转到特定页

### 基于游标的分页

```
GET /sessions/{session_id}/messages?limit=50&before=msg_xyz
```

适用场景：
- 实时数据流
- 数据频繁变化
- 只需前后翻页

## 性能优化

### 缓存策略

- 模型列表：缓存 1 小时
- 会话详情：缓存 5 分钟
- 消息历史：缓存 1 分钟

### 压缩

- 支持 gzip 压缩
- 请求头：`Accept-Encoding: gzip`
- 响应头：`Content-Encoding: gzip`

### 限流

- 每个 IP：100 请求/分钟
- 每个会话：10 消息/分钟
- 超出限制：返回 429 Too Many Requests

## 安全考虑

### 认证

- Bearer Token：`Authorization: Bearer <token>`
- Token 有效期：30 天
- Token 刷新：`POST /auth/refresh`

### CORS

- 允许的源：配置文件指定
- 允许的方法：GET, POST, PUT, DELETE
- 允许的头：Content-Type, Authorization

### 输入验证

- 所有输入参数验证
- SQL 注入防护
- XSS 防护

## 参考文档

- 后端架构：`docs/backend-architecture.md`
- 事件协议：`docs/event-protocol.md`
- 安全设计：`docs/security-design.md`
- 规范文件：`openspec/specs/*/spec.md`
