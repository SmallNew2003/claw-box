## ADDED Requirements

### Requirement: MCP 连接管理
系统 SHALL 支持创建、管理和监控 MCP 服务器连接。

#### Scenario: 创建连接
- **WHEN** 用户配置 MCP 服务器连接
- **THEN** 系统验证连接配置
- **AND** 建立连接并索引可用工具
- **AND** 发送 mcp.connected 事件

#### Scenario: 连接失败
- **WHEN** MCP 服务器连接失败
- **THEN** 系统返回 MCP_CONNECTION_FAILED 错误
- **AND** 记录失败原因

### Requirement: MCP 工具调用协议
系统 SHALL 支持调用 MCP 工具并监控执行。

#### Scenario: 调用工具
- **WHEN** 用户调用 MCP 工具
- **THEN** 系统发送请求到 MCP 服务器
- **AND** 发送 mcp.tool_called 事件
- **AND** 返回工具执行结果

#### Scenario: 工具调用超时
- **WHEN** MCP 工具调用超过 30 秒
- **THEN** 系统终止调用
- **AND** 返回 MCP_CALL_TIMEOUT 错误
