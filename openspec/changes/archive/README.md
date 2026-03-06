# 归档变更

本目录存放已完成的 OpenSpec changes。

## 目录结构

每个归档的 change 应该有以下结构：

```
YYYY-MM-DD-<change-id>/
├── proposal.md      # 变更提案
├── tasks.md         # 实施任务清单
└── specs/           # 规范增量
    └── <capability>/
        └── spec.md  # 规范定义
```

## 归档规范

### 目录命名

- 格式：`YYYY-MM-DD-<change-id>`
- 日期：归档日期（通常是完成日期）
- change-id：kebab-case，动词开头

### 归档时机

满足以下条件时归档：
- 所有任务已完成（tasks.md 中所有任务标记为 [x]）
- 代码已合并到主分支（如有代码变更）
- 所有测试通过
- 功能已部署或明确不需要部署

### 归档流程

1. 确认完成条件
2. 运行验证：
   ```bash
   openspec validate <change-id> --strict --no-interactive
   ```
3. 执行归档：
   ```bash
   openspec archive <change-id> --yes
   ```
4. 更新 `openspec/specs/` 中的相关规范
5. 在 `openspec/CHANGELOG.md` 中记录变更
6. 提交归档变更

## 查看归档

列出所有归档的 changes：
```bash
ls -1 openspec/changes/archive/
```

查看特定归档的内容：
```bash
cat openspec/changes/archive/YYYY-MM-DD-<change-id>/proposal.md
```

## 注意事项

- 归档后的 change 不应再修改
- 如需修改已归档的功能，创建新的 change
- 归档目录保留完整的变更历史，便于追溯
