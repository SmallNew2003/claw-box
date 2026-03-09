# OpenSpec 命令参考

## 查看命令

### 列出活跃的 changes
```bash
openspec list
```

### 列出所有 capabilities (specs)
```bash
openspec list --specs
```

### 显示详情
```bash
# 显示 change 或 spec 的详情
openspec show <item>

# 仅显示 change 的增量部分
openspec show <change-id> --json --deltas-only
```

## 验证命令

### 严格验证 change
```bash
openspec validate <change-id> --strict --no-interactive
```

在分享提案给用户前，必须运行此命令并修复所有问题。

## 归档命令

### 归档已完成的 change
```bash
openspec archive <change-id> --yes
```

在实现完成、测试通过、代码合并后执行。

## 搜索命令

### 枚举 specs
```bash
# 详细列表
openspec spec list --long

# JSON 格式（用于脚本）
openspec spec list --json
```

### 枚举 changes
```bash
openspec list

# JSON 格式（已废弃但可用）
openspec change list --json
```

### 显示详情
```bash
# 显示 spec 详情
openspec show <spec-id> --type spec

# 显示 change 的增量
openspec show <change-id> --json --deltas-only
```

### 全文搜索

使用 grep 工具搜索 openspec 目录：

```bash
# 搜索 Requirements 和 Scenarios
grep -rn "Requirement:|Scenario:" openspec/specs

# 搜索变更标记
grep -rn "ADDED|MODIFIED|REMOVED" openspec/changes
```

## 常见工作流

### 创建新 change
```bash
# 1. 检查现有工作
openspec list
openspec list --specs

# 2. 创建目录结构（手动）
mkdir -p openspec/changes/<change-id>/specs/<capability>

# 3. 编写 artifacts（手动）
# - proposal.md
# - specs/<capability>/spec.md
# - tasks.md

# 4. 验证
openspec validate <change-id> --strict --no-interactive
```

### 完成 change
```bash
# 1. 实现功能并测试通过

# 2. 归档 change
openspec archive <change-id> --yes

# 3. 提交代码
git add .
git commit -m "feat(模块): 中文描述"
```
