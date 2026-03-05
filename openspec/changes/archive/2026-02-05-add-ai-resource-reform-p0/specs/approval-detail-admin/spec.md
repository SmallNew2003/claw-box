## ADDED Requirements
### Requirement: 审批详情字段展示补齐（管理端）
审批详情页面 SHALL 展示 P0 要求的新增申请字段与关键业务信息。

#### Scenario: 审批详情字段完整
- **WHEN** 管理员打开审批详情
- **THEN** 页面展示场景目录、场景分类、申请模式、QPS/TPM、tokens、用户规模、知识库类型与数据量

### Requirement: 审批意见必填提示
审批详情页面 SHALL 在审批意见输入区域提供必填提示（界面提示即可）。

#### Scenario: 审批意见提示可见
- **WHEN** 管理员准备提交审批意见
- **THEN** 页面显示审批意见必填提示文案
