# OpenSpec Preflight Hard Gate Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 提供强制 OpenSpec 预检机制，自动推断新功能变更并在 git pre-commit 阶段阻断未通过提案校验的提交。

**Architecture:** 使用 Node 脚本读取 git diff，判定是否需要 OpenSpec，验证 openspec 变更目录结构，并执行 `openspec validate`。通过 npm script 与 git hook 集成。

**Tech Stack:** Node.js, TypeScript, git hooks

---

### Task 1: 创建 OpenSpec 预检脚本

**Files:**
- Create: `tools/openspec-preflight.ts`
- Modify: `package.json`
- Test: `tools/openspec-preflight.spec.md`（脚本自测说明）

**Step 1: 写失败测试（手动运行）**

```bash
node tools/openspec-preflight.ts --dry-run
```

Expected: FAIL（脚本不存在）

**Step 2: 实现最小脚本**

```ts
// 读取 git diff --name-only
// 判断是否需要 OpenSpec
// 若需要，检查 openspec/changes/<id> 是否存在（默认最新或通过 --change-id 指定）
// 校验 proposal/spec/tasks
// 执行 openspec validate
```

**Step 3: 运行检查**

Run: `node tools/openspec-preflight.ts --dry-run`
Expected: 输出检测结果并退出 0（dry-run）

**Step 4: 接入 npm script**

```json
"scripts": {
  "openspec:preflight": "node tools/openspec-preflight.ts"
}
```

**Step 5: Commit**

```bash
git add tools/openspec-preflight.ts package.json tools/openspec-preflight.spec.md
 git commit -m "feat(tools): 添加openspec预检脚本"
```

### Task 2: 添加 git pre-commit 安装脚本

**Files:**
- Create: `tools/install-git-hooks.sh`
- Modify: `README.md` 或 `AGENTS.md`

**Step 1: 写失败检查**

```bash
test -f tools/install-git-hooks.sh
```

Expected: FAIL

**Step 2: 实现脚本**

```bash
#!/usr/bin/env bash
set -euo pipefail
mkdir -p .git/hooks
cat > .git/hooks/pre-commit <<'HOOK'
#!/usr/bin/env bash
set -euo pipefail
npm run openspec:preflight
HOOK
chmod +x .git/hooks/pre-commit
```

**Step 3: 运行检查**

Run: `bash tools/install-git-hooks.sh`
Expected: `.git/hooks/pre-commit` 存在且可执行

**Step 4: Commit**

```bash
git add tools/install-git-hooks.sh README.md
 git commit -m "feat(tools): 添加git hooks安装脚本"
```

### Task 3: 更新强制流程文档

**Files:**
- Modify: `AGENTS.md`
- Modify: `CLAUDE.md`

**Step 1: 写失败检查**

```bash
grep -n "openspec:preflight" AGENTS.md
```

Expected: FAIL

**Step 2: 更新文档**

```md
- 新功能/破坏性变更必须先通过 `npm run openspec:preflight`
```

**Step 3: 运行检查**

Run: `grep -n "openspec:preflight" AGENTS.md`
Expected: 输出对应行

**Step 4: Commit**

```bash
git add AGENTS.md CLAUDE.md
 git commit -m "feat(docs): 强制openspec预检流程"
```
