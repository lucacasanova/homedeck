#!/usr/bin/env bash
# Runs everything CI runs, locally. Usage: pnpm check [--rust|--js]
set -uo pipefail
cd "$(dirname "$0")/.."

only="${1:-}"
results=()
fail=0

step() {
  local name="$1"; shift
  printf '\n\033[1m▶ %s\033[0m\n' "$name"
  if "$@"; then results+=("✅ $name"); else results+=("❌ $name"); fail=1; fi
}

if [[ "$only" != "--js" ]]; then
  # generate_context! needs a frontend dist folder to exist.
  [[ -d apps/desktop-mobile/dist ]] || { mkdir -p apps/desktop-mobile/dist && touch apps/desktop-mobile/dist/index.html; }
  step "cargo fmt" cargo fmt --all -- --check
  step "cargo clippy" cargo clippy --workspace --all-targets -q -- -D warnings
  step "cargo test" cargo test --workspace -q
  if cargo deny --version >/dev/null 2>&1; then
    step "cargo deny" cargo deny check -q advisories bans licenses sources
  else
    results+=("⏭  cargo deny (instale: cargo install cargo-deny --locked)")
  fi
fi

if [[ "$only" != "--rust" ]]; then
  step "pnpm lint" pnpm -s lint
  step "pnpm typecheck" pnpm -s typecheck
  step "pnpm test" pnpm -s test
fi

printf '\n\033[1mResumo\033[0m\n'
printf '  %s\n' "${results[@]}"
exit $fail
