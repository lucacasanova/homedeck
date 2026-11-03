import os, io, struct, shutil, json
from PIL import Image

K = os.path.abspath(os.environ.get('KIT', 'dist/homedeck-brand-kit'))
IG = os.path.abspath(os.environ.get('IG', 'dist/homedeck'))
L = lambda n: Image.open(f'ic/{n}.png').convert('RGBA')
SRC = {n: L(n) for n in ['glass', 'glass-full', 'glass-round', 'flat', 'flat-full', 'bg', 'fg', 'mono']}

def icon(size, kind='glass'):
    """small sizes (<=48) use the simplified flat art for legibility"""
    if kind == 'glass' and size <= 48: kind = 'flat'
    return SRC[kind].resize((size, size), Image.LANCZOS)

def save(img, path):
    os.makedirs(os.path.dirname(path), exist_ok=True); img.save(path, optimize=True)

def write_ico(path, imgs):
    pngs = []
    for im in imgs:
        b = io.BytesIO(); im.save(b, 'PNG'); pngs.append((im.size[0], b.getvalue()))
    off = 6 + 16 * len(pngs); hdr = struct.pack('<HHH', 0, 1, len(pngs)); ent = b''; body = b''
    for s, d in pngs:
        ent += struct.pack('<BBBBHHII', s % 256, s % 256, 0, 0, 1, 32, len(d), off); off += len(d); body += d
    os.makedirs(os.path.dirname(path), exist_ok=True); open(path, 'wb').write(hdr + ent + body)

def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True); open(path, 'w').write(text)

# ---------------- app icon masters ----------------
A = f'{K}/app-icon'
save(SRC['glass'], f'{A}/homedeck-icon-1024.png')
save(SRC['glass-full'], f'{A}/homedeck-icon-fullbleed-1024.png')
save(SRC['glass-round'], f'{A}/homedeck-icon-round-1024.png')
save(SRC['flat'], f'{A}/homedeck-icon-simples-1024.png')

# flat icon as SVG (favicon / linux scalable)
mark = open(f'{K}/logo/homedeck-simbolo-white.svg').read()
inner = mark[mark.index('<g'):mark.rindex('</svg>')]
s = 1024 * .74 / 594
flat_svg = f'''<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024" width="1024" height="1024">
<defs><linearGradient id="g" x1="0" y1="0" x2=".55" y2="1"><stop offset="0" stop-color="#34343a"/><stop offset=".48" stop-color="#1a1a1e"/><stop offset="1" stop-color="#0b0b0d"/></linearGradient></defs>
<rect width="1024" height="1024" rx="225" fill="url(#g)"/>
<rect x="2" y="2" width="1020" height="1020" rx="223" fill="none" stroke="#fff" stroke-opacity=".12" stroke-width="4"/>
<g transform="translate({(1024 - 594 * s) / 2:.2f} {(1024 - 528 * s) / 2 + 10:.2f}) scale({s:.5f})">{inner.replace('stroke-width="58"', 'stroke-width="78"')}</g>
</svg>
'''
write(f'{A}/homedeck-icon-simples.svg', flat_svg)

# ---------------- Tauri (src-tauri/icons) ----------------
T = f'{K}/tauri/src-tauri/icons'
for n, sz in [('32x32.png', 32), ('64x64.png', 64), ('128x128.png', 128), ('128x128@2x.png', 256), ('icon.png', 512)]:
    save(icon(sz), f'{T}/{n}')
write_ico(f'{T}/icon.ico', [icon(s) for s in (16, 24, 32, 48, 64, 128, 256)])
SRC['glass'].save(f'{T}/icon.icns', sizes=[(16,16),(32,32),(64,64),(128,128),(256,256),(512,512),(1024,1024)]) if False else SRC['glass'].save(f'{T}/icon.icns')
for sz in (30, 44, 71, 89, 107, 142, 150, 284, 310):
    save(icon(sz, 'glass-full' if sz > 48 else 'flat-full'), f'{T}/Square{sz}x{sz}Logo.png')
save(icon(50, 'glass-full'), f'{T}/StoreLogo.png')

# ---------------- Linux (hicolor) ----------------
LX = f'{K}/linux'
for sz in (16, 22, 24, 32, 48, 64, 128, 256, 512):
    save(icon(sz), f'{LX}/icons/hicolor/{sz}x{sz}/apps/homedeck.png')
write(f'{LX}/icons/hicolor/scalable/apps/homedeck.svg', flat_svg)
write(f'{LX}/homedeck.desktop', '''[Desktop Entry]
Type=Application
Name=homedeck
GenericName=Central de mídia
Comment=Sua central de mídia local, com cara de console
Exec=homedeck %U
Icon=homedeck
Terminal=false
Categories=AudioVideo;Player;Video;Audio;
Keywords=media;player;video;music;tv;p2p;
StartupWMClass=homedeck
''')

# ---------------- Android ----------------
AN = f'{K}/android/app/src/main/res'
dens = {'mdpi': 1, 'hdpi': 1.5, 'xhdpi': 2, 'xxhdpi': 3, 'xxxhdpi': 4}
for d, f in dens.items():
    l = round(48 * f); a = round(108 * f)
    save(icon(l), f'{AN}/mipmap-{d}/ic_launcher.png')
    save(icon(l, 'glass-round') if l > 48 else icon(l, 'glass-round'), f'{AN}/mipmap-{d}/ic_launcher_round.png')
    save(SRC['fg'].resize((a, a), Image.LANCZOS), f'{AN}/mipmap-{d}/ic_launcher_foreground.png')
    save(SRC['bg'].resize((a, a), Image.LANCZOS), f'{AN}/mipmap-{d}/ic_launcher_background.png')
    save(SRC['mono'].resize((a, a), Image.LANCZOS), f'{AN}/mipmap-{d}/ic_launcher_monochrome.png')
