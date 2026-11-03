# Mapa da marca — onde cada arquivo é usado

Regras de uso: [GUIA-DE-MARCA.md](GUIA-DE-MARCA.md). Todos os arquivos ficam em [`assets/brand/`](../../assets/brand/). Esta tabela diz **para onde cada um vai** no projeto. Quando o app correspondente for criado, copie o arquivo pro destino; não gere versões novas à mão.

## Logo e símbolo (`assets/brand/logo/`)

| Arquivo | Uso |
|---|---|
| `homedeck-logo-horizontal-white.svg` / `-black.svg` | README (via `<picture>`), site, telas de boas-vindas |
| `homedeck-logo-vertical-*.svg` | Splash screen, tela "Sobre" |
| `homedeck-simbolo-*.svg` | Navbar do app, marca d'água, loading |
| `homedeck-wordmark-*.svg` | Só o nome, quando o símbolo já aparece ao lado |
| `png/*` | Onde SVG não é aceito (slides, redes sociais) |

`white` = sobre fundo escuro · `black` = sobre fundo claro. O texto já está em curvas.

## Ícone do app (`assets/brand/app-icon/`)

| Arquivo | Uso |
|---|---|
| `homedeck-icon-1024.png` | **Master**: fonte para `pnpm tauri icon` regenerar tudo |
| `homedeck-icon-fullbleed-1024.png` | Lojas que aplicam máscara própria |
| `homedeck-icon-round-1024.png` | Launchers com ícone redondo |
| `homedeck-icon-simples-1024.png` / `.svg` | Tamanhos ≤ 48 px (sem vidro nem lua) |

## Por plataforma

| Origem | Destino no projeto | Issue |
|---|---|---|
| `tauri/src-tauri/icons/*` | `apps/desktop-mobile/src-tauri/icons/` | #4 |
| `linux/icons/hicolor/**` | pacote: `/usr/share/icons/hicolor/` (AppImage/AUR) | #25 |
| `linux/homedeck.desktop` | pacote: `/usr/share/applications/homedeck.desktop` | #25 |
| `android/app/src/main/res/**` | `apps/desktop-mobile/src-tauri/gen/android/app/src/main/res/` | #48 |
| `android/play-store-icon-512.png` | Google Play Console | #86 |
| `android-tv/app/src/main/res/drawable-xhdpi/banner.png` | idem + `android:banner="@drawable/banner"` no `<application>` | #50 |
| `android-tv/play-store-tv-banner-1280x720.png` | Google Play (Android TV) | #86 |
| `tizen/icon.png` (512×423) | `apps/tizen/icon.png` + `<icon src="icon.png"/>` no `config.xml` | #41 |
| `web/*` | raiz do site / `packages/ui/public/`; `head-snippet.html` no `<head>` | #109 |

## GitHub e redes (`assets/brand/github/`, `assets/brand/instagram/`)

| Arquivo | Uso |
|---|---|
| `github/readme-banner.png` | Topo do README |
| `github/social-preview.jpg` (1280×640) | Settings → Social preview do repositório |
| `github/avatar-1024.png` | Avatar de uma organização do projeto, se for criada |
| `instagram/instagram-perfil-*.png` | Foto de perfil (escura e clara) |
| `instagram/instagram-mosaico-{1,2,3}-*.png` | Mosaico: postar na ordem 3 → 2 → 1 |

## Fonte (`assets/brand/fonts/`)

Outfit 300/400/500/600 em `woff2`, sob licença SIL OFL 1.1 (`OFL.txt`). Na UI, use `@fontsource/outfit` (funciona offline); estes arquivos são a referência e o que os scripts do kit usam.

## Fonte das peças (`assets/brand/fonte/`)

Scripts (HTML + Playwright + Python) que geram todo o kit. Rode `./build.sh` pra regerar; a saída vai pra `dist/`, que é ignorada pelo git. Tudo deve ser regenerado por eles, nunca editado à mão.
