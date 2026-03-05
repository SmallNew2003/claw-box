# 政务大模型应用服务平台 - 项目上下文

<!-- 规范版本: v1.0.0 -->
<!-- 最后更新: 2026-03-05 -->
<!-- 维护者: 开发团队 -->

> **重要提示**：本文档是项目规范的唯一真相源。所有 Skills 和其他文档中的规范内容都应引用本文档。

## 如何更新本规范

规范变更需要通过 OpenSpec 流程：

1. **创建提案**：`changes/update-project-conventions/`
   - `proposal.md` - 说明变更原因、影响范围
   - `specs/development-standards/spec.md` - 规范变更内容

2. **更新本文档**
   - 修改对应章节内容
   - 更新版本号（遵循语义化版本）
   - 更新"最后更新"时间
   - 在 `openspec/CHANGELOG.md` 记录变更

3. **更新引用**
   - 检查所有引用此规范的 Skills
   - 如果引用路径（行号）变化，更新 Skills
   - 如果只是内容变化，Skills 会自动读取最新版本

4. **通知团队**
   - 归档提案后，通知团队成员
   - 重要变更需要团队评审

**版本号规则**：
- 主版本号（1.x.x）：破坏性变更（如禁止某种做法）
- 次版本号（x.1.x）：新增规范（如新增技术栈）
- 修订号（x.x.1）：澄清或修正（不改变语义）

## Purpose

政务大模型应用服务平台定位为**资源调度与运营中台**，负责统一管理、审批与调度算力、模型、知识库、智能体等资源。

**核心目标**：
- 规范政务大模型资源的申请和使用流程
- 提升资源利用效率，避免重复建设
- 保障数据安全和合规性
- 支持跨部门资源共享与复用

**系统定位**：
- 不实现具体 AI 能力（不做模型训练、推理）
- 通过对接 MB、AB、百舸、数据中枢等外部系统实现资源调度
- 提供统一的申请、审批、管理入口

## Tech Stack

### 前端
- Vue 3 + TypeScript + Vite
- Element Plus（UI 组件库）
- Vue Router（路由管理）
- Pinia（状态管理）

### 后端（外部系统）
- MB 模型服务平台（模型调用）
- AB 智能体开发平台（智能体开发）
- 百舸算力平台（算力调度）
- 数据中枢（数据权限）
- QAX 安全检测平台（安全预警）

### 工程
- 统一前端基座：`bj-gover-llm-admin`
- 构建工具：Vite
- 认证：统一入口登录态校验，支持多角色选择

## Project Conventions

### Code Style
- 使用 TypeScript 严格模式
- 组件命名：PascalCase（如 `ResourceCard.vue`）
- 文件命名：kebab-case（如 `resource-card.vue`）
- 提交格式：`feat(模块): 中文内容`（如 `feat(资源管理): 添加资源申请功能`）
- 禁止使用 emoji

### 后端开发规范

**数据访问层（MyBatis-Flex）**：
- ✅ **禁止使用 Mapper XML 文件**：XML 难以维护，字段变更需要同步修改多处
- ✅ **使用 QueryWrapper API**：类型安全、自动同步、易于重构
- ✅ **简单查询**：使用 `BaseMapper` 自动生成的 CRUD 方法
- ✅ **复杂查询**：在 Service 层使用 QueryWrapper 构建动态 SQL
- ✅ **特殊 SQL**：使用 `@Select`、`@Insert`、`@Update`、`@Delete` 注解直接写在 Mapper 接口上

**QueryWrapper 示例**：
```java
// 动态条件查询
QueryWrapper query = QueryWrapper.create()
    .select()
    .from(SYS_USER)
    .where(SYS_USER.DEL_FLAG.eq("F"))
    .and(SYS_USER.NICKNAME.like(dto.getNickname()).when(dto.getNickname() != null))
    .and(SYS_USER.DEPT_ID.eq(dto.getDeptId()).when(dto.getDeptId() != null))
    .orderBy(SYS_USER.CREATE_TIME.desc());

List<SysUserEntity> users = userMapper.selectListByQuery(query);
```

**优势**：
- 编译期类型检查，字段名错误立即发现
- 实体类字段变更，代码自动适配
- IDE 重构支持，所有引用自动更新
- SQL 逻辑和业务逻辑在同一文件，易于维护
- 动态 SQL 构建更简洁，无需 XML `<if>` 标签

