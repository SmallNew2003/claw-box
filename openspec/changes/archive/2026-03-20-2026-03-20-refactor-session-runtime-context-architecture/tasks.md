# Implementation Tasks

## 1. Context 骨架
- [x] 1.1 创建 `backend/src/contexts/session` 分层目录
- [x] 1.2 创建 `backend/src/contexts/runtime` 分层目录
- [x] 1.3 定义 Session/Runtime application facade 与 port 接口

## 2. Runtime 迁移
- [x] 2.1 迁移 RuntimeExecution 状态机到 runtime/domain
- [x] 2.2 迁移 AgentRuntimeService 到 runtime/application
- [x] 2.3 将事件存储/发布逻辑迁移为 runtime/adapters

## 3. Session 迁移
- [x] 3.1 迁移 Session 聚合与会话规则到 session/domain
- [x] 3.2 迁移 SessionService 到 session/application
- [x] 3.3 迁移会话持久化与查询到 session/adapters

## 4. 接口与兼容
- [x] 4.1 迁移 `sessions` HTTP 入口到 application facade
- [x] 4.2 保持 API 输入输出语义兼容
- [x] 4.3 清理旧 services/models 入口并保留必要兼容层
- [x] 4.4 建立控制面命令入口（Tauri Command）并映射到 application facade
- [x] 4.5 保持数据面事件流走 HTTP + SSE，并与控制面共享 trace_id

## 5. 验证
- [x] 5.1 补充并更新 Session/Runtime 单元测试
- [x] 5.2 验证会话主链路端到端行为不回归
- [x] 5.3 运行 `openspec validate 2026-03-20-refactor-session-runtime-context-architecture --strict --no-interactive`
- [x] 5.4 将后端内联测试模块迁移到独立 `*_tests.rs` 文件