adaptive = '''<?xml version="1.0" encoding="utf-8"?>
<adaptive-icon xmlns:android="http://schemas.android.com/apk/res/android">
    <background android:drawable="@mipmap/ic_launcher_background"/>
    <foreground android:drawable="@mipmap/ic_launcher_foreground"/>
    <monochrome android:drawable="@mipmap/ic_launcher_monochrome"/>
</adaptive-icon>
'''
write(f'{AN}/mipmap-anydpi-v26/ic_launcher.xml', adaptive)
write(f'{AN}/mipmap-anydpi-v26/ic_launcher_round.xml', adaptive)
save(icon(512, 'glass-full').convert('RGB'), f'{K}/android/play-store-icon-512.png')

# ---------------- Android TV ----------------
B = Image.open('ic/banner.png').convert('RGB')
save(B.resize((320, 180), Image.LANCZOS), f'{K}/android-tv/app/src/main/res/drawable-xhdpi/banner.png')
save(B, f'{K}/android-tv/play-store-tv-banner-1280x720.png')

# ---------------- Tizen ----------------
save(Image.open('ic/tizen.png').convert('RGBA').resize((512, 423), Image.LANCZOS), f'{K}/tizen/icon.png')

# ---------------- Web ----------------
WB = f'{K}/web'
write_ico(f'{WB}/favicon.ico', [icon(s) for s in (16, 32, 48)])
write(f'{WB}/favicon.svg', flat_svg)
save(icon(96), f'{WB}/favicon-96x96.png')
save(icon(180, 'glass-full').convert('RGB'), f'{WB}/apple-touch-icon.png')
save(icon(192), f'{WB}/web-app-manifest-192x192.png')
save(icon(512), f'{WB}/web-app-manifest-512x512.png')
save(icon(512, 'glass-full').convert('RGB'), f'{WB}/web-app-manifest-maskable-512x512.png')
Image.open('og.png').convert('RGB').resize((1200, 630), Image.LANCZOS).save(f'{WB}/og-image.jpg', quality=90)
write(f'{WB}/site.webmanifest', json.dumps({
    "name": "homedeck", "short_name": "homedeck",
    "description": "Sua central de mídia local, com cara de console.",
    "lang": "pt-BR", "start_url": "/", "display": "standalone",
    "background_color": "#0b0b0d", "theme_color": "#0b0b0d",
    "icons": [
        {"src": "/web-app-manifest-192x192.png", "sizes": "192x192", "type": "image/png", "purpose": "any"},
        {"src": "/web-app-manifest-512x512.png", "sizes": "512x512", "type": "image/png", "purpose": "any"},
        {"src": "/web-app-manifest-maskable-512x512.png", "sizes": "512x512", "type": "image/png", "purpose": "maskable"}]
}, ensure_ascii=False, indent=2) + '\n')
write(f'{WB}/head-snippet.html', '''<!-- homedeck: ícones e compartilhamento -->
<link rel="icon" href="/favicon.ico" sizes="48x48">
<link rel="icon" href="/favicon.svg" type="image/svg+xml">
<link rel="icon" href="/favicon-96x96.png" type="image/png" sizes="96x96">
<link rel="apple-touch-icon" href="/apple-touch-icon.png">
<link rel="manifest" href="/site.webmanifest">
<meta name="theme-color" content="#0b0b0d">

<meta property="og:title" content="homedeck">
<meta property="og:description" content="Sua central de mídia local, com cara de console. PC, celular e TV na rede de casa, sem servidor.">
<meta property="og:image" content="https://SEU-DOMINIO/og-image.jpg">
<meta property="og:image:width" content="1200">
<meta property="og:image:height" content="630">
<meta name="twitter:card" content="summary_large_image">
''')

# ---------------- GitHub ----------------
G = f'{K}/github'
save(icon(1024, 'glass-full').convert('RGB'), f'{G}/avatar-1024.png')
shutil.copy(f'{IG}/readme-banner.png', f'{G}/readme-banner.png')
shutil.copy(f'{IG}/github-social-preview.jpg', f'{G}/social-preview.jpg')
for n in ['homedeck-logo-horizontal-white.svg', 'homedeck-logo-horizontal-black.svg']:
    shutil.copy(f'{K}/logo/{n}', f'{G}/{n}')
write(f'{G}/README-snippet.md', '''<!-- copie os arquivos para docs/assets/ no repositório -->

<!-- Opção 1: banner completo -->
<p align="center">
  <img src="docs/assets/readme-banner.png" alt="homedeck — sua central de mídia local, com cara de console" width="100%">
</p>

<!-- Opção 2: só o logo, troca sozinho entre tema claro e escuro do GitHub -->
<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="docs/assets/homedeck-logo-horizontal-white.svg">
    <img src="docs/assets/homedeck-logo-horizontal-black.svg" alt="homedeck" height="56">
  </picture>
</p>
''')

# ---------------- Instagram (copy) ----------------
for f in os.listdir(IG):
    if f.startswith('instagram') or f.startswith('preview'):
        shutil.copy(f'{IG}/{f}', f'{K}/instagram/{f}' if os.path.isdir(f'{K}/instagram') else (os.makedirs(f'{K}/instagram'), f'{K}/instagram/{f}')[1])
shutil.copy('LEIAME-kit.md', f'{K}/LEIAME.md')
print('ok')
