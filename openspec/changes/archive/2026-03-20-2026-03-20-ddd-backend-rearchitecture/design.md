# 后端 DDD 重构设计

## Context
当前后端同时存在 `contexts` 与 `api/services/models/interfaces` 并列分层，导致依赖方向不一致、上下文边界不清晰。运行时应用层还直接依赖基础设施实现，弱化了端口-适配器约束。

## Goals / Non-Goals
- Goals:
  - 将后端收敛为按 bounded context 组织的 DDD 架构。
  - 强制依赖方向为 `interfaces -> application -> domain`，并通过 `ports` 连接 `adapters`。
  - 迁移并保留现有会话、运行时、HTTP/SSE、命令调用能力。
- Non-Goals:
  - 不在本次变更中引入持久化数据库或分布式部署能力。
  - 不在本次变更中改变对外业务行为语义。

## Architecture
后端按 `session` 与 `runtime` 两个上下文闭环组织，每个上下文包含 `domain/application/ports/adapters/interfaces`。顶层模块仅保留上下文聚合与最小跨域公共定义，不再承载业务分层。

## Tech Stack
- Rust
- serde / serde_json
- 端口-适配器（Hexagonal）
- 上下文内 in-memory adapter（当前阶段）

## Decisions
- Decision: 移除顶层 `api/services/models` 业务归属，迁入对应上下文。
  - Alternatives considered:
    - 保留旧目录并做薄包装：迁移成本低，但长期边界继续模糊。
    - 一次性迁入上下文：迁移成本高，但能形成稳定边界与清晰依赖方向。

- Decision: 运行时应用层通过 `ToolInvokerPort` 调用工具，不再直接依赖工具注册实现。
  - Alternatives considered:
    - 继续直接依赖具体 `ToolRegistry`：实现快，但应用层与基础设施耦合。
    - 端口抽象 + 适配器实现：可测试性与可替换性更好。

- Decision: 对外入口保持兼容函数签名，内部改为委托上下文应用服务。
  - Alternatives considered:
    - 直接改动全部调用方：收敛彻底但短期改动面大。
    - 保留兼容入口并逐步迁移：可控推进，风险更低。

## Risks / Trade-offs
- 大量模块移动导致编译错误概率升高 -> 按阶段迁移并在每阶段运行测试。
- 兼容层保留会延迟完全收敛 -> 在任务中明确后续清理步骤。
- 现有测试与目录结构强绑定 -> 同步迁移测试并补充分层边界测试。

## Migration Plan
1. 重构模块与导出边界，建立上下文内统一目录。
2. 引入 `ToolInvokerPort`，完成运行时应用层去基础设施耦合。
3. 迁移 API/命令处理器到上下文 `interfaces`，更新调用引用。
4. 清理顶层旧分层目录与转发导出。
5. 修复并补充测试，验证行为兼容。

## Open Questions
- 无。

---
