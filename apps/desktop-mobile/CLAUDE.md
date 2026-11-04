# apps/desktop-mobile — casco Tauri 2 (Linux, Android, Android TV)

Leia antes o `CLAUDE.md` da raiz. Aqui vão só as armadilhas deste app.

## Estrutura
- `src/` é a UI (React 19 + Vite 8). Componentes compartilháveis vão pra `packages/ui`, não aqui.
- `src-tauri/src/lib.rs` tem os comandos Tauri e os plugins; o `main.rs` só chama o supervisor e o `run()`.
- `src-tauri/src/launcher.rs` é o supervisor de inicialização (ADR 0010). Ele roda **antes** do WebKit: não use nada de Tauri ou GPU lá. O sinal de "pronto" é um token no stdout do filho; nunca volte a usar arquivo em `/tmp`.
- `src-tauri/src/diagnostics.rs` monta o relatório do ambiente. Todo texto que vai pra relatório passa por `redact()`.

## Regras
- **Versões casadas:** cada plugin Tauri tem crate (Rust) e pacote npm, e o major.minor dos dois precisa ser igual, senão o `tauri build` falha ("version mismatched Tauri packages"). Confira o `Cargo.lock` antes de adicionar o pacote npm.
- **Permissões mínimas:** comando ou plugin novo só funciona se for liberado em `src-tauri/capabilities/default.json`. Adicione só o necessário.
- **CSP:** a política fica em `tauri.conf.json`. Origem nova (imagem, mídia, conexão) exige justificativa no PR e revisão de segurança.
- **Funcionar fora do Tauri:** a UI também roda no navegador (`pnpm dev:web`) e vai rodar na Tizen. Toda chamada nativa passa por `src/native.ts`, com fallback web.
- **Desktop em janela, mobile e TV em tela cheia:** não force `fullscreen` no `tauri.conf.json`; o `window-state` lembra a escolha do usuário.
- **Navegação:** todo elemento interativo usa `useFocusable` da Norigin. Confira com `pnpm ui:check`.
- **Marca:** logos e ícones vêm de `assets/brand/` (imports relativos ou cópia em `src-tauri/icons`). Não redesenhe.

## Testar
```bash
pnpm --filter @homedeck/app test        # vitest
cargo test -p homedeck-app               # Rust (supervisor, diagnóstico)
pnpm ui:check                            # print + navegação por setas
cargo build --release -p homedeck-app --features test-hooks && HOMEDECK_TEST_CRASH_MODES=default ./target/release/homedeck-app
```
Depois de testar a recuperação, apague `~/.config/io.github.lucacasanova.homedeck/compat-mode`.
