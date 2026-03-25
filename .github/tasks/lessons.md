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

---

# Lição: links de documentação devem referenciar paths reais do repositório

Data: 2026-03-25

Resumo:
Durante a auditoria completa, foram encontrados 9 links quebrados espalhados pela documentação — todos apontando para caminhos que nunca existiram (ex.: `docs/architecture/architecture.md`, `docs/ui/styles.css`, `docs/design/azonix`). O padrão de falha foi documentar links "esperados" sem verificar a estrutura real de diretórios.

Ação tomada:
- Corrigidos 9 links quebrados em 7 arquivos.
- Criado índice `docs/architecture/adr/README.md` para facilitar navegação.
- Removidos arquivos redundantes: `docs/backend/index.md`, `docs/skills/index.source.md`.

Recomendações:
- Ao criar links internos em qualquer arquivo de docs/, usar `find docs/ -name "*.md"` ou `ls docs/caminho/` para confirmar que o alvo existe antes de referenciar.
- Ao criar um novo arquivo, verificar se o path já é referenciado em algum doc existente e, se sim, garantir que o arquivo está no lugar certo.
- Nunca criar stubs com "Referências:" sem verificar os targets.

---

# Lição: skills listadas em copilot-instructions devem estar em skills-lock.json

Data: 2026-03-25

Resumo:
A tabela de skills em `copilot-instructions.md` listava 10 skills como "disponíveis", mas apenas 5 estavam em `skills-lock.json`. As outras 8 (axum-patterns, mongodb-patterns, etc.) nunca foram criadas. Isso causava confusão sobre o que estava disponível para uso.

Ação tomada:
- Tabela atualizada: skills instaladas marcadas com ✅; skills planejadas (não criadas) marcadas com 🔜.
- `docs-guidelines` registrada em `skills-lock.json` (estava criada localmente mas nunca registrada).

Recomendações:
- Ao criar uma skill com `skill-creator`, imediatamente registrar em `skills-lock.json` E atualizar a tabela em `copilot-instructions.md`.
- Ao adicionar uma skill à tabela de instruções, verificar primeiro se ela existe em `skills-lock.json`.
- Skills planejadas podem ser listadas mas devem ser marcadas com [planejada] ou 🔜.

---

# Lição: mongosh substituiu mongo como client CLI a partir do MongoDB 6.0

Data: 2026-03-25

Resumo:
O `docker-compose.yml` tinha healthcheck usando `mongo --eval` que é depreciado no MongoDB 6.0+. O container `mongo:6.0.8` não inclui o binário `mongo` legado — apenas `mongosh`.

Ação tomada:
- Substituído `["CMD", "mongo", "--eval", ...]` por `["CMD", "mongosh", "--eval", "db.adminCommand('ping')"]`.
- Corrigidos também os comandos em `docs/database/setup.md`.

Recomendações:
- Para MongoDB 6.0+, sempre usar `mongosh` nos healthchecks e scripts de administração.
- Ao fixar uma versão de imagem Docker, verificar quais utilitários CLI estão disponíveis/depreciados naquela versão.
