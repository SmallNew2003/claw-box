## ADDED Requirements

### Requirement: 多会话生命周期
系统 SHALL 允许用户创建、重命名、归档与删除会话。

#### Scenario: 创建会话
- **WHEN** 用户点击新建会话
- **THEN** 系统创建一个新会话并出现在列表中
- **AND** 会话状态为 idle
- **AND** 会话 ID 自动生成

#### Scenario: 重命名会话
- **WHEN** 用户修改会话名称
- **THEN** 系统更新会话名称
- **AND** 更新 updatedAt 时间戳

#### Scenario: 归档会话
- **WHEN** 用户对会话执行归档操作
- **THEN** 会话从默认列表移入归档列表
- **AND** 会话状态变为 archived

#### Scenario: 删除会话
- **WHEN** 用户删除会话
- **THEN** 系统删除会话及其所有消息
- **AND** 发送 session.deleted 事件

### Requirement: 会话并发限制
系统 SHALL 限制单用户同时运行的会话数不超过 10 个。

#### Scenario: 超出并发限制
- **WHEN** 用户尝试启动第 11 个会话
- **THEN** 系统返回错误 "SESSION_LIMIT_EXCEEDED"
- **AND** 提示用户暂停或关闭其他会话

### Requirement: 消息管理
系统 SHALL 支持添加、查看、编辑和删除消息。

#### Scenario: 添加消息
- **WHEN** 用户发送消息
- **THEN** 系统将消息添加到会话
- **AND** 更新会话的 updatedAt 时间戳
- **AND** 发送 session.message_added 事件

#### Scenario: 编辑消息
- **WHEN** 用户编辑历史消息
- **THEN** 系统更新消息内容
- **AND** 标记消息为已编辑

#### Scenario: 删除消息
- **WHEN** 用户删除消息
- **THEN** 系统从会话中移除该消息
- **AND** 重新计算上下文占用

## MODIFIED Requirements

### Requirement: 会话上下文状态
系统 SHALL 显示会话的模型、状态与上下文占用信息，包括 Token 使用量和溢出警告。

#### Scenario: 查看会话状态
- **WHEN** 用户选中某个会话
- **THEN** 系统显示该会话的模型名称、运行状态与上下文占用
- **AND** 显示 Token 使用量（已用/总量）

#### Scenario: 上下文溢出警告
- **WHEN** 会话 Token 使用量超过 80%
- **THEN** 系统显示警告提示
- **AND** 建议用户归档或清理消息
