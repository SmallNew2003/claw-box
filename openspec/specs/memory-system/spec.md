# 记忆系统

## Requirements

### Requirement: 长期记忆存储
系统 SHALL 支持存储、检索与管理长期记忆条目。

#### Scenario: 存储记忆
- **WHEN** 用户标记某段对话为记忆
- **THEN** 系统提取内容、生成摘要与 embedding，存储为记忆条目

#### Scenario: 检索记忆
- **WHEN** 用户发起新对话
- **THEN** 系统根据语义相似度检索相关记忆并注入上下文

#### Scenario: 自动记忆提取
- **WHEN** 会话结束
- **THEN** 系统自动分析对话内容，提取重要信息作为记忆

### Requirement: 记忆管理
系统 SHALL 允许用户查看、编辑、删除与标签化记忆条目。

#### Scenario: 查看记忆列表
- **WHEN** 用户打开记忆管理界面
- **THEN** 系统展示所有记忆条目，包含摘要、标签与来源

#### Scenario: 编辑记忆
- **WHEN** 用户修改记忆内容或标签
- **THEN** 系统更新记忆条目并重新生成 embedding

#### Scenario: 删除记忆
- **WHEN** 用户删除记忆
- **THEN** 系统标记记忆为已删除状态（软删除）

#### Scenario: 批量管理记忆
- **WHEN** 用户选择多个记忆条目
- **THEN** 系统支持批量添加标签、批量删除操作

### Requirement: 向量检索
系统 SHALL 使用向量检索技术匹配语义相关的记忆。

#### Scenario: 语义检索
- **WHEN** 用户输入查询
- **THEN** 系统返回语义相似度最高的 Top-K 记忆条目

#### Scenario: 混合检索
- **WHEN** 用户同时使用关键词和语义检索
- **THEN** 系统结合全文检索和向量检索返回结果

#### Scenario: 过滤检索
- **WHEN** 用户指定标签或时间范围
- **THEN** 系统在过滤后的记忆集合中进行检索

### Requirement: Embedding 策略
系统 SHALL 支持多种 Embedding 模型和策略配置。

#### Scenario: 配置 Embedding 模型
- **WHEN** 用户选择 Embedding 模型
- **THEN** 系统使用指定模型生成向量

#### Scenario: 批量重新生成 Embedding
- **WHEN** 用户切换 Embedding 模型
- **THEN** 系统后台异步重新生成所有记忆的 Embedding

#### Scenario: 本地 Embedding
- **WHEN** 用户启用本地 Embedding
- **THEN** 系统使用本地模型生成向量，无需调用外部 API

### Requirement: 记忆分类与组织
系统 SHALL 支持记忆的自动分类和手动组织。

#### Scenario: 自动分类
- **WHEN** 系统创建新记忆
- **THEN** 系统自动分析内容并分配类别标签

#### Scenario: 手动分类
- **WHEN** 用户为记忆添加自定义标签
- **THEN** 系统保存标签并更新记忆元数据

#### Scenario: 记忆集合
- **WHEN** 用户创建记忆集合
- **THEN** 系统支持将多个记忆组织到集合中

### Requirement: 记忆导入导出
系统 SHALL 支持记忆的导入和导出功能。

#### Scenario: 导出记忆
- **WHEN** 用户导出记忆
- **THEN** 系统生成 JSON 格式的记忆文件

#### Scenario: 导入记忆
- **WHEN** 用户导入记忆文件
- **THEN** 系统解析文件并创建记忆条目

## Non-Functional Requirements

### Performance
- 记忆检索延迟 < 500ms
- Embedding 生成时间 < 2s
- 支持最多 100,000 条记忆
- 向量检索 Top-10 延迟 < 100ms

### Scalability
- 支持多种向量数据库（本地、Qdrant、Pinecone）
- 支持分片存储（按时间或类别）
- 支持增量索引更新

### Reliability
- 记忆数据持久化存储
- Embedding 生成失败自动重试（最多 3 次）
- 向量索引定期备份

### Accuracy
- 语义检索准确率 > 85%
- 自动分类准确率 > 80%
- 重复记忆检测准确率 > 90%

## Data Model

### Memory
```typescript
interface Memory {
  memory_id: string;            // 唯一标识符
  content: string;              // 记忆内容
  summary: string;              // 内容摘要
  embedding: number[];          // 向量表示
  metadata: MemoryMetadata;     // 元数据
  tags: string[];               // 标签列表
  category: MemoryCategory;     // 记忆类别
  source: MemorySource;         // 来源信息
  created_at: string;           // 创建时间（ISO 8601）
  updated_at: string;           // 更新时间（ISO 8601）
  deleted_at?: string;          // 删除时间（ISO 8601）
}

interface MemoryMetadata {
  session_id?: string;          // 来源会话 ID
  user_id?: string;             // 用户 ID
  importance: number;           // 重要性评分（0.0-1.0）
  access_count: number;         // 访问次数
  last_accessed_at?: string;    // 最后访问时间
  embedding_model: string;      // 使用的 Embedding 模型
  embedding_version: string;    // Embedding 版本
}

enum MemoryCategory {
  Preference = "preference",    // 用户偏好
  Knowledge = "knowledge",      // 知识信息
  Context = "context",          // 上下文信息
  Task = "task",                // 任务相关
  Other = "other"               // 其他
}

interface MemorySource {
  type: SourceType;
  reference: string;            // 来源引用（会话 ID、文件路径等）
  timestamp: string;            // 来源时间戳
}

enum SourceType {
  Session = "session",          // 来自会话
  Manual = "manual",            // 手动创建
  Import = "import",            // 导入
  System = "system"             // 系统生成
}
```

