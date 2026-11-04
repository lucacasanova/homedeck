# Política de Privacidade

_Última atualização: 2026-11-03 (atualizações e relatórios de falha)_

O homedeck é **local-first**: sua mídia, suas contas e suas configurações ficam nos seus dispositivos.

## O que fica só com você

- Arquivos de mídia, nomes, caminhos, miniaturas e metadados.
- Contas, senhas (guardadas como hash Argon2id) e sessões.
- Lista de dispositivos pareados.
- Biblioteca de jogos.

Nada disso é enviado ao projeto, a nenhum servidor nosso nem a terceiros.

## Comunicação na rede

No MVP, os dispositivos conversam **apenas na sua rede local**, com criptografia. Quando o acesso remoto existir (M11), ele vai ser opcional, com criptografia ponta a ponta, e esta política vai ser atualizada antes.

## Verificação de atualizações

- O app consulta as releases do projeto no GitHub pra saber se existe versão nova (ADR 0011).
- Essa consulta envia só o que qualquer download envia: IP e versão do app, que o GitHub recebe como qualquer acesso. O projeto não guarda nada disso.
- Pode ser desligada nas configurações ou com `HOMEDECK_UPDATES=0`.
- Toda atualização é assinada; o app recusa pacotes com assinatura inválida.

## Relatórios de falha na inicialização

- Se o app não conseguir abrir (por exemplo, por um problema no driver de vídeo), um relatório técnico é salvo **só no seu computador**, em `~/.local/share/io.github.lucacasanova.homedeck/crash/` (ADR 0010).
- Ele contém sistema, GPU, driver e o motivo da falha. A pasta pessoal aparece como `~`, e nenhum arquivo seu é incluído.
- Ele só sai do computador se você clicar em "Reportar", e você vê o conteúdo antes de enviar.

## Telemetria (opcional)

- **Desligada por padrão.** Só é enviada se você aceitar, na primeira abertura ou nas configurações.
- São dados anônimos de uso (telas, tipo de mídia, plataforma, versão) e, com uma permissão separada, relatórios de falha.
- A lista completa do que pode ser enviado está em [docs/privacy/TELEMETRY.md](docs/privacy/TELEMETRY.md). O que não está lá não sai do dispositivo.
- Não coletamos IP, identificador permanente nem nada sobre o seu conteúdo.
- Os dados ficam num servidor próprio do mantenedor e não são vendidos nem compartilhados.
- Eventos brutos são apagados depois de 180 dias.

**Base legal (LGPD, art. 7º, I):** consentimento, que você pode revogar a qualquer momento nas configurações.

## Seus direitos

Como os eventos são anônimos e sem identificador, não conseguimos associar dados a você. Pra dúvidas sobre privacidade, abra uma [discussão](https://github.com/lucacasanova/homedeck/discussions) ou use o [reporte privado](https://github.com/lucacasanova/homedeck/security/advisories/new) para assuntos sensíveis.

## Mudanças

Mudanças nesta política aparecem no histórico do repositório e nas notas de versão. Se o tipo de dado coletado mudar, o app pede seu consentimento de novo.
