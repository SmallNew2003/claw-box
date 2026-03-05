# Change: 建立可移植模块化架构并以 szadmin 为宿主

## Why
当前 demo 即将进入正式开发，需要在保持底座能力的同时，实现模块可复用与可移植。

## What Changes
- 定义模块化骨架与分层边界（domain/application/interfaces/infrastructure）
- 引入 shared-kernel 与标准 ports/adapters 契约
- 以 szadmin 作为宿主系统，通过适配层接入 RBAC/租户/审计/流程
- 统一 MyBatis-Flex 与东方通中间件的适配策略

## Impact
- Affected specs: modular-architecture
- Affected code: 后端模块结构、shared-kernel、host-szadmin 适配层
