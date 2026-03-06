# Event Protocol - SSE Event Streaming

<!-- 版本: v1.0.0 -->
<!-- 最后更新: 2026-03-06 -->
<!-- 维护者: 开发团队 -->

## 概述

Claw Box 使用 Server-Sent Events (SSE) 协议实现实时事件流，前端通过订阅事件流获取后端状态变化和运行日志。

### 设计目标

- 实时推送：后端状态变化立即推送到前端
- 单向通信：服务器到客户端的单向数据流
- 自动重连：客户端断线后自动重连
- 事件类型化：每个事件有明确的类型和数据结构
- 低延迟：事件推送延迟 < 100ms

### 技术选型

- 协议：SSE (Server-Sent Events)
- 传输：HTTP/1.1 长连接
- 格式：JSON 数据负载
- 编码：UTF-8

## SSE 协议规范

### 连接建立

客户端发起 GET 请求：
```
GET /api/v1/events HTTP/1.1
Host: localhost:8080
Accept: text/event-stream
Cache-Control: no-cache
```

服务器响应：
```
HTTP/1.1 200 OK
Content-Type: text/event-stream
Cache-Control: no-cache
Connection: keep-alive
```

### 事件格式

标准 SSE 事件格式：
```
event: <event_type>
data: <json_payload>
id: <event_id>

```

示例：
```
event: session_started
data: {"session_id": "sess_abc123", "timestamp": "2026-03-06T10:00:00Z"}
id: evt_001

event: message_added
data: {"message_id": "msg_abc123", "role": "user", "content": "Hello"}
id: evt_002

```

### 心跳机制

服务器每 30 秒发送心跳：
```
event: ping
data: {"timestamp": "2026-03-06T10:00:00Z"}

```

客户端收到心跳后重置超时计时器。

### 重连机制

客户端断线后自动重连：
1. 等待 1 秒后重连
2. 重连时携带 `Last-Event-ID` 头
3. 服务器从该 ID 后继续推送事件
4. 最多重试 5 次，每次等待时间翻倍

重连请求：
```
GET /api/v1/events HTTP/1.1
Host: localhost:8080
Accept: text/event-stream
Last-Event-ID: evt_042
```

## 事件类型定义

### 会话事件

#### session_created
会话创建事件

数据结构：
```typescript
{
  session_id: string;
  model_id: string;
  timestamp: string; // ISO 8601
}
```

示例：
```
event: session_created
data: {"session_id": "sess_abc123", "model_id": "claude-3-opus", "timestamp": "2026-03-06T10:00:00Z"}
```

#### session_started
会话开始运行

数据结构：
```typescript
{
  session_id: string;
  timestamp: string;
}
```

#### session_completed
会话完成

数据结构：
```typescript
{
  session_id: string;
  timestamp: string;
  duration_ms: number;
}
```

#### session_failed
会话失败

数据结构：
```typescript
{
  session_id: string;
  error: {
    code: string;
    message: string;
  };
  timestamp: string;
}
```

#### message_added
消息添加

数据结构：
```typescript
{
  session_id: string;
  message_id: string;
  role: "user" | "assistant" | "system";
  content: string;
  timestamp: string;
}
```

#### message_updated
消息更新（流式输出）

数据结构：
```typescript
{
  session_id: string;
  message_id: string;
  delta: string; // 增量内容
  index: number; // 片段索引
  timestamp: string;
}
```

### Agent Team 事件

#### team_created
团队创建

数据结构：
```typescript
{
  team_id: string;
  name: string;
  leader_id: string;
  timestamp: string;
}
```

#### team_activated
团队激活

数据结构：
```typescript
{
  team_id: string;
  timestamp: string;
}
```

#### worker_added
Worker 添加

数据结构：
```typescript
{
  team_id: string;
  worker_id: string;
  timestamp: string;
}
```

#### worker_removed
Worker 移除

数据结构：
```typescript
{
  team_id: string;
  worker_id: string;
  reason: "completed" | "failed" | "idle";
  timestamp: string;
}
```

#### task_assigned
任务分配

数据结构：
```typescript
{
  team_id: string;
  task_id: string;
  worker_id: string;
  task_description: string;
  timestamp: string;
}
```

#### task_progress
任务进度更新

数据结构：
```typescript
{
  team_id: string;
  task_id: string;
  worker_id: string;
  progress: number; // 0.0 - 1.0
  message: string;
  timestamp: string;
}
```

