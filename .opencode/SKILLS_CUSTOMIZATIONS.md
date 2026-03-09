# Skills 定制说明

本文档记录了相对于上游 superpowers 的所有定制修改。
当前仓库以 `.agents/skills` 为唯一来源，`.opencode/skills` 为符号链接。

## Fork 信息

- 上游仓库: https://github.com/superpowers/superpowers
- 我的 Fork: https://github.com/SmallNew2003/superpowers
- 定制分支: main

## 修改的 Skills

### 1. executing-plans

**修改时间**: 2026-03-03
**修改原因**: 需要同步 OpenSpec tasks.md 的 checkbox 状态

**具体修改**:
- `Step 2: Execute Batch` 增加第 5 步:
  ```markdown
  5. **Sync status to tasks.md**: Update the corresponding `- [ ]` to `- [x]` in the plan file
  ```
- `Remember` 部分增加:
  ```markdown
  - **Always sync task status**: Update both TodoWrite AND tasks.md checkbox when completing tasks
  ```

**合并上游时注意**:
- 保留状态同步逻辑
- 如果上游修改了 Step 2 结构，手动合并

---

### 2. using-git-worktrees

**修改时间**: 2026-03-03
**修改原因**: 避免小改动也创建 worktree，增加用户控制权

**具体修改**:
- `description` 改为: "Use ONLY for major changes requiring isolation... and ONLY after user confirmation"
- 新增 `When to Use Worktrees` 章节，明确使用场景:
  - 使用: 10+ 文件重构、破坏性变更、长期功能
  - 不使用: 单文件、Bug 修复、小功能
- 新增强制确认机制:
  ```markdown
  ALWAYS ask user first:
  "This change involves [scope]. I recommend using a git worktree for isolation.
  Should I create a worktree, or work directly on the current branch?"
  ```

**合并上游时注意**:
- 保留判断标准和确认机制
- 合并上游的技术改进（目录选择、安全验证等）

---

### 3. subagent-driven-development

**修改时间**: 2026-03-03
**修改原因**: 配合 using-git-worktrees 的修改

**具体修改**:
- `Integration` 部分将 worktrees 从 REQUIRED 改为 OPTIONAL:
  ```markdown
  - **superpowers:using-git-worktrees** - OPTIONAL: Only for major changes, requires user confirmation
  ```

**合并上游时注意**:
- 保持 worktrees 为可选状态

---

### 4. openspec-planning

**修改时间**: 2026-03-03
**修改原因**: 集成 OpenSpec 工作流，增加任务状态管理

**具体修改**:
- `Step 5: 等待批准` 增加状态同步说明:
  ```markdown
  用户批准后：
  1. 使用 TodoWrite 创建任务列表（基于 tasks.md）
  2. 标记第一个 todo 为 in_progress
  3. 开始实现
  4. **每完成一个任务，同步更新两处状态**：
     - TodoWrite 中标记为 completed
     - tasks.md 中将 `- [ ]` 改为 `- [x]`
  ```
- `关键提醒` 增加第 6 条:
  ```markdown
  6. **任务状态同步**：完成任务时必须同时更新 TodoWrite 和 tasks.md 的 checkbox
  ```

**合并上游时注意**:
- 这是项目特定的 skill，上游不会有
- 如果上游添加了类似功能，评估是否需要合并

---

## 更新流程

说明：本仓库不再直接使用 subtree 管理 `.opencode/skills`，统一以 `.agents/skills` 为准。

### 从上游拉取更新

```bash
# 1. 添加上游 remote（首次）
git remote add upstream https://github.com/superpowers/superpowers.git

# 2. 拉取上游更新
git fetch upstream

# 3. 查看差异
git diff upstream/main -- skills/

# 4. 合并更新（在 fork 仓库中处理完成后，同步到本项目）
# 复制更新后的 skills 到 .agents/skills
```

### 推送修改到 Fork

```bash
# 在 fork 仓库完成更新后，将内容同步到本项目的 .agents/skills
```

## 冲突解决策略

1. **技术改进优先**: 上游的技术改进（bug 修复、性能优化）优先采纳
2. **保留定制逻辑**: 我们的业务逻辑（状态同步、确认机制）必须保留
3. **文档优先**: 遇到冲突时，先更新本文档，再手动合并

## 贡献回上游

以下修改可以考虑提 PR 回上游:
- [ ] using-git-worktrees 的用户确认机制（通用改进）
- [ ] executing-plans 的状态同步逻辑（如果上游支持 tasks.md）

以下修改是项目特定的，不适合回上游:
- openspec-planning（项目特定）
