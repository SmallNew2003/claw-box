# Change: 更新模型申请选择流程

## Why
当前"更新模型申请"功能需要用户手动选择模型，缺少对已有模型实例的关联。用户无法直观地看到自己已审批的模型列表，也无法基于现有模型进行更新申请。这导致更新流程不够清晰，用户体验不佳。

## What Changes
- 在点击"更新模型申请"按钮后，先弹出对话框展示用户已审批的模型列表
- 用户从对话框中选择一个已有模型实例
- 选择后自动打开抽屉表单，并预填充该模型的基本信息
- 用户可以修改配置参数并填写更新说明
- 保存时标记为"更新"模式

## Impact
- 影响的 specs: `resource-application`
- 影响的代码: 
  - `bj-gover-llm-admin/src/views/portal/ResourceApply.vue` - 主要页面组件
  - `bj-gover-llm-admin/src/types/resource.ts` - 类型定义（可能需要扩展）
