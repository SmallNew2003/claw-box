## 1. Implementation
- [x] 1.1 重构 `backend/src/lib.rs` 导出边界，仅保留上下文聚合与共享模块
- [x] 1.2 在 runtime 上下文新增 `ToolInvokerPort`，应用层改为依赖端口抽象
- [x] 1.3 将工具调用实现迁移到 runtime adapters，并完成依赖注入组装
- [x] 1.4 将会话相关 API/命令处理器迁移到 session 上下文 interfaces 层
- [x] 1.5 清理顶层 `api/interfaces/models/services` 的业务职责并保留必要兼容入口
- [x] 1.6 修复编译与单元测试，确保行为兼容
- [x] 1.7 增加分层边界测试，覆盖应用层不直接依赖基础设施实现
- [x] 1.8 清退过渡兼容层，删除顶层旧目录 `api/interfaces/models/services`

## 2. Verification
- [x] 2.1 运行 `openspec validate 2026-03-20-ddd-backend-rearchitecture --strict --no-interactive`
- [x] 2.2 运行 `cargo test`（backend）并记录关键通过结果
