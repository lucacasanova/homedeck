---
name: adr
description: Registra uma decisão de arquitetura do homedeck como ADR em docs/adr/. Use quando surgir escolha de biblioteca, framework, protocolo, formato de dados, estrutura de pastas, segurança ou qualquer decisão difícil de desfazer, ou quando o usuário pedir "ADR", "registrar decisão".
---

# Criar ADR

1. Liste `docs/adr/` e pegue o próximo número (4 dígitos).
2. Copie `docs/adr/0000-template.md` pra `docs/adr/NNNN-titulo-em-kebab.md`.
3. Preencha:
   - **Contexto:** problema e restrições reais (plataformas afetadas, issues `type:risk` relacionadas).
   - **Decisão:** uma frase clara.
   - **Alternativas:** pelo menos 2, com o motivo do descarte.
   - **Consequências:** o que melhora e o que piora.
4. Status `proposto` se o usuário ainda não aprovou; `aceito` só com aprovação explícita.
5. Se substitui outro ADR, marque o antigo como `substituído por ADR NNNN`.
6. Se a decisão muda a arquitetura, atualize `docs/planning/ARCHITECTURE.md` no mesmo commit.
7. Commit: `docs(adr): NNNN titulo`.

Nunca edite a decisão de um ADR aceito. Crie outro que o substitua.
