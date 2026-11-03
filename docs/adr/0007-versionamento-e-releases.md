# ADR 0007 — Versionamento e canais de release

- **Status:** proposto
- **Data:** 2026-11-03

## Contexto
Monorepo com várias plataformas (Linux, Android, Android TV, Tizen) e um protocolo entre nós que precisa de compatibilidade entre versões diferentes instaladas na casa.

## Decisão
- **SemVer** com **uma versão única** pro monorepo inteiro (`X.Y.Z`). Todas as plataformas são lançadas juntas.
- **0.x até o MVP fechar (M5).** A versão `1.0.0` marca o MVP completo.
- Os **Conventional Commits** definem o próximo número: `fix` → patch, `feat` → minor, `!`/`BREAKING CHANGE` → major (só depois da 1.0).
- **release-please** gera o PR de release, o `CHANGELOG.md` e a tag `vX.Y.Z`.
- **Canais:**
  | Canal | Origem | Pra quem |
  |---|---|---|
  | nightly | cada merge na `main` | quem testa |
  | beta | tag `vX.Y.Z-beta.N` | early adopters |
  | stable | tag `vX.Y.Z` | todos |
- **Versões por plataforma:** Android `versionCode = X*10000 + Y*100 + Z`; Tizen `X.Y.Z`; AUR `pkgver=X.Y.Z`.
- **Protocolo versionado à parte** (`/api/v1`). Uma versão N do app conversa com o protocolo atual e o anterior. Quebra de protocolo exige ADR.
- Artefatos de release saem assinados, com `SHA256SUMS` e attestation de build (#115).

## Alternativas consideradas
- **Versão por pacote (changesets):** flexível, mas confunde o usuário, que tem o "homedeck" em vários dispositivos.
- **CalVer:** não comunica compatibilidade.

## Consequências
- O usuário vê a mesma versão em todos os dispositivos.
- Release de uma correção só pra Tizen também gera versão nova nas outras plataformas. Aceitável.
