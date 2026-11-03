# Telemetria — catálogo de eventos

> Desligada por padrão. Só é enviada se você aceitar. Veja o [ADR 0008](../adr/0008-telemetria.md) e a [Política de Privacidade](../../PRIVACY.md).

Este arquivo é a **lista completa** do que o homedeck pode enviar. O cliente só envia eventos listados aqui (allowlist). Se não está aqui, não sai do seu dispositivo.

## Propriedades enviadas em todo evento

| Propriedade | Exemplo | Por quê |
|---|---|---|
| `app_version` | `0.3.1` | Saber em qual versão um problema aparece |
| `platform` | `linux`, `android`, `android-tv`, `tizen` | Priorizar plataformas |
| `os_version` | `Tizen 7.0`, `Android 14` | Compatibilidade |
| `locale` | `pt-BR` | Priorizar traduções |
| `session` | rotativo, gerado pelo SDK | Agrupar eventos de uma sessão, sem identificar ninguém |

Os eventos não levam IP, nome do dispositivo, conta nem identificador permanente.

## Eventos

Os nomes seguem `area.acao`. As propriedades são só de valores fixos (enum) ou faixas numéricas, nunca texto livre.

### App
| Evento | Propriedades | Pra quê |
|---|---|---|
| `app.started` | `cold_start_ms` (faixa) | Desempenho de abertura |
| `app.consent_changed` | `usage: on/off`, `crash: on/off` | Taxa de aceite (enviado só se `usage` = on após a mudança) |

### Navegação
| Evento | Propriedades | Pra quê |
|---|---|---|
| `screen.viewed` | `screen`: `home`, `videos`, `music`, `images`, `devices`, `settings`, `player`… | Saber quais telas são usadas, ou ignoradas, e priorizar ou tirar destaque |
| `input.method` | `method`: `remote`, `gamepad`, `keyboard`, `touch`, `mouse` | Priorizar o tipo de controle |

### Reprodução
| Evento | Propriedades | Pra quê |
|---|---|---|
| `playback.started` | `kind`: `video/music/image`, `source`: `local/remote`, `transcoded`: bool | Uso por tipo de mídia e quanto precisa de transcode |
| `playback.failed` | `kind`, `reason`: `codec/network/permission/other` | Achar formatos e plataformas problemáticos |

### Rede
| Evento | Propriedades | Pra quê |
|---|---|---|
| `devices.discovered` | `count` (faixa 0, 1, 2–3, 4+), `method`: `mdns/scan/manual` | Validar descoberta (risco R1) |
| `pairing.completed` | `result`: `ok/failed/cancelled` | Facilidade de pareamento |
| `cast.started` | `from_platform`, `to_platform` | Combinações de cast mais usadas |

### Recursos
| Evento | Propriedades | Pra quê |
|---|---|---|
| `feature.used` | `feature`: valor fixo da lista em `packages/protocol` | Uso de cada recurso, pra decidir prioridade e destaque na UI |

## Relatório de falhas (permissão separada)

Stack trace, versão, plataforma e tipo de erro. Antes do envio, **caminhos de arquivo, nomes de usuário e nomes de mídia são removidos** do relatório.

## Como ver ou desligar

- **Configurações → Privacidade:** ligar e desligar uso e falhas separadamente; "Ver o que é enviado" mostra os eventos na fila.
- **Variável de ambiente:** `HOMEDECK_TELEMETRY=0`.
- **Build próprio:** compilar sem a feature `telemetry`.

## Retenção

Eventos brutos são guardados por **180 dias**. Depois, só agregados (contagens por dia, versão e plataforma).

## Histórico

| Versão | Mudança |
|---|---|
| — | Catálogo inicial (nenhum evento implementado ainda) |