### Architecture Patterns

**领域驱动设计（DDD）**：
- 限界上下文：用户与权限域、审批流程域、资源管理域、审计与合规域、通知与告警域、运营分析域
- 聚合设计：通过聚合根管理实体生命周期
- 领域事件：事件驱动实现领域间松耦合

**统一入口、角色分域**：
- A 端（业务服务门户）：委办局业务人员、应用开发商
- B 端（运营管理后台）：大数据中心、运营团队
- 登录后根据角色权限展示不同菜单和功能

**前后端分离**：
- 前端 SPA，后端 RESTful API
- 前端负责路由、状态管理、UI 渲染
- 后端负责业务逻辑、数据持久化、外部系统集成

### Testing Strategy
- 单元测试：核心业务逻辑
- 集成测试：外部系统对接
- E2E 测试：关键用户流程

### Git Workflow
- 主分支：`main`
- 功能分支：`feature/<change-id>`
- 提交前运行测试和 lint
- 使用 OpenSpec 管理变更提案

## Domain Context

### 用户角色

**A 端用户（业务服务门户）**：
- 委办局业务人员：申请场景和资源、开发智能体
- 应用开发商：基于平台资源开发应用

**B 端用户（运营管理后台）**：
- 一级管理员（大数据中心）：全局监管、最终审批
- 二级管理员（市级/区级）：本部门/区域审批
- 三级管理员（区级委办局）：本部门初审
- 运营服务团队：需求评估、资源配置、系统对接

### 核心领域

**用户与权限域**：
- 用户管理、角色分配、权限策略、组织架构
- 核心聚合根：User、Role、Organization

**审批流程域**：
- 工单管理、审批链、状态机、审批记录
- 核心聚合根：ApprovalWorkflow、ApprovalTask

**资源管理域**：
- 算力、模型、知识库、智能体的生命周期管理
- 核心聚合根：ComputeResource、Model、KnowledgeBase、Agent

**审计与合规域**：
- 操作日志、审计追踪、合规检查
- 核心聚合根：AuditLog、ComplianceRecord

**通知与告警域**：
- 消息推送、预警管理、通知模板
- 核心聚合根：Notification、Alert

**运营分析域**：
- 数据统计、运营大屏、报表生成
- 核心聚合根：OperationMetrics、Dashboard

### 职责边界

**我们负责**：
- ✅ 资源的元数据管理
- ✅ 资源的上下架控制
- ✅ 资源状态展示
- ✅ 资源绑定关系管理
- ✅ 申请审批流程
- ✅ 权限控制与数据隔离

**我们不负责**：
- ❌ 算力的实际调度和分配（百舸负责）
- ❌ 模型训练和推理（MB 负责）
- ❌ 智能体的实际开发和调试（AB 负责）
- ❌ 知识库文档的上传、解析、向量化（外部知识库系统负责）

**例外：自建通用智能问答助手**：
- ✅ 我们负责完整实现（文档上传、解析、向量化、检索、对话）

## Important Constraints

### 业务约束
- 政务合规要求：所有操作需可追溯、可审计
- 数据隔离：不同组织的数据严格隔离
- 审批流程：关键操作需经过多级审批

### 技术约束
- 前端统一使用 Vue 3 + TypeScript
- 不实现具体 AI 能力，通过外部系统集成
- 需支持多角色、多组织的权限管理

### 安全约束
- 所有 API 调用需认证和鉴权
- 敏感数据需加密存储
- 操作日志需完整记录

## External Dependencies

### MB 模型服务平台
- 功能：模型调用、模型 ID 映射、调用统计
- 集成方式：RESTful API

### AB 智能体开发平台
- 功能：SSO 跳转、Project ID 管理、开发进度回写
- 集成方式：SSO + RESTful API

### 百舸算力平台
- 功能：算力资源调度、使用情况同步
- 集成方式：RESTful API

### 数据中枢
- 功能：数据权限配置、数据访问管理
- 集成方式：RESTful API

### QAX 安全检测平台
- 功能：安全预警、内容检测
- 集成方式：RESTful API

## Reference Documents

详细设计文档：
- 完整架构设计：`docs/2026-02-28-system-architecture-design.md`（944 行，包含详细的模块设计、数据模型、接口规范）
- 产品设计：`docs/2026-02-01-product-design.md`
- 会议纪要：`docs/AI平台资源申请与管理全流程重构会议纪要.md`
