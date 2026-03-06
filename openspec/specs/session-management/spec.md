# 会话管理

## Requirements

### Requirement: 多会话生命周期
系统 SHALL 允许用户创建、重命名、归档与删除会话。

#### Scenario: 创建会话
- **WHEN** 用户点击新建会话
- **THEN** 系统创建一个新会话并出现在列表中

#### Scenario: 归档会话
- **WHEN** 用户对会话执行归档操作
- **THEN** 会话从默认列表移入归档列表

#### Scenario: 删除会话
- **WHEN** 用户删除会话
- **THEN** 系统标记会话为已删除状态，不再显示在列表中

#### Scenario: 重命名会话
- **WHEN** 用户修改会话名称
- **THEN** 系统更新会话名称并保存

### Requirement: 会话上下文状态
系统 SHALL 显示会话的模型、状态与上下文占用信息。

#### Scenario: 查看会话状态
- **WHEN** 用户选中某个会话
- **THEN** 系统显示该会话的模型名称、运行状态与上下文占用

#### Scenario: 上下文占用警告
- **WHEN** 会话上下文占用超过 80%
- **THEN** 系统显示警告提示用户清理历史消息

### Requirement: 消息历史管理
系统 SHALL 支持查看、搜索、导出会话消息历史。

#### Scenario: 查看消息历史
- **WHEN** 用户打开会话
- **THEN** 系统加载并显示该会话的所有消息

#### Scenario: 搜索消息
- **WHEN** 用户在会话中搜索关键词
- **THEN** 系统高亮显示匹配的消息

#### Scenario: 导出会话
- **WHEN** 用户导出会话
- **THEN** 系统生成 Markdown 或 JSON 格式的文件

### Requirement: 流式消息输出
系统 SHALL 支持实时流式显示 AI 响应内容。

#### Scenario: 流式输出
- **WHEN** AI 开始生成响应
- **THEN** 系统逐字显示生成的内容

#### Scenario: 停止生成
- **WHEN** 用户点击停止按钮
- **THEN** 系统中断流式输出并保存已生成的内容

### Requirement: 会话配置
系统 SHALL 允许用户配置会话级别的模型参数。

#### Scenario: 配置温度参数
- **WHEN** 用户调整温度参数
- **THEN** 系统保存配置并在下次请求时使用

#### Scenario: 配置系统提示词
- **WHEN** 用户设置系统提示词
- **THEN** 系统将提示词注入到会话上下文

## Non-Functional Requirements

### Performance
- 会话列表加载时间 < 500ms
- 消息发送响应时间 < 100ms
- 流式输出首字延迟 < 1s
- 支持单会话最多 10,000 条消息

### Scalability
- 支持最多 1,000 个并发会话
- 支持最多 100 个活跃会话
- 消息历史分页加载，每页 50 条

### Reliability
- 会话数据自动保存，无需手动操作
- 流式输出中断后可恢复
- 网络异常时自动重试（最多 3 次）

## Data Model

### Session
```typescript
interface Session {
  session_id: string;           // 唯一标识符
  name: string;                 // 会话名称
  model_id: string;             // 使用的模型 ID
  status: SessionStatus;        // 会话状态
  system_prompt?: string;       // 系统提示词
  config: SessionConfig;        // 会话配置
  message_count: number;        // 消息数量
  context_usage: ContextUsage;  // 上下文占用
  created_at: string;           // 创建时间（ISO 8601）
  updated_at: string;           // 更新时间（ISO 8601）
  archived_at?: string;         // 归档时间（ISO 8601）
  deleted_at?: string;          // 删除时间（ISO 8601）
}

enum SessionStatus {
  Idle = "idle",               // 空闲
  Running = "running",         // 运行中
  Completed = "completed",     // 已完成
  Failed = "failed",           // 失败
  Cancelled = "cancelled"      // 已取消
}

interface SessionConfig {
  temperature: number;         // 温度参数（0.0-1.0）
  max_tokens: number;          // 最大 Token 数
  top_p?: number;              // Top-p 采样
  frequency_penalty?: number;  // 频率惩罚
  presence_penalty?: number;   // 存在惩罚
}

interface ContextUsage {
  input_tokens: number;        // 输入 Token 数
  output_tokens: number;       // 输出 Token 数
  total_tokens: number;        // 总 Token 数
  max_tokens: number;          // 最大 Token 限制
  usage_percentage: number;    // 占用百分比（0.0-1.0）
}
```

