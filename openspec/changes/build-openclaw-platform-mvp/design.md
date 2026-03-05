# 设计要点（MVP）

## 架构选择
- Core Service 采用 Rust，强调单机一体化与可扩展性
- UI 层：Web + Tauri，共用 API
- 存储默认 SQLite + 本地向量检索，保留外部化扩展

## 关键决策
- 兼容 OpenClaw 配置格式，新增字段放在 extensions
- Agent Teams 参考 Claude Flow 的编排形态
- 事件流使用 SSE，后续可扩展 WebSocket/gRPC

## 取舍与风险
- Rust 生态整合成本较高，通过缩小 MVP 范围降低风险
- 本地向量检索质量与性能依赖实现，预留外部向量库切换