### MemoryCollection
```typescript
interface MemoryCollection {
  collection_id: string;        // 唯一标识符
  name: string;                 // 集合名称
  description: string;          // 集合描述
  memory_ids: string[];         // 记忆 ID 列表
  created_at: string;           // 创建时间（ISO 8601）
  updated_at: string;           // 更新时间（ISO 8601）
}
```

### EmbeddingConfig
```typescript
interface EmbeddingConfig {
  model: EmbeddingModel;        // Embedding 模型
  dimension: number;            // 向量维度
  strategy: EmbeddingStrategy;  // Embedding 策略
  batch_size: number;           // 批量处理大小
  cache_enabled: boolean;       // 是否启用缓存
}

enum EmbeddingModel {
  OpenAIAda002 = "openai-ada-002",           // OpenAI text-embedding-ada-002
  OpenAI3Small = "openai-3-small",           // OpenAI text-embedding-3-small
  OpenAI3Large = "openai-3-large",           // OpenAI text-embedding-3-large
  LocalBGESmall = "local-bge-small",         // 本地 BGE-small
  LocalBGEBase = "local-bge-base",           // 本地 BGE-base
  LocalBGELarge = "local-bge-large"          // 本地 BGE-large
}

enum EmbeddingStrategy {
  Immediate = "immediate",      // 立即生成
  Deferred = "deferred",        // 延迟生成
  Batch = "batch"               // 批量生成
}
```

### SearchQuery
```typescript
interface SearchQuery {
  query: string;                // 查询文本
  limit: number;                // 返回数量（默认 10）
  threshold?: number;           // 相似度阈值（0.0-1.0）
  filters?: SearchFilters;      // 过滤条件
  mode: SearchMode;             // 检索模式
}

interface SearchFilters {
  tags?: string[];              // 标签过滤
  categories?: MemoryCategory[]; // 类别过滤
  date_range?: DateRange;       // 时间范围过滤
  importance_min?: number;      // 最小重要性
}

interface DateRange {
  start: string;                // 开始时间（ISO 8601）
  end: string;                  // 结束时间（ISO 8601）
}

enum SearchMode {
  Semantic = "semantic",        // 语义检索
  Keyword = "keyword",          // 关键词检索
  Hybrid = "hybrid"             // 混合检索
}

interface SearchResult {
  memory: Memory;               // 记忆条目
  score: number;                // 相似度评分（0.0-1.0）
  highlights?: string[];        // 高亮片段
}
```

## State Machine

### Memory State Transitions
```
Created ────────────────────────────────────────────────────────┐
  │                                                              │
  │ generate_embedding()                                         │
  ▼                                                              │
EmbeddingPending ───────────────────────────────────────────────┤
  │                                                              │
  │ embedding_completed()                                        │
  ▼                                                              │
Active ─────────────────────────────────────────────────────────┤
  │                                                              │
  │ update_content()                                             │
  ▼                                                              │
EmbeddingPending ───────────────────────────────────────────────┘

Active ─────────────────────────────────────────────────────────┐
  │                                                              │
  │ soft_delete()                                                │
  ▼                                                              │
Deleted ────────────────────────────────────────────────────────┤
  │                                                              │
  │ restore()                                                    │
  └──────────────────────────────────────────────────────────> Active
```

### Embedding Generation State
```
Pending ────────────────────────────────────────────────────────┐
  │                                                              │
  │ start_generation()                                           │
  ▼                                                              │
Generating ─────────────────────────────────────────────────────┤
  │                                                              │
  │ generation_completed()                                       │
  ▼                                                              │
Completed

Generating ─────────────────────────────────────────────────────┐
  │                                                              │
  │ generation_failed()                                          │
  ▼                                                              │
Failed ─────────────────────────────────────────────────────────┤
  │                                                              │
  │ retry()                                                      │
  └──────────────────────────────────────────────────────────> Pending
```

## Error Scenarios

### Memory Creation Errors
- **VALIDATION_ERROR**: 记忆内容为空或超过 10,000 字符
- **DUPLICATE_MEMORY**: 检测到重复记忆（相似度 > 0.95）
- **QUOTA_EXCEEDED**: 记忆数量超过限制（100,000 条）
- **EMBEDDING_GENERATION_FAILED**: Embedding 生成失败

### Search Errors
- **INVALID_QUERY**: 查询文本为空或格式错误
- **INDEX_NOT_READY**: 向量索引未就绪
- **SEARCH_TIMEOUT**: 检索超时（5 秒）
- **NO_RESULTS**: 未找到匹配的记忆

### Embedding Errors
- **MODEL_NOT_AVAILABLE**: Embedding 模型不可用
- **API_ERROR**: 外部 API 调用失败
- **DIMENSION_MISMATCH**: 向量维度不匹配
- **BATCH_PROCESSING_FAILED**: 批量处理失败

### Import/Export Errors
- **INVALID_FORMAT**: 导入文件格式错误
- **FILE_TOO_LARGE**: 文件大小超过限制（100 MB）
- **PARSING_ERROR**: 文件解析失败
- **EXPORT_FAILED**: 导出失败

### Vector Database Errors
- **CONNECTION_FAILED**: 向量数据库连接失败
- **INDEX_CORRUPTED**: 索引损坏
- **STORAGE_FULL**: 存储空间不足
- **SYNC_FAILED**: 数据同步失败

## Design References

- 后端架构：`docs/backend-architecture.md` § 记忆聚合设计
- API 规范：`docs/api-specification.md` § 记忆系统 API
- 事件协议：`docs/event-protocol.md` § 记忆系统事件
