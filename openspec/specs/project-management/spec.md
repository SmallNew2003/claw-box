# 项目管理

## ADDED Requirements

### Requirement: 项目配置
系统 SHALL 支持创建与管理项目配置。

#### Scenario: 新建项目
- **WHEN** 用户输入项目名称与路径
- **THEN** 系统创建项目并关联到会话

### Requirement: 项目上下文
系统 SHALL 展示项目的会话、技能与模型配置概览。

#### Scenario: 查看项目概览
- **WHEN** 用户打开项目详情
- **THEN** 系统显示关联的会话与配置摘要
