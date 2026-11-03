# Arquitetura

## Visão geral

```
┌──────────────── Nó homedeck (qualquer dispositivo) ────────────────┐
│  UI web compartilhada (packages/ui)                                 │
│        │  IPC (Tauri) / HTTP+WS (Tizen)                             │
│  Core (crates/core, Rust)                                           │
│   ├─ indexer     → varre pastas, ffprobe, thumbnails                │
│   ├─ dedup       → (pós-MVP) tamanho → hash parcial → BLAKE3        │
│   ├─ storage     → (pós-MVP) uso de disco, quarentena               │
│   ├─ stream      → HTTP Range + HLS (ffmpeg) quando o codec exige   │
│   ├─ discovery   → mDNS/DNS-SD `_homedeck._tcp` (+ fallbacks)       │
│   ├─ pairing     → PIN/QR, troca de chaves, TLS com cert fixado     │
│   ├─ control     → WebSocket: comandos (play, pause, seek, browse)  │
│   ├─ telemetry   → opt-in, allowlist do TELEMETRY.md, lote offline  │
│   └─ games       → (pós-MVP) bibliotecas de jogos, Sunshine         │
└─────────────────────────────────────────────────────────────────────┘
```

## Cascos por plataforma

| Casco | Plataformas | Core |
|---|---|---|
| `apps/desktop-mobile` (Tauri 2) | Windows, macOS, Linux, Android, Android TV, iOS | Core Rust embutido |
| `apps/tizen` (.wgt) | Samsung TV | Só cliente: UI + player. Fala com outros nós por HTTP/WS |

A TV Tizen é **cliente**: navega e toca o que está nos outros nós, mas não indexa nem serve arquivos.

## Estrutura do monorepo

```
apps/
  desktop-mobile/   # Tauri 2
  tizen/            # wrapper Tizen web app
packages/
  ui/               # UI compartilhada (spatial navigation)
  protocol/         # tipos/mensagens entre nós (gerados a partir do Rust)
crates/
  core/             # lógica do nó
docs/
  planning/         # visão e arquitetura (roadmap e tarefas no GitHub)
  design/           # design system
  adr/              # decisões de arquitetura
```

## Protocolo entre nós (rascunho)

- `GET  /api/v1/info` → id, nome, plataforma, versão (sem autenticação; usado na descoberta)
- `POST /api/v1/pair` → início do pareamento com PIN
- `GET  /api/v1/library?path=` → listagem (autenticado)
- `GET  /api/v1/media/{id}` → stream com Range
- `GET  /api/v1/media/{id}/hls/index.m3u8` → transcode sob demanda
- `WS   /api/v1/control` → comandos e eventos (play, pause, seek, now-playing)

## Segurança

- Cada nó gera um par de chaves na instalação.
- Pareamento: um mostra o PIN, o outro digita. Os dois trocam e fixam os certificados.
- Senha opcional por dispositivo (pra acessar) e por perfil (pra assistir).
- Nó não pareado só vê `/info`.

## Decisões abertas

Riscos abertos: issues com label [`type:risk`](https://github.com/lucacasanova/homedeck/labels/type%3Arisk). Cada decisão tomada vira um ADR em `docs/adr/`.
