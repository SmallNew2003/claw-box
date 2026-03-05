# Change: 资源申请改为卡片列表 + 右侧抽屉编辑

## Why
当前 Tabs 内直接编辑表单，信息密度高且列表阅读困难。需要让申请列表更易浏览，新增/编辑统一走右侧抽屉，卡片尽量展示全量信息，提升快速浏览与操作效率。

## What Changes
- 模型/知识库/智能体 Tabs 仅展示卡片列表与新增按钮
- 新增/编辑统一使用右侧抽屉，抽屉一次仅编辑一个条目
- 卡片展示尽量完整信息，长文本两行省略并支持展开
- 空状态使用大按钮引导新增，非空时 Tab 顶部按钮新增
- 删除入口放在卡片右上角

## Impact
- Affected specs: resource-application
- Affected code:
  - bj-gover-llm-portal/src/views/ResourceApply.vue
