#!/usr/bin/env bash
# Checks the local dev environment and known pitfalls. Usage: pnpm doctor
set -uo pipefail

ok() { printf '  \033[32m✔\033[0m %s\n' "$*"; }
warn() { printf '  \033[33m!\033[0m %s\n' "$*"; }
bad() { printf '  \033[31m✘\033[0m %s\n' "$*"; missing=1; }
missing=0

echo "Ferramentas"
for t in rustc cargo node pnpm git gh; do
  if command -v "$t" >/dev/null; then ok "$t $("$t" --version 2>/dev/null | head -1)"; else bad "$t não encontrado"; fi
done
node_major=$(node -p 'process.versions.node.split(".")[0]' 2>/dev/null || echo 0)
(( node_major >= 22 )) || bad "Node ≥ 22 necessário (atual: $node_major)"
command -v pnpm >/dev/null || warn "instale com: npm i -g pnpm --prefix ~/.local  (e ~/.local/bin no PATH)"

echo "Dependências do Tauri (Linux)"
if command -v pacman >/dev/null; then
  for p in webkit2gtk-4.1 base-devel curl wget file openssl libappindicator-gtk3 librsvg xdotool; do
    pacman -Q "$p" >/dev/null 2>&1 && ok "$p" || bad "$p (sudo pacman -S --needed $p)"
  done
elif command -v dpkg >/dev/null; then
  for p in libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev; do
    dpkg -s "$p" >/dev/null 2>&1 && ok "$p" || bad "$p (sudo apt install $p)"
  done
else
  warn "distro não reconhecida: veja https://v2.tauri.app/start/prerequisites/"
fi

echo "Ferramentas opcionais"
for b in google-chrome-stable chromium google-chrome; do
  command -v "$b" >/dev/null && { ok "navegador pro ui:check: $b"; found=1; break; }
done
[[ "${found:-0}" == 1 ]] || warn "sem Chrome/Chromium: pnpm ui:check não vai funcionar"

echo "Problemas conhecidos"
if [[ -d /usr/lib/vmware/lib ]]; then
  warn "VMware instalado: o AppImage local falha no linuxdeploy (gdk-pixbuf em /usr/lib/vmware/lib). Use o .deb localmente; o AppImage oficial sai do CI."
else
  ok "nada encontrado"
fi

exit $missing
