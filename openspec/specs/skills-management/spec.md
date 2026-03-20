# 技能管理

## Purpose
定义技能安装、配置、权限控制、沙箱执行、市场分发与版本更新的统一规范，确保扩展能力在可治理的安全边界内运行并支持长期演进。

## Requirements

### Requirement: 技能安装与配置
系统 SHALL 支持技能的安装、启用与配置。

#### Scenario: 启用技能
- **WHEN** 用户在技能列表中启用技能
- **THEN** 系统保存配置并标记为启用状态

#### Scenario: 安装技能
- **WHEN** 用户从技能市场或本地安装技能
- **THEN** 系统下载、验证并安装技能包

#### Scenario: 配置技能
- **WHEN** 用户修改技能配置参数
- **THEN** 系统验证配置并保存

#### Scenario: 卸载技能
- **WHEN** 用户卸载技能
- **THEN** 系统移除技能文件并清理相关数据

### Requirement: 技能运行状态
系统 SHALL 展示技能运行状态与版本信息。

#### Scenario: 查看技能状态
- **WHEN** 用户进入技能详情
- **THEN** 系统显示技能版本与运行状态

#### Scenario: 查看技能日志
- **WHEN** 用户查看技能执行日志
- **THEN** 系统显示最近的执行记录和错误信息

### Requirement: 技能权限系统
系统 SHALL 实现细粒度的技能权限控制。

#### Scenario: 授予权限
- **WHEN** 用户安装技能时
- **THEN** 系统显示技能请求的权限列表，用户确认后授予

#### Scenario: 权限检查
- **WHEN** 技能执行需要特定权限的操作
- **THEN** 系统检查权限，未授权则拒绝执行

#### Scenario: 撤销权限
- **WHEN** 用户撤销技能的某项权限
- **THEN** 系统更新权限配置，技能无法再执行该操作

#### Scenario: 权限审计
- **WHEN** 用户查看权限使用记录
- **THEN** 系统显示技能的权限使用历史

### Requirement: 技能沙箱隔离
系统 SHALL 在沙箱环境中执行技能代码。

#### Scenario: 沙箱执行
- **WHEN** 技能代码执行
- **THEN** 系统在隔离的沙箱环境中运行，限制资源访问

#### Scenario: 资源限制
- **WHEN** 技能执行超过资源限制
- **THEN** 系统终止执行并返回错误

#### Scenario: 文件系统隔离
- **WHEN** 技能访问文件系统
- **THEN** 系统只允许访问授权的目录

### Requirement: 技能市场
系统 SHALL 提供技能市场，支持浏览、搜索、安装技能。

#### Scenario: 浏览技能市场
- **WHEN** 用户打开技能市场
- **THEN** 系统显示可用技能列表，包含名称、描述、评分

#### Scenario: 搜索技能
- **WHEN** 用户搜索技能
- **THEN** 系统返回匹配的技能列表

#### Scenario: 查看技能详情
- **WHEN** 用户点击技能
- **THEN** 系统显示技能的详细信息、权限要求、使用说明

### Requirement: 技能更新
系统 SHALL 支持技能的自动和手动更新。

#### Scenario: 检查更新
- **WHEN** 系统启动或用户手动检查
- **THEN** 系统检查已安装技能的更新

#### Scenario: 自动更新
- **WHEN** 检测到技能更新且用户启用自动更新
- **THEN** 系统后台下载并安装更新

#### Scenario: 手动更新
- **WHEN** 用户手动更新技能
- **THEN** 系统下载新版本并替换旧版本

### Requirement: 技能开发支持
系统 SHALL 提供技能开发工具和文档。

#### Scenario: 创建技能模板
- **WHEN** 用户创建新技能
- **THEN** 系统生成技能模板代码

#### Scenario: 本地测试
- **WHEN** 用户测试技能
- **THEN** 系统在本地环境中执行技能并显示结果

#### Scenario: 发布技能
- **WHEN** 用户发布技能到市场
- **THEN** 系统验证技能包并上传到市场

## Non-Functional Requirements

### Performance
- 技能安装时间 < 10s
- 技能执行启动延迟 < 500ms
- 权限检查延迟 < 10ms
- 支持最多 100 个已安装技能

### Scalability
- 支持最多 10,000 个技能市场条目
- 支持并发执行 50 个技能实例
- 技能包大小限制 100 MB

### Reliability
- 技能执行失败不影响系统稳定性
- 沙箱崩溃自动恢复
- 技能更新失败自动回滚

### Security
- 所有技能代码在沙箱中执行
- 权限最小化原则
- 技能包签名验证
- 敏感数据加密存储

## Data Model

