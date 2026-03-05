# Change: 通用智能问答增加附件/链接上下文与提示词优化

## Why
当前通用智能问答缺少上下文注入与提问辅助能力，无法展示“附件/链接作为上下文”和“提示词优化”的核心体验。

## What Changes
- 在输入区下方工具栏新增“上传附件”“添加网络链接”入口（前端 mock）
- 提供上下文列表展示（附件/链接），支持移除
- 新增“提示词优化”入口，输出优化后的问题文本（前端 mock）

## Impact
- Affected specs: assistant-qa
- Affected code: bj-gover-llm-admin/src/views/portal/GeneralQA.vue
