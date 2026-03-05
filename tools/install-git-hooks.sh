#!/usr/bin/env bash
set -euo pipefail

mkdir -p .git/hooks
cat > .git/hooks/pre-commit <<'HOOK'
#!/usr/bin/env bash
set -euo pipefail
npm run openspec:preflight
HOOK
chmod +x .git/hooks/pre-commit

echo "Installed git pre-commit hook for OpenSpec preflight."
