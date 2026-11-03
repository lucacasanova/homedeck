---
name: feature
description: Fluxo obrigatório pra implementar qualquer funcionalidade, correção ou tarefa do homedeck — da issue ao PR. Use sempre que for escrever ou alterar código do projeto, quando o usuário pedir "implementar", "fazer", "corrigir", "começar a tarefa X" ou citar um item do TODO/roadmap.
---

# Fluxo de tarefa do homedeck

Siga as etapas na ordem. Não pule nenhuma.

## 1. Situar
- Leia `docs/planning/ROADMAP.md` e `docs/planning/TODO.md`. Ache a fase e o item da tarefa.
- Se a tarefa não está no TODO, adicione na fase certa antes de começar. Se não pertence a nenhuma fase, pare e pergunte ao usuário.
- Se o item depende de um risco aberto em `docs/planning/RISKS.md`, rode antes a skill `poc`.

## 2. Branch
- Nunca trabalhe na `main`. Crie `tipo/descricao-curta` (`feat/dedup-partial-hash`, `fix/...`, `docs/...`, `chore/...`).

## 3. Decidir
- A tarefa escolhe biblioteca, formato, protocolo ou estrutura nova? Use a skill `adr` antes de codar.

## 4. Implementar
- Teste primeiro quando der (core Rust: `#[test]`; UI: teste de componente).
- Regras do projeto (CLAUDE.md): UI só com setas + OK + voltar; nunca apagar arquivo do usuário (quarentena); endpoint novo → documentar em `ARCHITECTURE.md`.
- Mudanças pequenas e focadas. Nada fora do escopo da tarefa.

## 5. Verificar
- Rode build, lint e testes. Cole a saída real. Se falhar, conserte antes de seguir.
- Rode a skill `review` sobre o diff.

## 6. Fechar
- Commits em Conventional Commits, em inglês, com escopo (`feat(dedup): add partial hash stage`).
- Marque o item no `TODO.md`.
- Rode a skill `sync`.
- Abra o PR com o template e ligue a issue (`Closes #N`).
