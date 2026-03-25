# Tarefa: [INFRA] Adicionar serviços ao docker-compose.yml

- [ ] Etapa 1: Planejamento (Consultar ADR 04 e ADR 01 para contratos entre serviços)
- [ ] Etapa 2: Implementação (Adicionar gateway Rust, ai-worker Python, web React/Bun)
- [ ] Etapa 3: Validação Docker (`docker compose up --build` sem erros, healthchecks passando)
- [ ] Etapa 4: Atualização de `.env.example` com todas as variáveis dos novos serviços

# Tarefa: [FEAT] Implementar gateway Rust/Axum (estrutura inicial)

- [ ] Etapa 1: Planejamento (Consultar ADR 04, skill axum-patterns [planejada] e multi-stage-dockerfile)
- [ ] Etapa 2: Implementação (Criar crate gateway com rotas /api/expenses, /api/tasks, JWT middleware)
- [ ] Etapa 3: Validação (cargo test passando; docker compose up --build sem erros)
- [ ] Etapa 4: Documentação (Atualizar docs/backend/README.md e docs/backend/rust/README.md)

# Tarefa: [FEAT] Implementar AI Worker Python/FastAPI (estrutura inicial)

- [ ] Etapa 1: Planejamento (Consultar ADR 02 para modelo Gemini; skill ai-worker-patterns [planejada])
- [ ] Etapa 2: Implementação (Criar FastAPI com endpoint de OCR + Gemini; usar toon_format)
- [ ] Etapa 3: Validação (pytest passando; docker compose run ai-worker pytest)
- [ ] Etapa 4: Documentação (Atualizar docs/ai/README.md e docs/ai/prompts.md)

# Tarefa: [FEAT] Implementar frontend React/Bun (estrutura inicial)

- [ ] Etapa 1: Planejamento (Consultar skill ui-ux-pro-max, vercel-react-best-practices, shadcn)
- [ ] Etapa 2: Implementação (Criar app React + Vite; aplicar tokens de design; páginas principais)
- [ ] Etapa 3: Validação (bun test passando; docker compose up web sem erros)
- [ ] Etapa 4: Documentação (Atualizar docs/frontend/)
