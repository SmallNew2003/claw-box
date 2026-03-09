# detect_changes 工具详细说明

## 用途

分析 git diff 的影响范围，识别变更的符号和受影响的进程

## 基本用法

```bash
# 检测所有变更的影响（包括未暂存和已暂存）
detect_changes({scope: "all"})

# 只检测已暂存的变更
detect_changes({scope: "staged"})
```

## 输出示例

```
summary:
  changed_count: 12          # 变更的符号数量
  affected_count: 3          # 受影响的符号数量
  changed_files: 4           # 变更的文件数量
  risk_level: medium         # 风险等级（low/medium/high）

changed_symbols:
  - name: validateUser
    type: Function
    filePath: src/auth/validate.ts
    change_type: modified
  
  - name: AuthConfig
    type: Interface
    filePath: src/types/auth.ts
    change_type: added

affected_symbols:
  - name: handleLogin
    type: Function
    filePath: src/api/auth.ts
    reason: calls validateUser
  
  - name: handleRegister
    type: Function
    filePath: src/api/auth.ts
    reason: calls validateUser

affected_processes:
  - name: LoginFlow
    affected_steps: [2, 3]
    risk: high
  
  - name: RegistrationFlow
    affected_steps: [3]
    risk: medium
```

## 何时使用

1. **提交前检查影响范围**
   - 确认变更的影响范围
   - 避免意外破坏其他功能

2. **OpenSpec 提案前评估风险**
   - 分析变更的复杂度
   - 决定是否需要创建提案

3. **Code review 时分析影响**
   - 快速了解 PR 的影响范围
   - 识别潜在的风险点

## 解读结果

### summary 部分
- **changed_count**: 直接修改的符号数量
- **affected_count**: 间接受影响的符号数量
- **changed_files**: 变更的文件数量
- **risk_level**: 风险等级
  - **low**: 影响范围小，可以直接提交
  - **medium**: 影响范围中等，建议仔细测试
  - **high**: 影响范围大，建议创建 OpenSpec 提案

### changed_symbols 部分
- 直接修改的符号列表
- **change_type**: added（新增）、modified（修改）、removed（删除）

### affected_symbols 部分
- 间接受影响的符号列表
- **reason**: 受影响的原因（如 "calls validateUser"）

### affected_processes 部分
- 受影响的执行流程
- **affected_steps**: 流程中受影响的步骤
- **risk**: 该流程的风险等级

## 典型场景

### 场景 1：提交前检查

```bash
# 修改了几个文件，想确认影响范围
detect_changes({scope: "all"})

# 结果显示：
# risk_level: low
# affected_count: 2
# → 影响范围小，可以直接提交
```

### 场景 2：决定是否需要 OpenSpec 提案

```bash
# 修改了核心函数，想评估影响
detect_changes({scope: "staged"})

# 结果显示：
# risk_level: high
# affected_count: 47
# affected_processes: [LoginFlow, RegistrationFlow, PasswordResetFlow]
# → 影响范围大，需要创建 OpenSpec 提案
```

### 场景 3：Code review

```bash
# 审查 PR 时，快速了解影响范围
detect_changes({scope: "all"})

# 结果显示：
# changed_symbols: [validateUser, checkPassword]
# affected_processes: [LoginFlow]
# → 重点测试登录流程
```

## 风险等级判断标准

| 风险等级 | 条件 | 建议 |
|---------|------|------|
| **low** | affected_count < 5 且 affected_processes < 2 | 直接提交 |
| **medium** | affected_count 5-20 或 affected_processes 2-5 | 仔细测试 |
| **high** | affected_count > 20 或 affected_processes > 5 | 创建 OpenSpec 提案 |

## 与其他工具的区别

| 工具 | 用途 | 输入 |
|------|------|------|
| **detect_changes** | 分析 git diff 影响 | git diff |
| **impact** | 分析符号影响 | 符号名称 |

**选择建议**：
- 提交前检查 → 用 `detect_changes`
- 评估单个符号的影响 → 用 `impact`

## 注意事项

1. **scope 参数**：
   - `all`: 包括未暂存和已暂存的变更
   - `staged`: 只包括已暂存的变更（git add 后的文件）

2. **风险等级**：仅供参考，最终决策需要人工判断

3. **增量索引**：如果代码变更后没有重新索引，结果可能不准确，建议先运行 `npx gitnexus analyze`

4. **多仓库**：在子项目中运行时，自动检测当前仓库
