---
name: feature
description: Fluxo obrigatório pra implementar qualquer funcionalidade, correção ou tarefa do homedeck — da issue no GitHub ao PR. Use sempre que for escrever ou alterar código do projeto, quando o usuário pedir "implementar", "fazer", "corrigir", "começar a tarefa X", "pegar a issue #N" ou citar um item do roadmap.
---

# Fluxo de tarefa do homedeck

O planejamento vive no GitHub (milestones + épicos + sub-issues). Siga as etapas na ordem e não pule nenhuma.

## 1. Situar
- Ache a issue: `gh issue view N` ou `gh issue list --milestone "<M>" --label priority:p0`.
- Sem issue? Crie uma antes de começar (`gh issue create`), com labels `type:*`, `area:*`, `platform:*`, `priority:*`, o milestone certo e como sub-issue do épico (`gh api -X POST repos/{owner}/{repo}/issues/<épico>/sub_issues -F sub_issue_id=<id>`).
- A tarefa tem label `later` ou não cabe no milestone atual? Pare e pergunte ao usuário.
- Se depende de uma issue `type:risk` aberta, rode antes a skill `poc`.
- Comente na issue que começou: `gh issue comment N -b "Começando."`.

## 2. Branch
- Nunca trabalhe na `main`. Crie `tipo/N-descricao-curta` (ex. `feat/22-music-queue`).

## 3. Decidir
- A tarefa escolhe biblioteca, formato, protocolo ou estrutura nova? Use a skill `adr` antes de codar.

## 4. Implementar
- Teste primeiro quando der (core Rust: `#[test]`; UI: teste de componente).
- Regras do CLAUDE.md: UI só com setas + OK + voltar; nunca apagar arquivo do usuário; endpoint novo → `ARCHITECTURE.md`; UI segue `docs/design/DESIGN.md`.
- Mudanças pequenas e focadas, dentro do escopo da issue.

## 5. Verificar
- Rode build, lint e testes. Cole a saída real. Se falhar, conserte antes de seguir.
- Rode a skill `review` sobre o diff.

## 6. Fechar
- Commits em Conventional Commits, em inglês, com escopo.
- PR com o template, título em Conventional Commits e `Closes #N` no corpo.
- Merge por squash. A issue fecha sozinha.
- Rode a skill `sync`.
