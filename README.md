# homedeck

Central de mídia local, estilo console, que roda em todos os seus dispositivos e os conecta pela rede de casa.

> **Status:** planejamento (fase 0). Ainda não existe nada pronto pra usar.

## O que vai fazer

- **Interface estilo console** (10-foot UI) navegável por controle remoto, gamepad ou teclado.
- **Biblioteca de vídeos locais**: indexa pastas, gera miniaturas, toca tudo.
- **Cuidado com espaço**: encontra arquivos duplicados (vão pra quarentena, nunca apagados direto) e mostra o que ocupa mais.
- **Dispositivos conectados**: PC, celular, Android TV e TV Samsung (Tizen) se descobrem na mesma rede.
- **Gerenciar e transmitir**: da TV ver os arquivos do PC; do PC mandar a TV tocar algo.
- **Senha por dispositivo e por perfil**: pareamento com PIN e conexão criptografada.
- **Jogos do PC na TV** (futuro): listar a biblioteca e jogar na TV com controle Xbox, via [Sunshine](https://github.com/LizardByte/Sunshine).

## Plataformas

| Plataforma | Distribuição |
|---|---|
| Windows | `.msi` / `.exe` |
| macOS | `.dmg` |
| Linux (Ubuntu, Fedora, Manjaro, Arch...) | `.deb`, `.rpm`, `.AppImage`, Flatpak, AUR |
| Android / Android TV | Google Play + APK |
| iOS | App Store |
| Samsung TV (Tizen) | Samsung Apps (`.wgt`) |

## Documentação

- [Visão](docs/planning/VISION.md)
- [Arquitetura](docs/planning/ARCHITECTURE.md)
- [Roadmap](docs/planning/ROADMAP.md)
- [Riscos e POCs](docs/planning/RISKS.md)
- [TODO](docs/planning/TODO.md)
- [Decisões de arquitetura (ADRs)](docs/adr/)
- [Como contribuir](CONTRIBUTING.md)

## Licença

[GPL-3.0](LICENSE). Forks e versões modificadas precisam continuar abertos.
