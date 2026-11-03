# Fonte das peças da marca homedeck

Tudo é gerado por código: páginas HTML renderizadas no Chromium (Playwright) e scripts Python.

    ./build.sh            # gera tudo em dist/
    OUT=/outro/lugar ./build.sh

| Arquivo | O que gera |
|---|---|
| `common.css` / `common.js` | Estilo vidro, paisagem (`drawScene`), símbolo (`markSVG`), ícones de linha |
| `mosaic.html` | Mosaico do Instagram (3240×1440, fatiado em 3) |
| `readme.html` | Banner do README, social preview e og-image (`?square=1`, `?h=672`) |
| `profile.html` | Foto de perfil (`?v=light` para a clara) |
| `icon.html` | Ícones (`?v=glass`, `glass-full`, `glass-round`, `flat`, `bg`, `fg`, `mono`, `banner`, `tizen`) |
| `logos.py` | SVGs do logo com texto em curvas |
| `kit.py` | Redimensiona e organiza tudo por plataforma |

Para mudar textos dos posts, edite o HTML e rode de novo. Para mudar a paisagem, ajuste `glows`, `moon` e `ridges` na chamada de `drawScene`.
Fonte Outfit (SIL Open Font License) incluída.
