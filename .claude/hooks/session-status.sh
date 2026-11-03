#!/usr/bin/env bash
# SessionStart hook: inject a short project status into Claude's context.
cd "${CLAUDE_PROJECT_DIR:-.}" || exit 0
command -v gh >/dev/null || exit 0
echo "## homedeck — status no início da sessão"
echo "Branch: $(git branch --show-current 2>/dev/null)  |  alterações locais: $(git status --porcelain 2>/dev/null | wc -l)"
timeout 8 gh api repos/lucacasanova/homedeck/milestones --jq \
  '.[] | select(.open_issues > 0) | "- \(.title): \(.closed_issues)/\(.open_issues + .closed_issues) fechadas"' 2>/dev/null | head -6
prs=$(timeout 5 gh pr list --json number,title --jq '.[] | "- PR #\(.number) \(.title)"' 2>/dev/null)
[ -n "$prs" ] && echo "PRs abertos:" && echo "$prs"
echo "Use a skill sync para detalhes e a skill feature antes de qualquer código."
exit 0
