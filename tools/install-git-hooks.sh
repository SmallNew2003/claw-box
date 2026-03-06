#!/usr/bin/env bash
set -euo pipefail

mkdir -p .git/hooks
cat > .git/hooks/pre-commit <<'HOOK'
#!/usr/bin/env bash
set -euo pipefail

# OpenSpec 预检
npm run openspec:preflight

# 规则检查
bash tools/check-rules.sh
HOOK
chmod +x .git/hooks/pre-commit

echo "Installed git pre-commit hook for OpenSpec preflight and rules check."
