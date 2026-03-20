# Change: 对齐客户端核心定位与双通道通信模型

## Metadata
- Created: 2026-03-20T11:55:00Z
- Affected Specs:
  - architecture-governance (modified: 2026-03-20T10:30:00Z)

## Why

当前项目强调客户端承担大量配置和操作职责，但现有文档对通信模式存在不一致：

1. 有的文档只强调 HTTP API，弱化了客户端控制面的本地调用优势
2. 有的文档倾向全部走 Tauri Command，忽略高频事件流的稳定与可观测要求

需要统一为一套可落地的高性能高稳定方案，避免后续提案在协议层继续分叉。

## What Changes

1. 明确客户端核心定位：客户端是控制中心，负责配置、操作与可视化
2. 明确控制面/数据面双通道：
   - 控制面：Tauri Command（低频命令、配置变更、触发操作）
   - 数据面：本地 HTTP + SSE（高频状态、日志、事件流）
3. 要求两通道通过 `trace_id` 贯通，支持端到端追踪与回放
4. 将该模型纳入架构治理规范，作为后续变更评审基线

## Impact

### Affected specs
- `architecture-governance`

### Affected docs
- `openspec/project.md`（后续实现阶段按本提案更新高层架构模式）

### Breaking changes
无直接运行时破坏性变更；属于架构规范对齐。

### Migration
后续变更按双通道模型实施，已有提案在更新时需对齐该约束。
