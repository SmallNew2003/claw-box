# OpenSpec Preflight 硬闸设计草案

日期：2026-03-05

## 背景

在新功能场景下必须先走 OpenSpec 提案流程，但仍存在流程被跳过的风险，需要通过机制性硬闸强制执行。

## 目标

- 仅对“新功能/破坏性变更”强制 OpenSpec
- 自动推断是否需要 OpenSpec，同时允许显式指定 change-id
- 一旦判定需要 OpenSpec，禁止绕过
- 与 git pre-commit 钩子集成，提交前强制校验

## 触发规则

### 自动推断
若 diff 涉及以下路径，则判定需要 OpenSpec：
- `src/`, `platform/`, `apps/`, `tools/`
- `electron*`, `tsconfig*`, `package.json`

### 忽略规则
仅修改以下内容时，不触发：
- `docs/`、`README.md`、`openspec/` 内部文件
- 纯 `.md` 变更

### 显式指定
支持 `--change-id <id>` 强制校验指定变更。

## 校验流程

1. 解析 `git diff --name-only`
2. 判断是否需要 OpenSpec
3. 若需要：
   - 检查 `openspec/changes/<id>` 是否存在
   - 检查 `proposal.md`、`specs/*/spec.md`、`tasks.md` 是否存在
   - 执行 `openspec validate <id> --strict --no-interactive`
4. 任一步失败则退出非 0

## 集成方式

- 新增脚本：`tools/openspec-preflight.ts`
- npm scripts：`openspec:preflight`
- 安装脚本：`tools/install-git-hooks.sh`，写入 `pre-commit` 执行 preflight
- 文档更新：`AGENTS.md` / `CLAUDE.md` 增加强制约束说明

## 取舍

- 不允许 override：确保流程刚性
- 自动推断 + 显式指定：兼顾默认体验与可控性
