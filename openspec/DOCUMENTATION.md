# 文档组织规范

<!-- 规范版本: v1.0.0 -->
<!-- 最后更新: 2026-03-06 -->
<!-- 维护者: 开发团队 -->

## 目录职责划分

### openspec/ - 规范与需求（What to build）

**用途**：OpenSpec 流程管理，定义"要构建什么"

**内容**：
- `project.md` - 项目上下文、技术栈、约束（唯一真相源）
- `AGENTS.md` - AI Agent 工作指南
- `CHANGELOG.md` - 规范变更历史
- `specs/` - 功能规范（需求 + 场景）
- `changes/` - 变更提案（待实现）
- `changes/archive/` - 已完成变更

**规则**：
- 所有功能需求必须在 `specs/` 中定义
- 新功能/破坏性变更必须先创建 `changes/` 提案
- 实现完成后归档到 `changes/archive/`
- 不包含实现细节、代码示例、操作手册

**project.md 内容规范**：
- [正确] 项目定位、目标、技术栈列表、架构模式概述、领域上下文、职责边界、外部依赖、重要约束
- [错误] 详细的开发规范（应放在 `specs/development-standards/spec.md`）
- [错误] 详细的数据库设计规范（应放在 `specs/database-design-standards/spec.md`）
- [错误] 代码示例和最佳实践（应放在 `specs/` 或 `docs/`）

**创建或修改文档前必须检查**：
1. 这是项目上下文吗？→ `openspec/project.md`
2. 这是开发规范吗？→ `openspec/specs/development-standards/spec.md`
3. 这是数据库设计规范吗？→ `openspec/specs/database-design-standards/spec.md`
4. 这是功能需求吗？→ `openspec/specs/<capability>/spec.md`
5. 这是技术设计吗？→ `openspec/changes/<change-id>/design.md`

### docs/ - 设计与文档（How to build）

**用途**：技术设计、实现指南、用户文档

**内容**：
- `2026-02-28-system-architecture-design.md` - 系统架构设计
- `2026-02-01-product-design.md` - 产品设计
- `sql/` - 数据库设计（ER 图）
- `plans/` - 临时设计文档（brainstorming 输出）
- `api/` - API 文档（待创建）
- `guides/` - 开发指南与操作手册（待创建）

**规则**：
- 包含技术实现细节、代码示例、架构图
- 面向开发者与用户的文档
- 不创建临时总结文档（`*_SUMMARY.md`、`*_ANALYSIS.md` 等）

### bj-gover-llm-service/ - 后端代码

**文档位置**：
- `README.md` - 项目说明
- `modules/*/README.md` - 模块说明（如需要）
- `src/main/resources/db/changelog/` - Liquibase 数据库变更

### bj-gover-llm-admin/ - 前端代码

**文档位置**：
- `README.md` - 项目说明
- `docs/` - 前端开发文档（如需要）

---

## 文档流转规则

### 新功能开发流程

```
1. brainstorming（如需求不明确）
   输出：docs/plans/[date]-[topic]-design.md

2. openspec-planning
   输入：docs/plans/ 或用户需求
   输出：openspec/changes/[change-id]/
     ├── proposal.md（需求说明）
     ├── design.md（技术设计）
     ├── tasks.md（实现清单）
     └── specs/[capability]/spec.md（规范定义）

3. 代码实现
   - 按 tasks.md 执行
   - 更新 docs/ 中的技术文档（如 architecture.md）
   - 创建/更新 API 文档

4. 归档
   - 移动到 openspec/changes/archive/[date]-[change-id]/
   - 更新 openspec/specs/[capability]/spec.md（合并变更）
   - 更新 openspec/CHANGELOG.md
```

### 文档清理规则

**保留**：
- 已归档的变更（`openspec/changes/archive/`）
- 正式的技术文档（`docs/architecture.md` 等）
- 项目必需文档（`README.md`、`CHANGELOG.md`）

**删除**：
- 临时分析文档（`*_SUMMARY.md`、`*_ANALYSIS.md`、`*_REVIEW.md`）
- 过时的设计文档（已被新版本替代）
- brainstorming 输出（实现完成后可删除 `docs/plans/`）

**不创建**：
- 总结文档（内容应在对话中回复用户）
- 重复文档（内容应整合到现有文档）

---

## 快速参考

| 问题 | 放在哪里 |
|------|---------|
| 这个功能要做什么？ | `openspec/specs/` |
| 这个功能怎么实现？ | `docs/` 或代码注释 |
| 新功能提案 | `openspec/changes/` |
| 系统架构设计 | `docs/2026-02-28-system-architecture-design.md` |
| 数据库设计 | `docs/sql/` 和 Liquibase changelog |
| API 接口文档 | `docs/api/`（待创建） |
| 用户操作手册 | `docs/guides/`（待创建） |
| 项目约束与规范 | `openspec/project.md` |
| Java 开发规范 | `openspec/project.md` § 后端开发规范 |
| Agent 工作规则 | `AGENTS.md` |

---

## 文档类型说明

### 需求文档（openspec/specs/）

**格式**：
```markdown
## ADDED Requirements
### Requirement: [需求名称]
系统 SHALL [需求描述]

#### Scenario: [场景名称]
- GIVEN [前置条件]
- WHEN [触发条件]
- THEN [预期结果]
```

**特点**：
- 使用 SHALL/SHOULD/MAY 表示强制程度
- 场景驱动，可验证
- 不包含实现细节

### 技术设计文档（docs/）

**格式**：
- Markdown 或图表
- 包含架构图、流程图、ER 图
- 代码示例、配置示例

**特点**：
- 面向开发者
- 包含实现细节
- 可包含代码片段

### 变更提案（openspec/changes/）

**结构**：
```
openspec/changes/[change-id]/
├── proposal.md      # 为什么做、做什么、影响范围
├── design.md        # 技术设计（可选）
├── tasks.md         # 实现清单
└── specs/
    └── [capability]/
        └── spec.md  # 规范增量（ADDED/MODIFIED/REMOVED）
```

**特点**：
- 临时文档，实现完成后归档
- 包含完整的变更上下文
- 可追溯、可回滚

---

## 维护责任

### OpenSpec 文档
- **更新时机**：需求变更时
- **更新方式**：通过 OpenSpec 流程（创建 change）
- **维护者**：产品经理 + 技术负责人

### 技术文档
- **更新时机**：代码变更时同步更新
- **更新方式**：直接修改或通过 PR
- **维护者**：开发团队

### 用户文档
- **更新时机**：功能发布时
- **更新方式**：创建或更新 `docs/guides/`
- **维护者**：技术写作 + 开发团队

---

## 文档审查清单

创建或修改文档时，检查：

- [ ] 文档放在正确的目录（openspec/ vs docs/）
- [ ] 没有使用 emoji
- [ ] 没有创建临时总结文档
- [ ] 文档命名符合规范
- [ ] 内容完整、格式正确
- [ ] 引用的文档路径正确
- [ ] 更新了相关的索引文档（如 CHANGELOG.md）

运行验证：
```bash
./tools/validate-docs.sh
```
