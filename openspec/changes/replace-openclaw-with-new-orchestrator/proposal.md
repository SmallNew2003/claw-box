# Change: 抛弃 OpenClaw，重构全新编排内核与控制台

## Why
当前 OpenClaw 仅提供多会话调度与状态汇总，缺少内核级多智能体协作与共识机制。为实现“真实 Teams 协作、可观察性与可靠回传”的最优解，需要重构一套新的编排内核并以 Claw Box 作为唯一控制台。

## What Changes
- 新建“编排内核（Orchestration Core）”与“控制台（Control Console）”双层架构
- 统一事件总线与日志流，支持实时状态、问题/分歧回传与决策链路
- 在 Claw Box 内置 Teams 面板与编排控制入口
- OpenClaw 仅作为过渡或完全退役

## Impact
- 新增核心模块与基础设施（事件总线、任务路由、共识机制）
- 影响所有现有 workflow 与技能接入方式
- 需要定义数据模型、协议与迁移策略

## Follow-ups
- 产出系统架构设计与 MVP 路线图
- 定义事件协议与日志格式
- 评估迁移与兼容策略
