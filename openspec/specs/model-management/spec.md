# 模型管理

## Requirements

### Requirement: 模型切换
系统 SHALL 支持在会话级别切换模型配置。

#### Scenario: 切换模型
- **WHEN** 用户在模型选择器中切换模型
- **THEN** 系统更新该会话的模型配置并显示新模型名称

#### Scenario: 全局默认模型
- **WHEN** 用户设置全局默认模型
- **THEN** 系统在创建新会话时使用该模型

#### Scenario: 模型切换确认
- **WHEN** 用户切换到不同提供商的模型
- **THEN** 系统提示用户确认，因为上下文可能不兼容

### Requirement: 模型可用性
系统 SHALL 显示模型的可用状态与基础配置项。

#### Scenario: 显示模型状态
- **WHEN** 系统加载模型列表
- **THEN** 每个模型显示可用/不可用状态与配置入口

#### Scenario: 测试模型连接
- **WHEN** 用户测试模型连接
- **THEN** 系统发送测试请求并显示延迟和可用性

#### Scenario: 模型状态监控
- **WHEN** 系统运行时
- **THEN** 系统定期检查模型可用性并更新状态

### Requirement: 模型配置
系统 SHALL 支持配置模型的 API 密钥、端点、参数。

#### Scenario: 配置 API 密钥
- **WHEN** 用户输入 API 密钥
- **THEN** 系统加密存储密钥并验证有效性

#### Scenario: 配置自定义端点
- **WHEN** 用户配置自定义 API 端点
- **THEN** 系统使用该端点进行模型调用

#### Scenario: 配置默认参数
- **WHEN** 用户设置模型默认参数（温度、最大 Token 等）
- **THEN** 系统在调用时使用这些默认值

#### Scenario: 配置代理
- **WHEN** 用户配置 HTTP 代理
- **THEN** 系统通过代理访问模型 API

### Requirement: 多提供商支持
系统 SHALL 支持多个 AI 模型提供商。

#### Scenario: 添加提供商
- **WHEN** 用户添加新的模型提供商
- **THEN** 系统加载该提供商的模型列表

#### Scenario: 提供商认证
- **WHEN** 用户配置提供商认证信息
- **THEN** 系统验证认证并保存配置

#### Scenario: 提供商特定功能
- **WHEN** 用户使用提供商特定功能（如 Claude 的 Artifacts）
- **THEN** 系统启用该功能并适配 UI

### Requirement: 模型能力检测
系统 SHALL 自动检测模型支持的能力。

#### Scenario: 检测视觉能力
- **WHEN** 系统加载模型
- **THEN** 系统检测模型是否支持图像输入

#### Scenario: 检测函数调用
- **WHEN** 系统加载模型
- **THEN** 系统检测模型是否支持函数调用

#### Scenario: 检测流式输出
- **WHEN** 系统加载模型
- **THEN** 系统检测模型是否支持流式输出

### Requirement: 模型使用统计
系统 SHALL 记录模型使用情况和成本。

#### Scenario: 查看使用统计
- **WHEN** 用户查看模型统计
- **THEN** 系统显示调用次数、Token 消耗、成本估算

#### Scenario: 导出使用报告
- **WHEN** 用户导出使用报告
- **THEN** 系统生成 CSV 或 JSON 格式的报告

#### Scenario: 成本告警
- **WHEN** 模型使用成本超过阈值
- **THEN** 系统显示告警提示用户

### Requirement: 模型回退策略
系统 SHALL 支持模型不可用时的回退机制。

#### Scenario: 自动回退
- **WHEN** 主模型不可用
- **THEN** 系统自动切换到备用模型

#### Scenario: 配置回退顺序
- **WHEN** 用户配置模型回退顺序
- **THEN** 系统按顺序尝试可用模型

#### Scenario: 回退通知
- **WHEN** 系统使用回退模型
- **THEN** 系统通知用户当前使用的模型

## Non-Functional Requirements

### Performance
- 模型列表加载时间 < 1s
- 模型切换延迟 < 200ms
- 模型测试响应时间 < 3s
- 支持最多 50 个模型配置

### Scalability
- 支持多个提供商（Anthropic、OpenAI、Google、本地模型）
- 支持自定义模型端点
- 支持模型配置导入导出

### Reliability
- API 密钥加密存储
- 模型调用失败自动重试（最多 3 次）
- 配置变更自动备份

### Security
- API 密钥加密存储（AES-256）
- 敏感配置不记录日志
- 支持密钥轮换

## Data Model

