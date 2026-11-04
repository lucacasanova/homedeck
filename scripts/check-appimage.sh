#!/usr/bin/env bash
# Fails if an AppImage bundles libwayland (issue #158, ADR 0010).
#
# linuxdeploy would copy the build machine's libwayland-*; on users' systems
# with the NVIDIA egl-wayland driver that copy makes WebKit crash (SIGSEGV) or
# render a blank window. Builds set LINUXDEPLOY_EXCLUDED_LIBRARIES="libwayland-*";
# this check makes sure it took effect.
#
# Usage: scripts/check-appimage.sh path/to/homedeck_X.Y.Z_amd64.AppImage
set -euo pipefail
APPIMAGE=$(realpath "$1")
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
cd "$work"
"$APPIMAGE" --appimage-extract >/dev/null
found=$(find squashfs-root -name 'libwayland-*.so*' | sed 's#^squashfs-root/##')
if [[ -n "$found" ]]; then
  echo "::error::AppImage traz libwayland embutida (quebra com NVIDIA no Wayland, #158):"
  echo "$found"
  exit 1
fi
echo "check-appimage: ok, nenhuma libwayland embutida"
