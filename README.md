# Claw Box

个人 AI 助手的桌面控制台（Tauri/Electron + React + Vite）。提供会话管理、Agent Teams、记忆系统、技能与 MCP 管理的统一界面。

目前处于原型阶段，UI 使用 mock 数据，后端 Core Service 开发中。

## 开发

```bash
npm install
npm run dev
```

### OpenSpec 预检硬闸（可选但推荐）

安装 git hooks，在提交前自动执行 OpenSpec 预检：

```bash
bash tools/install-git-hooks.sh
```

## 构建

```bash
npm run build
```

## 说明

- 主要界面位于 `src/renderer/App.tsx`
- 样式位于 `src/renderer/styles.css`
- 架构设计：`docs/architecture.md`
- 产品规格：`docs/product-spec.md`
- 功能规范：`openspec/specs/`
- 文档组织规范：`openspec/DOCUMENTATION.md`
- 会话仪表盘包含状态卡片、会话列表以及状态/模型筛选交互
