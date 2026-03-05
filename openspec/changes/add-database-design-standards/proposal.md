# Change: 添加数据库设计规范

## Why

当前数据库 ER 图存在多个设计不一致问题：
- 外键关系缺失和不明确，导致数据完整性风险
- 数据类型不统一（如 status 字段混用 varchar 和 int）
- 字段命名不一致（`_code` 后缀使用不规范）
- 跨文件实体定义重复，维护困难
- 审计字段不完整，部分表缺少 created_by/updated_by

需要建立统一的数据库设计规范，确保数据模型的一致性、可维护性和数据完整性。

## What Changes

- 新增数据库设计规范文档，涵盖：
  - 命名约定（表名、字段名、索引名、约束名）
  - 数据类型标准（状态码、时间戳、数值、文本）
  - 外键规范（关系定义、级联策略）
  - 审计字段标准（创建人、更新人、时间戳）
  - 索引策略（主键、唯一索引、普通索引）
  - 软删除和版本控制
  - ER 图绘制规范
- 修复所有现有 ER 图文件，使其符合新规范：
  - `docs/sql/erd_resource_application.mmd`
  - `docs/sql/erd_flowlong.mmd`
  - `docs/sql/erd_operations_audit.mmd`
  - `docs/sql/erd_user_access.mmd`
  - `docs/sql/erd_resource_management.mmd`
  - `docs/sql/erd_component_management.mmd`

## Impact

- Affected specs: 新增 `database-design-standards` 规范
- Affected code: 
  - 所有 ER 图文件 `docs/sql/*.mmd`
  - 未来所有数据库表设计需遵循此规范
- Breaking changes: 无（仅规范化现有设计，不改变已实现的表结构）
