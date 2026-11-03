# ADR 0009 — UI em React + Vite + TypeScript, com Norigin Spatial Navigation

- **Status:** aceito
- **Data:** 2026-11-03
- **Issue:** #3

## Contexto
A UI compartilhada (`packages/ui`, ADR 0001) roda em quatro lugares:
- Tauri 2 (Linux, depois Windows/macOS);
- Tauri 2 Android e Android TV;
- web app Tizen nas TVs Samsung.

Requisitos:
- navegação **só com setas, OK e voltar** em todas as telas;
- desempenho aceitável em TVs com GPU e CPU fracas;
- base de contribuidores grande;
- licença compatível com GPL-3.0.

## Decisão
- **React** (última versão estável) com **TypeScript** estrito.
- **Vite** como bundler e servidor de desenvolvimento, que é o padrão dos templates do Tauri 2.
- **@noriginmedia/norigin-spatial-navigation** para foco e navegação por controle remoto ou gamepad. É feita pra Smart TVs (incluindo Tizen), calcula o próximo foco pela geometria e dispensa ligar cada elemento à mão.
- Estilo em **CSS com variáveis** (tokens do `docs/design/DESIGN.md`), sem framework de CSS pesado. A fonte vem de `@fontsource/outfit`.
- Build com alvo compatível com o navegador mais antigo suportado da Tizen, a ser definido no POC #40 e no esqueleto #41.

## Alternativas consideradas
- **Svelte 5:** runtime menor e muito rápido, mas sem integração madura de navegação espacial; teríamos que usar só o núcleo da Norigin ou escrever a nossa. Comunidade menor.
- **Preact:** API do React em cerca de 4 KB, mas o modo de compatibilidade nem sempre funciona com bibliotecas React. Pode ser revisitado se o React pesar demais na TV.
- **Solid:** rápido, mas com ecossistema de TV pequeno.

## Consequências
- Contribuidores encontram uma stack conhecida.
- O runtime do React (~45 KB gzip) precisa de cuidado na TV: code splitting por tela, listas virtualizadas e o modo leve do vidro (#138).
- Se as medições na Tizen mostrarem um gargalo do React, a migração para Preact pode ser avaliada num ADR novo, porque a API é quase a mesma.
- Licenças: React (MIT), Vite (MIT) e Norigin (MIT), todas compatíveis com GPL-3.0.
