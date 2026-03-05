# impact 工具详细说明

## 用途

查看符号的依赖关系和影响范围，回答"谁依赖这个？"和"这个依赖谁？"

## 基本用法

```bash
# 查看谁依赖这个函数（上游 - 会被影响的代码）
impact({target: "UserService", direction: "upstream"})

# 查看这个函数依赖谁（下游 - 它依赖的代码）
impact({target: "validateUser", direction: "downstream"})
```

## 高级参数

```bash
impact({
  target: "AuthService",           # 目标符号名称
  direction: "upstream",            # upstream（谁依赖我）或 downstream（我依赖谁）
  maxDepth: 3,                      # 最大追踪深度（默认 2）
  minConfidence: 0.7,               # 最小置信度（0-1，默认 0.8）
  relationTypes: ["CALLS", "IMPORTS"], # 关系类型过滤
  includeTests: false,              # 是否包含测试文件（默认 false）
  repo: "bj-gover-llm"             # 多仓库时指定仓库
})
```

## 输出示例

```
TARGET: Class UserService (src/services/user.ts)

UPSTREAM (what depends on this):
  Depth 1 (WILL BREAK):
    handleLogin [CALLS 90%] -> src/api/auth.ts:45
    handleRegister [CALLS 90%] -> src/api/auth.ts:78
    UserController [CALLS 85%] -> src/controllers/user.ts:12
  
  Depth 2 (LIKELY AFFECTED):
    authRouter [IMPORTS] -> src/routes/auth.ts
    apiGateway [IMPORTS] -> src/gateway/index.ts
```

## 何时使用

1. **OpenSpec proposal 写 Impact 部分**
   - 评估破坏性变更的影响范围
   - 列出所有受影响的模块和文件

2. **重构前评估风险**
   - 查看有多少代码依赖当前符号
   - 决定是否需要创建 OpenSpec 提案

3. **理解代码依赖关系**
   - 追踪函数的调用链
   - 了解模块间的耦合度

## 解读结果

- **Depth 1 (WILL BREAK)**: 直接依赖，修改目标符号会立即影响这些代码
- **Depth 2 (LIKELY AFFECTED)**: 间接依赖，可能需要适配修改
- **置信度 90%+**: 高置信度，确定存在依赖关系
- **置信度 70-90%**: 中等置信度，建议人工确认
- **置信度 <70%**: 低置信度，可能是误报

## 典型场景

### 场景 1：评估 API 修改影响

```bash
# 要修改 UserService.validate() 的返回类型
impact({target: "UserService.validate", direction: "upstream", minConfidence: 0.8})

# 结果显示 47 个调用方
# → 在 proposal.md 写入：
#   "影响 47 个调用方，分布在 auth、user、admin 三个模块"
```

### 场景 2：重构前检查

```bash
# 想删除一个工具函数
impact({target: "formatDate", direction: "upstream"})

# 结果显示 3 个调用方
# → 可以安全删除，不需要 OpenSpec 提案
```

### 场景 3：理解依赖链

```bash
# 想知道 handleLogin 依赖哪些底层函数
impact({target: "handleLogin", direction: "downstream", maxDepth: 3})

# 结果显示完整的依赖树
# → 理解登录流程的实现细节
```

## 注意事项

1. **置信度阈值**：默认 0.8 适合大多数场景，降低阈值会增加结果数量但可能包含误报
2. **深度控制**：深度越大结果越多，建议从 2 开始，按需增加
3. **测试文件**：默认排除测试文件，如需包含设置 `includeTests: true`
4. **多仓库**：如果索引了多个仓库，必须指定 `repo` 参数
