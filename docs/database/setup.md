Setup do banco (desenvolvimento local)

Passos mínimos
---------------
1) Copiar o exemplo de variáveis de ambiente e ajustar credenciais locais:

   cp .env.example .env
   # editar .env conforme necessário

2) Subir o serviço MongoDB (isolado):

   docker compose up -d mongo

3) Aplicar validators e índices:

   NOTA: O serviço `mongo-init` (container one-shot) está **planejado** via ADR 03 (proposto, ainda não aceito).
   Enquanto ADR 03 não for aceito e implementado, usar o script Python para setup manual (teste/depuração):

       pip install -r scripts/requirements.txt
       MONGO_URI="mongodb://admin:secret@localhost:27017/largo?authSource=admin" python scripts/db/init_indexes.py

   Quando ADR 03 for aceito e o binário Rust `db-init` estiver disponível, o comando será:
       docker compose run --rm db-init

Verificações
------------
- Conferir coleções e índices:
  docker compose exec mongo mongosh --eval "db.getSiblingDB('largo').getCollectionNames()"
  docker compose exec mongo mongosh --eval "db.getSiblingDB('largo').expenses.getIndexes()"

Backup e restauração
--------------------
- Backup: mongodump --uri "$MONGO_URI" --archive=backup.gz
- Restauração: mongorestore --uri "$MONGO_URI" --archive=backup.gz --drop

Notas
-----
- Não commitar o arquivo `.env` com segredos. Mantenha `.env.example` atualizado.
- Em ambiente de produção considerar replica set e estratégias de backup automatizado.
