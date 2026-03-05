# Claw Box

面向 OpenClaw 的桌面控制台原型（Electron + React + Vite）。目前是 UI 壳，数据为本地 mock。

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
- 文档：`docs/product-spec.md`、`docs/architecture.md`
- 会话仪表盘包含状态卡片、会话列表以及状态/模型筛选交互
