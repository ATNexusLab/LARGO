# Lição: evitar env_file global no docker-compose

Data: 2026-03-24

Resumo:
O uso de `env_file: .env` injeta todas as variáveis do arquivo de ambiente no container, incluindo variáveis não relacionadas e segredos. Isso quebra isolamento por responsabilidade e pode levar ao vazamento acidental de segredos ou confusão entre serviços.

Ação tomada:
- Removido `env_file` do serviço `mongo` no `docker-compose.yml`.
- Mantido `environment` com as variáveis explicitamente necessárias para o MongoDB.

Recomendações:
- Usar arquivos separados por serviço (ex.: `.env.mongo`, `.env.ai`) ou definir `environment` explicitamente.
- Preferir Docker secrets ou variáveis fornecidas pelo ambiente/CI para produção.
- Validar com `docker compose config` para confirmar que apenas as variáveis esperadas são expostas.

Teste/Notas:
- Comando usado: `docker compose config` (verificou config válida).
