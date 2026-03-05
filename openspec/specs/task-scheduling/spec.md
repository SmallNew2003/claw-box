# 定时任务管理

## ADDED Requirements

### Requirement: 定时任务创建
系统 SHALL 支持创建定时任务并配置执行计划。

#### Scenario: 新建定时任务
- **WHEN** 用户填写任务名称与 Cron 表达式并保存
- **THEN** 系统创建任务并显示下一次执行时间

### Requirement: 执行记录与状态
系统 SHALL 展示定时任务的执行历史与当前状态。

#### Scenario: 查看任务执行结果
- **WHEN** 用户打开任务详情
- **THEN** 系统展示最近的执行结果与状态
