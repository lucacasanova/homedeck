---
name: review
description: Checklist de revisão específico do homedeck aplicado ao diff atual ou a um PR. Use antes de abrir PR, quando o usuário pedir "revisar", "review", "tá ok pra subir?", ou ao fim da skill feature.
---

# Revisão do homedeck

Rode `git diff main...HEAD` (ou o diff do PR) e confira cada item. Reporte só o que falhar, no formato `arquivo:linha — problema — correção`.

## Segurança
- [ ] Endpoint novo exige pareamento (só `/api/v1/info` é aberto).
- [ ] Sem segredo, chave ou caminho pessoal no código ou nos logs.
- [ ] Caminho vindo da rede é validado (sem `..`, sem sair das pastas da biblioteca).

## Dados do usuário
- [ ] Evento de telemetria novo ou alterado está em `docs/privacy/TELEMETRY.md`, só com propriedades enum/faixa, e respeita o consentimento.
- [ ] Nenhum dado de telemetria, segredo ou análise do repo privado foi copiado pra cá.
- [ ] Nada apaga arquivo direto. Remoção = mover pra quarentena com desfazer.
- [ ] Operação longa (hash, scan, transcode) é cancelável e não trava a UI.

## UI
- [ ] Tudo alcançável com setas + OK + voltar. Foco sempre visível.
- [ ] Texto legível a 3 m (TV): tamanho mínimo do tema respeitado.

## Desempenho
- [ ] Sem ler arquivo inteiro em memória. Usar streaming e chunks.
- [ ] Dedup segue a ordem tamanho → hash parcial → hash completo.

## Projeto
- [ ] Conventional Commits.
- [ ] Testes cobrindo o comportamento novo.
- [ ] `ARCHITECTURE.md` atualizado se mudou protocolo ou módulo.
- [ ] ADR criado se houve decisão nova.
- [ ] PR com `Closes #N` e título em Conventional Commits.
- [ ] UI segue `docs/design/DESIGN.md` (vidro, foco, modo leve na TV).
