# context 工具详细说明

## 用途

查看符号的完整上下文：调用方、被调用方、所属进程、文件位置

## 基本用法

```bash
# 查看函数的完整上下文
context({name: "validateUser"})

# 多仓库时指定仓库
context({name: "validateUser", repo: "bj-gover-llm"})
```

## 输出示例

```
symbol:
  uid: "Function:validateUser"
  kind: Function
  filePath: src/auth/validate.ts
  startLine: 15
  endLine: 42

incoming:
  calls: [handleLogin, handleRegister, UserController]
  imports: [authRouter, apiGateway]

outgoing:
  calls: [checkPassword, createSession, logAudit]
  imports: [bcrypt, jwt]

processes:
  - name: LoginFlow (step 2/7)
  - name: RegistrationFlow (step 3/5)
  - name: PasswordResetFlow (step 4/6)
```

## 何时使用

1. **Systematic debugging Phase 1**
   - 追踪函数的调用链
   - 找到 bug 的传播路径

2. **理解陌生代码**
   - 快速了解函数的作用
   - 查看函数在系统中的位置

3. **代码审查**
   - 检查函数的依赖关系
   - 评估代码的耦合度

## 解读结果

### symbol 部分
- **uid**: 符号的唯一标识符
- **kind**: 符号类型（Function, Class, Interface 等）
- **filePath**: 文件路径
- **startLine/endLine**: 代码位置

### incoming 部分
- **calls**: 谁调用了这个函数
- **imports**: 谁导入了这个符号

### outgoing 部分
- **calls**: 这个函数调用了谁
- **imports**: 这个函数导入了什么

### processes 部分
- 这个符号参与的执行流程
- 在流程中的位置（step X/Y）

## 典型场景

### 场景 1：调试 Bug

```bash
# 登录失败，追踪 handleLogin 的调用链
context({name: "handleLogin"})

# 结果显示：
# outgoing.calls: [validateUser, checkPassword, createSession]
# → 逐个检查这些函数，找到 bug 源头
```

### 场景 2：理解代码

```bash
# 不熟悉 validateUser 函数，想了解它的作用
context({name: "validateUser"})

# 结果显示：
# incoming.calls: [handleLogin, handleRegister]
# processes: [LoginFlow, RegistrationFlow]
# → 这是一个认证函数，被登录和注册流程使用
```

### 场景 3：评估重构

```bash
# 想重构 checkPassword 函数
context({name: "checkPassword"})

# 结果显示：
# incoming.calls: [validateUser, resetPassword, changePassword]
# → 有 3 个调用方，需要谨慎重构
```

## 与 impact 工具的区别

| 工具 | 用途 | 输出 |
|------|------|------|
| **context** | 查看符号的完整上下文 | 调用方、被调用方、所属进程 |
| **impact** | 分析变更的影响范围 | 按深度分组的依赖树 |

**选择建议**：
- 理解代码、调试 bug → 用 `context`
- 评估变更影响、写 OpenSpec proposal → 用 `impact`

## 注意事项

1. **同名符号**：如果有多个同名符号，工具会返回所有匹配项，需要根据 filePath 区分
2. **多仓库**：索引了多个仓库时，必须指定 `repo` 参数
3. **进程信息**：processes 部分显示符号参与的执行流程，有助于理解业务逻辑
