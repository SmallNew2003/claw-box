# Claw Box - 项目上下文

<!-- 规范版本: v2.1.0 -->
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

Claw Box 是个人 AI 助手的桌面控制台，提供会话管理、Agent Teams 协作、记忆系统、技能与 MCP 管理的统一界面。

**核心目标**：
- 提供清晰的会话与 Agent Teams 运行态可视化
- 管理长期记忆、技能与 MCP 连接
- 支持单机一体化部署（后端 Core Service + 前端控制台）
- 桌面端优先（Tauri/Electron），保留 Web 端扩展能力

**系统定位**：
- 控制台层：负责 UI 交互与状态展示
- 后端层：Core Service（Rust）提供统一 API
- 不承载模型推理，通过 API 调用外部模型服务

## Tech Stack

### 前端（控制台）
- Tauri（优先）或 Electron
- React 18 + TypeScript + Vite
- Zustand（状态管理）
- TailwindCSS（样式）

### 后端（Core Service）
- Rust（HTTP/JSON API）
- SQLite（默认存储）
- 本地向量检索（记忆系统）
- 可选：PostgreSQL + Qdrant/pgvector + Redis

### 工程
- 单机一体化部署（单二进制 + 内嵌前端）
- 前后端通过 HTTP/JSON + SSE/WebSocket 通信
- 配置分层：local / workspace / runtime

## Project Conventions

### Code Style
- 使用 TypeScript 严格模式
- 组件命名：PascalCase（如 `SessionList.tsx`）
- 文件命名：kebab-case（如 `session-list.tsx`）
- 提交格式：`feat(模块): 中文内容`
- 禁止使用 emoji

### Architecture Patterns

- 前端：纯 UI 层，通过 HTTP API 与后端通信
- 后端：Core Service 提供统一 API，内部事件流与任务编排
- 模块化能力：会话、Agent Teams、记忆、技能、MCP
- 事件流：SSE/WebSocket 实时推送运行态与日志

### Testing Strategy
- 单元测试：核心业务逻辑与状态流
- 集成测试：IPC 与本地存储
- E2E 测试：关键操作流程（会话创建、模型切换、任务调度）

### Git Workflow
- 主分支：`main`
- 功能分支：`feature/<change-id>`
- 提交前运行 lint（如有）
- 使用 OpenSpec 管理变更提案

## Domain Context

### 关键角色

- **个人用户**：使用 AI 助手完成日常任务，管理会话与记忆

### 核心领域

- **会话管理域**：会话生命周期、消息历史、运行态
- **Agent Teams 域**：团队编排、角色分配、协作流程
- **记忆系统域**：长期记忆存储、向量检索、摘要压缩
- **技能管理域**：技能安装、配置、执行监控
- **MCP 集成域**：MCP 服务器连接、工具索引、调用日志

### 职责边界

**控制台负责**：
- 会话与 Agent Teams 运行态可视化
- 记忆、技能、MCP 配置管理
- 用户交互与状态展示

**Core Service 负责**：
- 业务逻辑与任务编排
- 数据持久化与检索
- 模型调用与工具执行

**我们不负责**：
- 模型推理与训练
- 多租户与权限管理（MVP）

## Important Constraints

### 业务约束
- 所有关键操作可追溯
- 本地配置需可导出与迁移

### 技术约束
- 前端通过 HTTP API 与后端通信，无直接系统访问
- 默认本地数据落盘（SQLite），支持外部数据库扩展
- 单机一体化部署，可选分离部署

### 安全约束
- API Token 加密存储
- 敏感数据不出本机
- 变更日志可审计

## External Dependencies

- 模型服务（Claude API、OpenAI API、本地模型）
- 可选：外部向量数据库（Qdrant/pgvector）
- 可选：Redis（缓存与会话状态）

## Reference Documents

- 产品规格：`docs/product-spec.md`
- 架构设计：`docs/architecture.md`
