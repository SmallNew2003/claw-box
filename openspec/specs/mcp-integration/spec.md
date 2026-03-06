# MCP 集成

## Requirements

### Requirement: MCP 连接管理
系统 SHALL 支持添加、启用与移除 MCP 连接。

#### Scenario: 添加 MCP 连接
- **WHEN** 用户输入 MCP 服务器地址并保存
- **THEN** 系统建立连接并显示连接状态

#### Scenario: 测试连接
- **WHEN** 用户测试 MCP 连接
- **THEN** 系统发送测试请求并显示连接是否成功

#### Scenario: 断开连接
- **WHEN** 用户断开 MCP 连接
- **THEN** 系统关闭连接并更新状态

#### Scenario: 移除连接
- **WHEN** 用户移除 MCP 连接
- **THEN** 系统删除连接配置和相关数据

#### Scenario: 自动重连
- **WHEN** MCP 连接意外断开
- **THEN** 系统自动尝试重新连接（最多 3 次）

### Requirement: MCP 工具展示
系统 SHALL 展示 MCP 连接提供的工具与资源列表。

#### Scenario: 查看工具清单
- **WHEN** 用户打开某个 MCP 连接
- **THEN** 系统显示该连接的工具与资源信息

#### Scenario: 查看工具详情
- **WHEN** 用户点击某个工具
- **THEN** 系统显示工具的参数定义、返回值类型、使用示例

#### Scenario: 搜索工具
- **WHEN** 用户搜索工具
- **THEN** 系统返回匹配的工具列表

### Requirement: MCP 工具调用
系统 SHALL 支持调用 MCP 服务器提供的工具。

#### Scenario: 手动调用工具
- **WHEN** 用户手动调用 MCP 工具
- **THEN** 系统发送请求并显示返回结果

#### Scenario: AI 自动调用工具
- **WHEN** AI 决定调用 MCP 工具
- **THEN** 系统自动发送请求并将结果返回给 AI

#### Scenario: 批量调用工具
- **WHEN** 需要调用多个工具
- **THEN** 系统并行发送请求并聚合结果

### Requirement: 连接配置管理
系统 SHALL 支持配置 MCP 连接的认证、超时、重试等参数。

#### Scenario: 配置认证信息
- **WHEN** 用户配置 API Token 或密钥
- **THEN** 系统加密存储认证信息

#### Scenario: 配置超时时间
- **WHEN** 用户设置连接超时时间
- **THEN** 系统在超时后自动断开连接

#### Scenario: 配置重试策略
- **WHEN** 用户配置重试次数和间隔
- **THEN** 系统按配置进行重试

### Requirement: 连接健康监控
系统 SHALL 监控 MCP 连接的健康状态。

#### Scenario: 定期健康检查
- **WHEN** 系统运行时
- **THEN** 系统每 30 秒发送心跳检查连接状态

#### Scenario: 连接异常告警
- **WHEN** 连接健康检查失败
- **THEN** 系统显示告警并尝试重连

#### Scenario: 查看连接统计
- **WHEN** 用户查看连接统计
- **THEN** 系统显示请求数、成功率、平均延迟等指标

### Requirement: MCP 协议版本兼容
系统 SHALL 支持多个 MCP 协议版本。

#### Scenario: 协议版本协商
- **WHEN** 建立 MCP 连接
- **THEN** 系统与服务器协商使用的协议版本

#### Scenario: 降级兼容
- **WHEN** 服务器不支持最新协议版本
- **THEN** 系统降级到服务器支持的版本

### Requirement: 工具调用日志
系统 SHALL 记录所有 MCP 工具调用的日志。

#### Scenario: 查看调用历史
- **WHEN** 用户查看工具调用历史
- **THEN** 系统显示调用时间、参数、结果、耗时

#### Scenario: 导出调用日志
- **WHEN** 用户导出调用日志
- **THEN** 系统生成 JSON 或 CSV 格式的日志文件

#### Scenario: 调用统计分析
- **WHEN** 用户查看调用统计
- **THEN** 系统显示调用频率、成功率、平均耗时等指标

## Non-Functional Requirements

### Performance
- 连接建立时间 < 2s
- 工具调用延迟 < 1s
- 支持最多 50 个并发连接
- 支持最多 1000 个工具

### Scalability
- 支持多种传输协议（HTTP、WebSocket、stdio）
- 支持连接池管理
- 支持请求队列和限流

### Reliability
- 连接断开自动重连
- 请求失败自动重试（最多 3 次）
- 连接状态持久化

### Security
- 认证信息加密存储
- 支持 TLS/SSL 加密传输
- 工具调用权限控制

