# Backend (gateway)

O gateway é implementado em Rust com Axum. É o ponto único de entrada da aplicação e orquestra chamadas entre a UI, o AI Worker e o banco.

## Conteúdo

- Visão geral da arquitetura do gateway
- Rotas expostas e contratos (ex: `/api/expenses`, `/api/tasks`)
- Como rodar localmente (`cargo run` / Docker)
- Patterns: Adapters (MongoAdapter, GeminiAdapter), Error handling (thiserror/anyhow)
  - Nota: GeminiAdapter deve usar `gemini-3.1-flash` para entradas multimodais quando aplicável.
- Testes: como rodar `cargo test` e como escrever testes de integração com Axum

## Subdiretórios

- `rust/` — detalhes do crate gateway (estrutura, módulos, dependências)
- `python/` — detalhes do AI Worker (FastAPI, fluxo OCR/Gemini)

## Referências

- `docs/architecture/adr/architecture/04-architecture.md`
- `docs/architecture/adr/database/01-adr-db.md`

