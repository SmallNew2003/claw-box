#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
OUT_DIR="${OPENCLAW_DISPATCH_DIR:-$ROOT_DIR/.opencode/dispatch}"
OUT_FILE="$OUT_DIR/latest.json"

mkdir -p "$OUT_DIR"

TMP_OUT="$(mktemp)"
EXIT_CODE=0
CWD="$(pwd)"
STREAM_FILE="$OUT_DIR/stream.jsonl"
QUESTION_FILE="$OUT_DIR/question.json"
WAKE_REGEX="${OPENCLAW_WAKE_ON_REGEX:-问题|选择|请确认|确认|阻塞|blocked}"

if [ "$#" -ne 1 ]; then
  echo "run-codex.sh expects a single prompt string" >&2
  exit 2
fi

PROMPT_RAW="$1"
PROMPT_ESCAPED="$(python3 - <<'PY' "$PROMPT_RAW"
import sys
print(sys.argv[1].replace('\\', '\\\\').replace('"', '\\"'))
PY
)"

touch "$TMP_OUT"

python3 - <<'PY' "$TMP_OUT" "$STREAM_FILE" "$QUESTION_FILE" "$WAKE_REGEX" "$ROOT_DIR/tools/openclaw-dispatch/wake-openclaw.sh" &
import json
import re
import subprocess
import sys
import time
from datetime import datetime, timezone

out_path = sys.argv[1]
stream_file = sys.argv[2]
question_file = sys.argv[3]
regex = sys.argv[4]
wake_script = sys.argv[5]

pattern = re.compile(regex)
last_wake = 0.0
cooldown = 3.0

with open(out_path, "r", encoding="utf-8", errors="replace") as source, open(stream_file, "a", encoding="utf-8") as stream:
    source.seek(0, 2)
    while True:
        line = source.readline()
        if not line:
            time.sleep(0.2)
            continue
        line = line.rstrip("\n")
        payload = {
            "timestamp": datetime.now(timezone.utc).isoformat(),
            "event": "stdout",
            "line": line
        }
        stream.write(json.dumps(payload, ensure_ascii=False) + "\n")
        stream.flush()
        now = time.time()
        if pattern.search(line) and (now - last_wake) > cooldown:
            question = {
                "timestamp": datetime.now(timezone.utc).isoformat(),
                "event": "question",
                "line": line
            }
            with open(question_file, "w", encoding="utf-8") as qf:
                json.dump(question, qf, ensure_ascii=False, indent=2)
                qf.write("\n")
            subprocess.run([wake_script, question_file], check=False)
            last_wake = now
PY
TAIL_PID=$!

"/Users/jelvin/.nvm/versions/node/v22.16.0/bin/codex" exec --full-auto "$PROMPT_RAW" >"$TMP_OUT" 2>&1 || EXIT_CODE=$?

kill "$TAIL_PID" >/dev/null 2>&1 || true

EXIT_CODE=${PIPESTATUS[0]}

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