#### task_completed
任务完成

数据结构：
```typescript
{
  team_id: string;
  task_id: string;
  worker_id: string;
  result: any;
  duration_ms: number;
  timestamp: string;
}
```

#### task_failed
任务失败

数据结构：
```typescript
{
  team_id: string;
  task_id: string;
  worker_id: string;
  error: {
    code: string;
    message: string;
  };
  timestamp: string;
}
```

#### worker_request_support
Worker 请求支援

数据结构：
```typescript
{
  team_id: string;
  worker_id: string;
  task_id: string;
  reason: string;
  timestamp: string;
}
```

#### leader_response
Leader 响应

数据结构：
```typescript
{
  team_id: string;
  worker_id: string;
  task_id: string;
  action: "assign_helper" | "provide_context" | "reassign";
  details: any;
  timestamp: string;
}
```

#### team_disbanded
团队解散

数据结构：
```typescript
{
  team_id: string;
  reason: "completed" | "failed" | "manual";
  timestamp: string;
}
```

### 记忆系统事件

#### memory_created
记忆创建

数据结构：
```typescript
{
  memory_id: string;
  content: string;
  metadata: {
    source: string;
    category: string;
    tags: string[];
  };
  timestamp: string;
}
```

#### memory_updated
记忆更新

数据结构：
```typescript
{
  memory_id: string;
  changes: {
    content?: string;
    metadata?: any;
  };
  timestamp: string;
}
```

#### memory_deleted
记忆删除

数据结构：
```typescript
{
  memory_id: string;
  timestamp: string;
}
```

#### memory_search_completed
记忆搜索完成

数据结构：
```typescript
{
  query: string;
  result_count: number;
  duration_ms: number;
  timestamp: string;
}
```

### 技能管理事件

#### skill_installed
技能安装

数据结构：
```typescript
{
  skill_id: string;
  name: string;
  version: string;
  permissions: string[];
  timestamp: string;
}
```

#### skill_executed
技能执行

数据结构：
```typescript
{
  skill_id: string;
  execution_id: string;
  input: any;
  timestamp: string;
}
```

#### skill_completed
技能执行完成

数据结构：
```typescript
{
  skill_id: string;
  execution_id: string;
  output: any;
  duration_ms: number;
  timestamp: string;
}
```

#### skill_failed
技能执行失败

数据结构：
```typescript
{
  skill_id: string;
  execution_id: string;
  error: {
    code: string;
    message: string;
  };
  timestamp: string;
}
```

#### skill_uninstalled
技能卸载

数据结构：
```typescript
{
  skill_id: string;
  timestamp: string;
}
```

### MCP 集成事件

#### mcp_connected
MCP 连接建立

数据结构：
```typescript
{
  connection_id: string;
  name: string;
  url: string;
  tool_count: number;
  timestamp: string;
}
```

#### mcp_disconnected
MCP 连接断开

数据结构：
```typescript
{
  connection_id: string;
  reason: "manual" | "error" | "timeout";
  timestamp: string;
}
```

#### mcp_tool_invoked
MCP 工具调用

数据结构：
```typescript
{
  connection_id: string;
  invocation_id: string;
  tool_name: string;
  arguments: any;
  timestamp: string;
}
```

#### mcp_tool_completed
MCP 工具调用完成

数据结构：
```typescript
{
  connection_id: string;
  invocation_id: string;
  tool_name: string;
  result: any;
  duration_ms: number;
  timestamp: string;
}
```

#### mcp_tool_failed
MCP 工具调用失败

数据结构：
```typescript
{
  connection_id: string;
  invocation_id: string;
  tool_name: string;
  error: {
    code: string;
    message: string;
  };
  timestamp: string;
}
```

### 模型管理事件

#### model_configured
模型配置

数据结构：
```typescript
{
  model_id: string;
  timestamp: string;
}
```

#### model_switched
模型切换

数据结构：
```typescript
{
  from_model_id: string;
  to_model_id: string;
  session_id?: string;
  timestamp: string;
}
```

#### model_test_completed
模型测试完成

数据结构：
```typescript
{
  model_id: string;
  status: "available" | "unavailable";
  latency_ms: number;
  timestamp: string;
}
```

### 系统事件

#### system_started
系统启动

数据结构：
```typescript
{
  version: string;
  timestamp: string;
}
```

