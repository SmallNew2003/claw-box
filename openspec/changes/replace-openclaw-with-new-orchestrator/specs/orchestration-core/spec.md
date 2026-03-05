# 编排内核（Orchestration Core）

## ADDED Requirements

### Requirement: 多智能体协作内核
系统 SHALL 支持多智能体协作与任务路由，并提供可插拔的策略层。

#### Scenario: 任务路由
- **WHEN** 用户提交一个协作任务
- **THEN** 系统根据策略将任务分配给多个智能体并跟踪状态

### Requirement: 事件总线与日志流
系统 SHALL 提供统一事件总线，支持实时状态与日志回传。

#### Scenario: 事件回传
- **WHEN** 智能体输出日志或问题
- **THEN** 事件总线实时推送到控制台
