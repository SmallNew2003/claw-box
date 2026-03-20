# Change: 建立长期架构治理规范（DDD + 六边形 + CQRS + Outbox）

## Metadata

- Created: 2026-03-20T10:30:00Z
- Affected Specs:
  - architecture-governance (new)

## Why

项目将持续迭代并引入更多能力（Runtime、Session、Tools、Teams、Memory、Model）。

如果缺少统一的架构治理规范，后续实现容易出现边界不清、依赖方向混乱、状态重复建模和跨模块耦合，导致开发效率与可维护性持续下降。

## What Changes

1. 新增 `architecture-governance` 能力规范，定义长期适用的架构约束。
2. 明确限界上下文划分原则与上下文交互边界。
3. 明确分层规则：Domain / Application / Ports / Adapters / Interfaces。
4. 明确读写分离、事件发布与 Outbox 一致性要求。
5. 明确工程注释与契约文档标准（Rust `///`、TypeScript TSDoc）。

## Impact

- Affected specs:
  - `architecture-governance`（新增）

- Affected code:
  - 本次主要是规范变更，不直接引入功能代码。
  - 后续所有能力实现和重构将以该规范为约束基线。

### Breaking changes

无运行时破坏性变更；属于治理规范新增。

### Migration

采用渐进迁移：先按新增规范约束新代码，再逐步重构存量模块。
