# Documentação — Índice Raiz

Esta é a fonte única de verdade (single source of truth) para a documentação do projeto LARGO. Use-a para localizar documentos técnicos, ADRs, guias de uso e documentação de skills/agents.

Sumário

- [Backend (gateway)](./backend/index.md)
- [AI Worker (ai)](./ai/README.md)
- [Frontend (web + ui)](./frontend/README.md)
- [Database](./database/README.md)
- [Arquitetura e ADRs](./architecture/)
- [Skills](./skills/README.md)
- [Agents & Instruções](./agents/README.md)

Convenções

- Idioma: Português técnico (pt-BR).
- Formatos: Markdown para documentação; TOON para prompts enviados ao Gemini (ver `docs/ai/prompts.md`).
- Atualizações: prefira commits pequenos e atômicos; atualize este índice sempre que mover/criar documentos.
- Skills: registrar novas skills em `skills-lock.json` e documentar em `docs/skills/`.

Como contribuir

1. Atualize ou crie o arquivo adequado em `docs/<área>/`.
2. Atualize este índice (`docs/index.md`) apontando para o novo artefato.
3. Faça commits locais atômicos e push quando pronto.

Próximos passos

- Preencher os índices por área com links e exemplos. 
- Documentar prompts TOON em `docs/ai/prompts.md`.

---
