# Change: 添加 OpenSpec 预检硬闸

## Why
新功能实现存在跳过 OpenSpec 提案流程的风险，需要通过工具与流程硬闸强制执行，确保规范先行。

## What Changes
- 新增 OpenSpec 预检脚本，自动推断并校验提案
- 集成 git pre-commit 钩子阻断未通过校验的提交
- 补充文档说明与安装指引

## Impact
- Affected specs: developer-workflow
- Affected code: tools/
- Affected docs: README.md, AGENTS.md, CLAUDE.md
