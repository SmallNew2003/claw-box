## ADDED Requirements

### Requirement: 动态任务分配
系统 SHALL 支持 Leader 根据任务复杂度动态分配 Worker 数量。

#### Scenario: Leader 评估任务复杂度
- **WHEN** Leader 接收到任务（如处理 100 张图片）
- **THEN** Leader 分析任务规模和复杂度
- **AND** 计算所需 Worker 数量（如 5 个）
- **AND** 发送 run.workers_allocated 事件

#### Scenario: Leader 动态扩容
- **WHEN** Leader 判断任务需要 5 个 Worker
- **THEN** 系统创建 5 个 Worker 实例
- **AND** Leader 将任务分片（每个 Worker 处理 20 张图片）
- **AND** 并行分配任务给所有 Worker

#### Scenario: 任务分片策略
- **WHEN** Leader 分配任务
- **THEN** 系统按照负载均衡策略分片
- **AND** 每个 Worker 获得大致相等的工作量
- **AND** 记录每个 Worker 的任务范围

### Requirement: Worker 请求支援
系统 SHALL 支持 Worker 在任务过重时向 Leader 请求增援。

#### Scenario: Worker 检测负载过重
- **WHEN** Worker 执行任务时发现工作量超出预期
- **THEN** Worker 向 Leader 发送 worker.request_help 消息
- **AND** 消息包含当前进度和剩余工作量估算

#### Scenario: Leader 响应支援请求
- **WHEN** Leader 收到 Worker 的支援请求
- **THEN** Leader 评估是否需要增加 Worker
- **AND** 如果需要，创建新 Worker 并重新分配任务
- **AND** 发送 run.workers_reallocated 事件

#### Scenario: 任务重新分配
- **WHEN** Leader 决定增加 Worker
- **THEN** 系统暂停原 Worker 的任务
- **AND** 将剩余任务重新分片
- **AND** 分配给原 Worker 和新 Worker
- **AND** 所有 Worker 继续执行

### Requirement: Worker 协商机制
系统 SHALL 支持 Worker 与 Leader 协商任务分配。

#### Scenario: Worker 提出协商
- **WHEN** Worker 评估任务后认为需要调整
- **THEN** Worker 向 Leader 发送 worker.negotiate 消息
- **AND** 消息包含建议的 Worker 数量和理由

#### Scenario: Leader 接受协商
- **WHEN** Leader 收到协商请求且认为合理
- **THEN** Leader 调整 Worker 数量
- **AND** 重新分配任务
- **AND** 发送 run.plan_adjusted 事件

#### Scenario: Leader 拒绝协商
- **WHEN** Leader 收到协商请求但认为不必要
- **THEN** Leader 发送 worker.negotiate_rejected 消息
- **AND** Worker 继续执行原任务

### Requirement: 动态缩容
系统 SHALL 支持任务完成后自动释放空闲 Worker。

#### Scenario: Worker 完成任务
- **WHEN** Worker 完成分配的所有任务
- **THEN** Worker 向 Leader 报告完成状态
- **AND** Worker 进入空闲状态

#### Scenario: 释放空闲 Worker
- **WHEN** 所有任务完成且有空闲 Worker
- **THEN** 系统释放多余的 Worker 实例
- **AND** 只保留最小数量的 Worker（如 1 个）

### Requirement: 上下文管理优化
系统 SHALL 采用主代理+子代理架构优化 Token 消耗。

#### Scenario: Leader 保存任务摘要
- **WHEN** Leader 分配任务
- **THEN** Leader 只保存任务摘要、分配方案、Worker 结果
- **AND** 不保存完整的任务数据

#### Scenario: Worker 接收最小上下文
- **WHEN** Worker 启动
- **THEN** Worker 只接收任务片段和必要的执行上下文
- **AND** 不接收完整的任务数据

#### Scenario: Token 消耗优化
- **GIVEN** 任务包含 100 张图片的元数据
- **WHEN** 使用主代理+子代理架构
- **THEN** 每个 Worker 只携带 20 张图片的元数据
- **AND** Token 消耗减少约 60-70%

## MODIFIED Requirements

### Requirement: 团队创建与配置
系统 SHALL 支持创建团队并配置协作模式、角色与扩缩容策略。

#### Scenario: 创建团队时配置扩缩容策略
- **WHEN** 用户创建团队
- **THEN** 系统允许配置 minWorkers、maxWorkers、autoScale
- **AND** 系统允许配置 taskComplexityThreshold

#### Scenario: 创建顺序模式团队
- **WHEN** 用户创建 sequential 模式团队
- **THEN** 系统创建团队并设置角色顺序
- **AND** 第一个角色为 Leader

#### Scenario: 创建并行模式团队
- **WHEN** 用户创建 parallel 模式团队
- **THEN** 系统创建团队并允许所有角色并行执行
- **AND** 配置动态 Worker 分配策略

#### Scenario: 创建专家模式团队
- **WHEN** 用户创建 expert 模式团队
- **THEN** 系统创建团队并配置路由规则
- **AND** 根据任务类型自动选择专家

#### Scenario: 创建流水线模式团队
- **WHEN** 用户创建 pipeline 模式团队
- **THEN** 系统创建团队并定义数据流转规则
- **AND** 每个角色的输出作为下一个角色的输入

### Requirement: Run 执行与监控
系统 SHALL 支持启动 Run 并实时展示执行进度，包括 Worker 分配和任务分片信息。

#### Scenario: 启动 Run
- **WHEN** 用户启动团队任务
- **THEN** 系统创建 Run 并开始执行
- **AND** Run 状态为 running
- **AND** 发送 run.started 事件
- **AND** Leader 评估任务并分配 Worker

#### Scenario: Run 执行中
- **WHEN** Run 正在执行
- **THEN** 系统实时推送 step.started 和 step.completed 事件
- **AND** 系统推送 worker.request_help 和 run.workers_reallocated 事件
- **AND** 更新 Run 进度和 Worker 状态

#### Scenario: Run 完成
- **WHEN** 所有 Step 执行完成
- **THEN** Run 状态变为 completed
- **AND** 发送 run.completed 事件
- **AND** 聚合所有 Step 结果
- **AND** 释放空闲 Worker

#### Scenario: Run 失败
- **WHEN** 某个 Step 执行失败
- **THEN** Run 状态变为 failed
- **AND** 发送 run.failed 事件
- **AND** 记录失败原因
- **AND** 释放所有 Worker

#### Scenario: 取消 Run
- **WHEN** 用户取消正在执行的 Run
- **THEN** 系统停止所有未完成的 Step
- **AND** Run 状态变为 cancelled
- **AND** 释放所有 Worker
