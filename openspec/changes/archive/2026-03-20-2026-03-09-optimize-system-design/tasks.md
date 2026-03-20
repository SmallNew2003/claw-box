## 1. 创建设计文档
- [x] 1.1 创建 docs/backend-architecture.md（DDD 架构、聚合设计、技术选型）
- [x] 1.2 创建 docs/api-specification.md（RESTful 端点、响应格式、流式输出）
- [x] 1.3 创建 docs/event-protocol.md（SSE 事件流、事件类型、连接管理）
- [x] 1.4 创建 docs/security-design.md（Tauri 安全配置、密钥存储、权限控制）
- [x] 1.5 创建 docs/performance-strategy.md（性能指标、优化方案、监控测试）

## 2. 更新现有文档
- [x] 2.1 在 docs/architecture.md 开头添加文档索引

## 3. 扩充规范文件
- [x] 3.1 扩充 openspec/specs/session-management/spec.md
- [x] 3.2 扩充 openspec/specs/agent-teams/spec.md（含动态任务分配）
- [x] 3.3 扩充 openspec/specs/memory-system/spec.md（含 Embedding 策略）
- [x] 3.4 扩充 openspec/specs/skills-management/spec.md（含权限系统）
- [x] 3.5 扩充 openspec/specs/mcp-integration/spec.md（含连接管理）
- [x] 3.6 扩充 openspec/specs/model-management/spec.md（含模型配置）

## 4. 技术栈统一
- [x] 4.1 删除 electron.vite.config.ts
- [x] 4.2 删除 electron-builder.yml
- [x] 4.3 删除 src/main/ 目录
- [x] 4.4 删除 src/preload/ 目录
- [x] 4.5 重命名 src/renderer/ 为 src-ui/
- [x] 4.6 更新 package.json（移除 Electron，添加 Tauri）

## 5. 验证
- [x] 5.1 运行 openspec validate --strict --no-interactive
- [x] 5.2 检查所有设计文档是否创建
- [x] 5.3 检查所有规范文件是否扩充（每个文件至少 100 行）
- [x] 5.4 验证文档引用关系

## 6. 提交
- [x] 6.1 提交设计文档（5 个文件）
- [x] 6.2 提交文档索引更新
- [x] 6.3 提交规范文件扩充（6 个文件）
- [x] 6.4 提交技术栈统一变更
