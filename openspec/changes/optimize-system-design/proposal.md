# Change: 优化系统设计文档体系

## Why

通过系统设计评审，发现以下问题：

1. **后端架构设计缺失** - Rust Core Service 无详细的 DDD 架构设计
2. **API 契约不完整** - 缺少错误处理、流式输出、事件格式定义
3. **模块设计不具体** - Agent Teams、记忆系统等缺少实现细节
4. **安全实现不明确** - IPC 安全、密钥存储等缺少具体方案
5. **性能指标不可验证** - 缺少测量方法和具体优化策略
6. **规范文件过于简单** - openspec/specs/ 缺少完整的场景定义、数据模型、状态机
7. **技术栈不一致** - 文档说 Tauri 优先，实际用 Electron

这些问题导致：
- 开发团队无法根据现有文档进行实现
- 前后端接口约定不清晰
- 性能优化缺少可测量的目标
- 安全配置缺少具体指引

## What Changes

### 1. 创建完整的设计文档体系

新增 5 个设计文档：
- `docs/backend-architecture.md` - DDD 分层架构、聚合设计、技术选型
- `docs/api-specification.md` - RESTful 端点、响应格式、流式输出、错误处理
- `docs/event-protocol.md` - SSE 事件流、事件类型、连接管理
- `docs/security-design.md` - Tauri 安全配置、密钥存储、权限控制
- `docs/performance-strategy.md` - 性能指标、优化方案、监控测试

### 2. 扩充规范文件

扩充 6 个 openspec/specs/ 文件，每个文件包含：
- 详细的功能需求（Requirement + Scenario）
- 非功能需求（Performance、Scalability、Reliability）
- 完整的数据模型（TypeScript 接口定义）
- 状态机（关键实体的状态转换）
- 错误场景（所有可能的错误情况）
- 设计文档引用

### 3. Agent Teams 新增动态任务分配功能

参考 Kimi Agent Swarm 设计，新增：
- Leader 自动评估任务复杂度并分配 Worker 数量
- Worker 请求支援机制
- Worker 与 Leader 协商机制
- 动态缩容（任务完成后释放空闲 Worker）
- 上下文管理优化（主代理+子代理架构，减少 Token 消耗）

### 4. 统一技术栈为 Tauri

- 移除 Electron 依赖和配置文件
- 更新 package.json 使用 Tauri
- 重组目录结构（src/ → src-ui/）

### 5. 更新现有文档

- `docs/architecture.md` - 添加文档索引，指向各个专项文档

## Impact

### Affected Specs
- session-management - 扩充功能需求、数据模型、状态机
- agent-teams - 新增动态任务分配、Worker 协商、上下文优化
- memory-system - 扩充向量检索配置、Embedding 策略
- skills-management - 扩充权限系统、沙箱隔离
- mcp-integration - 扩充连接管理、工具调用协议
- model-management - 扩充模型配置、切换策略

### Affected Code
- `package.json` - 移除 Electron，添加 Tauri 依赖
- `electron.vite.config.ts` - 删除
- `electron-builder.yml` - 删除
- `src/main/` - 删除（Electron 主进程）
- `src/preload/` - 删除（Electron 预加载脚本）
- `src/renderer/` - 重命名为 `src-ui/`

### Affected Documentation
- 新增 5 个设计文档
- 扩充 6 个规范文件
- 更新 1 个现有文档

### Breaking Changes
- **BREAKING**: 技术栈从 Electron 切换为 Tauri
- **BREAKING**: 前后端通信从 IPC 切换为 Tauri Command

### Migration Path
1. 清理 Electron 相关文件
2. 安装 Tauri 依赖
3. 创建 Tauri 配置文件
4. 实现 Tauri Command API
5. 更新前端调用方式

## Benefits

1. **开发效率提升** - 完整的设计文档指导实现定清晰** - API 规范和事件协议明确定义
3. **性能可优化** - 可测量的性能指标和优化策略
4. **安全可保障** - 具体的安全配置和实施方案
5. **Token 成本降低** - Agent Teams 上下文优化减少 60-70% Token 消耗
6. **技术栈统一** - 符合 openspec/project.md 定义，更小的打包体积，更好的安全性

## Risks

1. **文档维护成本** - 需要保持文档与代码同步
   - 缓解：通过 OpenSpec 流程管理变更，文档与代码一起更新

2. **技术栈切换风险** - Tauri 生态相对 Electron 较新
   - 缓解：Tauri 已经成熟，有完善的文档和社区支持

3. **学习曲线** - 团队需要学习 DDD 架构和 Tauri
   - 缓解：提供详细的设计文档和示例代码

## Alternatives Considered

### 方案 A：保持 Electron，只补充文档
- 优点：无需技术栈切换
- 缺点：与 openspec/project.md 定义不一致，打包体积大

### 方案 B：分阶段优化（先文档，后技术栈）
- 优点：风险分散
- 缺点：延长整体周期，文档可能需要二次调整

### 方案 C：全面优化（推荐）
- 优点：一次性解决所有问题，建立完整基线
- 缺点：工作量大
- 选择理由：项目处于早期阶段，现在调整成本最低

## Timeline

- 文档创建：2-3 天
- 技术栈切换：1 天
- 验证与调整：1 天
- 总计：4-5 天
