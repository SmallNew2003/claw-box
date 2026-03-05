# Change: 增加全局悬浮聊天浮窗演示

## Why
为演示政务外网的私有化智能助手入口，需要一个全局可见的悬浮球与聊天窗口，用户无需真实后端即可看到效果。

## What Changes
- 新增全局悬浮球入口，覆盖后台所有页面
- 新增演示聊天窗口（本地 mock 对话与隐私提示）
- 不接入真实后端请求，仅用于演示

## Impact
- Affected specs: assistant-chat-widget（新增）
- Affected code: bj-gover-llm-admin/src/App.vue, bj-gover-llm-admin/src/components（新增组件）
