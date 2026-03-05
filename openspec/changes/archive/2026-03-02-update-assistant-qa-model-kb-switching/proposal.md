# Change: 通用智能问答增加模型与知识库选择

## Why
当前通用智能问答仅展示对话布局，缺少“模型切换”和“知识库指定”的效果展示，难以体现可配置性。

## What Changes
- 在通用智能问答页面新增模型切换与知识库选择区（本地 mock 数据）
- 在管理端与门户端均提供“通用智能问答”入口，且指向同一页面

## Impact
- Affected specs: assistant-qa
- Affected code: bj-gover-llm-admin/src/router, bj-gover-llm-admin/src/views, bj-gover-llm-admin/src/layout
