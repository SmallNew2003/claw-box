# OpenSpec Preflight 自测说明

## Dry Run

```bash
node --experimental-strip-types tools/openspec-preflight.ts --dry-run
```

预期：输出检测结果并退出 0。

## 触发校验

```bash
node --experimental-strip-types tools/openspec-preflight.ts --change-id <change-id>
```

预期：
- 若提案或规范缺失，返回非 0 并输出错误信息
- 若 `openspec validate` 通过，输出 "OpenSpec preflight: 通过。"
