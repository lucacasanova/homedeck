# ADR 0007 — Versionamento, numeração por componente e canais de release

- **Status:** aceito (mantenedor, 2026-11-03)
- **Data:** 2026-11-03
- **Issues:** #114, #9, #115

## Contexto
O homedeck é um monorepo com várias peças:
- interface;
- núcleo;
- protocolo entre aparelhos;
- banco local;
- app da TV;
- pacotes de cada plataforma.

Numa mesma casa, cada aparelho pode estar numa versão diferente. O mantenedor quer ver o **corpo inteiro da numeração**: qual versão de cada parte está rodando, não só "v0.3".

Ao mesmo tempo, as ferramentas exigem **SemVer de 3 partes** na versão do app: o atualizador automático (ADR 0011), o Android (`versionName`/`versionCode`), o AUR e o release-please. Um número com 5 partes (`0.x.xx.xxx.xxxx`) quebraria a comparação "qual versão é mais nova" e, com ela, a atualização automática.

## Decisão

### 1. Versão do produto — `vMAJOR.MINOR.PATCH`
É a única versão que lojas, atualizador e usuários comparam.

**Antes do MVP (`0.x`)**, o MINOR acompanha o milestone concluído:

| Versão | Marco |
|---|---|
| `0.1.x` | M0 — Fundação |
| `0.2.x` | M1 — Player Linux |
| `0.3.x` | M2 — Rede local |
| `0.4.x` | M3 — Tizen |
| `0.5.x` | M4 — Android e Android TV |
| `0.6.x` | M5 — Cast e controle remoto |
| **`1.0.0`** | **MVP completo** |

O PATCH sobe a cada release dentro do milestone (correções e partes entregues).

**Depois do MVP:**
- `MINOR` para funcionalidade nova;
- `PATCH` para correção;
- `MAJOR` quando quebra compatibilidade com aparelhos em versão anterior, por exemplo protocolo ou banco sem migração.

Os **Conventional Commits** definem o próximo número (release-please). Um MINOR que encerra um milestone antes da 1.0 é decidido pelo mantenedor (`Release-As:`).

### 2. Build — número sequencial
Cada build do CI recebe um **número de build crescente** (o `github.run_number` do workflow de release). Ele aparece como metadado SemVer, `0.2.3+build.1287`, e não afeta a comparação de versões.

### 3. Corpo da numeração — versões por componente
Cada build gera um **manifesto de versões** (`versions.json`), embutido no app e anexado à release:

| Componente | Formato | Sobe quando |
|---|---|---|
| `app` | SemVer do produto | toda release |
| `build` | inteiro | todo build |
| `commit` | SHA curto | todo build |
| `canal` | `stable` · `beta` · `nightly` | — |
| `interface` (`@homedeck/ui`) | SemVer próprio | muda a UI compartilhada |
| `nucleo` (`homedeck-core`) | SemVer próprio | muda o core |
| `protocolo` | inteiro (`1`, `2`…) | muda a API entre aparelhos |
| `banco` | inteiro (nº da última migration) | nova migration |
| `telemetria` | inteiro (versão do catálogo) | muda o `TELEMETRY.md` |
| `tizen` | SemVer do `.wgt` | muda o app da TV |
| `android` | `versionCode` | todo build Android |

Esse manifesto aparece:
- na tela **Configurações → Sobre**;
- no `homedeck --diagnostics`;
- nos relatórios de falha (ADR 0010);
- nas notas de release.

Exemplo de exibição:
```
homedeck v0.2.3 · build 1287 · beta
  interface 0.4.1 · núcleo 0.3.0 · protocolo 1 · banco 3 · telemetria 1
```

### 4. Compatibilidade entre aparelhos
O **protocolo** é o número que importa entre aparelhos. Uma versão do app fala com o protocolo atual e o anterior. Quando o protocolo muda sem compatibilidade, o MAJOR do produto sobe (depois da 1.0), e a tela de dispositivos mostra "atualize o homedeck na TV-sala".

### 5. Por plataforma
| Plataforma | Versão |
|---|---|
| Linux (AppImage, deb, rpm, AUR) | `X.Y.Z` (AUR: `pkgver=X.Y.Z`) |
| Android | `versionName = X.Y.Z`, `versionCode = X*1_000_000 + Y*10_000 + Z*100 + (build % 100)` |
| Tizen | `X.Y.Z` no `config.xml` |
| Site e notas | `vX.Y.Z (build N)` |

### 6. Canais
| Canal | Origem | Pra quem |
|---|---|---|
| nightly | cada merge na `main` (artefato do CI) | quem testa |
| beta | tag `vX.Y.Z-beta.N` | early adopters |
| stable | tag `vX.Y.Z` | todos |

### 7. Assinatura
- Atualizações e artefatos são **assinados** (minisign, via Tauri).
- A **chave pública** fica em `docs/release/updater-public-key.pub` e embutida no app.
- A **chave privada e a senha** existem só:
  - nos secrets do environment protegido `release`;
  - no backup offline do mantenedor.

## Alternativas consideradas
- **Versão de 4 ou 5 partes no produto:** quebra o atualizador, o Android e o AUR.
- **Uma versão independente por pacote, sem versão de produto:** confunde o usuário, que tem "o homedeck" em vários aparelhos.
- **CalVer:** não comunica compatibilidade.

## Consequências
- O usuário compara uma versão só (`v0.2.3`), e o detalhe completo está a um clique.
- O pipeline de release precisa gerar o `versions.json` e o número de build (#9).
- Mudança de protocolo, de schema do banco ou do catálogo de telemetria exige subir o número correspondente no mesmo PR (checklist da skill `review`).
