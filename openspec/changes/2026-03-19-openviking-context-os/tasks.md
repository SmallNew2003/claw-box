## 1. Design and Contracts
- [ ] 1.1 固化 Context Gateway API 契约（search/read/browse/messages:append/commit/tasks）
- [ ] 1.2 固化 `ctx://` URI 规范与 `viking://` 映射规则
- [ ] 1.3 固化检索解释字段 schema（retrieval_path、rank_features、usage）
- [ ] 1.4 固化 commit 任务状态机与错误码规范
- [ ] 1.5 固化 V1 边界（Shared Space、DSL、Scoring 强耦合均排除）

## 2. Backend Implementation (Core Service)
- [ ] 2.1 创建 context-gateway 模块与路由
- [ ] 2.2 实现 OpenViking HTTP 适配层（含超时、重试、错误映射）
- [ ] 2.3 实现 ContextSessionMap 持久化
- [ ] 2.4 实现消息 append 幂等校验（idempotency_key）
- [ ] 2.5 实现 commit 并发控制（单 session 单活跃任务）
- [ ] 2.6 实现 tasks 查询与状态对齐
- [ ] 2.7 实现 tenant/project 权限校验
- [ ] 2.8 实现审计日志与 trace 注入
- [ ] 2.9 实现指标埋点（latency、token、success_rate）

## 3. Frontend Integration
- [ ] 3.1 新增 Context Explorer 页面与数据接线
- [ ] 3.2 新增 Retrieval Trace 面板（轨迹与排序因子展示）
- [ ] 3.3 新增 Commit Console 面板（任务状态、失败重试）
- [ ] 3.4 将现有上下文概览改为真实网关数据
- [ ] 3.5 增加 OpenViking 不可用时的降级提示与重试入口

## 4. Testing and Verification
- [ ] 4.1 单元测试：URI 映射、幂等、状态机、策略校验
- [ ] 4.2 集成测试：Gateway 与 OpenViking API 映射链路
- [ ] 4.3 集成测试：commit 全流程（创建、轮询、完成、失败）
- [ ] 4.4 E2E：消息写入 -> commit -> search 命中闭环
- [ ] 4.5 E2E：检索解释与 Trace 展示一致性
- [ ] 4.6 故障注入：上游超时、鉴权失败、任务失败回退

## 5. Rollout
- [ ] 5.1 增加 feature flag：`context_gateway_enabled`
- [ ] 5.2 灰度启用并监控核心指标
- [ ] 5.3 验证回滚路径（关闭网关回退旧通路）
- [ ] 5.4 完成发布说明与运维手册（接口、监控、排障）
