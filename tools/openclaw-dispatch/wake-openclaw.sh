#!/usr/bin/env bash
set -euo pipefail

LATEST_FILE="${1:-}"
if [ -z "$LATEST_FILE" ]; then
  echo "latest file path is required" >&2
  exit 1
fi

GATEWAY_URL="${OPENCLAW_GATEWAY_URL:-http://127.0.0.1:18789}"
TOKEN="${OPENCLAW_GATEWAY_TOKEN:-${OPENCLAW_TOKEN:-}}"
WAKE_TEXT="${OPENCLAW_WAKE_TEXT:-Codex 任务完成，读取 latest.json}"

if [ -z "$TOKEN" ]; then
  echo "OPENCLAW_GATEWAY_TOKEN (or OPENCLAW_TOKEN) is required" >&2
  exit 1
fi

curl -sS -X POST "$GATEWAY_URL/api/cron/wake" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d "{\"text\":\"$WAKE_TEXT\",\"mode\":\"now\"}" \
  >/dev/null || true
