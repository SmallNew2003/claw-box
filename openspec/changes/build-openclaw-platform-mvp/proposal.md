# Change: 构建 OpenClaw 风格独立智能体平台 MVP

## Why
现有 Claw Box 以桌面控制台原型为主，尚未形成可独立部署的智能体平台。为了验证 Agent Teams、记忆系统、Skills 与 MCP 的完整闭环，需要建立独立前后端平台，并支持单机一体化部署与后续扩展。

## What Changes
- 新增独立 Core Service 与统一 API（Web/Tauri 共用）
- 引入 Agent Teams、记忆系统、Skills、MCP 的 MVP 能力
- 默认单机一体化部署，外部存储与向量检索可选
- 兼容 OpenClaw 的技能与 MCP 配置格式

## Impact
- Affected specs: agent-platform
- Affected code: platform/, apps/web, apps/desktop
- Affected docs: docs/plans
