<div align="center">

# 🎮 homedeck

**Sua central de mídia local, com cara de console, em todos os seus dispositivos e sem servidor.**

Instale no PC, no celular e na TV. Eles se encontram sozinhos na rede de casa, e qualquer um mostra e controla a mídia dos outros.

[![Stars](https://img.shields.io/github/stars/lucacasanova/homedeck?style=for-the-badge&logo=github&color=d4f75a&labelColor=141a05)](https://github.com/lucacasanova/homedeck/stargazers)
[![Forks](https://img.shields.io/github/forks/lucacasanova/homedeck?style=for-the-badge&logo=github&color=d4f75a&labelColor=141a05)](https://github.com/lucacasanova/homedeck/network/members)
[![Issues](https://img.shields.io/github/issues/lucacasanova/homedeck?style=for-the-badge&color=d4f75a&labelColor=141a05)](https://github.com/lucacasanova/homedeck/issues)
[![License](https://img.shields.io/github/license/lucacasanova/homedeck?style=for-the-badge&color=d4f75a&labelColor=141a05)](LICENSE)

[![Last commit](https://img.shields.io/github/last-commit/lucacasanova/homedeck?style=flat-square)](https://github.com/lucacasanova/homedeck/commits/main)
[![Commit activity](https://img.shields.io/github/commit-activity/m/lucacasanova/homedeck?style=flat-square)](https://github.com/lucacasanova/homedeck/pulse)
[![Open PRs](https://img.shields.io/github/issues-pr/lucacasanova/homedeck?style=flat-square)](https://github.com/lucacasanova/homedeck/pulls)
[![Contributors](https://img.shields.io/github/contributors/lucacasanova/homedeck?style=flat-square)](https://github.com/lucacasanova/homedeck/graphs/contributors)
[![Repo size](https://img.shields.io/github/repo-size/lucacasanova/homedeck?style=flat-square)](https://github.com/lucacasanova/homedeck)
[![Built with Claude Code](https://img.shields.io/badge/built%20with-Claude%20Code-d97757?style=flat-square&logo=anthropic&logoColor=white)](https://claude.com/claude-code)

[Objetivo](#-objetivo) · [Funcionalidades](#-funcionalidades) · [Plataformas](#-plataformas) · [Conquistas](#-conquistas) · [Roadmap](#-roadmap) · [Arquitetura](#-arquitetura) · [Claude](#-desenvolvido-com-claude) · [Contribuir](#-como-contribuir)

</div>

> [!NOTE]
> **Status: em desenvolvimento (M0 — Fundação).** Ainda não há versão pra instalar. Acompanhe pelos [milestones](https://github.com/lucacasanova/homedeck/milestones) ou deixe uma ⭐ pra receber novidades.

---

## 🎯 Objetivo

Seus vídeos, músicas e fotos ficam espalhados entre PC, celular e HD externo. Pra ver na TV é preciso copiar arquivo, usar pendrive ou montar um servidor que exige configuração.

O **homedeck** resolve isso com três ideias:

1. **Cada dispositivo é um nó.** Não existe servidor central, conta ou nuvem. Instalou, funciona.
2. **Interface de console.** Tela cheia, navegável só com controle remoto, gamepad ou setas. Feita pra sofá.
3. **Tudo se fala.** Da TV você navega o que está no PC. Do celular você manda a TV tocar algo.

## ✨ Funcionalidades

### No MVP
- 🎬 **Vídeos** com legendas (embutidas e `.srt`), troca de faixa de áudio e continuar assistindo
- 🎵 **Músicas** por artista, álbum e pasta, com fila, aleatório e repetir
- 🖼️ **Imagens** com visualizador e slideshow
- 📡 **Descoberta automática** de dispositivos na rede local (mDNS)
- 🔐 **Login e senha em todos os dispositivos**, pareamento por PIN, conexão criptografada, perfis com PIN e login na TV pelo celular
- 📺 **Tocar em outro dispositivo** e controle remoto entre eles
- 🪟 **Visual de vidro translúcido** sobre um papel de parede escolhido em cada dispositivo ([design](docs/design/DESIGN.md))

### 🔭 Pós-MVP (no escopo, com milestones próprios)
- 🧹 **Gestão de arquivos:** duplicados exatos e quase-duplicados, também entre dispositivos; quarentena com desfazer (nunca apaga direto); painel de espaço; reconversão para H.265/AV1. [M6](https://github.com/lucacasanova/homedeck/milestone/8)
- 🕹️ **Jogos do PC na TV — somente originais:** Steam, EA app, Epic, GOG, Ubisoft Connect, Battle.net, Amazon e Xbox, sempre pelo launcher oficial; abrir pelo controle; streaming via Sunshine/Moonlight com controle Xbox ligado na TV. [M7](https://github.com/lucacasanova/homedeck/milestone/9)
- 🪟 **Windows, macOS e iOS.** [M8](https://github.com/lucacasanova/homedeck/milestone/10)
- 🏪 **Lojas oficiais:** Play Store, Samsung Apps, Flathub e App Store. [M9](https://github.com/lucacasanova/homedeck/milestone/11)
- 🐧 **Modo console:** sessão Linux que inicia direto no homedeck (estilo SteamOS), kiosk no Windows e launcher na Android TV. [M10](https://github.com/lucacasanova/homedeck/milestone/12)
- 🌐 **Acesso remoto e painel web:** configurar e assistir fora de casa, sem abrir porta no roteador, com 2FA. [M11](https://github.com/lucacasanova/homedeck/milestone/13)

## 📱 Plataformas

| Plataforma | Prioridade | Status | Distribuição |
|---|---|---|---|
| 🐧 Linux (Manjaro primeiro; Arch, Ubuntu, Fedora...) | **MVP** | 🚧 em desenvolvimento | AppImage, AUR |
| 🤖 Android | **MVP** | 📋 planejado | APK (GitHub Releases) |
| 📺 Android TV | **MVP** | 📋 planejado | APK |
| 📺 Samsung TV (Tizen) | **MVP** | 📋 planejado | `.wgt` (modo desenvolvedor) |
| 🪟 Windows · 🍎 macOS | pós-MVP | 🔭 [M8](https://github.com/lucacasanova/homedeck/milestone/10) | .msi, .dmg |
| 📱 iOS | pós-MVP | 🔭 [M8](https://github.com/lucacasanova/homedeck/milestone/10) | App Store |
| 🏪 Play Store · Samsung Apps · Flathub | pós-MVP | 🔭 [M9](https://github.com/lucacasanova/homedeck/milestone/11) | lojas oficiais |

## 🏆 Conquistas

O progresso é atualizado automaticamente a partir dos milestones do GitHub.

#### 🎯 MVP

| Marco | O que entrega | Progresso |
|---|---|---|
| [M0 — Fundação](https://github.com/lucacasanova/homedeck/milestone/1) | Workspace, casco Tauri, CI Linux/Android | ![](https://img.shields.io/github/milestones/progress-percent/lucacasanova/homedeck/1?style=flat-square&color=d4f75a) |
| [M1 — Player Linux](https://github.com/lucacasanova/homedeck/milestone/2) | Biblioteca + vídeo, música e imagem no Manjaro | ![](https://img.shields.io/github/milestones/progress-percent/lucacasanova/homedeck/2?style=flat-square&color=d4f75a) |
| [M2 — Rede local](https://github.com/lucacasanova/homedeck/milestone/3) | Descoberta, pareamento, streaming entre PCs | ![](https://img.shields.io/github/milestones/progress-percent/lucacasanova/homedeck/3?style=flat-square&color=d4f75a) |
| [M3 — Tizen](https://github.com/lucacasanova/homedeck/milestone/4) | TV Samsung tocando mídia do PC | ![](https://img.shields.io/github/milestones/progress-percent/lucacasanova/homedeck/4?style=flat-square&color=d4f75a) |
| [M4 — Android e Android TV](https://github.com/lucacasanova/homedeck/milestone/5) | Celular e Android TV | ![](https://img.shields.io/github/milestones/progress-percent/lucacasanova/homedeck/5?style=flat-square&color=d4f75a) |
| [M5 — Cast e controle remoto](https://github.com/lucacasanova/homedeck/milestone/6) | "Tocar em…" e controle remoto | ![](https://img.shields.io/github/milestones/progress-percent/lucacasanova/homedeck/6?style=flat-square&color=d4f75a) |

#### 🔭 Pós-MVP

As ideias abaixo fazem parte do projeto e já têm milestone e issues. Começam depois do MVP.

| Marco | O que entrega | Progresso |
|---|---|---|
| [M6 — Gestão de arquivos](https://github.com/lucacasanova/homedeck/milestone/8) | Duplicados, quarentena, espaço, reconversão | ![](https://img.shields.io/github/milestones/progress-percent/lucacasanova/homedeck/8?style=flat-square&color=c2a4f5) |
| [M7 — Jogos](https://github.com/lucacasanova/homedeck/milestone/9) | Jogos do PC na TV com controle Xbox | ![](https://img.shields.io/github/milestones/progress-percent/lucacasanova/homedeck/9?style=flat-square&color=c2a4f5) |
| [M8 — Windows, macOS e iOS](https://github.com/lucacasanova/homedeck/milestone/10) | Novas plataformas | ![](https://img.shields.io/github/milestones/progress-percent/lucacasanova/homedeck/10?style=flat-square&color=c2a4f5) |
| [M9 — Lojas e distribuição](https://github.com/lucacasanova/homedeck/milestone/11) | Play Store, Samsung Apps, Flathub, App Store | ![](https://img.shields.io/github/milestones/progress-percent/lucacasanova/homedeck/11?style=flat-square&color=c2a4f5) |
| [M10 — Modo console](https://github.com/lucacasanova/homedeck/milestone/12) | Sessão estilo SteamOS, kiosk, launcher | ![](https://img.shields.io/github/milestones/progress-percent/lucacasanova/homedeck/12?style=flat-square&color=c2a4f5) |
| [M11 — Acesso remoto e painel web](https://github.com/lucacasanova/homedeck/milestone/13) | Configurar e assistir fora de casa | ![](https://img.shields.io/github/milestones/progress-percent/lucacasanova/homedeck/13?style=flat-square&color=c2a4f5) |

## 🗺️ Roadmap

O planejamento fica **no GitHub**, não em arquivos:

- **[Board do projeto](https://github.com/users/lucacasanova/projects/2):** Kanban com Backlog → Pronto pra começar → Em andamento → Em revisão → Feito.
- **[Milestones](https://github.com/lucacasanova/homedeck/milestones):** fases com critério de pronto.
- **[Issues](https://github.com/lucacasanova/homedeck/issues):** cada milestone tem um épico, e as tarefas são sub-issues dele.
- **Labels:** `type:*`, `area:*`, `platform:*`, `priority:p0–p2`, `pós-mvp`.
- **Riscos técnicos:** issues com `type:risk` e `type:poc`, validados em dispositivo real antes do código definitivo.

```
MVP
M0 Fundação ─► M1 Player Linux ─► M2 Rede local ─┬─► M3 Tizen ──────┐
                                                 └─► M4 Android/TV ─┴─► M5 Cast
Pós-MVP
M6 Gestão de arquivos · M7 Jogos · M8 Windows/macOS/iOS · M9 Lojas · M10 Modo console · M11 Acesso remoto
```

## 🧱 Arquitetura

```
┌─────────────── nó homedeck ───────────────┐
│  UI web compartilhada (vidro, 10-foot)    │
│            │                              │
│  Core em Rust: biblioteca · player ·      │
│  descoberta · pareamento · streaming      │
└───────────────────────────────────────────┘
  Linux / Android / Android TV → Tauri 2 (nó completo)
  Samsung TV (Tizen)           → web app (cliente)
```

| Camada | Tecnologia |
|---|---|
| Core | Rust, SQLite, ffmpeg/ffprobe |
| UI | Web (framework a decidir em [#3](https://github.com/lucacasanova/homedeck/issues/3)) com spatial navigation |
| Casco | Tauri 2 · web app Tizen |
| Rede | mDNS/DNS-SD, HTTP Range, HLS, WebSocket, TLS com certificado fixado |

Detalhes: [ARCHITECTURE.md](docs/planning/ARCHITECTURE.md) · [Visão](docs/planning/VISION.md) · [Design](docs/design/DESIGN.md) · [ADRs](docs/adr/)

## 🤖 Desenvolvido com Claude

Este projeto é desenvolvido com [Claude Code](https://claude.com/claude-code), com regras versionadas no próprio repositório pra que o processo continue consistente ao longo do tempo:

| Arquivo | Pra que serve |
|---|---|
| [`CLAUDE.md`](CLAUDE.md) | Regras do projeto: idioma, commits, branches, segurança, UI |
| [`.claude/skills/feature`](.claude/skills/feature/SKILL.md) | Fluxo de toda tarefa: issue → branch → ADR → teste → review → PR |
| [`.claude/skills/adr`](.claude/skills/adr/SKILL.md) | Registra decisões de arquitetura |
| [`.claude/skills/poc`](.claude/skills/poc/SKILL.md) | Valida riscos em dispositivo real |
| [`.claude/skills/review`](.claude/skills/review/SKILL.md) | Checklist de revisão do projeto |
| [`.claude/skills/sync`](.claude/skills/sync/SKILL.md) | Status do projeto a partir das issues e milestones |
| [`.claude/skills/triage`](.claude/skills/triage/SKILL.md) | Triagem de issues novas: labels, milestone, épico e board |
| [`.claude/skills/release`](.claude/skills/release/SKILL.md) | Release seguindo o ADR 0007 |
| [`.claude/agents/reviewer`](.claude/agents/reviewer.md) | Agente revisor independente (somente leitura) |
| [`.claude/agents/security-reviewer`](.claude/agents/security-reviewer.md) | Agente de revisão de segurança |
| [`.claude/hooks/session-status.sh`](.claude/hooks/session-status.sh) | Mostra o status do projeto ao abrir o Claude |
| [`.claude/hooks/guard-git.sh`](.claude/hooks/guard-git.sh) | Bloqueia commit na `main`, force-push e commit fora do padrão |

Todo código passa por PR e revisão humana; o Claude nunca faz merge nem publica releases. Commits feitos com Claude levam `Co-Authored-By`. Veja a [política de contribuições com IA](CONTRIBUTING.md#contribuições-com-ia).

## 🔢 Versionamento e releases

- **SemVer** com versão única pra todas as plataformas. `0.x` até o MVP; `1.0.0` = MVP completo ([ADR 0007](docs/adr/0007-versionamento-e-releases.md)).
- **Canais:** nightly (cada merge), beta (`vX.Y.Z-beta.N`) e stable (`vX.Y.Z`).
- **CHANGELOG** e releases gerados automaticamente a partir dos Conventional Commits.
- **Artefatos assinados**, com `SHA256SUMS` e attestation de build.

## 🕵️ Privacidade e telemetria

- Sua mídia, contas e configurações **nunca saem dos seus dispositivos**.
- A telemetria é **opcional e vem desligada**. No primeiro uso você escolhe; dá pra mudar a qualquer momento.
- O que pode ser enviado está todo listado em [TELEMETRY.md](docs/privacy/TELEMETRY.md): dados anônimos, sem IP, sem identificador e sem nada sobre o seu conteúdo.
- Veja a [Política de Privacidade](PRIVACY.md) e o [ADR 0008](docs/adr/0008-telemetria.md).

## 🔒 Segurança

Encontrou uma falha? **Não abra issue pública.** Use o [reporte privado](https://github.com/lucacasanova/homedeck/security/advisories/new). Veja o [SECURITY.md](SECURITY.md).

## 🤝 Como contribuir

1. Escolha uma issue, de preferência com [`good first issue`](https://github.com/lucacasanova/homedeck/labels/good%20first%20issue) ou [`help wanted`](https://github.com/lucacasanova/homedeck/labels/help%20wanted).
2. Comente na issue que vai pegar.
3. Siga o [CONTRIBUTING.md](CONTRIBUTING.md): branch `tipo/N-nome`, Conventional Commits assinados (`git commit -s`), PR ligado à issue.

Dúvidas e ideias: [Discussions](https://github.com/lucacasanova/homedeck/discussions) · Regras de convivência: [Código de Conduta](CODE_OF_CONDUCT.md) · Como decidimos: [GOVERNANCE.md](GOVERNANCE.md) · Ajuda: [SUPPORT.md](SUPPORT.md)

### Contribuidores

<a href="https://github.com/lucacasanova/homedeck/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=lucacasanova/homedeck" alt="Contribuidores" />
</a>

## ⭐ Histórico de estrelas

<a href="https://star-history.com/#lucacasanova/homedeck&Date">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=lucacasanova/homedeck&type=Date&theme=dark" />
    <img alt="Star History" src="https://api.star-history.com/svg?repos=lucacasanova/homedeck&type=Date" />
  </picture>
</a>

## 📄 Licença

[GPL-3.0-or-later](LICENSE). Use, estude, modifique e distribua à vontade. Versões modificadas distribuídas precisam continuar abertas.

<div align="center">
<sub>Feito no Brasil 🇧🇷 · Se o projeto te interessa, deixe uma ⭐</sub>
</div>
