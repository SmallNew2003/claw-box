# OpenClaw Dispatch - Codex 适配

## 目标

为 Codex CLI 提供与 Claude Code 类似的“结果落盘 + 唤醒”流程。

## 脚本

- `run-codex.sh`：执行 Codex，记录输出到 `latest.json` 并唤醒 OpenClaw
- `wake-openclaw.sh`：调用 OpenClaw wake 接口

## 环境变量

- `OPENCLAW_GATEWAY_TOKEN`：OpenClaw Gateway Token（推荐）
- `OPENCLAW_TOKEN`：兼容变量
- `OPENCLAW_GATEWAY_URL`：默认 `http://127.0.0.1:18789`
- `OPENCLAW_DISPATCH_DIR`：结果文件目录，默认 `.opencode/dispatch`
- `OPENCLAW_WAKE_TEXT`：唤醒文本

## 用法

```bash
export OPENCLAW_GATEWAY_TOKEN=xxx

# 例：执行任务并自动落盘 + 唤醒
./tools/openclaw-dispatch/run-codex.sh "实现会话仪表盘 UI"
```

## 输出文件格式

输出保存在 `.opencode/dispatch/latest.json`。
