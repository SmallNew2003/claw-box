# AI Agent 工作指南

<!-- 规范版本: v1.0.0 -->
<!-- 最后更新: 2026-03-06 -->
<!-- 维护者: 开发团队 -->

> **详细文档**：完整的工作流程和技能说明请参考 `openspec/AGENTS.md`

## 强制规则（每次必读）

### 规则 1：调用 using-superpowers
**收到用户消息 → 立即调用 `using-superpowers` skill**

- 即使只有 1% 可能性也必须检查
- 在做任何响应（包括澄清问题）之前先执行
- 这是强制要求，不可跳过

### 规则 2：禁止使用 emoji
所有输出（代码、文档、注释、对话）禁止使用 emoji。

**常见违规示例**：
- 状态标记：禁止 ✅ ❌ ⚠️，使用 [正确] [错误] [警告]
- 装饰符号：禁止 📋 🎯 🔧 📊 📝 🚀 📚 ⏱️ 📦

### 规则 3：禁止创建临时文档
禁止创建以下文件：
- `*_SUMMARY.md` - 总结文档
- `*_ANALYSIS.md` - 分析文档
- `*_REVIEW.md` - 评审文档
- `*_NOTES.md` - 笔记文档
- `TEMP_*.md`、`DRAFT_*.md` - 临时文档

**例外**：用户明确要求创建时可以创建

### 规则 4：文档位置
创建或修改文档前，必须先查阅 `openspec/DOCUMENTATION.md` 确认正确的文档位置。

**核心原则**：
- 需求规范 → `openspec/specs/`
- 技术设计 → `openspec/changes/<change-id>/design.md`
- 项目上下文 → `openspec/project.md`（不放详细规范）
- 详细规范 → `openspec/specs/<capability>/spec.md`

**详细规则**：参考 `openspec/DOCUMENTATION.md`

### 规则 5：提交格式
- 格式：`feat(模块): 中文内容`
- 使用中文回复
- 新功能/破坏性变更必须先通过 OpenSpec 流程

### 规则 6：提供选项时必须等待用户选择
当向用户展示多个方案、选项或替代方案时：

- **必须等待用户明确选择**，不得自行决定
- **禁止假设用户意图**，即使某个选项被标记为"推荐"
- **禁止开始执行**任何方案，直到用户明确批准

**常见违规示例**：
- 展示方案 A、B、C 后，直接开始实现方案 A
- "我推荐方案 B，现在开始实现..." → 错误，必须等待用户选择
- "基于你的需求，我选择方案 C..." → 错误，用户必须自己选择

**正确做法**：
- 展示所有方案及其权衡
- 给出推荐并说明理由
- 明确询问："你想选择哪个方案？"
- 等待用户回复后再继续

---

## 自我检查清单（完成前必做）

```bash
# 1. 检查 emoji 和文档规范
./tools/validate-docs.sh

# 2. 检查临时文档
ls *_SUMMARY.md *_ANALYSIS.md *_REVIEW.md *_NOTES.md 2>/dev/null

# 3. 验证 OpenSpec（如果修改了 changes/）
openspec validate <change-id> --strict --no-interactive

# 4. 运行测试（如果修改了代码）
mvn test

# 5. 检查技能调用（如适用）
# 确认是否调用了 using-superpowers
# 确认是否按照 skill 指引执行
grep -r "using-superpowers" .opencode/skills/
```

---

## 反模式警告（Red Flags）

如果你有以下想法，立即停止 - 你在合理化跳过规则：

| 想法 | 问题 |
|------|------|
| "这只是个简单问题" | 简单也要调用 skill |
| "我需要先了解上下文" | Skill 会告诉你如何了解 |
| "让我先探索代码库" | Skills 告诉你如何探索 |
| "我记得这个 skill" | Skills 会演进，重新读取 |
| "这不需要正式的 skill" | 如果 skill 存在，就使用它 |
| "我先做这一件事" | 先检查 skills 再做任何事 |
| "创建个总结文档" | 禁止，直接回复用户 |
| "加个 emoji 更清晰" | 禁止，用文本 |
| "用户肯定想要推荐方案" | 必须等待用户明确选择 |
| "我帮用户选个最好的" | 用户必须自己决定 |
| "不用查 DOCUMENTATION.md" | 创建文档前必须确认位置 |

---

## 常用 Skills 速查

| 场景 | 调用 Skill |
|------|-----------|
| 收到任何消息 | `using-superpowers` |
| 新功能（需求不明确） | `brainstorming` → `openspec-planning` |
| 新功能（需求明确） | `openspec-planning` |
| Bug 修复 | `systematic-debugging` |
| 完成前验证 | `verification-before-completion` |
| 代码审查 | `requesting-code-review` |
| 并行任务 | `dispatching-parallel-agents` |

---

## 常用文档速查

| 需求 | 文档位置 |
|------|---------|
| 项目上下文、技术栈 | `openspec/project.md` |
| 文档组织规范 | `openspec/DOCUMENTATION.md` |
| OpenSpec 完整流程 | `openspec/AGENTS.md` |
| 规范变更历史 | `openspec/CHANGELOG.md` |
| 项目特定规范 | `openspec/specs/*/spec.md` |

## 代码索引

如果项目配置了代码索引工具（如 GitNexus），详见对应的 skill 文档：
- `gitnexus-context` - GitNexus 代码知识图谱
