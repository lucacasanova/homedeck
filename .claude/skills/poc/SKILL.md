---
name: poc
description: Conduz uma prova de conceito pra validar um risco listado em docs/planning/RISKS.md (ex. R1 descoberta Tizen, R2 TLS, R4 gamepad, R6 Tauri Android TV). Use quando o usuário pedir "POC", "testar se funciona na TV", "validar risco", ou antes de começar uma fase que depende de risco aberto.
---

# POC de risco

1. Identifique o risco (`Rn`) em `docs/planning/RISKS.md`. Se não existe, adicione a linha primeiro.
2. Branch `poc/rn-descricao`. Código em `poc/rn-descricao/` na raiz. Código de POC **não** vai pra `apps/`, `crates/` nem `packages/`.
3. Escreva o critério de sucesso antes de codar: o que observar, em qual dispositivo/versão.
4. Faça o mínimo pra responder a pergunta. Sem arquitetura, sem polimento.
5. Peça ao usuário pra rodar no dispositivo real quando precisar (TV, celular). Registre modelo, ano e versão do SO.
6. Registre o resultado em `docs/planning/RISKS.md`:
   - Coluna de status: `validado ✅`, `falhou ❌ → plano B` ou `parcial ⚠️`.
   - Uma seção `## Rn — resultado` com data, dispositivo, o que funcionou e o que não funcionou.
7. Se o resultado muda a arquitetura → skill `adr`.
8. Commit `test(poc): Rn resultado` e PR. O código da POC pode ser apagado depois do merge; o registro em RISKS.md fica.
