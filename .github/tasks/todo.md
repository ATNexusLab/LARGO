# Tarefa: [FEAT] Implementar gateway Rust/Axum (estrutura inicial)

- [ ] Etapa 1: Planejamento (Consultar ADR 04; criar skill axum-patterns com skill-creator)
- [ ] Etapa 2: Implementação (Criar crate gateway com rotas /api/expenses, /api/tasks, JWT middleware)
- [ ] Etapa 3: Validação (cargo test passando; conecta ao MongoDB local)
- [ ] Etapa 4: Documentação (Atualizar docs/backend/README.md e docs/backend/rust/README.md)

# Tarefa: [FEAT] Implementar AI Worker Python/FastAPI (estrutura inicial)

- [ ] Etapa 1: Planejamento (Consultar ADR 02 para modelo Gemini; criar skill ai-worker-patterns com skill-creator)
- [ ] Etapa 2: Implementação (Criar FastAPI com endpoint de OCR + Gemini; usar toon_format)
- [ ] Etapa 3: Validação (pytest passando; conecta ao gateway local)
- [ ] Etapa 4: Documentação (Atualizar docs/ai/README.md e docs/ai/prompts.md)

# Tarefa: [FEAT] Implementar frontend React/Bun (estrutura inicial)

- [ ] Etapa 1: Planejamento (Consultar skill ui-ux-pro-max, vercel-react-best-practices, shadcn)
- [ ] Etapa 2: Implementação (Criar app React + Vite; aplicar tokens de design; páginas principais)
- [ ] Etapa 3: Validação (bun test passando; conecta ao gateway local)
- [ ] Etapa 4: Documentação (Atualizar docs/frontend/)

## Nota de infraestrutura
Durante o desenvolvimento, cada serviço roda localmente (cargo run / uvicorn / bun dev).
MongoDB fica em Docker (`docker compose up mongo`).
Docker Compose completo (gateway + ai-worker + web) será montado apenas ao finalizar o desenvolvimento.