### Model
```typescript
interface Model {
  model_id: string;             // 唯一标识符
  name: string;                 // 模型名称
  display_name: string;         // 显示名称
  provider: ModelProvider;      // 提供商
  status: ModelStatus;          // 模型状态
  capabilities: ModelCapabilities; // 模型能力
  config: ModelConfig;          // 模型配置
  limits: ModelLimits;          // 模型限制
  pricing?: ModelPricing;       // 定价信息
  metadata: ModelMetadata;      // 元数据
  created_at: string;           // 创建时间（ISO 8601）
  updated_at: string;           // 更新时间（ISO 8601）
}

enum ModelProvider {
  Anthropic = "anthropic",      // Anthropic (Claude)
  OpenAI = "openai",            // OpenAI (GPT)
  Google = "google",            // Google (Gemini)
  Local = "local",              // 本地模型
  Custom = "custom"             // 自定义提供商
}

enum ModelStatus {
  Available = "available",      // 可用
  Unavailable = "unavailable",  // 不可用
  Testing = "testing",          // 测试中
  Deprecated = "deprecated"     // 已弃用
}

interface ModelCapabilities {
  vision: boolean;              // 支持图像输入
  function_calling: boolean;    // 支持函数调用
  streaming: boolean;           // 支持流式输出
  json_mode: boolean;           // 支持 JSON 模式
  system_prompt: boolean;       // 支持系统提示词
  max_context_tokens: number;   // 最大上下文 Token 数
}

interface ModelConfig {
  api_key?: string;             // API 密钥（加密存储）
  base_url?: string;            // 自定义端点
  default_params: ModelParams;  // 默认参数
  proxy?: ProxyConfig;          // 代理配置
  timeout_ms: number;           // 超时时间（毫秒）
  retry_config: RetryConfig;    // 重试配置
}

interface ModelParams {
  temperature: number;          // 温度参数（0.0-1.0）
  max_tokens: number;           // 最大 Token 数
  top_p?: number;               // Top-p 采样
  top_k?: number;               // Top-k 采样
  frequency_penalty?: number;   // 频率惩罚
  presence_penalty?: number;    // 存在惩罚
  stop_sequences?: string[];    // 停止序列
}

interface ProxyConfig {
  enabled: boolean;             // 是否启用代理
  host: string;                 // 代理主机
  port: number;                 // 代理端口
  username?: string;            // 代理用户名
  password?: string;            // 代理密码（加密存储）
}

interface RetryConfig {
  max_retries: number;          // 最大重试次数
  initial_delay_ms: number;     // 初始延迟（毫秒）
  max_delay_ms: number;         // 最大延迟（毫秒）
  backoff_multiplier: number;   // 退避倍数
}

interface ModelLimits {
  max_input_tokens: number;     // 最大输入 Token 数
  max_output_tokens: number;    // 最大输出 Token 数
  rate_limit_rpm?: number;      // 速率限制（请求/分钟）
  rate_limit_tpm?: number;      // 速率限制（Token/分钟）
}

interface ModelPricing {
  input_price_per_1k: number;   // 输入价格（每 1K Token）
  output_price_per_1k: number;  // 输出价格（每 1K Token）
  currency: string;             // 货币单位
}

interface ModelMetadata {
  description: string;          // 模型描述
  version?: string;             // 模型版本
  release_date?: string;        // 发布日期
  documentation_url?: string;   // 文档 URL
  tags: string[];               // 标签
}
```

### ModelUsage
```typescript
interface ModelUsage {
  usage_id: string;             // 使用记录 ID
  model_id: string;             // 模型 ID
  session_id?: string;          // 会话 ID
  input_tokens: number;         // 输入 Token 数
  output_tokens: number;        // 输出 Token 数
  total_tokens: number;         // 总 Token 数
  cost?: number;                // 成本
  latency_ms: number;           // 延迟（毫秒）
  timestamp: string;            // 时间戳（ISO 8601）
}

interface UsageStatistics {
  model_id: string;             // 模型 ID
  period: StatisticsPeriod;     // 统计周期
  total_requests: number;       // 总请求数
  successful_requests: number;  // 成功请求数
  failed_requests: number;      // 失败请求数
  total_input_tokens: number;   // 总输入 Token 数
  total_output_tokens: number;  // 总输出 Token 数
  total_cost: number;           // 总成本
  avg_latency_ms: number;       // 平均延迟（毫秒）
}

enum StatisticsPeriod {
  Hour = "hour",                // 小时
  Day = "day",                  // 天
  Week = "week",                // 周
  Month = "month",              // 月
  Year = "year"                 // 年
}
```

