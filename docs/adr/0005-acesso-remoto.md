# ADR 0005 — Acesso remoto e painel web

- **Status:** proposto (decisão final na issue #108, M11)
- **Data:** 2026-11-03

## Contexto
O MVP funciona só na rede local. No futuro o usuário quer configurar e acessar o homedeck pela web pública. Expor um nó direto na internet (porta aberta no roteador) é o caminho mais arriscado.

## Direção
- Nenhum nó aceita conexão de entrada pela internet. **Os nós conectam pra fora**: via relay ou túnel.
- Criptografia **ponta a ponta**. O relay não enxerga conteúdo nem credenciais.
- Login remoto exige **2FA** (TOTP ou passkeys), usando as contas do ADR 0004.
- Painel web com a mesma identidade visual do app.

## Opções a comparar
| Opção | Prós | Contras |
|---|---|---|
| Relay próprio auto-hospedável | Controle total, open source | Custo de servidor; manter infraestrutura |
| Túnel WireGuard (Tailscale/Headscale) | Seguro e maduro | Dependência externa ou setup técnico |
| Painel hospedado pelo projeto | Fácil pro usuário | Custo e responsabilidade pelos dados |

## Consequências
- O protocolo entre nós deve ser independente do transporte (local ou relay) desde o M2.
- Antes de liberar, revisão de segurança obrigatória (#113).
