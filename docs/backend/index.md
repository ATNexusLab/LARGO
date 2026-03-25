# Backend (gateway)

Resumo: O gateway é implementado em Rust com Axum. É o ponto único de entrada da aplicação e orquestra chamadas entre a UI, o AI Worker e o banco.

Conteúdo sugerido:
- Visão geral da arquitetura do gateway
- Rotas expostas e contratos (ex: `/api/expenses`, `/api/tasks`)
- Como rodar localmente (cargo run / Docker)
- Patterns: Adapters (MongoAdapter, GeminiAdapter), Error handling (thiserror/anyhow)
- Testes: como rodar `cargo test` e como escrever testes de integração com Axum

Referências:
- `docs/architecture/architecture.md`
- `docs/architecture/adr-db.md`

