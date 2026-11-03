"""Exporta posts do Instagram, fotos de perfil, social preview e prévia da grade."""
import os
from PIL import Image, ImageDraw, ImageFont
IG = os.environ.get('IG', 'dist/homedeck'); os.makedirs(IG, exist_ok=True)
Image.open('social.png').convert('RGB').save(f'{IG}/github-social-preview.jpg', quality=92)
Image.open('profile_dark.png').convert('RGB').save(f'{IG}/instagram-perfil-escuro.png')
Image.open('profile_light.png').convert('RGB').save(f'{IG}/instagram-perfil-claro.png')
m = Image.open('mosaic_full.png').convert('RGB')
names = ['instagram-mosaico-1-esquerda.png', 'instagram-mosaico-2-centro.png', 'instagram-mosaico-3-direita.png']
for i, n in enumerate(names): m.crop((i * 1080, 0, (i + 1) * 1080, 1440)).save(f'{IG}/{n}')
W = 3 * 360 + 2 * 3; prof = Image.new('RGB', (W, 200 + 480 + 3 + 480), (0, 0, 0)); d = ImageDraw.Draw(prof)
av = Image.open('profile_dark.png').convert('RGB').resize((130, 130))
mk = Image.new('L', (130, 130), 0); ImageDraw.Draw(mk).ellipse((0, 0, 129, 129), fill=255); prof.paste(av, (30, 35), mk)
try: f = ImageFont.truetype('/usr/share/fonts/opentype/inter/Inter-SemiBold.otf', 30); f2 = ImageFont.truetype('/usr/share/fonts/opentype/inter/Inter-Regular.otf', 22)
except Exception: f = f2 = None
d.text((190, 60), 'homedeck', fill=(240, 240, 240), font=f)
d.text((190, 105), 'Central de mídia local, com cara de console.', fill=(170, 170, 170), font=f2)
for i in range(3):
    prof.paste(m.crop((i * 1080, 0, (i + 1) * 1080, 1440)).resize((360, 480), Image.LANCZOS), (i * 363, 200))
    d.rectangle((i * 363, 683, i * 363 + 359, 1162), fill=(28, 28, 30))
prof.save(f'{IG}/preview-grade-instagram.png')