### FallbackStrategy
```typescript
interface FallbackStrategy {
  strategy_id: string;          // 策略 ID
  name: string;                 // 策略名称
  primary_model_id: string;     // 主模型 ID
  fallback_models: FallbackModel[]; // 回退模型列表
  enabled: boolean;             // 是否启用
  created_at: string;           // 创建时间（ISO 8601）
}

interface FallbackModel {
  model_id: string;             // 模型 ID
  priority: number;             // 优先级（数字越小优先级越高）
  conditions?: FallbackCondition[]; // 回退条件
}

interface FallbackCondition {
  type: ConditionType;          // 条件类型
  value: any;                   // 条件值
}

enum ConditionType {
  ErrorCode = "error_code",     // 错误码
  StatusCode = "status_code",   // HTTP 状态码
  Timeout = "timeout",          // 超时
  RateLimit = "rate_limit"      // 速率限制
}
```

### ModelTest
```typescript
interface ModelTest {
  test_id: string;              // 测试 ID
  model_id: string;             // 模型 ID
  status: TestStatus;           // 测试状态
  latency_ms?: number;          // 延迟（毫秒）
  error?: string;               // 错误信息
  tested_at: string;            // 测试时间（ISO 8601）
}

enum TestStatus {
  Pending = "pending",          // 待测试
  Testing = "testing",          // 测试中
  Passed = "passed",            // 通过
  Failed = "failed"             // 失败
}
```

## State Machine

### Model State Transitions
```
Unavailable ────────────────────────────────────────────────────┐
  │                                                              │
  │ configure()                                                  │
  ▼                                                              │
Testing ────────────────────────────────────────────────────────┤
  │                                                              │
  │ test_passed()                                                │
  ▼                                                              │
Available ──────────────────────────────────────────────────────┤
  │                                                              │
  │ health_check_failed()                                        │
  └──────────────────────────────────────────────────────────> Unavailable

Available ──────────────────────────────────────────────────────┐
  │                                                              │
  │ deprecate()                                                  │
  ▼                                                              │
Deprecated
```

### Test State Transitions
```
Pending ────────────────────────────────────────────────────────┐
  │                                                              │
  │ start_test()                                                 │
  ▼                                                              │
Testing ────────────────────────────────────────────────────────┤
  │                                                              │
  │ test_completed()                                             │
  ▼                                                              │
Passed

Testing ────────────────────────────────────────────────────────┐
  │                                                              │
  │ test_failed()                                                │
  ▼                                                              │
Failed
```

## Error Scenarios

### Configuration Errors
- **INVALID_API_KEY**: API 密钥无效
- **INVALID_ENDPOINT**: 端点 URL 无效
- **AUTHENTICATION_FAILED**: 认证失败
- **ENCRYPTION_FAILED**: 密钥加密失败
- **DECRYPTION_FAILED**: 密钥解密失败

### Model Availability Errors
- **MODEL_NOT_FOUND**: 模型不存在
- **MODEL_UNAVAILABLE**: 模型不可用
- **MODEL_DEPRECATED**: 模型已弃用
- **PROVIDER_UNAVAILABLE**: 提供商服务不可用

### API Call Errors
- **API_ERROR**: API 调用失败
- **RATE_LIMIT_EXCEEDED**: 超过速率限制
- **QUOTA_EXCEEDED**: 超过配额
- **TIMEOUT**: 请求超时
- **NETWORK_ERROR**: 网络错误
- **INVALID_REQUEST**: 请求参数无效
- **CONTEXT_LENGTH_EXCEEDED**: 上下文长度超过限制

### Fallback Errors
- **NO_FALLBACK_AVAILABLE**: 没有可用的回退模型
- **FALLBACK_FAILED**: 回退模型也失败
- **CIRCULAR_FALLBACK**: 循环回退检测

### Usage Tracking Errors
- **USAGE_TRACKING_FAILED**: 使用统计记录失败
- **COST_CALCULATION_FAILED**: 成本计算失败

### Test Errors
- **TEST_TIMEOUT**: 测试超时
- **TEST_FAILED**: 测试失败
- **INVALID_TEST_CONFIG**: 测试配置无效

## Design References

- 后端架构：`docs/backend-architecture.md` § 模型聚合设计
- API 规范：`docs/api-specification.md` § 模型管理 API
- 事件协议：`docs/event-protocol.md` § 模型管理事件
- 安全设计：`docs/security-design.md` § API 密钥加密存储
