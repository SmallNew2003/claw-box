# GitNexus 故障排查

## 常见问题

### 问题 1：GitNexus 未安装

**症状**：
```bash
npx gitnexus status
# 输出：command not found 或 No index found
```

**解决方案**：
```bash
# 运行安装脚本
bash .opencode/skills/gitnexus-context/install.sh
```

---

### 问题 2：索引过期

**症状**：
- 工具返回的结果不包含最新代码
- 新增的函数找不到

**解决方案**：
```bash
# 重新索引当前项目
npx gitnexus analyze --skip-embeddings

# 强制完全重新索引
npx gitnexus analyze --force --skip-embeddings
```

---

### 问题 3：MCP 工具不可用

**症状**：
- AI 无法调用 impact、context 等工具
- 提示 "Tool not found"

**解决方案**：
```bash
# 重新配置 MCP
npx gitnexus setup

# 检查 MCP 配置文件
cat ~/.config/opencode/config.json  # OpenCode
cat ~/.cursor/mcp.json              # Cursor

# 重启编辑器
```

---

### 问题 4：多仓库场景

**症状**：
- 工具返回错误的仓库数据
- 提示 "Multiple repositories found"

**解决方案**：
```bash
# 列出所有已索引的仓库
npx gitnexus list

# 在工具调用时明确指定仓库
impact({target: "UserService", repo: "bj-gover-llm"})
context({name: "validateUser", repo: "bj-gover-llm-admin"})
```

---

### 问题 5：索引速度慢

**症状**：
- `npx gitnexus analyze` 运行很久
- 占用大量内存

**解决方案**：
```bash
# 跳过向量化（推荐，快 3-5 倍）
npx gitnexus analyze --skip-embeddings

# 只索引特定目录
npx gitnexus analyze src/

# 排除大型目录
# 编辑 .gitignore，GitNexus 会自动排除这些目录
```

---

### 问题 6：Node.js 版本不兼容

**症状**：
```bash
npx gitnexus analyze
# 输出：Error: Requires Node.js >= 18
```

**解决方案**：
```bash
# 检查 Node.js 版本
node --version

# 升级 Node.js（使用 nvm）
nvm install 18
nvm use 18
```

---

### 问题 7：索引损坏

**症状**：
- 工具返回空结果
- 提示 "Database error"

**解决方案**：
```bash
# 清理索引
npx gitnexus clean

# 重新索引
npx gitnexus analyze --skip-embeddings
```

---

## 诊断命令

```bash
# 检查当前项目索引状态
npx gitnexus status

# 列出所有已索引的仓库
npx gitnexus list

# 查看 GitNexus 版本
npx gitnexus --version

# 清理当前项目索引
npx gitnexus clean

# 清理所有索引
npx gitnexus clean --all --force
```

---

## 获取帮助

1. **查看 GitNexus 文档**：https://github.com/abhigyanpatwari/GitNexus
2. **在项目中创建 OpenSpec 提案**：讨论集成问题
3. **检查 MCP 日志**：编辑器的开发者工具中查看 MCP 通信日志
