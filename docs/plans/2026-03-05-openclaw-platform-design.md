# OpenClaw 类全能智能体平台设计草案（独立前后端）

日期：2026-03-05

## 背景与目标

构建一个独立的全能智能体平台（前端 + 后端），以 OpenClaw 架构为基础进行增强，优先满足个人助手场景，后续可演进为多节点协作平台。MVP 以单机一体化、零或极少中间件依赖为部署目标，同时保持对 OpenClaw 配置格式的兼容。

### 目标

- 提供 Agent Teams、记忆系统、Skills、MCP 的闭环能力
- 单机一体化部署，默认零中间件依赖，可选外部化
- Web + 桌面端（优先 mac）共用 API
- 兼容 OpenClaw 的技能、MCP、配置格式，降低迁移成本

### 非目标（MVP）

- 工具市场、供应商中心、成本分析
- 多租户/多用户权限体系
- 大规模多节点协作与分布式调度

## 总体架构

- Core Service（Rust）：统一业务 API（HTTP/JSON），内部事件流与任务编排
- UI 层：Tauri 桌面端（mac 优先）+ Web 前端，共用 API
- 存储：默认 SQLite + 本地向量检索；可配置切换到 PostgreSQL + Qdrant/pgvector + Redis
- 兼容层：OpenClaw 配置与技能适配器

## 核心模块

1. Agent Teams
   - Team / Role / Flow 定义
   - 基于 Claude Flow 形态：顺序/并行节点、工具调用、子任务
   - 运行态映射为 Run / Step

2. 记忆系统
   - 长期记忆：MemoryItem（原文/摘要/embedding/标签/来源）
   - 短期记忆：Session 上下文窗口
   - 检索：向量检索 + 规则筛选 + 摘要压缩

3. Skills
   - SkillPackage（元信息、版本、入口、权限）
   - SkillConfig（参数、密钥引用、运行约束）
   - 执行器：MCP 执行器 + HTTP/Command 执行器

4. MCP
   - McpServer 连接配置
   - McpTool 索引与映射
   - McpCallLog 调用记录

5. 会话与运行
   - Session / Run / Step 结构
   - 统一事件流推送 UI（SSE/WebSocket）

## 数据模型（核心表/结构）

- Session
- Run
- Step
- Team
- Role
- Flow
- MemoryItem
- MemoryLink
- SkillPackage
- SkillConfig
- McpServer
- McpTool
- McpCallLog
- ConfigProfile（兼容 OpenClaw，扩展字段置于 extensions）

## 运行流程

1. UI 发起 Run
2. Orchestrator 解析 Team Flow，生成 Step
3. Step 调用 Skill/MCP/工具
4. 写入 Run/Step 日志
5. 事件流回推 UI（状态、日志、结果）

## API 形态

- 外部：HTTP/JSON（供 Web/Tauri）
- 内部：事件流（SSE/WebSocket），预留 gRPC 扩展

## 存储与部署

- 默认：单二进制 + SQLite + 本地向量检索
- 可选：配置切换到 PostgreSQL + Qdrant/pgvector + Redis
- 配置分层：local / workspace / runtime
- 数据导出：JSON/SQL 备份

## MVP 范围

- Agent Teams（Claude Flow 形态）
- 记忆系统（长期 + 短期 + 向量检索 + 摘要）
- Skills（本地技能管理 + MCP 映射）
- MCP（连接管理、工具索引、调用日志）
- 单机一体化部署
- 基础控制台 UI（团队/会话/运行/记忆/技能/MCP）

## 迭代路线

- 工具市场
- 自定义供应商中心
- 成本分析（按 Run/Step/模型聚合）
- 多节点协作与调度
- 多用户/权限/审计

## 风险与缓解

- Rust 生态集成成本高 → MVP 限定能力范围，预留扩展点
- 本地向量检索性能与质量 → 预留外部向量库切换
- OpenClaw 兼容性不完全 → 适配层分离，扩展字段可回退

## 未决问题

- 具体 Claude Flow 节点与状态模型需要进一步对标
- 本地向量检索实现的选择与基准
- Tauri 与 Web UI 共享组件与状态管理的策略
