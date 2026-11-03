# Roadmap

Cada fase vira um milestone no GitHub. Só avança quando os critérios de pronto da fase atual forem atendidos.

## Fase 0 — Fundação
- Repo, licença, docs, ADRs, CI, templates.
- **Pronto quando:** CI verde num monorepo vazio que compila em Linux, Windows e macOS.

## Fase 1 — Desktop solo
- Adicionar pastas, indexar vídeos, miniaturas.
- Player integrado e UI navegável por controle.
- Detecção de duplicados + quarentena.
- Painel de uso de espaço.
- **Pronto quando:** dá pra gerenciar e assistir a própria biblioteca só com um gamepad.

## Fase 2 — Rede local
- Descoberta mDNS, pareamento com PIN, TLS.
- Navegar e tocar a biblioteca de outro PC.
- Senha por dispositivo e por perfil.
- **Pronto quando:** dois PCs pareados assistem os arquivos um do outro.

## Fase 3 — POC Tizen
- Validar os riscos R1, R2 e R4 (ver RISKS.md).
- App Tizen mínimo: listar nós, navegar, tocar.
- **Pronto quando:** a TV Samsung toca um vídeo do PC.

## Fase 4 — Android e Android TV
- Build Tauri Android, manifest leanback, navegação por D-pad.

## Fase 5 — Cast e controle remoto
- Do PC/celular: "tocar na TV", controlar play/pause/seek.
- Fila e "tocando agora" sincronizados.

## Fase 6 — iOS e lojas
- Build iOS, publicação na App Store, Play Store e Samsung Apps.
- Pacotes Linux: Flatpak e AUR.

## Fase 7 — Jogos
- **7a:** listar jogos do PC (Steam, Epic, GOG, Heroic, Lutris) e abrir pelo controle da TV.
- **7b:** streaming pra TV via Sunshine + cliente Moonlight, com controle Xbox ligado na TV.

## Extra — Sessão "SteamOS"
- Sessão Linux própria no display manager que inicia direto no homedeck.