### Message
```typescript
interface Message {
  message_id: string;          // 唯一标识符
  session_id: string;          // 所属会话 ID
  role: MessageRole;           // 消息角色
  content: string;             // 消息内容
  attachments?: Attachment[];  // 附件列表
  metadata?: MessageMetadata;  // 元数据
  timestamp: string;           // 时间戳（ISO 8601）
}

enum MessageRole {
  User = "user",               // 用户消息
  Assistant = "assistant",     // AI 助手消息
  System = "system"            // 系统消息
}

interface Attachment {
  type: "image" | "file";      // 附件类型
  url: string;                 // 附件 URL
  name: string;                // 附件名称
  size: number;                // 附件大小（字节）
}

interface MessageMetadata {
  model_id: string;            // 使用的模型
  finish_reason?: string;      // 完成原因
  usage?: {
    input_tokens: number;
    output_tokens: number;
  };
}
```

## State Machine

### Session State Transitions
```
Idle ──────────────────────────────────────────────────────────┐
  │                                                              │
  │ start_generation()                                           │
  ▼                                                              │
Running ────────────────────────────────────────────────────────┤
  │                                                              │
  │ complete_generation()                                        │
  ▼                                                              │
Completed ──────────────────────────────────────────────────────┤
  │                                                              │
  │ start_new_generation()                                       │
  └──────────────────────────────────────────────────────────> Idle

Running ────────────────────────────────────────────────────────┐
  │                                                              │
  │ cancel_generation()                                          │
  ▼                                                              │
Cancelled ──────────────────────────────────────────────────────┤
  │                                                              │
  │ start_new_generation()                                       │
  └──────────────────────────────────────────────────────────> Idle

Running ────────────────────────────────────────────────────────┐
  │                                                              │
  │ error_occurred()                                             │
  ▼                                                              │
Failed ─────────────────────────────────────────────────────────┤
  │                                                              │
  │ retry_generation()                                           │
  └──────────────────────────────────────────────────────────> Idle
```

## Error Scenarios

### Session Creation Errors
- **VALIDATION_ERROR**: 会话名称为空或超过 100 字符
- **MODEL_NOT_FOUND**: 指定的模型 ID 不存在
- **QUOTA_EXCEEDED**: 会话数量超过限制（1000 个）

### Message Sending Errors
- **SESSION_NOT_FOUND**: 会话 ID 不存在
- **SESSION_ARCHIVED**: 会话已归档，无法发送消息
- **SESSION_DELETED**: 会话已删除，无法发送消息
- **CONTEXT_OVERFLOW**: 上下文占用超过 100%
- **MODEL_API_ERROR**: 模型 API 调用失败
- **NETWORK_ERROR**: 网络连接失败
- **TIMEOUT_ERROR**: 请求超时（30 秒）

### Stream Output Errors
- **STREAM_INTERRUPTED**: 流式输出中断
- **STREAM_TIMEOUT**: 流式输出超时（5 分钟）
- **INVALID_STREAM_DATA**: 流式数据格式错误

### Session Management Errors
- **CONCURRENT_MODIFICATION**: 并发修改冲突
- **PERMISSION_DENIED**: 权限不足
- **STORAGE_ERROR**: 存储失败

## Design References

- 后端架构：`docs/backend-architecture.md` § 会话聚合设计
- API 规范：`docs/api-specification.md` § 会话管理 API
- 事件协议：`docs/event-protocol.md` § 会话事件
