# Change: 后端 DDD 架构重构

## Metadata
- Created: 2026-03-20T00:00:00Z
- Affected Specs:
  - backend-ddd-architecture (added: 2026-03-20T00:00:00Z)
  - session-management (modified: 2026-03-20T00:00:00Z)

## Why
当前后端目录与依赖方向处于过渡状态，存在上下文边界模糊与应用层依赖基础设施实现的问题。需要通过一次性架构重构建立可持续演进的 DDD 基线。

## What Changes
- 新增后端 DDD 架构能力规范，定义上下文边界、分层职责与依赖方向。
- 重构 `session` 与 `runtime` 上下文内部结构，统一为 `domain/application/ports/adapters/interfaces`。
- 抽象运行时工具调用端口，移除应用层对具体工具运行时实现的直接依赖。
- 将顶层旧分层中的业务模块迁移至上下文内并保留必要兼容入口。

## Impact
- Affected specs: backend-ddd-architecture, session-management
- Affected code:
  - `backend/src/lib.rs`
  - `backend/src/contexts/session/**`
  - `backend/src/contexts/runtime/**`
  - `backend/src/api/**`
  - `backend/src/interfaces/**`
  - `backend/src/models/**`
  - `backend/src/services/**`
