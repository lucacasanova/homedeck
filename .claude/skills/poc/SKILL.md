---
name: poc
description: Conduz uma prova de conceito pra validar um risco técnico do homedeck (issues com labels type:risk / type:poc no GitHub, ex. R1 descoberta Tizen #38, R2 TLS #39, R5 codecs #40, R6 Tauri Android TV #47). Use quando o usuário pedir "POC", "testar se funciona na TV", "validar risco", ou antes de começar trabalho que depende de risco aberto.
---

# POC de risco

1. Ache a issue: `gh issue list --label type:risk`. Se o risco não tem issue, crie uma com `type:risk,type:poc`, a plataforma e `priority:p0`.
2. Branch `poc/N-descricao`. O código vai em `poc/N-descricao/` na raiz, **nunca** em `apps/`, `crates/` ou `packages/`.
3. Antes de codar, comente na issue o critério de sucesso: o que observar e em qual dispositivo/versão.
4. Faça o mínimo pra responder a pergunta. Sem arquitetura, sem polimento.
5. Quando precisar de dispositivo real (TV, celular), peça ao usuário pra rodar e registre modelo, ano e versão do SO.
6. Registre o resultado como comentário na issue:
   - `✅ validado`, `❌ falhou → plano B` ou `⚠️ parcial`
   - data, dispositivo, o que funcionou, o que não funcionou, logs relevantes
7. Se o resultado muda a arquitetura → skill `adr`, e link do ADR na issue.
8. Feche a issue com `gh issue close N --reason completed`. O código da POC pode ficar fora do merge; o registro na issue é o que importa.
