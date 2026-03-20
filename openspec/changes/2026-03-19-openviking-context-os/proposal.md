# Change: 引入 OpenViking 作为 Context OS 并建设 Context Gateway

## Metadata
- Created: 2026-03-19T00:00:00Z
- Affected Specs:
  - context-gateway (added: 2026-03-19T00:00:00Z)

## Why
当前 Claw Box 在会话、记忆、技能、资源上下文能力上仍处于原型阶段，缺少可生产化的统一上下文引擎。
引入 OpenViking 作为 Context OS，并在 Core Service 建立 Context Gateway，可在不牺牲长期演进空间的前提下快速形成可观测、可治理、可解释的上下文闭环。

## What Changes
- 新增 Context Gateway 作为 Core 统一上下文入口
- 对接 OpenViking 搜索、读取、浏览、消息写入、异步提交与任务查询能力
- 统一 URI 规范为 `ctx://`，在网关层映射为 `viking://`
- 强制引入检索可解释字段（`retrieval_path`、`rank_features`）与 `trace_id` 贯穿
- 定义 commit 异步任务状态机与幂等约束
- 建立 `ContextSessionMap`、`CommitTaskSnapshot`、`ContextPolicyProfile` 数据模型
- 新增前端信息架构入口：Context Explorer、Retrieval Trace、Commit Console
- 对齐客户端双通道通信模型：控制面命令走 Tauri Command，检索与事件流走本地 HTTP + SSE

## Impact
- Affected specs: `context-gateway`
- Affected code:
  - `backend/`（新增 gateway、adapter、policy、audit、metrics 模块）
  - `src-ui/`（新增上下文浏览、检索轨迹、提交任务面板）
  - OpenViking 接口适配层（HTTP client + schema contract）
- Security:
  - 强制 tenant/project 边界
  - 审计日志与 trace 全链路
- Performance:
  - 异步 commit 降低长请求超时风险
  - 分层读取与预算控制降低 token 消耗
- Rollout:
  - 以 feature flag 渐进启用，保留旧路径回退开关

## Non-Goals (V1)
- 不引入跨租户共享知识空间（Shared Space）
- 不引入可视化策略 DSL
- 不将 Memory Scoring 强耦合到 commit 状态机
- 不改造 OpenViking 内部检索算法
