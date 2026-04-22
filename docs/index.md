# Documentação — Índice Raiz

Esta é a fonte única de verdade (single source of truth) para a documentação do projeto LARGO. Use-a para localizar documentos técnicos, ADRs, guias de uso e documentação de skills/agents.

## Estado atual vs. alvo

No estado atual desta branch de recovery:

- o repositório **já contém código versionado** para a foundation de `gateway/`;
- `ai-worker/` e `web/` ainda **não estão versionados** nesta branch;
- o `docker-compose.yml` disponível sobe **apenas o MongoDB**;
- a documentação de backend deve distinguir a **foundation já implementada** do **alvo arquitetural**;
- a documentação de frontend e AI deve ser lida como **alvo arquitetural/plano**, salvo quando um documento disser explicitamente que algo já está implementado.

## Sumário

- [Backend (gateway)](./backend/README.md)
- [AI Worker (ai)](./ai/README.md)
- [Frontend (web + ui)](./frontend/README.md)
- [Database](./database/README.md)
- [Arquitetura e ADRs](./architecture/)
- [Skills](./skills/README.md)
- [Agents & Instruções](./agents/README.md)

## Convenções

- Idioma: Português técnico (pt-BR).
- Formatos: Markdown para documentação; TOON para prompts enviados ao Gemini (ver `docs/ai/prompts.md`).
- Modelos Gemini: gemini-3.1-flash (imagem) e gemini-3.1-flash-lite (texto).
- Ao documentar features, sempre distinguir **implementado hoje** vs **planejado/alvo**.
- Atualizações: prefira commits pequenos e atômicos; atualize este índice sempre que mover/criar documentos.
- Skills: registrar novas skills em `skills-lock.json` e documentar em `docs/skills/`.

## Como contribuir

1. Atualize ou crie o arquivo adequado em `docs/<área>/`.
2. Atualize este índice (`docs/index.md`) apontando para o novo artefato.
3. Ao descrever uma capability, deixe explícito se ela é **estado atual** ou **estado alvo**.
4. Faça commits locais atômicos e push quando pronto.

## Próximos passos

- Atualizar esta documentação conforme as próximas tasks ampliarem o gateway e materializarem `ai-worker` e frontend.
- Documentar prompts TOON em `docs/ai/prompts.md`.

---
