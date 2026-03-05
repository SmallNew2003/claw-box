# 宿主适配指南

本指南面向宿主系统维护者，说明 ports/adapters 的接入方式与装配规范。

## 适配原则
- 业务模块只依赖 ports 接口，宿主通过 adapters 实现对接。
- 适配层禁止泄露宿主内部模型，必要时在 adapter 内做映射。

## 适配范围
- 鉴权与 RBAC：`AuthPort`
- 租户隔离：`TenantPort`
- 审计日志：`AuditPort`
- 工作流引擎：`WorkflowPort`
- 消息中间件：`MessagingPort`

## 现有适配实现
- szadmin 鉴权/租户/审计：
  - `bj-gover-llm-service/host-platform/src/main/java/com/bjgov/host/platform/adapter/szadmin`
- 飞龙工作流：
  - `bj-gover-llm-service/host-platform/src/main/java/com/bjgov/host/platform/adapter/workflow`
- 东方通消息：
  - `bj-gover-llm-service/host-platform/src/main/java/com/bjgov/host/platform/adapter/messaging`

## 装配规范
- 统一在 host-platform 中通过 Spring 配置注册客户端与适配器。
- 注册表默认使用空实现，避免宿主未装配时中断业务流程。

## 关键装配入口
- `FeilongWorkflowConfiguration`：注册飞龙工作流客户端。
- `TongTechMessagingConfiguration`：注册东方通消息客户端。

## 接入步骤
1) 引入对应中间件或宿主 SDK 依赖。
2) 实现对应 Client（如 `FeilongWorkflowClient`、`TongTechMessagingClient`）。
3) 在 Spring 配置中注册实现到 Registry。
4) 验证 adapters 的 publish/start/complete 等调用链路。

## 注意事项
- 适配层不做业务判断，专注于协议与数据映射。
- 发生异常时需提供清晰的失败原因，避免默默吞错。
