## 1. Arquitetura Macro (Sistema)

A estrutura de alto nível foca na separação de responsabilidades por tecnologia e ambiente de execução.

* **Padrão:** **Microservices Lite (Orquestrados)**.
* **Orquestração:** **Docker Compose** com rede interna isolada.
* **Serviços:**
    * **Gateway (Rust/Axum):** O núcleo operacional e ponto único de entrada.
    * **AI-Worker (Python/FastAPI):** Serviço especializado em processamento de linguagem natural e visão computacional.
    * **Web (React/Vite):** Interface do usuário servida via Bun.
    * **Database (MongoDB):** Persistência de dados local.
* **Comunicação:**
    * **Interna:** HTTP/REST entre serviços dentro da rede Docker.
    * **Tempo Real:** WebSockets para notificações de processamento de IA concluído.

---

## 2. Arquitetura Micro (Software)

Aqui definimos como o código é organizado dentro de cada serviço para evitar o "código espaguete".

### Backend (Rust & Python)
* **Padrão:** **Arquitetura Hexagonal (Ports & Adapters)**.
* **Camadas:**
    * **Domain (Core):** Contém apenas as regras de negócio e entidades (ex: cálculo de despesas), sem depender de frameworks.
    * **Application (Use Cases):** Orquestra o fluxo de dados (ex: "Processar novo recibo").
    * **Ports (Interfaces):** Define o contrato do que o sistema precisa (ex: "Preciso de um OCR", "Preciso salvar um gasto").
    * **Adapters (Infrastructure):** Implementações reais (ex: `GeminiAdapter`, `MongoAdapter`, `AxumRouter`).

### Frontend (React)
* **Padrão:** **Feature-Based Architecture**.
* **Organização:** O código é dividido por módulos de negócio (`Finance`, `Tasks`) em vez de tipos de arquivos genéricos.
* **Componentização:**
    * **Presentational Components:** Componentes "burros" que apenas recebem dados via props e não possuem lógica de API.
    * **Custom Hooks:** Isolam a lógica de chamadas ao Gateway e gerenciamento de estado (React Query/Zustand).

---

## 3. Padrões de Dados e Integração

Estes são os "protocolos" que garantem a eficiência do sistema.

| Domínio | Padrão Escolhido | Objetivo |
| :--- | :--- | :--- |
| **IA/Prompts** | **TOON (Token-Oriented Object Notation)** | Reduzir o custo de tokens no Gemini Free Tier em ~40%. |
| **Financeiro** | **Hierarquia Visual Obrigatória** | Legibilidade máxima: Rótulo → Valor (BRL) → Contexto. |
| **Segurança** | **JWT (Stateless)** | Alvo arquitetural de autenticação via claims validadas no Gateway Rust; exceção explícita da foundation Task 1: `POST /tasks` opera sem autenticação, em modo local/bootstrap, até a capability de identidade existir. |
| **Tipagem** | **TypeScript/Rust Strict** | Eliminar erros de runtime e garantir contratos de dados sólidos. |

