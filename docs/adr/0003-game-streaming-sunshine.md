# ADR 0003 — Game streaming via Sunshine/Moonlight

- **Status:** proposto (confirmar na fase 7)
- **Data:** 2026-11-03

## Contexto
Queremos jogar os jogos do PC na TV usando um controle Xbox conectado nela. Streaming de baixa latência (encode por hardware, transporte, controle virtual) é um problema grande e já resolvido por projetos open source.

## Decisão
- O homedeck lista as bibliotecas e lança os jogos.
- Pro streaming, usa o Sunshine como host e o protocolo Moonlight como cliente (inclusive o port pra Tizen).
- Não vamos implementar encode ou transporte próprios.

## Consequências
- Entrega muito mais rápida e qualidade comprovada.
- Dependência externa: precisa acompanhar as versões do Sunshine.
- Host macOS segue fraco (limitação do próprio Sunshine).
