# backend-ddd-architecture Specification

## Purpose
TBD - created by archiving change 2026-03-20-ddd-backend-rearchitecture. Update Purpose after archive.
## Requirements
### Requirement: 上下文闭环分层
后端系统 SHALL 以 bounded context 为顶层组织单元，每个上下文内部必须包含 `domain`、`application`、`ports`、`adapters` 与 `interfaces` 分层。

#### Scenario: 新增业务能力进入上下文
- **WHEN** 团队新增会话或运行时相关业务能力
- **THEN** 该能力的领域模型与用例必须定义在对应上下文内
- **AND** 不得在顶层创建并列业务分层目录承载该能力

### Requirement: 单向依赖约束
后端系统 SHALL 强制依赖方向为 `interfaces -> application -> domain`，并通过 `ports` 连接 `adapters`。

#### Scenario: 应用层调用外部能力
- **WHEN** 应用层需要访问存储或工具执行能力
- **THEN** 应用层必须依赖端口抽象而非具体适配器实现
- **AND** 具体实现由适配器在组装阶段注入

### Requirement: 顶层导出收敛
后端系统 SHALL 将顶层导出收敛为上下文聚合与最小共享模块，不再导出与上下文并列的业务分层模块。

#### Scenario: 模块导出检查
- **WHEN** 代码加载后端库入口模块
- **THEN** 顶层模块仅暴露上下文边界与共享能力
- **AND** 不再通过顶层 `api`、`services`、`models` 暴露业务语义

### Requirement: 兼容性迁移策略
后端系统 SHALL 在重构期间提供受控兼容入口，确保现有调用方在迁移窗口内保持行为兼容。

#### Scenario: 旧调用路径仍被使用
- **WHEN** 调用方仍通过旧入口访问会话或运行时能力
- **THEN** 系统应通过兼容委托保证返回语义与错误码一致
- **AND** 在迁移任务完成后可移除兼容入口

