# Agent Teams

## ADDED Requirements

### Requirement: 团队状态可视化
系统 SHALL 展示团队的当前状态和历史状态转换。

#### Scenario: 查看团队状态
- **WHEN** 用户查看团队详情
- **THEN** 系统显示团队当前状态
- **AND** 显示状态转换历史（创建时间、启动时间、完成时间）

#### Scenario: 查看团队成员状态
- **WHEN** 用户查看 running 状态的团队
- **THEN** 系统显示每个成员的当前任务和状态
- **AND** 显示成员之间的消息交互

### Requirement: 团队消息总线
系统 SHALL 提供团队成员之间的消息传递机制。

#### Scenario: 发送任务分配消息
- **WHEN** Leader 分配任务给 Worker
- **THEN** 系统发送 task_assigned 消息
- **AND** 消息包含任务摘要（< 100 tokens）

#### Scenario: 发送进度更新消息
- **WHEN** Worker 更新任务进度
- **THEN** 系统发送 progress_update 消息
- **AND** 消息包含进度摘要和产出物引用

#### Scenario: 发送支援请求消息
- **WHEN** Worker 需要其他成员支援
- **THEN** 系统发送 help_request 消息
- **AND** 消息包含问题描述和上下文引用

#### Scenario: 发送结果就绪消息
- **WHEN** Worker 完成任务
- **THEN** 系统发送 result_ready 消息
- **AND** 消息包含结果摘要和产出物引用

#### Scenario: 订阅团队消息
- **WHEN** 成员订阅团队消息
- **THEN** 系统推送相关消息
- **AND** 支持按消息类型过滤

### Requirement: 共享工作区
系统 SHALL 提供团队共享工作区用于存储产出物。

#### Scenario: 创建工作区
- **WHEN** 团队激活时
- **THEN** 系统创建共享工作区
- **AND** 写入任务目标和约束条件

#### Scenario: 写入产出物
- **WHEN** 成员完成子任务
- **THEN** 系统将产出物写入工作区
- **AND** 产出物包含名称、类型、内容

#### Scenario: 读取产出物
- **WHEN** 成员需要其他成员的产出
- **THEN** 系统返回指定产出物
- **AND** 支持按需读取（不自动广播）

#### Scenario: 列出产出物
- **WHEN** 成员查看工作区
- **THEN** 系统返回所有产出物列表
- **AND** 包含产出物元数据（名称、类型、创建时间）

### Requirement: 团队项目关联
系统 SHALL 支持团队关联到项目。

#### Scenario: 创建关联项目的团队
- **WHEN** 用户创建团队并指定项目
- **THEN** 系统关联团队到项目
- **AND** 团队可访问项目知识库

#### Scenario: 创建无项目的团队
- **WHEN** 用户创建团队但不指定项目
- **THEN** 系统创建独立团队
- **AND** 团队只能访问全局知识库

#### Scenario: 查看项目的所有团队
- **WHEN** 用户查看项目详情
- **THEN** 系统显示该项目的所有团队
- **AND** 包含团队状态和创建时间

### Requirement: 团队自动归档
系统 SHALL 自动归档长时间未活动的已完成团队。

#### Scenario: 自动归档完成的团队
- **WHEN** 团队处于 completed 状态超过 7 天
- **THEN** 系统自动执行归档流程
- **AND** 通知用户团队已归档

#### Scenario: 自动归档失败的团队
- **WHEN** 团队处于 failed 状态超过 7 天
- **THEN** 系统自动执行归档流程
- **AND** 保留失败日志供调试

## MODIFIED Requirements

### Requirement: 团队生命周期管理
系统 SHALL 支持创建、配置、激活、暂停、恢复、停止、归档与删除团队。

#### Scenario: 创建团队草稿
- **WHEN** 用户创建新团队
- **THEN** 系统创建 `draft` 状态的团队
- **AND** 团队包含基本信息（名称、模式）

#### Scenario: 配置团队
- **WHEN** 用户为 `draft` 团队添加角色和配置
- **THEN** 系统更新团队配置
- **AND** 配置完成后团队状态变为 `ready`

#### Scenario: 激活团队
- **WHEN** 用户激活 `ready` 状态的团队
- **THEN** 系统将团队状态变为 `running`
- **AND** 记录团队启动时间

#### Scenario: 暂停团队
- **WHEN** 用户暂停 `running` 状态的团队
- **THEN** 系统将团队状态变为 `paused`
- **AND** 保留团队当前执行状态

#### Scenario: 恢复团队
- **WHEN** 用户恢复 `paused` 状态的团队
- **THEN** 系统将团队状态变为 `running`
- **AND** 从暂停点继续执行

#### Scenario: 停止团队
- **WHEN** 用户停止 `running` 状态的团队
- **THEN** 系统将团队状态变为 `completed` 或 `failed`
- **AND** 记录团队完成时间

#### Scenario: 归档团队
- **WHEN** 用户归档 `completed` 或 `failed` 状态的团队
- **THEN** 系统执行归档流程（知识提升、工作区清理）
- **AND** 团队状态变为 `archived`

#### Scenario: 删除团队
- **WHEN** 用户删除 `draft` 或 `archived` 状态的团队
- **THEN** 系统删除团队及相关数据
- **AND** 无法删除其他状态的团队

### Requirement: 任务分派可视化
系统 SHALL 显示团队任务分派与执行进度，并支持实时消息流展示。

#### Scenario: 查看任务分派
- **WHEN** 团队任务开始执行
- **THEN** 系统展示任务负责人与进度状态
- **AND** 显示成员之间的消息交互历史

#### Scenario: 查看实时消息流
- **WHEN** 团队正在运行
- **THEN** 系统实时展示成员之间的消息
- **AND** 支持按消息类型过滤（任务分配、进度更新、支援请求、结果就绪）
