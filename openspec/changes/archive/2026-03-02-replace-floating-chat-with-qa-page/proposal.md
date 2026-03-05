# Change: 移除悬浮球并新增通用智能问答页面

## Why
当前悬浮球入口不符合“完整页面”展示诉求，需要以主菜单入口提供一页式的通用智能问答展示，体现文心一言式布局。

## What Changes
- 移除全局右下角悬浮球与浮窗入口
- 新增“通用智能问答”页面与主菜单一级入口
- 提供左侧会话/提示词与右侧对话区的演示布局（本地 mock 交互）

## Impact
- Affected specs: assistant-qa（新增）
- Affected code: bj-gover-llm-admin/src/App.vue, bj-gover-llm-admin/src/router, bj-gover-llm-admin/src/views, bj-gover-llm-admin/src/layout
