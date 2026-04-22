Setup do banco (desenvolvimento local)

Passos mínimos
---------------
1) Copiar o exemplo de variáveis de ambiente e ajustar credenciais locais:

   cp .env.example .env
   # editar .env conforme necessário

2) Subir o serviço MongoDB (isolado):

   docker compose up -d mongo

3) Aplicar validator e estrutura mínima da foundation:

   O caminho autoritativo foi fechado no ADR 03: um job one-shot em Rust chamado `db-init`.

   Contrato operacional esperado para a Task 1:

   - `db-init` deve ser executado com o Mongo já disponível;
   - cria/garante a coleção `tasks`;
   - aplica o validator mínimo da collection;
   - garante o conjunto mínimo de índices da foundation;
   - pode ser executado repetidamente sem efeitos colaterais incorretos.

   **Estado atual implementado:** o binário já existe no crate `gateway`, mas ainda **não** existe serviço `db-init` no `docker-compose.yml`.

   Caminho manual atual (host local):

       export MONGO_URI="mongodb://admin:change_me@127.0.0.1:27017/largo?authSource=admin"
       export MONGO_DB_NAME=largo
       cargo run -p gateway --bin db-init

   Observação importante:
   - o `MONGO_URI` de `.env.example` usa o hostname `mongo`, adequado para comunicação entre containers;
   - ao rodar o binário Rust no host, use `127.0.0.1`/`localhost` enquanto o compose continuar materializando apenas o MongoDB.

   Comando alvo futuro, quando o compose também materializar esse job:
       docker compose run --rm db-init

Verificações
------------
- Conferir coleções e índices:
   docker compose exec mongo mongosh --eval "db.getSiblingDB('largo').getCollectionNames()"
   docker compose exec mongo mongosh --eval "db.getSiblingDB('largo').tasks.getIndexes()"

- Smoke path mínimo esperado após `db-init`:
  - uma execução em banco vazio termina com sucesso;
  - uma segunda execução também termina com sucesso;
  - inserção de documento válido em `tasks` funciona;
  - inserção de documento inválido para `tasks` falha por validator.

- Verificação adicional útil na foundation atual:
   - `cargo test -p gateway` cobre healthcheck, conectividade Mongo, `POST /tasks` e smoke path de `db-init`.

Backup e restauração
--------------------
- Backup: mongodump --uri "$MONGO_URI" --archive=backup.gz
- Restauração: mongorestore --uri "$MONGO_URI" --archive=backup.gz --drop

Notas
-----
- Não commitar o arquivo `.env` com segredos. Mantenha `.env.example` atualizado.
- Em ambiente de produção considerar replica set e estratégias de backup automatizado.
- `db-init` não substitui migrations de produto futuras; ele é o mecanismo inicial autorizado para preparar a foundation do banco.
- O compose atual continua sendo **Mongo-only**; `db-init` e gateway seguem em caminho manual/Rust até entrarem como serviços.
