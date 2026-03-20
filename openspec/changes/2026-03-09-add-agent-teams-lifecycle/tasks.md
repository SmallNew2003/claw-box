# Implementation Tasks

## 1. 前置能力确认
- [ ] 1.1 确认单 Agent 运行时基础能力已可用（会话执行、消息流、工具调用）
- [ ] 1.2 确认模型管理能力已可用（供应商、模型、模式、思考级别）
- [ ] 1.3 确认 Skills 和 MCP 已有独立能力边界，不在本提案内重复建设

## 2. 数据模型与迁移
- [ ] 2.1 设计团队表 schema（id, name, project_id, state, mode, config, timestamps）
- [ ] 2.2 设计消息表 schema（id, team_id, type, from_agent, to_agent, payload, timestamp）
- [ ] 2.3 设计工作区表 schema（team_id, goal, constraints）
- [ ] 2.4 设计工作区产出物表 schema（id, team_id, name, type, content, created_at）
- [ ] 2.5 创建数据库迁移脚本和必要索引

## 3. 后端团队能力
- [ ] 3.1 实现 Team 模型与 TeamState 状态转换逻辑
- [ ] 3.2 实现 TeamLifecycleService（创建、配置、激活、暂停、恢复、停止、归档、删除）
- [ ] 3.3 实现 MessageBusService（发送、订阅、历史记录）
- [ ] 3.4 实现 WorkspaceService（创建、读取、写入产出物、列出产出物、归档清理）

## 4. API 与前端闭环
- [ ] 4.1 实现团队 API（创建、列表、详情、生命周期操作）
- [ ] 4.2 实现消息 API（发送、历史、流式订阅）
- [ ] 4.3 实现工作区 API（详情、产出物写入、列表、读取）
- [ ] 4.4 实现团队管理 UI（列表、详情、生命周期控制、消息流、工作区）

## 5. 验证与文档
- [ ] 5.1 编写团队生命周期、消息总线、工作区的单元与集成测试
- [ ] 5.2 编写前端基础组件测试和端到端团队流程测试
- [ ] 5.3 更新 API 文档与团队生命周期使用指南
