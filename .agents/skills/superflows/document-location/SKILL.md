---
name: document-location
description: Use when creating or modifying any documentation file. Ensures documents are placed in the correct location according to project standards.
---

# Document Location Checker

## 触发条件

必须使用此 skill：
- 创建任何 .md 文档前
- 修改 project.md 或其他规范文档前
- 不确定文档应该放在哪里时
- 准备添加代码示例或详细规范到文档时

## 核心原则

文档位置决策遵循"关注点分离"：
- 项目上下文 vs 详细规范
- 需求定义 vs 技术实现
- 通用规范 vs 项目特定

## 文档位置决策树

### 步骤 1：识别文档类型

问自己以下问题：

**Q1: 这是项目上下文信息吗？**
- 项目定位、目标
- 技术栈列表（不含详细用法）
- 架构模式概述（不含实现细节）
- 领域上下文、职责边界
- 外部依赖、重要约束

→ 是：放在 `openspec/project.md`
→ 否：继续 Q2

**Q2: 这是开发规范吗？**
- 代码风格、命名规范
- 框架使用规范（如 MyBatis-Flex）
- 编程最佳实践
- 代码示例

→ 是：放在 `openspec/specs/development-standards/spec.md`
→ 否：继续 Q3

**Q3: 这是数据库设计规范吗？**
- 表命名、字段命名
- 主键、外键规范
- 审计字段、软删除
- ER 图规范

→ 是：放在 `openspec/specs/database-design-standards/spec.md`
→ 否：继续 Q4

**Q4: 这是数据库 Schema 规范吗？**
- 主键生成策略（如雪花算法）
- 数据类型规范
- 索引策略

→ 是：放在 `openspec/specs/database-schema/spec.md`
→ 否：继续 Q5

**Q5: 这是功能需求规范吗？**
- 系统 SHALL/SHOULD/MAY 做什么
- 业务场景和验收标准
- 功能行为定义

→ 是：放在 `openspec/specs/<capability>/spec.md`
→ 否：继续 Q6

**Q6: 这是技术设计文档吗？**
- 架构决策
- 技术方案选型
- 实现细节
- 数据流、时序图

→ 是：放在 `openspec/changes/<change-id>/design.md`（活跃）或 `openspec/changes/archive/*/design.md`（已归档）
→ 否：继续 Q7

**Q7: 这是长期架构模式吗？**
- 跨多个 capability 的设计模式
- 重大技术选型决策
- 统一的技术标准

→ 是：放在 `openspec/specs/<capability>/patterns.md`
→ 否：继续 Q8

**Q8: 这是实现指南或用户文档吗？**
- API 文档
- 操作手册
- 开发指南
- 系统架构设计

→ 是：放在 `docs/`
→ 否：可能不需要创建文档，直接回复用户

### 步骤 2：验证内容是否适合目标位置

**如果目标是 project.md**，检查：
- [ ] 是高层概述，不是详细规范
- [ ] 没有代码示例
- [ ] 没有详细的使用说明
- [ ] 没有具体的实现细节

**如果目标是 specs/**，检查：
- [ ] 使用 Requirements 和 Scenario 格式
- [ ] 定义"做什么"，不是"怎么做"
- [ ] 可验证、可测试

**如果目标是 changes/**，检查：
- [ ] 是临时文档（实现完成后会归档）
- [ ] 包含变更上下文
- [ ] 有明确的 change-id

### 步骤 3：执行检查

**禁止的操作**：
- [错误] 在 project.md 中添加详细的开发规范
- [错误] 在 project.md 中添加代码示例
- [错误] 在 project.md 中添加详细的数据库设计规范
- [错误] 创建 SUMMARY.md、ANALYSIS.md 等临时文档
- [错误] 在 docs/plans/ 中创建新文档（已废弃）

**正确的操作**：
- [正确] 在 project.md 中引用详细规范的位置
- [正确] 在 specs/ 中定义详细规范
- [正确] 在 changes/ 中创建技术设计文档
- [正确] 直接回复用户，不创建临时文档

## 使用示例

### 示例 1：用户要求添加 MyBatis 使用规范

```
用户: "把 MyBatis-Flex 的使用规范写到文档里"

Agent 思考：
1. 这是开发规范吗？→ 是
2. 目标位置：openspec/specs/development-standards/spec.md
3. 验证：使用 Requirements 和 Scenario 格式
4. 执行：在 development-standards/spec.md 中添加规范
5. 在 project.md 中添加引用：
   "详细的开发规范请参考：openspec/specs/development-standards/spec.md"
```

### 示例 2：用户要求更新项目介绍

```
用户: "更新一下项目介绍，说明我们的技术栈"

Agent 思考：
1. 这是项目上下文信息吗？→ 是
2. 目标位置：openspec/project.md
3. 验证：只写技术栈列表，不写详细用法
4. 执行：在 project.md 的 Tech Stack 章节更新
```

### 示例 3：用户要求添加数据库表命名规范

```
用户: "把数据库表命名规范加到文档里"

Agent 思考：
1. 这是数据库设计规范吗？→ 是
2. 目标位置：openspec/specs/database-design-standards/spec.md
3. 验证：使用 Requirements 和 Scenario 格式
4. 执行：在 database-design-standards/spec.md 中添加规范
```

## 与其他 Skills 的配合

### 与 openspec-planning 配合

openspec-planning 在创建文档前会调用此 skill：
```
openspec-planning 准备创建文档
  ↓
调用 document-location skill
  ↓
确认文档位置
  ↓
创建文档
```

### 与 using-superpowers 配合

using-superpowers 在检测到文档操作时会提醒调用此 skill：
```
用户: "把这个规范写到文档里"
  ↓
using-superpowers 检测到文档操作
  ↓
调用 document-location skill
  ↓
确认位置后执行
```

## 快速参考

| 内容类型 | 目标位置 |
|---------|---------|
| 项目定位、技术栈列表 | `openspec/project.md` |
| 开发规范、代码示例 | `openspec/specs/development-standards/spec.md` |
| 数据库设计规范 | `openspec/specs/database-design-standards/spec.md` |
| 数据库 Schema 规范 | `openspec/specs/database-schema/spec.md` |
| 功能需求 | `openspec/specs/<capability>/spec.md` |
| 技术设计 | `openspec/changes/<change-id>/design.md` |
| 长期架构模式 | `openspec/specs/<capability>/patterns.md` |
| API 文档、操作手册 | `docs/` |

## 详细规则参考

完整的文档组织规范请参考：`openspec/DOCUMENTATION.md`

## Red Flags

如果你有以下想法，立即停止：

| 想法 | 问题 |
|------|------|
| "这个规范很短，直接放 project.md" | 规范应该在 specs/ 中 |
| "这个代码示例放 project.md 方便查看" | 代码示例应该在 specs/ 中 |
| "创建个 SUMMARY.md 总结一下" | 禁止创建临时文档 |
| "不确定放哪，先放 docs/plans/" | docs/plans/ 已废弃 |
| "这个设计文档放 docs/ 吧" | 应该放 changes/<change-id>/design.md |
