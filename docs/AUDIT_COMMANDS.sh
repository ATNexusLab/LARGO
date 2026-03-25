#!/usr/bin/env bash
set -euo pipefail

echo "Script sugerido: reorganizar docs — revisar antes de executar"

# Criar pastas necessárias
mkdir -p docs/design/generated
mkdir -p docs/design/assets/fonts

# 1) Mover CSS de UI para pasta de gerados (mantém histórico)
if [ -f docs/frontend/ui/styles.css ]; then
  git mv docs/frontend/ui/styles.css docs/design/generated/ui-styles.css
else
  echo "Pular: docs/frontend/ui/styles.css não encontrado"
fi

# 2) Organizar fonte Azonix dentro de assets/fonts
if [ -f docs/design/azonix/Azonix.otf ]; then
  git mv docs/design/azonix/Azonix.otf docs/design/assets/fonts/Azonix.otf
else
  echo "Pular: docs/design/azonix/Azonix.otf não encontrado"
fi

# 3) Padronizar index.md -> README.md quando seguro (não sobrescreve existentes)
if [ -f docs/ai/index.md ] && [ ! -f docs/ai/README.md ]; then
  git mv docs/ai/index.md docs/ai/README.md
else
  echo "Pular move docs/ai/index.md (target existe ou source ausente)"
fi

if [ -f docs/database/index.md ] && [ ! -f docs/database/README.md ]; then
  git mv docs/database/index.md docs/database/README.md
else
  echo "Pular move docs/database/index.md (target existe ou source ausente)"
fi

if [ -f docs/agents/index.md ] && [ ! -f docs/agents/README.md ]; then
  git mv docs/agents/index.md docs/agents/README.md
else
  echo "Pular move docs/agents/index.md (target existe ou source ausente)"
fi

if [ -f docs/frontend/web/index.md ] && [ ! -f docs/frontend/web/README.md ]; then
  git mv docs/frontend/web/index.md docs/frontend/web/README.md
else
  echo "Pular move docs/frontend/web/index.md (target existe ou source ausente)"
fi

# 4) Preservar index de skills para merge manual (existência de README.md detectada anteriormente)
if [ -f docs/skills/index.md ]; then
  git mv docs/skills/index.md docs/skills/index.source.md
  echo "Moved docs/skills/index.md -> docs/skills/index.source.md (review & merge manual)"
fi

echo "Revisar 'git status' e resolver conflitos localmente. Commit e abrir PR quando pronto."

echo "Exemplo de commit:
  git checkout -b docs/reorg/audit
  # executar este script
  git add -A
  git commit -m 'docs: reorganize docs (audit)\n\nCo-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>'
  git push origin HEAD
" 

echo "IMPORTANTE: confirme licenças de fontes antes de commitar arquivos binários."
