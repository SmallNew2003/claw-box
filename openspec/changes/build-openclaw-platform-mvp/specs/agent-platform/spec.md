## ADDED Requirements

### Requirement: Agent Teams 编排与运行
系统 SHALL 提供 Team/Role/Flow 的定义与运行能力，支持顺序与并行执行，并产出可追踪的 Run/Step 记录。

#### Scenario: 运行团队流程
- **WHEN** 用户在控制台触发某个 Team Flow
- **THEN** 系统生成 Run 与 Step 记录并持续更新状态

### Requirement: 记忆系统（短期 + 长期）
系统 SHALL 支持会话短期记忆与本地长期记忆，并提供向量检索与摘要能力。

#### Scenario: 检索长期记忆
- **WHEN** 用户基于关键词或向量搜索记忆
- **THEN** 系统返回匹配的记忆项及摘要

### Requirement: Skills 管理与执行
系统 SHALL 支持技能包的注册与配置，并可通过执行器触发技能运行。

#### Scenario: 注册技能
- **WHEN** 用户创建或导入 Skill 配置
- **THEN** 系统持久化该技能并可用于 Run

### Requirement: MCP 连接与工具索引
系统 SHALL 支持 MCP Server 连接管理与工具索引同步，并保留调用记录。

#### Scenario: 同步 MCP 工具
- **WHEN** 用户配置 MCP Server 并触发同步
- **THEN** 系统记录工具索引并可被 Skills 使用

### Requirement: 单机一体化部署
系统 SHALL 支持单机启动（无外部中间件依赖），并允许切换到外部存储与向量库。

#### Scenario: 本地一体化运行
- **WHEN** 用户启动 Core Service
- **THEN** 系统使用本地存储完成基本运行

### Requirement: Web + 桌面客户端
系统 SHALL 提供 Web 与桌面端（mac 优先）控制台，共用统一 API。

#### Scenario: Web 与桌面共享 API
- **WHEN** 用户通过 Web 或桌面端访问控制台
- **THEN** 系统展示一致的 Teams/Run/Memory/Skills/MCP 视图
