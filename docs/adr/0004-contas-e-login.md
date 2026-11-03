# ADR 0004 — Contas de usuário com login e senha em todos os dispositivos

- **Status:** aceito (requisito do projeto); os detalhes de implementação podem ser refinados
- **Data:** 2026-11-03

## Contexto
O pareamento (ADR implícito na arquitetura) prova que um **dispositivo** é confiável, mas não diz **quem** está usando. A casa tem várias pessoas, e o acesso à mídia precisa ser controlado em qualquer dispositivo: PC, celular e TV. No futuro haverá acesso pela web pública (ADR 0005), que exige identidade forte.

## Decisão
Duas camadas, ambas obrigatórias:

1. **Confiança de dispositivo:** pareamento por PIN e TLS com certificado fixado.
2. **Identidade de usuário:** conta com login e senha.

Detalhes:
- No primeiro uso da casa, cria-se uma conta **admin**.
- Senhas guardadas com **Argon2id**. Nunca em texto puro, nunca em log.
- Login gera um token de sessão por dispositivo, revogável pela UI.
- As contas são **replicadas entre nós pareados**, com mudanças assinadas pelo admin.
- **Papéis:** admin, membro e convidado. Perfis com PIN curto pra troca rápida na TV.
- **TV:** login aprovado pelo celular (QR/código), sem digitar senha no controle remoto.
- Limite de tentativas com atraso progressivo e log de acessos.

## Alternativas consideradas
- **Só pareamento, sem contas:** qualquer pessoa com o dispositivo vê tudo. Descartado.
- **Conta na nuvem (OAuth de terceiros):** contraria o princípio local-first e não funciona offline. Pode entrar como opção no M11.
- **Senha única por dispositivo:** não distingue pessoas nem permite papéis.

## Consequências
- Primeiro uso tem uma etapa a mais (criar admin), mas o acesso fica controlado desde o início.
- Replicação de contas entre nós precisa de resolução de conflito (versão e assinatura do admin).
- Base pronta pro acesso remoto e pra 2FA no M11.
