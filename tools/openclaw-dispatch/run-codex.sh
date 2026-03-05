#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
OUT_DIR="${OPENCLAW_DISPATCH_DIR:-$ROOT_DIR/.opencode/dispatch}"
OUT_FILE="$OUT_DIR/latest.json"

mkdir -p "$OUT_DIR"

TMP_OUT="$(mktemp)"
EXIT_CODE=0
CWD="$(pwd)"

"/Users/jelvin/.nvm/versions/node/v22.16.0/bin/codex" exec --full-auto "$@" 2>&1 | tee "$TMP_OUT" || EXIT_CODE=$?

python3 - <<'PY' "$TMP_OUT" "$OUT_FILE" "$EXIT_CODE" "$CWD"
import json
import sys
from datetime import datetime, timezone

out_path = sys.argv[1]
output_file = sys.argv[2]
exit_code = int(sys.argv[3])
cwd = sys.argv[4]

with open(out_path, "r", encoding="utf-8", errors="replace") as f:
    output = f.read()

payload = {
    "session_id": "codex",
    "timestamp": datetime.now(timezone.utc).isoformat(),
    "cwd": cwd,
    "event": "SessionEnd",
    "output": output,
    "status": "done" if exit_code == 0 else "failed",
}

with open(output_file, "w", encoding="utf-8") as f:
    json.dump(payload, f, ensure_ascii=False, indent=2)
    f.write("\n")
PY

"$ROOT_DIR/tools/openclaw-dispatch/wake-openclaw.sh" "$OUT_FILE"

exit "$EXIT_CODE"
