# OpenCode Skills 管理

本项目以 `.agents/skills` 为唯一来源，`.opencode/skills` 为符号链接指向该目录。

## 快速开始

### 修改 Skills

直接编辑 `.agents/skills/` 下的文件（`.opencode/skills` 为链接，不要手改）。

### 从 Fork 拉取更新

如需与 superpowers 同步，请先在独立 fork 仓库处理，再用更新内容覆盖 `.agents/skills/`。

## 目录结构

```
.opencode/
├── skills/                          # 指向 .agents/skills 的符号链接
│   ├── executing-plans/
│   ├── using-git-worktrees/
│   ├── openspec-planning/          # 项目特定 skill
│   └── ...
├── scripts/
│   ├── sync-skills-to-fork.sh      # 推送到 fork
│   └── pull-skills-from-fork.sh    # 从 fork 拉取
├── SKILLS_CUSTOMIZATIONS.md         # 定制说明文档
└── README.md                        # 本文件
```

## 工作流程

### 日常开发

1. **修改 skills**：直接编辑 `.agents/skills/` 下的文件
2. **提交到主项目**：`git commit -m "feat(skills): ..."`

### 更新 Skills

如需回溯定制或查看差异，参考 `SKILLS_CUSTOMIZATIONS.md`。

### 从上游 superpowers 拉取更新

```bash
# 1. 添加上游 remote（首次）
git remote add upstream https://github.com/superpowers/superpowers.git

# 2. 拉取上游更新
git fetch upstream

# 3. 查看差异
git diff upstream/main -- .opencode/skills/

# 4. 合并到你的 fork（在 fork 仓库中操作）
cd /tmp
git clone https://github.com/SmallNew2003/superpowers.git
cd superpowers
git remote add upstream https://github.com/superpowers/superpowers.git
git fetch upstream
git merge upstream/main
# 解决冲突，保留你的定制
git push origin main

# 5. 回到主项目，拉取你 fork 的更新
cd /Users/jelvin/000_source_code/bj-gover-llm
.opencode/scripts/pull-skills-from-fork.sh
```

## 定制的 Skills

详见 [SKILLS_CUSTOMIZATIONS.md](./SKILLS_CUSTOMIZATIONS.md)

当前定制的 skills：
- `executing-plans` - 增加 tasks.md 状态同步
- `using-git-worktrees` - 改为可选，需用户确认
- `openspec-planning` - 项目特定，集成 OpenSpec 工作流
- `subagent-driven-development` - worktree 改为可选

## Git Subtree 常用命令

已停用，历史参考保留。

## 故障排查

### 推送失败

```bash
# 检查 remote 配置
git remote -v

# 确保有推送权限
git remote set-url superpowers git@github.com:SmallNew2003/superpowers.git
```

### 拉取冲突

1. 查看冲突文件：`git status`
2. 参考 `SKILLS_CUSTOMIZATIONS.md` 了解定制内容
3. 手动解决冲突，保留定制逻辑
4. 提交合并结果

### Subtree 历史混乱

```bash
# 使用 --squash 压缩历史
git subtree pull --prefix=.opencode/skills superpowers main --squash
```

## 相关链接

- 你的 Fork: https://github.com/SmallNew2003/superpowers
- 上游仓库: https://github.com/superpowers/superpowers
- Git Subtree 文档: https://git-scm.com/book/en/v2/Git-Tools-Advanced-Merging
