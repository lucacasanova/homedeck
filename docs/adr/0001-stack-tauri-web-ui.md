# ADR 0001 — Tauri 2 + UI web compartilhada + core em Rust

- **Status:** aceito
- **Data:** 2026-11-03

## Contexto
Precisamos de Windows, macOS, Linux, Android, Android TV, iOS e Tizen. A Tizen só roda bem apps web (`.wgt`).

## Decisão
- Uma UI web compartilhada (`packages/ui`) em todas as plataformas.
- Tauri 2 como casco de desktop e mobile, com o core em Rust embutido.
- Na Tizen, a mesma UI empacotada como web app, atuando só como cliente.

## Alternativas consideradas
- **Flutter:** suporte Tizen via `flutter-tizen` (comunidade Samsung), mas a UI não seria reaproveitável como web e o ecossistema de mídia em desktop Linux é mais fraco.
- **Electron + React Native:** duas bases de código, binários pesados.
- **Kotlin Multiplatform:** sem caminho pra Tizen.

## Consequências
- Uma UI só pra manter.
- O core em Rust dá desempenho pro hash, a indexação e o streaming.
- Risco R6: o mobile do Tauri 2 ainda é mais jovem. Validar cedo.
