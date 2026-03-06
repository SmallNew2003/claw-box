# Implementation Tasks

## 1. 数据库设计与迁移
- [ ] 1.1 设计团队表 schema（id, name, project_id, state, mode, config, timestamps）
- [ ] 1.2 设计知识库表 schema（id, content, summary, embedding, scope, importance, state, timestamps）
- [ ] 1.3 设计消息表 schema（id, team_id, type, from_agent, to_agent, payload, timestamp）
- [ ] 1.4 设计工作区表 schema（team_id, goal, constraints）
- [ ] 1.5 设计工作区产出物表 schema（id, team_id, name, type, content, created_at）
- [ ] 1.6 创建数据库迁移脚本
- [ ] 1.7 添加索引（team_id, project_id, scope, state, importance, timestamp）

## 2. 后端数据模型
- [ ] 2.1 实现 Team 模型（backend/src/models/team.rs）
- [ ] 2.2 实现 TeamState 枚举和状态转换逻辑
- [ ] 2.3 实现 KnowledgeItem 模型（backend/src/models/knowledge.rs）
- [ ] 2.4 实现 KnowledgeLifecycle 状态机
- [ ] 2.5 实现 TeamMessage 模型（backend/src/models/message.rs）
- [ ] 2.6 实现 Workspace 和 Artifact 模型（backend/src/models/workspace.rs）

## 3. 团队生命周期服务
- [ ] 3.1 实现 TeamLifecycleService（backend/src/services/agent_teams.rs）
- [ ] 3.2 实现 create_team 方法（创建 draft 状态团队）
- [ ] 3.3 实现 configure_team 方法（draft → ready）
- [ ] 3.4 实现 activate_team 方法（ready → running）
- [ ] 3.5 实现 pause_team 方法（running → paused）
- [ ] 3.6 实现 resume_team 方法（paused → running）
- [ ] 3.7 实现 stop_team 方法（running → completed/failed）
- [ ] 3.8 实现 archive_team 方法（completed/failed → archived）
- [ ] 3.9 实现 delete_team 方法（仅 draft/archived）
- [ ] 3.10 实现状态转换验证逻辑

## 4. 知识库服务
- [ ] 4.1 实现 KnowledgeService（backend/src/services/knowledge.rs）
- [ ] 4.2 实现 store 方法（存储知识到指定作用域）
- [ ] 4.3 实现 search 方法（向量相似度检索）
- [ ] 4.4 实现 list 方法（按过滤条件列出知识）
- [ ] 4.5 实现 update 方法（更新知识）
- [ ] 4.6 实现 delete 方法（删除知识）
- [ ] 4.7 实现 promote 方法（提升知识层级）
- [ ] 4.8 实现 evaluate_importance 方法（LLM 评分）
- [ ] 4.9 实现 cleanup 方法（按生命周期策略清理）
- [ ] 4.10 实现向量 embedding 生成（集成 embedding 模型）

## 5. 知识生命周期管理
- [ ] 5.1 实现知识状态转换定时任务（active → cold → archived）
- [ ] 5.2 实现访问时状态恢复逻辑
- [ ] 5.3 实现容量管理和 LRU 淘汰
- [ ] 5.4 实现团队归档时的知识处理逻辑
- [ ] 5.5 实现项目归档时的知识处理逻辑

## 6. 消息总线服务
- [ ] 6.1 实现 MessageBusService（backend/src/services/message_bus.rs）
- [ ] 6.2 实现 send 方法（发送消息）
- [ ] 6.3 实现 subscribe 方法（订阅消息，返回 AsyncIterator）
- [ ] 6.4 实现 get_history 方法（获取历史消息）
- [ ] 6.5 实现消息持久化逻辑
- [ ] 6.6 实现消息广播机制（支持点对点和广播）

## 7. 共享工作区服务
- [ ] 7.1 实现 WorkspaceService（backend/src/services/workspace.rs）
- [ ] 7.2 实现 create 方法（创建工作区）
- [ ] 7.3 实现 read 方法（读取工作区）
- [ ] 7.4 实现 write_artifact 方法（写入产出物）
- [ ] 7.5 实现 read_artifact 方法（读取产出物）
- [ ] 7.6 实现 list_artifacts 方法（列出产出物）
- [ ] 7.7 实现工作区清理逻辑（团队归档时）

## 8. 团队 API 端点
- [ ] 8.1 实现 POST /api/teams（创建团队）
- [ ] 8.2 实现 GET /api/teams（列出团队）
- [ ] 8.3 实现 GET /api/teams/:id（获取团队详情）
- [ ] 8.4 实现 PUT /api/teams/:id/configure（配置团队）
- [ ] 8.5 实现 POST /api/teams/:id/activate（激活团队）
- [ ] 8.6 实现 POST /api/teams/:id/pause（暂停团队）
- [ ] 8.7 实现 POST /api/teams/:id/resume（恢复团队）
- [ ] 8.8 实现 POST /api/teams/:id/stop（停止团队）
- [ ] 8.9 实现 POST /api/teams/:id/archive（归档团队）
- [ ] 8.10 实现 DELETE /api/teams/:id（删除团队）

