---
name: dev
description: Como rodar, verificar e depurar o homedeck localmente — dev server, app nativo, checagem completa igual ao CI, print e teste de navegação da UI em Chrome headless, build de pacotes Linux e problemas conhecidos do ambiente. Use ao começar a programar, para conferir visualmente uma mudança de UI, quando um build falhar, ou quando o usuário pedir "roda o app", "mostra como ficou", "testa a navegação", "o que tem de errado no ambiente".
---

# Desenvolvimento local

## Primeiro uso
```bash
pnpm env:check     # confere ferramentas e dependências; aponta problemas conhecidos
pnpm install
```

## Rodar
| Comando | O que faz |
|---|---|
| `pnpm dev` | App nativo (Tauri) com hot reload. Abre em tela cheia; F11 alterna |
| `pnpm dev:web` | Só a UI no navegador (http://localhost:1420). Comandos nativos caem no fallback web |

## Verificar antes de qualquer PR
```bash
pnpm check            # tudo que o CI roda: cargo fmt/clippy/test + lint/typecheck/test
pnpm check --rust     # só Rust
pnpm check --js       # só JS
pnpm fix              # formata e aplica correções automáticas do clippy
```
Cole a saída real no PR. Não abra PR com ❌ no resumo.

## Conferir a UI visualmente (obrigatório em mudança de UI)
```bash
pnpm ui:check                                   # build + servidor + print + teste de setas
pnpm ui:check --size 1280x720                   # outra resolução (ex. TV 720p)
pnpm ui:check --keys ArrowDown,Enter            # outra sequência de teclas
pnpm ui:check --url http://localhost:1420       # usar o dev server já rodando
```
- Saída em `.ui-check/`: `home.png`, `after-keys.png` e `report.json`. **Abra os PNGs com a ferramenta de leitura de imagem** e confira contra `docs/design/GUIA-DE-MARCA.md`: monocromático, vidro, foco branco e Outfit.
- A tabela mostra aba selecionada e foco depois de cada tecla. Todo elemento novo precisa ser alcançável só com setas + Enter.
- Não depende da extensão do Playwright, porque usa o Chrome headless da máquina.

## Pacotes
```bash
pnpm build:linux      # AppImage + .deb em target/release/bundle/
```

## Problemas conhecidos
| Sintoma | Causa | Solução |
|---|---|---|
| `failed to run linuxdeploy` / `libgdk_pixbuf` em `/usr/lib/vmware/lib` | VMware instalado polui o caminho das libs | Use o `.deb` localmente (`pnpm tauri build --bundles deb` em `apps/desktop-mobile`); o AppImage oficial sai do CI |
| `linuxdeploy` falha no strip em Arch/Manjaro | Bibliotecas com `.relr.dyn` | `NO_STRIP=true` (já incluso em `pnpm build:linux`) |
| `generate_context!` reclama de `dist` | Front não buildado | `pnpm check` cria um placeholder; ou rode `pnpm --filter @homedeck/app build` |
| `pnpm: command not found` | pnpm instalado em `~/.local` | `export PATH=$HOME/.local/bin:$PATH` |

## Inicialização resiliente e diagnóstico (ADR 0010)
No Linux o binário roda primeiro como supervisor. Se a interface quebrar antes de abrir (driver de vídeo), ele tenta outros modos de compatibilidade e salva o que funcionou.

| Comando / variável | Uso |
|---|---|
| `homedeck --diagnostics` | Imprime o relatório do ambiente (GPU, driver, sessão, EGL) |
| `homedeck --compat <modo>` | Força `default`, `no-dmabuf`, `mesa-egl`, `x11` ou `software` |
| `homedeck --no-guard` | Roda sem supervisor |
| `HOMEDECK_TEST_CRASH_MODES=default,no-dmabuf` | Simula crash nesses modos (só em build com `cargo build -p homedeck-app --features test-hooks`) |

Arquivos (Linux):
- log: `~/.local/share/io.github.lucacasanova.homedeck/logs/homedeck.log`
- relatórios: `~/.local/share/io.github.lucacasanova.homedeck/crash/`
- modo salvo: `~/.config/io.github.lucacasanova.homedeck/compat-mode`

Depois de testar com `HOMEDECK_TEST_CRASH_MODES`, apague o `compat-mode` e o `crash/` pra não deixar a máquina presa num modo de compatibilidade.

## Formatação automática
O hook `.claude/hooks/format.sh` formata (Prettier / rustfmt) cada arquivo que o Claude edita. Mesmo assim, rode `pnpm check` no final.
