# OpenViking Context OS Integration Design

## Context

Claw Box 当前处于前端原型与后端基础域模型阶段，具备会话、Agent Teams、记忆系统、技能管理、MCP 集成的产品目标，但尚未形成可用于生产的统一上下文引擎。

OpenViking 已提供成熟的上下文数据库能力，包括文件系统范式、分层读取、目录递归检索与可观测轨迹、会话级异步记忆提交。

本设计选择将 OpenViking 作为独立 Context OS，Claw Box 作为编排与控制台层，通过明确边界与稳定契约实现长期最优架构。

## Goals / Non-Goals

- Goals:
  - 建立 Claw Box 与 OpenViking 的双中枢架构，职责硬隔离
  - 定义稳定的 Context Gateway API（对 UI 与 Agent 统一）
  - 实现检索可解释、记忆可追踪、任务可观测
  - 支持多 Agent 协作下的上下文预算与策略控制
  - 形成可扩展的权限与治理模型（tenant/project/session）

- Non-Goals:
  - 不在本 change 中重写 OpenViking 内部检索算法
  - 不在本 change 中交付完整前端视觉方案
  - 不在本 change 中处理多租户计费系统
  - 不在本 change 中替换全部现有业务 API（仅新增/兼容上下文通道）
  - 不在 V1 引入跨租户共享知识空间（Shared Space）
  - 不在 V1 引入可视化策略 DSL
  - 不在 V1 将 Memory Scoring 强耦合进 commit 状态机

## Architecture

系统采用四层架构：

1. Presentation Layer（Tauri/React）
   - Context Explorer
   - Retrieval Trace
   - Commit Console

2. Orchestration Layer（Claw Box Core Service, Rust）
   - 会话编排、Agent 调度、策略决策
   - Context Gateway（统一上下文入口）
   - 审计与指标聚合

3. Context OS Layer（OpenViking Service）
   - 上下文存储与组织（viking://）
   - 分层读取、搜索、浏览、提交、任务化处理

4. Observability & Policy Layer
   - 指标、日志、检索回放、权限边界、数据保留策略

## Tech Stack

- Core Service: Rust（Axum）
- UI: React + TypeScript + Tauri
- Context OS: OpenViking（HTTP API）
- Storage:
  - OpenViking 内部存储（memory/resource/skill）
  - Claw Box 策略与映射表（SQLite）
- Observability:
  - 结构化日志（JSON）
  - 指标（Prometheus 格式兼容）
  - Trace ID 跨服务贯穿

## Decisions

- Decision: 采用双中枢 + Gateway，而非 UI/Agent 直连 OpenViking
  - Why: 保证协议稳定、权限统一、审计完整
  - Alternatives considered: 直连实现快但失控；Core 自建引擎会重复造轮子

- Decision: 对外统一 `ctx://`，内部映射 `viking://`
  - Why: 避免业务层与 OpenViking URI 强耦合
  - Alternatives considered: 直接暴露 `viking://`，短期快但长期迁移成本高

- Decision: commit 默认异步任务化（`wait=false`）
  - Why: 避免长请求阻塞与超时重试风暴
  - Alternatives considered: 同步 commit 逻辑简单但体验差

- Decision: 检索结果必须可解释
  - Why: 可调试、可运营、可优化
  - Alternatives considered: 仅返回片段会形成黑盒

- Decision: Shared Space 延后到 V2
  - Why: 避免提前引入复杂鉴权与数据隔离联动

- Decision: Memory Scoring 作为独立异步巡检任务
  - Why: 保持 commit 状态机简单，避免影响主路径时延

- Decision: 客户端通信采用控制面/数据面分离
  - 控制面：Tauri Command 发起配置变更、任务触发与重试命令
  - 数据面：本地 HTTP + SSE 承载检索查询、提交任务状态与轨迹事件
  - 命令与事件必须共享 `trace_id`

## Risks / Trade-offs

- 双中枢提高系统复杂度 -> 通过严格边界、契约测试和可观测治理控制
- 上下游协议漂移 -> 固定 API 版本并增加 contract test
- 异步任务带来最终一致性延迟 -> UI 明确展示状态并提供重试

## Migration Plan

1. 建立 Context Gateway（仅新增，不破坏旧接口）
2. 落地 session map + message append + async commit
3. 接入 search/read/browse 并替换原型上下文卡片数据源
4. 增加检索解释字段与前端 Trace 面板
5. 打通策略层与审计
6. 切换旧记忆读写路径到新网关并保留回滚开关

Rollback:
- 保留 `context_gateway_enabled=false` 回退旧路径
- 回滚顺序：UI -> Core 路由 -> 适配层

## Open Questions

- V1 是否默认开启 `hit.explain` 字段过滤开关（建议默认关闭，仅高敏租户开启）？
- Memory Scoring 调度模式采用 cron、事件触发，或混合模式（推荐）？

---
