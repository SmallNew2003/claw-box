# 模块开发指南

本指南面向模块开发者，说明模块化分层、依赖边界与样板模块实践。

## 分层与边界
- domain：领域模型、聚合根、领域服务，仅依赖 shared-kernel。
- application：用例服务、命令/查询、DTO，依赖 domain 与 shared-kernel。
- interfaces：接口适配层（REST/事件入站等），依赖 application。
- infrastructure：外部资源适配（仓储、消息、流程、宿主能力），依赖 shared-kernel 与 ports。

禁止事项：
- 模块不得直接依赖宿主实现（如 szadmin 或具体中间件 SDK）。
- 模块间不允许直接调用内部实现，只允许通过应用层契约或共享接口交互。

## 依赖约定
- 必须依赖 `shared-kernel`，以获取上下文、端口与基础值对象。
- 需要宿主能力时，通过 ports 接口调用，例如 `AuthPort`、`TenantPort`、`AuditPort`、`WorkflowPort`、`MessagingPort`。

## 样板模块参考
当前样板模块选用 `approval-workflow`：
- 领域模型：`bj-gover-llm-service/modules/approval-workflow/src/main/java/com/bjgov/approvalworkflow/domain`
- 用例服务：`bj-gover-llm-service/modules/approval-workflow/src/main/java/com/bjgov/approvalworkflow/application`
- 接口层：`bj-gover-llm-service/modules/approval-workflow/src/main/java/com/bjgov/approvalworkflow/interfaces`
- 基础设施装配：`bj-gover-llm-service/modules/approval-workflow/src/main/java/com/bjgov/approvalworkflow/infrastructure`

## 典型用例流程
1) 在 application 层定义命令对象与用例服务。
2) 在用例服务中读取 Auth/Tenant 上下文，校验权限。
3) 通过 WorkflowPort 启动流程，并记录审计事件。
4) 发布领域事件到 MessagingPort。

## 测试规范
- 样板模块单元测试覆盖权限校验、流程启动、审计记录、消息发布。
- 测试位于模块 `src/test/java` 下，使用 `spring-boot-starter-test`。

## 扩展建议
- 新模块尽量复用 shared-kernel 中的通用能力（如方言解析）。
- 遇到数据库差异时，隔离在 infrastructure 层。
