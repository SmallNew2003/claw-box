# Claw Box - 项目上下文

<!-- 规范版本: v2.0.0 -->
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

Claw Box 是面向 OpenClaw 的桌面控制台应用，提供多会话、模型、任务、技能、MCP、Agent Teams 与项目管理的一体化操作界面。

**核心目标**：
- 提供稳定、可扩展的多会话与模型切换体验
- 统一管理定时任务、技能与 MCP 连接
- 展示 Agent Teams 运行态并支持协作管理
- 支持项目级上下文与配置
- 保留桌面端高效交互，后续可扩展到 Web/Windows/移动端

**系统定位**：
- 不直接承载模型推理，作为 OpenClaw 的控制层
- 通过本地 Gateway/API 读取状态与下发指令
- 以桌面应用优先，保留跨平台移植能力

## Tech Stack

### 桌面端
- Electron + TypeScript
- React 18 + Vite
- 轻量状态管理（Zustand 或 React Context）

### Web/跨端（规划）
- React 18 + Vite
- 统一 UI 组件库与状态管理

### 工程
- Electron 主进程 + 渲染进程 + Preload 安全隔离
- 本地配置存储（JSON/SQLite/IndexedDB 组合）
- IPC 统一桥接 OpenClaw 能力

## Project Conventions

### Code Style
- 使用 TypeScript 严格模式
- 组件命名：PascalCase（如 `SessionList.tsx`）
- 文件命名：kebab-case（如 `session-list.tsx`）
- 提交格式：`feat(模块): 中文内容`
- 禁止使用 emoji

### Architecture Patterns

- 主进程负责系统集成与数据持久化
- 渲染进程负责 UI、状态管理与用户交互
- IPC 统一封装为服务层，避免渲染进程直接访问系统资源
- 采用模块化能力包：会话、模型、任务、技能、MCP、Teams、项目

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

- **桌面用户**：需要同时管理多个会话与模型
- **系统管理员**：管理全局模型与技能配置
- **协作者**：关注 Agent Teams 状态与任务协同

### 核心领域

- **会话管理域**：会话生命周期、上下文与消息
- **模型管理域**：模型配置、切换与状态
- **任务调度域**：定时任务与执行历史
- **技能与 MCP 域**：技能安装、配置与 MCP 连接
- **Agent Teams 域**：团队编排与运行状态
- **项目管理域**：项目配置与上下文收敛
- **市场域**：技能市场与更新管理

### 职责边界

**我们负责**：
- ✅ 多会话管理与状态展示
- ✅ 模型切换与配置管理
- ✅ 定时任务创建与监控
- ✅ 技能与 MCP 配置管理
- ✅ Agent Teams 状态可视化
- ✅ 项目上下文管理

**我们不负责**：
- ❌ 模型推理与训练
- ❌ OpenClaw 以外的业务系统集成

## Important Constraints

### 业务约束
- 所有关键操作可追溯
- 本地配置需可导出与迁移

### 技术约束
- Electron 安全隔离（禁用 NodeIntegration）
- IPC 仅暴露白名单能力
- 默认本地数据落盘，支持后续云同步扩展

### 安全约束
- API Token 加密存储
- 敏感数据不出本机
- 变更日志可审计

## External Dependencies

- OpenClaw Gateway API（本地或内网部署）
- 可选：技能市场服务（后续接入）

## Reference Documents

- 产品规格：`docs/product-spec.md`
- 架构设计：`docs/architecture.md`
