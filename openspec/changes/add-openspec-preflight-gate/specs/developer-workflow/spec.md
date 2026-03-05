## ADDED Requirements

### Requirement: OpenSpec 预检硬闸
系统 SHALL 在新功能或破坏性变更提交前执行 OpenSpec 预检，未通过校验则阻断提交。

#### Scenario: 提交前预检
- **WHEN** 开发者提交包含功能改动的变更
- **THEN** 系统执行 OpenSpec 预检并要求通过验证
