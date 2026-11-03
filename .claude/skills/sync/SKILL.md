---
name: sync
description: Sincroniza o planejamento do homedeck com o estado real do código — TODO, ROADMAP, RISKS e issues do GitHub. Use ao fim de cada tarefa ou sessão, quando o usuário perguntar "onde paramos", "o que falta", "status do projeto", ou ao começar uma sessão nova.
---

# Sincronizar planejamento

1. Leia `docs/planning/TODO.md`, `ROADMAP.md`, `RISKS.md` e `git log --oneline -20`.
2. Marque como feito no TODO o que os commits/PRs entregaram. Desmarque o que foi revertido.
3. Se o repo tem remoto no GitHub: confira `gh issue list` e `gh api repos/{owner}/{repo}/milestones`. Item do TODO sem issue → ofereça criar. Issue fechada → marque no TODO.
4. Fase com todos os itens feitos e critério "Pronto quando" atendido → avise o usuário e proponha fechar o milestone.
5. Responda com um resumo curto:
   - Fase atual e % de itens feitos.
   - Últimos itens concluídos.
   - Próximos 3 itens sugeridos (respeitando dependências e riscos abertos).
6. Se alterou docs: commit `docs(planning): sync status`.
