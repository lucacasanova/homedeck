#!/usr/bin/env bash
# PreToolUse hook (Bash): enforce homedeck git rules for Claude.
# - blocks commits directly on main (of the repo the command actually runs in)
# - blocks force-push
# - blocks commit messages outside Conventional Commits
# Only the first line of each command segment is inspected, so text inside
# heredocs or file contents does not trigger false positives.
set -euo pipefail

input=$(cat)
cmd=$(jq -r '.tool_input.command // ""' <<<"$input")
cwd=$(jq -r '.cwd // ""' <<<"$input")
[ -n "$cwd" ] || cwd="${CLAUDE_PROJECT_DIR:-.}"

# Command lines only: drop heredoc bodies (from "<<" marker line to its terminator).
lines=$(awk '
  inhd { if ($0 == term) inhd = 0; next }
  {
    print
    if (match($0, /<<-?[[:space:]]*["\x27]?[A-Za-z_]+["\x27]?/)) {
      t = substr($0, RSTART, RLENGTH); gsub(/<<-?[[:space:]]*|["\x27]/, "", t); term = t; inhd = 1
    }
  }' <<<"$cmd")

grep -qE '(^|[;&|[:space:]])git[[:space:]]' <<<"$lines" || exit 0

# Repo dir: explicit "git -C DIR", else last "cd DIR" before the git call, else hook cwd.
dir="$cwd"
if d=$(grep -oE 'git[[:space:]]+-C[[:space:]]+[^[:space:];&|]+' <<<"$lines" | head -1 | awk '{print $3}'); [ -n "$d" ]; then
  dir="$d"
elif d=$(grep -oE '(^|[;&|[:space:]])cd[[:space:]]+[^[:space:];&|]+' <<<"$lines" | tail -1 | awk '{print $NF}'); [ -n "$d" ]; then
  dir="$d"
fi
dir="${dir/#\~/$HOME}"
[[ "$dir" = /* ]] || dir="$cwd/$dir"

if grep -qE '(^|[;&|[:space:]])git[[:space:]]+push\b.*(--force([[:space:]]|$)|[[:space:]]-f([[:space:]]|$))' <<<"$lines"; then
  echo "Blocked: force-push is not allowed. Use --force-with-lease on your own branch if really needed." >&2
  exit 2
fi

if grep -qE '(^|[;&|[:space:]])git([[:space:]]+-C[[:space:]]+[^[:space:]]+)?[[:space:]]+commit\b' <<<"$lines"; then
  branch=$(git -C "$dir" branch --show-current 2>/dev/null || echo "")
  has_commits=$(git -C "$dir" rev-parse --verify -q HEAD >/dev/null && echo yes || echo no)
  # The very first commit of a brand-new repo is allowed on main.
  if [[ "$branch" == "main" && "$has_commits" == "yes" ]]; then
    echo "Blocked: never commit directly on main ($dir). Create a branch first (feat/..., fix/..., docs/..., chore/...)." >&2
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
