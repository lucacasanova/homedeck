# TODO

Lista viva. Quando o repo estiver no GitHub, cada item vira uma issue no milestone da fase.

## Fase 0 — Fundação
- [x] Estrutura de pastas e docs de planejamento
- [x] Licença GPL-3.0
- [x] CLAUDE.md, CONTRIBUTING.md, templates de issue/PR
- [x] ADRs iniciais (stack, licença, game streaming)
- [x] Skills do Claude (feature, adr, poc, review, sync) + hook de git
- [ ] Criar repo no GitHub e subir
- [ ] Proteção da branch `main` (PR obrigatório, CI verde)
- [ ] Milestones das fases 0–7 e labels (`area:*`, `type:*`, `platform:*`)
- [ ] Workspace: pnpm (JS) + Cargo (Rust)
- [ ] `crates/core` vazio com teste
- [ ] `packages/ui` vazio (decidir React ou Svelte → ADR 0004)
- [ ] `apps/desktop-mobile` Tauri 2 hello-world
- [ ] CI: lint + teste + build em Linux, Windows e macOS
- [ ] Conventional Commits verificado no CI
- [ ] Release automatizado (release-please ou similar)

## Fase 1 — Desktop solo
- [ ] Indexador: varrer pastas, observar mudanças
- [ ] Metadados via `ffprobe`
- [ ] Miniaturas
- [ ] Banco local (SQLite)
- [ ] Player
- [ ] Spatial navigation (setas/gamepad)
- [ ] Dedup: tamanho → hash parcial → BLAKE3
- [ ] Quarentena com desfazer
- [ ] Painel de espaço

## POCs pra fazer cedo (ver RISKS.md)
- [ ] R1 — descoberta na Tizen
- [ ] R2 — TLS autoassinado na Tizen/iOS
- [ ] R4 — Gamepad API na Tizen
- [ ] R6 — Tauri 2 Android TV com D-pad
