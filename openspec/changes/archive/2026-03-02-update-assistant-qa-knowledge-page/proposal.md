# Change: 通用智能问答增加知识库管理页与侧边栏选中态

## Why
需要在通用智能问答内提供独立的知识库管理页面，并让左侧迷你侧边栏能够切换与高亮选中状态。

## What Changes
- 新增“知识库管理”页面视图（参考图风格，移除模型嵌入）
- 左侧迷你侧边栏支持“对话/知识库管理”选中态切换

## Impact
- Affected specs: assistant-qa
- Affected code: bj-gover-llm-admin/src/views/portal/GeneralQA.vue
