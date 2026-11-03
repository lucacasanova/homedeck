# Política de segurança

O homedeck guarda contas, senhas e acesso à mídia pessoal de cada casa. Levamos segurança a sério.

## Versões suportadas

| Versão | Suporte |
|---|---|
| última `0.x` | ✅ correções de segurança |
| anteriores | ❌ |

Depois da `1.0.0`, a última versão minor recebe correções.

## Como reportar uma vulnerabilidade

**Não abra issue pública.**

1. Use o [reporte privado do GitHub](https://github.com/lucacasanova/homedeck/security/advisories/new).
2. Descreva o problema, a versão, a plataforma e, se possível, os passos pra reproduzir.
3. Prazos:
   - confirmação de recebimento em até **72 h**;
   - plano de correção em até **14 dias**.
4. Combinamos juntos a data de divulgação, e você recebe crédito no advisory, se quiser.

## Escopo

Entram no escopo:
- autenticação, sessões e contas;
- pareamento e TLS;
- acesso a arquivos fora das pastas da biblioteca (path traversal);
- vazamento de mídia ou metadados pela rede;
- execução remota de código;
- artefatos de release adulterados.

## Contribuições externas

- Workflows de PRs de fork só rodam depois da aprovação do mantenedor.
- Todo PR passa pelo pré-scan (`.github/scripts/pr_prescan.py`), que lê o diff pela API sem executar o código e procura: caracteres invisíveis, `curl | sh`, blobs ofuscados, binários, scripts de instalação, domínios e APIs sensíveis novos, e alterações em áreas críticas.
- A revisão de dependências bloqueia vulnerabilidades moderadas ou piores e licenças incompatíveis com a GPL.
- O mantenedor revisa todo PR externo sem executar o código localmente.

## Princípios do projeto

- Nó não pareado e usuário não logado só acessam `/api/v1/info`.
- Senhas com Argon2id. Nunca em log.
- Nenhum nó aceita conexão de entrada pela internet (ver ADR 0005).
- Dependências auditadas no CI (cargo-deny, CodeQL, Dependabot).
