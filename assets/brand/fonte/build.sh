#!/usr/bin/env bash
# Regenera todas as peças da marca homedeck.
# Requisitos: node + playwright (npm i -D playwright && npx playwright install chromium)
#             python3 + pillow uharfbuzz fonttools brotli
set -euo pipefail
cd "$(dirname "$0")"
OUT=${OUT:-dist}
export KIT="$OUT/homedeck-brand-kit" IG="$OUT/homedeck"
mkdir -p ic "$IG" "$KIT/logo/png"
R="node render.js"

echo "› cenas grandes"
$R mosaic.html 3240 1440 1 mosaic_full.png
$R readme.html 1280 640 2 "$IG/readme-banner.png" 1
$R "readme.html?square=1" 1280 640 1 social.png
$R "readme.html?square=1&h=672" 1280 672 1 og.png
$R profile.html 1080 1080 1 profile_dark.png
$R "profile.html?v=light" 1080 1080 1 profile_light.png

echo "› ícones"
for v in glass glass-full glass-round flat flat-full bg fg mono; do $R "icon.html?v=$v" 1024 1024 1 "ic/$v.png" 1; done
$R "icon.html?v=banner&w=1280&h=720" 1280 720 1 ic/banner.png 1
$R "icon.html?v=tizen&w=512&h=423" 512 423 2 ic/tizen.png 1

echo "› instagram + github"
python3 slice.py

echo "› logos vetoriais"
python3 logos.py "$KIT/logo"
node svg2png.js "$KIT/logo"

echo "› kit por plataforma"
python3 kit.py
echo "pronto: $OUT/"
