# Kit de marca homedeck

Monocromático (grafite + branco), estilo vidro. Fonte: **Outfit** (500 no logo, 300/400 nos textos).

| Pasta | Onde usar |
|---|---|
| `logo/` | Símbolo, wordmark e logos horizontal/vertical em SVG (texto já em curvas) e PNG. `white` = fundo escuro, `black` = fundo claro. |
| `app-icon/` | Masters 1024 px. Use `homedeck-icon-1024.png` como fonte para regenerar tudo (`npx tauri icon`). Abaixo de 48 px use a versão `simples`. |
| `tauri/src-tauri/icons/` | Copie por cima de `src-tauri/icons/` do projeto (ico, icns, png, Square*Logo). |
| `linux/` | `icons/hicolor/...` → `/usr/share/icons/hicolor/`; `homedeck.desktop` → `/usr/share/applications/`. |
| `android/app/src/main/res/` | Ícone adaptável (fundo, frente e monocromático do Android 13) + legado. `play-store-icon-512.png` para a loja. |
| `android-tv/` | `drawable-xhdpi/banner.png` (320×180) → `android:banner="@drawable/banner"` no `<application>`. Banner 1280×720 para a loja. |
| `tizen/icon.png` | 512×423 → raiz do app web, `<icon src="icon.png"/>` no `config.xml`. |
| `web/` | Raiz do site. Cole `head-snippet.html` no `<head>` e troque `SEU-DOMINIO`. |
| `github/` | Avatar, banner do README, social preview (Settings → Social preview) e `README-snippet.md`. |
| `instagram/` | Fotos de perfil e mosaico (postar 3 → 2 → 1). |

Paleta: `#0B0B0D` fundo · `#1A1A1E` superfície · `#F4F4F5` texto · branco 100% para destaque.
