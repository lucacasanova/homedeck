# Visão

## Problema

Vídeos ficam espalhados entre PC, celular e HD externo. Pra ver na TV é preciso copiar arquivo, usar pendrive ou montar um servidor (Plex/Jellyfin) que exige configuração. Arquivos duplicados ocupam espaço sem ninguém perceber.

## Proposta

Instalar o homedeck em cada dispositivo e pronto: eles se acham na rede, cada um mostra o que os outros têm, e qualquer um controla qualquer outro.

## Princípios

1. **Sem servidor central.** Todo dispositivo é um nó. Não depende de nuvem nem de conta.
2. **Local-first e privado.** Nada sai da rede de casa.
3. **Interface de console.** Tudo funciona só com setas + OK + voltar.
4. **Seguro por padrão.** Pareamento do dispositivo + login e senha do usuário em todos os dispositivos; tráfego criptografado.
5. **Nunca apagar sem confirmação.** Duplicados vão pra quarentena, com desfazer.
6. **Leve.** Roda bem em TV e em PC fraco.

## Diferencial frente a Jellyfin / Kodi / Plex

| | homedeck | Jellyfin/Plex | Kodi |
|---|---|---|---|
| Sem servidor central (P2P) | ✅ | ❌ | ❌ |
| Gestão de duplicados e espaço | ✅ | ❌ | ❌ |
| Cast entre qualquer par de dispositivos | ✅ | parcial | parcial |
| Jogos do PC na TV | ✅ (via Sunshine) | ❌ | ❌ |

## MVP

- **Plataformas:** Linux (Manjaro primeiro), Android, Android TV e Samsung TV (Tizen).
- **Escopo:** reproduzir vídeo, música e imagem, localmente e entre dispositivos da rede, com pareamento seguro e cast.

## Pós-MVP (label `pós-mvp`, milestones M6–M10)

Essas ideias fazem parte da visão do projeto e têm milestones próprios. Só começam depois do MVP.

- **M6 — Gestão de arquivos:** duplicados (exatos, quase-duplicados, entre dispositivos), quarentena, painel de espaço, reconversão.
- **M7 — Jogos:** listar e abrir jogos do PC pela TV; streaming com controle Xbox ligado na TV (Sunshine/Moonlight).
- **M8 — Windows, macOS e iOS.**
- **M9 — Lojas:** Play Store, Samsung Apps, Flathub, App Store.
- **M10 — Modo console:** sessão Linux estilo SteamOS, kiosk no Windows, launcher na Android TV.
- **M11 — Acesso remoto e painel web:** configurar e acessar fora de casa, sem expor nós na internet, com 2FA.

Jogos: **somente originais**, de lojas oficiais (Steam, EA app, Epic, GOG, Ubisoft Connect, Battle.net, Amazon Games, Xbox/Game Pass). Ver ADR 0006.

## Fora do escopo

- Jogos não originais, ROMs, cracks ou contorno de DRM.
- Download de metadados online (capas, sinopses).