### Skill
```typescript
interface Skill {
  skill_id: string;             // 唯一标识符
  name: string;                 // 技能名称
  version: string;              // 版本号（语义化版本）
  description: string;          // 技能描述
  author: string;               // 作者
  status: SkillStatus;          // 技能状态
  permissions: Permission[];    // 权限列表
  config: SkillConfig;          // 技能配置
  metadata: SkillMetadata;      // 元数据
  installed_at: string;         // 安装时间（ISO 8601）
  updated_at: string;           // 更新时间（ISO 8601）
}

enum SkillStatus {
  Installed = "installed",      // 已安装
  Enabled = "enabled",          // 已启用
  Disabled = "disabled",        // 已禁用
  Updating = "updating",        // 更新中
  Failed = "failed"             // 失败
}

interface SkillConfig {
  entry_point: string;          // 入口文件
  runtime: Runtime;             // 运行时环境
  environment: Record<string, string>; // 环境变量
  resource_limits: ResourceLimits; // 资源限制
  auto_update: boolean;         // 是否自动更新
}

enum Runtime {
  NodeJS = "nodejs",            // Node.js
  Python = "python",            // Python
  Deno = "deno",                // Deno
  WASM = "wasm"                 // WebAssembly
}

interface ResourceLimits {
  max_memory_mb: number;        // 最大内存（MB）
  max_cpu_percent: number;      // 最大 CPU 占用（%）
  max_execution_time_ms: number; // 最大执行时间（毫秒）
  max_file_size_mb: number;     // 最大文件大小（MB）
}

interface SkillMetadata {
  homepage?: string;            // 主页 URL
  repository?: string;          // 代码仓库 URL
  license: string;              // 许可证
  tags: string[];               // 标签
  rating?: number;              // 评分（0.0-5.0）
  download_count: number;       // 下载次数
  last_updated: string;         // 最后更新时间
}
```

### Permission
```typescript
interface Permission {
  permission_id: string;        // 权限 ID
  name: string;                 // 权限名称
  description: string;          // 权限描述
  level: PermissionLevel;       // 权限级别
  granted: boolean;             // 是否已授予
  granted_at?: string;          // 授予时间（ISO 8601）
}

enum PermissionLevel {
  Read = "read",                // 只读
  Write = "write",              // 读写
  Execute = "execute",          // 执行
  Network = "network",          // 网络访问
  FileSystem = "filesystem",    // 文件系统访问
  System = "system"             // 系统级访问
}

interface PermissionScope {
  type: ScopeType;
  resources: string[];          // 资源列表（路径、URL 等）
}

enum ScopeType {
  FileSystem = "filesystem",    // 文件系统
  Network = "network",          // 网络
  Environment = "environment",  // 环境变量
  Process = "process"           // 进程
}
```

### SkillExecution
```typescript
interface SkillExecution {
  execution_id: string;         // 执行 ID
  skill_id: string;             // 技能 ID
  input: any;                   // 输入参数
  output?: any;                 // 输出结果
  status: ExecutionStatus;      // 执行状态
  error?: ExecutionError;       // 错误信息
  started_at: string;           // 开始时间（ISO 8601）
  completed_at?: string;        // 完成时间（ISO 8601）
  duration_ms?: number;         // 执行时长（毫秒）
  resource_usage: ResourceUsage; // 资源使用情况
}

enum ExecutionStatus {
  Pending = "pending",          // 待执行
  Running = "running",          // 运行中
  Completed = "completed",      // 已完成
  Failed = "failed",            // 失败
  Timeout = "timeout",          // 超时
  Cancelled = "cancelled"       // 已取消
}

interface ExecutionError {
  code: string;                 // 错误码
  message: string;              // 错误消息
  stack?: string;               // 堆栈跟踪
}

interface ResourceUsage {
  memory_mb: number;            // 内存使用（MB）
  cpu_percent: number;          // CPU 占用（%）
  execution_time_ms: number;    // 执行时间（毫秒）
}
```

### SkillMarketplace
```typescript
interface MarketplaceSkill {
  skill_id: string;             // 技能 ID
  name: string;                 // 技能名称
  description: string;          // 描述
  author: string;               // 作者
  version: string;              // 最新版本
  icon_url?: string;            // 图标 URL
  screenshots?: string[];       // 截图 URL 列表
  permissions: Permission[];    // 权限要求
  rating: number;               // 评分（0.0-5.0）
  review_count: number;         // 评论数
  download_count: number;       // 下载次数
  size_mb: number;              // 包大小（MB）
  published_at: string;         // 发布时间（ISO 8601）
  updated_at: string;           // 更新时间（ISO 8601）
}

interface SkillReview {
  review_id: string;            // 评论 ID
  skill_id: string;             // 技能 ID
  user_id: string;              // 用户 ID
  rating: number;               // 评分（0.0-5.0）
  comment: string;              // 评论内容
  created_at: string;           // 创建时间（ISO 8601）
}
```

