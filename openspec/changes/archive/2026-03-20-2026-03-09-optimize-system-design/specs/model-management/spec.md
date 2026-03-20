## ADDED Requirements

### Requirement: 模型配置管理
系统 SHALL 支持配置和管理多个模型。

#### Scenario: 添加模型配置
- **WHEN** 用户添加新模型配置
- **THEN** 系统保存模型 ID、名称、API 端点、Token 限制
- **AND** 验证 API 密钥有效性

#### Scenario: 切换模型
- **WHEN** 用户切换会话的模型
- **THEN** 系统更新会话的 modelId
- **AND** 重新计算上下文限制

### Requirement: 模型能力检测
系统 SHALL 支持检测模型的能力特性。

#### Scenario: 检测流式输出支持
- **WHEN** 系统调用模型 API
- **THEN** 系统检测模型是否支持流式输出
- **AND** 根据能力选择调用方式
