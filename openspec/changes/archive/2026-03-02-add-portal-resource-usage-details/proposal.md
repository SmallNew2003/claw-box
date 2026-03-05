# Change: 门户资源使用概览详情页

## Why
工作台资源使用概览卡片无法进入详情，普通用户缺少只读的资源使用全局视图。

## What Changes
- 在门户端新增算力、模型、知识库、智能体四个只读详情页
- 工作台资源使用概览四张卡片跳转到对应详情页
- 详情页采用统一只读概览布局（KPI、趋势、告警/活动）

## Impact
- Affected specs: portal-resource-usage
- Affected code: bj-gover-llm-admin/src/views/portal/Dashboard.vue, bj-gover-llm-admin/src/views/portal/*
