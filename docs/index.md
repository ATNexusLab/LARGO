# Documentação — Índice Raiz

Esta é a fonte única de verdade (single source of truth) para a documentação do projeto LARGO. Use-a para localizar documentos técnicos, ADRs, guias de uso e documentação de skills/agents.

Sumário

- [Backend (gateway)](./backend/index.md)
- [AI Worker (ai)](./ai/index.md)
- [Web (frontend)](./web/index.md)
- [Database](./database/index.md)
- [Arquitetura e ADRs](./architecture/)
- [Skills](./skills/index.md)
- [Agents & Instruções](./agents/index.md)

Convenções

- Idioma: Português técnico (pt-BR).
- Formatos: Markdown para documentação; TOON para prompts enviados ao Gemini (ver `docs/ai/prompts.md`).
- Atualizações: prefira PRs pequenos e revisáveis. Atualize este índice sempre que mover/criar documentos.
- Skills: registrar novas skills em `skills-lock.json` e documentar em `docs/skills/`.

Como contribuir

1. Atualize ou crie o arquivo adequado em `docs/<área>/`.
2. Atualize este índice (`docs/index.md`) apontando para o novo artefato.
3. Abra PR pequeno descrevendo apenas mudanças relacionadas à documentação.

Próximos passos

- Preencher os índices por área com links e exemplos. 
- Documentar prompts TOON em `docs/ai/prompts.md`.

---
