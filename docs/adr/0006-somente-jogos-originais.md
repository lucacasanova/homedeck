# ADR 0006 — Somente jogos originais de lojas oficiais

- **Status:** aceito
- **Data:** 2026-11-03

## Contexto
O M7 traz os jogos do PC pra TV. Precisamos de uma regra clara sobre de onde os jogos vêm, por razões legais, éticas e de reputação do projeto, e pra simplificar o escopo.

## Decisão
O homedeck só lista e abre jogos **instalados por lojas e launchers oficiais**:

Steam · EA app · Epic Games · GOG · Ubisoft Connect · Battle.net · Amazon Games · Xbox/Game Pass

- A execução sempre passa pelo **launcher oficial** (URI ou CLI da loja), respeitando login, DRM, anti-cheat e atualizações.
- No Linux, vale usar camadas de compatibilidade oficiais ou abertas que rodam o launcher original (Steam/Proton, Heroic, Lutris).
- **Fora do projeto:** ROMs, emuladores com jogos não comprados, cracks, contorno de DRM ou anti-cheat, lojas não oficiais.
- PRs que adicionem esse tipo de suporte são recusados.

## Consequências
- Escopo menor e bem definido.
- Alguns jogos com anti-cheat podem não aceitar controle virtual no streaming: documentamos em vez de contornar (#80).
- Política citada no README e no CONTRIBUTING.
