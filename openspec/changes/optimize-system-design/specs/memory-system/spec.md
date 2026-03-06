## ADDED Requirements

### Requirement: Embedding 生成策略
系统 SHALL 使用指定的 Embedding 模型生成向量表示。

#### Scenario: 生成 Embedding
- **WHEN** 用户创建记忆条目
- **THEN** 系统使用 text-embedding-3-small 模型生成 1536 维向量
- **AND** 异步执行，不阻塞响应

#### Scenario: Embedding 生成失败
- **WHEN** Embedding 生成失败
- **THEN** 系统记录错误日志
- **AND** 记忆条目保存但标记为未索引
- **AND** 后台自动重试（最多 3 次）

### Requirement: 向量检索配置
系统 SHALL 支持配置向量检索参数。

#### Scenario: 语义检索阈值
- **WHEN** 用户查询与记忆的余弦相似度 > 0.7
- **THEN** 系统将该记忆包含在搜索结果中

#### Scenario: Top-K 检索
- **WHEN** 用户执行语义搜索
- **THEN** 系统返回相似度最高的 Top-K 条记忆（默认 10 条）
- **AND** 按相似度降序排列

### Requirement: 记忆标签化
系统 SHALL 支持为记忆添加和管理标签。

#### Scenario: 添加标签
- **WHEN** 用户为记忆添加标签
- **THEN** 系统保存标签并更新记忆
- **AND** 标签可用于过滤和分类

#### Scenario: 按标签过滤
- **WHEN** 用户按标签过滤记忆
- **THEN** 系统返回包含指定标签的所有记忆

## MODIFIED Requirements

### Requirement: 长期记忆存储
系统 SHALL 支持存储、检索与管理长期记忆条目，包括 Embedding 生成和向量索引。

#### Scenario: 存储记忆
- **WHEN** 用户标记某段对话为记忆
- **THEN** 系统提取内容、生成摘要
- **AND** 异步生成 embedding 并存储为记忆条目
- **AND** 发送 memory.created 事件

#### Scenario: 检索记忆
- **WHEN** 用户发起新对话
- **THEN** 系统根据语义相似度检索相关记忆
- **AND** 注入到上下文中（相似度 > 0.7）

### Requirement: 记忆管理
系统 SHALL 允许用户查看、编辑、删除与标签化记忆条目，并支持重新生成 Embedding。

#### Scenario: 查看记忆列表
- **WHEN** 用户打开记忆管理界面
- **THEN** 系统展示所有记忆条目，包含摘要、标签、来源与相似度

#### Scenario: 编辑记忆
- **WHEN** 用户修改记忆内容或标签
- **THEN** 系统更新记忆条目
- **AND** 重新生成 embedding
- **AND** 更新向量索引
