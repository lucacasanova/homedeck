# CLAUDE.md

Guia pro Claude (e pra humanos) trabalharem neste repo.

## Projeto
homedeck: central de mídia local P2P, estilo console, multiplataforma. Ler antes de mexer:
`docs/planning/VISION.md`, `docs/planning/ARCHITECTURE.md`, `docs/planning/ROADMAP.md`.

## Regras
- Idioma dos docs: PT-BR. Código, identificadores e mensagens de commit: inglês.
- Commits: Conventional Commits (`feat:`, `fix:`, `docs:`, `chore:`, `refactor:`, `test:`, `ci:`). Escopo = área (`feat(dedup): ...`).
- Nunca commitar direto na `main`. Branch `tipo/descricao-curta` + PR.
- Decisão de arquitetura nova → criar ADR em `docs/adr/` a partir do `0000-template.md`.
- Concluiu um item → marcar em `docs/planning/TODO.md` (ou fechar a issue).
- Nunca apagar arquivo do usuário em código: usar quarentena.
- Toda UI precisa funcionar só com setas + OK + voltar.
- Endpoints novos → atualizar a seção de protocolo em `ARCHITECTURE.md`.

## Skills do projeto (`.claude/skills/`)
Usar sempre:
- `feature`: fluxo de toda tarefa (TODO → branch → ADR? → teste → review → sync → PR)
- `adr`: registrar decisão de arquitetura
- `poc`: validar risco de RISKS.md em dispositivo real
- `review`: checklist do projeto antes de PR
- `sync`: atualizar TODO/ROADMAP/issues; usar ao começar e terminar sessão

Hook `.claude/hooks/guard-git.sh` bloqueia commit na `main`, force-push e mensagem fora de Conventional Commits.

## Estrutura
- `crates/core`: lógica do nó (Rust)
- `packages/ui`: UI compartilhada
- `packages/protocol`: tipos entre nós
- `apps/desktop-mobile`: Tauri 2
- `apps/tizen`: web app Samsung TV

## Comandos
(preencher quando o workspace existir: build, teste, lint)
