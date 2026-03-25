#!/bin/sh
set -e

echo "Criando diretórios de destino..."
mkdir -p docs/architecture/adr/database
mkdir -p docs/architecture/adr/ai
mkdir -p docs/architecture/adr/architecture

echo "Aplicando git mv (revisar saídas):"
# Os comandos abaixo ignoram erros se os arquivos de origem não existirem
git mv docs/architecture/adr-db.md docs/architecture/adr/database/01-adr-db.md || true
git mv docs/architecture/adr-gemini-model-selection.md docs/architecture/adr/ai/02-adr-gemini-model-selection.md || true
git mv docs/architecture/adr-migrations.md docs/architecture/adr/database/03-adr-migrations.md || true
git mv docs/architecture/architecture.md docs/architecture/adr/architecture/04-architecture.md || true

echo "Concluído. Revise com 'git status' e 'git diff --staged' antes de commitar."