#!/usr/bin/env bash
# Removes the bundled libwayland-* from an AppImage and repacks it (issue #158).
#
# linuxdeploy bundles the build machine's libwayland. On systems with the
# NVIDIA proprietary driver (egl-wayland), that copy is ABI-incompatible with
# the host driver and WebKit crashes (SIGSEGV) or renders nothing
# (EGL_BAD_PARAMETER). libwayland is part of every Wayland desktop and ABI
# stable, so the host copy is always the right one.
#
# Usage: scripts/fix-appimage.sh path/to/homedeck_X.Y.Z_amd64.AppImage
set -euo pipefail

APPIMAGE=$(realpath "$1")
TOOL_VERSION=1.9.1
TOOL_SHA256=ed4ce84f0d9caff66f50bcca6ff6f35aae54ce8135408b3fa33abfc3cb384eb0

work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

tool="${APPIMAGETOOL:-}"
if [[ -z "$tool" ]]; then
  tool="$work/appimagetool"
  curl -sSfL -o "$tool" \
    "https://github.com/AppImage/appimagetool/releases/download/${TOOL_VERSION}/appimagetool-x86_64.AppImage"
  echo "${TOOL_SHA256}  $tool" | sha256sum -c --quiet -
  chmod +x "$tool"
fi

cd "$work"
"$APPIMAGE" --appimage-extract >/dev/null
removed=$(find squashfs-root -name 'libwayland-*.so*' -print -delete | wc -l)
echo "fix-appimage: removidas $removed bibliotecas libwayland embutidas"

# --appimage-extract-and-run: works without FUSE (CI containers).
ARCH=x86_64 "$tool" --appimage-extract-and-run --no-appstream squashfs-root "$work/out.AppImage" >/dev/null 2>&1
chmod +x "$work/out.AppImage"
mv "$work/out.AppImage" "$APPIMAGE"
echo "fix-appimage: $APPIMAGE reempacotado"
