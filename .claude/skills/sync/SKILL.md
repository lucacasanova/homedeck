---
name: sync
description: Mostra e sincroniza o status do homedeck a partir do GitHub (milestones, épicos, sub-issues, PRs). Use ao começar e terminar cada sessão, ao fim de cada tarefa, ou quando o usuário perguntar "onde paramos", "o que falta", "status do projeto", "próximo passo".
---

# Sincronizar status

1. Colete:
   - `gh api repos/{owner}/{repo}/milestones --jq '.[] | "\(.number) \(.title) \(.closed_issues)/\(.open_issues+.closed_issues)"'`
   - `gh issue list --state open --milestone "<milestone atual>" --json number,title,labels,assignees`
   - `gh pr list --state open`
   - `git log --oneline -10` e `git status -s`
2. Ajuste o que estiver fora de sincronia:
   - PR mergeado sem `Closes #N` → feche a issue certa com `--reason completed`.
   - Issue em andamento sem comentário/branch → pergunte ao usuário.
   - Trabalho feito sem issue → ofereça criar e já fechar.
3. Milestone com todas as issues fechadas e critério "Pronto quando" (descrição do milestone) atendido → avise e proponha fechar: `gh api -X PATCH repos/{owner}/{repo}/milestones/N -f state=closed`. Depois de fechar, use a skill `announce`.
4. Responda com um resumo curto:
   - milestone atual e progresso (x/y)
   - últimas issues fechadas
   - próximas 3 issues sugeridas (priority:p0 primeiro, respeitando dependências e riscos `type:risk` abertos)
