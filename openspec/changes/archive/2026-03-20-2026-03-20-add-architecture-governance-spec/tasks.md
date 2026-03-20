# Implementation Tasks

## 1. 规范落地
- [x] 1.1 创建 architecture-governance 规范增量并通过校验
- [x] 1.2 在 project.md 增加架构治理总纲引用（仅高层原则）
- [x] 1.3 建立注释与契约文档检查清单（Rust / TypeScript）

## 2. 架构约束实施
- [x] 2.1 梳理现有模块并映射到限界上下文
- [x] 2.2 定义统一目录分层（Domain/Application/Ports/Adapters/Interfaces）
- [x] 2.3 补充跨上下文交互契约（应用服务接口或事件契约）

## 3. 一致性机制演进
- [x] 3.1 识别关键状态变化并定义领域事件清单
- [x] 3.2 设计 Outbox 表结构与发布流程
- [x] 3.3 规划读写分离迁移路径（写模型/读模型）

## 4. 验证
- [x] 4.1 运行 OpenSpec 严格校验
- [x] 4.2 验证文档规范检查脚本通过