## Data Model

### MCPConnection
```typescript
interface MCPConnection {
  connection_id: string;        // 唯一标识符
  name: string;                 // 连接名称
  url: string;                  // 服务器 URL
  status: ConnectionStatus;     // 连接状态
  protocol: MCPProtocol;        // 协议类型
  version: string;              // 协议版本
  config: ConnectionConfig;     // 连接配置
  tools: MCPTool[];             // 工具列表
  resources: MCPResource[];     // 资源列表
  statistics: ConnectionStatistics; // 统计信息
  created_at: string;           // 创建时间（ISO 8601）
  connected_at?: string;        // 连接时间（ISO 8601）
  disconnected_at?: string;     // 断开时间（ISO 8601）
}

enum ConnectionStatus {
  Disconnected = "disconnected", // 已断开
  Connecting = "connecting",     // 连接中
  Connected = "connected",       // 已连接
  Failed = "failed",             // 失败
  Reconnecting = "reconnecting"  // 重连中
}

enum MCPProtocol {
  HTTP = "http",                // HTTP/HTTPS
  WebSocket = "websocket",      // WebSocket
  Stdio = "stdio"               // 标准输入输出
}

interface ConnectionConfig {
  auth?: AuthConfig;            // 认证配置
  timeout_ms: number;           // 超时时间（毫秒）
  retry_config: RetryConfig;    // 重试配置
  health_check_interval_ms: number; // 健康检查间隔（毫秒）
  max_concurrent_requests: number;  // 最大并发请求数
}

interface AuthConfig {
  type: AuthType;               // 认证类型
  credentials: Record<string, string>; // 认证凭据（加密存储）
}

enum AuthType {
  None = "none",                // 无认证
  Bearer = "bearer",            // Bearer Token
  ApiKey = "api_key",           // API Key
  Basic = "basic",              // Basic Auth
  OAuth2 = "oauth2"             // OAuth 2.0
}

interface RetryConfig {
  max_retries: number;          // 最大重试次数
  initial_delay_ms: number;     // 初始延迟（毫秒）
  max_delay_ms: number;         // 最大延迟（毫秒）
  backoff_multiplier: number;   // 退避倍数
}

interface ConnectionStatistics {
  total_requests: number;       // 总请求数
  successful_requests: number;  // 成功请求数
  failed_requests: number;      // 失败请求数
  avg_latency_ms: number;       // 平均延迟（毫秒）
  last_request_at?: string;     // 最后请求时间（ISO 8601）
}
```

### MCPTool
```typescript
interface MCPTool {
  tool_id: string;              // 工具 ID
  connection_id: string;        // 所属连接 ID
  name: string;                 // 工具名称
  description: string;          // 工具描述
  input_schema: JSONSchema;     // 输入参数 Schema
  output_schema?: JSONSchema;   // 输出结果 Schema
  examples?: ToolExample[];     // 使用示例
  metadata?: ToolMetadata;      // 元数据
}

interface JSONSchema {
  type: string;                 // 类型
  properties?: Record<string, any>; // 属性定义
  required?: string[];          // 必需字段
  description?: string;         // 描述
}

interface ToolExample {
  description: string;          // 示例描述
  input: any;                   // 输入示例
  output: any;                  // 输出示例
}

interface ToolMetadata {
  category?: string;            // 工具类别
  tags?: string[];              // 标签
  version?: string;             // 版本
  deprecated?: boolean;         // 是否已弃用
}
```

### MCPResource
```typescript
interface MCPResource {
  resource_id: string;          // 资源 ID
  connection_id: string;        // 所属连接 ID
  uri: string;                  // 资源 URI
  name: string;                 // 资源名称
  description: string;          // 资源描述
  mime_type?: string;           // MIME 类型
  metadata?: Record<string, any>; // 元数据
}
```

### ToolInvocation
```typescript
interface ToolInvocation {
  invocation_id: string;        // 调用 ID
  connection_id: string;        // 连接 ID
  tool_name: string;            // 工具名称
  arguments: any;               // 调用参数
  result?: any;                 // 调用结果
  status: InvocationStatus;     // 调用状态
  error?: InvocationError;      // 错误信息
  started_at: string;           // 开始时间（ISO 8601）
  completed_at?: string;        // 完成时间（ISO 8601）
  duration_ms?: number;         // 耗时（毫秒）
}

enum InvocationStatus {
  Pending = "pending",          // 待执行
  Running = "running",          // 执行中
  Completed = "completed",      // 已完成
  Failed = "failed",            // 失败
  Timeout = "timeout",          // 超时
  Cancelled = "cancelled"       // 已取消
}

interface InvocationError {
  code: string;                 // 错误码
  message: string;              // 错误消息
  details?: any;                // 错误详情
}
```

