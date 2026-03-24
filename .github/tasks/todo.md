# Tarefa: [FEAT] Sprint 1 — Arquitetura, validação e instanciação do DB

- [ ] Etapa 1: Planejamento (Consultar docs/architecture/adr-db.md e README.md)
- [ ] Etapa 2: Implementação (Serviço afetado: infra — adicionar/validar serviço `mongo` em docker-compose.yml e documentar validators)
- [ ] Etapa 3: Validação Docker (`docker compose up --build` sem erros; healthcheck do Mongo ok)
- [ ] Etapa 4: Testes (integração para verificar índices/validators — `tests/integration/test_db_init.py`)
- [ ] Etapa 5: Atualização de docs e `.env.example` (docs/database/*)

## Revisão/Post-Mortem

- [Notas sobre desafios ou débitos técnicos gerados]

# Tarefa: [DOCS] Atualizar ADR e .env.example para seleção de modelos Gemini

- [ ] Etapa 1: Planejamento (Consultar docs/architecture/adr-gemini-model-selection.md e README.md)
- [ ] Etapa 2: Implementação (Criar ADR e atualizar .env.example com GEMINI_MODEL_IMAGE e GEMINI_MODEL_TEXT)
- [ ] Etapa 3: Validação Docker (`docker compose up --build` para garantir integridade)
- [ ] Etapa 4: Testes (documentação e variáveis carregadas em ambiente de dev)
- [ ] Etapa 5: Atualização de docs e `.env.example` (docs/architecture/adr-gemini-model-selection.md)

## Revisão/Post-Mortem

- [Notas sobre desafios ou débitos técnicos gerados]
