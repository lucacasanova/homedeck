#!/usr/bin/env bash
# PostToolUse hook (Edit|Write): format the file Claude just changed.
# Never blocks: formatting problems are reported by `pnpm check`.
input=$(cat)
file=$(jq -r '.tool_input.file_path // empty' <<<"$input")
[[ -n "$file" && -f "$file" ]] || exit 0
root="${CLAUDE_PROJECT_DIR:-$(pwd)}"
case "$file" in "$root"/*) ;; *) exit 0 ;; esac          # only files inside the repo
case "$file" in */node_modules/*|*/target/*|*/dist/*|*/assets/brand/*|*/src-tauri/gen/*) exit 0 ;; esac

case "$file" in
  *.rs)
    command -v rustfmt >/dev/null && rustfmt --edition 2024 --quiet "$file" 2>/dev/null ;;
  *.ts|*.tsx|*.js|*.mjs|*.css|*.json|*.html|*.yml|*.yaml)
    [[ -x "$root/node_modules/.bin/prettier" ]] && \
      "$root/node_modules/.bin/prettier" --log-level silent --ignore-unknown --write "$file" 2>/dev/null ;;
esac
exit 0
