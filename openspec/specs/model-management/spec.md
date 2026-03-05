# 模型管理

## ADDED Requirements

### Requirement: 模型切换
系统 SHALL 支持在会话级别切换模型配置。

#### Scenario: 切换模型
- **WHEN** 用户在模型选择器中切换模型
- **THEN** 系统更新该会话的模型配置并显示新模型名称

### Requirement: 模型可用性
系统 SHALL 显示模型的可用状态与基础配置项。

#### Scenario: 显示模型状态
- **WHEN** 系统加载模型列表
- **THEN** 每个模型显示可用/不可用状态与配置入口
