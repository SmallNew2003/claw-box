# GitNexus 工作流集成

## 与 openspec-planning 配合

### 场景：创建 OpenSpec 提案

```
用户: "我想修改 UserService.validate() 的返回类型"
  ↓
using-superpowers: 检测到破坏性变更
  ↓
openspec-planning: 开始创建 proposal.md
  ↓
写到 Impact 部分时，调用 gitnexus-context:
  1. 检查 GitNexus 是否可用：npx gitnexus status
  2. 使用 impact 工具分析影响范围：
     impact({target: "UserService.validate", direction: "upstream"})
  3. 结果显示：47 个调用方，分布在 auth、user、admin 模块
  ↓
在 proposal.md 的 Impact 部分写入：
  ## Impact
  - Affected specs: user-auth, user-management, admin-operations
  - Affected code: 47 个调用方
    - auth 模块：handleLogin, handleRegister (src/api/auth.ts)
    - user 模块：UserController, UserService (src/controllers/user.ts)
    - admin 模块：AdminUserService (src/admin/user.ts)
  ↓
继续 openspec-planning 流程
```

### 工具选择

- **impact**: 分析变更影响范围，列出所有依赖方
- **detect_changes**: 如果已经修改了代码，分析 git diff 影响

## 与 systematic-debugging 配合

### 场景：调试 Bug

```
用户: "登录失败，但没有错误日志"
  ↓
systematic-debugging: Phase 1 Root Cause Investigation
  ↓
调用 gitnexus-context:
  1. 使用 context 工具追踪调用链：
     context({name: "handleLogin"})
     → 看到：handleLogin → validateUser → checkPassword → createSession
  
  2. 使用 query 工具查找工作示例：
     query({query: "login error handling"})
     → 找到其他模块的错误处理代码
  ↓
systematic-debugging: Phase 2 Pattern Analysis
  对比工作示例和问题代码，找到差异
  ↓
systematic-debugging: Phase 3 Hypothesis and Testing
  形成假设并测试
```

### 工具选择

- **context**: 追踪函数调用链，找到 bug 传播路径
- **query**: 查找工作示例，对比正常代码和问题代码

## 与代码重构配合

### 场景：重构前评估

```
用户: "想重构 checkPassword 函数"
  ↓
调用 gitnexus-context:
  1. 使用 impact 工具评估影响：
     impact({target: "checkPassword", direction: "upstream"})
     → 发现 3 个调用方
  
  2. 使用 context 工具查看上下文：
     context({name: "checkPassword"})
     → 看到参与的进程：LoginFlow, PasswordResetFlow, ChangePasswordFlow
  ↓
决策：
  - 如果影响范围小（< 5 个调用方）→ 直接重构
  - 如果影响范围大（> 20 个调用方）→ 创建 OpenSpec 提案
```

### 工具选择

- **impact**: 评估重构影响范围
- **context**: 查看函数在系统中的位置
- **detect_changes**: 重构后检查实际影响

## 工作流决策树

```
需要分析代码依赖？
  ↓
已知符号名称？
  ↓ 是
  需要评估变更影响？
    ↓ 是
    使用 impact 工具
    ↓ 否
    使用 context 工具
  ↓ 否
  使用 query 工具搜索
  ↓
已经修改了代码？
  ↓ 是
  使用 detect_changes 工具
```

## 最佳实践

1. **OpenSpec 提案**：
   - 先用 `impact` 分析影响范围
   - 将结果写入 proposal.md 的 Impact 部分
   - 包含具体的文件路径和行号

2. **Systematic Debugging**：
   - Phase 1 用 `context` 追踪调用链
   - Phase 2 用 `query` 查找工作示例
   - 对比差异，形成假设

3. **代码重构**：
   - 重构前用 `impact` 评估风险
   - 重构后用 `detect_changes` 验证影响
   - 根据影响范围决定是否需要 OpenSpec 提案

4. **提交前检查**：
   - 使用 `detect_changes` 检查影响范围
   - 如果 risk_level 为 high，考虑创建 OpenSpec 提案