#### system_shutdown
系统关闭

数据结构：
```typescript
{
  reason: "manual" | "error";
  timestamp: string;
}
```

#### system_error
系统错误

数据结构：
```typescript
{
  error: {
    code: string;
    message: string;
    stack?: string;
  };
  timestamp: string;
}
```

## 事件订阅

### 全局事件流

订阅所有事件：
```
GET /api/v1/events
```

适用场景：
- 全局监控面板
- 系统日志收集
- 调试工具

### 会话事件流

订阅特定会话的事件：
```
GET /api/v1/sessions/{session_id}/events
```

事件类型：
- session_started
- session_completed
- session_failed
- message_added
- message_updated

### 团队事件流

订阅特定团队的事件：
```
GET /api/v1/agent-teams/{team_id}/events
```

事件类型：
- team_activated
- worker_added
- worker_removed
- task_assigned
- task_progress
- task_completed
- task_failed
- team_disbanded

### 过滤事件

通过查询参数过滤事件类型：
```
GET /api/v1/events?types=session_created,session_completed
```

## 连接管理

### 连接生命周期

```
客户端发起连接
    ↓
服务器建立 SSE 连接
    ↓
服务器推送事件
    ↓
客户端接收事件
    ↓
[心跳维持连接]
    ↓
连接断开（超时/错误/主动关闭）
    ↓
客户端自动重连
```

### 超时控制

- 心跳间隔：30 秒
- 客户端超时：60 秒（2 个心跳周期）
- 服务器超时：300 秒（5 分钟无活动）

### 连接限制

- 每个客户端：最多 10 个并发连接
- 全局：最多 1000 个并发连接
- 超出限制：返回 503 Service Unavailable

### 连接关闭

客户端主动关闭：
```javascript
eventSource.close();
```

服务器主动关闭：
```
event: close
data: {"reason": "server_shutdown"}

```

## 错误处理

### 连接错误

客户端无法建立连接：
- 检查网络连接
- 检查服务器状态
- 检查认证 Token

### 事件解析错误

客户端无法解析事件数据：
- 记录错误日志
- 跳过该事件
- 继续接收后续事件

### 服务器错误

服务器推送错误事件：
```
event: error
data: {"code": "INTERNAL_ERROR", "message": "Database connection lost"}
```

客户端处理：
- 显示错误提示
- 尝试重连
- 记录错误日志

## 性能优化

### 事件批量推送

短时间内多个事件合并推送：
```
event: batch
data: [
  {"type": "message_added", "data": {...}},
  {"type": "message_updated", "data": {...}},
  {"type": "message_updated", "data": {...}}
]
```

### 事件压缩

大型事件数据压缩：
- 使用 gzip 压缩
- 响应头：`Content-Encoding: gzip`

### 事件缓存

服务器缓存最近 1000 个事件：
- 客户端重连时从缓存恢复
- 避免事件丢失
- 减少数据库查询

### 连接池管理

服务器维护连接池：
- 空闲连接定期清理
- 活跃连接优先保留
- 连接数限制

## 安全考虑

### 认证

SSE 连接需要认证：
```
GET /api/v1/events HTTP/1.1
Authorization: Bearer <token>
```

### 权限控制

- 全局事件流：需要管理员权限
- 会话事件流：只能订阅自己的会话
- 团队事件流：只能订阅自己的团队

### 数据过滤

敏感数据不推送到事件流：
- API Token
- 密码
- 个人身份信息

### 速率限制

- 每个连接：最多 100 事件/秒
- 超出限制：暂停推送 1 秒

## 客户端实现

### JavaScript 示例

```javascript
// 建立连接
const eventSource = new EventSource('/api/v1/events', {
  headers: {
    'Authorization': 'Bearer <token>'
  }
});

// 监听特定事件
eventSource.addEventListener('session_created', (event) => {
  const data = JSON.parse(event.data);
  console.log('Session created:', data.session_id);
});

// 监听所有事件
eventSource.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log('Event:', event.type, data);
};

// 错误处理
eventSource.onerror = (error) => {
  console.error('SSE error:', error);
  // 自动重连
};

// 关闭连接
eventSource.close();
```

### TypeScript 类型定义

