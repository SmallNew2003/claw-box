## Context
项目从 demo 进入正式开发，需要在继承 szadmin 底座能力的同时，建立可移植的模块化架构，降低未来跨项目复用成本。

## Goals / Non-Goals
- Goals:
  - 形成可移植模块骨架与统一契约
  - 将宿主能力通过适配层解耦
  - 支持 MyBatis-Flex 与东方通中间件
- Non-Goals:
  - 不在本次变更中重写全部业务模块
  - 不更换 szadmin 现有底座能力实现

## Decisions
- Decision: 以 szadmin 为宿主，通过 ports/adapters 接入 RBAC、租户、审计、流程
- Decision: 业务模块只依赖 shared-kernel 与应用层契约，不依赖宿主实现
- Decision: MyBatis-Flex 仓储实现放在 infrastructure 层，数据库方言在仓储层隔离

## Alternatives Considered
- 直接在 szadmin 上进行 DDD 化重构
  - 缺点：耦合度高、改造范围大、收益难以沉淀为可移植模块

## Risks / Trade-offs
- 适配层设计不足可能导致宿主能力泄露到业务模块
- 多数据库方言隔离需要统一规范，否则会出现 SQL 不一致

## Migration Plan
1) 先搭建模块骨架与 shared-kernel
2) 抽取宿主适配层并接入核心能力
3) 选取一个核心模块做样板并验证复用性
4) 逐步迁移其他模块

## Open Questions
- 样板模块优先选择哪个业务域
- 东方通消息协议与 topic 规范是否已有统一标准
