#!/usr/bin/env bash
# PreToolUse hook (Bash): enforce homedeck git rules for Claude.
# - blocks commits directly on main
# - blocks force-push
# - blocks commit messages outside Conventional Commits
set -euo pipefail

cmd=$(jq -r '.tool_input.command // ""')

case "$cmd" in
  *"git "*) ;;
  *) exit 0 ;;
esac

if [[ "$cmd" =~ git[[:space:]]+push.*(--force|-f([[:space:]]|$)) ]] && [[ ! "$cmd" =~ --force-with-lease ]]; then
  echo "Blocked: force-push is not allowed. Use --force-with-lease on your own branch if really needed." >&2
  exit 2
fi

if [[ "$cmd" =~ git[[:space:]]+commit ]]; then
  branch=$(git -C "${CLAUDE_PROJECT_DIR:-.}" branch --show-current 2>/dev/null || echo "")
  if [[ "$branch" == "main" ]]; then
    echo "Blocked: never commit directly on main. Create a branch first (feat/..., fix/..., docs/..., chore/...)." >&2
    exit 2
  fi
  if [[ "$cmd" =~ -m[[:space:]]+[\"\']([^\"\']*) ]]; then
    subject="${BASH_REMATCH[1]%%$'\n'*}"
    if ! [[ "$subject" =~ ^(feat|fix|docs|chore|refactor|test|ci|perf|build|style|revert)(\([a-z0-9-]+\))?!?:\ .+ ]]; then
      echo "Blocked: commit subject must follow Conventional Commits, e.g. 'feat(dedup): add partial hash'. Got: '$subject'" >&2
      exit 2
    fi
  fi
fi

exit 0