### Sandbox
```typescript
interface Sandbox {
  sandbox_id: string;           // 沙箱 ID
  skill_id: string;             // 技能 ID
  status: SandboxStatus;        // 沙箱状态
  isolated_fs: string;          // 隔离文件系统路径
  allowed_paths: string[];      // 允许访问的路径
  allowed_hosts: string[];      // 允许访问的主机
  resource_limits: ResourceLimits; // 资源限制
  created_at: string;           // 创建时间（ISO 8601）
}

enum SandboxStatus {
  Created = "created",          // 已创建
  Running = "running",          // 运行中
  Stopped = "stopped",          // 已停止
  Failed = "failed"             // 失败
}
```

## State Machine

### Skill State Transitions
```
Installed ──────────────────────────────────────────────────────┐
  │                                                              │
  │ enable()                                                     │
  ▼                                                              │
Enabled ────────────────────────────────────────────────────────┤
  │                                                              │
  │ disable()                                                    │
  └──────────────────────────────────────────────────────────> Disabled

Enabled ────────────────────────────────────────────────────────┐
  │                                                              │
  │ start_update()                                               │
  ▼                                                              │
Updating ───────────────────────────────────────────────────────┤
  │                                                              │
  │ update_completed()                                           │
  └──────────────────────────────────────────────────────────> Enabled

Updating ───────────────────────────────────────────────────────┐
  │                                                              │
  │ update_failed()                                              │
  ▼                                                              │
Failed ─────────────────────────────────────────────────────────┤
  │                                                              │
  │ rollback()                                                   │
  └──────────────────────────────────────────────────────────> Enabled
```

### Execution State Transitions
```
Pending ────────────────────────────────────────────────────────┐
  │                                                              │
  │ start_execution()                                            │
  ▼                                                              │
Running ────────────────────────────────────────────────────────┤
  │                                                              │
  │ execution_completed()                                        │
  ▼                                                              │
Completed

Running ────────────────────────────────────────────────────────┐
  │                                                              │
  │ execution_failed()                                           │
  ▼                                                              │
Failed

Running ────────────────────────────────────────────────────────┐
  │                                                              │
  │ timeout_exceeded()                                           │
  ▼                                                              │
Timeout

Running ────────────────────────────────────────────────────────┐
  │                                                              │
  │ cancel_execution()                                           │
  ▼                                                              │
Cancelled
```

## Error Scenarios

### Installation Errors
- **INVALID_PACKAGE**: 技能包格式错误或损坏
- **SIGNATURE_VERIFICATION_FAILED**: 签名验证失败
- **INCOMPATIBLE_VERSION**: 技能版本与系统不兼容
- **DEPENDENCY_MISSING**: 缺少依赖项
- **STORAGE_FULL**: 存储空间不足
- **QUOTA_EXCEEDED**: 已安装技能数量超过限制

### Execution Errors
- **PERMISSION_DENIED**: 权限不足
- **SANDBOX_CREATION_FAILED**: 沙箱创建失败
- **RESOURCE_LIMIT_EXCEEDED**: 超过资源限制
- **EXECUTION_TIMEOUT**: 执行超时
- **RUNTIME_ERROR**: 运行时错误
- **INVALID_INPUT**: 输入参数无效

### Permission Errors
- **PERMISSION_NOT_GRANTED**: 权限未授予
- **PERMISSION_REVOKED**: 权限已撤销
- **INVALID_PERMISSION**: 无效的权限请求
- **PERMISSION_CONFLICT**: 权限冲突

### Update Errors
- **UPDATE_CHECK_FAILED**: 检查更新失败
- **DOWNLOAD_FAILED**: 下载失败
- **UPDATE_VERIFICATION_FAILED**: 更新验证失败
- **ROLLBACK_FAILED**: 回滚失败

### Marketplace Errors
- **MARKETPLACE_UNAVAILABLE**: 技能市场不可用
- **SKILL_NOT_FOUND**: 技能不存在
- **NETWORK_ERROR**: 网络错误
- **AUTHENTICATION_REQUIRED**: 需要认证

## Design References

- 后端架构：`docs/backend-architecture.md` § 技能聚合设计
- API 规范：`docs/api-specification.md` § 技能管理 API
- 事件协议：`docs/event-protocol.md` § 技能管理事件
- 安全设计：`docs/security-design.md` § 技能沙箱隔离
