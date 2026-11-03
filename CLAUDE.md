# CLAUDE.md

Guia pro Claude (e pra humanos) trabalharem neste repo.

## Projeto
homedeck: central de mídia local P2P, estilo console, multiplataforma. Ler antes de mexer:
`docs/planning/VISION.md`, `docs/planning/ARCHITECTURE.md`, `docs/design/DESIGN.md`.
Planejamento (roadmap, tarefas, riscos) vive no GitHub: milestones M0–M5 + "Later", um épico por milestone, tarefas como sub-issues.

## Prioridade atual
MVP = **Linux (Manjaro), Android/Android TV e Tizen**, só **reprodução de mídia** (vídeo, música, imagem) + rede local + cast. Gestão de arquivos (dedup/espaço), jogos e outras plataformas têm label `later`: não implementar sem o usuário pedir.

## Regras
- Idioma dos docs: PT-BR. Código, identificadores e mensagens de commit: inglês.
- Commits: Conventional Commits (`feat:`, `fix:`, `docs:`, `chore:`, `refactor:`, `test:`, `ci:`). Escopo = área (`feat(dedup): ...`).
- Nunca commitar direto na `main`. Branch `tipo/descricao-curta` + PR.
- Decisão de arquitetura nova → criar ADR em `docs/adr/` a partir do `0000-template.md`.
- Todo trabalho tem issue. PR fecha a issue com `Closes #N`; merge por squash.
- Nunca apagar arquivo do usuário em código: usar quarentena.
- Toda UI precisa funcionar só com setas + OK + voltar.
- UI segue `docs/design/DESIGN.md` (vidro translúcido, papel de parede por dispositivo, modo leve na TV).
- Endpoints novos → atualizar a seção de protocolo em `ARCHITECTURE.md`.

## Skills do projeto (`.claude/skills/`)
Usar sempre:
- `feature`: fluxo de toda tarefa (issue → branch → ADR? → teste → review → PR → sync)
- `adr`: registrar decisão de arquitetura
- `poc`: validar issue `type:risk` em dispositivo real
- `review`: checklist do projeto antes de PR
- `sync`: status a partir de milestones/issues/PRs; usar ao começar e terminar sessão

Hook `.claude/hooks/guard-git.sh` bloqueia commit na `main`, force-push e mensagem fora de Conventional Commits.

## Estrutura
- `crates/core`: lógica do nó (Rust)
- `packages/ui`: UI compartilhada
- `packages/protocol`: tipos entre nós
- `apps/desktop-mobile`: Tauri 2
- `apps/tizen`: web app Samsung TV

## Comandos
(preencher quando o workspace existir: build, teste, lint)
