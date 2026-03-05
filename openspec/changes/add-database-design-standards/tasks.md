# Implementation Tasks

## 1. 文档规范

- [ ] 1.1 创建数据库设计规范文档
- [ ] 1.2 在 `openspec/specs/database-design-standards/spec.md` 中定义完整规范
- [ ] 1.3 更新 `openspec/project.md` 引用数据库设计规范

## 2. ER 图修复

- [ ] 2.1 修复 `docs/sql/erd_resource_application.mmd`
  - [ ] 2.1.1 统一状态字段类型为 varchar
  - [ ] 2.1.2 补充缺失的外键关系（MODEL_APPLICATION.model_id、KNOWLEDGE_APPLICATION.kb_id 等）
  - [ ] 2.1.3 修正 AGENT_APPLICATION 命名混淆
  - [ ] 2.1.4 添加缺失的实体定义或引用说明
  - [ ] 2.1.5 补充审计字段（created_by、updated_by）

- [ ] 2.2 修复 `docs/sql/erd_flowlong.mmd`
  - [ ] 2.2.1 明确 FLW_EXT_INSTANCE 与 FLW_HIS_INSTANCE 的外键关系
  - [ ] 2.2.2 确保所有外键字段标注 FK

- [ ] 2.3 修复 `docs/sql/erd_operations_audit.mmd`
  - [ ] 2.3.1 明确标注 OPERATION_LOG.operator_id 为 FK
  - [ ] 2.3.2 明确标注 WARNING.handle_user_id 为 FK

- [ ] 2.4 修复 `docs/sql/erd_user_access.mmd`
  - [ ] 2.4.1 确保所有外键关系清晰标注
  - [ ] 2.4.2 统一状态字段命名（is_delete 改为 is_deleted）

- [ ] 2.5 修复 `docs/sql/erd_resource_management.mmd`
  - [ ] 2.5.1 补充缺失的关系（COMPUTE_RESOURCE 与 MODEL 的资源分配）
  - [ ] 2.5.2 确保所有外键字段标注 FK

- [ ] 2.6 修复 `docs/sql/erd_component_management.mmd`
  - [ ] 2.6.1 补充 COMPONENT.owner_id 的外键关系
  - [ ] 2.6.2 统一 AGENT_智能体 实体定义（引用 erd_resource_management.mmd）

## 3. 验证

- [ ] 3.1 检查所有 ER 图语法正确性
- [ ] 3.2 验证所有外键关系完整性
- [ ] 3.3 验证命名规范一致性
- [ ] 3.4 验证数据类型一致性

## 4. 文档更新

- [ ] 4.1 在 README 或相关文档中添加数据库设计规范链接
- [ ] 4.2 创建 ER 图修复说明文档（如需要）
