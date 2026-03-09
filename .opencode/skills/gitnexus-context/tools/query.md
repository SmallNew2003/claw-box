# query 工具详细说明

## 用途

按进程分组的智能搜索，结合 BM25 和语义搜索，找到相关代码

## 基本用法

```bash
# 搜索相关代码
query({query: "authentication middleware"})

# 多仓库时指定仓库
query({query: "error handling", repo: "bj-gover-llm-admin"})
```

## 输出示例

```
processes:
  - summary: "LoginFlow"
    priority: 0.042
    symbol_count: 4
    process_type: cross_community
    step_count: 7

process_symbols:
  - name: validateUser
    type: Function
    filePath: src/auth/validate.ts
    process_id: proc_login
    step_index: 2
  
  - name: checkPassword
    type: Function
    filePath: src/auth/password.ts
    process_id: proc_login
    step_index: 3

definitions:
  - name: AuthConfig
    type: Interface
    filePath: src/types/auth.ts
  
  - name: UserCredentials
    type: Interface
    filePath: src/types/user.ts
```

## 何时使用

1. **Systematic debugging Phase 2**
   - 查找工作示例
   - 对比正常代码和问题代码

2. **探索相关功能**
   - 了解某个功能的实现方式
   - 查找类似的代码模式

3. **学习代码库**
   - 快速定位相关代码
   - 理解业务流程

## 解读结果

### processes 部分
- **summary**: 进程名称（如 LoginFlow）
- **priority**: 相关性评分（越高越相关）
- **symbol_count**: 进程包含的符号数量
- **process_type**: 进程类型（cross_community 表示跨模块）
- **step_count**: 进程步骤数

### process_symbols 部分
- 进程中的具体符号
- 包含文件路径和在进程中的位置

### definitions 部分
- 相关的类型定义、接口、类等
- 不属于任何进程，但与查询相关

## 典型场景

### 场景 1：查找工作示例

```bash
# 想实现错误处理，查找现有的错误处理代码
query({query: "error handling middleware"})

# 结果显示：
# processes: [ErrorHandlingFlow, RequestValidationFlow]
# process_symbols: [handleError, logError, sendErrorResponse]
# → 参考这些代码实现新的错误处理
```

### 场景 2：调试时对比

```bash
# 登录失败，查找其他认证相关的代码
query({query: "user authentication"})

# 结果显示：
# processes: [LoginFlow, RegistrationFlow, PasswordResetFlow]
# → 对比这些流程，找到问题代码的差异
```

### 场景 3：探索功能

```bash
# 想了解支付功能的实现
query({query: "payment processing"})

# 结果显示：
# processes: [PaymentFlow, RefundFlow]
# process_symbols: [processPayment, validateCard, createTransaction]
# → 快速了解支付功能的架构
```

## 搜索技巧

1. **使用具体的术语**：
   - ✅ "authentication middleware"
   - ❌ "auth stuff"

2. **组合关键词**：
   - ✅ "user login validation"
   - ❌ "login"（太宽泛）

3. **使用业务术语**：
   - ✅ "order payment processing"
   - ✅ "user registration flow"

4. **英文搜索效果更好**：
   - GitNexus 的语义搜索基于英文模型训练

## 与其他工具的区别

| 工具 | 用途 | 输出 |
|------|------|------|
| **query** | 搜索相关代码 | 按进程分组的搜索结果 |
| **context** | 查看符号上下文 | 单个符号的完整信息 |
| **impact** | 分析变更影响 | 依赖树 |

**选择建议**：
- 不知道从哪里开始 → 用 `query` 搜索
- 已知符号名称 → 用 `context` 查看详情
- 评估变更影响 → 用 `impact` 分析

## 注意事项

1. **结果按相关性排序**：priority 越高越相关，优先查看高分结果
2. **进程分组**：结果按执行流程分组，有助于理解业务逻辑
3. **语义搜索**：支持自然语言查询，不需要精确匹配符号名称
4. **多仓库**：索引了多个仓库时，建议指定 `repo` 参数缩小范围