### HealthCheck
```typescript
interface HealthCheck {
  check_id: string;             // 检查 ID
  connection_id: string;        // 连接 ID
  status: HealthStatus;         // 健康状态
  latency_ms: number;           // 延迟（毫秒）
  checked_at: string;           // 检查时间（ISO 8601）
  error?: string;               // 错误信息
}

enum HealthStatus {
  Healthy = "healthy",          // 健康
  Degraded = "degraded",        // 降级
  Unhealthy = "unhealthy"       // 不健康
}
```

## State Machine

### Connection State Transitions
```
Disconnected ───────────────────────────────────────────────────┐
  │                                                              │
  │ connect()                                                    │
  ▼                                                              │
Connecting ─────────────────────────────────────────────────────┤
  │                                                              │
  │ connection_established()                                     │
  ▼                                                              │
Connected ──────────────────────────────────────────────────────┤
  │                                                              │
  │ disconnect()                                                 │
  └──────────────────────────────────────────────────────────> Disconnected

Connecting ─────────────────────────────────────────────────────┐
  │                                                              │
  │ connection_failed()                                          │
  ▼                                                              │
Failed ─────────────────────────────────────────────────────────┤
  │                                                              │
  │ retry()                                                      │
  └──────────────────────────────────────────────────────────> Connecting

Connected ──────────────────────────────────────────────────────┐
  │                                                              │
  │ connection_lost()                                            │
  ▼                                                              │
Reconnecting ───────────────────────────────────────────────────┤
  │                                                              │
  │ reconnection_succeeded()                                     │
  └──────────────────────────────────────────────────────────> Connected

Reconnecting ───────────────────────────────────────────────────┐
  │                                                              │
  │ reconnection_failed()                                        │
  └──────────────────────────────────────────────────────────> Failed
```

### Invocation State Transitions
```
Pending ────────────────────────────────────────────────────────┐
  │                                                              │
  │ start_invocation()                                           │
  ▼                                                              │
Running ────────────────────────────────────────────────────────┤
  │                                                              │
  │ invocation_completed()                                       │
  ▼                                                              │
Completed

Running ────────────────────────────────────────────────────────┐
  │                                                              │
  │ invocation_failed()                                          │
  ▼                                                              │
Failed

Running ────────────────────────────────────────────────────────┐
  │                                                              │
  │ timeout_exceeded()                                           │
  ▼                                                              │
Timeout

Running ────────────────────────────────────────────────────────┐
  │                                                              │
  │ cancel_invocation()                                          │
  ▼                                                              │
Cancelled
```

## Error Scenarios

### Connection Errors
- **INVALID_URL**: URL 格式错误
- **CONNECTION_REFUSED**: 连接被拒绝
- **CONNECTION_TIMEOUT**: 连接超时
- **AUTHENTICATION_FAILED**: 认证失败
- **PROTOCOL_MISMATCH**: 协议版本不匹配
- **TLS_ERROR**: TLS/SSL 错误
- **MAX_CONNECTIONS_REACHED**: 达到最大连接数

### Tool Discovery Errors
- **DISCOVERY_FAILED**: 工具发现失败
- **INVALID_TOOL_SCHEMA**: 工具 Schema 无效
- **NO_TOOLS_AVAILABLE**: 没有可用工具

### Invocation Errors
- **TOOL_NOT_FOUND**: 工具不存在
- **INVALID_ARGUMENTS**: 参数无效
- **INVOCATION_TIMEOUT**: 调用超时
- **INVOCATION_FAILED**: 调用失败
- **RATE_LIMIT_EXCEEDED**: 超过速率限制
- **PERMISSION_DENIED**: 权限不足

### Health Check Errors
- **HEALTH_CHECK_FAILED**: 健康检查失败
- **HEALTH_CHECK_TIMEOUT**: 健康检查超时

### Configuration Errors
- **INVALID_CONFIG**: 配置无效
- **MISSING_CREDENTIALS**: 缺少认证凭据
- **ENCRYPTION_FAILED**: 加密失败
- **DECRYPTION_FAILED**: 解密失败

## Design References

- 后端架构：`docs/backend-architecture.md` § MCP 连接聚合设计
- API 规范：`docs/api-specification.md` § MCP 集成 API
- 事件协议：`docs/event-protocol.md` § MCP 集成事件
- 安全设计：`docs/security-design.md` § MCP 认证与加密
