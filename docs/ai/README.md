# AI Worker

Nesta branch de recovery, o serviço `ai-worker` **ainda não está versionado no repositório**. Este documento registra o papel planejado do componente e referencia a documentação de prompts já presente em `docs/ai/`.

## Estado atual

- não existe diretório `ai-worker/` com código Python/FastAPI versionado;
- não há serviço OCR/Gemini executável a partir deste repositório;
- o artefato mais concreto da área hoje é a documentação de prompts TOON.

## Escopo planejado

Resumo: Serviço Python/FastAPI responsável por OCR e por conversão de texto em JSON estruturado via Gemini. Segue o padrão TOON para prompts.

Recomenda-se Gemini 3.1 Flash para processamento multimodal e gemini-3.1-flash-lite para processamento de texto.

Conteúdo sugerido:
- Fluxo: Vision API → TOON → Gemini → JSON normalizado
- Como configurar chaves (variáveis de ambiente necessárias)
- Como mockar Vision API/Gemini em testes
- Exemplos de prompts TOON e schemas esperados
- Conexão com o gateway (endpoints expostos)

Referências:
- `docs/architecture/adr/architecture/04-architecture.md`
- `docs/ai/prompts.md`
