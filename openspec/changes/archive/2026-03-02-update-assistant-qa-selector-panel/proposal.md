# Change: 通用智能问答选择器改为输入框下方弹层

## Why
现有模型/知识库选择位于顶部，不符合参考样式与交互期望，需要将选择器下移到输入框下方并提供弹层选择体验。

## What Changes
- 将模型与知识库的入口移动到输入框下方，显示为可点击胶囊按钮
- 点击胶囊打开轻量弹层，展示可选项并支持切换
- 保留顶部“当前模型/知识库”提示，不影响现有问答演示逻辑

## Impact
- Affected specs: assistant-qa
- Affected code: bj-gover-llm-admin/src/views/portal/GeneralQA.vue
