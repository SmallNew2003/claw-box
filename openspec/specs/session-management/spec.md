# 会话管理

## ADDED Requirements

### Requirement: 多会话生命周期
系统 SHALL 允许用户创建、重命名、归档与删除会话。

#### Scenario: 创建会话
- **WHEN** 用户点击新建会话
- **THEN** 系统创建一个新会话并出现在列表中

#### Scenario: 归档会话
- **WHEN** 用户对会话执行归档操作
- **THEN** 会话从默认列表移入归档列表

### Requirement: 会话上下文状态
系统 SHALL 显示会话的模型、状态与上下文占用信息。

#### Scenario: 查看会话状态
- **WHEN** 用户选中某个会话
- **THEN** 系统显示该会话的模型名称、运行状态与上下文占用
