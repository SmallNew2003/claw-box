# MCP 集成

## ADDED Requirements

### Requirement: MCP 连接管理
系统 SHALL 支持添加、启用与移除 MCP 连接。

#### Scenario: 添加 MCP 连接
- **WHEN** 用户输入 MCP 服务器地址并保存
- **THEN** 系统建立连接并显示连接状态

### Requirement: MCP 工具展示
系统 SHALL 展示 MCP 连接提供的工具与资源列表。

#### Scenario: 查看工具清单
- **WHEN** 用户打开某个 MCP 连接
- **THEN** 系统显示该连接的工具与资源信息
