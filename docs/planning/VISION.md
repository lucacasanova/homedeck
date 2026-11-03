# Visão

## Problema

Vídeos ficam espalhados entre PC, celular e HD externo. Pra ver na TV é preciso copiar arquivo, usar pendrive ou montar um servidor (Plex/Jellyfin) que exige configuração. Arquivos duplicados ocupam espaço sem ninguém perceber.

## Proposta

Instalar o homedeck em cada dispositivo e pronto: eles se acham na rede, cada um mostra o que os outros têm, e qualquer um controla qualquer outro.

## Princípios

1. **Sem servidor central.** Todo dispositivo é um nó. Não depende de nuvem nem de conta.
2. **Local-first e privado.** Nada sai da rede de casa.
3. **Interface de console.** Tudo funciona só com setas + OK + voltar.
4. **Seguro por padrão.** Pareamento obrigatório; tráfego criptografado.
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

## Depois do MVP (label `later`)

- Gestão de arquivos: duplicados, quarentena, painel de espaço.
- Jogos do PC na TV (Sunshine/Moonlight).
- Windows, macOS, iOS e publicação nas lojas.
- Sessão Linux estilo SteamOS.

## Fora do escopo

- Acesso fora da rede local (internet).
- Download de metadados online (capas, sinopses).
