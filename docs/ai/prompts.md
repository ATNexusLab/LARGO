# Prompts TOON — AI Worker

Objetivo
--------
Documentar o padrão TOON usado para enviar contexto ao Gemini (modelos generativos) a partir do AI Worker. Contém exemplos para processamento de NF-e (OCR -> Gemini) e para interpretação de texto livre (tarefas).

Visão geral
----------
TOON (Token-Oriented Object Notation) é o formato compacto recomendado para prompts: reduz tokens e mantém estrutura. Sempre encoder o objeto com a lib `toon_format.encode()` antes de enviar ao Gemini.

Modelos gerativos: Gemini 3.1 Flash (imagem) / gemini-3.1-flash-lite (texto).

Regras rápidas
-------------
- Usar `toon.encode(obj)` para arrays de objetos.
- Para prompts de NF-e: enviar contexto (metadados do arquivo, texto OCR) codificado em TOON.
- Nunca enviar JSON verboso direto no prompt — sempre usar TOON.

Exemplo — NF-e (pseudo):

```python
from toon_format import encode
nfes = [
  {"id": 1, "descricao": "Mercado", "valor": 150.0}
]
payload = encode(nfes)
# payload exemplo: gastos[1]{id,descricao,valor}:\n 1,Mercado,150.0
```

Prompt (esqueleto) enviado ao Gemini:

- Contexto: TOON-encoded NF-e + metadados (data, cnpj, estabelecimento)
- Instruções: "Extrair total, itens[], data, cnpj, estabelecimento no formato JSON normalizado"

Schema de saída esperado (exemplo)
----------------------------------
{
  "total": 150.0,
  "itens": [{"descricao": "Produto X", "valor": 50.0, "quantidade": 1}],
  "data": "2026-03-01",
  "cnpj": "00.000.000/0000-00",
  "estabelecimento": "Loja Exemplo"
}

Onde documentar exemplos
------------------------
- Colocar exemplos reais (texto OCR -> TOON -> Gemini -> JSON) neste arquivo.
- Adicionar testes unitários que mockem Gemini/Vision e validem parser do AI Worker.

Notas
-----
- Atualizar `docs/ai/prompts.md` sempre que o schema TOON mudar.
- Referenciar este arquivo em `docs/index.md` e no README do AI Worker.