## 9. 知识库 API 端点
- [ ] 9.1 实现 POST /api/knowledge（存储知识）
- [ ] 9.2 实现 GET /api/knowledge/search（检索知识）
- [ ] 9.3 实现 GET /api/knowledge（列出知识）
- [ ] 9.4 实现 GET /api/knowledge/:id（获取知识详情）
- [ ] 9.5 实现 PUT /api/knowledge/:id（更新知识）
- [ ] 9.6 实现 DELETE /api/knowledge/:id（删除知识）
- [ ] 9.7 实现 POST /api/knowledge/:id/promote（提升知识层级）
- [ ] 9.8 实现 POST /api/knowledge/:id/evaluate（评估重要性）
- [ ] 9.9 实现 POST /api/knowledge/cleanup（清理知识）

## 10. 消息 API 端点
- [ ] 10.1 实现 POST /api/teams/:id/messages（发送消息）
- [ ] 10.2 实现 GET /api/teams/:id/messages（获取消息历史）
- [ ] 10.3 实现 GET /api/teams/:id/messages/stream（SSE 订阅消息）

## 11. 工作区 API 端点
- [ ] 11.1 实现 GET /api/teams/:id/workspace（获取工作区）
- [ ] 11.2 实现 POST /api/teams/:id/workspace/artifacts（写入产出物）
- [ ] 11.3 实现 GET /api/teams/:id/workspace/artifacts（列出产出物）
- [ ] 11.4 实现 GET /api/teams/:id/workspace/artifacts/:artifactId（读取产出物）

## 12. 前端 API 客户端
- [ ] 12.1 实现 TeamsAPI 客户端（src-ui/api/teams.ts）
- [ ] 12.2 实现 KnowledgeAPI 客户端（src-ui/api/knowledge.ts）
- [ ] 12.3 实现 MessageAPI 客户端（src-ui/api/messages.ts）
- [ ] 12.4 实现 WorkspaceAPI 客户端（src-ui/api/workspace.ts）
- [ ] 12.5 添加 TypeScript 类型定义

## 13. 前端团队管理组件
- [ ] 13.1 实现 TeamList 组件（显示团队列表）
- [ ] 13.2 实现 TeamDetail 组件（显示团队详情和状态）
- [ ] 13.3 实现 TeamLifecycleControls 组件（生命周期操作按钮）
- [ ] 13.4 实现 TeamStateTimeline 组件（状态转换历史）
- [ ] 13.5 实现 TeamMemberStatus 组件（成员状态展示）
- [ ] 13.6 实现 TeamMessageStream 组件（实时消息流）
- [ ] 13.7 实现 TeamWorkspace 组件（工作区产出物展示）

## 14. 前端知识库管理组件
- [ ] 14.1 实现 KnowledgeList 组件（显示知识列表）
- [ ] 14.2 实现 KnowledgeDetail 组件（显示知识详情）
- [ ] 14.3 实现 KnowledgeSearch 组件（知识检索界面）
- [ ] 14.4 实现 KnowledgeScopeFilter 组件（作用域过滤）
- [ ] 14.5 实现 KnowledgeLifecycleIndicator 组件（生命周期状态指示器）
- [ ] 14.6 实现 KnowledgeImportanceScore 组件（重要性评分展示）
- [ ] 14.7 实现 KnowledgePromoteDialog 组件（知识提升对话框）

## 15. 前端状态管理
- [ ] 15.1 创建 teamsStore（Zustand）
- [ ] 15.2 创建 knowledgeStore（Zustand）
- [ ] 15.3 创建 messagesStore（Zustand）
- [ ] 15.4 创建 workspaceStore（Zustand）
- [ ] 15.5 实现 SSE 消息订阅和状态更新

## 16. 测试
- [ ] 16.1 编写团队生命周期服务单元测试
- [ ] 16.2 编写知识库服务单元测试
- [ ] 16.3 编写消息总线服务单元测试
- [ ] 16.4 编写工作区服务单元测试
- [ ] 16.5 编写 API 端点集成测试
- [ ] 16.6 编写前端组件测试
- [ ] 16.7 编写 E2E 测试（团队创建 → 运行 → 归档流程）

## 17. 文档
- [ ] 17.1 更新 API 文档（docs/api-specification.md）
- [ ] 17.2 更新架构文档（docs/architecture.md）
- [ ] 17.3 编写知识库使用指南（docs/knowledge-guide.md）
- [ ] 17.4 编写团队生命周期使用指南（docs/team-lifecycle-guide.md）

## 18. 性能优化
- [ ] 18.1 优化向量检索性能（添加索引、批量查询）
- [ ] 18.2 优化消息推送性能（连接池、批量推送）
- [ ] 18.3 优化知识库清理性能（批量删除、异步处理）
- [ ] 18.4 添加缓存层（热门知识、团队状态）

## 19. 监控和日志
- [ ] 19.1 添加团队生命周期事件日志
- [ ] 19.2 添加知识库操作日志
- [ ] 19.3 添加消息传递日志
- [ ] 19.4 添加性能监控指标（API 响应时间、知识库大小、消息延迟）

## 20. 部署和迁移
- [ ] 20.1 准备数据库迁移脚本
- [ ] 20.2 编写部署文档
- [ ] 20.3 执行数据库迁移
- [ ] 20.4 验证功能正常运行
