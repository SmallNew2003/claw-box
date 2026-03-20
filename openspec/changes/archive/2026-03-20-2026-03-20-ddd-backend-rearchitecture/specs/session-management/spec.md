## MODIFIED Requirements

### Requirement: 会话上下文状态
系统 SHALL 显示会话的模型、状态与上下文占用信息，并通过 Session 上下文内应用服务统一编排状态流转。

#### Scenario: 查看会话状态
- **WHEN** 用户选中某个会话
- **THEN** 系统显示该会话的模型名称、运行状态与上下文占用

#### Scenario: 上下文占用警告
- **WHEN** 会话上下文占用超过 80%
- **THEN** 系统显示警告提示用户清理历史消息

#### Scenario: 分层迁移后的状态一致性
- **WHEN** Session/Runtime 完成上下文分层迁移
- **THEN** 系统仍应返回与迁移前一致的会话状态语义
- **AND** 会话状态变化必须通过 Session Application 层统一编排

#### Scenario: 会话领域规则归位
- **WHEN** 系统处理会话标题、消息内容和执行状态流转
- **THEN** 标题与消息有效性校验必须在 Session 领域对象中表达
- **AND** 运行中不可重复执行等状态规则必须由 Session 领域对象强制

#### Scenario: 运行时调用边界
- **WHEN** 会话应用服务触发一次运行时执行
- **THEN** 会话应用服务只能通过运行时端口触发执行
- **AND** 运行时工具调用细节不得泄漏为会话领域规则
