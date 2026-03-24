# ADR: Seleção de modelo Gemini para tipos de entrada

Status: Aceito
Data: 2026-03-24

Contexto
- O LARGO deve rodar liso e com custo mínimo no "vasco" (gratuito).
- Há dois tipos principais de entrada: imagens/recibos (fotos de NF-e) e texto natural (frases, mensagens curtas).

Decisão
- Usar Gemini 3.1 Flash (gemini-3.1-flash) para processamento multimodal de imagens/recibos (OCR + interpretação). Este modelo lida melhor com fotos, baixa iluminação e papel amassado.
- Usar Gemini 3.1 Flash-Lite (gemini-3.1-flash-lite) para processamento de texto natural (mensagens, comandos, pequenas descrições) por ser mais barato e muito mais rápido.

Motivação
- Otimizar custo e latência: reduzir chamadas ao modelo multimodal caro ao usar a variante Lite para casos textuais simples.
- Melhor robustez em OCR/parse de NF-e ao usar a variante multimodal adequada.

Consequências e implementação
- Variáveis de ambiente novas (adicionadas em .env.example):
  - GEMINI_MODEL_IMAGE=gemini-3.1-flash
  - GEMINI_MODEL_TEXT=gemini-3.1-flash-lite
- ai-worker: adicionar lógica para escolher modelo com base no tipo de entrada (por exemplo, payload.busca.type == "image" -> GEMINI_MODEL_IMAGE). Garantir chamadas assíncronas e mock nos testes.
- Gateway: incluir campo no payload que indica tipo de entrada (e.g., "type": "image" | "text").
- Prompts: continuar usando TOON para estruturar contexto antes de enviar ao Gemini.
- Testes: unitários que mockam Gemini (Flash e Flash-Lite) e integração mínima no contêiner.

Notas operacionais
- Monitorar custos do modelo Flash e aplicar limitações/ratelimits caso necessário.
- Documentar a decisão em docs/architecture (este arquivo) e atualizar docs/ai/prompts quando existir.

Revisões futuras
- Se a qualidade/latência justificar, avaliar consolidar para um único modelo multimodal se o custo for aceitável.

---
