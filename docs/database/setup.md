Setup do banco (desenvolvimento local)

Passos mínimos
---------------
1) Copiar o exemplo de variáveis de ambiente e ajustar credenciais locais:

   cp .env.example .env
   # editar .env conforme necessário

2) Subir o serviço MongoDB (isolado):

   docker compose up -d mongo

3) Aplicar validators e índices (duas opções):

   a) Usando container one-shot (recomendado):
      docker compose run --rm mongo-init

   b) Rodando o script localmente (teste/depuração):
      pip install -r scripts/requirements.txt
      MONGO_URI="mongodb://admin:secret@localhost:27017/largo?authSource=admin" python scripts/db/init_indexes.py

Verificações
------------
- Conferir coleções e índices:
  docker compose exec mongo mongo --eval "db.getSiblingDB('largo').getCollectionNames()"
  docker compose exec mongo mongo --eval "db.getSiblingDB('largo').expenses.getIndexes()"

Backup e restauração
--------------------
- Backup: mongodump --uri "$MONGO_URI" --archive=backup.gz
- Restauração: mongorestore --uri "$MONGO_URI" --archive=backup.gz --drop

Notas
-----
- Não commitar o arquivo `.env` com segredos. Mantenha `.env.example` atualizado.
- Em ambiente de produção considerar replica set e estratégias de backup automatizado.
