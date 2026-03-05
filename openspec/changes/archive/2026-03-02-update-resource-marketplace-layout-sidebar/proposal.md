# Change: 资源广场改为侧栏筛选布局

## Why
当前筛选位于顶部且信息密度高，难以突出筛选与结果分区。参考布局需要更清晰的左右结构以提升发现效率。

## What Changes
- 资源广场页面改为“左侧筛选 + 右侧列表”的两栏布局
- 筛选项以胶囊按钮组呈现，保留现有筛选维度与行为
- 右侧顶部提供搜索与排序工具条，列表卡片调整为信息密度更高的样式

## Impact
- Affected specs: resource-marketplace
- Affected code: bj-gover-llm-admin/src/views/portal/ResourceMarketplace.vue
