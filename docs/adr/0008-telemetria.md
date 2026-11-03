# ADR 0008 — Telemetria opt-in, código público e dados privados

- **Status:** aceito
- **Data:** 2026-11-03

## Contexto
Queremos entender o uso real do app pra decidir prioridades: quais telas são usadas, quais recursos ninguém usa (e podem perder destaque), onde há erros e como cada plataforma se comporta.

O homedeck promete privacidade (local-first) e é GPL. A LGPD e a GDPR exigem consentimento claro.

## Decisão

### Consentimento
- **Desligada por padrão (opt-in).** No primeiro uso, uma tela pergunta se o usuário aceita enviar dados de uso, com as opções "Aceitar", "Não aceitar" e "Ver o que é enviado". As duas primeiras têm o mesmo destaque.
- Duas permissões separadas: **uso** (eventos anônimos) e **relatório de falhas** (crash).
- Dá pra mudar a qualquer momento nas configurações. Recusar não muda nada no app.
- A escolha vale por dispositivo e pode ser definida pelo admin da casa pra todos.
- Também pode ser desligada por variável de ambiente (`HOMEDECK_TELEMETRY=0`) e por build (`--no-default-features` sem o módulo de telemetria).

### O que é público e o que é privado
| Público (neste repo) | Privado (só do mantenedor) |
|---|---|
| Código do cliente de telemetria | Dados coletados |
| Catálogo de eventos ([TELEMETRY.md](../privacy/TELEMETRY.md)) | Dashboards e análises |
| Política de privacidade ([PRIVACY.md](../../PRIVACY.md)) | Infra do servidor e credenciais de admin |
| Endpoint usado | Repo privado `homedeck-ops` |

O código precisa ser público: é GPL, e o usuário tem o direito de auditar o que sai do dispositivo. Os **dados** são privados.

### O que nunca é coletado
- Nomes, caminhos ou hashes de arquivos.
- Títulos, metadados ou miniaturas de mídia.
- Nomes de dispositivos, contas, e-mails, IPs (o servidor não guarda IP), SSIDs, listas de jogos.
- Identificador permanente de usuário ou dispositivo.

### Ferramenta
**Aptabase** auto-hospedado:
- privacy-first, sem identificador único, ID de sessão rotativo diário;
- SDKs MIT, compatíveis com GPL;
- servidor AGPL com Postgres + ClickHouse.

Relatórios de falha: **GlitchTip** (compatível com Sentry) auto-hospedado, com scrub de caminhos e nomes antes do envio.

### Regras de engenharia
- Todo evento novo precisa estar no catálogo `TELEMETRY.md` **no mesmo PR**. Evento fora do catálogo não é enviado (allowlist no cliente).
- PR que adiciona ou altera evento passa pelo agente `security-reviewer`.
- O cliente agrupa e envia em lote, funciona offline e descarta a fila se o usuário desligar a telemetria.
- Modo "ver o que é enviado" mostra na UI os eventos locais antes do envio.
- Builds de desenvolvimento e CI nunca enviam.

## Alternativas consideradas
- **Sem telemetria:** decisões no escuro. Descartado.
- **Opt-out (ligada por padrão):** mais dados, mas quebra a confiança e é arriscado com a LGPD.
- **PostHog:** muito completo, mas pesado pra auto-hospedar e com mais risco de coletar demais.
- **Google/Firebase Analytics:** contraria a privacidade do projeto.

## Consequências
- Menos dados (só de quem aceita), mas confiáveis e defensáveis.
- Custo de servidor próprio, pago pelo mantenedor.
- O catálogo público obriga a pensar em cada evento antes de coletar.