```typescript
// 事件基类
interface BaseEvent {
  timestamp: string;
}

// 会话事件
interface SessionCreatedEvent extends BaseEvent {
  session_id: string;
  model_id: string;
}

interface MessageAddedEvent extends BaseEvent {
  session_id: string;
  message_id: string;
  role: "user" | "assistant" | "system";
  content: string;
}

// 团队事件
interface TeamCreatedEvent extends BaseEvent {
  team_id: string;
  name: string;
  leader_id: string;
}

interface TaskAssignedEvent extends BaseEvent {
  team_id: string;
  task_id: string;
  worker_id: string;
  task_description: string;
}

// 事件联合类型
type Event =
  | SessionCreatedEvent
  | MessageAddedEvent
  | TeamCreatedEvent
  | TaskAssignedEvent
  | ...;

// 事件处理器
type EventHandler<T extends Event> = (event: T) => void;

// 事件订阅
class EventSubscriber {
  private eventSource: EventSource;
  
  constructor(url: string, token: string) {
    this.eventSource = new EventSource(url, {
      headers: { 'Authorization': `Bearer ${token}` }
    });
  }
  
  on<T extends Event>(
    eventType: string,
    handler: EventHandler<T>
  ): void {
    this.eventSource.addEventListener(eventType, (event) => {
      const data = JSON.parse(event.data) as T;
      handler(data);
    });
  }
  
  close(): void {
    this.eventSource.close();
  }
}
```

### React Hook 示例

```typescript
import { useEffect, useState } from 'react';

function useEventStream<T>(url: string, eventType: string) {
  const [data, setData] = useState<T | null>(null);
  const [error, setError] = useState<Error | null>(null);
  
  useEffect(() => {
    const eventSource = new EventSource(url);
    
    eventSource.addEventListener(eventType, (event) => {
      try {
        const data = JSON.parse(event.data) as T;
        setData(data);
      } catch (err) {
        setError(err as Error);
      }
    });
    
    eventSource.onerror = (err) => {
      setError(new Error('SSE connection error'));
    };
    
    return () => {
      eventSource.close();
    };
  }, [url, eventType]);
  
  return { data, error };
}

// 使用示例
function SessionMonitor({ sessionId }: { sessionId: string }) {
  const { data, error } = useEventStream<MessageAddedEvent>(
    `/api/v1/sessions/${sessionId}/events`,
    'message_added'
  );
  
  if (error) return <div>Error: {error.message}</div>;
  if (!data) return <div>Waiting for events...</div>;
  
  return <div>New message: {data.content}</div>;
}
```

## 测试与调试

### 测试工具

使用 curl 测试 SSE 连接：
```bash
curl -N -H "Accept: text/event-stream" \
  -H "Authorization: Bearer <token>" \
  http://localhost:8080/api/v1/events
```

### 调试日志

服务器端日志：
```
[2026-03-06 10:00:00] SSE connection established: client_id=abc123
[2026-03-06 10:00:05] Event pushed: type=session_created, client_id=abc123
[2026-03-06 10:00:30] Heartbeat sent: client_id=abc123
[2026-03-06 10:05:00] SSE connection closed: client_id=abc123, reason=client_disconnect
```

客户端日志：
```javascript
eventSource.addEventListener('open', () => {
  console.log('SSE connection opened');
});

eventSource.addEventListener('error', (error) => {
  console.error('SSE error:', error);
});

eventSource.addEventListener('message', (event) => {
  console.log('SSE event:', event.type, event.data);
});
```

## 参考文档

- API 规范：`docs/api-specification.md`
- 后端架构：`docs/backend-architecture.md`
- 安全设计：`docs/security-design.md`
- 规范文件：`openspec/specs/*/spec.md`

## 附录

### SSE vs WebSocket

| 特性 | SSE | WebSocket |
|------|-----|-----------|
| 通信方向 | 单向（服务器→客户端） | 双向 |
| 协议 | HTTP | WebSocket |
| 重连 | 自动 | 手动 |
| 浏览器支持 | 广泛 | 广泛 |
| 实现复杂度 | 低 | 中 |

Claw Box 选择 SSE 的原因：
- 单向推送满足需求
- 自动重连机制
- 实现简单
- HTTP 协议兼容性好

### 事件命名规范

- 使用下划线分隔：`session_created`
- 动词过去式：`created`, `updated`, `deleted`
- 主语在前：`session_created` 而非 `created_session`
- 避免缩写：`message_added` 而非 `msg_add`
