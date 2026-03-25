# AI Worker

Resumo: Serviço Python/FastAPI responsável por OCR (Vision API) e por conversão de texto em JSON estruturado via Gemini. Segue o padrão TOON para prompts.

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

