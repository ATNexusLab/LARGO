# Backend (gateway)

Nesta branch de recovery, o `gateway` **já está versionado no repositório** como foundation inicial em Rust/Axum. Este documento separa o que já existe hoje do que continua sendo alvo arquitetural.

## Estado atual

- existe diretório `gateway/` com código Rust/Axum versionado;
- `GET /healthz` já existe e retorna `200 OK`;
- `POST /tasks` já existe com validação de payload e persistência em MongoDB;
- existe validação explícita de conectividade com Mongo;
- existe o binário Rust `db-init` para o smoke path inicial do banco;
- o `docker-compose.yml` continua expondo **apenas o MongoDB**;
- o gateway ainda **não está ligado como serviço do compose**.

## Foundation implementada nesta task

Primeiro fluxo real já materializado no repositório:

- capability inicial: `tasks`;
- primeira rota persistida: `POST /tasks`;
- banco exigido: Mongo ponta a ponta;
- autenticação de aplicação: fora da foundation inicial;
- contrato mínimo e smoke path: ver `docs/architecture/notes/01-tasks-foundation.md`.

### Verificação manual atual

Hoje o caminho prático é:

1. subir Mongo com `docker compose up -d mongo`;
2. executar `db-init` manualmente via Rust;
3. rodar `cargo test -p gateway` para validar a foundation.

Exemplo para host local:

```sh
export MONGO_URI="mongodb://admin:change_me@127.0.0.1:27017/largo?authSource=admin"
export MONGO_DB_NAME=largo
cargo run -p gateway --bin db-init
```

## Escopo planejado

O gateway continua tendo como alvo atuar como ponto único de entrada da aplicação e orquestrar chamadas entre UI, AI Worker e banco.

## Foundation inicial aprovada

A foundation inicial aprovada já foi parcialmente materializada no código e segue sendo a referência para evolução incremental do backend.

## Conteúdo

- foundation atual: healthcheck, `POST /tasks`, conectividade Mongo e `db-init`
- visão geral da arquitetura do gateway
- rotas expostas e contratos conforme novas capabilities forem entrando
- evolução futura: auth, WebSocket, integração com AI Worker e novos adapters

## Referências

- `docs/architecture/adr/architecture/04-architecture.md`
- `docs/architecture/adr/database/01-adr-db.md`
- `docs/architecture/adr/database/03-adr-migrations.md`
- `docs/architecture/notes/01-tasks-foundation.md`
