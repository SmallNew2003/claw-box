# 验证工具

本目录包含用于验证项目规范合规性的自动化工具。

## validate-docs.sh

检查文档规范合规性。

### 运行方式

```bash
./tools/validate-docs.sh
```

### 检查项

1. **emoji 使用检查**
   - 检测 Unicode emoji 字符
   - 检测常见 emoji 符号（如 ✅ ❌ ⚠️ 等）
   - 提示使用文本替代：[正确] [错误] [警告] [注意]

2. **禁止的文档类型检查**
   - 检测临时文档：TEMP_*.md, DRAFT_*.md
   - 检测总结文档：*_SUMMARY.md, *_ANALYSIS.md, *_REVIEW.md
   - 检测其他禁止文档：OVERVIEW.md, REPORT.md

3. **OpenSpec 结构完整性检查**
   - 检查活跃 change 是否包含必需文件（proposal.md, tasks.md, specs/）
   - 检查归档 change 的日期格式（YYYY-MM-DD-<change-id>）
   - 检查 spec.md 格式（必须包含 Scenario 和 delta 操作标记）

4. **任务状态同步检查**
   - 调用 validate-task-sync.sh 检查任务完成情况

### 退出码

- 0: 所有检查通过
- 1: 发现错误

---

## validate-task-sync.sh

检查 OpenSpec changes 的任务完成状态。

### 运行方式

```bash
./tools/validate-task-sync.sh
```

### 功能

- 遍历所有活跃 changes（跳过 archive/）
- 统计每个 change 的任务进度
- 显示未完成任务数量
- 输出总体任务统计

### 输出示例

```
检查任务状态同步...
   [add-validation-tools] 任务进度: 15/23 完成
      未完成任务: 8
   [migrate-szadmin-modules] 任务进度: 49/100 完成
      未完成任务: 51

总计: 2 个活跃 change, 64/123 任务完成
[提示] 还有 59 个任务未完成
```

---

## commit-msg hook

Git commit message 格式验证。

### 位置

`.git/hooks/commit-msg`

### 格式要求

```
type(scope): message
```

### 支持的类型

- **feat**: 新功能
- **fix**: Bug 修复
- **docs**: 文档变更
- **refactor**: 重构
- **test**: 测试
- **chore**: 构建/工具变更

### 规则

1. 必须符合 `type(scope): message` 格式
2. scope 不能为空
3. message 必须包含内容（建议使用中文）
4. 禁止使用 emoji

### 示例

正确格式：
```bash
git commit -m "feat(tools): 添加验证工具"
git commit -m "fix(api): 修复用户登录问题"
git commit -m "docs(readme): 更新安装说明"
```

错误格式：
```bash
git commit -m "添加功能"           # 缺少 type 和 scope
git commit -m "feat: 添加功能"     # 缺少 scope
git commit -m "feat(tools): 添加功能 ✅"  # 包含 emoji
```

### 错误提示

当提交格式不正确时，hook 会显示：
```
错误：提交信息格式不正确
正确格式：feat(模块): 中文内容
支持的类型：feat, fix, docs, refactor, test, chore
```

---

## 使用建议

1. **开发过程中**：定期运行 `./tools/validate-docs.sh` 检查规范
2. **提交前**：commit-msg hook 会自动验证提交信息
3. **完成任务后**：运行 `./tools/validate-task-sync.sh` 确认任务状态同步
4. **归档前**：运行所有验证工具确保规范合规

---

## 故障排查

### commit-msg hook 不生效

检查执行权限：
```bash
chmod +x .git/hooks/commit-msg
```

### validate-docs.sh 报告误报

检查文件编码是否为 UTF-8：
```bash
file -I <文件路径>
```

### validate-task-sync.sh 统计不准确

确认 tasks.md 使用正确的 checkbox 格式：
```markdown
- [ ] 未完成任务
- [x] 已完成任务
```
